use axum::{
    body::Body,
    http::{header::ALLOW, Method, Request, StatusCode},
};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

#[tokio::test]
async fn routes_register_all_methods_for_each_path() {
    let pool = PgPoolOptions::new()
        .connect_lazy("postgres://localhost/yumstash")
        .unwrap();
    let router = super::router(pool);

    for (path, expected) in [
        ("/api/recipes", &["GET", "HEAD", "POST"][..]),
        ("/api/recipes/1", &["GET", "HEAD", "PUT", "DELETE"][..]),
    ] {
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::OPTIONS)
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
        let allowed = response.headers()[ALLOW]
            .to_str()
            .unwrap();
        let methods: Vec<_> = allowed
            .split(',')
            .map(str::trim)
            .collect();
        assert_eq!(methods.len(), expected.len());
        for method in expected {
            assert!(methods.contains(method), "{method} missing from {allowed}");
        }
    }
}
