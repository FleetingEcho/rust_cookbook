//! 集成测试辅助：在测试模式下构建完整 App 并测试 HTTP API。
//!
//! 仅在 `cargo test` 时编译。

use axum::{
    body::Body,
    http::{self, Request},
    Router,
};
use serde::Serialize;
use sqlx::SqlitePool;
use tower::ServiceExt;

use crate::{
    app,
    config::Config,
    state::AppState,
};

/// 用 `:memory:` 数据库创建一个测试 App，返回 (router, pool)。
/// pool 保持连接存活直到测试结束。
pub async fn test_app() -> (Router, SqlitePool) {
    let config = Config {
        bind_addr: "0.0.0.0:0".parse().unwrap(),
        database_url: "sqlite::memory:".to_string(),
        upload_dir: std::env::temp_dir().join("issue_tracker_test_uploads"),
        api_key: "test-key".to_string(),
    };
    let state = AppState::new(&config).await.unwrap();
    let pool = state.db.clone();
    let router = app::build_app(state, config.api_key);
    (router, pool)
}

/// 发送请求并返回响应的 JSON body（作为字节）
pub async fn send_request<B: Serialize>(
    router: &mut Router,
    method: http::Method,
    path: &str,
    body: Option<B>,
) -> (http::StatusCode, Vec<u8>) {
    let req_builder = Request::builder()
        .method(method)
        .uri(path)
        .header("x-api-key", "test-key")
        .header("content-type", "application/json");

    let req = if let Some(b) = body {
        req_builder
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&b).unwrap()))
            .unwrap()
    } else {
        req_builder.body(Body::empty()).unwrap()
    };

    let response = router.oneshot(req).await.unwrap();
    let status = response.status();
    let body_bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .unwrap()
        .to_vec();
    (status, body_bytes)
}

// ── 集成测试 ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use http::Method;

    #[tokio::test]
    async fn health_check() {
        let (mut router, _pool) = test_app().await;
        let (status, body) = send_request::<()>(&mut router, Method::GET, "/health", None).await;
        assert_eq!(status, 200);
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ok");
    }

    #[tokio::test]
    async fn health_check_requires_no_auth() {
        let (router, _pool) = test_app().await;
        let req = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();
        let response = router.oneshot(req).await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn api_requires_api_key() {
        let (router, _pool) = test_app().await;
        let req = Request::builder()
            .uri("/api/issues")
            .header("content-type", "application/json")
            .body(Body::empty())
            .unwrap();
        let response = router.oneshot(req).await.unwrap();
        assert_eq!(response.status(), 401);
    }

    #[tokio::test]
    async fn create_and_list_issues() {
        let (mut router, _pool) = test_app().await;

        // 创建 3 个 issue
        for i in 0..3 {
            let (status, _) = send_request(
                &mut router,
                Method::POST,
                "/api/issues",
                Some(serde_json::json!({
                    "title": format!("Test Issue {}", i),
                    "description": format!("Description {}", i),
                    "priority": "high",
                    "issueType": "bug",
                    "createdBy": "tester",
                    "labelIds": []
                })),
            )
            .await;
            assert_eq!(status, 200);
        }

        // 列表
        let (status, body) = send_request::<()>(&mut router, Method::GET, "/api/issues", None).await;
        assert_eq!(status, 200);
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["items"].as_array().unwrap().len(), 3);
        assert_eq!(json["total"], 3);
    }

    #[tokio::test]
    async fn create_issue_validates_fields() {
        let (mut router, _pool) = test_app().await;

        // 空 title
        let (status, body) = send_request(
            &mut router,
            Method::POST,
            "/api/issues",
            Some(serde_json::json!({
                "title": "",
                "description": "desc",
                "priority": "high",
                "issueType": "bug",
                "createdBy": "tester",
            })),
        )
        .await;
        assert_eq!(status, 400);
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json["error"].as_str().unwrap().contains("title"));

        // 无效 priority
        let (status, _) = send_request(
            &mut router,
            Method::POST,
            "/api/issues",
            Some(serde_json::json!({
                "title": "title",
                "description": "desc",
                "priority": "urgent",
                "issueType": "bug",
                "createdBy": "tester",
            })),
        )
        .await;
        assert_eq!(status, 400);
    }

    #[tokio::test]
    async fn pagination_works() {
        let (mut router, _pool) = test_app().await;

        // 创建 5 个 issue
        for i in 0..5 {
            send_request(
                &mut router,
                Method::POST,
                "/api/issues",
                Some(serde_json::json!({
                    "title": format!("Issue {}", i),
                    "description": "desc",
                    "priority": "medium",
                    "issueType": "task",
                    "createdBy": "tester",
                })),
            )
            .await;
        }

        // limit=2, offset=1
        let (status, body) = send_request::<()>(
            &mut router,
            Method::GET,
            "/api/issues?limit=2&offset=1",
            None,
        )
        .await;
        assert_eq!(status, 200);
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["items"].as_array().unwrap().len(), 2);
        assert_eq!(json["total"], 5);
        assert_eq!(json["limit"], 2);
        assert_eq!(json["offset"], 1);
    }

    #[tokio::test]
    async fn create_and_get_issue() {
        let (mut router, _pool) = test_app().await;

        let (status, body) = send_request(
            &mut router,
            Method::POST,
            "/api/issues",
            Some(serde_json::json!({
                "title": "My Issue",
                "description": "My Description",
                "priority": "high",
                "issueType": "bug",
                "createdBy": "alice",
            })),
        )
        .await;
        assert_eq!(status, 200);
        let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let id = created["id"].as_i64().unwrap();

        let (status, body) =
            send_request::<()>(&mut router, Method::GET, &format!("/api/issues/{}", id), None).await;
        assert_eq!(status, 200);
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["title"], "My Issue");
        // camelCase 响应
        assert_eq!(json["createdBy"], "alice");
        assert_eq!(json["issueType"], "bug");
    }

    #[tokio::test]
    async fn labels_flow() {
        let (mut router, _pool) = test_app().await;

        // 创建 label
        let (status, body) = send_request(
            &mut router,
            Method::POST,
            "/api/labels",
            Some(serde_json::json!({ "name": "bug", "color": "#ff0000" })),
        )
        .await;
        assert_eq!(status, 200);
        let label: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let label_id = label["id"].as_i64().unwrap();

        // 创建 issue 并关联 label
        let (status, body) = send_request(
            &mut router,
            Method::POST,
            "/api/issues",
            Some(serde_json::json!({
                "title": "Buggy",
                "description": "Found a bug",
                "priority": "high",
                "issueType": "bug",
                "createdBy": "tester",
                "labelIds": [label_id],
            })),
        )
        .await;
        assert_eq!(status, 200);
        let issue: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(issue["labels"].as_array().unwrap().len(), 1);
        assert_eq!(issue["labels"][0]["name"], "bug");
    }
}
