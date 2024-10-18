use std::sync::{Arc, Mutex};
use egui::Align2;
use crate::app::State;


pub fn render_var_delete(ctx: &egui::Context, state: Arc<Mutex<State>>) {
    let deleting_var = state.lock().unwrap().deleting_var.clone();
    if let Some(var_name) = deleting_var {
        let mut add_var_dialog_open = true;
        egui::Window::new("Delete Variable")
            .open(&mut add_var_dialog_open)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .show(ctx, |ui| {
                ui.label(&var_name);
                ui.horizontal(|ui| {
                    if ui.button("Delete Variable").clicked() {
                        let mut state = state.lock().unwrap();
                        state.vars_has_changed = true;
                        if let Some(current) = state.current_var.clone() {
                            if current.eq(&var_name) {
                                state.current_var = None;
                            }
                        }
                        state.deleting_var = None;
                        let mut vars = state.vars.lock().unwrap();
                        vars.retain(|k,_v|k.ne(&var_name));
                    }
                    if ui.button("Cancel").clicked() {
                        state.lock().unwrap().deleting_var = None;
                    }
                });
            });
    }
}