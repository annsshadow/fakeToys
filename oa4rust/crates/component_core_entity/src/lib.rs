use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ComponentInfo {
    pub id: String,
    pub name: String,
    pub title: String,
    pub r#type: String,
    pub visible: bool,
    pub order_number: Option<i32>,
    pub path: String,
    pub icon_path: String,
}

pub async fn component_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("comp-001".to_string())),
            ("name".to_string(), Value::String("desktop".to_string())),
            ("title".to_string(), Value::String("工作台".to_string())),
            ("type".to_string(), Value::String("system".to_string())),
            ("visible".to_string(), Value::Bool(true)),
            ("orderNumber".to_string(), Value::Number(serde_json::Number::from(1))),
            ("path".to_string(), Value::String("/desktop".to_string())),
            ("iconPath".to_string(), Value::String("/icon/desktop.png".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("comp-002".to_string())),
            ("name".to_string(), Value::String("message".to_string())),
            ("title".to_string(), Value::String("消息".to_string())),
            ("type".to_string(), Value::String("system".to_string())),
            ("visible".to_string(), Value::Bool(true)),
            ("orderNumber".to_string(), Value::Number(serde_json::Number::from(2))),
            ("path".to_string(), Value::String("/message".to_string())),
            ("iconPath".to_string(), Value::String("/icon/message.png".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("comp-003".to_string())),
            ("name".to_string(), Value::String("calendar".to_string())),
            ("title".to_string(), Value::String("日程".to_string())),
            ("type".to_string(), Value::String("custom".to_string())),
            ("visible".to_string(), Value::Bool(false)),
            ("orderNumber".to_string(), Value::Number(serde_json::Number::from(3))),
            ("path".to_string(), Value::String("/calendar".to_string())),
            ("iconPath".to_string(), Value::String("/icon/calendar.png".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn component_get(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("comp-001".to_string())),
            ("name".to_string(), Value::String("desktop".to_string())),
            ("title".to_string(), Value::String("工作台".to_string())),
            ("type".to_string(), Value::String("system".to_string())),
            ("visible".to_string(), Value::Bool(true)),
            ("orderNumber".to_string(), Value::Number(serde_json::Number::from(1))),
            ("path".to_string(), Value::String("/desktop".to_string())),
            ("iconPath".to_string(), Value::String("/icon/desktop.png".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("comp-002".to_string())),
            ("name".to_string(), Value::String("message".to_string())),
            ("title".to_string(), Value::String("消息".to_string())),
            ("type".to_string(), Value::String("system".to_string())),
            ("visible".to_string(), Value::Bool(true)),
            ("orderNumber".to_string(), Value::Number(serde_json::Number::from(2))),
            ("path".to_string(), Value::String("/message".to_string())),
            ("iconPath".to_string(), Value::String("/icon/message.png".to_string())),
        ])),
    ];

    let component = data.iter().find(|v| {
        v.get("id").map(|id| id.as_str() == Some(&flag)).unwrap_or(false)
            || v.get("name").map(|n| n.as_str() == Some(&flag)).unwrap_or(false)
    });

    match component {
        Some(c) => Ok(Json(ActionResult::success(c.clone()))),
        None => Err(AppError::NotFound),
    }
}

pub async fn component_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let count = 3i64;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub fn component_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/component/core/entity/list/all", get(component_list_all))
        .route("/jaxrs/component/core/entity/{flag}", get(component_get))
        .route("/jaxrs/component/core/entity/count", get(component_count))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
