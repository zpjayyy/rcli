use std::sync::Arc;

use anyhow::Result;
use axum::{
    Router,
    extract::{Path, State},
    routing::get,
};
use tokio::{fs, net::TcpListener};

use crate::cli::http::ServeOpts;

struct HttpState {
    path: String,
}

pub async fn process_http_serve(opts: ServeOpts) -> Result<()> {
    let http_state = HttpState { path: opts.path };
    let route = Router::new()
        .route("/{*path}", get(file_handler))
        .with_state(Arc::new(http_state));
    let listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", opts.port)).await?;
    axum::serve(listener, route).await?;
    println!("🚀 HTTP server starting on http://localhost:{}", opts.port);
    Ok(())
}

async fn file_handler(
    State(http_state): State<Arc<HttpState>>,
    Path(path): Path<String>,
) -> String {
    let file_path = format!("{}/{}", http_state.path, path);
    fs::read_to_string(file_path).await.unwrap()
}
