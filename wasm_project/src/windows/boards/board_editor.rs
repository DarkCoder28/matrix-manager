use std::sync::{Arc, Mutex};

use egui::{Align2, Color32, Ui};
use shared::boards::{BoardDefinition, BoardElement, BoardElementValue, ColourOption, ElementColour};
use wasm_bindgen_futures::spawn_local;

use crate::app::State;

pub fn render_board_editor(
    ctx: &egui::Context,
    state: Arc<Mutex<State>>,
    board_editor_open: &mut bool,
) {
    if !*board_editor_open || state.lock().unwrap().current_board.is_none() {
        return;
    }
    egui::Window::new("Board Editor")
        .open(board_editor_open)
        .anchor(Align2::CENTER_TOP, [0., 5.])
        .show(ctx, |ui| {
            let current_board = state.lock().unwrap().current_board.clone();
            let boards = state.lock().unwrap().boards.clone();
            let mut boards = boards.lock().unwrap();
            let board = boards.get_mut(&current_board.unwrap()).unwrap();
            //
            if render_board_name_editor(ui, board, state.clone()).is_some() {
                state.lock().unwrap().current_board = None;
                return;
            }
            render_board_size_editor(ui, board);
            render_board_elements(ui, board, state.clone());
            //
            render_config_panel(ctx, board);
        });
}

fn render_board_name_editor(
    ui: &mut Ui,
    board: &mut BoardDefinition,
    state: Arc<Mutex<State>>,
) -> Option<char> {
    ui.group(|ui| {
        ui.label("Board Name");
        ui.horizontal(|ui| {
            let state = &mut state.lock().unwrap();
            let mut board_name = if state.board_name_edit.is_some() {
                state.board_name_edit.clone().unwrap()
            } else {
                board.name.clone()
            };
            ui.text_edit_singleline(&mut board_name);
            if !board_name.eq(&board.name) {
                state.board_name_edit = Some(board_name);
            }
            if ui.button("Save").clicked() && state.board_name_edit.is_some() {
                state.rename_board =
                    Some((board.name.clone(), state.board_name_edit.clone().unwrap()));
            }
        });
    });
    None
}

fn render_board_size_editor(ui: &mut Ui, board: &mut BoardDefinition) {
    ui.group(|ui| {
        ui.label("Board Size");
        // Width
        ui.horizontal(|ui| {
            ui.label("Width");
            let mut board_width_string = if board.size.0 > 0 {
                board.size.0.to_string()
            } else {
                String::new()
            };
            ui.add(egui::TextEdit::singleline(&mut board_width_string).hint_text("Enter a number"));
            if let Ok(value) = board_width_string.parse::<u8>() {
                board.size.0 = value;
            } else if board_width_string.is_empty() {
                board.size.0 = 0;
            }
        });
        // Height
        ui.horizontal(|ui| {
            ui.label("Height");
            let mut board_height_string = if board.size.1 > 0 {
                board.size.1.to_string()
            } else {
                String::new()
            };
            ui.add(
                egui::TextEdit::singleline(&mut board_height_string).hint_text("Enter a number"),
            );
            if let Ok(value) = board_height_string.parse::<u8>() {
                board.size.1 = value;
            } else if board_height_string.is_empty() {
                board.size.1 = 0;
            }
        });
    });
}

fn render_board_elements(ui: &mut Ui, board: &mut BoardDefinition, state: Arc<Mutex<State>>) {
    let fonts: Vec<String>;
    {
        fonts = state.lock().unwrap().fonts.lock().unwrap().clone();
    }
    ui.group(|ui| {
        ui.label("Board Elements");
        for (idx, element) in board.board_elements.clone().iter().enumerate() {
            let open_board_elements = &mut state.lock().unwrap().open_board_elements;
            egui::CollapsingHeader::new(&element.name)
                // .open(if should_open {Some(true)} else {None})
                .show(ui, |ui| {
                    ui.group(|ui| {
                        ui.label("Name");
                        let mut element_name = if open_board_elements.contains_key(&idx) {
                            open_board_elements.get(&idx).unwrap().name.clone()
                        } else {
                            element.name.clone()
                        };
                        ui.text_edit_singleline(&mut element_name);
                        if element_name.ne(&element.name) {
                            if !open_board_elements.contains_key(&idx) {
                                open_board_elements.insert(idx, element.clone());
                            }
                            if element_name.len() > 0 {
                                open_board_elements.get_mut(&idx).unwrap().name = element_name;
                            } else {
                                board.board_elements.get_mut(idx).unwrap().name = String::from("_");
                            }
                        }
                    });
                    ui.group(|ui| {
                        ui.label("Position");
                        ui.horizontal(|ui| {
                            ui.label("X: ");
                            {
                                let mut val = if open_board_elements.contains_key(&idx) {
                                    element_u8_option_to_string(open_board_elements.get(&idx).unwrap().x)
                                } else {
                                    element_u8_option_to_string(element.x)
                                };
                                ui.add(
                                    egui::TextEdit::singleline(&mut val)
                                        .hint_text("Enter a number"),
                                );
                                if val.ne(&element_u8_option_to_string(element.x)) {
                                    if !open_board_elements.contains_key(&idx) {
                                        open_board_elements.insert(idx, element.clone());
                                    }
                                    if let Ok(value) = val.parse::<u8>() {
                                        open_board_elements.get_mut(&idx).unwrap().x = Some(value);
                                    } else if val.is_empty() {
                                        open_board_elements.get_mut(&idx).unwrap().x = None;
                                    }
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Y: ");
                            {
                                let mut val = if open_board_elements.contains_key(&idx) {
                                    element_u8_to_string(open_board_elements.get(&idx).unwrap().y)
                                } else {
                                    element_u8_to_string(element.y)
                                };
                                ui.add(
                                    egui::TextEdit::singleline(&mut val)
                                        .hint_text("Enter a number"),
                                );
                                if val.ne(&element_u8_to_string(element.y)) {
                                    if !open_board_elements.contains_key(&idx) {
                                        open_board_elements.insert(idx, element.clone());
                                    }
                                    if let Ok(value) = val.parse::<u8>() {
                                        open_board_elements.get_mut(&idx).unwrap().y = value;
                                    } else if val.is_empty() {
                                        open_board_elements.get_mut(&idx).unwrap().y = 0;
                                    }
                                }
                            }
                        });
                    });
                    ui.horizontal_wrapped(|ui| {
                        ui.vertical(|ui| {
                            ui.group(|ui| {
                                ui.label("Colour");
                                {
                                    let mut colour_type = if open_board_elements.contains_key(&idx) {
                                        open_board_elements.get(&idx).unwrap().colour.get_option()
                                    } else {
                                        element.colour.get_option()
                                    };
                                    egui::ComboBox::from_id_salt("abc19488-9692-482c-9ab3-2315a6cd1389")
                                        .selected_text(&colour_type)
                                        .show_ui(ui, |ui| {
                                            for opt in ColourOption::get_options() {
                                                ui.selectable_value(&mut colour_type, opt.clone(), opt.clone());
                                            }
                                        });
                                    if colour_type.ne(&element.colour.get_option()) {
                                        log::info!("{:#?} -> {:#?}", &colour_type, &element.colour.get_option());
                                        if !open_board_elements.contains_key(&idx) {
                                            open_board_elements.insert(idx, element.clone());
                                        }
                                        open_board_elements.get_mut(&idx).unwrap().colour = match colour_type.as_str() {
                                            "Default" => ColourOption::Default,
                                            "Specific" => ColourOption::Specific(ElementColour::default()),
                                            "Parse Temperature" => ColourOption::ParseTemperature,
                                            _ => ColourOption::Default
                                        };
                                    }
                                }
                                {
                                    let colour = if open_board_elements.contains_key(&idx) {
                                        open_board_elements.get(&idx).unwrap().colour.clone()
                                    } else {
                                        element.colour.clone()
                                    };
                                    if let ColourOption::Specific(colour) = colour {
                                        let mut colour_edit = colour.to_egui_colour();
                                        ui.color_edit_button_srgba(&mut colour_edit);
                                        if colour_edit.ne(&colour.to_egui_colour()) {
                                            log::info!("{:#?} -> {:#?}", &colour.to_egui_colour(), &colour_edit);
                                            if !open_board_elements.contains_key(&idx) {
                                                open_board_elements.insert(idx, element.clone());
                                            }
                                            open_board_elements.get_mut(&idx).unwrap().colour = ColourOption::Specific(ElementColour::from_egui_colour(&colour_edit));
                                            log::info!("{:#?}", open_board_elements);
                                        }
                                        ui.label(format!("({}, {}, {}, {})", colour.r, colour.g, colour.b, colour.a));
                                    }
                                }
                            });
                        });
                        ui.vertical(|ui| {
                            ui.group(|ui| {
                                ui.label("Font");
                                let mut selected_font = if open_board_elements.contains_key(&idx) {
                                    open_board_elements.get(&idx).unwrap().font.clone().unwrap_or(String::from("5x8"))
                                } else {
                                    element.font.clone().unwrap_or(String::from("5x8"))
                                };
                                egui::ComboBox::from_label("")
                                    .selected_text(&selected_font)
                                    .show_ui(ui, |ui| {
                                        for font in &fonts {
                                            ui.selectable_value(&mut selected_font, font.clone(), font.clone());
                                        }
                                    });
                                if selected_font.ne(&element.font.clone().unwrap_or(String::from("5x8"))) {
                                    if !open_board_elements.contains_key(&idx) {
                                        open_board_elements.insert(idx, element.clone());
                                    }
                                    open_board_elements.get_mut(&idx).unwrap().font = Some(selected_font);
                                }
                            });
                        });
                    });
                    ui.group(|ui| {
                        let (mut var_type, mut value) = if open_board_elements.contains_key(&idx) {
                            open_board_elements.get(&idx).unwrap().value.extract_element_value()
                        } else {
                            element.value.extract_element_value()
                        };
                        ui.horizontal(|ui| {
                            ui.label("Value");
                            egui::ComboBox::from_label("")
                                .selected_text(&var_type)
                                .show_ui(ui, |ui| {
                                    for element_type in BoardElementValue::get_types() {
                                        ui.selectable_value(&mut var_type, element_type.clone(), element_type.clone());
                                    }
                                });
                        });
                        ui.text_edit_singleline(&mut value);
                        if var_type.ne(&element.value.extract_element_value().0.clone())
                            || value.ne(&element.value.extract_element_value().1.clone()) {
                            if !open_board_elements.contains_key(&idx) {
                                open_board_elements.insert(idx, element.clone());
                            }
                            open_board_elements.get_mut(&idx).unwrap().value = BoardElementValue::from_strings(&var_type, value);
                        }
                    });
                    ui.group(|ui|{
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                if open_board_elements.contains_key(&idx) {
                                    let f = board.board_elements.get_mut(idx).unwrap();
                                    f.set(open_board_elements.remove(&idx).unwrap());
                                    let state2 = state.clone();
                                    spawn_local(async move {
                                        state2.lock().unwrap().boards_has_changed = true;
                                    });
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                open_board_elements.remove(&idx);
                            }
                        });
                    });
                });
        }
        ui.separator();
        if ui.button("Add Element").clicked() {
            board.board_elements.push(BoardElement::default());
        }
    });
}

fn render_config_panel(ctx: &egui::Context, board: &BoardDefinition) {
    egui::Window::new("Board Config")
        .anchor(Align2::RIGHT_TOP, [-5.0, 5.0])
        .show(ctx, |ui| {
            ui.label(serde_json::to_string(board).unwrap());
        });
}

fn element_u8_to_string(val: u8) -> String {
    if val > 0 {
        val.to_string()
    } else {
        String::new()
    }
}
fn element_u8_option_to_string(val: Option<u8>) -> String {
    if val.is_some() {
        val.unwrap().to_string()
    } else {
        String::new()
    }
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