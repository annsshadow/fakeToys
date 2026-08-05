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
pub struct Definition {
    pub id: String,
    pub name: String,
    pub category: String,
    pub type_: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub level: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Identity {
    pub id: String,
    pub person_id: String,
    pub name: String,
    pub type_: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Custom {
    pub id: String,
    pub identity_id: String,
    pub field_name: String,
    pub field_value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Bind {
    pub id: String,
    pub identity_id: String,
    pub group_id: String,
    pub role: Option<String>,
}

pub async fn definition_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, category, type FROM x_org_definition ORDER BY name LIMIT 20",
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
                ("type".to_string(), Value::String(row.get("type"))),
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

pub async fn group_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, level FROM x_org_group ORDER BY level, name LIMIT 20",
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
                (
                    "parentId".to_string(),
                    row.get::<_, Option<String>>("parent_id")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("level".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("level")))),
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

pub async fn identity_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, person_id, name, type FROM x_org_identity ORDER BY name LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
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

pub async fn person_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, mobile, email FROM x_org_person ORDER BY name LIMIT 20",
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
                (
                    "mobile".to_string(),
                    row.get::<_, Option<String>>("mobile")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "email".to_string(),
                    row.get::<_, Option<String>>("email")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
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

pub async fn custom_list(
    pool: Extension<Pool>,
    axum::extract::Path(identity_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, identity_id, field_name, field_value FROM x_org_custom WHERE identity_id = $1",
            &[&identity_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("fieldName".to_string(), Value::String(row.get("field_name"))),
                ("fieldValue".to_string(), Value::String(row.get("field_value"))),
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

pub async fn bind_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, identity_id, group_id, role FROM x_org_bind ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("groupId".to_string(), Value::String(row.get("group_id"))),
                (
                    "role".to_string(),
                    row.get::<_, Option<String>>("role")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
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

pub fn organization_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/organization/definition/list", get(definition_list))
        .route("/jaxrs/organization/group/list", get(group_list))
        .route("/jaxrs/organization/identity/list", get(identity_list))
        .route("/jaxrs/organization/person/list", get(person_list))
        .route("/jaxrs/organization/custom/list/{identityId}", get(custom_list))
        .route("/jaxrs/organization/bind/list", get(bind_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/organization_core_entity/health", axum::routing::get(|| async { "TODO: organization_core_entity - real implementation needed" }))
}