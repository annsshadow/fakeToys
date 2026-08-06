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
pub async fn stub_ai_assemble_control_config_base_config() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String("default-config".to_string())),
        ("name".to_string(), Value::String("Default Config".to_string())),
        ("defaultModel".to_string(), Value::String("gpt-4".to_string())),
        ("temperature".to_string(), Value::Number(serde_json::Number::from_f64(0.7).unwrap())),
        ("maxTokens".to_string(), Value::Number(serde_json::Number::from(4096i64))),
        ("enabled".to_string(), Value::Bool(true)),
        ("creator".to_string(), Value::String("system".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
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
pub async fn stub_ai_assemble_control_config_create_model() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(uuid::Uuid::new_v4().to_string())),
        ("name".to_string(), Value::String("New Model".to_string())),
        ("url".to_string(), Value::String("".to_string())),
        ("enabled".to_string(), Value::Bool(true)),
        ("creator".to_string(), Value::String("system".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
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
pub async fn stub_ai_assemble_control_config_delete_model_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_get_mcp_ext_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String("ext-mcp-1".to_string())),
        ("name".to_string(), Value::String("Extended MCP".to_string())),
        ("url".to_string(), Value::String("http://ext-mcp.example.com".to_string())),
        ("enabled".to_string(), Value::Bool(true)),
        ("creator".to_string(), Value::String("system".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
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
pub async fn stub_ai_assemble_control_config_get_model_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String("model-1".to_string())),
        ("name".to_string(), Value::String("GPT-4".to_string())),
        ("url".to_string(), Value::String("".to_string())),
        ("enabled".to_string(), Value::Bool(true)),
        ("creator".to_string(), Value::String("system".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_config_list_enable_model() -> Result<Json<ActionResult<Value>>, AppError> {
    let models = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("gpt-4".to_string())),
            ("name".to_string(), Value::String("GPT-4".to_string())),
            ("url".to_string(), Value::String("".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("creator".to_string(), Value::String("system".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("gpt-3.5-turbo".to_string())),
            ("name".to_string(), Value::String("GPT-3.5 Turbo".to_string())),
            ("url".to_string(), Value::String("".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("creator".to_string(), Value::String("system".to_string())),
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
pub async fn stub_ai_assemble_control_config_list_model_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    let models = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("gpt-4".to_string())),
            ("name".to_string(), Value::String("GPT-4".to_string())),
            ("url".to_string(), Value::String("".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("creator".to_string(), Value::String("system".to_string())),
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
pub async fn stub_ai_assemble_control_config_save() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
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
pub async fn stub_ai_assemble_control_config_update_model_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_copy_file() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("copied".to_string(), Value::Bool(true)),
            ("newId".to_string(), Value::String(uuid::Uuid::new_v4().to_string())),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_delete_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_list() -> Result<Json<ActionResult<Value>>, AppError> {
    let files = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("file-1".to_string())),
            ("name".to_string(), Value::String("document.pdf".to_string())),
            ("fileName".to_string(), Value::String("document.pdf".to_string())),
            ("fileSize".to_string(), Value::Number(serde_json::Number::from(1024i64))),
            ("fileType".to_string(), Value::String("application/pdf".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("creator".to_string(), Value::String("system".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(files.len() as i64))),
            ("data".to_string(), Value::Array(files)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    let files = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("file-1".to_string())),
            ("name".to_string(), Value::String("document.pdf".to_string())),
            ("fileName".to_string(), Value::String("document.pdf".to_string())),
            ("fileSize".to_string(), Value::Number(serde_json::Number::from(1024i64))),
            ("fileType".to_string(), Value::String("application/pdf".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(files.len() as i64))),
            ("data".to_string(), Value::Array(files)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_upload() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(uuid::Uuid::new_v4().to_string())),
            ("uploaded".to_string(), Value::Bool(true)),
            ("fileName".to_string(), Value::String("uploaded_file.pdf".to_string())),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String("file-1".to_string())),
        ("name".to_string(), Value::String("document.pdf".to_string())),
        ("fileName".to_string(), Value::String("document.pdf".to_string())),
        ("fileSize".to_string(), Value::Number(serde_json::Number::from(1024i64))),
        ("fileType".to_string(), Value::String("application/pdf".to_string())),
        ("enabled".to_string(), Value::Bool(true)),
        ("creator".to_string(), Value::String("system".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String("file-1".to_string())),
        ("name".to_string(), Value::String("document.pdf".to_string())),
        ("url".to_string(), Value::String("/download/file-1".to_string())),
        ("fileSize".to_string(), Value::Number(serde_json::Number::from(1024i64))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_file_id_download_scale() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String("file-1".to_string())),
        ("url".to_string(), Value::String("/download/file-1?scale=0.5".to_string())),
        ("scale".to_string(), Value::Number(serde_json::Number::from_f64(0.5).unwrap())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_index_cms_doc_with_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    let docs = vec![
        Value::Object(serde_json::Map::from_iter([
            ("docId".to_string(), Value::String("doc-1".to_string())),
            ("title".to_string(), Value::String("CMS Document 1".to_string())),
            ("appId".to_string(), Value::String("app-1".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(docs.len() as i64))),
            ("data".to_string(), Value::Array(docs)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_index_cms_doc_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("docId".to_string(), Value::String("doc-1".to_string())),
        ("title".to_string(), Value::String("CMS Document 1".to_string())),
        ("content".to_string(), Value::String("Document content".to_string())),
        ("appId".to_string(), Value::String("app-1".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_index_delete_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_index_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    let indices = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("index-1".to_string())),
            ("docId".to_string(), Value::String("doc-1".to_string())),
            ("appId".to_string(), Value::String("app-1".to_string())),
            ("title".to_string(), Value::String("Document Title".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(indices.len() as i64))),
            ("data".to_string(), Value::Array(indices)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn stub_ai_assemble_control_index_sync_to_knowledge() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(1i64))),
        ]),
    ))))
}
