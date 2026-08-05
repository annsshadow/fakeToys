use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub async fn applications(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, app_id, disable FROM x_applications ORDER BY name",
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
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("disable".to_string(), Value::Bool(row.get("disable"))),
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

pub async fn current_style(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, app_id, disable FROM x_applications ORDER BY name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let portal_list: Vec<Value> = rows
        .iter()
        .take(3)
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("indexType".to_string(), Value::String("portal".to_string())),
            ("indexPortal".to_string(), Value::Null),
            ("indexId".to_string(), Value::Null),
            ("portalList".to_string(), Value::Array(portal_list)),
        ]),
    ))))
}

pub async fn modules_all(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("name".to_string(), Value::String("Application".to_string())),
            ("className".to_string(), Value::String("com.x.organization.core.entity.Application".to_string())),
            ("entityCount".to_string(), Value::Number(serde_json::Number::from(12))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("name".to_string(), Value::String("Person".to_string())),
            ("className".to_string(), Value::String("com.x.organization.core.entity.Person".to_string())),
            ("entityCount".to_string(), Value::Number(serde_json::Number::from(8))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("name".to_string(), Value::String("Unit".to_string())),
            ("className".to_string(), Value::String("com.x.organization.core.entity.Unit".to_string())),
            ("entityCount".to_string(), Value::Number(serde_json::Number::from(5))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("name".to_string(), Value::String("Process".to_string())),
            ("className".to_string(), Value::String("com.x.process.core.entity.Process".to_string())),
            ("entityCount".to_string(), Value::Number(serde_json::Number::from(15))),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn program_center_router() -> Router {
    Router::new()
        .route("/jaxrs/program/applications", get(applications))
        .route("/jaxrs/program/appstyle/current/style", get(current_style))
        .route("/jaxrs/program/datastructure/modules/all", get(modules_all))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/program_center/health", axum::routing::get(|| async { "TODO: program_center - real implementation needed" }))
}