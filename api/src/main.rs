use std::{env, net::SocketAddr, time::Duration};

use axum::{routing::get, Json, Router};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, ToSchema)]
struct HealthResponse {
    status: String,
}

#[derive(OpenApi)]
#[openapi(paths(health), components(schemas(HealthResponse)))]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/api/health",
    responses((status = 200, description = "API is running", body = HealthResponse))
)]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok".into() })
}

async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

fn app() -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/openapi.json", get(openapi))
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")))
        .init();

    // Create the pool lazily; feature repositories can receive it through application state.
    // No database connection is required for the liveness endpoint.
    let database_url = env::var("DATABASE_URL")?;
    let _pool = PgPoolOptions::new().max_connections(10).connect_lazy(&database_url)?;
    let address: SocketAddr = env::var("API_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into()).parse()?;
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(%address, "API listening");
    axum::serve(listener, app()).with_graceful_shutdown(shutdown()).await?;
    Ok(())
}

async fn shutdown() {
    let _ = tokio::signal::ctrl_c().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_is_available() {
        let response = app().oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap()).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
