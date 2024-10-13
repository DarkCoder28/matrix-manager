use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use eframe::egui;
use shared::{
    board_variables::BoardVariables,
    boards::{BoardDefinition, BoardElement},
    device_config::DeviceConfigs,
};

use crate::{
    get::get,
    windows::{
        boards::{
            board_add::render_board_add, board_editor::render_board_editor,
            board_list::render_board_list,
        },
        devices::{device_editor::render_device_editor, device_list::render_device_list},
        vars::{var_add::render_var_add, var_editor::render_var_editor, var_list::render_var_list},
    },
};

#[derive(Default)]
pub struct MyEguiApp {
    pub state: Arc<Mutex<State>>,
    pub board_editor_open: bool,
    pub var_editor_open: bool,
    pub device_editor_open: bool,
}

#[derive(Default)]
pub struct State {
    pub boards: Arc<Mutex<HashMap<String, BoardDefinition>>>,
    pub boards_has_changed: bool,
    pub fonts: Arc<Mutex<Vec<String>>>,
    pub current_board: Option<String>,
    pub add_board_dialog: AddDialog,
    pub board_name_edit: Option<String>,
    pub rename_board: Option<(String, String)>,
    pub open_board_elements: HashMap<usize, BoardElement>,
    //
    pub vars: Arc<Mutex<BoardVariables>>,
    pub current_var: Option<String>,
    pub add_var_dialog: AddDialog,
    pub vars_has_changed: bool,
    pub var_name_edit: Option<String>,
    pub rename_var: Option<(String, String)>,
    //
    pub devices: Arc<Mutex<DeviceConfigs>>,
    pub current_device: Option<String>,
    pub add_device_dialog: AddDialog,
    pub devices_has_changed: bool,
    //
    pub current_editor: Option<u8>,
}

pub struct AddDialog {
    pub open: bool,
    pub element_name: String,
}
impl Default for AddDialog {
    fn default() -> Self {
        AddDialog {
            open: false,
            element_name: String::new(),
        }
    }
}

impl MyEguiApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let board_data: Arc<Mutex<HashMap<String, BoardDefinition>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let var_data: Arc<Mutex<BoardVariables>> = Arc::new(Mutex::new(BoardVariables::new()));
        let device_data: Arc<Mutex<DeviceConfigs>> = Arc::new(Mutex::new(DeviceConfigs::new()));
        let font_data: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        // Load Boards from server...
        get("/api/boards", board_data.clone());

        // Load Variables from server...
        get("/api/vars", var_data.clone());

        // Load Device Configs from server...
        get("/api/devices", device_data.clone());

        // Load Fonts from server...
        get("/api/fonts", font_data.clone());

        let setup = MyEguiApp {
            state: Arc::new(Mutex::new(State {
                boards: board_data,
                vars: var_data,
                devices: device_data,
                fonts: font_data,
                ..Default::default()
            })),
            ..Default::default()
        };
        return setup;
    }
}

impl eframe::App for MyEguiApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::to_normalized_gamma_f32(egui::Color32::from_rgb(24, 26, 27))
    }
    fn persist_egui_memory(&self) -> bool {
        true
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            render_board_list(ctx, self.state.clone(), &mut self.board_editor_open);
            render_board_editor(ctx, self.state.clone(), &mut self.board_editor_open);
            render_board_add(ctx, self.state.clone());
            //
            render_var_list(ctx, self.state.clone(), &mut self.var_editor_open);
            render_var_editor(ctx, self.state.clone(), &mut self.var_editor_open);
            render_var_add(ctx, self.state.clone());
            //
            render_device_list(ctx, self.state.clone(), &mut self.device_editor_open);
            render_device_editor(ctx, self.state.clone(), &mut self.device_editor_open);
            //
            // Lock State
            let mut state = self.state.lock().unwrap();
            // Close oof windows
            if let Some(editor) = state.current_editor {
                match editor {
                    0 => {
                        self.var_editor_open = false;
                        self.device_editor_open = false;
                    }
                    1 => {
                        self.board_editor_open = false;
                        self.device_editor_open = false;
                    }
                    2 => {
                        self.board_editor_open = false;
                        self.var_editor_open = false;
                    }
                    _ => {}
                }
            }
            // Modifications
            let should_rename_board = state.rename_board.is_some();
            if should_rename_board {
                let rename_boards = state.rename_board.clone().unwrap();
                let old_name = rename_boards.0;
                let new_name = rename_boards.1;
                {
                    let mut boards = state.boards.lock().unwrap();
                    let mut board = boards.remove(&old_name).unwrap();
                    board.name = new_name.clone();
                    boards.insert(new_name.clone(), board);
                }
                state.rename_board = None;
                state.current_board = Some(new_name);
                state.board_name_edit = None;
                state.boards_has_changed = true;
            }
            let should_rename_var = state.rename_var.is_some();
            if should_rename_var {
                let (old_name, new_name) = state.rename_var.clone().unwrap();
                {
                    let mut vars = state.vars.lock().unwrap();
                    let var = vars.remove(&old_name).unwrap();
                    vars.insert(new_name.clone(), var);
                }
                state.rename_var = None;
                state.current_var = Some(new_name);
                state.var_name_edit = None;
                state.vars_has_changed = true;
            }
            if !self.board_editor_open {
                state.current_board = None;
            }
        });
    }
}
