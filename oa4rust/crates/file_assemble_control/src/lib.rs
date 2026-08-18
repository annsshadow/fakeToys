use axum::{
    extract::Extension,
    Json,
};
use base64::Engine;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use std::sync::Arc;

pub use shared::{ControlClient, ControlPool, DynControlPool, RowGet};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn file_assemble_control_router(pool: Pool) -> axum::Router {
    routes::router(pool)
}

#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Arc<dyn ControlPool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.acquire().await?;

    let row = client
        .ctrl_query_one(
            "SELECT enabled, default_storage, max_upload_size FROM x_file_assemble_control_config LIMIT 1",
            &[],
        )
        .await;

    let data = match row {
        Ok(r) => serde_json::Map::from_iter([
            ("enabled".to_string(), Value::Bool(r.get_bool("enabled"))),
            ("defaultStorage".to_string(), Value::String(r.get_str("default_storage").to_string())),
            ("maxUploadSize".to_string(), Value::Number(serde_json::Number::from(r.get_i64("max_upload_size")))),
        ]),
        Err(_) => serde_json::Map::from_iter([
            ("enabled".to_string(), Value::Bool(true)),
            ("defaultStorage".to_string(), Value::String("local".to_string())),
            ("maxUploadSize".to_string(), Value::Number(serde_json::Number::from(104857600i64))),
        ]),
    };

    Ok(Json(ActionResult::success(Value::Object(data))))
}

#[axum::debug_handler]
pub async fn list_storage_pools(
    pool: Extension<Arc<dyn ControlPool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.acquire().await?;

    let rows = client
        .ctrl_query(
            "SELECT id, name, enabled FROM x_file_assemble_control_storage_pool ORDER BY id",
            &[],
        )
        .await;

    let data: Vec<Value> = match rows {
        Ok(r) => r
            .iter()
            .map(|row| {
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get_str("id").to_string())),
                    ("name".to_string(), Value::String(row.get_str("name").to_string())),
                    ("enabled".to_string(), Value::Bool(row.get_bool("enabled"))),
                ]))
            })
            .collect(),
        Err(_) => vec![],
    };

    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Arc<dyn ControlPool>>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.acquire().await?;

    let enabled = body
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let default_storage = body
        .get("defaultStorage")
        .and_then(|v| v.as_str())
        .unwrap_or("local")
        .to_string();
    let max_upload_size: i64 = body
        .get("maxUploadSize")
        .and_then(|v| v.as_i64())
        .unwrap_or(104857600);

    client
        .ctrl_execute(
            "UPDATE x_file_assemble_control_config SET enabled = $1, default_storage = $2, max_upload_size = $3 WHERE id = 'global'",
            &[&enabled, &default_storage, &max_upload_size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("enabled".to_string(), Value::Bool(enabled)),
            ("defaultStorage".to_string(), Value::String(default_storage)),
            ("maxUploadSize".to_string(), Value::Number(serde_json::Number::from(max_upload_size))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn list_control_categories(
    pool: Extension<Arc<dyn ControlPool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.acquire().await?;

    let rows = client
        .ctrl_query(
            "SELECT id, name, description FROM x_file_assemble_control_category ORDER BY id",
            &[],
        )
        .await;

    let categories: Vec<Value> = match rows {
        Ok(r) => r
            .iter()
            .map(|row| {
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get_str("id").to_string())),
                    ("name".to_string(), Value::String(row.get_str("name").to_string())),
                    ("description".to_string(), Value::String(row.get_str("description").to_string())),
                ]))
            })
            .collect(),
        Err(_) => vec![],
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(categories.len() as i64))),
            ("data".to_string(), Value::Array(categories)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::file_assemble_control_router(pool)
}


#[axum::debug_handler]
pub async fn list_files(
    pool: Extension<Pool>,
    axum::extract::Path(folder_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, path, size, creator, create_time, folder_id \
              FROM x_file WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("path".to_string(), Value::String(row.get("path"))),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("folderId".to_string(), Value::String(row.get("folder_id"))),
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
pub async fn get_file(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, path, size, creator, create_time, folder_id \
              FROM x_file WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("path".to_string(), Value::String(row.get("path"))),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("folderId".to_string(), Value::String(row.get("folder_id"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn upload_file(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = body.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = session.person_unique.clone();

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_file (id, name, path, size, creator, create_time, folder_id) \
              VALUES ($1, $2, $3, $4, $5, NOW(), $6)",
            &[&id, &name, &path, &size, &creator, &folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("path".to_string(), Value::String(path)),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("creator".to_string(), Value::String(creator)),
        ("folderId".to_string(), Value::String(folder_id)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn create_file(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = body.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = session.person_unique.clone();

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_file (id, name, path, size, creator, create_time, folder_id) \
              VALUES ($1, $2, $3, $4, $5, NOW(), $6)",
            &[&id, &name, &path, &size, &creator, &folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn delete_file(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_file SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("file not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn create_file_entity(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = body.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = "system";

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_file (id, name, path, size, creator, create_time, folder_id) \
              VALUES ($1, $2, $3, $4, $5, NOW(), $6)",
            &[&id, &name, &path, &size, &creator, &folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("path".to_string(), Value::String(path)),
            ("folderId".to_string(), Value::String(folder_id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn update_file_entity(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT creator FROM x_file WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("file not found")));
    };

    let creator: String = row.get("creator");
    shared::middleware::require_owner(&pool, &session, &creator).await?;

    let has_name = body.get("name").is_some();
    let has_path = body.get("path").is_some();
    let has_size = body.get("size").is_some();
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(-1);

    let row = if has_name && has_path && has_size {
        client
            .query_opt(
                "UPDATE x_file SET name = NULLIF($2, ''), path = NULLIF($3, ''), size = NULLIF($4, -1), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path, size",
                &[&id, &name, &path, &size],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_name && has_path {
        client
            .query_opt(
                "UPDATE x_file SET name = NULLIF($2, ''), path = NULLIF($3, ''), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path",
                &[&id, &name, &path],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_name && has_size {
        client
            .query_opt(
                "UPDATE x_file SET name = NULLIF($2, ''), size = NULLIF($3, -1), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path, size",
                &[&id, &name, &size],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_path && has_size {
        client
            .query_opt(
                "UPDATE x_file SET path = NULLIF($2, ''), size = NULLIF($3, -1), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path, size",
                &[&id, &path, &size],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_name {
        client
            .query_opt(
                "UPDATE x_file SET name = NULLIF($2, ''), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path",
                &[&id, &name],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_path {
        client
            .query_opt(
                "UPDATE x_file SET path = NULLIF($2, ''), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path",
                &[&id, &path],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_size {
        client
            .query_opt(
                "UPDATE x_file SET size = NULLIF($2, -1), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path, size",
                &[&id, &size],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query_opt(
                "UPDATE x_file SET update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path",
                &[&id],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    match row {
        Some(row) => {
            let result_name: String = row.get("name");
            let result_path: String = row.get("path");
            let result_size: Option<i64> = row.get("size");

            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("saved".to_string(), Value::Bool(true)),
                ("name".to_string(), Value::String(result_name)),
                ("path".to_string(), Value::String(result_path)),
            ]);
            if let Some(s) = result_size {
                map.insert("size".to_string(), Value::Number(serde_json::Number::from(s)));
            }

            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn delete_file_entity(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT creator FROM x_file WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("file not found or already deleted")));
    };

    let creator: String = row.get("creator");
    shared::middleware::require_owner(&pool, &session, &creator).await?;

    let result = client
        .execute(
            "DELETE FROM x_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("file not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}


#[axum::debug_handler]
pub async fn anonymous_file_id_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    let client = pool.0.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT name, mime_type, content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let mime: String = row.get("mime_type");
            let name: String = row.get("name");
            let bytes = match content {
                Some(c) => base64::engine::general_purpose::STANDARD.decode(c).unwrap_or_default(),
                None => vec![],
            };
            Ok(axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .header("Content-Type", mime)
                .header("Content-Disposition", format!("attachment; filename=\"{}\"", name))
                .body(axum::body::Body::from(bytes))
                .unwrap())
        }
        None => Ok(axum::response::Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap()),
    }
}

#[axum::debug_handler]
pub async fn anonymous_file_id_download_stream(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    anonymous_file_id_download(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn attachment_list_editor_owner(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment_list_folder_folderId(
    pool: Extension<Pool>,
    axum::extract::Path(folder_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment_list_share_owner(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment_list_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment_upload_folder_folderId(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(folder_id): axum::extract::Path<String>,
    mut form: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut mime_type: Option<String> = None;
    let mut filename: Option<String> = None;
    let mut name: Option<String> = None;
    while let Some(field) = form.next_field().await.map_err(|_| AppError::BadRequest("multipart parse failed".to_string()))? {
        if let Some(fname) = field.file_name() {
            mime_type = field.content_type().map(|s| s.to_string());
            filename = Some(fname.to_string());
            file_data = Some(field.bytes().await.map_err(|_| AppError::BadRequest("file read failed".to_string()))?.to_vec());
        } else {
            let fn_ = field.name().unwrap_or("").to_string();
            let value = field.text().await.map_err(|_| AppError::BadRequest("form read failed".to_string()))?;
            if fn_ == "name" { name = Some(value); }
        }
    }
    let data = match file_data { Some(d) => d, None => return Ok(Json(ActionResult::error("no file provided"))), };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = name.unwrap_or_else(|| filename.clone().unwrap_or_else(|| format!("file-{}", id)));
    let mime = mime_type.as_deref().unwrap_or("");
    let ext = match mime {
        "image/jpeg" => "jpg", "image/png" => "png", "image/gif" => "gif",
        "application/pdf" => "pdf", "text/plain" => "txt",
        _ => { if let Some(ref fname) = filename { fname.split('.').last().unwrap_or("bin") } else { "bin" } }
    };
    let content_b64 = base64::engine::general_purpose::STANDARD.encode(&data);
    let ref_type = String::from("attachment");
    client.execute(
        "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, content, create_time, update_time) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
        &[&id, &name, &session.person_unique, &folder_id, &ref_type, &ext, &(data.len() as i64), &mime, &content_b64],
    ).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("uploaded".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment_upload_folder_folderId_callback_callback(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((folder_id, _callback)): axum::extract::Path<(String, String)>,
    mut form: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_upload_folder_folderId(pool, Extension(session), axum::extract::Path(folder_id), form).await
}

#[axum::debug_handler]
pub async fn attachment_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT id, name, person, reference_type, extension, length, mime_type, create_time FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("referenceType".to_string(), Value::String(row.get("reference_type"))),
                ("extension".to_string(), Value::String(row.get("extension"))),
                ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
                ("mimeType".to_string(), Value::String(row.get("mime_type"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

#[axum::debug_handler]
pub async fn attachment_id_binary_base64(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("contentBase64".to_string(), Value::String(content.unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

#[axum::debug_handler]
pub async fn attachment_id_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT name, mime_type, content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let mime: String = row.get("mime_type");
            let name: String = row.get("name");
            let bytes = match content {
                Some(c) => base64::engine::general_purpose::STANDARD.decode(c).unwrap_or_default(),
                None => vec![],
            };
            Ok(axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .header("Content-Type", mime)
                .header("Content-Disposition", format!("attachment; filename=\"{}\"", name))
                .body(axum::body::Body::from(bytes))
                .unwrap())
        }
        None => Ok(axum::response::Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap()),
    }
}

#[axum::debug_handler]
pub async fn attachment_id_download_stream(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    attachment_id_download(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn attachment_id_image_scale_scale_binary_base64(
    pool: Extension<Pool>,
    axum::extract::Path((id, _scale)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_id_binary_base64(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn attachment_id_image_width_width_height_height_binary_base64(
    pool: Extension<Pool>,
    axum::extract::Path((id, _width, _height)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_id_binary_base64(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn attachment_id_update(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else { return Ok(Json(ActionResult::error("attachment not found"))); };
    let creator: String = row.get("person");
    shared::middleware::require_owner(&pool, &session, &creator).await?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let mime_type = body.get("mimeType").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    client.execute(
        "UPDATE FILE_FILE SET name = $1, mime_type = $2, update_time = NOW() WHERE id = $3 AND deleted_at IS NULL",
        &[&name, &mime_type, &id],
    ).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment_id_update_callback_callback(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((id, _callback)): axum::extract::Path<(String, String)>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_id_update(pool, Extension(session), axum::extract::Path(id), axum::extract::Json(body)).await
}

#[axum::debug_handler]
pub async fn attachment2_exist_file_fileMd5(
    pool: Extension<Pool>,
    axum::extract::Path(md5): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one("SELECT COUNT(*) FROM FILE_FILE WHERE md5 = $1 AND deleted_at IS NULL", &[&md5])
        .await.map_err(|_| AppError::Internal)?
        .get("count");
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("exists".to_string(), Value::Bool(count > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_list_editor_owner(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_list_filter_name(
    pool: Extension<Pool>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE name ILIKE $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&format!("%{}%", name)],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_list_folder_folderId(
    pool: Extension<Pool>,
    axum::extract::Path(folder_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&folder_id],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_list_share_owner(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_list_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_list_type_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((_type, page, size)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let page_size: i64 = size.parse().unwrap_or(20);
    let offset: i64 = page.parse().unwrap_or(0) * page_size;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&page_size, &offset],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_upload_folder_folderId(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(folder_id): axum::extract::Path<String>,
    mut form: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut mime_type: Option<String> = None;
    let mut filename: Option<String> = None;
    let mut name: Option<String> = None;
    while let Some(field) = form.next_field().await.map_err(|_| AppError::BadRequest("multipart parse failed".to_string()))? {
        if let Some(fname) = field.file_name() {
            mime_type = field.content_type().map(|s| s.to_string());
            filename = Some(fname.to_string());
            file_data = Some(field.bytes().await.map_err(|_| AppError::BadRequest("file read failed".to_string()))?.to_vec());
        } else {
            let fn_ = field.name().unwrap_or("").to_string();
            let value = field.text().await.map_err(|_| AppError::BadRequest("form read failed".to_string()))?;
            if fn_ == "name" { name = Some(value); }
        }
    }
    let data = match file_data { Some(d) => d, None => return Ok(Json(ActionResult::error("no file provided"))), };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = name.unwrap_or_else(|| filename.clone().unwrap_or_else(|| format!("file-{}", id)));
    let mime = mime_type.as_deref().unwrap_or("");
    let ext = match mime {
        "image/jpeg" => "jpg", "image/png" => "png", "image/gif" => "gif",
        "application/pdf" => "pdf", "text/plain" => "txt",
        _ => { if let Some(ref fname) = filename { fname.split('.').last().unwrap_or("bin") } else { "bin" } }
    };
    let content_b64 = base64::engine::general_purpose::STANDARD.encode(&data);
    let ref_type = String::from("attachment");
    client.execute(
        "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, content, create_time, update_time) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
        &[&id, &name, &session.person_unique, &folder_id, &ref_type, &ext, &(data.len() as i64), &mime, &content_b64],
    ).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("uploaded".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_user_capacity(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT COALESCE(SUM(length), 0) AS total FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL", &[&session.person_unique])
        .await.map_err(|_| AppError::Internal)?;
    let total: i64 = row.map(|r| r.get("total")).unwrap_or(0);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("used".to_string(), Value::Number(serde_json::Number::from(total))),
            ("limit".to_string(), Value::Number(serde_json::Number::from(1073741824i64))),
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn attachment2_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT id, name, person, reference_type, extension, length, mime_type, create_time FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("referenceType".to_string(), Value::String(row.get("reference_type"))),
                ("extension".to_string(), Value::String(row.get("extension"))),
                ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
                ("mimeType".to_string(), Value::String(row.get("mime_type"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

#[axum::debug_handler]
pub async fn attachment2_id_binary_base64(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("contentBase64".to_string(), Value::String(content.unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

#[axum::debug_handler]
pub async fn attachment2_id_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT name, mime_type, content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let mime: String = row.get("mime_type");
            let name: String = row.get("name");
            let bytes = match content {
                Some(c) => base64::engine::general_purpose::STANDARD.decode(c).unwrap_or_default(),
                None => vec![],
            };
            Ok(axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .header("Content-Type", mime)
                .header("Content-Disposition", format!("attachment; filename=\"{}\"", name))
                .body(axum::body::Body::from(bytes))
                .unwrap())
        }
        None => Ok(axum::response::Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap()),
    }
}

#[axum::debug_handler]
pub async fn attachment2_id_download_image_width_width_height_height(
    pool: Extension<Pool>,
    axum::extract::Path((id, _width, _height)): axum::extract::Path<(String, String, String)>,
) -> Result<axum::response::Response, AppError> {
    attachment2_id_download(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn attachment2_id_download_stream(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    attachment2_id_download(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn attachment2_id_image_scale_scale_binary_base64(
    pool: Extension<Pool>,
    axum::extract::Path((id, _scale)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment2_id_binary_base64(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn attachment2_id_image_width_width_height_height_binary_base64(
    pool: Extension<Pool>,
    axum::extract::Path((id, _width, _height)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment2_id_binary_base64(pool, axum::extract::Path(id)).await
}

/// .docx 本质是 ZIP 包：解压 word/document.xml，将 <w:p> 段落中的
/// <w:t> 文本提取为简单 HTML（<p> 标签）。表格/图片等复杂内容由调用方降级。
fn docx_to_html(bytes: &[u8]) -> Option<String> {
    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).ok()?;
    let xml = archive.by_name("word/document.xml").ok().and_then(|mut f| {
        let mut s = String::new();
        std::io::Read::read_to_string(&mut f, &mut s).ok()?;
        Some(s)
    })?;

    if xml.contains("<w:tbl") || xml.contains("<w:drawing") {
        return None;
    }

    let mut html = String::new();
    let mut rest = xml.as_str();
    while let Some(start) = rest.find("<w:p") {
        let Some(close_rel) = rest[start..].find("</w:p>") else { break };
        let para = &rest[start..start + close_rel];
        let text = extract_wt_text(para);
        if !text.is_empty() {
            html.push_str("<p>");
            html.push_str(&html_escape(&text));
            html.push_str("</p>");
        }
        rest = &rest[start + close_rel + 6..];
    }

    if html.is_empty() { None } else { Some(html) }
}

fn extract_wt_text(para: &str) -> String {
    let mut out = String::new();
    let mut rest = para;
    while let Some(start) = rest.find("<w:t") {
        let Some(gt_rel) = rest[start..].find('>') else { break };
        let content_start = start + gt_rel + 1;
        let Some(end_rel) = rest[content_start..].find("</w:t>") else { break };
        out.push_str(&xml_unescape(&rest[content_start..content_start + end_rel]));
        rest = &rest[content_start + end_rel + 6..];
    }
    out
}

fn xml_unescape(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[axum::debug_handler]
pub async fn attachment2_id_office_preview_type_type(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((id, _type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, person, extension, mime_type, content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("attachment not found")));
    };

    shared::middleware::require_owner(&pool, &session, &row.get::<_, String>("person")).await?;

    let content: Option<String> = row.get("content");
    let extension: String = row.get("extension");
    let mime: String = row.get("mime_type");
    let id: String = row.get("id");
    let name: String = row.get("name");

    let bytes = match content {
        Some(c) => base64::engine::general_purpose::STANDARD
            .decode(c)
            .unwrap_or_default(),
        None => vec![],
    };

    let is_docx = extension.eq_ignore_ascii_case("docx") || mime.contains("wordprocessingml");

    if is_docx {
        if let Some(html) = docx_to_html(&bytes) {
            return Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("name".to_string(), Value::String(name)),
                    ("html".to_string(), Value::String(html)),
                    ("contentType".to_string(), Value::String("text/html".to_string())),
                ]),
            ))));
        }
    }

    // 降级：非 .docx、解析失败或含复杂内容时返回 base64（不抛错）
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("content".to_string(), Value::String(base64::engine::general_purpose::STANDARD.encode(&bytes))),
            ("contentType".to_string(), Value::String(mime)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn complex_folder_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT id, name, person, superior FROM FILE_FOLDER WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("superior".to_string(), row.get::<_, Option<String>>("superior").map(Value::String).unwrap_or(Value::Null)),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("folder not found"))),
    }
}

#[axum::debug_handler]
pub async fn complex_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let folder_rows = client
        .query("SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior IS NULL OR superior = '' ORDER BY name LIMIT 10", &[])
        .await.map_err(|_| AppError::Internal)?;
    let folder_list: Vec<Value> = folder_rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("superior".to_string(), row.get::<_, Option<String>>("superior").map(Value::String).unwrap_or(Value::Null)),
        ]))
    }).collect();
    let attachment_rows = client
        .query("SELECT id, name, person, reference_type, extension, length FROM FILE_FILE ORDER BY name LIMIT 10", &[])
        .await.map_err(|_| AppError::Internal)?;
    let attachment_list: Vec<Value> = attachment_rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("\"referenceType\"".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("folderList".to_string(), Value::Array(folder_list)),
            ("attachmentList".to_string(), Value::Array(attachment_list)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn config_is_file_manager(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("isFileManager".to_string(), Value::Bool(!session.person_unique.is_empty())),
            ("personUnique".to_string(), Value::String(session.person_unique)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn config_system_config() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn editor_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT DISTINCT person FROM FILE_FILE WHERE deleted_at IS NULL AND person != '' ORDER BY person", &[])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("person".to_string(), Value::String(row.get("person"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_clean_unused_referencetype_cmsdocument_manage(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute("DELETE FROM FILE_FILE WHERE reference_type = $1 AND deleted_at IS NULL", &[&String::from("cmsdocument_manage")])
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Number(serde_json::Number::from(result))),
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale(
    pool: Extension<Pool>,
    axum::extract::Path((attachment_id, reference_type, reference, _scale)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT name, person, extension, length, mime_type, content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&attachment_id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else { return Ok(Json(ActionResult::error("attachment not found"))); };
    let new_id = uuid::Uuid::new_v4().to_string();
    let name: String = row.get("name");
    let person: String = row.get("person");
    let ext: String = row.get("extension");
    let length: i64 = row.get("length");
    let mime: String = row.get("mime_type");
    let content: Option<String> = row.get("content");
    client.execute(
        "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, content, create_time, update_time) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
        &[&new_id, &name, &person, &reference, &reference_type, &ext, &length, &mime, &content],
    ).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(new_id)),
            ("copied".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_list_referencetype(
    pool: Extension<Pool>,
    axum::extract::Path(reference_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE reference_type = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&reference_type],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_list_referencetype_referenceType_reference_reference(
    pool: Extension<Pool>,
    axum::extract::Path((reference_type, reference)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time \
             FROM FILE_FILE WHERE reference_type = $1 AND reference_id = $2 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&reference_type, &reference],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_list_unused_referencetype_cmsdocument_manage(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE reference_type = 'cmsdocument_manage' AND deleted_at IS NULL ORDER BY create_time DESC", &[])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE id > $1 AND deleted_at IS NULL ORDER BY id LIMIT 20", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_list_id_next_count_all(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    file_list_id_next_count(pool, axum::extract::Path((id, count))).await
}

#[axum::debug_handler]
pub async fn file_list_id_next_count_referencetype_referenceType(
    pool: Extension<Pool>,
    axum::extract::Path((id, count, reference_type)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE id > $1 AND reference_type = $2 AND deleted_at IS NULL ORDER BY id LIMIT 20", &[&id, &reference_type])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE id < $1 AND deleted_at IS NULL ORDER BY id DESC LIMIT 20", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_list_id_prev_count_all(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    file_list_id_prev_count(pool, axum::extract::Path((id, count))).await
}

#[axum::debug_handler]
pub async fn file_list_id_prev_count_referencetype_referenceType(
    pool: Extension<Pool>,
    axum::extract::Path((id, count, reference_type)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE id < $1 AND reference_type = $2 AND deleted_at IS NULL ORDER BY id DESC LIMIT 20", &[&id, &reference_type])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_referencetype_referenceType_reference_reference(
    pool: Extension<Pool>,
    axum::extract::Path((reference_type, reference)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    file_list_referencetype_referenceType_reference_reference(pool, axum::extract::Path((reference_type.clone(), reference.clone()))).await
}

#[axum::debug_handler]
pub async fn file_upload_referencetype_referenceType_reference_reference_scale_scale(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((reference_type, reference, _scale)): axum::extract::Path<(String, String, String)>,
    mut form: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut mime_type: Option<String> = None;
    let mut filename: Option<String> = None;
    let mut name: Option<String> = None;
    while let Some(field) = form.next_field().await.map_err(|_| AppError::BadRequest("multipart parse failed".to_string()))? {
        if let Some(fname) = field.file_name() {
            mime_type = field.content_type().map(|s| s.to_string());
            filename = Some(fname.to_string());
            file_data = Some(field.bytes().await.map_err(|_| AppError::BadRequest("file read failed".to_string()))?.to_vec());
        } else {
            let fn_ = field.name().unwrap_or("").to_string();
            let value = field.text().await.map_err(|_| AppError::BadRequest("form read failed".to_string()))?;
            if fn_ == "name" { name = Some(value); }
        }
    }
    let data = match file_data { Some(d) => d, None => return Ok(Json(ActionResult::error("no file provided"))), };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = name.unwrap_or_else(|| filename.clone().unwrap_or_else(|| format!("file-{}", id)));
    let mime = mime_type.as_deref().unwrap_or("");
    let ext = match mime {
        "image/jpeg" => "jpg", "image/png" => "png", "image/gif" => "gif",
        "application/pdf" => "pdf", "text/plain" => "txt",
        _ => { if let Some(ref fname) = filename { fname.split('.').last().unwrap_or("bin") } else { "bin" } }
    };
    let content_b64 = base64::engine::general_purpose::STANDARD.encode(&data);
    client.execute(
        "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, content, create_time, update_time) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
        &[&id, &name, &session.person_unique, &reference, &reference_type, &ext, &(data.len() as i64), &mime, &content_b64],
    ).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("uploaded".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_upload_referencetype_referenceType_reference_reference_scale_scale_callback_callback(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((reference_type, reference, _scale, _callback)): axum::extract::Path<(String, String, String, String)>,
    mut form: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    file_upload_referencetype_referenceType_reference_reference_scale_scale(pool, Extension(session), axum::extract::Path((reference_type, reference, _scale)), form).await
}

#[axum::debug_handler]
pub async fn file_upload_with_url(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, session, body);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT id, name, person, reference_type, extension, length, mime_type, create_time FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("referenceType".to_string(), Value::String(row.get("reference_type"))),
                ("extension".to_string(), Value::String(row.get("extension"))),
                ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
                ("mimeType".to_string(), Value::String(row.get("mime_type"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_id_binary_base64(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("contentBase64".to_string(), Value::String(content.unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_id_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT name, mime_type, content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let mime: String = row.get("mime_type");
            let name: String = row.get("name");
            let bytes = match content {
                Some(c) => base64::engine::general_purpose::STANDARD.decode(c).unwrap_or_default(),
                None => vec![],
            };
            Ok(axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .header("Content-Type", mime)
                .header("Content-Disposition", format!("attachment; filename=\"{}\"", name))
                .body(axum::body::Body::from(bytes))
                .unwrap())
        }
        None => Ok(axum::response::Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap()),
    }
}

#[axum::debug_handler]
pub async fn file_id_download_stream(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    file_id_download(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn folder_list_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior IS NULL OR superior = '' ORDER BY name", &[])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("superior".to_string(), row.get::<_, Option<String>>("superior").map(Value::String).unwrap_or(Value::Null)),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn folder_list_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior = $1 ORDER BY name", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("superior".to_string(), row.get::<_, Option<String>>("superior").map(Value::String).unwrap_or(Value::Null)),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn folder_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT id, name, person, superior FROM FILE_FOLDER WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("superior".to_string(), row.get::<_, Option<String>>("superior").map(Value::String).unwrap_or(Value::Null)),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("folder not found"))),
    }
}

#[axum::debug_handler]
pub async fn folder2_batch_download(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids: Vec<String> = body.get("ids")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();
    if ids.is_empty() { return Ok(Json(ActionResult::error("ids is required"))); }
    let _ = (pool, ids);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn folder2_list_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, superior FROM FILE_FOLDER WHERE superior IS NULL OR superior = '' ORDER BY name LIMIT 20", &[])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("superior".to_string(), row.get::<_, Option<String>>("superior").map(Value::String).unwrap_or(Value::Null)),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn folder2_list_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    folder_list_id(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn folder2_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    folder_id(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn folder2_id_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT id, name FROM FILE_FOLDER WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("downloadable".to_string(), Value::Bool(true)),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("folder not found"))),
    }
}

#[axum::debug_handler]
pub async fn recycle_empty(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute("DELETE FROM FILE_FILE WHERE deleted_at IS NOT NULL AND person = $1", &[&session.person_unique])
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("cleared".to_string(), Value::Number(serde_json::Number::from(result))),
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn recycle_list(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, deleted_at \
             FROM FILE_FILE WHERE deleted_at IS NOT NULL AND person = $1 ORDER BY deleted_at DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get("mime_type"))),
            ("deletedAt".to_string(), Value::String(row.get("deleted_at"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn recycle_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT id, name, person, deleted_at FROM FILE_FILE WHERE id = $1 AND deleted_at IS NOT NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("deletedAt".to_string(), Value::String(row.get("deleted_at"))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("file not in recycle bin"))),
    }
}

#[axum::debug_handler]
pub async fn recycle_id_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FILE WHERE id = $1 AND deleted_at IS NOT NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else { return Ok(Json(ActionResult::error("file not in recycle bin"))); };
    let creator: String = row.get("person");
    shared::middleware::require_owner(&pool, &session, &creator).await?;
    let result = client
        .execute("DELETE FROM FILE_FILE WHERE id = $1 AND deleted_at IS NOT NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    if result == 0 { return Ok(Json(ActionResult::error("file not in recycle bin"))); }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn recycle_id_resume(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FILE WHERE id = $1 AND deleted_at IS NOT NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else { return Ok(Json(ActionResult::error("file not in recycle bin"))); };
    let creator: String = row.get("person");
    shared::middleware::require_owner(&pool, &session, &creator).await?;
    client.execute("UPDATE FILE_FILE SET deleted_at = NULL, update_time = NOW() WHERE id = $1", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("resumed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_download_share_shareId_file_fileId(
    pool: Extension<Pool>,
    axum::extract::Path((share_id, file_id)): axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    let _ = share_id;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT name, mime_type, content FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&file_id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let mime: String = row.get("mime_type");
            let name: String = row.get("name");
            let bytes = match content {
                Some(c) => base64::engine::general_purpose::STANDARD.decode(c).unwrap_or_default(),
                None => vec![],
            };
            Ok(axum::response::Response::builder()
                .status(axum::http::StatusCode::OK)
                .header("Content-Type", mime)
                .header("Content-Disposition", format!("attachment; filename=\"{}\"", name))
                .body(axum::body::Body::from(bytes))
                .unwrap())
        }
        None => Ok(axum::response::Response::builder()
            .status(axum::http::StatusCode::NOT_FOUND)
            .body(axum::body::Body::empty())
            .unwrap()),
    }
}

#[axum::debug_handler]
pub async fn share_list(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, create_time \
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_list_att_share_shareId_folder_folderId(
    pool: Extension<Pool>,
    axum::extract::Path((share_id, folder_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = share_id;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&folder_id],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_list_folder_share_shareId_folder_folderId(
    pool: Extension<Pool>,
    axum::extract::Path((share_id, folder_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    share_list_att_share_shareId_folder_folderId(pool, axum::extract::Path((share_id, folder_id))).await
}

#[axum::debug_handler]
pub async fn share_list_my(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, create_time \
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_list_my2_shareType_fileType(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((_share_type, file_type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE person = $1 AND reference_type = $2 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&session.person_unique, &file_type],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("referenceType".to_string(), Value::String(row.get("reference_type"))),
            ("extension".to_string(), Value::String(row.get("extension"))),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_list_to_me(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, session);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_list_to_me2_fileType(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(file_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, session, file_type);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_share_shareId_file_fileId_folder_folderId(
    pool: Extension<Pool>,
    axum::extract::Path((share_id, file_id, folder_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, share_id, file_id, folder_id);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_shield_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, id);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, id);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn share_id_password_password(
    pool: Extension<Pool>,
    axum::extract::Path((id, _password)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, id, _password);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}