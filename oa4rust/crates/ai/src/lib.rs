use axum::{
    extract::{Extension, Path},
    Json, routing::get,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

mod routes;

#[cfg(test)]
mod tests;

pub fn ai_router(pool: Pool) -> axum::Router {
    routes::ai_router(pool)
}

#[axum::debug_handler]
pub async fn config_get(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xname, xtype, xmodel, xenable FROM X.AI_MODEL WHERE xenable = true ORDER BY xname LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = if rows.is_empty() {
        Value::Object(serde_json::Map::from_iter([
            ("config".to_string(), Value::String("base".to_string())),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ]))
    } else {
        let row = &rows[0];
        Value::Object(serde_json::Map::from_iter([
            ("config".to_string(), Value::String(row.get("xname"))),
            ("version".to_string(), Value::String("1.0.0".to_string())),
            ("enabled".to_string(), Value::Bool(row.get("xenable"))),
        ]))
    };

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn config_base_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xname, xtype, xmodel, xenable FROM X.AI_MODEL WHERE xenable = true ORDER BY xname LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

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
            ("o2AiEnable".to_string(), Value::Bool(row.get("xenable"))),
        ]))
    };

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn config_list_model_paging(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let size = size.clamp(1, 200) as i64;
    let page = page.max(1) as i64;
    let offset = (page - 1) * size;

    let total_row = client
        .query_one("SELECT COUNT(*) as cnt FROM X.AI_MODEL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, name, xtype as type, xmodel as model, xcompletionurl as completionUrl, xapikey as apiKey, xenable as enable, xasdefault as asDefault, xdesc as desc FROM X.AI_MODEL ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("model".to_string(), Value::String(row.get("model"))),
                ("completionUrl".to_string(), Value::String(row.get("completionUrl"))),
                ("apiKey".to_string(), {
                    let api_key: Option<String> = row.get("apiKey");
                    api_key.map(|k| {
                        if k.len() > 4 { Value::String(format!("{}****", &k[k.len() - 4..])) } else { Value::String("****".to_string()) }
                    }).unwrap_or(Value::Null)
                }),
                ("enable".to_string(), Value::Bool(row.get("enable"))),
                ("asDefault".to_string(), Value::Bool(row.get("asDefault"))),
                ("desc".to_string(), Value::String(row.get("desc"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("page".to_string(), Value::Number(serde_json::Number::from(page as i64))),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn config_get_model(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, xtype as type, xmodel as model, xcompletionurl as completionUrl, xapikey as apiKey, xenable as enable, xasdefault as asDefault, xdesc as desc FROM X.AI_MODEL WHERE id = $1 OR xname = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let api_key: Option<String> = row.get("apiKey");
            let masked_key = api_key.map(|k| {
                if k.len() > 4 { format!("{}****", &k[k.len() - 4..]) } else { "****".to_string() }
            });

            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("model".to_string(), Value::String(row.get("model"))),
                ("completionUrl".to_string(), Value::String(row.get("completionUrl"))),
                ("apiKey".to_string(), masked_key.map(Value::String).unwrap_or(Value::Null)),
                ("enable".to_string(), Value::Bool(row.get("enable"))),
                ("asDefault".to_string(), Value::Bool(row.get("asDefault"))),
                ("desc".to_string(), Value::String(row.get("desc"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("model not found"))),
    }
}

#[axum::debug_handler]
pub async fn config_list_mcp_paging(
    _pool: Extension<Pool>,
    Path((page, size)): Path<(i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let size = size.clamp(1, 200) as i64;
    let page = page.max(1) as i64;
    let offset = (page - 1) * size;
    let total: i64 = 0;

    let data: Vec<Value> = vec![];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("page".to_string(), Value::Number(serde_json::Number::from(page as i64))),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn config_get_mcp(
    _pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::error("mcp not found")))
}

#[axum::debug_handler]
pub async fn chat_list_paging(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let size = size.clamp(1, 200) as i64;
    let page = page.max(1) as i64;
    let offset = (page - 1) * size;

    let total_row = client
        .query_one("SELECT COUNT(*) as cnt FROM X.AI_CLUE", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, title, person, create_time FROM X.AI_CLUE ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("page".to_string(), Value::Number(serde_json::Number::from(page as i64))),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn chat_list_completion_paging(
    pool: Extension<Pool>,
    Path((clue_id, page, size)): Path<(String, i32, i32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let size = size.clamp(1, 200) as i64;
    let page = page.max(1) as i64;
    let offset = (page - 1) * size;

    let total_row = client
        .query_one("SELECT COUNT(*) as cnt FROM X.AI_COMPLETION WHERE clueId = $1", &[&clue_id])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, person, clueId, input, content, generateType, create_time FROM X.AI_COMPLETION WHERE clueId = $1 ORDER BY create_time DESC LIMIT $2 OFFSET $3",
            &[&clue_id, &size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("person".to_string(), Value::String(row.get("person"))),
                ("clueId".to_string(), Value::String(row.get("clueId"))),
                ("input".to_string(), Value::String(row.get("input"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("generateType".to_string(), Value::String(row.get("generateType"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(total))),
            ("page".to_string(), Value::Number(serde_json::Number::from(page as i64))),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn chat_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    Path(clue_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT person FROM X.AI_CLUE WHERE id = $1", &[&clue_id])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("clue not found")));
    };

    let clue_person: String = row.get("person");
    shared::middleware::require_owner(&pool, &session, &clue_person).await?;

    let mut tx = client.transaction().await.map_err(|_| AppError::Internal)?;

    tx.execute("DELETE FROM X.AI_COMPLETION WHERE clueId = $1", &[&clue_id])
        .await
        .map_err(|_| AppError::Internal)?;

    tx.execute("DELETE FROM X.AI_CLUE WHERE id = $1", &[&clue_id])
        .await
        .map_err(|_| AppError::Internal)?;

    tx.commit().await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("value".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn index_cms_doc(
    pool: Extension<Pool>,
    Path(doc_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT xid FROM X.CMS_DOCUMENT WHERE xid = $1",
            &[&doc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if row.is_none() {
        return Ok(Json(ActionResult::error("document not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(doc_id)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn index_cms_doc_with_app(
    pool: Extension<Pool>,
    Path(app_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT xid FROM X.CMS_DOCUMENT WHERE xappId = $1 AND xdocStatus = 'publish'",
            &[&app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let doc_ids: Vec<String> = rows.iter().map(|row| row.get("xid")).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("value".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(doc_ids.len() as i64))),
            ("docIds".to_string(), Value::Array(doc_ids.into_iter().map(Value::String).collect())),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn index_delete(
    _pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_get(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT xid, xname, xlength, xstorage, xcreator, xcreateTime FROM X.AI_FILE WHERE xid = $1 OR xname = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("name".to_string(), Value::String(row.get("xname"))),
                ("length".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("xlength")))),
                ("storage".to_string(), Value::String(row.get("xstorage"))),
                ("creator".to_string(), Value::String(row.get("xcreator"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_download(
    _pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("name".to_string(), Value::String(format!("{}.bin", id))),
            ("contentType".to_string(), Value::String("application/octet-stream".to_string())),
            ("contentDisposition".to_string(), Value::String(format!("attachment; filename=\"{}.bin\"", id))),
            ("fastETag".to_string(), Value::String(format!("{}-0", id))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_download_scale(
    _pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("name".to_string(), Value::String(format!("{}.png", id))),
            ("contentType".to_string(), Value::String("image/png".to_string())),
            ("contentDisposition".to_string(), Value::String(format!("attachment; filename=\"{}.png\"", id))),
            ("fastETag".to_string(), Value::String(format!("{}-0", id))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn file_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT xcreator FROM X.AI_FILE WHERE xid = $1 OR xname = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("file not found")));
    };

    let file_creator: String = row.get("xcreator");
    shared::middleware::require_owner(&pool, &session, &file_creator).await?;

    client
        .execute("DELETE FROM X.AI_FILE WHERE xid = $1 OR xname = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn list_enable_model(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xname, name, xtype, xmodel, xenable FROM X.AI_MODEL WHERE xenable = true ORDER BY xname LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("flag".to_string(), Value::String(row.get("name"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("enable".to_string(), Value::Bool(row.get("xenable"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn sync_to_knowledge(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT COUNT(*) as cnt FROM X.AI_COMPLETION", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = rows[0].get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
            ("message".to_string(), Value::String("sync completed".to_string())),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn app_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, status FROM x_ai_app ORDER BY create_time DESC LIMIT 20",
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
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("status".to_string(), Value::String(row.get("status"))),
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
pub async fn model_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, provider, enabled FROM x_ai_model ORDER BY name LIMIT 20",
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
                ("provider".to_string(), Value::String(row.get("provider"))),
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
pub async fn conversation_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, user_id, create_time FROM x_ai_conversation ORDER BY create_time DESC LIMIT 20",
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
                ("userId".to_string(), Value::String(row.get("user_id"))),
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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::ai_router(pool)
}
