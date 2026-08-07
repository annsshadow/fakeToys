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
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, alias, category, validated, creator_person FROM CTE_INVOKE WHERE deleted_at IS NULL ORDER BY name",
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
                ("application".to_string(), Value::String(row.get("id"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("category".to_string(), Value::String(row.get("category"))),
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

pub async fn agent_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, alias, description, validated, enable, cron FROM CTE_AGENT WHERE deleted_at IS NULL ORDER BY name",
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
                ("type".to_string(), Value::String("agent".to_string())),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("validated".to_string(), Value::Bool(row.get("validated"))),
                ("enable".to_string(), Value::Bool(row.get("enable"))),
                ("cron".to_string(), Value::String(row.get("cron"))),
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

pub async fn structure_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, extension, storage, length, description FROM CTE_STRUCTURE WHERE deleted_at IS NULL ORDER BY name",
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
                ("type".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
                ("storage".to_string(), Value::String(row.get("storage"))),
                ("length".to_string(), row.get::<_, Option<i64>>("length").map(|v| Value::Number(serde_json::Number::from(v))).unwrap_or(Value::Null)),
                ("description".to_string(), Value::String(row.get("description"))),
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

pub fn program_center_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/program_center/application/list", get(application_list))
        .route("/jaxrs/program_center/script/list", get(script_list))
        .route("/jaxrs/program_center/invoke/list", get(invoke_list))
        .route("/jaxrs/program_center/agent/list", get(agent_list))
        .route("/jaxrs/program_center/structure/list", get(structure_list))
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::program_center_core_entity_router(pool)
}
