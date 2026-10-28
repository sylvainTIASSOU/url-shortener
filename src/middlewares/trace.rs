use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;


pub async fn require_api_key(request: Request, next: Next) -> Result<Response, StatusCode> {
    let key = request.headers().get("x-api-key").and_then(|v| v.to_str().ok());

    match key {
        Some(k) if k == "secret" => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".into()),
        )).with(tracing_subscriber::fmt::layer())
        .init();
}