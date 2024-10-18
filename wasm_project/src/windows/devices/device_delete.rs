use std::sync::{Arc, Mutex};
use egui::Align2;
use crate::app::State;


pub fn render_device_delete(ctx: &egui::Context, state: Arc<Mutex<State>>) {
    let deleting_device = state.lock().unwrap().deleting_device.clone();
    if let Some((device_ip, device_name)) = deleting_device {
        let mut add_device_dialog_open = true;
        egui::Window::new("Delete Device")
            .open(&mut add_device_dialog_open)
            .anchor(Align2::CENTER_CENTER, [0., 0.])
            .show(ctx, |ui| {
                ui.label(&device_name);
                ui.horizontal(|ui| {
                    if ui.button("Delete Device").clicked() {
                        let mut state = state.lock().unwrap();
                        state.devices_has_changed = true;
                        if let Some(current) = state.current_device.clone() {
                            if current.eq(&device_ip) {
                                state.current_device = None;
                            }
                        }
                        state.deleting_device = None;
                        let mut devices = state.devices.lock().unwrap();
                        devices.retain(|k,_v|k.ne(&device_ip));
                    }
                    if ui.button("Cancel").clicked() {
                        state.lock().unwrap().deleting_device = None;
                    }
                });
            });
    }
}