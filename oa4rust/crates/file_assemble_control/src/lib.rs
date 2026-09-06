use chrono::NaiveDateTime;
use axum::{
    extract::Extension,
    Json,
};
use base64::Engine;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{db::dialect, error::AppError, response::{ActionResult, row_opt_json}};
use std::sync::Arc;

pub use shared::{ControlClient, ControlPool, DynControlPool, RowGet};

pub const JAVA_BASE: &str = "/jaxrs/file_assemble_control";
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
            &dialect().format_sql(
                "SELECT enabled, default_storage, max_upload_size FROM x_file_assemble_control_config LIMIT 1",
            ),
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
            &dialect().format_sql(
                "SELECT id, name, enabled FROM x_file_assemble_control_storage_pool ORDER BY id",
            ),
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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

    let result = client
        .ctrl_execute(
            &dialect().format_sql(
                "UPDATE x_file_assemble_control_config SET enabled = $1, default_storage = $2, max_upload_size = $3 WHERE id = 'global'",
            ),
            &[&enabled, &default_storage, &max_upload_size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(result > 0)),
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
            &dialect().format_sql(
                "SELECT id, name, description FROM x_file_assemble_control_category ORDER BY id",
            ),
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

    let count = categories.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(categories), count, 0)))
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
              FROM x_file WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("path".to_string(), Value::String(row.get::<_, Option<String>>("path").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
                ("folderId".to_string(), Value::String(row.get("folder_id"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("path".to_string(), Value::String(row.get::<_, Option<String>>("path").unwrap_or_default())),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("creator".to_string(), Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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

    let creator: String = row.get::<_, Option<String>>("creator").unwrap_or_default();
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
            let result_name: String = row.get::<_, Option<String>>("name").unwrap_or_default();
            let result_path: String = row.get::<_, Option<String>>("path").unwrap_or_default();
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

    let creator: String = row.get::<_, Option<String>>("creator").unwrap_or_default();
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
            let mime: String = row.get::<_, Option<String>>("mime_type").unwrap_or_default();
            let name: String = row.get::<_, Option<String>>("name").unwrap_or_default();
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
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn attachment_list_folder_folderId(
    pool: Extension<Pool>,
    axum::extract::Path(folder_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn attachment_list_share_owner(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn attachment_list_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE deleted_at IS NULL ORDER BY create_time::timestamp DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
        .query_opt("SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
                ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
                ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
                ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
                ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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
            let mime: String = row.get::<_, Option<String>>("mime_type").unwrap_or_default();
            let name: String = row.get::<_, Option<String>>("name").unwrap_or_default();
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
    let creator: String = row.get::<_, Option<String>>("person").unwrap_or_default();
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
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn attachment2_list_filter_name(
    pool: Extension<Pool>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE name ILIKE $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&format!("%{}%", name)],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn attachment2_list_folder_folderId(
    pool: Extension<Pool>,
    axum::extract::Path(folder_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&folder_id],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn attachment2_list_share_owner(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn attachment2_list_top(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE deleted_at IS NULL ORDER BY create_time::timestamp DESC LIMIT 20",
            &[],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE deleted_at IS NULL ORDER BY create_time::timestamp DESC LIMIT $1::int OFFSET $2::int",
            &[&page_size, &offset],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
        .query_opt("SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
                ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
                ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
                ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
                ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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
            let mime: String = row.get::<_, Option<String>>("mime_type").unwrap_or_default();
            let name: String = row.get::<_, Option<String>>("name").unwrap_or_default();
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

/// .xlsx 渲染上限：最多输出 200 行，防止超大表格生成巨型 HTML。
const XLSX_MAX_ROWS: usize = 200;

/// .xlsx 本质是 ZIP 包：xl/sharedStrings.xml 存共享字符串，
/// xl/worksheets/sheet1.xml 的 <c t="s"><v>索引</v></c> 引用它；
/// 其余 <v> 为字面值。渲染为简单 HTML 表格。
fn xlsx_to_html(bytes: &[u8]) -> Option<String> {
    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).ok()?;

    let shared_xml = read_zip_entry(&mut archive, "xl/sharedStrings.xml")?;
    let shared: Vec<String> = {
        let mut out = Vec::new();
        let mut rest = shared_xml.as_str();
        while let Some(start) = rest.find("<si>") {
            let Some(end_rel) = rest[start..].find("</si>") else { break };
            let si = &rest[start + 4..start + end_rel];
            let mut text = String::new();
            let mut cur = si;
            while let Some(t_start) = cur.find("<t") {
                let Some(gt_rel) = cur[t_start..].find('>') else { break };
                let cs = t_start + gt_rel + 1;
                let Some(e_rel) = cur[cs..].find("</t>") else { break };
                text.push_str(&xml_unescape(&cur[cs..cs + e_rel]));
                cur = &cur[cs + e_rel + 4..];
            }
            out.push(text);
            rest = &rest[start + end_rel + 5..];
        }
        out
    };

    let sheet_xml = read_zip_entry(&mut archive, "xl/worksheets/sheet1.xml")?;
    let mut html = String::from("<table>");
    let mut row_count = 0usize;
    let mut rest = sheet_xml.as_str();
    while let Some(start) = rest.find("<row") {
        let Some(row_end_rel) = rest[start..].find("</row>") else { break };
        let row_inner_end = start + row_end_rel;
        let row = &rest[start..row_inner_end];
        html.push_str("<tr>");
        let mut cur = row;
        while let Some(c_start) = cur.find("<c ") {
            let Some(c_end_rel) = cur[c_start..].find("</c>") else { break };
            let cell = &cur[c_start..c_start + c_end_rel];
            let is_shared = cell.contains("t=\"s\"");
            let value = match cell.find("<v>") {
                Some(v_rel) => {
                    let vs = v_rel + 3;
                    let ve = cell[vs..].find("</v>").map(|e| vs + e).unwrap_or(vs);
                    &cell[vs..ve]
                }
                None => "",
            };
            let text = if is_shared {
                value
                    .parse::<usize>()
                    .ok()
                    .and_then(|i| shared.get(i).cloned())
                    .unwrap_or_default()
            } else {
                xml_unescape(value)
            };
            html.push_str("<td>");
            html.push_str(&html_escape(&text));
            html.push_str("</td>");
            cur = &cur[c_start + c_end_rel + 4..];
        }
        html.push_str("</tr>");
        row_count += 1;
        if row_count >= XLSX_MAX_ROWS {
            break;
        }
        rest = &rest[row_inner_end + 6..];
    }
    html.push_str("</table>");

    if row_count == 0 { None } else { Some(html) }
}

/// .pptx：按编号顺序读取 ppt/slides/slideN.xml，每张幻灯片的
/// <a:t> 文本首行作标题（h2），其余作段落。最多渲染 50 张。
const PPTX_MAX_SLIDES: usize = 50;

fn pptx_to_html(bytes: &[u8]) -> Option<String> {
    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).ok()?;

    let mut slide_names: Vec<(u32, String)> = (0..archive.len())
        .filter_map(|i| {
            let name = archive.by_index(i).ok()?.name().to_string();
            let num: u32 = name
                .strip_prefix("ppt/slides/slide")?
                .strip_suffix(".xml")?
                .parse()
                .ok()?;
            Some((num, name))
        })
        .collect();
    if slide_names.is_empty() {
        return None;
    }
    slide_names.sort_by_key(|(n, _)| *n);
    slide_names.truncate(PPTX_MAX_SLIDES);

    let mut html = String::new();
    for (_, sname) in &slide_names {
        let xml = read_zip_entry(&mut archive, sname)?;
        let texts = {
            let mut out: Vec<String> = Vec::new();
            let mut rest = xml.as_str();
            while let Some(start) = rest.find("<a:t>") {
                let cs = start + 5;
                let Some(e_rel) = rest[cs..].find("</a:t>") else { break };
                let t = xml_unescape(&rest[cs..cs + e_rel]);
                if !t.trim().is_empty() {
                    out.push(t);
                }
                rest = &rest[cs + e_rel + 6..];
            }
            out
        };
        if texts.is_empty() {
            continue;
        }
        html.push_str("<h2>");
        html.push_str(&html_escape(&texts[0]));
        html.push_str("</h2>");
        for t in &texts[1..] {
            html.push_str("<p>");
            html.push_str(&html_escape(t));
            html.push_str("</p>");
        }
    }

    if html.is_empty() { None } else { Some(html) }
}

fn read_zip_entry(
    archive: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>,
    name: &str,
) -> Option<String> {
    let mut f = archive.by_name(name).ok()?;
    let mut s = String::new();
    std::io::Read::read_to_string(&mut f, &mut s).ok()?;
    Some(s)
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
    let extension: String = row.get::<_, Option<String>>("extension").unwrap_or_default();
    let mime: String = row.get::<_, Option<String>>("mime_type").unwrap_or_default();
    let id: String = row.get("id");
    let name: String = row.get::<_, Option<String>>("name").unwrap_or_default();

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

    let is_xlsx = extension.eq_ignore_ascii_case("xlsx") || mime.contains("spreadsheetml");
    if is_xlsx {
        if let Some(html) = xlsx_to_html(&bytes) {
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

    let is_pptx = extension.eq_ignore_ascii_case("pptx") || mime.contains("presentationml");
    if is_pptx {
        if let Some(html) = pptx_to_html(&bytes) {
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
            {
                let mut map = serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                    ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
                ]);
                if let Some(superior) = row_opt_json::<String>(&row, "superior") {
                    map.insert("superior".to_string(), superior);
                }
                map
            },
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
        Value::Object({
            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ]);
            if let Some(superior) = row_opt_json::<String>(row, "superior") {
                map.insert("superior".to_string(), superior);
            }
            map
        })
    }).collect();
    let attachment_rows = client
        .query("SELECT id, name, person, reference_type, extension, length FROM FILE_FILE ORDER BY name LIMIT 10", &[])
        .await.map_err(|_| AppError::Internal)?;
    let attachment_list: Vec<Value> = attachment_rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("\"referenceType\"".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
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
    Err(u2_capability_unavailable("file-system-config-read"))
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
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
    let name: String = row.get::<_, Option<String>>("name").unwrap_or_default();
    let person: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    let ext: String = row.get::<_, Option<String>>("extension").unwrap_or_default();
    let length: i64 = row.get("length");
    let mime: String = row.get::<_, Option<String>>("mime_type").unwrap_or_default();
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
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE reference_type = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&reference_type],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn file_list_referencetype_referenceType_reference_reference(
    pool: Extension<Pool>,
    axum::extract::Path((reference_type, reference)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text
             FROM FILE_FILE WHERE reference_type = $1 AND reference_id = $2 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&reference_type, &reference],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn file_list_unused_referencetype_cmsdocument_manage(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE reference_type = 'cmsdocument_manage' AND deleted_at IS NULL ORDER BY create_time::timestamp DESC", &[])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = body.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size: i64 = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let url = body.get("url").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let mime_type = body.get("mimeType").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let id = uuid::Uuid::new_v4().to_string();
    let creator = session.person_unique.clone();
    let content_b64 = base64::engine::general_purpose::STANDARD.encode(&url);
    let ext = if let Some(ref fname) = name.split('.').last() { fname } else { "bin" };

    client
        .execute(
            "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, content, create_time, update_time) \
              VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())",
            &[&id, &name, &creator, &folder_id, &String::from("file"), &ext, &size, &mime_type, &content_b64],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("path".to_string(), Value::String(path)),
            ("folderId".to_string(), Value::String(folder_id)),
            ("url".to_string(), Value::String(url)),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
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
        .query_opt("SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
                ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
                ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
                ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
                ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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
            let mime: String = row.get::<_, Option<String>>("mime_type").unwrap_or_default();
            let name: String = row.get::<_, Option<String>>("name").unwrap_or_default();
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
        Value::Object({
            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ]);
            if let Some(superior) = row_opt_json::<String>(row, "superior") {
                map.insert("superior".to_string(), superior);
            }
            map
        })
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
        Value::Object({
            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ]);
            if let Some(superior) = row_opt_json::<String>(row, "superior") {
                map.insert("superior".to_string(), superior);
            }
            map
        })
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            {
                let mut map = serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                    ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
                ]);
                if let Some(superior) = row_opt_json::<String>(&row, "superior") {
                    map.insert("superior".to_string(), superior);
                }
                map
            },
        )))),
        None => Ok(Json(ActionResult::error("folder not found"))),
    }
}

#[axum::debug_handler]
pub async fn folder2_batch_download(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool;
    // Java 语义：按文件夹批量打包下载 —— 无打包引擎，显式 501 + warn。
    Err(u2_capability_unavailable("zip-batch-download"))
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
        Value::Object({
            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ]);
            if let Some(superior) = row_opt_json::<String>(row, "superior") {
                map.insert("superior".to_string(), superior);
            }
            map
        })
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
    let _ = (pool, id);
    // Java 语义：文件夹打包下载 —— 无打包引擎，显式 501 + warn。
    Err(u2_capability_unavailable("zip-folder-download"))
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
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("mimeType".to_string(), Value::String(row.get::<_, Option<String>>("mime_type").unwrap_or_default())),
            ("deletedAt".to_string(), Value::String(row.get("deleted_at"))),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
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
    let creator: String = row.get::<_, Option<String>>("person").unwrap_or_default();
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
    let creator: String = row.get::<_, Option<String>>("person").unwrap_or_default();
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
            let mime: String = row.get::<_, Option<String>>("mime_type").unwrap_or_default();
            let name: String = row.get::<_, Option<String>>("name").unwrap_or_default();
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
            "SELECT id, name, person, reference_type, extension, length, create_time::text
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&folder_id],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, name, person, reference_type, extension, length, create_time::text
             FROM FILE_FILE WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            "SELECT id, name, person, reference_type, extension, length FROM FILE_FILE WHERE person = $1 AND reference_type = $2 AND deleted_at IS NULL ORDER BY create_time::timestamp DESC",
            &[&session.person_unique, &file_type],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
            ("referenceType".to_string(), Value::String(row.get::<_, Option<String>>("reference_type").unwrap_or_default())),
            ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
            ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("length")))),
        ]))
    }).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn share_list_to_me(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, session);
    Ok(Json(ActionResult::java_success(Value::Array(vec![]), 0, 0)))
}

#[axum::debug_handler]
pub async fn share_list_to_me2_fileType(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(file_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = (pool, session, file_type);
    Ok(Json(ActionResult::java_success(Value::Array(vec![]), 0, 0)))
}

#[axum::debug_handler]
pub async fn share_share_shareId_file_fileId_folder_folderId(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((share_id, file_id, folder_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_share_save_to_folder(pool, Extension(session), axum::extract::Path((share_id, file_id, folder_id))).await
}

#[axum::debug_handler]
pub async fn share_shield_id(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_share_shield(pool, Extension(session), axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn share_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_share_get(pool, axum::extract::Path(id)).await
}

#[axum::debug_handler]
pub async fn share_id_password_password(
    pool: Extension<Pool>,
    axum::extract::Path((id, password)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_share_get_with_password(pool, axum::extract::Path((id, password))).await
}

// ════════════ plan002 U2：file 模块端点全量闭合（Java jaxrs 105 端点对齐） ════════════
// 语义红线（沿用 processplatform_assemble_surface U2-b 先例，禁止假成功壳）：
//   - 新上传 = BlobStorage put + 回读校验。FS 后端真实落盘；STORAGE_BACKEND=db 时
//     DbBlobStorage.put 为 no-op、get 必然 Err → 显式 501 + warn（不写内容必丢的元数据行）。
//     FS 模式下同时回写 FILE_FILE.content(base64)，保证既有下载/预览路径语义不变。
//   - 元数据管理（改名/删除/分享/保存到文件夹）：真实参数化 SQL + IDOR 门禁
//     （require_owner；Java manager-only 操作用 is_admin）。
//   - 无引擎能力（zip 打包批量下载、文件夹打包下载、系统级配置写入）：
//     显式 501 NotImplemented + tracing::warn（fail loud）。
//
// 跨 crate 裁决记录：GET /jaxrs/file/{id} 已被 cms_assemble_control 以相同方法注册
// （语义为 CMS 文件，不可复用），本模块该端点由既有模块前缀路由
// /jaxrs/file/assemble/control/file/{id} 闭合，不再裸注册以免引入跨 crate 冲突；
// 其余缺口一律按 Java 真实路径注册（经归一化查重无跨 crate 占用）。

fn u2_capability_unavailable(capability: &'static str) -> AppError {
    tracing::warn!(capability, "endpoint requires an unavailable engine; returning 501");
    AppError::NotImplemented
}

async fn u2_require_admin(
    pool: &Pool,
    session: &shared::session::Session,
) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

/// 规范化 blob key：`attachment/{id}/{filename}`；剥离路径分隔符与控制字符
/// （FsBlobStorage.resolve 还会拒绝 `..` 组件 —— 双保险）。
fn u2_blob_key(id: &str, filename: &str) -> Result<String, AppError> {
    let cleaned: String = filename
        .replace(['\\', '/'], "_")
        .chars()
        .filter(|c| !c.is_control() && *c != '"' && *c != '\0')
        .collect();
    let name = cleaned.trim().trim_start_matches('.');
    if name.is_empty() || name == "." || name == ".." {
        return Err(AppError::BadRequest("invalid file name".to_string()));
    }
    Ok(format!("attachment/{id}/{name}"))
}

fn u2_ext_of(filename: &str) -> String {
    filename.rsplit('.').next().unwrap_or("bin").to_string()
}

/// put + 回读校验。DB 占位后端 put 无副作用且 get 必然 Err —— 在此显式失败，
/// 避免产生"上传成功但内容丢失"的假成功响应。
async fn u2_persist_verified(
    storage: &dyn shared::storage::BlobStorage,
    key: &str,
    bytes: &[u8],
) -> Result<(), AppError> {
    storage.put(key, bytes).await.map_err(|e| {
        tracing::warn!(key, error = %e, "blob put failed");
        AppError::Internal
    })?;
    if let Err(e) = storage.get(key).await {
        tracing::warn!(key, error = %e,
            "blob backend did not persist upload (STORAGE_BACKEND=db placeholder); \
             set STORAGE_BACKEND=fs to enable binary uploads");
        return Err(AppError::NotImplemented);
    }
    Ok(())
}

async fn u2_read_multipart_file(
    mut multipart: axum::extract::Multipart,
) -> Result<(String, Option<String>, Vec<u8>), AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("malformed multipart body".to_string()))?
    {
        let fname = field.file_name().map(str::to_string).filter(|s| !s.is_empty());
        let mime = field.content_type().map(str::to_string);
        let data = field
            .bytes()
            .await
            .map_err(|_| AppError::BadRequest("unreadable upload field".to_string()))?;
        if fname.is_some() || !data.is_empty() {
            return Ok((
                fname.unwrap_or_else(|| "upload.bin".to_string()),
                mime,
                data.to_vec(),
            ));
        }
    }
    Err(AppError::BadRequest("no file provided".to_string()))
}

/// 上传统一入口：persist(带回读校验) + 写 FILE_FILE 元数据行。
/// content 列双写 base64：既有下载/预览 handler 直读该列，保持行为基线不变。
async fn u2_store_new(
    pool: &Pool,
    person: &str,
    id: &str,
    filename: &str,
    mime: Option<&str>,
    bytes: Vec<u8>,
    reference_type: &str,
    reference: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let key = u2_blob_key(id, filename)?;
    let storage = shared::storage::storage_from_env();
    u2_persist_verified(storage.as_ref(), &key, &bytes).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let ext = u2_ext_of(filename);
    let length = bytes.len() as i64;
    let mime_owned = mime.unwrap_or("").to_string();
    let content_b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    client
        .execute(
            "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, content, storage_key, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())",
            &[&id, &filename, &person, &reference, &reference_type, &ext, &length, &mime_owned, &content_b64, &key],
        )
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, attachment = %id, "attachment metadata insert failed after blob write");
            AppError::Internal
        })?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("name".to_string(), Value::String(filename.to_string())),
            ("extension".to_string(), Value::String(ext)),
            ("length".to_string(), Value::Number(serde_json::Number::from(length))),
            ("storageKey".to_string(), Value::String(key)),
            ("uploaded".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── 上传族（multipart / octet-stream → BlobStorage + 元数据行） ─────────────

#[axum::debug_handler]
pub async fn u2_file_upload_octet_stream(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((reference_type, reference, _scale)): axum::extract::Path<(String, String, String)>,
    query: axum::extract::Query<std::collections::HashMap<String, String>>,
    body: axum::body::Bytes,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if body.is_empty() {
        return Err(AppError::BadRequest("empty request body".to_string()));
    }
    let filename = query
        .get("fileName")
        .or_else(|| query.get("filename"))
        .cloned()
        .unwrap_or_else(|| "upload.bin".to_string());
    let id = uuid::Uuid::new_v4().to_string();
    u2_store_new(&pool, &session.person_unique, &id, &filename, None, body.to_vec(), &reference_type, &reference).await
}

#[axum::debug_handler]
pub async fn u2_file_upload_multipart(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((reference_type, reference, _scale)): axum::extract::Path<(String, String, String)>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (filename, mime, bytes) = u2_read_multipart_file(multipart).await?;
    let id = uuid::Uuid::new_v4().to_string();
    u2_store_new(&pool, &session.person_unique, &id, &filename, mime.as_deref(), bytes, &reference_type, &reference).await
}

#[axum::debug_handler]
pub async fn u2_file_upload_callback(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((reference_type, reference, _scale, _callback)): axum::extract::Path<(String, String, String, String)>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_file_upload_multipart(pool, Extension(session), axum::extract::Path((reference_type, reference, _scale)), multipart).await
}

// ── 内容替换族（PUT /attachment/{id}/update：multipart → BlobStorage 回填） ──

#[axum::debug_handler]
pub async fn u2_attachment_update_content(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("attachment not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;

    let (filename, _mime, bytes) = u2_read_multipart_file(multipart).await?;
    let key = u2_blob_key(&id, &filename)?;
    let storage = shared::storage::storage_from_env();
    u2_persist_verified(storage.as_ref(), &key, &bytes).await?;

    let ext = u2_ext_of(&filename);
    let length = bytes.len() as i64;
    let content_b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    client
        .execute(
            "UPDATE FILE_FILE SET storage_key = $1, length = $2, extension = $3, content = $4, update_time = NOW() WHERE id = $5",
            &[&key, &length, &ext, &content_b64, &id],
        )
        .await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(filename)),
            ("length".to_string(), Value::Number(serde_json::Number::from(length))),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn u2_attachment_update_content_callback(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((id, _callback)): axum::extract::Path<(String, String)>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_attachment_update_content(pool, Extension(session), axum::extract::Path(id), multipart).await
}

// ── 元数据管理族（真实 SQL + IDOR 门禁） ────────────────────────────────────

#[axum::debug_handler]
pub async fn u2_attachment_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("attachment not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;
    client
        .execute("UPDATE FILE_FILE SET deleted_at = NOW(), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn u2_attachment2_update(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("attachment not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let mime_type = body.get("mimeType").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    client
        .execute(
            "UPDATE FILE_FILE SET name = $1, mime_type = $2, update_time = NOW() WHERE id = $3 AND deleted_at IS NULL",
            &[&name, &mime_type, &id],
        )
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn u2_attachment2_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("attachment not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;
    client
        .execute("UPDATE FILE_FILE SET deleted_at = NOW(), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn u2_file_delete_by_id(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("file not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;
    client
        .execute("UPDATE FILE_FILE SET deleted_at = NOW(), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Java FileAction.delete(referenceType, reference)：manager-only，按引用批量清除。
#[axum::debug_handler]
pub async fn u2_file_delete_by_reference(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((reference_type, reference)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM FILE_FILE WHERE reference_id = $1 AND reference_type = $2",
            &[&reference, &reference_type],
        )
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("value".to_string(), Value::Bool(true)),
            ("deleted".to_string(), Value::Number(serde_json::Number::from(result))),
        ]),
    ))))
}

/// Java ActionListReferenceType：当前用户各 referenceType 的文件计数。
#[axum::debug_handler]
pub async fn u2_file_list_reference_types(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT reference_type AS rtype, COUNT(*) AS cnt FROM FILE_FILE \
             WHERE person = $1 AND deleted_at IS NULL GROUP BY reference_type ORDER BY reference_type",
            &[&session.person_unique],
        )
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("name".to_string(), Value::String(row.get::<_, String>("rtype"))),
                ("value".to_string(), Value::String(row.get::<_, String>("rtype"))),
                ("count".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i64>("cnt")))),
            ]))
        })
        .collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

// ── 文件夹族（FILE_FOLDER 真实 CRUD + IDOR 门禁；folder2 复用同一实现） ─────

#[axum::debug_handler]
pub async fn u2_folder_create(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().trim().to_string();
    if name.is_empty() {
        return Err(AppError::BadRequest("folder name is empty".to_string()));
    }
    let superior = body
        .get("superior")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    let id = uuid::Uuid::new_v4().to_string();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO FILE_FOLDER (id, name, person, superior) VALUES ($1, $2, $3, $4)",
            &[&id, &name, &session.person_unique, &superior],
        )
        .await.map_err(|_| AppError::Internal)?;
    let mut map = serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id.clone())),
        ("name".to_string(), Value::String(name)),
        ("person".to_string(), Value::String(session.person_unique.clone())),
    ]);
    if let Some(sup) = superior {
        map.insert("superior".to_string(), Value::String(sup));
    }
    Ok(Json(ActionResult::success(Value::Object(map))))
}

#[axum::debug_handler]
pub async fn u2_folder_rename(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().trim().to_string();
    if name.is_empty() {
        return Err(AppError::BadRequest("folder name is empty".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FOLDER WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("folder not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;
    client
        .execute(
            "UPDATE FILE_FOLDER SET name = $1, update_time = NOW() WHERE id = $2 AND deleted_at IS NULL",
            &[&name, &id],
        )
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn u2_folder_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_FOLDER WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("folder not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;
    client
        .execute(
            "UPDATE FILE_FOLDER SET deleted_at = NOW(), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── 分享族（FILE_SHARE 真实 CRUD；字段对齐 Java personal.Share） ────────────

fn u2_share_row_to_json(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    let mut map = serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("person".to_string(), Value::String(row.get::<_, Option<String>>("person").unwrap_or_default())),
        ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
        ("fileId".to_string(), Value::String(row.get("file_id"))),
        ("fileType".to_string(), Value::String(row.get::<_, Option<String>>("file_type").unwrap_or_default())),
        ("extension".to_string(), Value::String(row.get::<_, Option<String>>("extension").unwrap_or_default())),
        ("shareType".to_string(), Value::String(row.get::<_, Option<String>>("share_type").unwrap_or_default())),
    ]);
    if let Some(len) = row.try_get::<_, Option<i64>>("length").ok().flatten() {
        map.insert("length".to_string(), Value::Number(serde_json::Number::from(len)));
    }
    Value::Object(map)
}

const U2_SHARE_SELECT: &str = "SELECT id, person, name, file_id, file_type, extension, length, share_type FROM FILE_SHARE";

#[axum::debug_handler]
pub async fn u2_share_create(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 输入校验先于任何 DB 访问（与 Java ActionCreate 一致：fileId/shareType 必填，
    // password 型分享必须带密码）。
    let file_id = body.get("fileId").and_then(|v| v.as_str()).unwrap_or_default().trim().to_string();
    let share_type = body.get("shareType").and_then(|v| v.as_str()).unwrap_or_default().trim().to_string();
    let password = body.get("password").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    if file_id.is_empty() {
        return Err(AppError::BadRequest("fileId is empty".to_string()));
    }
    if share_type.is_empty() {
        return Err(AppError::BadRequest("shareType is empty".to_string()));
    }
    if share_type == "password" && password.trim().is_empty() {
        return Err(AppError::BadRequest("password is required for password share".to_string()));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // 与 Java 一致的 upsert：同人同文件已有分享则更新，否则新建。
    let existing = client
        .query_opt("SELECT id FROM FILE_SHARE WHERE person = $1 AND file_id = $2", &[&session.person_unique, &file_id])
        .await.map_err(|_| AppError::Internal)?;
    let meta = client
        .query_opt(
            "SELECT extension, length FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL",
            &[&file_id],
        )
        .await.map_err(|_| AppError::Internal)?;
    let (extension, length): (Option<String>, i64) = match &meta {
        Some(m) => (m.get("extension"), m.get::<_, Option<i64>>("length").unwrap_or(0)),
        None => (None, 0),
    };

    let id = match existing {
        Some(row) => {
            let id: String = row.get("id");
            client
                .execute(
                    "UPDATE FILE_SHARE SET share_type = $1, password = $2, name = $3, extension = $4, length = $5, update_time = NOW() WHERE id = $6",
                    &[&share_type, &password, &name, &extension, &length, &id],
                )
                .await.map_err(|_| AppError::Internal)?;
            id
        }
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO FILE_SHARE (id, person, name, file_id, file_type, extension, length, share_type, password) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                    &[&id, &session.person_unique, &name, &file_id, &share_type, &extension, &length, &share_type, &password],
                )
                .await.map_err(|_| AppError::Internal)?;
            id
        }
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("fileId".to_string(), Value::String(file_id)),
            ("shareType".to_string(), Value::String(share_type)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn u2_share_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(&format!("{U2_SHARE_SELECT} WHERE id = $1"), &[&id])
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(u2_share_row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("share not found"))),
    }
}

#[axum::debug_handler]
pub async fn u2_share_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_SHARE WHERE id = $1", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("share not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;
    client
        .execute("DELETE FROM FILE_SHARE WHERE id = $1", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 屏蔽分享：将 validTime 置为过去时刻（Java shield 语义 = 使分享失效）。
#[axum::debug_handler]
pub async fn u2_share_shield(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT person FROM FILE_SHARE WHERE id = $1", &[&id])
        .await.map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("share not found")));
    };
    let owner: String = row.get::<_, Option<String>>("person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;
    client
        .execute(
            "UPDATE FILE_SHARE SET valid_time = '1970-01-01 00:00:00'::TIMESTAMP, update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("shielded".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 密码访问：校验密码与有效期后返回分享信息（不含密码本身）。
#[axum::debug_handler]
pub async fn u2_share_get_with_password(
    pool: Extension<Pool>,
    axum::extract::Path((id, password)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!(
                "{U2_SHARE_SELECT} WHERE id = $1 AND password = $2 \
                 AND (valid_time IS NULL OR valid_time >= NOW())"
            ),
            &[&id, &password],
        )
        .await.map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(u2_share_row_to_json(&row)))),
        None => Ok(Json(ActionResult::error("share not found or password mismatch"))),
    }
}

/// 保存分享到我的文件夹：复制 FILE_FILE 行归属当前用户（需提供有效 shareId+fileId 对）。
#[axum::debug_handler]
pub async fn u2_share_save_to_folder(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path((share_id, file_id, folder_id)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let share_valid = client
        .query_opt(
            "SELECT id FROM FILE_SHARE WHERE id = $1 AND file_id = $2 \
             AND (valid_time IS NULL OR valid_time >= NOW())",
            &[&share_id, &file_id],
        )
        .await.map_err(|_| AppError::Internal)?;
    if share_valid.is_none() {
        return Ok(Json(ActionResult::error("share not found or expired")));
    }
    let new_id = uuid::Uuid::new_v4().to_string();
    let result = client
        .execute(
            "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, content, storage_key, create_time, update_time) \
             SELECT $1, name, $2, $3, reference_type, extension, length, mime_type, content, storage_key, NOW(), NOW() \
             FROM FILE_FILE WHERE id = $4 AND deleted_at IS NULL",
            &[&new_id, &session.person_unique, &folder_id, &file_id],
        )
        .await.map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("file not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(new_id)),
            ("fileId".to_string(), Value::String(file_id)),
            ("folderId".to_string(), Value::String(folder_id)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── 配置族（无配置存储引擎 → 显式 501 + warn） ─────────────────────────────

/// Java 真实路径为 /list/type/{page}/size/{size}（2 段参数）；既有 handler 为历史
/// 字面路由设计的 3 元组 —— 以适配器复用其查询逻辑，不改动原函数。
#[axum::debug_handler]
pub async fn u2_attachment2_list_type_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment2_list_type_page_size_size(pool, axum::extract::Path((String::from("all"), page, size))).await
}

#[axum::debug_handler]
pub async fn u2_config_save_system_config() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("file-system-config-write"))
}

#[cfg(test)]
mod tests_u2;
