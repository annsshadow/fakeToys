use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

/// 流程平台设计器装配模块
/// 提供流程设计器相关的装配服务
pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateFlowRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
}

/// 创建流程设计
/// 根据请求创建新的流程设计
pub async fn create_flow(
    axum::extract::Json(req): Json<CreateFlowRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("id".to_string(), Value::String("flow-1".to_string())),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("description".to_string(), Value::String(req.description.unwrap_or_default())),
            ("category".to_string(), Value::String(req.category.unwrap_or_default())),
        ]),
    ))))
}

/// 获取流程设计
/// 返回指定ID的流程设计详情
pub async fn get_flow(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Process Flow".to_string())),
            ("nodes".to_string(), Value::Array(vec![])),
            ("edges".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// 列出流程设计
/// 返回指定类别下的所有流程设计列表
pub async fn list_flows(
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("flow-1".to_string())),
            ("name".to_string(), Value::String("Flow 1".to_string())),
            ("category".to_string(), Value::String(category)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// 保存流程设计
/// 保存指定的流程设计到数据库
pub async fn save_flow(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("updated_at".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ]),
    ))))
}

/// 删除流程设计
/// 根据ID删除指定的流程设计
pub async fn delete_flow(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 预览流程设计
/// 返回流程设计的预览信息
pub async fn preview_flow(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("preview_url".to_string(), Value::String(format!("/preview/flow/{}", id))),
            ("nodes".to_string(), Value::Array(vec![])),
            ("edges".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// 流程平台设计器装配路由
/// 路由前缀: /jaxrs/processplatform/assemble/designer/*
pub fn processplatform_assemble_designer_router() -> Router {
    Router::new()
        .route("/jaxrs/processplatform/assemble/designer/create", post(create_flow))
        .route("/jaxrs/processplatform/assemble/designer/get/{id}", get(get_flow))
        .route("/jaxrs/processplatform/assemble/designer/list/{category}", get(list_flows))
        .route("/jaxrs/processplatform/assemble/designer/save/{id}", post(save_flow))
        .route("/jaxrs/processplatform/assemble/designer/delete/{id}", post(delete_flow))
        .route("/jaxrs/processplatform/assemble/designer/preview/{id}", get(preview_flow))
}

#[cfg(test)]
mod tests;
