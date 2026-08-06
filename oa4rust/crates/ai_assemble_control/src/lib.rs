use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

pub fn ai_assemble_control_router(pool: Pool) -> axum::Router {
    routes::router(pool.clone())
        .merge(mcp_router(Some(pool)))
}

#[axum::debug_handler]
pub async fn get_ai_control_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("defaultModel".to_string(), Value::String("gpt-4".to_string())),
        ("temperature".to_string(), Value::Number(serde_json::Number::from_f64(0.7).unwrap())),
        ("maxTokens".to_string(), Value::Number(serde_json::Number::from(4096i64))),
        ("enabled".to_string(), Value::Bool(true)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_ai_models(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("gpt-4".to_string())),
            ("name".to_string(), Value::String("GPT-4".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("contextWindow".to_string(), Value::Number(serde_json::Number::from(8192i64))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("gpt-3.5-turbo".to_string())),
            ("name".to_string(), Value::String("GPT-3.5 Turbo".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("contextWindow".to_string(), Value::Number(serde_json::Number::from(4096i64))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("claude-3-sonnet".to_string())),
            ("name".to_string(), Value::String("Claude 3 Sonnet".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("contextWindow".to_string(), Value::Number(serde_json::Number::from(200000i64))),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(models.len() as i64))),
            ("data".to_string(), Value::Array(models)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn update_ai_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating AI assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn get_usage_stats(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("totalRequests".to_string(), Value::Number(serde_json::Number::from(0i64))),
        ("totalTokens".to_string(), Value::Number(serde_json::Number::from(0i64))),
        ("costThisMonth".to_string(), Value::Number(serde_json::Number::from_f64(0.0).unwrap())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub fn mcp_router(pool: Option<Pool>) -> axum::Router {
    let router = Router::new()
        .route("/jaxrs/ai/assemble/control/config/list/mcp/paging/{page}/size/{size}", get(stub_ai_assemble_control_config_list_mcp_paging_page_size_size))
        .route("/jaxrs/ai/assemble/control/config/get/mcp/{id}", get(stub_ai_assemble_control_config_get_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/create/mcp", post(stub_ai_assemble_control_config_create_mcp))
        .route("/jaxrs/ai/assemble/control/config/update/mcp/{id}", post(stub_ai_assemble_control_config_update_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/delete/mcp/{id}", post(stub_ai_assemble_control_config_delete_mcp_flag));

    if let Some(pool) = pool {
        router.layer(Extension(pool))
    } else {
        router
    }
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/ai_assemble_control/health", axum::routing::get(|| async { "TODO: ai_assemble_control - real implementation needed" }))
}


#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_base_config(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, default_model, temperature, max_tokens, enabled, creator FROM x_ai_mcp_config WHERE is_base = true LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("defaultModel".to_string(), Value::String(row.get("default_model"))),
                ("temperature".to_string(), Value::Number(serde_json::Number::from_f64(row.get::<_, f64>("temperature")).unwrap_or_else(|| serde_json::Number::from_f64(0.0).unwrap()))),
                ("maxTokens".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("max_tokens")))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("base config not found"))),
    }
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_create_mcp(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let url = req.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_ai_mcp_config (id, name, url, enabled, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &url, &enabled, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("url".to_string(), Value::String(url)),
        ("enabled".to_string(), Value::Bool(enabled)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_create_model(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let url = req.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_ai_model_config (id, name, url, enabled, creator, create_time, update_time) VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &url, &enabled, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("url".to_string(), Value::String(url)),
        ("enabled".to_string(), Value::Bool(enabled)),
        ("creator".to_string(), Value::String(creator.to_string())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_delete_mcp_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "DELETE FROM x_ai_mcp_config WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("mcp config not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_delete_model_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "DELETE FROM x_ai_model_config WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model config not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_get_mcp_ext_flag(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, url, enabled, creator, create_time, update_time FROM x_ai_mcp_config WHERE is_extended = true LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("extended mcp config not found"))),
    }
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_get_mcp_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, url, enabled, creator, create_time, update_time \
             FROM x_ai_mcp_config WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("mcp config not found"))),
    }
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_get_model_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, url, enabled, creator, create_time, update_time FROM x_ai_model_config WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("model config not found"))),
    }
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_list_enable_model(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, url, enabled, creator, create_time, update_time FROM x_ai_model_config WHERE enabled = true ORDER BY create_time DESC",
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
                ("url".to_string(), Value::String(row.get("url"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
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
pub async fn stub_ai_assemble_control_config_list_mcp_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let offset = (page - 1) * size;
    let rows = client
        .query(
            "SELECT id, name, url, enabled, creator, create_time, update_time \
             FROM x_ai_mcp_config \
             ORDER BY update_time DESC \
             LIMIT $2 OFFSET $1",
            &[&offset, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
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
pub async fn stub_ai_assemble_control_config_list_model_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let offset = (page - 1) * size;
    let rows = client
        .query(
            "SELECT id, name, url, enabled, creator, create_time, update_time FROM x_ai_model_config ORDER BY update_time DESC LIMIT $2 OFFSET $1",
            &[&offset, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
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
pub async fn stub_ai_assemble_control_config_save(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = req.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let url = req.get("url").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let default_model = req.get("defaultModel").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let temperature = req.get("temperature").and_then(|v| v.as_f64()).unwrap_or(0.7);
    let max_tokens = req.get("maxTokens").and_then(|v| v.as_i64()).unwrap_or(4096);
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let creator = req.get("creator").and_then(|v| v.as_str()).unwrap_or("system");

    let existing = client
        .query_opt("SELECT id FROM x_ai_mcp_config WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if existing.is_some() {
        client
            .execute(
                "UPDATE x_ai_mcp_config SET name = $1, url = $2, default_model = $3, temperature = $4, max_tokens = $5, enabled = $6, update_time = NOW() WHERE id = $7",
                &[&name, &url, &default_model, &temperature, &max_tokens, &enabled, &id],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    } else {
        let new_id = if id.is_empty() {
            uuid::Uuid::new_v4().to_string()
        } else {
            id.clone()
        };
        client
            .execute(
                "INSERT INTO x_ai_mcp_config (id, name, url, default_model, temperature, max_tokens, enabled, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())",
                &[&new_id, &name, &url, &default_model, &temperature, &max_tokens, &enabled, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_update_mcp_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let url = req.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

    let result = client
        .execute(
            "UPDATE x_ai_mcp_config SET name = $1, url = $2, enabled = $3, update_time = NOW() \
             WHERE id = $4",
            &[&name, &url, &enabled, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("mcp config not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("url".to_string(), Value::String(url)),
            ("enabled".to_string(), Value::Bool(enabled)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_update_model_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let url = req.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

    let result = client
        .execute(
            "UPDATE x_ai_model_config SET name = $1, url = $2, enabled = $3, update_time = NOW() WHERE id = $4",
            &[&name, &url, &enabled, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model config not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("url".to_string(), Value::String(url)),
            ("enabled".to_string(), Value::Bool(enabled)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_copy_file(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let source_id = req.get("sourceId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let new_name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let new_id = uuid::Uuid::new_v4().to_string();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_ai_file (id, name, file_name, file_size, file_type, enabled, creator, create_time) \
             SELECT $1, $2, file_name, file_size, file_type, enabled, $3, NOW() FROM x_ai_file WHERE id = $4",
            &[&new_id, &new_name, &creator, &source_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("copied".to_string(), Value::Bool(true)),
            ("newId".to_string(), Value::String(new_id)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_delete_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "DELETE FROM x_ai_file WHERE id = $1",
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
pub async fn stub_ai_assemble_control_file_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, file_name, file_size, file_type, enabled, creator, create_time FROM x_ai_file ORDER BY create_time DESC",
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
                ("fileName".to_string(), Value::String(row.get("file_name"))),
                ("fileSize".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("file_size")))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
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
pub async fn stub_ai_assemble_control_file_list_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let offset = (page - 1) * size;
    let rows = client
        .query(
            "SELECT id, name, file_name, file_size, file_type, enabled, creator, create_time FROM x_ai_file ORDER BY create_time DESC LIMIT $2 OFFSET $1",
            &[&offset, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("fileName".to_string(), Value::String(row.get("file_name"))),
                ("fileSize".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("file_size")))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
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
pub async fn stub_ai_assemble_control_file_upload(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let file_name = req.get("fileName").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let file_size = req.get("fileSize").and_then(|v| v.as_i64()).unwrap_or(0);
    let file_type = req.get("fileType").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_ai_file (id, name, file_name, file_size, file_type, enabled, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&id, &name, &file_name, &file_size, &file_type, &enabled, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("uploaded".to_string(), Value::Bool(true)),
            ("fileName".to_string(), Value::String(file_name)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, file_name, file_size, file_type, enabled, creator, create_time FROM x_ai_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("fileName".to_string(), Value::String(row.get("file_name"))),
                ("fileSize".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("file_size")))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_id_download(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, file_name, file_size, file_type FROM x_ai_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("url".to_string(), Value::String(format!("/download/{}", row.get::<_, String>("id")))),
                ("fileSize".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("file_size")))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_id_download_scale(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, file_name FROM x_ai_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(_) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id.clone())),
                ("url".to_string(), Value::String(format!("/download/{}?scale=0.5", id))),
                ("scale".to_string(), Value::Number(serde_json::Number::from_f64(0.5).unwrap())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_index_cms_doc_with_app_appId(
    pool: Option<Extension<Pool>>,
    Path(app_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, doc_id, app_id, title, enabled, creator FROM x_ai_index WHERE app_id = $1 ORDER BY create_time DESC",
            &[&app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
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
pub async fn stub_ai_assemble_control_index_cms_doc_docId(
    pool: Option<Extension<Pool>>,
    Path(doc_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, doc_id, title, content, app_id, creator FROM x_ai_index WHERE doc_id = $1",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("document not found"))),
    }
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_index_delete_flag(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "DELETE FROM x_ai_index WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("index not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_index_list_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let offset = (page - 1) * size;
    let rows = client
        .query(
            "SELECT id, doc_id, app_id, title, enabled, creator, create_time FROM x_ai_index ORDER BY create_time DESC LIMIT $2 OFFSET $1",
            &[&offset, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("docId".to_string(), Value::String(row.get("doc_id"))),
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
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
pub async fn stub_ai_assemble_control_index_sync_to_knowledge(
    pool: Option<Extension<Pool>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let doc_id = req.get("docId").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let result = client
        .execute(
            "UPDATE x_ai_index SET synced = true, update_time = NOW() WHERE doc_id = $1",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}
