use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[axum::debug_handler]
pub async fn config_get(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xname, xtype, xmodel, xenable FROM x_ai_model WHERE xenable = true ORDER BY xname LIMIT 1",
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
            "SELECT xname, xtype, xmodel, xenable FROM x_ai_model WHERE xenable = true ORDER BY xname LIMIT 1",
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
        .query_one("SELECT COUNT(*) as cnt FROM x_ai_model", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, name, xtype as type, xmodel as model, xcompletionurl as \"completionUrl\", xapikey as \"apiKey\", xenable as enable, xasdefault as \"asDefault\", xdesc as desc FROM x_ai_model ORDER BY create_time DESC LIMIT $1::bigint OFFSET $2::bigint",
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
                ("\"completionUrl\"".to_string(), Value::String(row.get("\"completionUrl\""))),
                ("\"apiKey\"".to_string(), {
                    let api_key: Option<String> = row.get("\"apiKey\"");
                    api_key.map(|k| {
                        if k.len() > 4 { Value::String(format!("{}****", &k[k.len() - 4..])) } else { Value::String("****".to_string()) }
                    }).unwrap_or(Value::Null)
                }),
                ("enable".to_string(), Value::Bool(row.get("enable"))),
                ("\"asDefault\"".to_string(), Value::Bool(row.get("\"asDefault\""))),
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
            "SELECT id, name, xtype as type, xmodel as model, xcompletionurl as \"completionUrl\", xapikey as \"apiKey\", xenable as enable, xasdefault as \"asDefault\", xdesc as desc FROM x_ai_model WHERE id = $1 OR xname = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let api_key: Option<String> = row.get("\"apiKey\"");
            let masked_key = api_key.map(|k| {
                if k.len() > 4 { format!("{}****", &k[k.len() - 4..]) } else { "****".to_string() }
            });

            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("model".to_string(), Value::String(row.get("model"))),
                ("\"completionUrl\"".to_string(), Value::String(row.get("\"completionUrl\""))),
                ("\"apiKey\"".to_string(), masked_key.map(Value::String).unwrap_or(Value::Null)),
                ("enable".to_string(), Value::Bool(row.get("enable"))),
                ("\"asDefault\"".to_string(), Value::Bool(row.get("\"asDefault\""))),
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
pub async fn list_enable_model(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xname, name, xtype, xmodel, xenable FROM x_ai_model WHERE xenable = true ORDER BY xname LIMIT 20",
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
