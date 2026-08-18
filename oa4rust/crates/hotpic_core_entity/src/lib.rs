use axum::{
    extract::{Extension, Path},
    routing::{delete, get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::hotpic;

// 热图实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct HotPic {
    pub id: String,
    pub application: String,
    pub info_id: String,
    pub title: String,
    pub base64: String,
}

/// 获取热图列表
/// 从数据库查询 x_hotpic 表
pub async fn list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = hotpic::Entity::find()
        .order_by_desc(hotpic::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("application".to_string(), Value::String(m.application.clone())),
                ("infoId".to_string(), Value::String(m.info_id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                ("base64".to_string(), Value::String(m.base64.clone().unwrap_or_default())),
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

/// 根据应用和信息ID获取热图
/// 查询 x_hotpic 表中指定应用和信息ID的热图
pub async fn list_by_app_and_info(
    db: Extension<DatabaseConnection>,
    axum::extract::Path((application, info_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = hotpic::Entity::find()
        .filter(
            hotpic::Column::Application.eq(&application).and(hotpic::Column::InfoId.eq(&info_id)),
        )
        .order_by_desc(hotpic::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("application".to_string(), Value::String(m.application.clone())),
                ("infoId".to_string(), Value::String(m.info_id.clone())),
                ("title".to_string(), Value::String(m.title.clone())),
                ("base64".to_string(), Value::String(m.base64.clone().unwrap_or_default())),
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

/// 检查热图是否存在
/// 验证指定应用和信息ID的热图是否存在
pub async fn exists_check(
    db: Extension<DatabaseConnection>,
    axum::extract::Path((application, info_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let count: u64 = hotpic::Entity::find()
        .filter(
            hotpic::Column::Application.eq(&application).and(hotpic::Column::InfoId.eq(&info_id)),
        )
        .count(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("allExists".to_string(), Value::Bool(count > 0)),
        ("count".to_string(), Value::Number(serde_json::Number::from(count))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn create(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let application = payload.get("application").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let info_id = payload.get("infoId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let base64 = payload.get("base64").and_then(|v| v.as_str()).map(|s| s.to_string());

    let active_model = hotpic::ActiveModel {
        id: Set(id.clone()),
        application: Set(application.clone()),
        info_id: Set(info_id.clone()),
        title: Set(title.clone()),
        base64: Set(base64),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
        deleted_at: Set(None),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("application".to_string(), Value::String(application)),
        ("infoId".to_string(), Value::String(info_id)),
        ("title".to_string(), Value::String(title)),
    ])))))
}

pub async fn delete_by_id(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = hotpic::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let active = hotpic::ActiveModel {
                id: Set(m.id.clone()),
                application: Set(m.application.clone()),
                info_id: Set(m.info_id.clone()),
                title: Set(m.title.clone()),
                base64: Set(m.base64.clone()),
                create_time: Set(m.create_time.clone()),
                deleted_at: Set(Some(chrono::Utc::now().naive_utc())),
            };
            active.update(&db.0).await.map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(serde_json::json!({"success": true}))))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

/// 创建热图核心实体路由
/// 注册以下路由：
/// - /jaxrs/hotpic/core/entity/list - 热图列表
/// - /jaxrs/hotpic/core/entity/list/by/{application}/{infoId} - 按条件查询
/// - /jaxrs/hotpic/core/entity/exists/check/{application}/{infoId} - 检查存在
/// - /jaxrs/hotpic/core/entity/create - 创建热图
/// - /jaxrs/hotpic/core/entity/delete/{id} - 删除热图
pub fn hotpic_core_entity_router(_pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/hotpic/core/entity/list", get(list))
        .route("/jaxrs/hotpic/core/entity/list/by/{application}/{infoId}", get(list_by_app_and_info))
        .route(
            "/jaxrs/hotpic/core/entity/exists/check/{application}/{infoId}",
            get(exists_check),
        )
        .route("/jaxrs/hotpic/core/entity/create", post(create))
        .route("/jaxrs/hotpic/core/entity/delete/{id}", delete(delete_by_id))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::hotpic_core_entity_router(pool)
}
