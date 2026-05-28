use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, post},
    Json, Router,
};
use http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderName, HeaderValue, Method,
};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::TraceLayer,
};

const MAX_UPLOAD_BYTES: usize = 10 * 1024 * 1024;

fn is_local_dev_origin(origin: &str) -> bool {
    let rest = match origin.strip_prefix("http://") {
        Some(r) => r,
        None => return false,
    };
    let host = rest.split(':').next().unwrap_or("");
    matches!(host, "localhost" | "127.0.0.1")
}

use crate::{
    dto::HealthResponse,
    handlers::{attachments, comments, issues, labels},
    middleware,
    state::AppState,
};

#[derive(Clone)]
struct MakeUuidRequestId;

impl MakeRequestId for MakeUuidRequestId {
    fn make_request_id<B>(&mut self, _request: &http::Request<B>) -> Option<RequestId> {
        Some(RequestId::new(middleware::request_id()))
    }
}

pub fn build_app(state: AppState, api_key: String) -> Router {
    let request_id_header = HeaderName::from_static("x-request-id");
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin: &HeaderValue, _| {
            origin
                .to_str()
                .map(is_local_dev_origin)
                .unwrap_or(false)
        }))
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([
            CONTENT_TYPE,
            ACCEPT,
            AUTHORIZATION,
            HeaderName::from_static("x-api-key"),
        ]);

    let api = Router::new()
        .route(
            "/issues",
            get(issues::list_issues).post(issues::create_issue),
        )
        .route(
            "/issues/{id}",
            get(issues::get_issue)
                .patch(issues::update_issue)
                .delete(issues::delete_issue),
        )
        .route(
            "/issues/{id}/comments",
            get(comments::list_comments).post(comments::create_comment),
        )
        .route("/comments/{id}", delete(comments::delete_comment))
        .route(
            "/labels",
            get(labels::list_labels).post(labels::create_label),
        )
        .route(
            "/issues/{id}/labels/{label_id}",
            post(labels::add_issue_label).delete(labels::remove_issue_label),
        )
        .route(
            "/issues/{id}/attachments",
            get(attachments::list_attachments).post(attachments::upload_attachment),
        )
        .route(
            "/attachments/{id}/download",
            get(attachments::download_attachment),
        )
        .route("/attachments/{id}", delete(attachments::delete_attachment))
        .route_layer(from_fn_with_state(api_key, middleware::require_api_key))
        .layer(RequestBodyLimitLayer::new(MAX_UPLOAD_BYTES));

    Router::new()
        .route("/health", get(|| async { Json(HealthResponse { status: "ok" }) }))
        .nest("/api", api)
        .with_state(state)
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(request_id_header, MakeUuidRequestId))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
