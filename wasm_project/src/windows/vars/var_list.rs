use std::sync::{Arc, Mutex};

use egui::{Align2, Color32, RichText};

use crate::{app::State, post::post};

pub fn render_var_list(ctx: &egui::Context, state: Arc<Mutex<State>>, var_editor_open: &mut bool) {
    let mut window_height = ctx.screen_rect().height();
    window_height-=100.;
    window_height/=2.;
    egui::Window::new("Variables")
        .anchor(Align2::LEFT_BOTTOM, [5., -5.])
        .min_height(window_height)
        .max_height(window_height)
        .scroll([false, true])
        .movable(false)
        .show(ctx, |ui| {
            let vars = state.lock().unwrap().vars.lock().unwrap().clone();
            let mut keys = vars.keys().collect::<Vec<&String>>();
            keys.sort();
            for var_name in keys {
                ui.horizontal(|ui| {
                    if ui.button("🗑").clicked() {
                        state.lock().unwrap().deleting_var = Some(var_name.to_owned());
                    }
                    if ui.button(var_name).clicked() {
                        state.lock().unwrap().current_var = Some(var_name.to_string());
                        state.lock().unwrap().current_editor = Some(1);
                        *var_editor_open = true;
                    }
                });
            }
            {
                let vars_changed = state.lock().unwrap().vars_has_changed;
                let mut spacer = ui.available_height();
                spacer -= 30.;
                if vars_changed {
                    spacer -= 30.;
                }
                if spacer > 0. {
                    ui.add_space(spacer);
                }
                ui.separator();
                if ui.button("Add var").clicked() {
                    state.lock().unwrap().add_var_dialog.open = true;
                }
                if vars_changed {
                    if ui.button(RichText::new("Post Changes").color(Color32::RED)).clicked() {
                        post("/api/update/vars", &vars);
                        state.lock().unwrap().vars_has_changed = false;
                    }
                }
            }
        });
}