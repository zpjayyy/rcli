use anyhow::Result;
use axum::{
    Router,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use std::path::Path as StdPath;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

use crate::cli::http::ServeOpts;

pub async fn serve(opts: ServeOpts) -> Result<()> {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/health", get(health_handler))
        .nest_service("/static", ServeDir::new(&opts.path))
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(format!("0.0.0.0:{}", opts.port)).await?;

    println!("🚀 HTTP server starting on http://localhost:{}", opts.port);
    println!("📁 Serving files from: {}", opts.path);
    println!("🔗 Health check: http://localhost:{}/health", opts.port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index_handler() -> impl IntoResponse {
    let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>RCLI HTTP Server</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .container { max-width: 800px; margin: 0 auto; }
            .header { background: #f0f0f0; padding: 20px; border-radius: 8px; margin-bottom: 20px; }
            .endpoint { background: #e8f4fd; padding: 15px; border-radius: 5px; margin: 10px 0; }
            code { background: #f5f5f5; padding: 2px 6px; border-radius: 3px; }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <h1>🚀 RCLI HTTP Server</h1>
                <p>Welcome to the RCLI HTTP server! This server is running and ready to serve files.</p>
            </div>

            <h2>Available Endpoints:</h2>
            <div class="endpoint">
                <strong>GET /</strong> - This page (server info)
            </div>
            <div class="endpoint">
                <strong>GET /health</strong> - Health check endpoint
            </div>
            <div class="endpoint">
                <strong>GET /static/*</strong> - Static file serving
            </div>

            <h2>Usage:</h2>
            <p>Access static files using: <code>http://localhost:8080/static/filename</code></p>
            <p>For example: <code>http://localhost:8080/static/README.md</code></p>
        </div>
    </body>
    </html>
    "#;

    Html(html)
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
