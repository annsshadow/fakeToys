use axum::{
    extract::Extension, extract::Path,
    routing::get, routing::post,
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{jpush_device, jpush_template};

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
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = jpush_device::Entity::find()
        .order_by_desc(jpush_device::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("userId".to_string(), Value::String(m.user_id.clone())),
                ("platform".to_string(), Value::String(m.platform.clone())),
                ("token".to_string(), Value::String(m.token.clone())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 获取推送设备详情
pub async fn device_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = jpush_device::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(m.id.clone())),
                    ("userId".to_string(), Value::String(m.user_id.clone())),
                    ("platform".to_string(), Value::String(m.platform.clone())),
                    ("token".to_string(), Value::String(m.token.clone())),
                ]),
            ))))
        }
        None => Err(AppError::NotFound),
    }
}

/// 创建推送设备
pub async fn device_create(
    db: Extension<DatabaseConnection>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let user_id = req.get("userId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let platform = req.get("platform").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let token = req.get("token").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let active_model = jpush_device::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        user_id: sea_orm::ActiveValue::Set(user_id.clone()),
        platform: sea_orm::ActiveValue::Set(platform.clone()),
        token: sea_orm::ActiveValue::Set(token.clone()),
        create_time: sea_orm::ActiveValue::NotSet,
    };

    active_model.insert(&db.0).await.map_err(|_| AppError::Internal)?;

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
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = jpush_template::Entity::find()
        .order_by_asc(jpush_template::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                ("content".to_string(), Value::String(m.content.clone())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// 获取推送模板详情
pub async fn template_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = jpush_template::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(m.id.clone())),
                    ("name".to_string(), Value::String(m.name.clone())),
                    ("title".to_string(), Value::String(m.title.clone())),
                    ("content".to_string(), Value::String(m.content.clone())),
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
pub fn jpush_core_entity_router(_pool: Pool) -> Router {
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        .route("/jaxrs/jpush/core/entity/device/list", get(device_list))
        .route("/jaxrs/jpush/core/entity/device/{id}", get(device_get))
        .route("/jaxrs/jpush/core/entity/device/create", post(device_create))
        .route("/jaxrs/jpush/core/entity/template/list", get(template_list))
        .route("/jaxrs/jpush/core/entity/template/{id}", get(template_get));
    match db {
        Some(conn) => router.layer(Extension(conn)),
        None => router,
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::jpush_core_entity_router(pool)
}
