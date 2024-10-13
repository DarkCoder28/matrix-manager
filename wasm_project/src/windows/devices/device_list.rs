use std::sync::{Arc, Mutex};

use egui::{Align2, Color32, RichText, Ui};
use shared::device_config::DeviceConfig;

use crate::{app::State, post::post};

pub fn render_device_list(ctx: &egui::Context, state: Arc<Mutex<State>>, device_editor_open: &mut bool) {
    egui::Window::new("Devices")
        .anchor(Align2::RIGHT_BOTTOM, [-5., -5.])
        .show(ctx, |ui| {
            let devices = state.lock().unwrap().devices.lock().unwrap().clone();
            // Render Default First
            if let Some(device) = devices.get("default") {
                render_device(ui, "default", device, state.clone(), device_editor_open);
            }
            for (device_ip, device_data) in devices.iter() {
                if device_ip.eq("default") {
                    continue;
                }
                render_device(ui, device_ip, device_data, state.clone(), device_editor_open);
            }
            ui.separator();
            if ui.button("Add device").clicked() {
                state.lock().unwrap().add_device_dialog.open = true;
            }
            let devices_changed = state.lock().unwrap().devices_has_changed;
            if devices_changed {
                if ui.button(RichText::new("Post Changes").color(Color32::RED)).clicked() {
                    post("/api/update/devices", &devices);
                    state.lock().unwrap().devices_has_changed = false;
                }
            }
        });
}

fn render_device(ui: &mut Ui, device_ip: &str, device_data: &DeviceConfig, state: Arc<Mutex<State>>, device_editor_open: &mut bool) {
    if ui.button(format!("{} ({})", device_data.name, device_ip)).clicked() {
        state.lock().unwrap().current_device = Some(device_ip.to_string());
        state.lock().unwrap().current_editor = Some(2);
        *device_editor_open = true;
    }
}