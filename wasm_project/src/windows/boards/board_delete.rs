use std::sync::{Arc, Mutex};
use egui::Align2;
use crate::app::State;


pub fn render_board_delete(ctx: &egui::Context, state: Arc<Mutex<State>>) {
    let deleting_board = state.lock().unwrap().deleting_board.clone();
    if let Some(board_name) = deleting_board {
        let mut add_board_dialog_open = true;
        egui::Window::new("Delete Board")
            .open(&mut add_board_dialog_open)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .show(ctx, |ui| {
                ui.label(&board_name);
                ui.horizontal(|ui| {
                    if ui.button("Delete Board").clicked() {
                        let mut state = state.lock().unwrap();
                        state.boards_has_changed = true;
                        if let Some(current) = state.current_board.clone() {
                            if current.eq(&board_name) {
                                state.current_board = None;
                            }
                        }
                        state.deleting_board = None;
                        let mut boards = state.boards.lock().unwrap();
                        boards.retain(|k,_v|k.ne(&board_name));
                    }
                    if ui.button("Cancel").clicked() {
                        state.lock().unwrap().deleting_board = None;
                    }
                });
            });
    }
}