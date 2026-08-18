use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post, delete},
    Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{mind_mind, mind_folder, mind_version};

/// 获取思维导图列表
pub async fn list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = mind_mind::Entity::find()
        .order_by_desc(mind_mind::Column::CreateTime)
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
                ("folderId".to_string(), Value::String(m.folder_id.clone())),
                (
                    "description".to_string(),
                    m.description
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creator".to_string(), Value::String(m.creator.clone())),
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

/// 获取思维导图文件夹列表
pub async fn folder_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = mind_folder::Entity::find()
        .order_by_asc(mind_folder::Column::OrderNumber)
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
                (
                    "\"parentId\"".to_string(),
                    m.parent_id
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "orderNumber".to_string(),
                    Value::Number(serde_json::Number::from(m.order_number)),
                ),
                (
                    "description".to_string(),
                    m.description
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creator".to_string(), Value::String(m.creator.clone())),
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

/// 获取思维导图版本列表
pub async fn version_list(
    db: Extension<DatabaseConnection>,
    Path(mind_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = mind_version::Entity::find()
        .filter(mind_version::Column::MindId.eq(&mind_id))
        .order_by_desc(mind_version::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("mindId".to_string(), Value::String(m.mind_id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("folderId".to_string(), Value::String(m.folder_id.clone())),
                (
                    "description".to_string(),
                    m.description
                        .clone()
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creator".to_string(), Value::String(m.creator.clone())),
                (
                    "fileVersion".to_string(),
                    Value::Number(serde_json::Number::from(m.file_version)),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        m.create_time
                            .clone()
                            .map(|dt| dt.to_string())
                            .unwrap_or_default(),
                    ),
                ),
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

/// 创建思维导图
pub async fn create_mind(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use mind_mind::ActiveModel;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = payload
        .get("folderId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let creator = payload
        .get("creator")
        .and_then(|v| v.as_str())
        .unwrap_or("system")
        .to_string();

    let active = ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        folder_id: sea_orm::ActiveValue::Set(folder_id.clone()),
        description: sea_orm::ActiveValue::Set(description.clone()),
        creator: sea_orm::ActiveValue::Set(creator.clone()),
        create_time: sea_orm::ActiveValue::Set(None),
    };

    active
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("folderId".to_string(), Value::String(folder_id)),
    ])))))
}

/// 更新思维导图
pub async fn update_mind(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use mind_mind::ActiveModel;

    let model = mind_mind::Entity::find_by_id(id.clone())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::NotFound)?;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| model.name.clone());
    let folder_id = payload
        .get("folderId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| model.folder_id.clone());
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| model.description.clone())
        .unwrap_or_default();

    let mut active: ActiveModel = model.into();
    active.name = sea_orm::ActiveValue::Set(name);
    active.folder_id = sea_orm::ActiveValue::Set(folder_id);
    active.description = sea_orm::ActiveValue::Set(Some(description));

    active
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

/// 删除思维导图
pub async fn delete_mind(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let deleted = mind_mind::Entity::delete_by_id(id.clone())
        .exec(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if deleted.rows_affected == 0 {
        return Ok(Json(ActionResult::error("mind not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

/// 创建文件夹
pub async fn create_folder(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use mind_folder::ActiveModel;

    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let parent_id = payload.get("\"parentId\"").and_then(|v| v.as_str()).map(|s| s.to_string());
    let order_number = payload
        .get("orderNumber")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let creator = payload
        .get("creator")
        .and_then(|v| v.as_str())
        .unwrap_or("system")
        .to_string();

    let active = ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        parent_id: sea_orm::ActiveValue::Set(parent_id.clone()),
        order_number: sea_orm::ActiveValue::Set(order_number),
        description: sea_orm::ActiveValue::Set(description.clone()),
        creator: sea_orm::ActiveValue::Set(creator.clone()),
    };

    active
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        (
            "orderNumber".to_string(),
            Value::Number(serde_json::Number::from(order_number)),
        ),
    ])))))
}

/// 更新文件夹
pub async fn update_folder(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use mind_folder::ActiveModel;

    let model = mind_folder::Entity::find_by_id(id.clone())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or_else(|| AppError::NotFound)?;

    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| model.name.clone());
    let parent_id = payload
        .get("\"parentId\"")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| model.parent_id.clone())
        .unwrap_or_default();
    let order_number = payload
        .get("orderNumber")
        .and_then(|v| v.as_i64())
        .map(|i| i as i32)
        .unwrap_or_else(|| model.order_number);
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| model.description.clone())
        .unwrap_or_default();

    let mut active: ActiveModel = model.into();
    active.name = sea_orm::ActiveValue::Set(name);
    active.parent_id = sea_orm::ActiveValue::Set(Some(parent_id));
    active.order_number = sea_orm::ActiveValue::Set(order_number);
    active.description = sea_orm::ActiveValue::Set(Some(description));

    active
        .update(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

/// 删除文件夹
pub async fn delete_folder(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let deleted = mind_folder::Entity::delete_by_id(id.clone())
        .exec(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if deleted.rows_affected == 0 {
        return Ok(Json(ActionResult::error("folder not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

/// 创建版本
pub async fn create_version(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use mind_version::ActiveModel;

    let id = uuid::Uuid::new_v4().to_string();
    let mind_id = payload
        .get("mindId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = payload
        .get("folderId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let description = payload
        .get("description")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let creator = payload
        .get("creator")
        .and_then(|v| v.as_str())
        .unwrap_or("system")
        .to_string();
    let creator_unit = payload
        .get("creatorUnit")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let file_version = payload
        .get("fileVersion")
        .and_then(|v| v.as_i64())
        .unwrap_or(1) as i32;
    let shared = payload.get("shared").and_then(|v| v.as_bool()).unwrap_or(false);

    let active = ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        mind_id: sea_orm::ActiveValue::Set(mind_id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        folder_id: sea_orm::ActiveValue::Set(folder_id.clone()),
        description: sea_orm::ActiveValue::Set(description.clone()),
        creator: sea_orm::ActiveValue::Set(creator.clone()),
        creator_unit: sea_orm::ActiveValue::Set(Some(creator_unit.clone())),
        file_version: sea_orm::ActiveValue::Set(file_version),
        shared: sea_orm::ActiveValue::Set(shared),
        create_time: sea_orm::ActiveValue::Set(None),
        update_time: sea_orm::ActiveValue::Set(None),
    };

    active
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("mindId".to_string(), Value::String(mind_id)),
        ("name".to_string(), Value::String(name)),
        (
            "fileVersion".to_string(),
            Value::Number(serde_json::Number::from(file_version)),
        ),
    ])))))
}

/// 创建思维导图核心实体路由
pub fn mind_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    Router::new()
        .route("/jaxrs/mind/core/entity/list", get(list))
        .route("/jaxrs/mind/core/entity/folder/list", get(folder_list))
        .route(
            "/jaxrs/mind/core/entity/version/list/{mindId}",
            get(version_list),
        )
        .route("/jaxrs/mind/core/entity/mind", post(create_mind))
        .route("/jaxrs/mind/core/entity/mind/{id}", post(update_mind))
        .route("/jaxrs/mind/core/entity/mind/{id}", delete(delete_mind))
        .route("/jaxrs/mind/core/entity/folder", post(create_folder))
        .route("/jaxrs/mind/core/entity/folder/{id}", post(update_folder))
        .route("/jaxrs/mind/core/entity/folder/{id}", delete(delete_folder))
        .route("/jaxrs/mind/core/entity/version", post(create_version))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::mind_core_entity_router(pool)
}
