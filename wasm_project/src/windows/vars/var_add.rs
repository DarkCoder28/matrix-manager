use std::sync::{Arc, Mutex};

use egui::Align2;
use shared::board_variables::BoardVariable;

use crate::app::State;



pub fn render_var_add(ctx: &egui::Context, state: Arc<Mutex<State>>) {
    if !state.lock().unwrap().add_var_dialog.open {
        return;
    }
    let mut add_var_dialog_open = true;
    egui::Window::new("Add var")
        .open(&mut add_var_dialog_open)
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .show(ctx, |ui| {
            let mut add_var_dialog = state.lock().unwrap().add_var_dialog.element_name.clone();
            ui.add(
                egui::TextEdit::singleline(&mut add_var_dialog)
                    .hint_text("Enter var name"),
            );
            state.lock().unwrap().add_var_dialog.element_name = add_var_dialog;
            if ui.button("Add var").clicked() {
                let mut state = state.lock().unwrap();
                state.vars.lock().unwrap().insert(state.add_var_dialog.element_name.clone(), BoardVariable::get_default_by_type("DateTime"));
                state.add_var_dialog.open = false;
                state.add_var_dialog.element_name = String::new();
                state.vars_has_changed = true;
            }
        });
    if !add_var_dialog_open {
        state.lock().unwrap().add_var_dialog.open = false;
    }
}