pub(crate) async fn draw_image(x: Option<u8>, y: u8, image: String) -> String {
    format!("i{:02}{:02}{:=<5}", x.unwrap_or_default(), y, &image)
}