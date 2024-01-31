use std::collections::HashMap;

use arrayvec::ArrayVec;

use crate::{bb_settings::{self, KBSettings}, chess_move::{self, ChessMove}, colored_piece_type::ColoredPieceType, endgame_table::{self, EndgameTable}, evaluation, game::{Game, GameState}, opening_book::OpeningBook, piece_type::PieceType, search_stats::SearchStats};

#[derive(PartialEq, Eq, Clone, Copy)]
enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
    Unknown,
}

#[derive(Clone)]
struct TTEntry {
    depth: i8,
    score: i32,
    best_move: ChessMove,
    node_type: NodeType,
}

pub struct KarpfenBot {
    stats: SearchStats,
    opening_book: OpeningBook,
    endgame_table: EndgameTable,
    quiet_move_history: [[u64; 64]; 64],
    killer_moves: [ChessMove; 256],
    transposition_table: HashMap<u64, TTEntry>,
    settings: KBSettings,
}

impl KarpfenBot {
    pub fn new() -> KarpfenBot {
        return KarpfenBot {
            stats: SearchStats::new(),
            opening_book: OpeningBook::new(),
            endgame_table: EndgameTable::load(4),
            quiet_move_history: [[0; 64]; 64],
            killer_moves: [chess_move::NULL_MOVE; 256],
            transposition_table: HashMap::new(),
            settings: bb_settings::STANDARD_KB_SETTINGS
        };
    }

    pub fn reset(&mut self) {
        for i in 0..64 {
            self.quiet_move_history[i].fill(0);
        }
        self.transposition_table.clear();
        self.stats.reset();
    }

    pub fn get_best_move(&mut self, game: &mut Game) -> ChessMove {

        let mut score = 0;
        for i in 0..self.quiet_move_history.len() {
            for j in 0..self.quiet_move_history[i].len() {
                self.quiet_move_history[i][j] /= 8;
            }
        }

        for i in 0..self.killer_moves.len() {
            self.killer_moves[i] = chess_move::NULL_MOVE;
        }

        for max_depth in 1..(self.settings.max_depth + 1) {
            let mut window = 40;
            
            println!("Depth: {}", max_depth);
            loop {
                let alpha = score - window;
                let beta = score + window;
    
                score = self.search(0, max_depth, alpha, beta, true, game);
                
                println!("Alpha: {} Beta: {} Score: {}", alpha, beta, score);
                if alpha < score && score < beta {
                    break;
                }

                window *= 2;
            }
        }

        return self.transposition_table[&game.get_board().get_zoberist_hash()].best_move;
    }

    pub fn search(&mut self, ply: i8, depth_left: i8, mut alpha: i32, beta: i32, null_allowed: bool, game: &mut Game) -> i32 {
        let gs = game.get_game_state();

        if gs.is_draw() {
            return 0;
        }

        if gs.is_checkmate() {
            return -1_000_000_000 + ply as i32;
        }

        if depth_left <= 0 {
            return self.quiescence_search(ply, depth_left, alpha, beta, game);
        }

        let in_check = game.get_board().in_check();
        let zkey = game.get_board().get_zoberist_hash();
        let is_pawn_endgame = game.get_board().is_only_pawns();

        let tt_entry = if self.transposition_table.contains_key(&zkey) {
            self.transposition_table[&zkey].clone()
        }
        else {
            TTEntry {
                depth: 0,
                score: 0,
                best_move: chess_move::NULL_MOVE,
                node_type: NodeType::Unknown,
            }
        };

        //alpha < score < beta
        if alpha == beta -1 && tt_entry.depth >= depth_left && 
            match tt_entry.node_type {
                NodeType::Exact         => true,
                NodeType::LowerBound    => tt_entry.score >= beta,
                NodeType::UpperBound    => tt_entry.score <= alpha,
                NodeType::Unknown       => false,
            } {
            return tt_entry.score;
        }


        let mut local_score = evaluation::static_eval_int(&game.get_board(), &self.settings.eval_factors);
        
        if match tt_entry.node_type {
            NodeType::Exact         => true,
            NodeType::LowerBound    => tt_entry.score > local_score,
            NodeType::UpperBound    => tt_entry.score < local_score,
            NodeType::Unknown       => false,
        } {
            local_score = tt_entry.score;
        }

        //Internal iterative reductions
        //if tt_entry.node_type == NodeType::Unknown && depth_left > 3 {
            //depth_left -= 1;
        //}



        let do_pruning = alpha == beta - 1 && !in_check;

        if do_pruning {
            //Reverse futility pruning 
            if depth_left < 7 && local_score - 750 * depth_left as i32 > beta {
                return local_score;
            }

            //Null move pruning
            if null_allowed && local_score >= beta && depth_left > 2 && !in_check {
                game.make_move(chess_move::NULL_MOVE);
                
                let r = -self.search(ply + 1, depth_left - 3, -beta, -alpha, false, game);

                game.undo_move();

                if r >= beta {
                    return beta;
                }
            }
        }

        let mut moves = game.get_legal_moves();

        //[Todo] try no caching
        moves.sort_by_cached_key(|cm| {
            if cm == &tt_entry.best_move {
                return 1_u64 << 60;
            }

            if cm.is_direct_capture() {
                return (1_u64 << 50) * piece_value(cm.capture_piece_type) - piece_value(cm.move_piece_type);
            }

            if cm == &self.killer_moves[ply as usize] {
                return 1_u64 << 49;
            }

            return self.quiet_move_history[cm.start_square as usize][cm.target_square as usize];

            fn piece_value (cpt: ColoredPieceType) -> u64 {
                return PieceType::from_cpt(cpt) as u64;
            }
        });
        
        let mut best_score = -2_000_000_000;
        let mut best_move = chess_move::NULL_MOVE;        

        let mut node_type = NodeType::UpperBound;
        let mut quiets_evaluated = ArrayVec::<ChessMove, 200>::new();
        let mut moves_evaluated = 0;

        for m in moves {
            game.make_move(m);

            let is_quiet = !m.is_capture() 
                && !m.is_promotion()
                && !game.get_board().in_check();

            let mut full_search = moves_evaluated == 0;


            local_score = -self.search(ply + 1, depth_left - 1, -beta, -alpha, true, game);
         

            game.undo_move();

            moves_evaluated += 1;

            if local_score > best_score {
                best_score = local_score;
                
                if ply == 0 {
                    println!("Best move: {} Score: {}", m.get_board_name(&game.get_board()), best_score);
                }

                if local_score > alpha {
                    best_move = m;
                    
                    alpha = local_score;

                    node_type = NodeType::Exact;

                    if local_score >= beta {
                        node_type = NodeType::LowerBound;

                        if is_quiet {
                            self.quiet_move_history[m.start_square as usize][m.target_square as usize] += depth_left as u64 * depth_left as u64;

                            for qm in quiets_evaluated {
                                self.quiet_move_history[qm.start_square as usize][qm.target_square as usize] -= depth_left as u64 * depth_left as u64;
                            }

                            self.killer_moves[ply as usize] = m;
                        }
                        break;
                    }
                }
            }
                    
            if is_quiet {
                quiets_evaluated.push(m);
            }

            //Late move pruning
            if do_pruning && quiets_evaluated.len() > 3 + depth_left as usize * depth_left as usize {
                break;
            }
        }

        self.transposition_table.insert(zkey, TTEntry {
            depth: depth_left,
            score: best_score,
            best_move: best_move,
            node_type: node_type,
        });

        return best_score;
    }

    pub fn in_check_search(&mut self, ply: i8, depth_left: i8, mut alpha: i32, game: &mut Game) {
        
    }

    pub fn quiescence_search(&mut self, ply: i8, depth_left: i8, mut alpha: i32, beta: i32, game: &mut Game) -> i32 {
        let gs = game.get_game_state();

        if gs.is_draw() {
            return 0;
        }

        if gs.is_checkmate() {
            return -1_000_000_000 + ply as i32;
        }

        let in_check = game.get_board().in_check();
        let zkey = game.get_board().get_zoberist_hash();
        let is_pawn_endgame = game.get_board().is_only_pawns();

        let tt_entry = if self.transposition_table.contains_key(&zkey) {
            self.transposition_table[&zkey].clone()
        }
        else {
            TTEntry {
                depth: 0,
                score: 0,
                best_move: chess_move::NULL_MOVE,
                node_type: NodeType::Unknown,
            }
        };

        //alpha < score < beta
        if alpha == beta -1 && tt_entry.depth >= depth_left && 
            match tt_entry.node_type {
                NodeType::Exact         => true,
                NodeType::LowerBound    => tt_entry.score >= beta,
                NodeType::UpperBound    => tt_entry.score <= alpha,
                NodeType::Unknown       => false,
            } {
            return tt_entry.score;
        }


        let mut local_score = evaluation::static_eval_int(&game.get_board(), &self.settings.eval_factors);
        
        if match tt_entry.node_type {
            NodeType::Exact         => true,
            NodeType::LowerBound    => tt_entry.score > local_score,
            NodeType::UpperBound    => tt_entry.score < local_score,
            NodeType::Unknown       => false,
        } {
            local_score = tt_entry.score;
        }

        //Internal iterative reductions
        //if tt_entry.node_type == NodeType::Unknown && depth_left > 3 {
            //depth_left -= 1;
        //}
        
        //Quiesence only
        if local_score >= beta {
            return local_score;
        }

        if local_score > alpha {
            alpha = local_score;
        }

        let mut moves = game.get_legal_moves();

        //[Todo] try no caching
        moves.sort_by_cached_key(|cm| {
            if cm == &tt_entry.best_move {
                return 1_u64 << 60;
            }

            if cm.is_direct_capture() {
                return (1_u64 << 50) * 
                    (piece_value(cm.capture_piece_type) - piece_value(cm.move_piece_type));
            }

            if cm == &self.killer_moves[ply as usize] {
                return 1_u64 << 49;
            }

            return self.quiet_move_history[cm.start_square as usize][cm.target_square as usize];

            fn piece_value (cpt: ColoredPieceType) -> u64 {
                return PieceType::from_cpt(cpt) as u64;
            }
        });
        
        let mut best_score = local_score;         //Quiesence only
        let mut best_move = chess_move::NULL_MOVE;        

        let mut node_type = NodeType::UpperBound;
        let mut quiets_evaluated = ArrayVec::<ChessMove, 200>::new();
        let mut moves_evaluated = 0;

        for m in moves {
            //Quiesence only
            if !m.is_capture() {
                continue;
            }

            game.make_move(m);


            local_score = -self.quiescence_search(ply + 1, depth_left - 1, -beta, -alpha, game);
            

            game.undo_move();

            moves_evaluated += 1;

            if local_score > best_score {
                best_score = local_score;
                
                if local_score > alpha {
                    best_move = m;
                    
                    alpha = local_score;

                    node_type = NodeType::Exact;

                    if local_score >= beta {
                        node_type = NodeType::LowerBound;


                        break;
                    }
                }
            }
        }

        /* 
        self.transposition_table.insert(zkey, TTEntry {
            depth: depth_left,
            score: best_score,
            best_move: best_move,
            node_type: node_type,
        });
        */


        return best_score;
    }
}