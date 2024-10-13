use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Error;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::error;

use shared::{
    board_variables::{BoardVariable, BoardVariables, TimeData},
    boards::{BoardDefinition, BoardElementBuilder, BoardElementValue}, device_config::{DeviceConfig, DeviceConfigs},
};

pub(crate) type ConfigWrapper = Arc<RwLock<Config>>;
// pub(crate) type BoardVariables = HashMap<String, BoardVariable>;
pub(crate) type Boards = HashMap<String, BoardDefinition>;

#[derive(Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(skip)]
    pub(crate) config_path: String,
    pub(crate) device_configs: DeviceConfigs,
    pub(crate) board_variables: BoardVariables,
    boards: Boards,
}

impl Config {
    pub(crate) fn get_boards(&self) -> &Boards {
        return &self.boards;
    }
    pub(crate) fn add_board(&mut self, board: BoardDefinition) {
        self.boards.insert(board.name.clone(), board);
    }
    pub(crate) fn update_boards(&mut self, boards: &HashMap<String, BoardDefinition>) {
        for (name, _) in self.boards.clone() {
            if !boards.contains_key(&name) {
                self.remove_board(name).unwrap();
            }
        }
        for (name, board) in boards {
            self.boards.insert(name.clone(), board.clone());
        }
    }
    pub(crate) fn remove_board(&mut self, board_name: String) -> Result<(), Error> {
        if self.boards.contains_key(&board_name) {
            self.boards.remove_entry(&board_name);
            return Ok(());
        } else {
            return Err(anyhow::anyhow!("Board ({}) does not exist!", board_name));
        }
    }
    fn new(config_file: &Path) -> Config {
        if config_file.exists() {
            let config_data = fs::read(config_file).unwrap();
            let mut config: Config = serde_json::from_slice(&config_data).unwrap();
            config.config_path = config_file.to_string_lossy().to_string();
            return config;
        }
        let mut default_board_variables = HashMap::new();
        default_board_variables.insert(
            String::from("weekday"),
            BoardVariable::Time(TimeData::Weekday),
        );
        default_board_variables.insert(String::from("time"), BoardVariable::Time(TimeData::Time));
        default_board_variables.insert(String::from("date"), BoardVariable::Time(TimeData::Date));

        let mut new_config = Config {
            config_path: String::from(config_file.to_str().unwrap()),
            device_configs: HashMap::new(),
            board_variables: default_board_variables,
            boards: HashMap::new(),
        };

        new_config.device_configs.insert(String::from("default"), DeviceConfig {
            name: String::from("Default"),
            ..Default::default()
        });

        new_config.add_board(BoardDefinition {
            name: String::from("clock"),
            size: (64, 32),
            board_elements: vec![
                BoardElementBuilder::default()
                    .y(0)
                    .value(BoardElementValue::Text(String::from("__weekday__")))
                    .build()
                    .unwrap(),
                BoardElementBuilder::default()
                    .y(9)
                    .font(Some(String::from("7x14B")))
                    .value(BoardElementValue::Text(String::from("__time__")))
                    .build()
                    .unwrap(),
                BoardElementBuilder::default()
                    .y(24)
                    .value(BoardElementValue::Text(String::from("__date__")))
                    .build()
                    .unwrap(),
            ],
        });

        return new_config;
    }

    pub(crate) fn save(&self) {
        let config_file = PathBuf::from(&self.config_path);
        let serialized_config = serde_json::to_string_pretty(&self);
        if serialized_config.is_err() {
            error!(
                "Failed serializing default config...\n{}",
                serialized_config.unwrap_err()
            );
            panic!();
        }
        let serialized_config = serialized_config.unwrap();
        if fs::create_dir_all((&config_file).parent().unwrap()).is_ok() {
            let file = fs::File::create(&config_file);
            if file
                .unwrap()
                .write_all(serialized_config.as_bytes())
                .is_err()
            {
                error!("Unable to write default config file!");
                panic!();
            }
        } else {
            error!("Error creating config directory");
            panic!();
        }
    }

    pub(crate) async fn from_async(config_file: Option<String>) -> Config {
        let config_path: PathBuf = match config_file {
            Some(x) => PathBuf::from(x),
            None => {
                if let Some(directories) =
                    ProjectDirs::from("com", "aidensheeran", "matrix-manager")
                {
                    let mut config_location = directories.config_dir().to_path_buf();
                    config_location.push("config.json");
                    config_location
                } else {
                    error!("!!!COULD NOT DETERMINE CONFIG DIRECTORY!!!");
                    panic!();
                }
            }
        };

        if config_path.exists() {
            // Load Config
            return Config::new(&config_path.as_path());
        } else {
            tracing::info!(
                "No config found... downloading missing assets and creating new default config..."
            );
            // Download Default Assets
            download_default_assets(&config_path.as_path()).await;
            // Create Default Config
            let config = Config::new(&config_path.as_path());
            config.save();
            tracing::info!("Done.");
            return config;
        }
    }
}

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.3";
async fn download_default_assets(config_file: &Path) {
    let config_dir = config_file
        .parent()
        .expect("The config file somehow doesn't have a parent folder??? How???");
    let mut assets_dir = config_dir.to_path_buf();
    assets_dir.push("assets/");
    let assets_dir = assets_dir.as_path();
    let mut fonts_dir = assets_dir.to_path_buf();
    fonts_dir.push("fonts/");
    let fonts_dir = fonts_dir.as_path();
    let mut fonts = HashMap::new();
    fonts.insert("5x8.bdf", "https://raw.githubusercontent.com/DarkCoder28/matrix-manager-default-assets/refs/heads/main/fonts/5x8.bdf");
    fonts.insert("7x14B.bdf", "https://raw.githubusercontent.com/DarkCoder28/matrix-manager-default-assets/refs/heads/main/fonts/7x14B.bdf");
    fs::create_dir_all(fonts_dir).expect("Couldn't create fonts folder");
    for (file_name, font) in &fonts {
        let mut file_path = fonts_dir.to_path_buf();
        file_path.push(file_name);
        let file_path = file_path.as_path();
        let file_stream = ureq::get(font).set("User-Agent", USER_AGENT).call();
        if (&file_stream).is_err() {
            error!(
                "Error downloading \"{}\"\n{}",
                file_name,
                &file_stream.unwrap_err().to_string()
            );
            panic!();
        }
        let file_stream = file_stream.unwrap();
        if file_stream.status() == 200 {
            fs::write(file_path, file_stream.into_string().unwrap().as_bytes()).unwrap();
        } else {
            error!(
                "Error downloading \"{}\"\nHTTP Response Code {}: {}",
                file_name,
                (&file_stream).status(),
                (&file_stream).status_text()
            );
            panic!();
        }
    }
}
