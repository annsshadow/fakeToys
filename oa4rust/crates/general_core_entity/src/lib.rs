use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{
    general_application_dict, general_application_dict_item, general_file, general_invoice,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApplicationDict {
    pub id: String,
    pub name: String,
    pub application: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApplicationDictItem {
    pub id: String,
    pub dict_id: String,
    pub name: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GeneralFile {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub size: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Invoice {
    pub id: String,
    pub number: String,
    pub date: String,
    pub amount: f64,
    pub status: String,
}

pub async fn dict_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = general_application_dict::Entity::find()
        .order_by_asc(general_application_dict::Column::Name)
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
                ("application".to_string(), Value::String(m.application.clone())),
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

pub async fn dict_item_list(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(dict_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = general_application_dict_item::Entity::find()
        .filter(general_application_dict_item::Column::DictId.eq(&dict_id))
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("dictId".to_string(), Value::String(m.dict_id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("value".to_string(), Value::String(m.value.clone())),
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

pub async fn dict_create(
    db: Extension<DatabaseConnection>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let application = payload
        .get("application")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let active_model = general_application_dict::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        application: sea_orm::ActiveValue::Set(application.clone()),
    };

    active_model.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("application".to_string(), Value::String(application)),
        ]),
    ))))
}

pub async fn dict_get(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_application_dict::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("application".to_string(), Value::String(m.application.clone())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn dict_update(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let application = payload
        .get("application")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let model = general_application_dict::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let mut active: general_application_dict::ActiveModel = m.into();
            active.name = sea_orm::ActiveValue::Set(name.clone());
            active.application = sea_orm::ActiveValue::Set(application.clone());
            active.update(&db.0).await.map_err(|_| AppError::Internal)?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("saved".to_string(), Value::Bool(true)),
                    ("name".to_string(), Value::String(name)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn dict_delete(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let result = general_application_dict::Entity::delete_by_id(&id)
        .exec(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if result.rows_affected == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn dict_item_create(
    db: Extension<DatabaseConnection>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let dict_id = payload
        .get("dictId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let value = payload.get("value").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let active_model = general_application_dict_item::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        dict_id: sea_orm::ActiveValue::Set(dict_id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        value: sea_orm::ActiveValue::Set(value.clone()),
    };

    active_model.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("dictId".to_string(), Value::String(dict_id)),
            ("name".to_string(), Value::String(name)),
            ("value".to_string(), Value::String(value)),
        ]),
    ))))
}

pub async fn dict_item_get(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_application_dict_item::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("dictId".to_string(), Value::String(m.dict_id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("value".to_string(), Value::String(m.value.clone())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict item not found"))),
    }
}

pub async fn dict_item_update(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let dict_id = payload
        .get("dictId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let value = payload.get("value").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let model = general_application_dict_item::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let mut active: general_application_dict_item::ActiveModel = m.into();
            active.dict_id = sea_orm::ActiveValue::Set(dict_id.clone());
            active.name = sea_orm::ActiveValue::Set(name.clone());
            active.value = sea_orm::ActiveValue::Set(value.clone());
            active.update(&db.0).await.map_err(|_| AppError::Internal)?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("saved".to_string(), Value::Bool(true)),
                    ("name".to_string(), Value::String(name)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("dict item not found"))),
    }
}

pub async fn dict_item_delete(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let result = general_application_dict_item::Entity::delete_by_id(&id)
        .exec(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if result.rows_affected == 0 {
        return Ok(Json(ActionResult::error("dict item not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = general_file::Entity::find()
        .order_by_desc(general_file::Column::CreateTime)
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
                ("mimeType".to_string(), Value::String(m.mime_type.clone())),
                ("size".to_string(), Value::Number(serde_json::Number::from(m.size))),
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

pub async fn invoice_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = general_invoice::Entity::find()
        .order_by_desc(general_invoice::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("number".to_string(), Value::String(m.number.clone())),
                ("date".to_string(), Value::String(m.date.clone())),
                (
                    "amount".to_string(),
                    Value::Number(
                        serde_json::Number::from_f64(m.amount).unwrap_or_else(|| serde_json::Number::from(0)),
                    ),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
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

pub async fn file_create(
    db: Extension<DatabaseConnection>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let mime_type = payload
        .get("mimeType")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let size = payload.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system");

    let active_model = general_file::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        mime_type: sea_orm::ActiveValue::Set(mime_type.clone()),
        size: sea_orm::ActiveValue::Set(size),
        creator: sea_orm::ActiveValue::Set(Some(creator.to_string())),
        create_time: sea_orm::ActiveValue::NotSet,
    };

    active_model.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("mimeType".to_string(), Value::String(mime_type)),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ]),
    ))))
}

pub async fn file_get(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_file::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("mimeType".to_string(), Value::String(m.mime_type.clone())),
                ("size".to_string(), Value::Number(serde_json::Number::from(m.size))),
                (
                    "creator".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
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
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_update(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let mime_type = payload
        .get("mimeType")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let model = general_file::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let mut active: general_file::ActiveModel = m.into();
            active.name = sea_orm::ActiveValue::Set(name.clone());
            active.mime_type = sea_orm::ActiveValue::Set(mime_type.clone());
            active.update(&db.0).await.map_err(|_| AppError::Internal)?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("saved".to_string(), Value::Bool(true)),
                    ("name".to_string(), Value::String(name)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_delete(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let result = general_file::Entity::delete_by_id(&id)
        .exec(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if result.rows_affected == 0 {
        return Ok(Json(ActionResult::error("file not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_download(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_file::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("mimeType".to_string(), Value::String(m.mime_type.clone())),
                ("size".to_string(), Value::Number(serde_json::Number::from(m.size))),
                (
                    "creator".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
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
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn invoice_create(
    db: Extension<DatabaseConnection>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let number = payload
        .get("number")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let date = payload.get("date").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let amount = payload.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("draft")
        .to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system");

    let active_model = general_invoice::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        number: sea_orm::ActiveValue::Set(number.clone()),
        date: sea_orm::ActiveValue::Set(date.clone()),
        amount: sea_orm::ActiveValue::Set(amount),
        status: sea_orm::ActiveValue::Set(status.clone()),
        creator: sea_orm::ActiveValue::Set(Some(creator.to_string())),
        create_time: sea_orm::ActiveValue::NotSet,
    };

    active_model.insert(&db.0).await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("number".to_string(), Value::String(number)),
            ("date".to_string(), Value::String(date)),
            (
                "amount".to_string(),
                Value::Number(
                    serde_json::Number::from_f64(amount).unwrap_or_else(|| serde_json::Number::from(0)),
                ),
            ),
            ("status".to_string(), Value::String(status)),
        ]),
    ))))
}

pub async fn invoice_get(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_invoice::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("number".to_string(), Value::String(m.number.clone())),
                ("date".to_string(), Value::String(m.date.clone())),
                (
                    "amount".to_string(),
                    Value::Number(
                        serde_json::Number::from_f64(m.amount).unwrap_or_else(|| serde_json::Number::from(0)),
                    ),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "creator".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
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
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("invoice not found"))),
    }
}

pub async fn invoice_update(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let number = payload
        .get("number")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let date = payload.get("date").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let amount = payload.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let model = general_invoice::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let mut active: general_invoice::ActiveModel = m.into();
            active.number = sea_orm::ActiveValue::Set(number.clone());
            active.date = sea_orm::ActiveValue::Set(date.clone());
            active.amount = sea_orm::ActiveValue::Set(amount);
            active.status = sea_orm::ActiveValue::Set(status.clone());
            active.update(&db.0).await.map_err(|_| AppError::Internal)?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("saved".to_string(), Value::Bool(true)),
                    ("number".to_string(), Value::String(number)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("invoice not found"))),
    }
}

pub async fn invoice_delete(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let result = general_invoice::Entity::delete_by_id(&id)
        .exec(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    if result.rows_affected == 0 {
        return Ok(Json(ActionResult::error("invoice not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn general_core_entity_router(_pool: Pool) -> Router {
    let db = std::panic::catch_unwind(|| {
        tokio::runtime::Handle::current()
            .block_on(shared::db::create_sea_orm_pool())
    })
    .ok()
    .and_then(|r| r.ok());

    let router = Router::new()
        .route("/jaxrs/general/dict/list", get(dict_list))
        .route("/jaxrs/general/dict/create", post(dict_create))
        .route("/jaxrs/general/dict/{id}", get(dict_get))
        .route("/jaxrs/general/dict/update/{id}", post(dict_update))
        .route("/jaxrs/general/dict/delete/{id}", post(dict_delete))
        .route("/jaxrs/general/dict/item/list/{dictId}", get(dict_item_list))
        .route("/jaxrs/general/dict/item/create", post(dict_item_create))
        .route("/jaxrs/general/dict/item/{id}", get(dict_item_get))
        .route("/jaxrs/general/dict/item/update/{id}", post(dict_item_update))
        .route("/jaxrs/general/dict/item/delete/{id}", post(dict_item_delete))
        .route("/jaxrs/general/file/list", get(file_list))
        .route("/jaxrs/general/file/create", post(file_create))
        .route("/jaxrs/general/file/{id}", get(file_get))
        .route("/jaxrs/general/file/update/{id}", post(file_update))
        .route("/jaxrs/general/file/delete/{id}", post(file_delete))
        .route("/jaxrs/general/file/download/{id}", get(file_download))
        .route("/jaxrs/general/invoice/list", get(invoice_list))
        .route("/jaxrs/general/invoice/create", post(invoice_create))
        .route("/jaxrs/general/invoice/{id}", get(invoice_get))
        .route("/jaxrs/general/invoice/update/{id}", post(invoice_update))
        .route("/jaxrs/general/invoice/delete/{id}", post(invoice_delete));
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
    crate::general_core_entity_router(pool)
}
