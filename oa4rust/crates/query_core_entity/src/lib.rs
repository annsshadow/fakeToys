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
pub struct Item {
    pub id: String,
    pub name: String,
    pub application: String,
    #[serde(rename = "itemAccess")]
    pub item_access: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ItemAccess {
    pub id: String,
    pub item_id: String,
    pub person: Option<String>,
    pub group: Option<String>,
    pub activity: Option<String>,
    pub property: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct View {
    pub id: String,
    pub name: String,
    pub item_id: String,
    #[serde(rename = "viewType")]
    pub view_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ImportModel {
    pub id: String,
    pub name: String,
    pub application: String,
    #[serde(rename = "importType")]
    pub import_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ImportRecord {
    pub id: String,
    pub model_id: String,
    pub name: String,
    pub status: String,
    pub count: i64,
}

pub async fn item_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, application FROM x_query_item ORDER BY create_time DESC LIMIT 20",
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
                ("application".to_string(), Value::String(row.get("application"))),
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

pub async fn item_access_list(
    pool: Extension<Pool>,
    axum::extract::Path(item_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, item_id, person, group, activity, property FROM x_query_item_access WHERE item_id = $1",
            &[&item_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("itemId".to_string(), Value::String(row.get("item_id"))),
                (
                    "person".to_string(),
                    row.get::<_, Option<String>>("person")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "group".to_string(),
                    row.get::<_, Option<String>>("group")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "activity".to_string(),
                    row.get::<_, Option<String>>("activity")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "property".to_string(),
                    row.get::<_, Option<String>>("property")
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

pub async fn view_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, item_id, view_type FROM x_query_view ORDER BY create_time DESC LIMIT 20",
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
                ("itemId".to_string(), Value::String(row.get("item_id"))),
                ("viewType".to_string(), Value::String(row.get("view_type"))),
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

pub async fn import_model_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, application, import_type FROM x_query_import_model ORDER BY create_time DESC LIMIT 20",
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
                ("application".to_string(), Value::String(row.get("application"))),
                ("importType".to_string(), Value::String(row.get("import_type"))),
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

pub async fn import_record_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, model_id, name, status, count FROM x_query_import_record ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelId".to_string(), Value::String(row.get("model_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("count".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("count")))),
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
        .route("/jaxrs/query/item/access/list/{itemId}", get(item_access_list))
        .route("/jaxrs/query/view/list", get(view_list))
        .route("/jaxrs/query/import/model/list", get(import_model_list))
        .route("/jaxrs/query/import/record/list", get(import_record_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
