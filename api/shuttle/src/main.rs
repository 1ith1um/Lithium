use std::path::PathBuf;

use axum::{routing::get, routing::get_service, Router};
use tower_http::services::{ServeDir, ServeFile};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let serve_dir = ServeDir::new(static_folder.clone());
    let fall = static_folder
        .clone()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/index.html";
    // let router = Router::new()
    //.route("/", get(|| async { "test" }))
    //.nest_service("/assets", serve_dir)
    //.fallback_service(ServeFile::new(fall));

    let router = Router::new()
        .route("/api", get(|| async { "test" }))
        .nest_service("/", serve_dir);
    //.fallback_service(ServeFile::new(fall));
    Ok(router.into())
}
