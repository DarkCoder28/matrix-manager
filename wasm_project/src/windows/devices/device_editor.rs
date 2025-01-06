use std::
    sync::{Arc, Mutex}
;
use regex::Regex;
use egui::{Align2, Ui};
use shared::device_config::{Brightness, DeviceConfig, DeviceConfigs};

use crate::app::State;

pub fn render_device_editor(
    ctx: &egui::Context,
    state: Arc<Mutex<State>>,
    device_editor_open: &mut bool,
) {
    if !*device_editor_open || state.lock().unwrap().current_device.is_none() {
        return;
    }
    egui::Window::new("Device Editor")
        .open(device_editor_open)
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .default_height(ctx.screen_rect().height()*0.75)
        .max_height(ctx.screen_rect().height()*0.75)
        .min_width(ctx.screen_rect().width()*0.2)
        .scroll([true, false])
        .show(ctx, |ui| {
            let current_device = state.lock().unwrap().current_device.clone();
            let devices = state.lock().unwrap().devices.clone();
            let mut devices = devices.lock().unwrap();
            let device_ip = current_device.unwrap();
            //
            render_name_editor(ui, &device_ip, &mut devices, state.clone());
            ui.horizontal(|ui| {
                render_size(ui, &device_ip, &devices);
                render_proto_version(ui, &device_ip, &devices);
                render_potd_brightness_editor(ui, &device_ip, &mut devices, state.clone());
            });
            render_temperature_colours_editor(ui, &device_ip, &mut devices, state.clone());
            render_brightness_editor(ui, &device_ip, &mut devices, state.clone());
            render_board_list_editor(ui, &device_ip, &mut devices, state.clone());
            //
            render_config_panel(ctx, &devices.get(&device_ip).unwrap());
        });
}

fn render_name_editor(
    ui: &mut Ui,
    device_ip: &str,
    devices: &mut DeviceConfigs,
    state: Arc<Mutex<State>>,
) {
    ui.group(|ui| {
        ui.label("Name");
        let name = &mut devices.get_mut(device_ip).unwrap().name;
        let mut name_edit = name.clone();
        ui.text_edit_singleline(&mut name_edit);
        if name_edit.ne(&*name) {
            *name = name_edit;
            state.lock().unwrap().devices_has_changed = true;
        }
    });
}

fn render_size(ui: &mut Ui, device_ip: &str, devices: &DeviceConfigs) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label("Board Size");
            ui.indent("", |ui| {
                let size = devices.get(device_ip).unwrap().size;
                ui.label(format!("Width: {}", size.0));
                ui.label(format!("Height: {}", size.1));
            });
        });
    });
}

fn render_proto_version(ui: &mut Ui, device_ip: &str, devices: &DeviceConfigs) {
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label(format!("Protocol"));
            ui.indent("227efe8e-7838-4601-9b2d-6e26e3b8803b", |ui| {
                ui.label(format!("Version: {}", devices.get(device_ip).unwrap().proto_version));
            });
        });
    });
}

fn render_temperature_colours_editor(
    ui: &mut Ui,
    device_ip: &str,
    devices: &mut DeviceConfigs,
    state: Arc<Mutex<State>>,
) {
    ui.group(|ui| {
        ui.label("Temperature Colours Editor");
        let device = devices.get_mut(device_ip).unwrap();
        ui.indent("808c51b6-7faa-4566-8abd-17f6efb14ca0", |ui| {
            render_colour_editor(
                ui,
                "Cold",
                &mut device.temperature_colours.cold,
                state.clone(),
            );
            render_colour_editor(
                ui,
                "Freezing",
                &mut device.temperature_colours.freezing,
                state.clone(),
            );
            render_colour_editor(
                ui,
                "Neutral",
                &mut device.temperature_colours.neutral,
                state.clone(),
            );
            render_colour_editor(
                ui,
                "Warm",
                &mut device.temperature_colours.warm,
                state.clone(),
            );
        });
    });
}

fn render_potd_brightness_editor(ui: &mut Ui, device_ip: &str, devices: &mut DeviceConfigs, state: Arc<Mutex<State>>) {
    ui.vertical(|ui| {
        ui.group(|ui| {
            ui.label("Picture of The Day");
            ui.indent("a2fd0174-63e4-48da-841d-0b31dc9f7d01", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Brightness Threshold:");
                    let device = devices.get_mut(device_ip).unwrap();
                    let mut threshold_edit = device.skip_brightness_threshold.to_string();
                    ui.text_edit_singleline(&mut threshold_edit);
                    if threshold_edit.ne(&device.skip_brightness_threshold.to_string()) {
                        if threshold_edit.len() <= 0 {
                            threshold_edit = 0.to_string();
                        }
                        if let Ok(num) = threshold_edit.parse::<u8>() {
                            device.skip_brightness_threshold = num;
                            state.lock().unwrap().devices_has_changed = true;
                        }
                    }
                });
            });
        });
    });
}

fn render_colour_editor(
    ui: &mut Ui,
    colour_name: &str,
    colour: &mut i16,
    state: Arc<Mutex<State>>,
) {
    ui.horizontal(|ui| {
        ui.label(format!("{}: ", colour_name));
        let mut colour_edit = colour.to_string();
        ui.text_edit_singleline(&mut colour_edit);
        if colour_edit.ne(&colour.to_string()) {
            if colour_edit.len() <= 0 {
                colour_edit = 0.to_string();
            }
            if let Ok(col) = colour_edit.parse::<i16>() {
                *colour = col;
                state.lock().unwrap().devices_has_changed = true;
            }
        }
    });
}

fn render_board_list_editor(ui: &mut Ui, device_ip: &str, devices: &mut DeviceConfigs, state: Arc<Mutex<State>>) {
    ui.group(|ui| {
        ui.label("Boards:");
        ui.horizontal_wrapped(|ui| {
            let device = devices.get_mut(device_ip).unwrap();
            let boards = device.boards.clone();
            for (id, board) in boards.iter().enumerate() {
                match render_board_list_item(ui, &board, state.clone()) {
                    Consequences::None => {},
                    Consequences::Delete => {
                        device.boards.remove(id);
                    },
                    Consequences::MoveUp => {
                        if id > 0 {
                            device.boards.swap(id, id-1);
                        }
                    },
                    Consequences::MoveDown => {
                        if id < boards.len()-1 {
                            device.boards.swap(id, id+1);
                        }
                    },
                }
            }
        });
        ui.separator();
        let mut selection = String::from("Add Board");
        egui::ComboBox::from_id_salt("85e1a822-f2d2-47b0-9e5b-defd004264cd")
            .selected_text(&selection)
            .show_ui(ui, |ui| {
                let state = state.lock().unwrap();
                let boards = state.boards.lock().unwrap();
                for board in boards.keys() {
                    ui.selectable_value(&mut selection, board.clone(), board.clone());
                }
            });
        {
            if selection.ne("Add Board") {
                let device = devices.get_mut(device_ip).unwrap();
                device.boards.push(selection);
                state.lock().unwrap().devices_has_changed = true;
            }
        }
    });
}

#[derive(Debug)]
enum Consequences {
    None, Delete, MoveUp, MoveDown
}
fn render_board_list_item(ui: &mut Ui, name: &str, state: Arc<Mutex<State>>) -> Consequences {
    let mut consequence = Consequences::None;
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label(name);
            ui.set_width(64.);
            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    consequence = Consequences::MoveUp;
                    state.lock().unwrap().devices_has_changed = true;
                }
                if ui.button("Delete").clicked() {
                    consequence = Consequences::Delete;
                    state.lock().unwrap().devices_has_changed = true;
                }
                if ui.button(">").clicked() {
                    consequence = Consequences::MoveDown;
                    state.lock().unwrap().devices_has_changed = true;
                }
            });
        });
    });
    return consequence;
}

fn render_brightness_editor(ui: &mut Ui, device_ip: &str, devices: &mut DeviceConfigs, state: Arc<Mutex<State>>) {
    ui.group(|ui| {
        ui.collapsing("Brightness", |ui| {
            let device = devices.get_mut(device_ip).unwrap();
            let mut brightnesses = device.brightness.clone();
            brightnesses.sort_by(|a,b| {
                let time_a = a.time.split_once(':').unwrap();
                let time_b = b.time.split_once(':').unwrap();
                let time_a_hr = time_a.0.parse::<u8>().unwrap_or(0);
                let time_b_hr = time_b.0.parse::<u8>().unwrap_or(0);
                let time_a_min = time_a.1.parse::<u8>().unwrap_or(0);
                let time_b_min = time_b.1.parse::<u8>().unwrap_or(0);
                let order_hr = time_a_hr.cmp(&time_b_hr);
                if order_hr.is_eq() {
                    return time_a_min.cmp(&time_b_min);
                }
                return order_hr;
            });
            for (idx, time) in brightnesses.iter().enumerate() {
                let result = render_brightness_item_editor(ui, &time, state.clone());
                match result {
                    BrightnessConsequences::None => {},
                    BrightnessConsequences::ModTime(new_time) => {
                        let brightness = device.brightness.get_mut(idx).unwrap();
                        brightness.time = new_time;
                    },
                    BrightnessConsequences::ModPercent(new_brightness) => {
                        let brightness = device.brightness.get_mut(idx).unwrap();
                        brightness.percentage = new_brightness;
                    },
                    BrightnessConsequences::Delete => {
                        device.brightness.remove(idx);
                        return;
                    },
                }
            }
            ui.separator();
            if ui.button("Add Brightness Time").clicked() {
                device.brightness.push(Brightness::default());
                state.lock().unwrap().devices_has_changed = true;
            }
        });
    });
}

#[derive(Debug)]
enum BrightnessConsequences {
    None, ModTime(String), ModPercent(u8), Delete
}

fn render_brightness_item_editor(ui: &mut Ui, time: &Brightness, state: Arc<Mutex<State>>) -> BrightnessConsequences {
    let mut result = BrightnessConsequences::None;
    ui.horizontal(|ui| {
        let the_time = parse_time(&time.time).unwrap();
        let mut time_edit = format!("{}:{}", the_time.0, the_time.1);
        let time_editor = egui::TextEdit::singleline(&mut time_edit)
            .desired_width(32.)
            .char_limit(5);
        ui.add(time_editor);
        let mut brightness_edit = time.percentage.to_string();
        let brightness_editor = egui::TextEdit::singleline(&mut brightness_edit)
            .desired_width(24.)
            .char_limit(3);
        ui.add(brightness_editor);
        //
        if time_edit.ne(&format!("{}:{}", the_time.0, the_time.1)) {
            if let Some(time) = parse_time(&time_edit) {
                let new_time = format!("{:<02}:{:<02}", time.0, time.1);
                result = BrightnessConsequences::ModTime(new_time);
                state.lock().unwrap().devices_has_changed = true;
            }
        }
        //
        if brightness_edit.ne(&time.percentage.to_string()) {
            if let Ok(mut num) = brightness_edit.parse::<u8>() {
                if num > 100 {
                    num = 100;
                }
                result = BrightnessConsequences::ModPercent(num);
                state.lock().unwrap().devices_has_changed = true;
            }
        }
        //
        if ui.button("❌").clicked() {
            result = BrightnessConsequences::Delete;
            state.lock().unwrap().devices_has_changed = true;
        }
    });
    result
}

fn parse_time(time: &str) -> Option<(u8, u8)> {
    let regex = Regex::new(r"^(?P<hour>\d{1,2}):(?P<minute>\d{1,2})$").unwrap();
    let res = regex.captures(&time);
    if res.is_some() {
        let res = res.unwrap();
        let hr = &res["hour"];
        let hr_num = hr.parse::<u8>().unwrap();
        if hr_num > 24 {
            return None;
        }
        let min = &res["minute"];
        let min_num = min.parse::<u8>().unwrap();
        if min_num > 60 {
            return None;
        }   
        return Some((hr_num, min_num));
    }
    None
}

fn render_config_panel(ctx: &egui::Context, device: &DeviceConfig) {
    let mut window_height = ctx.screen_rect().height();
    window_height-=120.;
    window_height/=2.;
    egui::Window::new("Device Config")
        .anchor(Align2::RIGHT_TOP, [-5.0, 5.0])
        .min_height(window_height)
        .max_height(window_height)
        .scroll([false, true])
        .movable(false)
        .show(ctx, |ui| {
            ui.label(serde_json::to_string(device).unwrap());
        });
}
