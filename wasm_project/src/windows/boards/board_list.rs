use std::sync::{Arc, Mutex};

use egui::{Align2, Color32, RichText};

use crate::{app::State, post::post};

pub fn render_board_list(ctx: &egui::Context, state: Arc<Mutex<State>>, board_editor_open: &mut bool) {
    egui::Window::new("Boards")
        .anchor(Align2::LEFT_TOP, [5., 5.])
        .show(ctx, |ui| {
            let boards = state.lock().unwrap().boards.lock().unwrap().clone();
            for (board_name, _) in boards.iter() {
                if ui.button(board_name).clicked() {
                    state.lock().unwrap().current_board = Some(board_name.to_string());
                    state.lock().unwrap().current_editor = Some(0);
                    *board_editor_open = true;
                }
            }
            ui.separator();
            if ui.button("Add Board").clicked() {
                state.lock().unwrap().add_board_dialog.open = true;
            }
            let boards_changed = state.lock().unwrap().boards_has_changed;
            if boards_changed {
                if ui.button(RichText::new("Post Changes").color(Color32::RED)).clicked() {
                    post("/api/update/boards", &boards);
                    state.lock().unwrap().boards_has_changed = false;
                }
            }
        });
}