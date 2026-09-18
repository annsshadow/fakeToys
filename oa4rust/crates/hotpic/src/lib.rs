use axum::{extract::Extension, extract::Path, Json, Router};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn hotpic_router(pool: Pool) -> Router {
    routes::hotpic_router(pool)
}

#[axum::debug_handler]
pub async fn exists_check(
    pool: Extension<Pool>,
    Path((application, info_id)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_hotpic WHERE application = $1 AND info_id = $2 AND deleted_at IS NULL",
            &[&application, &info_id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("allExists".to_string(), Value::Bool(count > 0)),
        (
            "count".to_string(),
            Value::Number(serde_json::Number::from(count)),
        ),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn get_by_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if id.is_empty() {
        return Err(AppError::BadRequest("id cannot be empty".to_string()));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "base64".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                ("creator".to_string(), Value::String(row.get("creator"))),
                (
                    // create_time 可为 NULL（migrations/053 的 seed 行即未提供该列），
                    // 故必须按 Option 读取；用非 Option 的 String 会在 NULL 上 panic
                    // （error deserializing column create_time）。与上方 image_url 同惯例。
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]),
        )))),
        None => Err(AppError::NotFound),
    }
}

#[axum::debug_handler]
pub async fn list_by_application_and_info_id(
    pool: Extension<Pool>,
    Path((application, info_id)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE application = $1 AND info_id = $2 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&application, &info_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                (
                    "application".to_string(),
                    Value::String(application.clone()),
                ),
                ("infoId".to_string(), Value::String(info_id.clone())),
                ("title".to_string(), Value::String(row.get("title"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                (
                    // 同上：create_time 可为 NULL，必须按 Option 读取。
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::hotpic_router(pool)
}
