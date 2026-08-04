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
pub struct ApplicationDict {
    pub id: String,
    pub name: String,
    pub application: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ApplicationDictItem {
    pub id: String,
    pub dict_id: String,
    pub name: String,
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GeneralFile {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub size: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Invoice {
    pub id: String,
    pub number: String,
    pub date: String,
    pub amount: f64,
    pub status: String,
}

pub async fn dict_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, application FROM x_general_application_dict ORDER BY name LIMIT 20",
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

pub async fn dict_item_list(
    pool: Extension<Pool>,
    axum::extract::Path(dict_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, dict_id, name, value FROM x_general_application_dict_item WHERE dict_id = $1",
            &[&dict_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("dictId".to_string(), Value::String(row.get("dict_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("value".to_string(), Value::String(row.get("value"))),
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

pub async fn file_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, mime_type, size FROM x_general_file ORDER BY create_time DESC LIMIT 20",
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
                ("mimeType".to_string(), Value::String(row.get("mime_type"))),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
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

pub async fn invoice_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, number, date, amount, status FROM x_general_invoice ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("number".to_string(), Value::String(row.get("number"))),
                ("date".to_string(), Value::String(row.get("date"))),
                ("amount".to_string(), Value::Number(serde_json::Number::from_f64(row.get::<_, f64>("amount")).unwrap_or_else(|| serde_json::Number::from(0)))),
                ("status".to_string(), Value::String(row.get("status"))),
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

pub fn general_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/general/dict/list", get(dict_list))
        .route("/jaxrs/general/dict/item/list/{dictId}", get(dict_item_list))
        .route("/jaxrs/general/file/list", get(file_list))
        .route("/jaxrs/general/invoice/list", get(invoice_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
