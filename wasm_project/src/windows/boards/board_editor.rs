use std::sync::{Arc, Mutex};

use egui::{Align2, Ui};
use shared::boards::BoardDefinition;

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
            render_board_elements(ui, board, state.clone());
            // TODO: Make 'Add Element' button
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

fn render_config_panel(ctx: &egui::Context, board: &BoardDefinition) {
    egui::Window::new("Board Config")
        .anchor(Align2::RIGHT_TOP, [-5.0, 5.0])
        .show(ctx, |ui| {
            ui.label(serde_json::to_string(board).unwrap());
        });
}