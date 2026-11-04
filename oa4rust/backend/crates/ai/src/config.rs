// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn config_get(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT xname, xtype, xmodel, xenable FROM x_ai_model WHERE xenable = true ORDER BY xname LIMIT 1",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data = if rows.is_empty() {
        Value::Object(serde_json::Map::from_iter([
            ("config".to_string(), Value::String("base".to_string())),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            ("enabled".to_string(), Value::Bool(false)),
            (
                "appIconUrl".to_string(),
                Value::String(std::env::var("APP_ICON_URL").unwrap_or_default()),
            ),
            (
                "o2AiToken".to_string(),
                Value::String(std::env::var("O2_AI_TOKEN").unwrap_or_default()),
            ),
            (
                "aliApiUrl".to_string(),
                Value::String(std::env::var("ALI_API_URL").unwrap_or_default()),
            ),
            (
                "deepSeekApiUrl".to_string(),
                Value::String(std::env::var("DEEPSEEK_API_URL").unwrap_or_default()),
            ),
            ("o2AiFileList".to_string(), Value::Array(vec![])),
            (
                "o2AiBaseUrl".to_string(),
                Value::String(std::env::var("O2_AI_BASE_URL").unwrap_or_default()),
            ),
            (
                "appName".to_string(),
                Value::String(std::env::var("APP_NAME").unwrap_or_else(|_| "O2OA".to_string())),
            ),
            ("o2AiEnable".to_string(), Value::Bool(false)),
            ("title".to_string(), Value::String(String::new())),
            ("desc".to_string(), Value::String(String::new())),
        ]))
    } else {
        let row = &rows[0];
        Value::Object(serde_json::Map::from_iter([
            (
                "config".to_string(),
                Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default()),
            ),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            (
                "enabled".to_string(),
                Value::Bool(row.get::<_, Option<bool>>("xenable").unwrap_or(false)),
            ),
            (
                "appIconUrl".to_string(),
                Value::String(std::env::var("APP_ICON_URL").unwrap_or_default()),
            ),
            (
                "o2AiToken".to_string(),
                Value::String(std::env::var("O2_AI_TOKEN").unwrap_or_default()),
            ),
            (
                "aliApiUrl".to_string(),
                Value::String(std::env::var("ALI_API_URL").unwrap_or_default()),
            ),
            (
                "deepSeekApiUrl".to_string(),
                Value::String(std::env::var("DEEPSEEK_API_URL").unwrap_or_default()),
            ),
            ("o2AiFileList".to_string(), Value::Array(vec![])),
            (
                "o2AiBaseUrl".to_string(),
                Value::String(std::env::var("O2_AI_BASE_URL").unwrap_or_default()),
            ),
            (
                "appName".to_string(),
                Value::String(std::env::var("APP_NAME").unwrap_or_else(|_| "O2OA".to_string())),
            ),
            (
                "o2AiEnable".to_string(),
                Value::Bool(row.get::<_, Option<bool>>("xenable").unwrap_or(false)),
            ),
            (
                "title".to_string(),
                Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default()),
            ),
            ("desc".to_string(), Value::String(String::new())),
        ]))
    };

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn config_base_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT xname, xtype, xmodel, xenable FROM x_ai_model WHERE xenable = true ORDER BY xname LIMIT 1",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data = if rows.is_empty() {
        Value::Object(serde_json::Map::from_iter([
            ("appName".to_string(), Value::String("O2OA".to_string())),
            ("appIconUrl".to_string(), Value::String("".to_string())),
            ("title".to_string(), Value::String("".to_string())),
            ("desc".to_string(), Value::String("".to_string())),
            ("o2AiEnable".to_string(), Value::Bool(false)),
        ]))
    } else {
        let row = &rows[0];
        Value::Object(serde_json::Map::from_iter([
            ("appName".to_string(), Value::String("O2OA".to_string())),
            ("appIconUrl".to_string(), Value::String("".to_string())),
            ("title".to_string(), Value::String("".to_string())),
            ("desc".to_string(), Value::String("".to_string())),
            (
                "o2AiEnable".to_string(),
                Value::Bool(row.get::<_, Option<bool>>("xenable").unwrap_or(false)),
            ),
        ]))
    };

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn config_list_model_paging(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;

    let (size, offset) = shared::response::page_window(page as i64, size as i64)?;

    let total_row = client
        .query_one("SELECT COUNT(*) as cnt FROM x_ai_model", &[])
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, name, xtype as type, xmodel as model, xcompletionurl as \"completionUrl\", xapikey as \"apiKey\", xenable as enable, (xasdefault = 'true') as \"asDefault\", xdesc as desc FROM x_ai_model ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&size, &offset],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let api_key_val: Option<Value> = {
                let api_key: Option<String> = row.get("apiKey");
                api_key.map(|k| {
                    if k.len() > 4 {
                        Value::String(format!("{}****", &k[k.len() - 4..]))
                    } else {
                        Value::String("****".to_string())
                    }
                })
            };
            Value::Object(serde_json::Map::from_iter(
                [
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    (
                        "type".to_string(),
                        Value::String(row.get::<_, Option<String>>("type").unwrap_or_default()),
                    ),
                    (
                        "model".to_string(),
                        Value::String(row.get::<_, Option<String>>("model").unwrap_or_default()),
                    ),
                    (
                        "completionUrl".to_string(),
                        Value::String(row.get("completionUrl")),
                    ),
                    (
                        "enable".to_string(),
                        Value::Bool(row.get::<_, Option<bool>>("enable").unwrap_or(false)),
                    ),
                    ("asDefault".to_string(), Value::Bool(row.get("asDefault"))),
                    (
                        "desc".to_string(),
                        Value::String(row.get::<_, Option<String>>("desc").unwrap_or_default()),
                    ),
                ]
                .into_iter()
                .chain(api_key_val.into_iter().map(|v| ("apiKey".to_string(), v))),
            ))
        })
        .collect();

    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        total,
        size,
    )))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn config_get_model(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;

    let row = client
        .query_opt(
            "SELECT id, name, xtype as type, xmodel as model, xcompletionurl as \"completionUrl\", xapikey as \"apiKey\", xenable as enable, (xasdefault = 'true') as \"asDefault\", xdesc as desc FROM x_ai_model WHERE id = $1 OR xname = $1",
            &[&flag],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => {
            let api_key: Option<String> = row.get("apiKey");
            let masked_key: Option<Value> = api_key
                .map(|k| {
                    if k.len() > 4 {
                        format!("{}****", &k[k.len() - 4..])
                    } else {
                        "****".to_string()
                    }
                })
                .map(Value::String);

            let result = Value::Object(serde_json::Map::from_iter(
                [
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    (
                        "type".to_string(),
                        Value::String(row.get::<_, Option<String>>("type").unwrap_or_default()),
                    ),
                    (
                        "model".to_string(),
                        Value::String(row.get::<_, Option<String>>("model").unwrap_or_default()),
                    ),
                    (
                        "completionUrl".to_string(),
                        Value::String(row.get("completionUrl")),
                    ),
                    (
                        "enable".to_string(),
                        Value::Bool(row.get::<_, Option<bool>>("enable").unwrap_or(false)),
                    ),
                    ("asDefault".to_string(), Value::Bool(row.get("asDefault"))),
                    (
                        "desc".to_string(),
                        Value::String(row.get::<_, Option<String>>("desc").unwrap_or_default()),
                    ),
                ]
                .into_iter()
                .chain(masked_key.into_iter().map(|v| ("apiKey".to_string(), v))),
            ));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("model not found"))),
    }
}

// O20：原 config_list_mcp_paging / config_get_mcp 是本 crate 的空桩
// （恒返回空数组 / "mcp not found"），旧词路由 /api/ai/config/list|get/mcp
// 因此长期返回假数据。旧词路由已改挂 ai_assemble_control 的真实现，
// 本文件不再保留桩函数。

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn list_enable_model(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT xname, name, xtype, xmodel, xenable FROM x_ai_model WHERE xenable = true ORDER BY xname LIMIT 20",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("flag".to_string(), Value::String(row.get("name"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "enable".to_string(),
                    Value::Bool(row.get::<_, Option<bool>>("xenable").unwrap_or(false)),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}
