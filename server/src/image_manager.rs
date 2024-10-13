use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::fs::{self, ReadDir};

use crate::config_manager::ConfigWrapper;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FileTree {
    File(String), Dir(String, Vec<FileTree>)
}

pub async fn get_image_list(config: ConfigWrapper) -> Vec<FileTree> {
    let images_path = get_image_path(config, None).await;
    let mut dir_enumeration = tokio::fs::read_dir(&images_path).await.expect(&format!("Couldn't enumerate images directory at '{}'", images_path.to_string_lossy()));
    let mut images = Vec::new();
    enumerate_dir(None, &mut dir_enumeration, &mut images).await;
    tracing::info!("{:#?}", &images);
    images
}

async fn enumerate_dir(dir_name: Option<String>, dir: &mut ReadDir, images: &mut Vec<FileTree>) {
    let local_tree = &mut Vec::new();
    while let Some(file) = dir.next_entry().await.expect("Unable to enumerate images") {
        if file.file_type().await.unwrap().is_file() {
            let file_name = file.file_name().into_string().unwrap();
            if !file_name.ends_with(".bmp") {
                continue;
            }
            local_tree.push(FileTree::File(file_name.replace(".bmp", "")));
        } else if file.file_type().await.unwrap().is_dir() {
            let mut dir = tokio::fs::read_dir(&file.path()).await.expect(&format!("Couldn't enumerate images directory at '{}'", file.path().to_string_lossy()));
            let fut = Box::pin(enumerate_dir(Some(file.file_name().into_string().unwrap()), &mut dir, local_tree));
            fut.await;
        }
    }
    if dir_name.is_some() {
        images.push(FileTree::Dir(dir_name.unwrap(), local_tree.to_vec()));
    } else {
        images.append(local_tree);
    }
}

pub async fn get_image_path(config: ConfigWrapper, image_name: Option<&str>) -> PathBuf {
    let config_file = PathBuf::from(&config.read().await.config_path.clone());
    let mut images_path = config_file.parent().expect("Config file has no parent folder???").to_path_buf();
    images_path.push("assets/images/");
    if !images_path.exists() {
        fs::create_dir_all(&images_path).await.expect(&format!("Couldn't make images directory at '{}'", images_path.to_string_lossy()));
    }
    if let Some(image_name) = image_name {
        if image_name.contains("..") {
            panic!();
        }
        let mut temp_path = images_path.clone();
        temp_path.push(image_name);
        if !temp_path.exists() {
            images_path.push("5x8.bdf");
        } else {
            images_path.push(image_name);
        }
    }
    return images_path;
}