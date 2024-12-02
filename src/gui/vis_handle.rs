use std::sync::mpsc::{channel, Receiver, Sender};

use crate::moves::chess_move::ChessMove;

use super::{engine_handle::EngineHandle, render_state::RenderState};

pub struct VisHandle {
    render_sender: Sender<RenderState>,
    move_reciver: Receiver<ChessMove>
}

impl VisHandle {
    fn new(render_sender: Sender<RenderState>, click_reciver: Receiver<ChessMove>) -> Self {
        VisHandle {
            render_sender,
            move_reciver: click_reciver,
        }
    }

    pub fn create_handles() -> (VisHandle, EngineHandle) {
        let (render_sender, render_reciver) = channel();
        let (move_sender, move_reciver) = channel();

        let vis_handle = VisHandle::new(render_sender, move_reciver);
        let engine_handle = EngineHandle::new(render_reciver, move_sender);

        (vis_handle, engine_handle)
    }

    pub fn send_render_state(&self, render_state: RenderState) {
        self.render_sender.send(render_state).unwrap();
    }

    pub fn recive_move(&self) -> ChessMove {
        self.move_reciver.recv().unwrap()
    }
}