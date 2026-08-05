use axum::{
    extract::Extension, extract::Path,
    Json,
};
use serde::{Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[derive(Debug, Serialize, Clone)]
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

fn mock_components() -> Vec<ComponentInfo> {
    vec![
        ComponentInfo {
            id: "comp-001".to_string(),
            name: "desktop".to_string(),
            title: "工作台".to_string(),
            r#type: "system".to_string(),
            visible: true,
            order_number: Some(1),
            path: "/desktop".to_string(),
            icon_path: "/icon/desktop.png".to_string(),
        },
        ComponentInfo {
            id: "comp-002".to_string(),
            name: "message".to_string(),
            title: "消息".to_string(),
            r#type: "system".to_string(),
            visible: true,
            order_number: Some(2),
            path: "/message".to_string(),
            icon_path: "/icon/message.png".to_string(),
        },
        ComponentInfo {
            id: "comp-003".to_string(),
            name: "calendar".to_string(),
            title: "日程".to_string(),
            r#type: "custom".to_string(),
            visible: false,
            order_number: Some(3),
            path: "/calendar".to_string(),
            icon_path: "/icon/calendar.png".to_string(),
        },
    ]
}

pub async fn list_all(
    _pool: Extension<deadpool_postgres::Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let components = mock_components();
    let data: Vec<Value> = components
        .iter()
        .map(|c| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(c.id.clone())),
                ("name".to_string(), Value::String(c.name.clone())),
                ("title".to_string(), Value::String(c.title.clone())),
                ("type".to_string(), Value::String(c.r#type.clone())),
                ("visible".to_string(), Value::Bool(c.visible)),
                ("orderNumber".to_string(), c.order_number.map(|v| Value::Number(serde_json::Number::from(v))).unwrap_or(Value::Null)),
                ("path".to_string(), Value::String(c.path.clone())),
                ("iconPath".to_string(), Value::String(c.icon_path.clone())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn get_component(
    _pool: Extension<deadpool_postgres::Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let components = mock_components();
    let component = components.iter().find(|c| c.id == flag || c.name == flag);

    match component {
        Some(c) => {
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(c.id.clone())),
                ("name".to_string(), Value::String(c.name.clone())),
                ("title".to_string(), Value::String(c.title.clone())),
                ("type".to_string(), Value::String(c.r#type.clone())),
                ("visible".to_string(), Value::Bool(c.visible)),
                ("orderNumber".to_string(), c.order_number.map(|v| Value::Number(serde_json::Number::from(v))).unwrap_or(Value::Null)),
                ("path".to_string(), Value::String(c.path.clone())),
                ("iconPath".to_string(), Value::String(c.icon_path.clone())),
            ])))))
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn count(
    _pool: Extension<deadpool_postgres::Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let components = mock_components();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(components.len() as i64))),
    ])))))
}

pub mod routes;

pub use routes::component_router;

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/component/health", axum::routing::get(|| async { "TODO: component - real implementation needed" }))
}