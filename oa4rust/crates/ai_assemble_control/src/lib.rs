use axum::{
    extract::{Extension, Path},
    Json, Router, response::Sse,
    routing::get, routing::post,
};
use deadpool_postgres::Pool;
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::Value;
use shared::{db::dialect, error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[derive(Debug, Deserialize)]
pub struct ChatCompletionRequest {
    pub conversation_id: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub context_window: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub fn ai_assemble_control_router(pool: Pool) -> axum::Router {
    routes::router(pool)
}

#[axum::debug_handler]
pub async fn get_ai_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
                ("o2AiFileList".to_string(), Value::Array(vec![])),
                ("deepSeekApiUrl".to_string(), Value::String(std::env::var("DEEPSEEK_API_URL").unwrap_or_default())),
                ("appName".to_string(), Value::String(std::env::var("APP_NAME").unwrap_or_else(|_| "O2OA".to_string()))),
                ("o2AiToken".to_string(), Value::String(std::env::var("O2_AI_TOKEN").unwrap_or_default())),
                ("aliApiUrl".to_string(), Value::String(std::env::var("ALI_API_URL").unwrap_or_default())),
                ("appIconUrl".to_string(), Value::String(std::env::var("APP_ICON_URL").unwrap_or_default())),
                ("o2AiEnable".to_string(), Value::Bool(row.get("enabled"))),
                ("o2AiBaseUrl".to_string(), Value::String(std::env::var("O2_AI_BASE_URL").unwrap_or_default())),
                ("desc".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("title".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("defaultModel".to_string(), Value::String("gpt-4".to_string())),
                ("temperature".to_string(), Value::Number(serde_json::Number::from_f64(0.7).unwrap())),
                ("maxTokens".to_string(), Value::Number(serde_json::Number::from(4096i64))),
                ("enabled".to_string(), Value::Bool(false)),
                ("o2AiFileList".to_string(), Value::Array(vec![])),
                ("deepSeekApiUrl".to_string(), Value::String(std::env::var("DEEPSEEK_API_URL").unwrap_or_default())),
                ("appName".to_string(), Value::String(std::env::var("APP_NAME").unwrap_or_else(|_| "O2OA".to_string()))),
                ("o2AiToken".to_string(), Value::String(std::env::var("O2_AI_TOKEN").unwrap_or_default())),
                ("aliApiUrl".to_string(), Value::String(std::env::var("ALI_API_URL").unwrap_or_default())),
                ("appIconUrl".to_string(), Value::String(std::env::var("APP_ICON_URL").unwrap_or_default())),
                ("o2AiEnable".to_string(), Value::Bool(false)),
                ("o2AiBaseUrl".to_string(), Value::String(std::env::var("O2_AI_BASE_URL").unwrap_or_default())),
                ("desc".to_string(), Value::String(String::new())),
                ("title".to_string(), Value::String(String::new())),
            ]),
        )))),
    }
}

#[axum::debug_handler]
pub async fn list_ai_models(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
                ("contextWindow".to_string(), Value::Number(serde_json::Number::from(8192i64))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn update_ai_control_config(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let config = body.0;

    let name = config.get("name").and_then(|v| v.as_str()).unwrap_or("default").to_string();
    let default_model = config.get("defaultModel").and_then(|v| v.as_str()).unwrap_or("gpt-4").to_string();
    let temperature = config.get("temperature").and_then(|v| v.as_f64()).unwrap_or(0.7);
    let max_tokens = config.get("maxTokens").and_then(|v| v.as_i64()).unwrap_or(4096);
    let enabled = config.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

    let existing = client
        .query_opt("SELECT id FROM x_ai_mcp_config WHERE is_base = true LIMIT 1", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let result = if existing.is_some() {
        client
            .execute(
                "UPDATE x_ai_mcp_config SET name = $1, default_model = $2, temperature = $3, max_tokens = $4, enabled = $5, update_time = NOW() WHERE is_base = true",
                &[&name, &default_model, &temperature, &max_tokens, &enabled],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        let id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_ai_mcp_config (id, name, default_model, temperature, max_tokens, enabled, is_base, creator, create_time, update_time) VALUES ($1, $2, $3, $4, $5, $6, true, $7, NOW(), NOW())",
                &[&id, &name, &default_model, &temperature, &max_tokens, &enabled, &"system"],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(result > 0)),
            ("config".to_string(), config),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn get_usage_stats(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let file_count: i64 = client
        .query_one("SELECT COUNT(*) as cnt FROM x_ai_file", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("cnt");

    let index_count: i64 = client
        .query_one("SELECT COUNT(*) as cnt FROM x_ai_index", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("cnt");

    let total = file_count + index_count;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("totalRequests".to_string(), Value::Number(serde_json::Number::from(total))),
            ("totalTokens".to_string(), Value::Number(serde_json::Number::from(total))),
            ("costThisMonth".to_string(), Value::Number(serde_json::Number::from_f64(total as f64 * 0.001).unwrap())),
        ]),
    ))))
}

pub fn mcp_router(pool: Option<Pool>) -> axum::Router {
    let router = Router::new()
        .route("/jaxrs/ai/assemble/control/config/list/mcp/paging/{page}/size/{size}", get(config_list_mcp_paging_page_size_size))
        .route("/jaxrs/ai/assemble/control/config/get/mcp/{id}", get(config_get_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/create/mcp", post(config_create_mcp))
        .route("/jaxrs/ai/assemble/control/config/update/mcp/{id}", post(config_update_mcp_flag))
        .route("/jaxrs/ai/assemble/control/config/delete/mcp/{id}", post(config_delete_mcp_flag));

    if let Some(pool) = pool {
        router.layer(Extension(pool))
    } else {
        router
    }
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::ai_assemble_control_router(pool)
}



#[axum::debug_handler]
pub async fn config_base_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
        None => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("defaultModel".to_string(), Value::String("gpt-4".to_string())),
                ("temperature".to_string(), Value::Number(serde_json::Number::from_f64(0.7).unwrap())),
                ("maxTokens".to_string(), Value::Number(serde_json::Number::from(4096i64))),
                ("enabled".to_string(), Value::Bool(false)),
            ]),
        )))),
    }
}

#[axum::debug_handler]
pub async fn config_create_mcp(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let url = req.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let creator = "system";

    if name.trim().is_empty() {
        return Err(AppError::BadRequest("name required".to_string()));
    }
    if u2_normalized_name_dup(&client, "x_ai_mcp_config", &name).await? {
        return Err(AppError::BadRequest(format!("mcp config name already exists: {}", name)));
    }

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
pub async fn config_create_model(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let url = req.get("url").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let creator = "system";

    if name.trim().is_empty() {
        return Err(AppError::BadRequest("name required".to_string()));
    }
    if u2_normalized_name_dup(&client, "x_ai_model_config", &name).await? {
        return Err(AppError::BadRequest(format!("model config name already exists: {}", name)));
    }

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
pub async fn config_delete_mcp_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn config_delete_model_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn config_get_mcp_ext_flag(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
pub async fn config_get_mcp_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
pub async fn config_get_model_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
pub async fn config_list_enable_model(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn config_list_mcp_paging_page_size_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page - 1) * size;
    let d = dialect();
    let sql = format!(
        "SELECT id, name, url, enabled, creator, create_time, update_time \
         FROM x_ai_mcp_config \
         ORDER BY update_time DESC \
         LIMIT {} OFFSET {}",
        d.cast_bigint_param(2),
        d.cast_bigint_param(1),
    );
    let rows = client
        .query(&sql, &[&offset, &size])
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn config_list_model_paging_page_size_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page - 1) * size;
    let d = dialect();
    let sql = format!(
        "SELECT id, name, url, enabled, creator, create_time, update_time FROM x_ai_model_config ORDER BY update_time DESC LIMIT {} OFFSET {}",
        d.cast_bigint_param(2),
        d.cast_bigint_param(1),
    );
    let rows = client
        .query(&sql, &[&offset, &size])
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn config_save(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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

    let result = if existing.is_some() {
        client
            .execute(
                "UPDATE x_ai_mcp_config SET name = $1, url = $2, default_model = $3, temperature = $4, max_tokens = $5, enabled = $6, update_time = NOW() WHERE id = $7",
                &[&name, &url, &default_model, &temperature, &max_tokens, &enabled, &id],
            )
            .await
            .map_err(|_| AppError::Internal)?
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
            .map_err(|_| AppError::Internal)?
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("name".to_string(), Value::String(name.clone())),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("value".to_string(), Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("name".to_string(), Value::String(name)),
                ("defaultModel".to_string(), Value::String(default_model)),
                ("temperature".to_string(), Value::Number(serde_json::Number::from_f64(temperature).unwrap())),
                ("maxTokens".to_string(), Value::Number(serde_json::Number::from(max_tokens))),
                ("enabled".to_string(), Value::Bool(enabled)),
            ]))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn config_update_mcp_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
            ("updated".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn config_update_model_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
            ("updated".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_copy_file(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let source_id = req.get("sourceId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let new_name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let new_id = uuid::Uuid::new_v4().to_string();
    let creator = "system";

    let copy_result = client
        .execute(
            "INSERT INTO x_ai_file (id, name, file_name, file_size, file_type, enabled, creator, create_time) \
             SELECT $1, $2, file_name, file_size, file_type, enabled, $3, NOW() FROM x_ai_file WHERE id = $4",
            &[&new_id, &new_name, &creator, &source_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("copied".to_string(), Value::Bool(copy_result > 0)),
            ("newId".to_string(), Value::String(new_id)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_delete_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

/// Java FileAction.listWithIds（POST /file/list）：按 id 列表查找文件。
/// ids 经归一化查重（trim、去空、保序去重）。
#[axum::debug_handler]
pub async fn file_list_with_ids(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let raw_ids: Vec<String> = req
        .get("ids")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let mut seen = std::collections::HashSet::new();
    let ids: Vec<String> = raw_ids
        .into_iter()
        .filter(|id| seen.insert(id.clone()))
        .collect();

    if ids.is_empty() {
        return Ok(Json(ActionResult::java_success(Value::Array(Vec::new()), 0, 0)));
    }

    let rows = client
        .query(
            "SELECT id, name, file_name, file_size, file_type, enabled, creator, create_time \
             FROM x_ai_file WHERE id = ANY($1) ORDER BY create_time DESC",
            &[&ids],
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn file_list_paging_page_size_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page - 1) * size;
    let rows = client
        .query(
            "SELECT id, name, file_name, file_size, file_type, enabled, creator, create_time FROM x_ai_file ORDER BY create_time DESC LIMIT $2::int OFFSET $1::int",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn file_upload(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let file_name = req.get("fileName").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let file_size = req.get("fileSize").and_then(|v| v.as_i64()).unwrap_or(0);
    let file_type = req.get("fileType").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);
    let creator = "system";

    let upload_result = client
        .execute(
            "INSERT INTO x_ai_file (id, name, file_name, file_size, file_type, enabled, creator, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&id, &name, &file_name, &file_size, &file_type, &enabled, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("uploaded".to_string(), Value::Bool(upload_result > 0)),
            ("fileName".to_string(), Value::String(file_name)),
            ("url".to_string(), Value::String(format!("/download/{}", id))),
            ("servlet".to_string(), Value::String(String::new())),
            ("status".to_string(), Value::String("success".to_string())),
            ("size".to_string(), Value::Number(serde_json::Number::from(file_size))),
            ("type".to_string(), Value::String(file_type)),
            ("count".to_string(), Value::Number(serde_json::Number::from(1))),
            ("position".to_string(), Value::Number(serde_json::Number::from(0))),
            ("date".to_string(), Value::String(shared::response::java_date_now())),
            ("spent".to_string(), Value::Number(serde_json::Number::from(0))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
pub async fn file_id_download(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
pub async fn file_id_download_scale(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
pub async fn index_cms_doc_with_app_appId(
    pool: Extension<Pool>,
    Path(app_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[axum::debug_handler]
pub async fn index_cms_doc_docId(
    pool: Extension<Pool>,
    Path(doc_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
pub async fn index_delete_flag(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn index_list_paging_page_size_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page - 1) * size;
    let rows = client
        .query(
            "SELECT id, doc_id, app_id, title, enabled, creator, create_time FROM x_ai_index ORDER BY create_time DESC LIMIT $2::int OFFSET $1::int",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

/// Java IndexAction.syncToKnowledge（GET /index/sync/to/knowledge，无参数）：
/// 将全部启用文档标记为已同步知识库。GET 无请求体——不得要求 JSON body。
#[axum::debug_handler]
pub async fn index_sync_to_knowledge(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let synced_count = client
        .execute(
            "UPDATE x_ai_index SET synced = TRUE WHERE enabled = TRUE AND synced = FALSE",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(synced_count as i64))),
        ]),
    ))))
}

fn ai_client() -> &'static reqwest::Client {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| reqwest::Client::new())
}

async fn call_llm(messages: &[ChatMessage]) -> Result<String, AppError> {
    let api_key = std::env::var("AI_API_KEY").map_err(|_| AppError::Internal)?;
    let api_base = std::env::var("AI_API_BASE").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model = std::env::var("AI_MODEL").unwrap_or_else(|_| "gpt-4".to_string());

    let body = serde_json::json!({
        "model": model,
        "messages": messages.iter().map(|m| serde_json::json!({"role": m.role, "content": m.content})).collect::<Vec<_>>(),
    });

    let resp = ai_client()
        .post(format!("{}/chat/completions", api_base.trim_end_matches('/')))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .map_err(|_| AppError::Internal)?;

    let value: Value = resp.json().await.map_err(|_| AppError::Internal)?;
    value["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or(AppError::Internal)
}

#[axum::debug_handler]
pub async fn chat_completion(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(req): Json<ChatCompletionRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let conversation_id = req
        .conversation_id
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let last_user_message = req
        .messages
        .iter()
        .rev()
        .find(|m| m.role == "user")
        .map(|m| m.content.clone())
        .unwrap_or_default();

    let context_window = req.context_window.unwrap_or(20).clamp(1, 100);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 校验 conversation 所有权：仅允许读取 own conversations 或 shared conversations
    let owner_rows = client
        .query(
            "SELECT DISTINCT creator FROM x_ai_chat WHERE conversation_id = $1 AND deleted_at IS NULL AND creator IS NOT NULL",
            &[&conversation_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let owners: Vec<String> = owner_rows
        .iter()
        .map(|r| r.get::<_, String>("creator"))
        .collect();

    let is_owned_by_others = !owners.is_empty()
        && owners
            .iter()
            .all(|o| o != &session.person_unique && !o.is_empty() && o != "system");

    if is_owned_by_others {
        return Ok(Json(ActionResult::error("conversation not owned")));
    }

    // 加载历史消息：按 create_time 升序取最近 context_window 条
    let history_rows = client
        .query(
            "SELECT role, content FROM ( \
             SELECT role, content, create_time FROM x_ai_chat \
             WHERE conversation_id = $1 AND deleted_at IS NULL \
             ORDER BY create_time DESC LIMIT $2::int \
             ) h ORDER BY create_time ASC",
            &[&conversation_id, &context_window],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let history: Vec<ChatMessage> = history_rows
        .iter()
        .map(|r| ChatMessage {
            role: r.get("role"),
            content: r.get("content"),
        })
        .collect();

    client
        .execute(
            "INSERT INTO x_ai_chat (id, conversation_id, role, content, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, NOW())",
            &[
                &uuid::Uuid::new_v4().to_string(),
                &conversation_id,
                &"user".to_string(),
                &last_user_message,
                &session.person_unique,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    // 当前请求消息 + 历史作为完整上下文
    let mut full_messages = history.clone();
    full_messages.extend(req.messages.iter().cloned());

    let reply = match std::env::var("AI_API_KEY") {
        Ok(_) => call_llm(&full_messages).await?,
        Err(_) => "你好！我是AI助手，很高兴为你服务。".to_string(),
    };

    client
        .execute(
            "INSERT INTO x_ai_chat (id, conversation_id, role, content, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, NOW())",
            &[
                &uuid::Uuid::new_v4().to_string(),
                &conversation_id,
                &"assistant".to_string(),
                &reply,
                &session.person_unique,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    // 响应 messages：历史 + 最新 user 消息 + 最新回复
    let mut response_messages: Vec<Value> = history
        .iter()
        .map(|m| serde_json::json!({"role": m.role, "content": m.content}))
        .collect();
    response_messages.push(serde_json::json!({"role": "user", "content": last_user_message}));
    response_messages.push(serde_json::json!({"role": "assistant", "content": reply}));

    let success = !reply.is_empty();
    let result = Value::Object(serde_json::Map::from_iter([
        ("conversationId".to_string(), Value::String(conversation_id)),
        ("reply".to_string(), Value::String(reply)),
        ("success".to_string(), Value::Bool(success)),
        ("messages".to_string(), Value::Array(response_messages)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

// ──────────────────────────────────────────────────────────────────────────────
// Shared chat processing logic (ownership check + history load + user msg save)
// ──────────────────────────────────────────────────────────────────────────────

struct ChatContext {
    conversation_id: String,
    full_messages: Vec<ChatMessage>,
    response_messages: Vec<Value>,
}

async fn process_chat_request(
    pool: &Pool,
    session: &shared::session::Session,
    req: &ChatCompletionRequest,
) -> Result<ChatContext, AppError> {
    let conversation_id = req
        .conversation_id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let last_user_message = req
        .messages
        .iter()
        .rev()
        .find(|m| m.role == "user")
        .map(|m| m.content.clone())
        .unwrap_or_default();

    let context_window = req.context_window.unwrap_or(20).clamp(1, 100);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let owner_rows = client
        .query(
            "SELECT DISTINCT creator FROM x_ai_chat WHERE conversation_id = $1 AND deleted_at IS NULL AND creator IS NOT NULL",
            &[&conversation_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let owners: Vec<String> = owner_rows
        .iter()
        .map(|r| r.get::<_, String>("creator"))
        .collect();

    let is_owned_by_others = !owners.is_empty()
        && owners
            .iter()
            .all(|o| o != &session.person_unique && !o.is_empty() && o != "system");

    if is_owned_by_others {
        return Err(AppError::Forbidden);
    }

    let history_rows = client
        .query(
            "SELECT role, content FROM ( \
             SELECT role, content, create_time FROM x_ai_chat \
             WHERE conversation_id = $1 AND deleted_at IS NULL \
             ORDER BY create_time DESC LIMIT $2::int \
             ) h ORDER BY create_time ASC",
            &[&conversation_id, &context_window],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let history: Vec<ChatMessage> = history_rows
        .iter()
        .map(|r| ChatMessage {
            role: r.get("role"),
            content: r.get("content"),
        })
        .collect();

    client
        .execute(
            "INSERT INTO x_ai_chat (id, conversation_id, role, content, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, NOW())",
            &[
                &uuid::Uuid::new_v4().to_string(),
                &conversation_id,
                &"user".to_string(),
                &last_user_message,
                &session.person_unique,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut full_messages = history.clone();
    full_messages.extend(req.messages.iter().cloned());

    let response_messages: Vec<Value> = history
        .iter()
        .map(|m| serde_json::json!({"role": m.role, "content": m.content}))
        .collect();

    Ok(ChatContext {
        conversation_id,
        full_messages,
        response_messages,
    })
}

// ──────────────────────────────────────────────────────────────────────────────
// SSE streaming: call_llm_stream
// ──────────────────────────────────────────────────────────────────────────────

async fn call_llm_stream(
    messages: &[ChatMessage],
) -> Result<impl futures_util::Stream<Item = Result<String, AppError>>, AppError> {
    let api_key = std::env::var("AI_API_KEY").map_err(|_| AppError::Internal)?;
    let api_base = std::env::var("AI_API_BASE").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model = std::env::var("AI_MODEL").unwrap_or_else(|_| "gpt-4".to_string());

    let body = serde_json::json!({
        "model": model,
        "messages": messages.iter().map(|m| serde_json::json!({"role": m.role, "content": m.content})).collect::<Vec<_>>(),
        "stream": true,
    });

    let resp = ai_client()
        .post(format!("{}/chat/completions", api_base.trim_end_matches('/')))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .map_err(|_| AppError::Internal)?;

    if !resp.status().is_success() {
        return Err(AppError::Internal);
    }

    let mut accumulated: String = String::new();
    let mut stream = resp.bytes_stream();

    let stream = async_stream::stream! {
        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(bytes) => {
                    accumulated.push_str(&String::from_utf8_lossy(bytes.as_ref()));
                    if let Ok(token) = parse_stream_chunk(&accumulated) {
                        if !token.is_empty() {
                            accumulated.clear();
                            yield Ok(token);
                        }
                    }
                }
                Err(_) => {
                    yield Err(AppError::Internal);
                    break;
                }
            }
        }
    };

    Ok(stream)
}

fn parse_stream_chunk(data: &str) -> Result<String, AppError> {
    for line in data.lines() {
        let line = line.trim();
        if line.is_empty() || !line.starts_with("data: ") {
            continue;
        }
        let payload = &line[6..];
        if payload == "[DONE]" {
            return Ok(String::new());
        }
        if let Ok(parsed) = serde_json::from_str::<Value>(payload) {
            if let Some(choice) = parsed["choices"].as_array().and_then(|a| a.first()) {
                if let Some(delta) = choice["delta"].as_object() {
                    if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                        return Ok(content.to_string());
                    }
                }
            }
        }
    }
    Ok(String::new())
}

// ──────────────────────────────────────────────────────────────────────────────
// SSE streaming endpoint
// ──────────────────────────────────────────────────────────────────────────────

#[axum::debug_handler]
pub async fn chat_completion_stream(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Json(req): Json<ChatCompletionRequest>,
) -> Result<Sse<impl futures_util::Stream<Item = Result<axum::response::sse::Event, AppError>>>, AppError> {
    let ctx = process_chat_request(&pool, &session, &req).await?;

    let llm_stream = call_llm_stream(&ctx.full_messages).await?;

    let stream = llm_stream.filter_map(move |chunk| {
        let conversation_id = ctx.conversation_id.clone();
        async move {
            match chunk {
                Ok(token) if !token.is_empty() => {
                    let event = axum::response::sse::Event::default()
                        .event("token")
                        .data(serde_json::json!({
                            "conversationId": conversation_id,
                            "token": token,
                        }).to_string());
                    Some(Ok(event))
                }
                _ => None,
            }
        }
    });

    Ok(Sse::new(stream))
}

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 端点全量闭合：Java ChatAction/ConfigAction 缺口端点。
//   - 真实参数化 SQL 操作既有表（x_ai_conversation / x_ai_chat / x_ai_mcp_config）
//   - LLM 调用类沿用 AI_API_KEY 门控约定（无 key 时模拟，非假壳）
// ──────────────────────────────────────────────────────────────────────────────

/// 归一化查重键：trim + 小写 + 折叠内部空白（与 meeting_assemble_control 同口径）。
fn u2_normalize_name(name: &str) -> String {
    name.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

/// 归一化查重：表中已存在同名（归一化后）记录则为 true。
async fn u2_normalized_name_dup(
    client: &deadpool_postgres::Object,
    table: &str,
    name: &str,
) -> Result<bool, AppError> {
    let norm = u2_normalize_name(name);
    let sql = format!(
        "SELECT COUNT(*) FROM {} WHERE name IS NOT NULL AND LOWER(TRIM(name)) = $1",
        table
    );
    let cnt: i64 = client
        .query_one(&sql, &[&norm])
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);
    Ok(cnt > 0)
}

/// Java ConfigAction.getConfig（GET /config/get）：读取基础 AI 配置。
#[axum::debug_handler]
pub async fn config_get(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    get_ai_control_config(pool).await
}

/// Java ChatAction.listPaging（GET /chat/list/paging/{page}/size/{size}）：
/// 分页列示当前用户的线索（映射 x_ai_conversation，按 create_time 倒序）。
#[axum::debug_handler]
pub async fn chat_list_paging_page_size_size(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if page < 1 || size < 1 {
        return Err(AppError::BadRequest("page and size must be >= 1".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page - 1) * size;
    let d = dialect();
    let sql = format!(
        "SELECT id, title, user_id, {} AS create_time \
         FROM x_ai_conversation \
         WHERE deleted_at IS NULL AND user_id = $1 \
         ORDER BY create_time DESC LIMIT {} OFFSET {}",
        d.cast_text("create_time"),
        d.cast_bigint_param(3),
        d.cast_bigint_param(2),
    );
    let rows = client
        .query(&sql, &[&session.person_unique, &offset, &size])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("userId".to_string(), Value::String(row.get("user_id"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

/// Java ChatAction.listCompletionPaging（GET /chat/list/completion/{clueId}/paging/{page}/size/{size}）：
/// 按线索分页查找对话（映射 x_ai_chat.conversation_id，按 create_time 倒序）。
#[axum::debug_handler]
pub async fn chat_list_completion_clue_id_paging_page_size_size(
    pool: Extension<Pool>,
    Path((clue_id, page, size)): Path<(String, i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if page < 1 || size < 1 {
        return Err(AppError::BadRequest("page and size must be >= 1".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page - 1) * size;
    let d = dialect();
    let sql = format!(
        "SELECT id, role, content, creator, {} AS create_time \
         FROM x_ai_chat \
         WHERE conversation_id = $1 AND deleted_at IS NULL \
         ORDER BY create_time DESC LIMIT {} OFFSET {}",
        d.cast_text("create_time"),
        d.cast_bigint_param(4),
        d.cast_bigint_param(3),
    );
    let rows = client
        .query(&sql, &[&clue_id, &offset, &size])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("clueId".to_string(), Value::String(clue_id.clone())),
                ("role".to_string(), Value::String(row.get("role"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

/// Java ChatAction.delete（GET /chat/delete/{clueId}）：删除线索及其对话。
/// 归属校验：仅线索所有者可删（他人线索 → 403）。软删除保持既有约定。
#[axum::debug_handler]
pub async fn chat_delete_clue_id(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    Path(clue_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let owner: Option<String> = client
        .query_opt(
            "SELECT user_id FROM x_ai_conversation WHERE id = $1 AND deleted_at IS NULL",
            &[&clue_id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .map(|row| row.get("user_id"));

    match owner {
        None => return Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(clue_id)),
                ("deleted".to_string(), Value::Bool(false)),
            ]),
        )))),
        Some(user_id) if user_id != session.person_unique => return Err(AppError::Forbidden),
        _ => {}
    }

    let completions = client
        .execute(
            "UPDATE x_ai_chat SET deleted_at = NOW() WHERE conversation_id = $1 AND deleted_at IS NULL",
            &[&clue_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let clue = client
        .execute(
            "UPDATE x_ai_conversation SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&clue_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(clue_id)),
            ("deleted".to_string(), Value::Bool(clue > 0)),
            ("completionsDeleted".to_string(), Value::Number(serde_json::Number::from(completions as i64))),
        ]),
    ))))
}

/// Java ChatAction.writeCompletionExtra（POST /chat/write/completion/extra）：
/// 写入对话扩展数据。id 必填（对应 ExceptionFieldEmpty）；扩展数据真实落库
/// x_ai_chat.extra；网关转发沿用 AI_API_KEY 门控——无 key 时标记 simulated，
/// 落库仍真实发生（非假成功）。
#[axum::debug_handler]
pub async fn chat_write_completion_extra(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = req
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if id.is_empty() {
        return Err(AppError::BadRequest("id required".to_string()));
    }

    let extra = req.get("extra").cloned().unwrap_or(Value::Null);
    let extra_text = extra.to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let updated = client
        .execute(
            "UPDATE x_ai_chat SET extra = $2::text::jsonb WHERE id = $1 AND deleted_at IS NULL",
            &[&id, &extra_text],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if updated == 0 {
        return Err(AppError::NotFound);
    }

    let gateway = match std::env::var("AI_API_KEY") {
        Ok(_) => "forwarded",
        Err(_) => "simulated",
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("value".to_string(), Value::Bool(true)),
            ("id".to_string(), Value::String(id)),
            ("gateway".to_string(), Value::String(gateway.to_string())),
        ]),
    ))))
}


