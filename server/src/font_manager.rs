use std::path::{Path, PathBuf};

use tokio::fs;

use crate::config_manager::ConfigWrapper;



pub async fn get_font_list(config: ConfigWrapper) -> Vec<String> {
    let fonts_path = get_font_path(config, None).await;
    let mut dir_enumeration = tokio::fs::read_dir(&fonts_path).await.expect(&format!("Couldn't enumerate fonts directory at '{}'", fonts_path.to_string_lossy()));
    let mut fonts = Vec::new();
    while let Some(file) = dir_enumeration.next_entry().await.expect("Unable to enumerate fonts") {
        if file.file_type().await.unwrap().is_file() {
            let file_name = file.file_name().into_string().unwrap();
            if !file_name.ends_with(".bdf") {
                continue;
            }
            fonts.push(file_name.replace(".bdf", ""));
        }
    }
    fonts
}

pub async fn get_font_path(config: ConfigWrapper, font_name: Option<&str>) -> PathBuf {
    let config_file = PathBuf::from(&config.read().await.config_path.clone());
    let mut fonts_path = config_file.parent().expect("Config file has no parent folder???").to_path_buf();
    fonts_path.push("assets/fonts/");
    if !fonts_path.exists() {
        fs::create_dir_all(&fonts_path).await.expect(&format!("Couldn't make fonts directory at '{}'", fonts_path.to_string_lossy()));
    }
    if let Some(font_name) = font_name {
        if font_name.contains("..") {
            panic!();
        }
        let mut temp_path = fonts_path.clone();
        temp_path.push(font_name);
        if !temp_path.exists() {
            fonts_path.push("5x8.bdf");
        } else {
            fonts_path.push(font_name);
        }
    }
    return fonts_path;
}