use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[tokio::test]
async fn serves_documentation_page() {
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://localhost/yumstash")
        .unwrap();
    let response = super::router(pool)
        .oneshot(
            Request::builder()
                .uri("/docs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 16_384)
        .await
        .unwrap();
    let html = std::str::from_utf8(&body).unwrap();
    assert!(html.contains("SwaggerUIBundle"));
    assert!(html.contains("/api/openapi.json"));
}

#[test]
fn openapi_groups_operations_by_module() {
    let doc = serde_json::to_value(super::openapi_document()).unwrap();
    for (path, method, tag) in [
        ("/api/health", "get", "Health"),
        ("/api/recipes", "get", "Recipes"),
        ("/api/recipes", "post", "Recipes"),
        ("/api/recipes/{id}", "get", "Recipes"),
        ("/api/recipes/{id}", "put", "Recipes"),
        ("/api/recipes/{id}", "delete", "Recipes"),
    ] {
        assert_eq!(doc["paths"][path][method]["tags"], serde_json::json!([tag]));
    }
}
