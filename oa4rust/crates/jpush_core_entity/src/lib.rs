use axum::{
    extract::Extension, extract::Path,
    routing::get, routing::post,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

// 推送设备实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PushDevice {
    pub id: String,
    pub user_id: String,
    pub platform: String,
    pub token: String,
}

// 推送模板实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PushTemplate {
    pub id: String,
    pub name: String,
    pub title: String,
    pub content: String,
}

/// 获取推送设备列表
/// 从数据库查询 x_jpush_device 表
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
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("platform".to_string(), Value::String(row.get("platform"))),
                ("token".to_string(), Value::String(row.get("token"))),
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

/// 获取推送设备详情
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
                    ("userId".to_string(), Value::String(row.get("user_id"))),
                    ("platform".to_string(), Value::String(row.get("platform"))),
                    ("token".to_string(), Value::String(row.get("token"))),
                ]),
            ))))
        }
        None => Err(AppError::NotFound),
    }
}

/// 创建推送设备
pub async fn device_create(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let user_id = req.get("userId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let platform = req.get("platform").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let token = req.get("token").and_then(|v| v.as_str()).unwrap_or("").to_string();

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
            ("userId".to_string(), Value::String(user_id)),
            ("platform".to_string(), Value::String(platform)),
            ("token".to_string(), Value::String(token)),
        ]),
    ))))
}

/// 获取推送模板列表
/// 从数据库查询 x_jpush_template 表
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 获取推送模板详情
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

/// 创建推送核心实体路由
/// 注册以下路由：
/// - /jaxrs/jpush/core/entity/device/list - 设备列表
/// - /jaxrs/jpush/core/entity/device/{id} - 设备详情
/// - /jaxrs/jpush/core/entity/device/create - 创建设备
/// - /jaxrs/jpush/core/entity/template/list - 模板列表
/// - /jaxrs/jpush/core/entity/template/{id} - 模板详情
pub fn jpush_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/jpush/core/entity/device/list", get(device_list))
        .route("/jaxrs/jpush/core/entity/device/{id}", get(device_get))
        .route("/jaxrs/jpush/core/entity/device/create", post(device_create))
        .route("/jaxrs/jpush/core/entity/template/list", get(template_list))
        .route("/jaxrs/jpush/core/entity/template/{id}", get(template_get))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::jpush_core_entity_router(pool)
}
