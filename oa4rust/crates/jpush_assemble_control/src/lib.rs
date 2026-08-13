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


#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT COUNT(*) as cnt FROM x_jpush WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.map(|r| r.get("cnt")).unwrap_or(0);
    let enabled = count > 0;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("enabled".to_string(), Value::Bool(enabled)),
            ("defaultAppKey".to_string(), Value::String("default".to_string())),
            ("maxPushCount".to_string(), Value::Number(serde_json::Number::from(10000i64))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn list_control_apps(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT target as id FROM x_jpush WHERE deleted_at IS NULL ORDER BY target ASC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("id"))),
                ("enabled".to_string(), Value::Bool(true)),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let config = body.0;
    tracing::info!("Updating jpush assemble control config: {:?}", config);

    let id = uuid::Uuid::new_v4().to_string();
    let title = config.get("name").and_then(|v| v.as_str()).unwrap_or("default").to_string();

    client
        .execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &title, &"", &"all", &"system"],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

pub fn jpush_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::jpush_assemble_control_router(pool)
}


#[derive(Debug, serde::Deserialize)]
pub struct JpushRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub target: Option<String>,
}

#[axum::debug_handler]
pub async fn list_jpushs(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, content, target, creator, create_time FROM x_jpush WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("target".to_string(), Value::String(row.get("target"))),
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
pub async fn get_jpush(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, target, creator, create_time FROM x_jpush WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("target".to_string(), Value::String(row.get("target"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("jpush not found"))),
    }
}

#[axum::debug_handler]
pub async fn create_jpush(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<JpushRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let title = req.title.unwrap_or_default();
    let content = req.content.unwrap_or_default();
    let target = req.target.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &title, &content, &target, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("content".to_string(), Value::String(content)),
        ("target".to_string(), Value::String(target)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn save_jpush(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<JpushRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let title = req.title.unwrap_or_default();
    let content = req.content.unwrap_or_default();
    let target = req.target.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_jpush SET title = $1, content = $2, target = $3 WHERE id = $4 AND deleted_at IS NULL",
            &[&title, &content, &target, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("jpush not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("title".to_string(), Value::String(title)),
            ("content".to_string(), Value::String(content)),
            ("target".to_string(), Value::String(target)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn delete_jpush(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_jpush SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("jpush not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn device_admin_unbind_all_person(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let person_id = req.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();

    if person_id.is_empty() {
        return Ok(Json(ActionResult::error("personId is required")));
    }

    let result = client
        .execute(
            "DELETE FROM x_jpush WHERE creator = $1 AND deleted_at IS NULL",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("personId".to_string(), Value::String(person_id)),
            ("unbound".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn device_bind(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let device_name = req.get("deviceName").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let device_type = req.get("deviceType").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let push_type = req.get("pushType").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let creator = req.get("personId").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &device_name, &device_type, &push_type, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deviceName".to_string(), Value::String(device_name)),
            ("deviceType".to_string(), Value::String(device_type)),
            ("pushType".to_string(), Value::String(push_type)),
            ("bound".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn device_check_deviceName_deviceType_pushType(
    pool: Extension<Pool>,
    Path((device_name, device_type, push_type)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, target, creator, create_time FROM x_jpush WHERE title = $1 AND content = $2 AND target = $3 AND deleted_at IS NULL LIMIT 1",
            &[&device_name, &device_type, &push_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let exists = row.is_some();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("exists".to_string(), Value::Bool(exists)),
            ("deviceName".to_string(), Value::String(device_name)),
            ("deviceType".to_string(), Value::String(device_type)),
            ("pushType".to_string(), Value::String(push_type)),
        ]),
    ))))
}

pub async fn device_config_push_type(
    pool: Extension<Pool>,
    Path(push_type): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM x_jpush WHERE target = $1 AND deleted_at IS NULL",
            &[&push_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("pushType".to_string(), Value::String(push_type)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn device_list_pushType(
    pool: Extension<Pool>,
    Path(push_type): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, content, target, creator, create_time FROM x_jpush WHERE target = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&push_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("target".to_string(), Value::String(row.get("target"))),
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

pub async fn device_unbind_new_deviceName_deviceType_pushType(
    pool: Extension<Pool>,
    Path((device_name, device_type, push_type)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_jpush SET deleted_at = NOW() WHERE title = $1 AND content = $2 AND target = $3 AND deleted_at IS NULL",
            &[&device_name, &device_type, &push_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("device not found or already unbound")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deviceName".to_string(), Value::String(device_name)),
            ("deviceType".to_string(), Value::String(device_type)),
            ("pushType".to_string(), Value::String(push_type)),
            ("unbound".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn device_unbind_deviceName_deviceType(
    pool: Extension<Pool>,
    Path((device_name, device_type)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_jpush SET deleted_at = NOW() WHERE title = $1 AND content = $2 AND deleted_at IS NULL",
            &[&device_name, &device_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("device not found or already unbound")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deviceName".to_string(), Value::String(device_name)),
            ("deviceType".to_string(), Value::String(device_type)),
            ("unbound".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn message_test_send(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let title = req.get("title").and_then(|v| v.as_str()).unwrap_or("test").to_string();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let target = req.get("target").and_then(|v| v.as_str()).unwrap_or("all").to_string();
    let creator = "system";

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_jpush (id, title, content, target, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &title, &content, &target, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("title".to_string(), Value::String(title)),
            ("content".to_string(), Value::String(content)),
            ("target".to_string(), Value::String(target)),
            ("sent".to_string(), Value::Bool(true)),
        ]),
    ))))
}

