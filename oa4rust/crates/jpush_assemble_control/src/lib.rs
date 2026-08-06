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

#[axum::debug_handler]
pub async fn get_control_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(true)),
        ("defaultAppKey".to_string(), Value::String("default".to_string())),
        ("maxPushCount".to_string(), Value::Number(serde_json::Number::from(10000i64))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_apps(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let apps = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("default".to_string())),
            ("name".to_string(), Value::String("Default App".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Array(apps))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating jpush assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

pub fn jpush_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
        .route("/jaxrs/jpush/assemble/control/device/admin/unbind/all/person", post(stub_jpush_assemble_control_device_admin_unbind_all_person))
        .route("/jaxrs/jpush/assemble/control/device/bind", post(stub_jpush_assemble_control_device_bind))
        .route("/jaxrs/jpush/assemble/control/device/check/{deviceName}/{deviceType}/{pushType}", get(stub_jpush_assemble_control_device_check_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush/assemble/control/device/config/push/type", get(stub_jpush_assemble_control_device_config_push_type))
        .route("/jaxrs/jpush/assemble/control/device/list/{pushType}", get(stub_jpush_assemble_control_device_list_pushType))
        .route("/jaxrs/jpush/assemble/control/device/unbind/new/{deviceName}/{deviceType}/{pushType}", post(stub_jpush_assemble_control_device_unbind_new_deviceName_deviceType_pushType))
        .route("/jaxrs/jpush/assemble/control/device/unbind/{deviceName}/{deviceType}", post(stub_jpush_assemble_control_device_unbind_deviceName_deviceType))
        .route("/jaxrs/jpush/assemble/control/message/test/send", post(stub_jpush_assemble_control_message_test_send))
}

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    jpush_assemble_control_router(pool)
        .route("/jpush_assemble_control/health", axum::routing::get(|| async { "ok" }))
}

#[derive(Debug, serde::Deserialize)]
pub struct JpushRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub target: Option<String>,
}

#[axum::debug_handler]
pub async fn list_jpushs(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): Json<JpushRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<JpushRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/jpush/assemble/control/device/admin/unbind/all/person
/// TODO: Implement real business logic
pub async fn stub_jpush_assemble_control_device_admin_unbind_all_person(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/jpush/assemble/control/device/bind
/// TODO: Implement real business logic
pub async fn stub_jpush_assemble_control_device_bind(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/jpush/assemble/control/device/check/{deviceName}/{deviceType}/{pushType}
/// TODO: Implement real business logic
pub async fn stub_jpush_assemble_control_device_check_deviceName_deviceType_pushType(
    pool: Option<Extension<Pool>>,
    Path((device_name, device_type, push_type)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/jpush/assemble/control/device/config/push/type
/// TODO: Implement real business logic
pub async fn stub_jpush_assemble_control_device_config_push_type(
    pool: Option<Extension<Pool>>,
    Path(push_type): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/jpush/assemble/control/device/list/{pushType}
/// TODO: Implement real business logic
pub async fn stub_jpush_assemble_control_device_list_pushType(
    pool: Option<Extension<Pool>>,
    Path(push_type): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/jpush/assemble/control/device/unbind/new/{deviceName}/{deviceType}/{pushType}
/// TODO: Implement real business logic
pub async fn stub_jpush_assemble_control_device_unbind_new_deviceName_deviceType_pushType(
    pool: Option<Extension<Pool>>,
    Path((device_name, device_type, push_type)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/jpush/assemble/control/device/unbind/{deviceName}/{deviceType}
/// TODO: Implement real business logic
pub async fn stub_jpush_assemble_control_device_unbind_deviceName_deviceType(
    pool: Option<Extension<Pool>>,
    Path((device_name, device_type)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/jpush/assemble/control/message/test/send
/// TODO: Implement real business logic
pub async fn stub_jpush_assemble_control_message_test_send(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
