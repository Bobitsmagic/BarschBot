pub struct SearchStats {
    pub nodes: u64,
    pub qs: u64,
    pub best_move_hits: u64,
    pub not_best_move_hits: u64,
    pub null_move_prunes: u64,
}

impl SearchStats {
    pub fn new() -> SearchStats {
        return SearchStats { nodes: 0, qs: 0, best_move_hits: 0, not_best_move_hits: 0, null_move_prunes: 0 };
    }

    pub fn reset(&mut self) {
        self.nodes = 0;
        self.qs = 0;
        self.best_move_hits = 0;
        self.not_best_move_hits = 0;
        self.null_move_prunes = 0;
    }
    pub fn print(&self) {
        println!("Nodes: {} Qs: {} BMFM ratio: {} NMP: {}", self.nodes, self.qs, self.best_move_hits as f32 / (self.not_best_move_hits + self.best_move_hits) as f32, self.null_move_prunes);
    }
}