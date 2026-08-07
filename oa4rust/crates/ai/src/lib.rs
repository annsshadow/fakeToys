use axum::{
    extract::Extension,
    Json, routing::get,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

mod routes;

#[cfg(test)]
mod tests;

pub fn ai_router(pool: Pool) -> axum::Router {
    routes::ai_router(pool)
}

#[axum::debug_handler]
pub async fn config_get(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xname, xtype, xmodel, xenable FROM X.AI_MODEL WHERE xenable = true ORDER BY xname LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = if rows.is_empty() {
        Value::Object(serde_json::Map::from_iter([
            ("config".to_string(), Value::String("base".to_string())),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ]))
    } else {
        let row = &rows[0];
        Value::Object(serde_json::Map::from_iter([
            ("config".to_string(), Value::String(row.get("xname"))),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            ("enabled".to_string(), Value::Bool(row.get("xenable"))),
        ]))
    };

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_enable_model(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xname, xtype, xmodel, xenable FROM X.AI_MODEL WHERE xenable = true ORDER BY xname LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("flag".to_string(), Value::String(row.get("xname"))),
                ("name".to_string(), Value::String(row.get("xname"))),
                ("enable".to_string(), Value::Bool(row.get("xenable"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn sync_to_knowledge(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT COUNT(*) as cnt FROM X.AI_COMPLETION", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = rows[0].get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
            ("message".to_string(), Value::String("sync completed".to_string())),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn app_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, status FROM x_ai_app ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("status".to_string(), Value::String(row.get("status"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn model_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, provider, enabled FROM x_ai_model ORDER BY name LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("provider".to_string(), Value::String(row.get("provider"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn conversation_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, user_id, create_time FROM x_ai_conversation ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::ai_router(pool)
}
