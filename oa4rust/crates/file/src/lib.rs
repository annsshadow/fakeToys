use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub fn file_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/file/folder/list/top", get(folder_list_top))
        .route("/jaxrs/file/folder/list/{id}", get(folder_list_with_folder))
        .route("/jaxrs/file/complex/top", get(complex_top))
        .layer(Extension(pool))
}

#[derive(Debug, Serialize)]
struct ComplexTopResponse {
    folder_list: Vec<Value>,
    attachment_list: Vec<Value>,
}

#[axum::debug_handler]
pub async fn folder_list_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior IS NULL OR superior = '' ORDER BY name",
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
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
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

#[axum::debug_handler]
pub async fn folder_list_with_folder(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior = $1 ORDER BY name",
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
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
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

#[axum::debug_handler]
pub async fn complex_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let folder_rows = client
        .query(
            "SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior IS NULL OR superior = '' ORDER BY name LIMIT 10",
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
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
            ]))
        })
        .collect();

    let attachment_rows = client
        .query(
            "SELECT id, name, person, referenceType, extension, length FROM FILE_FILE ORDER BY name LIMIT 10",
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
                (
                    "referenceType".to_string(),
                    Value::String(row.get::<_, String>("referenceType")),
                ),
                (
                    "extension".to_string(),
                    Value::String(row.get::<_, String>("extension")),
                ),
                (
                    "length".to_string(),
                    Value::Number(serde_json::Number::from(
                        row.get::<_, Option<i64>>("length").unwrap_or(0),
                    )),
                ),
            ]))
        })
        .collect();

    let response = ComplexTopResponse {
        folder_list,
        attachment_list,
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::to_value(&response)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_result_success_serialization() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 2, "data": []}));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_file_router_builds() {
        let pool = Pool::builder(deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap();

        let _ = file_router(pool);
    }

    #[test]
    fn test_folder_list_top_returns_error_without_db() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = Pool::builder(deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ))
            .build()
            .unwrap();

            let result: Result<Json<ActionResult<Value>>, AppError> =
                folder_list_top(Extension(pool)).await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }

    #[test]
    fn test_complex_top_returns_error_without_db() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = Pool::builder(deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ))
            .build()
            .unwrap();

            let result: Result<Json<ActionResult<Value>>, AppError> =
                complex_top(Extension(pool)).await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }
}
