use axum::{
    extract::Extension,
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use uuid::Uuid;

pub mod entities;
pub mod routes;

use entities::cms_view;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn cms_express_router() -> Router {
    routes::cms_express_router()
}

#[axum::debug_handler]
pub async fn uuid_random() -> Result<Json<ActionResult<Value>>, AppError> {
    let uuid = Uuid::new_v4().to_string();
    // Java returns Array [String(uuid)]
    Ok(Json(ActionResult::success(Value::Array(vec![Value::String(uuid)]))))
}

#[axum::debug_handler]
pub async fn template_form_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xcategory FROM x_cms_templateform ORDER BY xname LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("name".to_string(), Value::String(row.get("xname"))),
                ("category".to_string(), Value::String(row.get("xcategory"))),
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

#[axum::debug_handler]
pub async fn view_list_all(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = cms_view::Entity::find()
        .order_by_asc(cms_view::Column::Xname)
        .limit(50)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(m.xid.clone())),
                ("name".to_string(), Value::String(m.xname.clone())),
                ("appId".to_string(), Value::String(m.xapp_id.clone())),
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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    cms_express_router()
        .layer(Extension(pool))
}
