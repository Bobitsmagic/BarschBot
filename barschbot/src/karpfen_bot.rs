use std::{cmp::min, collections::{btree_map::Values, HashMap}};

use arrayvec::ArrayVec;

use crate::{bb_settings, bit_board::BitBoard, chess_move::{self, ChessMove}, colored_piece_type::ColoredPieceType, endgame_table::{self, EndgameTable}, evaluation, game::{Game, GameState}, kb_settings::{self, KBSettings}, opening_book::OpeningBook, piece_type::PieceType, search_stats::SearchStats, square::{self, Square}};

#[derive(PartialEq, Eq, Clone, Copy)]
enum NodeType {
    Exact,
    LowerBound,
    UpperBound,
    Unknown,
}

#[derive(Clone)]
struct TTEntry {
    depth: u8,
    score: i32,
    best_move: ChessMove,
    node_type: NodeType,
}

pub struct KarpfenBot {
    stats: SearchStats,
    quiet_move_history: [[u64; 64]; 64],
    killer_moves: [ChessMove; 256],
    transposition_table: HashMap<u64, TTEntry>,
    settings: KBSettings,
    root_move: ChessMove,
}

const CHECKMATE_VALUE: i32 = 100_000;
const DO_PRINT: bool = false;

impl KarpfenBot {
    pub fn new() -> KarpfenBot {
        return KarpfenBot {
            stats: SearchStats::new(),
            quiet_move_history: [[0; 64]; 64],
            killer_moves: [chess_move::NULL_MOVE; 256],
            transposition_table: HashMap::new(),
            settings: kb_settings::STANDARD_KB_SETTINGS,
            root_move: chess_move::NULL_MOVE,
        };
    }

    pub fn with_settings(settings: KBSettings) -> KarpfenBot {
        return KarpfenBot {
            stats: SearchStats::new(),
            quiet_move_history: [[0; 64]; 64],
            killer_moves: [chess_move::NULL_MOVE; 256],
            transposition_table: HashMap::new(),
            settings: settings,
            root_move: chess_move::NULL_MOVE,
        };
    }   

    pub fn reset(&mut self) {
        for i in 0..64 {
            self.quiet_move_history[i].fill(0);
        }
        self.transposition_table.clear();
        self.killer_moves.fill(chess_move::NULL_MOVE);
        self.root_move = chess_move::NULL_MOVE;
        self.stats.reset();
    }

    pub fn get_best_move(&mut self, game: &mut Game, opening_book: &OpeningBook, endgame_table: &EndgameTable) -> ChessMove {
        let om = opening_book.get_move(game.get_board().get_zoberist_hash());

        if om != chess_move::NULL_MOVE {
            //println!("Book move");
            return om;
        }

        self.root_move == chess_move::NULL_MOVE;

        let moves = game.get_legal_moves();
        if moves.len() == 1 {
            return moves[0];
        }

        let mut score = 0;
        for i in 0..self.quiet_move_history.len() {
            for j in 0..self.quiet_move_history[i].len() {
                self.quiet_move_history[i][j] /= 8;
            }
        }

        for i in 0..self.killer_moves.len() {
            self.killer_moves[i] = chess_move::NULL_MOVE;
        }

        let start = std::time::Instant::now();
        let mut max_depth = 1;
        loop {
            let mut window = 220;
            
            if DO_PRINT {
                println!("Depth: {}", max_depth);
            }
            loop {
                let alpha = score - window;
                let beta = score + window;
    
                if DO_PRINT {
                    println!("\t[{}, {}]", alpha, beta);
                }

                score = self.search(0, max_depth, alpha, beta, true, game, endgame_table);
                
                window *= 2;

                if score.abs() > CHECKMATE_VALUE - 100 {
                    if DO_PRINT {
                        println!("Checkmate found   ");
                    }
                    break;
                }

                if alpha < score && score < beta {
                    break;
                }
            }


            let elapsed = start.elapsed().as_millis();

            if DO_PRINT {
                println!("Elapsed time: {}ms", elapsed);
            }

            if max_depth >= self.settings.max_depth && elapsed as u64 >= self.settings.min_search_time {
                break;
            }

            max_depth += 1;
        }

        return self.root_move;
    }

    pub fn search(&mut self, ply: i8, depth_left: u8, mut alpha: i32, beta: i32, null_allowed: bool, game: &mut Game, endgame_table: &EndgameTable) -> i32 {
        let gs = game.get_game_state();

        if gs.is_draw() {
            return 0;
        }

        if gs.is_checkmate() {
            return -CHECKMATE_VALUE + ply as i32;
        }

        let pair = get_relative_endgame_eval(&game.get_board(), endgame_table);
        if pair.1 != GameState::Undecided && ply > 0 {
            return pair.0;
        }

        if depth_left == 0 {
            return self.quiescence_search(ply, alpha, beta, game, endgame_table);
        }

        let min_window_search = alpha == beta - 1;

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

        //When doing a min_window_search it is sufficient to know whether we fail high or low
        //alpha < score < beta
        if min_window_search && tt_entry.depth >= depth_left && 
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

        //Null move pruning
        if null_allowed && local_score >= beta && depth_left >= 3 && !in_check && !game.get_board().is_only_pawns() {
            game.make_move(chess_move::NULL_MOVE);
            
            let r = -self.search(ply + 1, depth_left - 3, -beta, -beta + 1, false, game, endgame_table);

            game.undo_move();

            if r >= beta {
                return beta;
            }
        }

        let mut moves = game.get_legal_moves();
        //[Todo] try no caching
        moves.sort_by_cached_key(|cm| {
            if *cm == tt_entry.best_move {
                return 1_u64 << 60;
            }

            if cm.is_direct_capture() {
                return (1_u64 << 50) * piece_value(cm.capture_piece_type) - piece_value(cm.move_piece_type);
            }

            if cm.is_promotion() || cm.is_en_passant() {
                return 1_u64 << 45;
            }

            if *cm == self.killer_moves[ply as usize] {
                return 1_u64 << 40;
            }

            return self.quiet_move_history[cm.start_square as usize][cm.target_square as usize];

            fn piece_value (cpt: ColoredPieceType) -> u64 {
                const VALUES: [u64; 6] = [1, 3, 3, 5, 11, 100];
                return VALUES[PieceType::from_cpt(cpt) as usize];
            }
        });
        
        let mut best_score = -2_000_000_000;
        let mut best_move = chess_move::NULL_MOVE;        

        let mut node_type = NodeType::UpperBound;
        let mut quiets_evaluated = ArrayVec::<ChessMove, 200>::new();

        for m in moves.iter().rev() {
            let m = *m;
            game.make_move(m);

            let m_in_check = game.get_board().in_check();
            let is_quiet = !m.is_capture() 
                && !m.is_promotion();

            let reduction = if m_in_check { 0 } else { 1 };

            local_score = -self.search(ply + 1, depth_left - reduction, -beta, -alpha, true, game, endgame_table);
         
            game.undo_move();

            if local_score > best_score {
                best_score = local_score;
                
                if ply == 0 {
                    self.root_move = m;

                    if DO_PRINT {
                        println!("\t\tBest move: {} Score: {}", m.get_board_name(&game.get_board()), best_score);
                    }
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
                                let reduction = depth_left as u64 * depth_left as u64;
                                let val = self.quiet_move_history[qm.start_square as usize][qm.target_square as usize];
                                self.quiet_move_history[qm.start_square as usize][qm.target_square as usize] -= min(val, reduction);
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
        }

        if depth_left >= tt_entry.depth && (node_type == NodeType::Exact ||
            tt_entry.node_type == NodeType::Unknown || 
            tt_entry.node_type == node_type) {
            
            self.transposition_table.insert(zkey, TTEntry {
                depth: depth_left,
                score: best_score,
                best_move: best_move,
                node_type: node_type,
            });
        }

        return best_score;
    }

    /* 
    pub fn in_check_search(&mut self, ply: i8, depth_left: i8, mut alpha: i32, game: &mut Game) {
        
    }
    */

    pub fn quiescence_search(&mut self, ply: i8, mut alpha: i32, beta: i32, game: &mut Game, endgame_table: &EndgameTable) -> i32 {
        let gs = game.get_game_state();

        if gs.is_draw() {
            return 0;
        }

        if gs.is_checkmate() {
            return -CHECKMATE_VALUE + ply as i32;
        }

        let pair = get_relative_endgame_eval(&game.get_board(), endgame_table);
        if pair.1 != GameState::Undecided {
            return pair.0;
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
        if alpha == beta -1 &&
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

        for m in moves {
            //Quiesence only
            if !m.is_capture() && !in_check {
                continue;
            }

            game.make_move(m);

            local_score = -self.quiescence_search(ply + 1, -beta, -alpha, game, endgame_table);
            
            game.undo_move();

            if local_score > best_score {
                best_score = local_score;
                
                if local_score > alpha {
                    best_move = m;
                    
                    alpha = local_score;

                    if local_score >= beta {

                        break;
                    }
                }
            }
        }

        return best_score;
    }
}

pub fn get_relative_endgame_eval(board: &BitBoard, table: &EndgameTable) -> (i32, GameState) {
    if board.get_all_piece_count() <= table.max_piece_count as u32 {

        //println!("This should not happen {}", table.max_piece_count);
        let score = table.get_score(&board);
        
        if score == -128 {
            return (0, GameState::Undecided);
        }

        let mut res = 0;
        let mut gs = GameState::Undecided;
        if score == 0 {
            res = 0;
            gs = GameState::InsuffMaterial;
        }
        else {
            gs = if score > 0 { GameState::BlackCheckmate } else { GameState::WhiteCheckmate };

            if board.is_whites_turn() {
                res = if score > 0 { CHECKMATE_VALUE } else { -CHECKMATE_VALUE };
            }
            else {
                res = if score < 0 { CHECKMATE_VALUE } else { -CHECKMATE_VALUE };
            }

            for i in 0..(127 - score.abs()) {
                res -= 1;
            }
        }

        return (res, gs);
    }

    return (0, GameState::Undecided);
}