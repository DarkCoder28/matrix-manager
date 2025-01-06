use std::sync::{Arc, Mutex};

use egui::{Align2, Ui};
use shared::boards::{BoardDefinition, BoardElement};

use crate::{app::State, windows::boards::board_element_editor::render_element_editor};



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
        .anchor(Align2::CENTER_CENTER, [0., 0.])
        .default_height(ctx.screen_rect().height()*0.75)
        .max_height(ctx.screen_rect().height()*0.75)
        .min_width(ctx.screen_rect().width()*0.2)
        .scroll([true, false])
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
            if render_use_skip_brightness_threshold(ui, board) {
                state.lock().unwrap().boards_has_changed = true;
                return;
            }
            render_board_elements(ui, board, state.clone());
            render_board_element_add_delete(ui, board, state.clone());
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

fn render_use_skip_brightness_threshold(ui: &mut Ui, board: &mut BoardDefinition) -> bool {
    let mut changed = false;
    let orig_value = board.use_skip_brightness_threshold;
    ui.group(|ui| {
        ui.checkbox(&mut board.use_skip_brightness_threshold, "Skip if below brightness threshold");
    });
    if board.use_skip_brightness_threshold != orig_value {
        changed = true;
    }
    changed
}

fn render_board_elements(ui: &mut Ui, board: &mut BoardDefinition, state: Arc<Mutex<State>>) {
    for element in &mut board.board_elements {
        let mut open = None;
        if let Some(new_name) = &state.lock().unwrap().rename_board_element {
            if element.name.eq(new_name) {
                open = Some(true);
            }
        }
        egui::CollapsingHeader::new(&element.name)
            .open(open)
            .show(ui, |ui| {
                render_element_editor(ui, element, open.is_some(), state.clone());
            });
    }
}

fn render_board_element_add_delete(ui: &mut Ui, board: &mut BoardDefinition, state: Arc<Mutex<State>>) {
    ui.horizontal(|ui| {
        if ui.button("Add Element").clicked() {
            board.board_elements.push(BoardElement::default());
            state.lock().unwrap().boards_has_changed = true;
        }
        let mut to_delete = String::from("Delete Element");
        let element_list = board.board_elements.iter().map(|x|x.name.clone()).collect::<Vec<String>>();
        egui::ComboBox::from_id_salt("6b32ddc5-209c-4cc0-b64c-6bf1e0829ec0")
            .selected_text(&to_delete)
            .show_ui(ui, |ui| {
                for element in element_list {
                    ui.selectable_value(&mut to_delete, element.clone(), element);
                }
            });
        if to_delete.ne("Delete Element") {
            board.board_elements.retain(|x|x.name.ne(&to_delete));
            state.lock().unwrap().boards_has_changed = true;
        }
    });
}

fn render_config_panel(ctx: &egui::Context, board: &BoardDefinition) {
    let mut window_height = ctx.screen_rect().height();
    window_height-=120.;
    window_height/=2.;
    egui::Window::new("Board Config")
        .anchor(Align2::RIGHT_TOP, [-5.0, 5.0])
        .min_height(window_height)
        .max_height(window_height)
        .scroll([false, true])
        .movable(false)
        .show(ctx, |ui| {
            ui.label(serde_json::to_string(board).unwrap());
        });
}