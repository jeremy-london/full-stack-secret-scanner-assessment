use secrets_scanner_api::{ApiDoc, routes};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let app = routes::router()
        .layer(axum::extract::DefaultBodyLimit::max(16 * 1024 * 1024))
        .merge(SwaggerUi::new("/swagger-ui").url("/api/openapi.json", ApiDoc::openapi()));
    let port = std::env::var("API_PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3001);
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind API listener");

    println!("Rust API listening at http://{address}");
    axum::serve(listener, app).await.expect("API server failed");
}
