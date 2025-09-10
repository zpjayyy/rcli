use anyhow::Result;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::cli::http::ServeOpts;

pub async fn process_http_serve(opts: ServeOpts) -> Result<()> {
    let route = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", opts.port)).await?;
    axum::serve(listener, route).await?;
    Ok(())
}
