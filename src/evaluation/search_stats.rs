pub struct SearchStats {
    pub nodes: u64,
    pub evals: u64,
}
impl SearchStats {
    pub fn new() -> Self {
        SearchStats {
            nodes: 0,
            evals: 0,
        }
    }

    pub fn add(&mut self, other: &SearchStats) {
        self.nodes += other.nodes;
        self.evals += other.evals;
    }

    pub fn print(&self) {
        println!("Nodes: {:3}", self.nodes);
        println!("Evals: {:3}", self.evals);
    }
}