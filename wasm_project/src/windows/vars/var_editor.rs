use std::{str::FromStr, sync::{Arc, Mutex}};

use egui::{Align2, Ui};
use shared::board_variables::{BoardVariable, BoardVariables, TimeData};

use crate::app::State;

pub fn render_var_editor(
    ctx: &egui::Context,
    state: Arc<Mutex<State>>,
    vars_editor_open: &mut bool,
) {
    if !*vars_editor_open || state.lock().unwrap().current_var.is_none() {
        return;
    }
    egui::Window::new("Variable Editor")
    .open(vars_editor_open)
    .anchor(Align2::CENTER_CENTER, [0., 0.])
    .default_height(ctx.screen_rect().height()*0.75)
    .max_height(ctx.screen_rect().height()*0.75)
    .min_width(ctx.screen_rect().width()*0.2)
    .scroll([true, false])
    .show(ctx, |ui| {
        let current_var = state.lock().unwrap().current_var.clone();
        let vars = state.lock().unwrap().vars.clone();
        let mut vars = vars.lock().unwrap();
        let var_name = current_var.unwrap();
        if render_var_name_editor(ui, &var_name, state.clone()).is_some() {
            state.lock().unwrap().current_var = None;
            return;
        }
        render_var_type_selector(ui, &var_name, &mut vars, state.clone());
        let var_type = vars.get(&var_name).unwrap();
        match var_type {
            BoardVariable::URL(_, _, _) => render_url_var_editor(ui, &var_name, &mut vars, state.clone()),
            BoardVariable::JsonURL(_, _, _, _) => render_json_extractor_var_editor(ui, &var_name, &mut vars, state.clone()),
            BoardVariable::Time(_) => render_datetime_var_editor(ui, &var_name, &mut vars, state.clone()),
        }
        render_config_panel(ctx, &vars);
    });
}

pub fn render_var_name_editor(ui: &mut Ui, var_name_in: &str, state: Arc<Mutex<State>>) -> Option<char> {
    ui.group(|ui| {
        ui.label("Variable Name");
        ui.horizontal(|ui| {
            let state = &mut state.lock().unwrap();
            let mut var_name = if state.var_name_edit.is_some() {
                state.var_name_edit.clone().unwrap()
            } else {
                var_name_in.to_string()
            };
            ui.text_edit_singleline(&mut var_name);
            if var_name.ne(var_name_in) {
                state.var_name_edit = Some(var_name);
            }
            if ui.button("Save").clicked() && state.var_name_edit.is_some() {
                state.rename_var = Some((var_name_in.to_string(), state.var_name_edit.clone().unwrap()));
            }
        });
    });
    None
}

fn render_var_type_selector(ui: &mut Ui, var_name: &str, vars: &mut BoardVariables, state: Arc<Mutex<State>>) {
    ui.group(|ui| {
        ui.label("Variable Type");
        let mut selected_type = vars.get(var_name).unwrap().get_variable_type();
        egui::ComboBox::from_id_salt("4db94127-04db-4836-97d0-080cd8581fb0")
            .selected_text(&selected_type)
            .show_ui(ui, |ui| {
                for var_type in BoardVariable::get_all_variable_types() {
                    ui.selectable_value(&mut selected_type, var_type.clone(), var_type.clone());
                }
            });
        {
            let orig_type = vars.get(var_name).unwrap().get_variable_type();
            if selected_type.ne(&orig_type) {
                log::info!("Type Changed: {} -> {}", &orig_type, &selected_type);
                vars.insert(var_name.to_string(), BoardVariable::get_default_by_type(&selected_type));
                state.lock().unwrap().vars_has_changed = true;
            }
        }
    });
}

fn render_datetime_var_editor(ui: &mut Ui, var_name: &str, vars: &mut BoardVariables, state: Arc<Mutex<State>>) {
    ui.group(|ui| {
        ui.label("DateTime Variable Editor");
        ui.separator();
        if let BoardVariable::Time(data) = vars.get(var_name).unwrap() {
            let mut selected = data.to_string();
            egui::ComboBox::from_id_salt("e6c62f61-9be4-4b63-85b7-fc27ba1cf88a")
                .selected_text(&selected)
                .show_ui(ui, |ui| {
                    for td in TimeData::get_all_time_data_types() {
                        ui.selectable_value(&mut selected, td.clone(), td.clone());
                    }
                });
            {
                let orig_val = data.to_string();
                if selected.ne(&orig_val) {
                    log::info!("Selection Changed: {} -> {}", &orig_val, &selected);
                    vars.insert(var_name.to_string(), BoardVariable::Time(TimeData::from_str(&selected).unwrap()));
                    state.lock().unwrap().vars_has_changed = true;
                }
            }
        }
    });
}

fn render_url_var_editor(ui: &mut Ui, var_name: &str, vars: &mut BoardVariables, state: Arc<Mutex<State>>) {
    ui.group(|ui| {
        ui.label("URL Variable Editor");
        if let BoardVariable::URL(id, url, timeout_secs) = vars.get(var_name).unwrap() {
            ui.group(|ui| {
                ui.label("URL ID");
                ui.label(id.to_string());
            });
            ui.separator();
            {
                let mut edit_url = url.clone();
                ui.horizontal(|ui| {
                    ui.label("URL: ");
                    ui.text_edit_singleline(&mut edit_url);
                });
                if edit_url.ne(url) {
                    vars.insert(var_name.to_string(), BoardVariable::URL(id.to_owned(), edit_url, timeout_secs.to_owned()));
                    state.lock().unwrap().vars_has_changed = true;
                    return;
                }
            }
            {
                let mut edit_timeout = timeout_secs.clone().to_string();
                ui.horizontal(|ui| {
                    ui.label("Timeout: ");
                    ui.text_edit_singleline(&mut edit_timeout);
                });
                if edit_timeout.ne(&timeout_secs.to_string()) {
                    if let Ok(mut val) = edit_timeout.parse::<i64>() {
                        if val < 15 {
                            val = 15;
                        }
                        vars.insert(var_name.to_string(), BoardVariable::URL(id.to_owned(), url.to_owned(), val));
                        state.lock().unwrap().vars_has_changed = true;
                        return;
                    }
                }
            }
        }
    });
}

fn render_json_extractor_var_editor(ui: &mut Ui, var_name: &str, vars: &mut BoardVariables, state: Arc<Mutex<State>>) {
    ui.group(|ui| {
        ui.label("JSON Extractor Variable Editor");
        ui.separator();
        if let BoardVariable::JsonURL(url_id, json_path, round_numbers, substring) = vars.get(var_name).unwrap() {
            {
                let mut edit_parent_id = url_id.clone().to_string();
                ui.horizontal(|ui| {
                    ui.label("URL ID: ");
                    ui.text_edit_singleline(&mut edit_parent_id);
                });
                if edit_parent_id.ne(&url_id.to_string()) {
                    if let Ok(val) = edit_parent_id.parse::<u32>() {
                        vars.insert(var_name.to_string(), BoardVariable::JsonURL(val.to_owned(), json_path.to_owned(), round_numbers.to_owned(), *substring));
                        state.lock().unwrap().vars_has_changed = true;
                        return;
                    }
                }
            }
            {
                let mut edit_path = json_path.clone();
                ui.horizontal(|ui| {
                    ui.label("JSON Path: ");
                    ui.text_edit_singleline(&mut edit_path);
                });
                if edit_path.ne(json_path) {
                    vars.insert(var_name.to_string(), BoardVariable::JsonURL(url_id.to_owned(), edit_path, round_numbers.to_owned(), *substring));
                    state.lock().unwrap().vars_has_changed = true;
                    return;
                }
            }
            {
                let mut round_numbers_edit = round_numbers.clone();
                ui.checkbox(&mut round_numbers_edit, "Round Numbers");
                if round_numbers_edit.ne(round_numbers) {
                    vars.insert(var_name.to_string(), BoardVariable::JsonURL(url_id.to_owned(), json_path.to_owned(), round_numbers_edit, *substring));
                    state.lock().unwrap().vars_has_changed = true;
                    return;
                }
            }
            {
                let substring_extract = if let Some(substring) = substring {
                    substring
                } else {
                    &(0 as u8,0 as i16)
                };
                let mut start = substring_extract.0.to_string();
                let mut end = substring_extract.1.to_string();
                ui.label("Substring");
                ui.indent("3b8169be-1a9a-4629-ac22-cfcdb4599bb5", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Start:");
                        ui.text_edit_singleline(&mut start);
                    });
                    ui.horizontal(|ui| {
                        ui.label("End:");
                        ui.text_edit_singleline(&mut end);
                    });
                });
                let mut new_substr = (0 as u8, 0 as i16);
                if start.len() > 0 {
                    if let Ok(new_start) = start.parse::<u8>() {
                        new_substr.0 = new_start;
                    }
                }
                if end.len() > 0 {
                    if let Ok(new_end) = end.parse::<i16>() {
                        new_substr.1 = new_end;
                    }
                }
                if new_substr.ne(substring_extract) {
                    if new_substr.0 == 0 && new_substr.1 == 0 {
                        vars.insert(var_name.to_string(), BoardVariable::JsonURL(url_id.to_owned(), json_path.to_owned(), round_numbers.to_owned(), None));
                    } else {
                        vars.insert(var_name.to_string(), BoardVariable::JsonURL(url_id.to_owned(), json_path.to_owned(), round_numbers.to_owned(), Some(new_substr)));
                    }
                    state.lock().unwrap().vars_has_changed = true;
                    return;
                }
            }
        }
    });
}

fn render_config_panel(ctx: &egui::Context, vars: &BoardVariables) {
    egui::Window::new("Variables Config")
        .anchor(Align2::RIGHT_TOP, [-5.0, 5.0])
        .show(ctx, |ui| {
            ui.label(serde_json::to_string(vars).unwrap());
        });
}