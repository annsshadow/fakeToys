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

pub fn hotpic_assemble_control_router(pool: Pool) -> axum::Router {
    routes::router(pool)
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/bbs/{id}", get(stub_hotpic_assemble_control_cipher_hotpic_bbs_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/cms/{id}", get(stub_hotpic_assemble_control_cipher_hotpic_cms_id))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/{page}/count/{count}", get(stub_hotpic_assemble_control_cipher_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic/assemble/control/cipher/hotpic/{id}", get(stub_hotpic_assemble_control_cipher_hotpic_id))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/changeTitle", post(stub_hotpic_assemble_control_user_hotpic_changeTitle))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/exists/check", post(stub_hotpic_assemble_control_user_hotpic_exists_check))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count}", get(stub_hotpic_assemble_control_user_hotpic_filter_list_page_page_count_count))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/{application}/{infoId}", get(stub_hotpic_assemble_control_user_hotpic_application_infoId))
        .route("/jaxrs/hotpic/assemble/control/user/hotpic/{id}", get(stub_hotpic_assemble_control_user_hotpic_id))
}

#[axum::debug_handler]
pub async fn get_control_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(true)),
        ("cacheEnabled".to_string(), Value::Bool(true)),
        ("defaultScale".to_string(), Value::Number(serde_json::Number::from_f64(1.0).unwrap())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_panels(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let panels = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("heatmap".to_string())),
            ("name".to_string(), Value::String("Heatmap".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("type".to_string(), Value::String("hotpic".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("annotation".to_string())),
            ("name".to_string(), Value::String("Annotation".to_string())),
            ("enabled".to_string(), Value::Bool(false)),
            ("type".to_string(), Value::String("hotpic".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(panels.len() as i64))),
            ("data".to_string(), Value::Array(panels)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating hotpic assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn list_control_applications(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let applications = vec![
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String("hr".to_string())),
            ("name".to_string(), Value::String("HR Application".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String("finance".to_string())),
            ("name".to_string(), Value::String("Finance Application".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(applications.len() as i64))),
            ("data".to_string(), Value::Array(applications)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    hotpic_assemble_control_router(pool)
        .route("/hotpic_assemble_control/health", axum::routing::get(|| async { "ok" }))
}

#[derive(Debug, serde::Deserialize)]
pub struct HotpicRequest {
    pub title: Option<String>,
    pub imageUrl: Option<String>,
}

#[axum::debug_handler]
pub async fn list_hotpics(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
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
pub async fn get_hotpic(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

#[axum::debug_handler]
pub async fn create_hotpic(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): Json<HotpicRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let title = req.title.unwrap_or_default();
    let image_url = req.imageUrl.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_hotpic (id, title, image_url, creator, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &title, &image_url, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("imageUrl".to_string(), Value::String(image_url)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn save_hotpic(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<HotpicRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let title = req.title.unwrap_or_default();
    let image_url = req.imageUrl.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_hotpic SET title = $1, image_url = $2 WHERE id = $3 AND deleted_at IS NULL",
            &[&title, &image_url, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("hotpic not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("title".to_string(), Value::String(title)),
            ("imageUrl".to_string(), Value::String(image_url)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn delete_hotpic(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "UPDATE x_hotpic SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("hotpic not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/hotpic/assemble/control/cipher/hotpic/bbs/{id}
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_cipher_hotpic_bbs_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

/// Stub handler for /jaxrs/hotpic/assemble/control/cipher/hotpic/cms/{id}
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_cipher_hotpic_cms_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

/// Stub handler for /jaxrs/hotpic/assemble/control/cipher/hotpic/filter/list/page/{page}/count/{count}
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_cipher_hotpic_filter_list_page_page_count_count(
    pool: Option<Extension<Pool>>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let offset = (page - 1) * count;
    let rows = client
        .query(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET $1",
            &[&offset, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
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

/// Stub handler for /jaxrs/hotpic/assemble/control/cipher/hotpic/{id}
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_cipher_hotpic_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

/// Stub handler for /jaxrs/hotpic/assemble/control/user/hotpic/changeTitle
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_user_hotpic_changeTitle(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = req.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let title = req.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();

    if id.is_empty() {
        return Ok(Json(ActionResult::error("id is required")));
    }

    let result = client
        .execute(
            "UPDATE x_hotpic SET title = $1 WHERE id = $2 AND deleted_at IS NULL",
            &[&title, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("hotpic not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("title".to_string(), Value::String(title)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/hotpic/assemble/control/user/hotpic/exists/check
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_user_hotpic_exists_check(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let person_id = req.get("personId").and_then(|v| v.as_str()).unwrap_or("");
    let _application = req.get("application").and_then(|v| v.as_str()).unwrap_or("");

    if person_id.is_empty() {
        return Ok(Json(ActionResult::success(Value::Bool(false))));
    }

    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM x_hotpic WHERE creator = $1 AND deleted_at IS NULL",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");
    Ok(Json(ActionResult::success(Value::Bool(count > 0))))
}

/// Stub handler for /jaxrs/hotpic/assemble/control/user/hotpic/filter/list/page/{page}/count/{count}
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_user_hotpic_filter_list_page_page_count_count(
    pool: Option<Extension<Pool>>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let offset = (page - 1) * count;
    let rows = client
        .query(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET $1",
            &[&offset, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
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

/// Stub handler for /jaxrs/hotpic/assemble/control/user/hotpic/{application}/{infoId}
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_user_hotpic_application_infoId(
    pool: Option<Extension<Pool>>,
    Path((application, info_id)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&info_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let mut result = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]);
            result.insert("application".to_string(), Value::String(application));
            Ok(Json(ActionResult::success(Value::Object(result))))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

/// Stub handler for /jaxrs/hotpic/assemble/control/user/hotpic/{id}
/// TODO: Implement real business logic
pub async fn stub_hotpic_assemble_control_user_hotpic_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("imageUrl".to_string(), Value::String(row.get("image_url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}
