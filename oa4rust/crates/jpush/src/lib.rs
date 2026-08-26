use axum::{
    extract::Extension, extract::Path,
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

pub fn jpush_router(pool: Pool) -> Router {
    routes::jpush_router(pool)
}

pub async fn hello() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::String("hello".to_string()))))
}

#[axum::debug_handler]
pub async fn device_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, user_id, platform, token FROM x_jpush_device ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("\"userId\"".to_string(), Value::String(row.get("user_id"))),
                ("platform".to_string(), Value::String(row.get("platform"))),
                ("token".to_string(), Value::String(row.get("token"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn device_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, user_id, platform, token FROM x_jpush_device WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("\"userId\"".to_string(), Value::String(row.get("user_id"))),
                    ("platform".to_string(), Value::String(row.get("platform"))),
                    ("token".to_string(), Value::String(row.get("token"))),
                ]),
            ))))
        }
        None => Err(AppError::NotFound),
    }
}

#[axum::debug_handler]
pub async fn device_create(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let user_id = req.get("\"userId\"").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let platform = req.get("platform").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let token = req.get("token").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_jpush_device (id, user_id, platform, token, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &user_id, &platform, &token],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("\"userId\"".to_string(), Value::String(user_id)),
            ("platform".to_string(), Value::String(platform)),
            ("token".to_string(), Value::String(token)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn template_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, title, content FROM x_jpush_template ORDER BY name LIMIT 20",
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
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn template_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, title, content FROM x_jpush_template WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("title".to_string(), Value::String(row.get("title"))),
                    ("content".to_string(), Value::String(row.get("content"))),
                ]),
            ))))
        }
        None => Err(AppError::NotFound),
    }
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::jpush_router(pool)
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

