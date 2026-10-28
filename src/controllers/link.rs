

use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::{Json, debug_handler};
use crate::error::error::AppError;
use crate::models::links::{Link, CreateLink, Pagination};

async fn find_by_code(code: &str) -> Result<Link, AppError> {
    if code.is_empty() {
        return Err(AppError::InvalidInput("code cannot be empty".to_string()));
    }
    Ok(Link {
        code: code.to_string(),
        url: "".to_string(),
        clicks: 32,
    })
}

#[debug_handler]
pub async fn health() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Server is alive.")
}

#[debug_handler]
pub async fn get_link(Path(code): Path<String>) -> Result<Json<Link>, AppError> {
    if code.is_empty() {
        return Err(AppError::InvalidInput("code cannot be empty".to_string()));
    }
    let link = find_by_code(&code).await?;

    Ok(Json(link))
}

#[debug_handler]
pub async fn list_links(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    format!("liste of links page={} limit={}", page, limit)
}




#[debug_handler]
pub async fn create_link(Json(body): Json<CreateLink>) -> (StatusCode, Json<Link>) {
    let link = Link {
        url: body.url,
        code: "abc123".to_string(),
        clicks: 32,
    };
    (StatusCode::CREATED, Json(link))
}
