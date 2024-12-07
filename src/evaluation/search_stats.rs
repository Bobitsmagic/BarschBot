pub struct SearchStats {
    pub nodes: u64,
    pub evals: u64,
    pub quiessence_nodes: u64,
}
impl SearchStats {
    pub fn new() -> Self {
        SearchStats {
            nodes: 0,
            evals: 0,
            quiessence_nodes: 0,
        }
    }

    pub fn add(&mut self, other: &SearchStats) {
        self.nodes += other.nodes;
        self.evals += other.evals;
        self.quiessence_nodes += other.quiessence_nodes;
    }

    pub fn print(&self) {
        println!("Nodes: {:3}", self.nodes);
        println!("QNodes: {:3}", self.quiessence_nodes);
        println!("Evals: {:3}", self.evals);
    }
}