use std::sync::{Arc, Mutex};
use egui::{Color32, Ui};
use shared::boards::{BoardElement, BoardElementValue, ColourOption, ElementColour};

use crate::app::State;


pub fn render_element_editor(ui: &mut Ui, board_element: &mut BoardElement, refocus_name: bool, state: Arc<Mutex<State>>) {
    let value_type = board_element.value.get_type();
    let mut modified = false;
    ui.group(|ui| {
        // TODO: Finish up image element editor
        // TODO: Add element delete button
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                if render_element_name_editor(ui, board_element, refocus_name, state.clone()) { modified = true; }
            });
            ui.separator();
            ui.vertical(|ui| {
                if render_element_type_selector(ui, board_element) { modified = true; }
            });
        });
        ui.separator();
        if render_element_position_editor(ui, board_element) { modified = true; }
        ui.separator();
        if value_type.eq("Text") {
            if render_text_style_editor(ui, board_element, state.clone()) { modified = true; }
            ui.separator();
        }
        if match value_type.as_str() {
            "Text" => render_text_value_editor(ui, &mut board_element.value),
            "Image" => render_image_value_editor(ui, &board_element.name, &mut board_element.value, state.clone()),
            _ => false
        } { modified = true; }
    });
    if modified {
        state.lock().unwrap().boards_has_changed = true;
    }
}

fn render_element_name_editor(ui: &mut Ui, board_element: &mut BoardElement, refocus: bool, state: Arc<Mutex<State>>) -> bool {
    let mut modified = false;
    ui.label("Element Name");
    let text_edit = ui.add(egui::TextEdit::singleline(&mut board_element.name).hint_text("Element Name").desired_width(128.));
    if refocus {
        text_edit.request_focus();
        state.lock().unwrap().rename_board_element = None;
    }
    if text_edit.changed() {
        modified = true;
        state.lock().unwrap().rename_board_element = Some(board_element.name.clone());
    }
    return modified;
}

fn render_element_type_selector(ui: &mut Ui, board_element: &mut BoardElement) -> bool {
    let mut modified = false;
    let mut type_edit = board_element.value.get_type();
    let mut salt = board_element.name.clone();
    salt.push_str("66b4210e-a9f1-4def-9099-10e1aa6aea54");
    ui.label("Element Type");
    egui::ComboBox::from_id_salt(salt)
        .selected_text(&type_edit)
        .show_ui(ui, |ui| {
            for option in BoardElementValue::get_types() {
                ui.selectable_value(&mut type_edit, option.clone(), option);
            }
        });
    if type_edit.ne(&board_element.value.get_type()) {
        board_element.value = BoardElementValue::from_strings(&type_edit, board_element.value.extract_element_value().1, true);
        modified = true;
    }
    modified
}

fn render_element_position_editor(ui: &mut Ui, board_element: &mut BoardElement) -> bool {
    let mut modified = false;
    let mut x_edit = element_u8_option_to_string(board_element.x);
    let mut y_edit = element_u8_to_string(board_element.y);
    ui.label("Position");
    ui.horizontal(|ui| {
        ui.label("X:");
        ui.add(egui::TextEdit::singleline(&mut x_edit).hint_text("Centered").desired_width(64.));
        ui.separator();
        ui.label("Y:");
        ui.add(egui::TextEdit::singleline(&mut y_edit).desired_width(64.));
    });
    if x_edit.ne(&element_u8_option_to_string(board_element.x)) {
        let num_string = get_num_from_string(&x_edit);
        if num_string.is_none() {
            board_element.x = None;
        } else {
            let num_string = num_string.unwrap();
            if let Ok(x) = num_string.parse::<u8>() {
                board_element.x = Some(x);
            }
        }
        modified = true;
    }
    if y_edit.ne(&element_u8_to_string(board_element.y)) {
        let num_string = get_num_from_string(&y_edit);
        if num_string.is_none() {
            board_element.y = 0;
        } else {
            let num_string = num_string.unwrap();
            if let Ok(y) = num_string.parse::<u8>() {
                board_element.y = y;
            }
        }
        modified = true;
    }
    modified
}

fn render_text_style_editor(ui: &mut Ui, board_element: &mut BoardElement, state: Arc<Mutex<State>>) -> bool {
    let mut modified = false;
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            if render_text_colour_editor(ui, board_element) { modified = true; }
        });
        ui.separator();
        ui.vertical(|ui| {
            if render_text_font_editor(ui, board_element, state) { modified = true; }
        });
    });
    modified
}

fn render_text_colour_editor(ui: &mut Ui, board_element: &mut BoardElement) -> bool {
    let mut modified = false;
    ui.label("Colour");
    if render_text_colour_type_selector(ui, board_element) { modified = true; }
    if board_element.colour.get_option().eq("Specific") {
        if render_text_colour_select(ui, board_element) { modified = true; }
    }
    modified
}

fn render_text_colour_type_selector(ui: &mut Ui, board_element: &mut BoardElement) -> bool {
    let mut modified = false;
    let mut colour_type = board_element.colour.get_option();
    let mut salt = board_element.name.clone();
    salt.push_str("abc19488-9692-482c-9ab3-2315a6cd1389");
    egui::ComboBox::from_id_salt(salt)
        .selected_text(&colour_type)
        .show_ui(ui, |ui| {
            for opt in ColourOption::get_options() {
                ui.selectable_value(&mut colour_type, opt.clone(), opt.clone());
            }
        });
    if colour_type.ne(&board_element.colour.get_option()) {
        board_element.colour = ColourOption::from_str(&colour_type);
        modified = true;
    }
    modified
}

fn render_text_colour_select(ui: &mut Ui, board_element: &mut BoardElement) -> bool {
    let mut modified = false;
    let mut change_value = None;
    if let ColourOption::Specific(colour) = &board_element.colour {
        let mut colour_edit = colour.to_egui_colour();
        ui.color_edit_button_srgba(&mut colour_edit);
        if colour_edit.ne(&colour.to_egui_colour()) {
            change_value = Some(colour_edit);
        }
    }
    if let Some(value) = change_value {
        board_element.colour = ColourOption::Specific(ElementColour::from_egui_colour(&value));
        modified = true;
    }
    modified
}

fn render_text_font_editor(ui: &mut Ui, board_element: &mut BoardElement, state: Arc<Mutex<State>>) -> bool {
    let mut modified = false;
    let mut font = board_element.font.clone().unwrap_or(String::from("Default"));
    ui.label("Font");
    let mut salt = board_element.name.clone();
    salt.push_str("4e6bee20-0610-4b8c-9a72-10f8297144d0");
    egui::ComboBox::from_id_salt(salt)
        .selected_text(&font)
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut font, "Default".to_string(), "Default".to_string());
            let mut state = state.lock().unwrap();
            {
                if !state.fonts_sorted {
                    state.fonts.lock().unwrap().sort();
                    state.fonts_sorted = true;
                }
            }
            let fonts = state.fonts.lock().unwrap().clone();
            for opt in fonts {
                ui.selectable_value(&mut font, opt.clone(), opt.clone());
            }
        });
    if font.ne(&board_element.font.clone().unwrap_or(String::from("Default"))) {
        board_element.font = if font.eq("Default") { None } else { Some(font) };
        modified = true;
    }
    modified
}

fn render_text_value_editor(ui: &mut Ui, value: &mut BoardElementValue) -> bool {
    let mut modified = false;
    ui.label("Text Editor");
    if let BoardElementValue::Text(value) = value {
        let old_val = value.clone();
        ui.text_edit_singleline(value);
        if old_val.ne(value) {
            modified = true;
        }
    }
    modified
}

fn render_image_value_editor(ui: &mut Ui, board_name: &str, value: &mut BoardElementValue, state: Arc<Mutex<State>>) -> bool {
    let mut modified = false;
    ui.label("Image Editor");
    if let BoardElementValue::Img(value, dynamic) = value {
        let old_dynamic = *dynamic;
        ui.checkbox(dynamic, "Dynamic");
        if *dynamic != old_dynamic {
            modified = true;
        }
        //
        let mut salt = board_name.to_string();
        salt.push_str("2a59c793-8f64-44c8-a140-b40df0a0fcf4");
        ui.indent(salt, |ui| {
            if *dynamic {
                if render_image_value_editor_dynamic(ui, value) { modified = true; }
            } else {
                if render_image_value_editor_static(ui, board_name, value, state) { modified = true; }
            }
        });
    }
    modified
}

fn render_image_value_editor_static(ui: &mut Ui, board_name: &str, value: &mut String, state: Arc<Mutex<State>>) -> bool {
    let mut modified = false;
    let mut value_edit = value.clone();
    let mut salt = board_name.to_string();
    salt.push_str("25d5238e-9cc1-4b13-a495-28f4aa9d30d5");
    ui.label("Static Image Editor");
    egui::ComboBox::from_id_salt(salt)
        .selected_text(&value_edit)
        .show_ui(ui, |ui| {
            let state = state.lock().unwrap();
            let images = state.images.lock().unwrap();
            if !images.contains(&value_edit) {
                ui.selectable_value(&mut value_edit, value.clone(), value.clone());
            }
            for image in &*images {
                ui.selectable_value(&mut value_edit, image.clone(), image.clone());
            }
        });
    if value_edit.ne(value) {
        *value = value_edit;
        modified = true;
    }
    modified
}

fn render_image_value_editor_dynamic(ui: &mut Ui, value: &mut String) -> bool {
    let mut modified = false;
    let old_val = value.clone();
    ui.label("Dynamic Image Editor");
    ui.text_edit_singleline(value);
    if old_val.ne(value) {
        modified = true;
    }
    modified
}



// Utility Functions
fn element_u8_to_string(val: u8) -> String {
    if val > 0 {
        val.to_string()
    } else {
        String::from("0")
    }
}
fn element_u8_option_to_string(val: Option<u8>) -> String {
    if val.is_some() {
        val.unwrap().to_string()
    } else {
        String::new()
    }
}
fn get_num_from_string(input: &str) -> Option<String> {
    let re = regex::Regex::new(r"(?P<number>\d+)").unwrap();
    let vals = re.captures(input);
    if vals.is_none() {
        return None;
    }
    let vals = vals.unwrap();
    Some(vals["number"].to_owned())
}

trait EguiColourCompat {
    fn to_egui_colour(&self) -> Color32;
    fn import_egui_colour(&mut self, colour: &Color32);
    fn from_egui_colour(colour: &Color32) -> ElementColour;
}
impl EguiColourCompat for ElementColour {
    fn to_egui_colour(&self) -> Color32 {
        Color32::from_rgba_unmultiplied(self.r, self.g, self.b, self.a)
    }
    fn import_egui_colour(&mut self, colour: &Color32) {
        self.r = colour.r();
        self.g = colour.g();
        self.b = colour.b();
        self.a = colour.a();
    }
    fn from_egui_colour(colour: &Color32) -> ElementColour {
        let mut col = ElementColour::default();
        col.import_egui_colour(colour);
        col
    }
}
