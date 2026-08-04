use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

pub async fn application_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, category, sub_category, version, publisher FROM x_application ORDER BY name",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("subCategory".to_string(), Value::String(row.get("sub_category"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("publisher".to_string(), Value::String(row.get("publisher"))),
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

pub async fn script_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, alias, validated, creator_person FROM x_script ORDER BY name",
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
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("validated".to_string(), Value::Bool(row.get("validated"))),
                ("creatorPerson".to_string(), Value::String(row.get("creator_person"))),
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

pub async fn invoke_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("invoke-001".to_string())),
            ("name".to_string(), Value::String("API调用1".to_string())),
            ("application".to_string(), Value::String("app-001".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("invoke-002".to_string())),
            ("name".to_string(), Value::String("API调用2".to_string())),
            ("application".to_string(), Value::String("app-001".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn agent_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("agent-001".to_string())),
            ("name".to_string(), Value::String("代理服务1".to_string())),
            ("type".to_string(), Value::String("webhook".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn structure_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("struct-001".to_string())),
            ("name".to_string(), Value::String("数据结构1".to_string())),
            ("type".to_string(), Value::String("table".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("struct-002".to_string())),
            ("name".to_string(), Value::String("数据结构2".to_string())),
            ("type".to_string(), Value::String("document".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("struct-003".to_string())),
            ("name".to_string(), Value::String("数据结构3".to_string())),
            ("type".to_string(), Value::String("form".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn program_center_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/program_center/application/list", get(application_list))
        .route("/jaxrs/program_center/script/list", get(script_list))
        .route("/jaxrs/program_center/invoke/list", get(invoke_list))
        .route("/jaxrs/program_center/agent/list", get(agent_list))
        .route("/jaxrs/program_center/structure/list", get(structure_list))
        .layer(Extension(pool))
}
