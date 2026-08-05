use axum::{
    extract::Extension,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

pub async fn get_general_control_status(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, system_name, maintenance_mode, allow_registration, version FROM x_general_assemble_control_config LIMIT 1",
            &[],
        )
        .await;

    let data = match row {
        Ok(r) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("systemName".to_string(), Value::String(r.get("system_name"))),
            ("maintenanceMode".to_string(), Value::Bool(r.get("maintenance_mode"))),
            ("allowRegistration".to_string(), Value::Bool(r.get("allow_registration"))),
            ("version".to_string(), Value::String(r.get("version"))),
        ]),
        Err(_) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(String::new())),
            ("systemName".to_string(), Value::String(String::new())),
            ("maintenanceMode".to_string(), Value::Bool(false)),
            ("allowRegistration".to_string(), Value::Bool(true)),
            ("version".to_string(), Value::String(String::new())),
        ]),
    };

    Ok(Json(ActionResult::success(Value::Object(data))))
}

pub async fn update_general_control_status(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let maintenance_mode: bool = payload.get("maintenanceMode").and_then(|v| v.as_bool()).unwrap_or(false);
    let allow_registration: bool = payload.get("allowRegistration").and_then(|v| v.as_bool()).unwrap_or(true);

    client
        .execute(
            "UPDATE x_general_assemble_control_config SET maintenance_mode = $1, allow_registration = $2 WHERE id = 'global'",
            &[&maintenance_mode, &allow_registration],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("maintenanceMode".to_string(), Value::Bool(maintenance_mode)),
        ("allowRegistration".to_string(), Value::Bool(allow_registration)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn get_module_permissions(
    pool: Extension<Pool>,
    axum::extract::Path(module): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, module_name, user_id, can_view, can_edit, can_delete FROM x_general_assemble_control_permission WHERE module_name = $1",
            &[&module],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("moduleName".to_string(), Value::String(row.get("module_name"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("canView".to_string(), Value::Bool(row.get("can_view"))),
                ("canEdit".to_string(), Value::Bool(row.get("can_edit"))),
                ("canDelete".to_string(), Value::Bool(row.get("can_delete"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("module".to_string(), Value::String(module)),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub fn general_assemble_control_router(pool: Pool) -> Router {
    routes::general_assemble_control_routes(pool)
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/general_assemble_control/health", axum::routing::get(|| async { "TODO: general_assemble_control - real implementation needed" }))
}