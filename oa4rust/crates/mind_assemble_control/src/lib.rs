use axum::{
    extract::{Extension, Path},
    Json, Router,
    routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, config_data, creator, create_time FROM x_mind_assemble_control_config LIMIT 1",
            &[],
        )
        .await;

    let data = match row {
        Ok(r) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            ("configData".to_string(), Value::String(r.get("config_data"))),
            ("creator".to_string(), Value::String(r.get("creator"))),
            ("createTime".to_string(), Value::String(r.get("create_time"))),
        ]),
        Err(_) => serde_json::Map::from_iter([
            ("id".to_string(), Value::String(String::new())),
            ("configData".to_string(), Value::String(String::new())),
            ("creator".to_string(), Value::String(String::new())),
            ("createTime".to_string(), Value::String(String::new())),
        ]),
    };

    Ok(Json(ActionResult::success(Value::Object(data))))
}

pub async fn update_control_config(
    pool: Extension<Pool>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let config_id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let config_data = payload.get("configData").and_then(|v| v.as_str()).unwrap_or("");

    if config_id.is_empty() {
        return Ok(Json(ActionResult::error("id is required")));
    }

    client
        .execute(
            "UPDATE x_mind_assemble_control_config SET config_data = $1 WHERE id = $2",
            &[&config_data, &config_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(config_id.to_string())),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

pub fn mind_assemble_control_router(pool: Pool) -> Router {
    routes::mind_assemble_control_routes(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::routes::mind_assemble_control_routes(pool)
}


#[derive(Debug, serde::Deserialize)]
pub struct MindFolderRequest {
    pub name: Option<String>,
    pub content: Option<String>,
    pub parentId: Option<String>,
}

#[axum::debug_handler]
pub async fn list_folders(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, content, creator, create_time FROM x_mind WHERE parent_id IS NULL AND deleted_at IS NULL ORDER BY create_time DESC",
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
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
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
pub async fn get_folder(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content, creator, create_time FROM x_mind WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("mind folder not found"))),
    }
}

#[axum::debug_handler]
pub async fn save_folder(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<MindFolderRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let content = req.content.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_mind (id, name, content, creator, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &name, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("content".to_string(), Value::String(content)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn update_folder(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content FROM x_mind WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ActionResult::error("mind folder not found"))),
    };

    let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("name"));
    let content = payload.get("content").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("content"));

    client
        .execute(
            "UPDATE x_mind SET name = $1, content = $2 WHERE id = $3 AND deleted_at IS NULL",
            &[&name, &content, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn folder_move_folderId(
    pool: Extension<Pool>,
    Path(folder_id): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let parent_id = req.get("parentId").and_then(|v| v.as_str()).map(|s| s.to_string());

    let result = if let Some(pid) = parent_id {
        if pid.is_empty() {
            client
                .execute(
                    "UPDATE x_mind SET parent_id = NULL WHERE id = $1 AND deleted_at IS NULL",
                    &[&folder_id],
                )
                .await
                .map_err(|_| AppError::Internal)?
        } else {
            client
                .execute(
                    "UPDATE x_mind SET parent_id = $1 WHERE id = $2 AND deleted_at IS NULL",
                    &[&pid, &folder_id],
                )
                .await
                .map_err(|_| AppError::Internal)?
        }
    } else {
        client
            .execute(
                "UPDATE x_mind SET parent_id = NULL WHERE id = $1 AND deleted_at IS NULL",
                &[&folder_id],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    if result == 0 {
        return Ok(Json(ActionResult::error("folder not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("folderId".to_string(), Value::String(folder_id)),
            ("moved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn folder_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content, creator, create_time FROM x_mind WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("mind folder not found"))),
    }
}

pub async fn folder_id_force(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_mind SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("mind folder not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}
