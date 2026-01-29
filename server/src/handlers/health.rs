use axum::{extract::State, response::IntoResponse, response::Response};
use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;

use crate::utils::response::success;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    timestamp: String,
}

#[derive(Serialize)]
struct HealthDbResponse {
    status: &'static str,
    database: &'static str,
    timestamp: String,
}

#[derive(Serialize)]
struct HealthReadyResponse {
    status: &'static str,
    api: &'static str,
    database: &'static str,
}

/// GET /health - Basic health check endpoint
/// Returns 200 if the API is running
pub async fn health_check() -> Response {
    let payload = HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
    };

    success(payload, "API is healthy").into_response()
}

/// GET /health/db - Database health check endpoint
/// Returns 200 if database is connected, 503 if down
pub async fn health_check_db(State(pool): State<PgPool>) -> Response {
    match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => {
            let payload = HealthDbResponse {
                status: "ok",
                database: "connected",
                timestamp: Utc::now().to_rfc3339(),
            };
            success(payload, "Database is healthy").into_response()
        }
        Err(e) => {
            tracing::error!("Database health check failed: {:?}", e);
            let payload = HealthDbResponse {
                status: "error",
                database: "disconnected",
                timestamp: Utc::now().to_rfc3339(),
            };
            (
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                axum::Json(payload),
            )
                .into_response()
        }
    }
}

/// GET /health/ready - Readiness check endpoint
/// Returns 200 only if both API and database are healthy
pub async fn health_check_ready(State(pool): State<PgPool>) -> Response {
    let db_status = match sqlx::query("SELECT 1").fetch_one(&pool).await {
        Ok(_) => "ok",
        Err(e) => {
            tracing::error!("Database readiness check failed: {:?}", e);
            "error"
        }
    };

    let api_status = "ok";

    if db_status == "ok" && api_status == "ok" {
        let payload = HealthReadyResponse {
            status: "ready",
            api: api_status,
            database: db_status,
        };
        success(payload, "Service is ready").into_response()
    } else {
        let payload = HealthReadyResponse {
            status: "not_ready",
            api: api_status,
            database: db_status,
        };
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            axum::Json(payload),
        )
            .into_response()
    }
}
