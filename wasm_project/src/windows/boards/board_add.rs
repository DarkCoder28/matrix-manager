use std::sync::{Arc, Mutex};

use egui::Align2;
use shared::boards::BoardDefinition;

use crate::app::State;



pub fn render_board_add(ctx: &egui::Context, state: Arc<Mutex<State>>) {
    if !state.lock().unwrap().add_board_dialog.open {
        return;
    }
    let mut add_board_dialog_open = true;
    egui::Window::new("Add Board")
        .open(&mut add_board_dialog_open)
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(ctx, |ui| {
            let mut add_board_dialog = state.lock().unwrap().add_board_dialog.element_name.clone();
            ui.add(
                egui::TextEdit::singleline(&mut add_board_dialog)
                    .hint_text("Enter board name"),
            );
            state.lock().unwrap().add_board_dialog.element_name = add_board_dialog;
            if ui.button("Add Board").clicked() {
                let mut state = state.lock().unwrap();
                state.boards.lock().unwrap().insert(state.add_board_dialog.element_name.clone(), BoardDefinition {
                    name: state.add_board_dialog.element_name.clone(),
                    ..Default::default()
                });
                state.add_board_dialog.open = false;
                state.add_board_dialog.element_name = String::new();
                state.boards_has_changed = true;
            }
        });
    if !add_board_dialog_open {
        state.lock().unwrap().add_board_dialog.open = false;
    }
}