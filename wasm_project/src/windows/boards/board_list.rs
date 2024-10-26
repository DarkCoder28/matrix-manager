use std::sync::{Arc, Mutex};

use egui::{popup, Align2, Color32, RichText};

use crate::{app::State, post::post};

pub fn render_board_list(ctx: &egui::Context, state: Arc<Mutex<State>>, board_editor_open: &mut bool) {
    egui::Window::new("Boards")
        .anchor(Align2::LEFT_TOP, [5., 5.])
        .show(ctx, |ui| {
            let boards = state.lock().unwrap().boards.lock().unwrap().clone();
            let mut keys = boards.keys().collect::<Vec<&String>>();
            keys.sort();
            for board_name in keys {
                ui.horizontal(|ui| {
                    if ui.button("🗑").clicked() {
                        state.lock().unwrap().deleting_board = Some(board_name.to_owned());
                    }
                    let board_btn = ui.button(board_name);
                    if board_btn.clicked() {
                        state.lock().unwrap().current_board = Some(board_name.to_string());
                        state.lock().unwrap().current_editor = Some(0);
                        *board_editor_open = true;
                    }
                    let popup_id = ui.make_persistent_id(format!("board_ctx->{}", board_name));
                    if board_btn.secondary_clicked() {
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                    }
                    popup::popup_below_widget(ui, popup_id, &board_btn, egui::PopupCloseBehavior::CloseOnClickOutside, |ui| {
                        ui.set_width(128.);
                        if ui.button(format!("Duplicate {}", board_name)).clicked() {
                            let mut duplicate_board_name = format!("{}_duplicate", board_name);
                            while boards.contains_key(&duplicate_board_name) {
                                duplicate_board_name = format!("{}-", duplicate_board_name);
                            }
                            let mut duplicate_board = boards.get(board_name).unwrap().to_owned();
                            duplicate_board.name = duplicate_board_name.clone();
                            let mut state = state.lock().unwrap();
                            state.boards.lock().unwrap().insert(duplicate_board_name, duplicate_board);
                            state.boards_has_changed = true;
                        }
                    });
                });
            }
            ui.separator();
            if ui.button("Add Board").clicked() {
                state.lock().unwrap().add_board_dialog.open = true;
            }
            let boards_changed = state.lock().unwrap().boards_has_changed;
            if boards_changed {
                if ui.button(RichText::new("Post Changes").color(Color32::RED)).clicked() {
                    post("/api/update/boards", &boards);
                    state.lock().unwrap().boards_has_changed = false;
                }
            }
        });
}