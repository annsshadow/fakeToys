use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod entities;
pub mod routes;

use entities::query_view;

pub async fn query_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = query_view::Entity::find()
        .order_by_desc(query_view::Column::CreateTime)
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
                ("queryType".to_string(), Value::String(m.query_type.clone())),
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

pub fn query_express_router(_pool: Pool) -> Router {
    let db = tokio::runtime::Handle::current()
        .block_on(shared::db::create_sea_orm_pool())
        .expect("failed to create sea-orm connection");

    Router::new()
        .route("/jaxrs/query/list", get(query_list))
        .layer(Extension(db))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_express_router(pool)
}

#[cfg(test)]
mod tests;
