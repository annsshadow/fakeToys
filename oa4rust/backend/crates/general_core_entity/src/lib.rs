// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Statement,
};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{
    general_application_dict, general_application_dict_item, general_file, general_invoice,
};

/// 会话存在档：无会话返回 401。本 crate 全 sea_orm，不依赖 deadpool，门在 DB 写之前，
/// 用于关闭 R7（已认证路由但体内无会话无门禁）。dict 类全局配置仍为会话档；
/// file/invoice 的个人数据按「本人或管理员」在 SQL WHERE 内收口（见下方谓词）。
fn require_session(
    session: Option<Extension<shared::session::Session>>,
) -> Result<shared::session::Session, AppError> {
    session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)
}

/// 个人数据归属谓词：`creator` 为本人 OR 调用者管理员。**写进 SQL WHERE**，
/// 不做「先查后判」；`creator` 为 NULL 时不命中（fail-closed）。
///
/// `$1` = 行 id，`$2` = 会话人（person_unique）。
const CREATOR_OWNER_PREDICATE: &str = "id = $1 AND (creator = $2 OR EXISTS(\
     SELECT 1 FROM auth_person p \
       JOIN auth_person_role pr ON pr.person_id = p.id \
       JOIN auth_role r ON r.id = pr.role_id \
      WHERE (p.unique_id = $2 OR p.id = $2) \
        AND r.name = 'admin' AND r.deleted_at IS NULL AND r.disable = false))";

/// 写语句影响 0 行时区分「不存在」(404) 与「无权」(403)。
/// 此处的读只用于选错误码，不参与授权判定（授权已在写语句的 WHERE 内）。
async fn ownership_error(db: &DatabaseConnection, table: &str, id: &str) -> AppError {
    let sql = format!("SELECT EXISTS(SELECT 1 FROM {table} WHERE id = $1) AS row_exists");
    match db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &sql,
            vec![id.into()],
        ))
        .await
    {
        Ok(Some(row)) => match row.try_get::<bool>("", "row_exists") {
            Ok(true) => AppError::Forbidden,
            Ok(false) => AppError::NotFound,
            Err(e) => AppError::internal_with_source(concat!(file!(), ":", line!()), e),
        },
        Ok(None) => AppError::NotFound,
        Err(e) => AppError::internal_with_source(concat!(file!(), ":", line!()), e),
    }
}

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

#[allow(non_snake_case)]
pub async fn dict_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = general_application_dict::Entity::find()
        .order_by_asc(general_application_dict::Column::Name)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                (
                    "application".to_string(),
                    Value::String(m.application.clone()),
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

#[allow(non_snake_case)]
pub async fn dict_item_list(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(dict_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = general_application_dict_item::Entity::find()
        .filter(general_application_dict_item::Column::DictId.eq(&dict_id))
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

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

#[allow(non_snake_case)]
pub async fn dict_create(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
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

    active_model
        .insert(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("application".to_string(), Value::String(application)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn dict_get(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_application_dict::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                (
                    "application".to_string(),
                    Value::String(m.application.clone()),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn dict_update(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let application = payload
        .get("application")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let model = general_application_dict::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => {
            let mut active: general_application_dict::ActiveModel = m.into();
            active.name = sea_orm::ActiveValue::Set(name.clone());
            active.application = sea_orm::ActiveValue::Set(application.clone());
            active.update(&db.0).await?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("name".to_string(), Value::String(name)),
                    ("application".to_string(), Value::String(application)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn dict_delete(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let model = general_application_dict::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => {
            general_application_dict::Entity::delete_by_id(&m.id)
                .exec(&db.0)
                .await
                .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                (
                    "application".to_string(),
                    Value::String(m.application.clone()),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn dict_item_create(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let id = uuid::Uuid::new_v4().to_string();
    let dict_id = payload
        .get("dictId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let value = payload
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let active_model = general_application_dict_item::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        dict_id: sea_orm::ActiveValue::Set(dict_id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        value: sea_orm::ActiveValue::Set(value.clone()),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("dictId".to_string(), Value::String(dict_id)),
            ("name".to_string(), Value::String(name)),
            ("value".to_string(), Value::String(value)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn dict_item_get(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_application_dict_item::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

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

#[allow(non_snake_case)]
pub async fn dict_item_update(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let dict_id = payload
        .get("dictId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let value = payload
        .get("value")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let model = general_application_dict_item::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => {
            let mut active: general_application_dict_item::ActiveModel = m.into();
            active.dict_id = sea_orm::ActiveValue::Set(dict_id.clone());
            active.name = sea_orm::ActiveValue::Set(name.clone());
            active.value = sea_orm::ActiveValue::Set(value.clone());
            active.update(&db.0).await?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("dictId".to_string(), Value::String(dict_id)),
                    ("name".to_string(), Value::String(name)),
                    ("value".to_string(), Value::String(value)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("dict item not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn dict_item_delete(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_session(session)?;
    let model = general_application_dict_item::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => {
            general_application_dict_item::Entity::delete_by_id(&m.id)
                .exec(&db.0)
                .await
                .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
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

#[allow(non_snake_case)]
pub async fn file_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = general_file::Entity::find()
        .order_by_desc(general_file::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("mimeType".to_string(), Value::String(m.mime_type.clone())),
                (
                    "size".to_string(),
                    Value::Number(serde_json::Number::from(m.size)),
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

#[allow(non_snake_case)]
pub async fn invoice_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = general_invoice::Entity::find()
        .order_by_desc(general_invoice::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

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
                        serde_json::Number::from_f64(m.amount)
                            .unwrap_or_else(|| serde_json::Number::from(0)),
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

#[allow(non_snake_case)]
pub async fn file_create(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_session(session)?.person_unique;
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let mime_type = payload
        .get("mimeType")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let size = payload.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    // 归属只能来自会话：忽略请求体 creator，避免「创建时冒名」把他人行写成自己可控的归属。
    let creator = actor;

    let active_model = general_file::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        name: sea_orm::ActiveValue::Set(name.clone()),
        mime_type: sea_orm::ActiveValue::Set(mime_type.clone()),
        size: sea_orm::ActiveValue::Set(size),
        creator: sea_orm::ActiveValue::Set(Some(creator)),
        create_time: sea_orm::ActiveValue::NotSet,
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("mimeType".to_string(), Value::String(mime_type)),
            (
                "size".to_string(),
                Value::Number(serde_json::Number::from(size)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn file_get(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_file::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("mimeType".to_string(), Value::String(m.mime_type.clone())),
                (
                    "size".to_string(),
                    Value::Number(serde_json::Number::from(m.size)),
                ),
                (
                    "creator".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn file_update(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_session(session)?.person_unique;
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let mime_type = payload
        .get("mimeType")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    // 归属条件在 WHERE 内：非本人且非管理员时影响 0 行（无 TOCTOU 窗口）。
    let sql = format!(
        "UPDATE x_general_file SET name = $3, mime_type = $4 \
         WHERE {CREATOR_OWNER_PREDICATE} RETURNING id"
    );
    let updated =
        db.0.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &sql,
            vec![
                id.clone().into(),
                actor.into(),
                name.clone().into(),
                mime_type.clone().into(),
            ],
        ))
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    if updated.is_none() {
        return Err(ownership_error(&db.0, "x_general_file", &id).await);
    }

    let m = general_file::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or(AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("mimeType".to_string(), Value::String(mime_type)),
            (
                "size".to_string(),
                Value::Number(serde_json::Number::from(m.size)),
            ),
            (
                "creator".to_string(),
                Value::String(m.creator.clone().unwrap_or_default()),
            ),
            (
                "createTime".to_string(),
                Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn file_delete(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_session(session)?.person_unique;
    // 预取仅用于组装响应；授权判定在下面 DELETE 的 WHERE 内。
    let model = general_file::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    let sql = format!("DELETE FROM x_general_file WHERE {CREATOR_OWNER_PREDICATE} RETURNING id");
    let deleted =
        db.0.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &sql,
            vec![id.clone().into(), actor.into()],
        ))
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    if deleted.is_none() {
        return Err(ownership_error(&db.0, "x_general_file", &id).await);
    }

    let m = model.ok_or(AppError::NotFound)?;
    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(m.id.clone())),
        ("name".to_string(), Value::String(m.name.clone())),
        ("mimeType".to_string(), Value::String(m.mime_type.clone())),
        (
            "size".to_string(),
            Value::Number(serde_json::Number::from(m.size)),
        ),
        (
            "creator".to_string(),
            Value::String(m.creator.clone().unwrap_or_default()),
        ),
        (
            "createTime".to_string(),
            Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
        ),
    ]));
    Ok(Json(ActionResult::success(result)))
}

#[allow(non_snake_case)]
pub async fn file_download(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_file::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("mimeType".to_string(), Value::String(m.mime_type.clone())),
                (
                    "size".to_string(),
                    Value::Number(serde_json::Number::from(m.size)),
                ),
                (
                    "creator".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn invoice_create(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_session(session)?.person_unique;
    let id = uuid::Uuid::new_v4().to_string();
    let number = payload
        .get("number")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let date = payload
        .get("date")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let amount = payload
        .get("amount")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("draft")
        .to_string();
    // 归属只能来自会话：忽略请求体 creator（同 file_create）。
    let creator = actor;

    let active_model = general_invoice::ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        number: sea_orm::ActiveValue::Set(number.clone()),
        date: sea_orm::ActiveValue::Set(date.clone()),
        amount: sea_orm::ActiveValue::Set(amount),
        status: sea_orm::ActiveValue::Set(status.clone()),
        creator: sea_orm::ActiveValue::Set(Some(creator)),
        create_time: sea_orm::ActiveValue::NotSet,
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("number".to_string(), Value::String(number)),
            ("date".to_string(), Value::String(date)),
            (
                "amount".to_string(),
                Value::Number(
                    serde_json::Number::from_f64(amount)
                        .unwrap_or_else(|| serde_json::Number::from(0)),
                ),
            ),
            ("status".to_string(), Value::String(status)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn invoice_get(
    db: Extension<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = general_invoice::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match model {
        Some(m) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("number".to_string(), Value::String(m.number.clone())),
                ("date".to_string(), Value::String(m.date.clone())),
                (
                    "amount".to_string(),
                    Value::Number(
                        serde_json::Number::from_f64(m.amount)
                            .unwrap_or_else(|| serde_json::Number::from(0)),
                    ),
                ),
                ("status".to_string(), Value::String(m.status.clone())),
                (
                    "creator".to_string(),
                    Value::String(m.creator.clone().unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("invoice not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn invoice_update(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_session(session)?.person_unique;
    let number = payload
        .get("number")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let date = payload
        .get("date")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let amount = payload
        .get("amount")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    // 归属条件在 WHERE 内：非本人且非管理员时影响 0 行（无 TOCTOU 窗口）。
    let sql = format!(
        "UPDATE x_general_invoice SET number = $3, date = $4, amount = $5, status = $6 \
         WHERE {CREATOR_OWNER_PREDICATE} RETURNING id"
    );
    let updated =
        db.0.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &sql,
            vec![
                id.clone().into(),
                actor.into(),
                number.clone().into(),
                date.clone().into(),
                amount.into(),
                status.clone().into(),
            ],
        ))
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    if updated.is_none() {
        return Err(ownership_error(&db.0, "x_general_invoice", &id).await);
    }

    let m = general_invoice::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .ok_or(AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("number".to_string(), Value::String(number)),
            ("date".to_string(), Value::String(date)),
            (
                "amount".to_string(),
                Value::Number(
                    serde_json::Number::from_f64(amount)
                        .unwrap_or_else(|| serde_json::Number::from(0)),
                ),
            ),
            ("status".to_string(), Value::String(status)),
            (
                "creator".to_string(),
                Value::String(m.creator.clone().unwrap_or_default()),
            ),
            (
                "createTime".to_string(),
                Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn invoice_delete(
    db: Extension<DatabaseConnection>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let actor = require_session(session)?.person_unique;
    // 预取仅用于组装响应；授权判定在下面 DELETE 的 WHERE 内。
    let model = general_invoice::Entity::find_by_id(&id)
        .one(&db.0)
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    let sql = format!("DELETE FROM x_general_invoice WHERE {CREATOR_OWNER_PREDICATE} RETURNING id");
    let deleted =
        db.0.query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            &sql,
            vec![id.clone().into(), actor.into()],
        ))
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    if deleted.is_none() {
        return Err(ownership_error(&db.0, "x_general_invoice", &id).await);
    }

    let m = model.ok_or(AppError::NotFound)?;
    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(m.id.clone())),
        ("number".to_string(), Value::String(m.number.clone())),
        ("date".to_string(), Value::String(m.date.clone())),
        (
            "amount".to_string(),
            Value::Number(
                serde_json::Number::from_f64(m.amount)
                    .unwrap_or_else(|| serde_json::Number::from(0)),
            ),
        ),
        ("status".to_string(), Value::String(m.status.clone())),
        (
            "creator".to_string(),
            Value::String(m.creator.clone().unwrap_or_default()),
        ),
        (
            "createTime".to_string(),
            Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
        ),
    ]));
    Ok(Json(ActionResult::success(result)))
}

pub fn general_core_entity_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/api/general/dict/list", get(dict_list))
        .route("/api/general/dict/create", post(dict_create))
        .route("/api/general/dict/{id}", get(dict_get))
        .route("/api/general/dict/update/{id}", post(dict_update))
        .route("/api/general/dict/delete/{id}", post(dict_delete))
        .route("/api/general/dict/item/list/{dictId}", get(dict_item_list))
        .route("/api/general/dict/item/create", post(dict_item_create))
        .route("/api/general/dict/item/{id}", get(dict_item_get))
        .route("/api/general/dict/item/update/{id}", post(dict_item_update))
        .route("/api/general/dict/item/delete/{id}", post(dict_item_delete))
        .route("/api/general/file/list", get(file_list))
        .route("/api/general/file/create", post(file_create))
        .route("/api/general/file/{id}", get(file_get))
        .route("/api/general/file/update/{id}", post(file_update))
        .route("/api/general/file/delete/{id}", post(file_delete))
        .route("/api/general/file/download/{id}", get(file_download))
        .route("/api/general/invoice/list", get(invoice_list))
        .route("/api/general/invoice/create", post(invoice_create))
        .route("/api/general/invoice/{id}", get(invoice_get))
        .route("/api/general/invoice/update/{id}", post(invoice_update))
        .route("/api/general/invoice/delete/{id}", post(invoice_delete))
        .layer(Extension(db))
}

#[cfg(test)]
mod security_gate_tests_b65;
#[cfg(test)]
mod security_gate_tests_idor_r6w;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn router(db: DatabaseConnection) -> axum::Router {
    crate::general_core_entity_router(db)
}
