use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

mod routes;

#[axum::debug_handler]
pub async fn state_summary(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE deleted_at IS NULL) as total,
                COUNT(*) FILTER (WHERE status = 'running' AND deleted_at IS NULL) as running,
                COUNT(*) FILTER (WHERE status = 'completed' AND deleted_at IS NULL) as completed,
                COUNT(*) FILTER (WHERE status = 'expired' AND deleted_at IS NULL) as expired
            FROM x_process_bam
            "#,
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let total: i64 = row.get("total");
    let running: i64 = row.get("running");
    let completed: i64 = row.get("completed");
    let expired: i64 = row.get("expired");

    let data = Value::Object(serde_json::Map::from_iter([
        ("totalProcesses".to_string(), Value::Number(serde_json::Number::from(total))),
        ("running".to_string(), Value::Number(serde_json::Number::from(running))),
        ("completed".to_string(), Value::Number(serde_json::Number::from(completed))),
        ("expired".to_string(), Value::Number(serde_json::Number::from(expired))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn state_running(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            r#"
            SELECT DISTINCT process_key, process_name, application
            FROM x_process_bam
            WHERE status = 'running' AND deleted_at IS NULL
            ORDER BY process_name ASC
            "#,
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let applications: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("processKey".to_string(), Value::String(row.get("process_key"))),
                ("processName".to_string(), Value::String(row.get("process_name"))),
                ("application".to_string(), Value::String(row.get("application"))),
            ]))
        })
        .collect();

    let data = Value::Object(serde_json::Map::from_iter([
        ("runningCount".to_string(), Value::Number(serde_json::Number::from(applications.len() as i64))),
        ("applications".to_string(), Value::Array(applications)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn state_organization(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            r#"
            SELECT
                o.id as organization_id,
                o.name as organization_name,
                COUNT(p.id) as process_count
            FROM x_process_bam p
            JOIN x_process_organization o ON p.organization_id = o.id
            WHERE p.deleted_at IS NULL
            GROUP BY o.id, o.name
            ORDER BY process_count DESC
            "#,
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let organizations: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("organization_id"))),
                ("name".to_string(), Value::String(row.get("organization_name"))),
                ("count".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("process_count")))),
            ]))
        })
        .collect();

    let data = Value::Object(serde_json::Map::from_iter([(
        "organizations".to_string(),
        Value::Array(organizations),
    )]));

    Ok(Json(ActionResult::success(data)))
}

pub fn process_bam_router(pool: Pool) -> axum::Router {
    routes::process_bam_router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::process_bam_router(pool)
}

#[cfg(test)]
mod tests;
