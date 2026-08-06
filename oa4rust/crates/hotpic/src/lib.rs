use axum::{
    extract::Extension, extract::Path,
    Json, Router, routing::get,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

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
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
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
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("base64".to_string(), Value::String(row.get::<_, Option<String>>("image_url").unwrap_or_default())),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ])))))
        }
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
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE application = $1 AND info_id = $2 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&application, &info_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("application".to_string(), Value::String(application.clone())),
                ("infoId".to_string(), Value::String(info_id.clone())),
                ("title".to_string(), Value::String(row.get("title"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub fn router(pool: Pool) -> axum::Router {
    axum::Router::new()
        .route("/hotpic/health", axum::routing::get(|| async { "ok" }))
        .route("/jaxrs/hotpic/user/hotpic/exists/check", get(exists_check))
        .route("/jaxrs/hotpic/user/hotpic/{id}", get(get_by_id))
        .route("/jaxrs/hotpic/user/hotpic/{application}/{infoId}", get(list_by_application_and_info_id))
        .layer(Extension(pool))
}