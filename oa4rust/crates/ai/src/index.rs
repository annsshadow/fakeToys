use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[axum::debug_handler]
pub async fn index_cms_doc(
    pool: Extension<Pool>,
    Path(doc_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT xid FROM x_cms_document WHERE xid = $1",
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
            "SELECT xid FROM x_cms_document WHERE xappId = $1 AND xdocStatus = 'publish'",
            &[&app_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let doc_ids: Vec<String> = rows.iter().map(|row| row.get("xid")).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(doc_ids.len() as i64))),
            ("ids".to_string(), Value::Array(doc_ids.into_iter().map(Value::String).collect())),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn index_delete(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute("DELETE FROM x_ai_index WHERE id = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag)),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}
