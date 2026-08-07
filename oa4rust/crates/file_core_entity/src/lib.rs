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
pub struct FolderInfo {
    pub id: String,
    pub name: String,
    pub person: String,
    pub superior: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub id: String,
    pub name: String,
    pub person: String,
    pub reference_type: String,
    pub extension: String,
    pub length: i64,
}

pub async fn folder_list_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior IS NULL OR superior = '' ORDER BY name LIMIT 50",
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
                ("person".to_string(), Value::String(row.get("person"))),
                (
                    "superior".to_string(),
                    row.get::<_, Option<String>>("superior")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("attachmentCount".to_string(), Value::Number(serde_json::Number::from(0))),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                ("folderCount".to_string(), Value::Number(serde_json::Number::from(0))),
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

pub async fn folder_list_with_folder(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior = $1 ORDER BY name LIMIT 50",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                (
                    "superior".to_string(),
                    row.get::<_, Option<String>>("superior")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("attachmentCount".to_string(), Value::Number(serde_json::Number::from(0))),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                ("folderCount".to_string(), Value::Number(serde_json::Number::from(0))),
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
            "SELECT id, name, person, reference_type, extension, length FROM FILE_FILE ORDER BY name LIMIT 50",
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
                ("person".to_string(), Value::String(row.get("person"))),
                ("referenceType".to_string(), Value::String(row.get("reference_type"))),
                ("extension".to_string(), Value::String(row.get("extension"))),
                (
                    "length".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i64>("length"))),
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

pub async fn complex_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let folder_rows = client
        .query(
            "SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior IS NULL OR superior = '' ORDER BY name LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let folder_list: Vec<Value> = folder_rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                (
                    "superior".to_string(),
                    row.get::<_, Option<String>>("superior")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("attachmentCount".to_string(), Value::Number(serde_json::Number::from(0))),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                ("folderCount".to_string(), Value::Number(serde_json::Number::from(0))),
            ]))
        })
        .collect();

    let attachment_rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length FROM FILE_FILE ORDER BY name LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let attachment_list: Vec<Value> = attachment_rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("referenceType".to_string(), Value::String(row.get("reference_type"))),
                ("extension".to_string(), Value::String(row.get("extension"))),
                (
                    "length".to_string(),
                    Value::Number(serde_json::Number::from(
                        row.get::<_, i64>("length"),
                    )),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("folderList".to_string(), Value::Array(folder_list)),
            ("attachmentList".to_string(), Value::Array(attachment_list)),
        ]),
    ))))
}

pub fn file_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/file/core/entity/folder/list/top", get(folder_list_top))
        .route("/jaxrs/file/core/entity/folder/list/{id}", get(folder_list_with_folder))
        .route("/jaxrs/file/core/entity/file/list", get(file_list))
        .route("/jaxrs/file/core/entity/complex/top", get(complex_top))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::file_core_entity_router(pool)
}
