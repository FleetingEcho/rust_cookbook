use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::Response,
};

use crate::error::AppError;

pub async fn require_api_key(
    State(expected_key): State<String>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let provided = request
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok());

    if provided == Some(expected_key.as_str()) {
        Ok(next.run(request).await)
    } else {
        Err(AppError::Unauthorized)
    }
}

pub fn request_id() -> HeaderValue {
    HeaderValue::from_str(&uuid::Uuid::new_v4().to_string()).expect("uuid is a valid header value")
}
