use axum::{
    extract::{Extension, Json},
    routing::{get, post},
    Router,
};
use axum::extract::Path;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct QueryView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "querySql")]
    pub query_sql: Option<String>,
    #[serde(rename = "creatorId")]
    pub creator_id: String,
    pub status: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct QueryItem {
    pub id: String,
    #[serde(rename = "viewId")]
    pub view_id: String,
    pub name: String,
    #[serde(rename = "fieldName")]
    pub field_name: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct QueryImport {
    pub id: String,
    #[serde(rename = "viewId")]
    pub view_id: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    pub status: String,
    #[serde(rename = "importTime")]
    pub import_time: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: String,
}

pub async fn view_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, query_sql, creator_id, status, create_time FROM QUERY_VIEW WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
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
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "querySql".to_string(),
                    row.get::<_, Option<String>>("query_sql")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creatorId".to_string(), Value::String(row.get("creator_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
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

pub async fn view_get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, description, query_sql, creator_id, status, create_time FROM QUERY_VIEW WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "querySql".to_string(),
                    row.get::<_, Option<String>>("query_sql")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creatorId".to_string(), Value::String(row.get("creator_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

#[axum::debug_handler]
pub async fn view_create(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let query_sql = payload.get("querySql").and_then(|v| v.as_str()).map(|s| s.to_string());
    let creator_id = payload.get("creatorId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or_else(|| "active").to_string();

    client
        .execute(
            "INSERT INTO QUERY_VIEW (id, name, description, query_sql, creator_id, status, create_time) VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &name, &description, &query_sql, &creator_id, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("creatorId".to_string(), Value::String(creator_id)),
    ])))))
}

pub async fn item_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, view_id, name, field_name, data_type, create_time FROM QUERY_ITEM WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("viewId".to_string(), Value::String(row.get("view_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("fieldName".to_string(), Value::String(row.get("field_name"))),
                ("dataType".to_string(), Value::String(row.get("data_type"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
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

pub async fn import_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, view_id, file_name, status, import_time, create_time FROM QUERY_IMPORT WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("viewId".to_string(), Value::String(row.get("view_id"))),
                ("fileName".to_string(), Value::String(row.get("file_name"))),
                ("status".to_string(), Value::String(row.get("status"))),
                (
                    "importTime".to_string(),
                    row.get::<_, Option<String>>("import_time")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
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

pub fn query_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/query/item/list", get(item_list))
        .route("/jaxrs/query/view/list", get(view_list))
        .route("/jaxrs/query/view/{id}", get(view_get))
        .route("/jaxrs/query/view/create", post(view_create))
        .route("/jaxrs/query/import/list", get(import_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::query_core_entity_router(pool)
}
