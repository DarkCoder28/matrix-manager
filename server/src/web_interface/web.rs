use crate::{
    config_manager::{Boards, ConfigWrapper},
    font_manager,
    image_manager::{get_image_list, get_image_path},
    state_manager::StateWrapper,
};
use axum::{
    body::{Body, Bytes},
    extract::Path,
    http::{Response, StatusCode},
    routing::{get, post},
    Extension, Json, Router,
};
use shared::{board_variables::BoardVariables, device_config::DeviceConfigs};
use tokio::fs;

static FAVICON: &[u8] = include_bytes!("./favicon.ico");
static WASM_BINARY: &[u8] = include_bytes!("../../../wasm_project/pkg/wasm_project_bg.wasm");
static JS_LOADER: &str = include_str!("../../../wasm_project/pkg/wasm_project.js");

pub async fn run_web_server(config: ConfigWrapper, state: StateWrapper) {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/default.css", get(serve_css))
        .route("/app.js", get(serve_js_loader))
        .route("/wasm_project_bg.wasm", get(serve_wasm))
        .route("/favicon.ico", get(serve_favicon))
        .route("/api/fonts", get(serve_fonts))
        .route("/api/get_font/:font", get(serve_font))
        .route("/api/boards", get(serve_boards))
        .route("/api/update/boards", post(accept_boards_update))
        .route("/api/vars", get(serve_vars))
        .route("/api/update/vars", post(accept_vars_update))
        .route("/api/devices", get(serve_devices))
        .route("/api/update/devices", post(accept_device_update))
        .route("/api/images", get(serve_image_index))
        .route("/api/image_list", get(serve_image_list))
        .route("/api/get_image/:image", get(serve_image))
        .route("/config.json", get(serve_current_config))
        .fallback(serve_index)
        .layer(Extension(config.clone()))
        .layer(Extension(state.clone()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:12345")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn accept_boards_update(
    Extension(config): Extension<ConfigWrapper>,
    Json(json): Json<Boards>,
) -> Response<Body> {
    {
        let mut config_mut = config.write().await;
        config_mut.update_boards(&json);
    }
    let config2 = config.clone();
    tokio::spawn(async move {
        config2.read().await.save();
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::from(StatusCode::OK.to_string()))
        .unwrap()
}

async fn accept_vars_update(
    Extension(config): Extension<ConfigWrapper>,
    Json(json): Json<BoardVariables>,
) -> Response<Body> {
    {
        let mut config_mut = config.write().await;
        config_mut.board_variables.clear();
        for (name, var) in json {
            config_mut.board_variables.insert(name, var);
        }
    }
    let config2 = config.clone();
    tokio::spawn(async move {
        config2.read().await.save();
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::from(StatusCode::OK.to_string()))
        .unwrap()
}

async fn accept_device_update(
    Extension(config): Extension<ConfigWrapper>,
    Json(json): Json<DeviceConfigs>,
) -> Response<Body> {
    {
        let mut config_mut = config.write().await;
        config_mut.device_configs.clear();
        for (ip, data) in json {
            config_mut.device_configs.insert(ip, data);
        }
    }
    let config2 = config.clone();
    tokio::spawn(async move {
        config2.read().await.save();
    });
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::from(StatusCode::OK.to_string()))
        .unwrap()
}

async fn serve_index() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(include_str!("./index.html")))
        .unwrap()
}

async fn serve_css() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(Body::from(include_str!("./default.css")))
        .unwrap()
}

async fn serve_js_loader() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(Body::from(JS_LOADER))
        .unwrap()
}

async fn serve_wasm() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/wasm")
        .body(Body::from(Bytes::from(WASM_BINARY)))
        .unwrap()
}

async fn serve_favicon() -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/vnd.microsoft.icon")
        .body(Body::from(Bytes::from(FAVICON)))
        .unwrap()
}

async fn serve_boards(Extension(config): Extension<ConfigWrapper>) -> Response<Body> {
    let config = config.read().await;
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(
            serde_json::to_string(config.get_boards()).unwrap(),
        ))
        .unwrap()
}

async fn serve_vars(Extension(config): Extension<ConfigWrapper>) -> Response<Body> {
    let config = config.read().await;
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(
            serde_json::to_string(&config.board_variables).unwrap(),
        ))
        .unwrap()
}

async fn serve_devices(Extension(config): Extension<ConfigWrapper>) -> Response<Body> {
    let config = config.read().await;
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(
            serde_json::to_string(&config.device_configs).unwrap(),
        ))
        .unwrap()
}

async fn serve_fonts(Extension(config): Extension<ConfigWrapper>) -> Response<Body> {
    let fonts = font_manager::get_font_list(config.clone()).await;
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&fonts).unwrap()))
        .unwrap()
}

async fn serve_font(
    Path(font): Path<String>,
    Extension(config): Extension<ConfigWrapper>,
) -> Response<Body> {
    let font_path = font_manager::get_font_path(config.clone(), Some(&font)).await;
    if !font_path.exists() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body(Body::from(StatusCode::NOT_FOUND.as_str()))
            .unwrap();
    }
    let file = fs::read(font_path).await;
    if file.is_err() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body(Body::from(StatusCode::NOT_FOUND.as_str()))
            .unwrap();
    }
    let file = file.unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-font-bdf")
        .body(Body::from(Bytes::from(file)))
        .unwrap()
}

async fn serve_image_index(
    Extension(config): Extension<ConfigWrapper>,
    Extension(state): Extension<StateWrapper>,
) -> Response<Body> {
    let images_path = get_image_list(config.clone(), state.clone(), false).await;
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&images_path).unwrap()))
        .unwrap()
}

async fn serve_image_list(
    Extension(config): Extension<ConfigWrapper>,
    Extension(state): Extension<StateWrapper>,
) -> Response<Body> {
    let images_path = get_image_list(config.clone(), state.clone(), false).await;
    let images: Vec<String> = images_path.values().map(|x|x.to_owned()).collect();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&images).unwrap()))
        .unwrap()
}

async fn serve_image(
    Extension(config): Extension<ConfigWrapper>,
    Extension(state): Extension<StateWrapper>,
    Path(image): Path<String>,
) -> Response<Body> {
    let images = get_image_list(config.clone(), state.clone(), true).await;
    let img_path = images.get(&image);
    if img_path.is_none() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body(Body::from(StatusCode::NOT_FOUND.to_string()))
            .unwrap();
    }
    let img_path = img_path.unwrap();
    let img_path = get_image_path(config.clone(), Some(img_path)).await;
    //
    let file = fs::read(img_path).await;
    if file.is_err() {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body(Body::from(StatusCode::NOT_FOUND.to_string()))
            .unwrap();
    }
    let file = file.unwrap();
    //
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/bmp")
        .body(Body::from(Bytes::from(file)))
        .unwrap()
}

async fn serve_current_config(Extension(config): Extension<ConfigWrapper>) -> Response<Body> {
    let config = config.read().await;
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string_pretty(&*config).unwrap()))
        .unwrap()
}