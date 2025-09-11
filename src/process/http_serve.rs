use std::sync::Arc;

use anyhow::Result;
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use tokio::{
    fs::{self},
    net::TcpListener,
};
use tower_http::services::ServeDir;

use crate::cli::http::ServeOpts;

struct HttpState {
    path: String,
}

pub async fn process_http_serve(opts: ServeOpts) -> Result<()> {
    let http_state = HttpState {
        path: opts.path.clone(),
    };
    let route = Router::new()
        .route("/{*path}", get(file_handler))
        .nest_service("/tower", ServeDir::new(&opts.path))
        .with_state(Arc::new(http_state));
    let listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", opts.port)).await?;
    axum::serve(listener, route).await?;
    println!("🚀 HTTP server starting on http://localhost:{}", opts.port);
    Ok(())
}

async fn file_handler(
    State(http_state): State<Arc<HttpState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let file_path = format!("{}/{}", http_state.path, path);
    let path_buf = std::path::Path::new(&file_path);
    if !path_buf.exists() {
        return (StatusCode::NOT_FOUND, "File not found".to_string());
    }
    let content = fs::read_to_string(file_path).await.unwrap();
    (StatusCode::OK, content)
}
