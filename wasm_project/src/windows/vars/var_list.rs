use std::sync::{Arc, Mutex};

use egui::{Align2, Color32, RichText};

use crate::{app::State, post::post};

pub fn render_var_list(ctx: &egui::Context, state: Arc<Mutex<State>>, var_editor_open: &mut bool) {
    egui::Window::new("Variables")
        .anchor(Align2::LEFT_BOTTOM, [5., -5.])
        .show(ctx, |ui| {
            let vars = state.lock().unwrap().vars.lock().unwrap().clone();
            for (var_name, _) in vars.iter() {
                if ui.button(var_name).clicked() {
                    state.lock().unwrap().current_var = Some(var_name.to_string());
                    state.lock().unwrap().current_editor = Some(1);
                    *var_editor_open = true;
                }
            }
            ui.separator();
            if ui.button("Add var").clicked() {
                state.lock().unwrap().add_var_dialog.open = true;
            }
            let vars_changed = state.lock().unwrap().vars_has_changed;
            if vars_changed {
                if ui.button(RichText::new("Post Changes").color(Color32::RED)).clicked() {
                    post("/api/update/vars", &vars);
                    state.lock().unwrap().vars_has_changed = false;
                }
            }
        });
}