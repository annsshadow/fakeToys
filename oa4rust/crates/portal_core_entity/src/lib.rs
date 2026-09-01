use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post},
    Json as AxumJson, Router,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Statement};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::{portal, widget, portal_page, script};

pub async fn portal_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = portal::Entity::find()
        .order_by_asc(portal::Column::Name)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("alias".to_string(), Value::String(m.alias.clone())),
                ("description".to_string(), Value::String(m.description.clone())),
                (
                    "portalCategory".to_string(),
                    Value::String(m.portal_category.clone()),
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

pub async fn widget_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = widget::Entity::find()
        .order_by_asc(widget::Column::Name)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("alias".to_string(), Value::String(m.alias.clone())),
                ("category".to_string(), Value::String(m.category.clone())),
                ("portal".to_string(), Value::String(m.portal.clone())),
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

pub async fn page_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = portal_page::Entity::find()
        .filter(portal_page::Column::DeletedAt.is_null())
        .order_by_desc(portal_page::Column::CreateTime)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut obj = serde_json::Map::new();
            obj.insert("id".to_string(), Value::String(m.id.clone()));
            obj.insert("portalId".to_string(), Value::String(m.portal_id.clone()));
            obj.insert("name".to_string(), Value::String(m.name.clone()));
            if let Some(ref v) = m.content {
                obj.insert("content".to_string(), Value::String(v.clone()));
            }
            obj.insert("status".to_string(), Value::String(m.status.clone()));
            obj.insert(
                "createTime".to_string(),
                Value::String(
                    m.create_time
                        .clone()
                        .map(|dt| dt.to_string())
                        .unwrap_or_default(),
                ),
            );
            Value::Object(obj)
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

pub async fn page_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = portal_page::Entity::find()
        .filter(portal_page::Column::Id.eq(&id))
        .filter(portal_page::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let mut obj = serde_json::Map::new();
            obj.insert("id".to_string(), Value::String(m.id.clone()));
            obj.insert("portalId".to_string(), Value::String(m.portal_id.clone()));
            obj.insert("name".to_string(), Value::String(m.name.clone()));
            if let Some(ref v) = m.content {
                obj.insert("content".to_string(), Value::String(v.clone()));
            }
            obj.insert("status".to_string(), Value::String(m.status.clone()));
            obj.insert(
                "createTime".to_string(),
                Value::String(
                    m.create_time
                        .clone()
                        .map(|dt| dt.to_string())
                        .unwrap_or_default(),
                ),
            );
            let result = Value::Object(obj);
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_create(
    db: Extension<DatabaseConnection>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    use portal_page::ActiveModel;

    let portal_id = payload
        .get("portalId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("name is required".to_string()))?;
    let name = name.to_string();
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("active");
    let id = uuid::Uuid::new_v4().to_string();

    let active = ActiveModel {
        id: sea_orm::ActiveValue::Set(id.clone()),
        portal_id: sea_orm::ActiveValue::Set(portal_id),
        name: sea_orm::ActiveValue::Set(name.clone()),
        content: sea_orm::ActiveValue::Set(Some(content)),
        status: sea_orm::ActiveValue::Set(status.to_string()),
        create_time: sea_orm::ActiveValue::Set(None),
        update_time: sea_orm::ActiveValue::Set(None),
        deleted_at: sea_orm::ActiveValue::Set(None),
    };

    active
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("status".to_string(), Value::String(status.to_string())),
    ])))))
}

pub async fn page_update(
    db: Extension<DatabaseConnection>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = payload
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("id is required".to_string()))?;
    let id = id.to_string();
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("active");

    let result = db.0
        .execute(Statement::from_sql_and_values(
            sea_orm::DbBackend::Postgres,
            "UPDATE portal_page SET name = $1, content = $2, status = $3 WHERE id = $4 AND deleted_at IS NULL",
            vec![name.into(), content.into(), status.into(), id.clone().into()],
        ))
        .await
        .map_err(|_| AppError::Internal)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(result.rows_affected() > 0)),
        ]),
    ))))
}

pub async fn page_remove(
    db: Extension<DatabaseConnection>,
    AxumJson(payload): AxumJson<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = payload
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or(AppError::BadRequest("id is required".to_string()))?;
    let id = id.to_string();

    let now = chrono::Utc::now().naive_utc();

    let result = db.0
        .execute(Statement::from_sql_and_values(
            sea_orm::DbBackend::Postgres,
            "UPDATE portal_page SET deleted_at = $1 WHERE id = $2 AND deleted_at IS NULL",
            vec![now.into(), id.clone().into()],
        ))
        .await
        .map_err(|_| AppError::Internal)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(result.rows_affected() > 0)),
        ]),
    ))))
}

pub async fn script_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = script::Entity::find()
        .order_by_asc(script::Column::Name)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.id.clone())),
                ("name".to_string(), Value::String(m.name.clone())),
                ("alias".to_string(), Value::String(m.alias.clone())),
                ("validated".to_string(), Value::Bool(m.validated)),
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

pub fn portal_core_entity_router(_pool: deadpool_postgres::Pool) -> Router {
    Router::new()
        .route("/jaxrs/portal/portal/list", get(portal_list))
        .route("/jaxrs/portal/widget/list", get(widget_list))
        .route("/jaxrs/portal/page/list", get(page_list))
        .route("/jaxrs/portal/page/get/{id}", get(page_get))
        .route("/jaxrs/portal/page/update", post(page_update))
        .route("/jaxrs/portal/page/remove", post(page_remove))
        .route("/jaxrs/portal/script/list", get(script_list))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::portal_core_entity_router(pool)
}
