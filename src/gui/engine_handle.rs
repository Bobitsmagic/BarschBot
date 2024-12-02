use std::sync::mpsc::{Receiver, Sender};

use crate::moves::{chess_move::ChessMove, uci_move::UciMove};

use super::render_state::RenderState;

pub struct EngineHandle {
    render_reciver: Receiver<RenderState>,
    move_sender: Sender<ChessMove>,
}
impl EngineHandle {
    pub fn new(render_reciver: Receiver<RenderState>, move_sender: Sender<ChessMove>) -> Self {
        EngineHandle {
            render_reciver,
            move_sender,
        }
    }

    pub fn recive_render_state(&self) -> Option<RenderState> {
        if let Ok(render_state) = self.render_reciver.try_recv() {
            Some(render_state)
        } else {
            None
        }
    }

    pub fn send_move(&self, uci_move: ChessMove) {
        self.move_sender.send(uci_move).unwrap();
    }
}