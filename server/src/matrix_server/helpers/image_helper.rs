use crate::{config_manager::ConfigWrapper, image_manager::get_hash_by_image_path, state_manager::StateWrapper};

pub(crate) async fn draw_image(x: Option<u8>, y: u8, image: String, legacy_mode: bool, config: ConfigWrapper, state: StateWrapper) -> String {
    if legacy_mode {
        if image.starts_with("^i") || image.starts_with("^1") || image.starts_with("^2") {
            return format!("i{:02}{:02}{:=<5}", x.unwrap_or_default(), y, &image);
        } else {
            return String::new();
        }
    }
    let img = get_hash_by_image_path(&image, config, state).await;
    if let Some(img_hash) = img {
        format!("i{:02}{:02}{:=<5}", x.unwrap_or_default(), y, &img_hash)
    } else {
        tracing::warn!("No image matching path ({})", &image);
        String::new()
    }
}