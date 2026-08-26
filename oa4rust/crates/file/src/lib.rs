use axum::{
    extract::{Extension, Multipart},
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::Value;
use shared::{
    error::AppError,
    input_validation::{validate_file_size, validate_length, validate_mime_type, validate_required},
    response::ActionResult,
};

const ALLOWED_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "application/pdf",
    "text/plain",
];
const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024;

pub mod routes;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}

#[derive(Debug, Serialize)]
struct ComplexTopResponse {
    folder_list: Vec<Value>,
    attachment_list: Vec<Value>,
}

#[utoipa::path(
    get,
    path = "/jaxrs/file/folder/list/top",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
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
            let mut entries = vec![
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
            ];
            if let Some(superior) = row.get::<_, Option<String>>("superior") {
                entries.push(("superior".to_string(), Value::String(superior)));
            }
            Value::Object(serde_json::Map::from_iter(entries))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[utoipa::path(
    get,
    path = "/jaxrs/file/folder/list/{id}",
    params(
        ("id" = String, Path, description = "Folder ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
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
            let mut entries = vec![
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
            ];
            if let Some(superior) = row.get::<_, Option<String>>("superior") {
                entries.push(("superior".to_string(), Value::String(superior)));
            }
            Value::Object(serde_json::Map::from_iter(entries))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[utoipa::path(
    get,
    path = "/jaxrs/file/complex/top",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
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
            let mut entries = vec![
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                (
                    "attachmentCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
                ("size".to_string(), Value::Number(serde_json::Number::from(0))),
                (
                    "folderCount".to_string(),
                    Value::Number(serde_json::Number::from(0)),
                ),
            ];
            if let Some(superior) = row.get::<_, Option<String>>("superior") {
                entries.push(("superior".to_string(), Value::String(superior)));
            }
            Value::Object(serde_json::Map::from_iter(entries))
        })
        .collect();

    let attachment_rows = client
        .query(
            "SELECT id, name, person, \"referenceType\", extension, length FROM FILE_FILE ORDER BY name LIMIT 10",
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
                    "\"referenceType\"".to_string(),
                    Value::String(row.get::<_, Option<String>>("referenceType").unwrap_or_default()),
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

#[utoipa::path(
    post,
    path = "/jaxrs/file/upload",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
#[axum::debug_handler]
pub async fn file_upload(
    pool: Extension<Pool>,
    mut form: Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut mime_type: Option<String> = None;
    let mut filename: Option<String> = None;
    let mut name: Option<String> = None;
    let mut person: Option<String> = None;
    let mut reference_id: Option<String> = None;
    let mut reference_type: Option<String> = None;

    while let Some(field) = form
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("表单解析失败".to_string()))?
    {
        if let Some(fname) = field.file_name() {
            mime_type = field.content_type().map(|s| s.to_string());
            filename = Some(fname.to_string());
            file_data = Some(
                field
                    .bytes()
                    .await
                    .map_err(|_| AppError::BadRequest("文件读取失败".to_string()))?
                    .to_vec(),
            );
        } else {
            let field_name = field.name().unwrap_or("").to_string();
            let value = field
                .text()
                .await
                .map_err(|_| AppError::BadRequest("表单读取失败".to_string()))?;
            match field_name.as_str() {
                "name" => name = Some(value),
                "person" => person = Some(value),
                "referenceId" => reference_id = Some(value),
                "\"referenceType\"" => reference_type = Some(value),
                _ => {}
            }
        }
    }

    let data = match file_data {
        Some(d) => d,
        None => return Ok(Json(ActionResult::error("未提供文件"))),
    };

    let mime = mime_type.as_deref().unwrap_or("");
    validate_mime_type("mimeType", mime, ALLOWED_MIME_TYPES).map_err(|e| e.to_app_error())?;
    validate_file_size("file", data.len() as u64, MAX_FILE_SIZE).map_err(|e| e.to_app_error())?;

    let ext = match mime {
        "image/jpeg" => "jpg".to_string(),
        "image/png" => "png".to_string(),
        "image/gif" => "gif".to_string(),
        "application/pdf" => "pdf".to_string(),
        "text/plain" => "txt".to_string(),
        _ => {
            if let Some(ref fname) = filename {
                if let Some(ext) = fname.split('.').last() {
                    ext.to_lowercase()
                } else {
                    "bin".to_string()
                }
            } else {
                "bin".to_string()
            }
        }
    };

    let id = uuid::Uuid::new_v4().to_string();
    let name = name.unwrap_or_else(|| filename.clone().unwrap_or_else(|| format!("file-{}", id)));
    let person = person.unwrap_or_else(|| "system".to_string());
    let reference_id = reference_id.unwrap_or_default();
    let reference_type = reference_type.unwrap_or_else(|| "file".to_string());

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())",
            &[&id, &name, &person, &reference_id, &reference_type, &ext, &(data.len() as i64), &mime],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("person".to_string(), Value::String(person)),
        ("referenceId".to_string(), Value::String(reference_id)),
        ("referenceType".to_string(), Value::String(reference_type)),
        ("extension".to_string(), Value::String(ext)),
        (
            "length".to_string(),
            Value::Number(serde_json::Number::from(data.len() as i64)),
        ),
        ("mimeType".to_string(), Value::String(mime.to_string())),
        ("createTime".to_string(), Value::String(chrono::Local::now().to_rfc3339())),
        ("updateTime".to_string(), Value::String(chrono::Local::now().to_rfc3339())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub(crate) async fn upload_file_record(
    pool: Extension<Pool>,
    data: Vec<u8>,
    mime: String,
    filename: Option<String>,
    name: Option<String>,
    person: Option<String>,
    reference_id: Option<String>,
    reference_type: Option<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_mime_type("mimeType", &mime, ALLOWED_MIME_TYPES).map_err(|e| e.to_app_error())?;
    validate_file_size("file", data.len() as u64, MAX_FILE_SIZE).map_err(|e| e.to_app_error())?;

    let ext = match mime.as_str() {
        "image/jpeg" => "jpg".to_string(),
        "image/png" => "png".to_string(),
        "image/gif" => "gif".to_string(),
        "application/pdf" => "pdf".to_string(),
        "text/plain" => "txt".to_string(),
        _ => {
            if let Some(ref fname) = filename {
                if let Some(ext) = fname.split('.').last() {
                    ext.to_lowercase()
                } else {
                    "bin".to_string()
                }
            } else {
                "bin".to_string()
            }
        }
    };

    let id = uuid::Uuid::new_v4().to_string();
    let name = name.unwrap_or_else(|| filename.clone().unwrap_or_else(|| format!("file-{}", id)));
    let person = person.unwrap_or_else(|| "system".to_string());
    let reference_id = reference_id.unwrap_or_default();
    let reference_type = reference_type.unwrap_or_else(|| "file".to_string());

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO FILE_FILE (id, name, person, reference_id, reference_type, extension, length, mime_type, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())",
            &[&id, &name, &person, &reference_id, &reference_type, &ext, &(data.len() as i64), &mime],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("person".to_string(), Value::String(person)),
        ("referenceId".to_string(), Value::String(reference_id)),
        ("referenceType".to_string(), Value::String(reference_type)),
        ("extension".to_string(), Value::String(ext)),
        (
            "length".to_string(),
            Value::Number(serde_json::Number::from(data.len() as i64)),
        ),
        ("mimeType".to_string(), Value::String(mime)),
        ("createTime".to_string(), Value::String(chrono::Local::now().to_rfc3339())),
        ("updateTime".to_string(), Value::String(chrono::Local::now().to_rfc3339())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[utoipa::path(
    get,
    path = "/jaxrs/file/download/{id}",
    params(
        ("id" = String, Path, description = "File ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
#[axum::debug_handler]
pub async fn file_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, person, reference_type, extension, length, mime_type, create_time::text FROM FILE_FILE WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("referenceType".to_string(), Value::String(row.get("reference_type"))),
                ("extension".to_string(), Value::String(row.get("extension"))),
                (
                    "length".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i64>("length"))),
                ),
                ("mimeType".to_string(), Value::String(row.get("mime_type"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[utoipa::path(
    post,
    path = "/jaxrs/file/folder/create",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
#[axum::debug_handler]
pub async fn folder_create(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let superior = body.get("superior").and_then(|v| v.as_str());
    let person = body.get("person").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    validate_required("name", &name).map_err(|e| e.to_app_error())?;
    validate_length("name", &name, 1, 255).map_err(|e| e.to_app_error())?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO FILE_FOLDER (id, name, person, superior, create_time, update_time) \
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            &[&id, &name, &person, &superior],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut entries = vec![
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("person".to_string(), Value::String(person)),
    ];
    if let Some(superior) = superior {
        entries.push(("superior".to_string(), Value::String(superior.to_string())));
    }
    let result = Value::Object(serde_json::Map::from_iter(entries));

    Ok(Json(ActionResult::success(result)))
}

#[utoipa::path(
    post,
    path = "/jaxrs/file/folder/update",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
#[axum::debug_handler]
pub async fn folder_update(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    validate_required("name", &name).map_err(|e| e.to_app_error())?;
    validate_length("name", &name, 1, 255).map_err(|e| e.to_app_error())?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE FILE_FOLDER SET name = $1, update_time = NOW() WHERE id = $2 AND deleted_at IS NULL",
            &[&name, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("folder not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("updateTime".to_string(), Value::String(chrono::Local::now().to_rfc3339())),
        ]),
    ))))
}

#[utoipa::path(
    post,
    path = "/jaxrs/file/folder/remove",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
#[axum::debug_handler]
pub async fn folder_remove(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    let result = client
        .execute(
            "UPDATE FILE_FOLDER SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("folder not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deletedAt".to_string(), Value::String(chrono::Local::now().to_rfc3339())),
        ]),
    ))))
}

#[utoipa::path(
    post,
    path = "/jaxrs/file/permission/set",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "file"
)]
#[axum::debug_handler]
pub async fn permission_set(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let target_type = body.get("targetType").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let target_id = body.get("targetId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let permissions: Option<Value> = body.get("permissions").cloned();
    let permissions_str = permissions
        .as_ref()
        .map(|v| serde_json::to_string(v).unwrap_or_default())
        .unwrap_or_default();

    if target_id.is_empty() {
        return Ok(Json(ActionResult::error("targetId is required")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO FILE_PERMISSION (id, target_type, target_id, permissions, create_time) \
             VALUES ($1, $2, $3, $4, NOW()) \
             ON CONFLICT (target_type, target_id) DO UPDATE SET permissions = $4, update_time = NOW()",
            &[&id, &target_type, &target_id, &permissions_str],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("targetType".to_string(), Value::String(target_type)),
            ("targetId".to_string(), Value::String(target_id)),
            ("permissions".to_string(), permissions.unwrap_or(Value::Object(serde_json::Map::new()))),
            ("updateTime".to_string(), Value::String(chrono::Local::now().to_rfc3339())),
        ]),
    ))))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


