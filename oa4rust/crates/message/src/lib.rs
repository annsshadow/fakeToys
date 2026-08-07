use axum::{
    extract::Extension,
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use shared::{error::AppError, response::ActionResult};

#[derive(Debug, Deserialize)]
pub struct ConsumeListRequest {
    pub consume: String,
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub r#type: Option<String>,
    pub consumer: Option<String>,
}

pub async fn consume_list(
    pool: Extension<Pool>,
    axum::extract::Path((consume, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xbody, xtype, xconsumer, xperson, xcreateTime FROM X.MSG_MESSAGE WHERE xconsumer = $1 AND xconsumed = false ORDER BY xcreateTime ASC LIMIT $2",
            &[&consume, &(count.min(200).max(1) as i64)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("title".to_string(), Value::String(row.get("xtitle"))),
                ("body".to_string(), Value::String(row.get("xbody"))),
                ("type".to_string(), Value::String(row.get("xtype"))),
                ("consumer".to_string(), Value::String(row.get("xconsumer"))),
                ("person".to_string(), Value::String(row.get("xperson"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
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

pub async fn update_single(
    pool: Extension<Pool>,
    axum::extract::Path((id, r#type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT xid FROM X.MSG_MESSAGE WHERE xid = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if rows.is_empty() {
        return Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("type".to_string(), Value::String(r#type)),
                ("updated".to_string(), Value::Bool(false)),
            ]),
        ))));
    }

    client
        .execute(
            "UPDATE X.MSG_MESSAGE SET xconsumed = true WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("type".to_string(), Value::String(r#type)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn custom_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateMessageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = Uuid::new_v4().to_string();
    let title = req.title.unwrap_or_default();
    let body = req.body.unwrap_or_default();
    let msg_type = req.r#type.unwrap_or_default();

    client
        .execute(
            "INSERT INTO X.MSG_MESSAGE (xid, xtitle, xbody, xtype, xconsumed) VALUES ($1, $2, $3, $4, false)",
            &[&id, &title, &body, &msg_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("id".to_string(), Value::String(id)),
            ("title".to_string(), Value::String(title)),
        ]),
    ))))
}

pub async fn mark_read(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT xid FROM X.MSG_MESSAGE WHERE xid = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if !rows.is_empty() {
        client
            .execute(
                "UPDATE X.MSG_MESSAGE SET xconsumed = true WHERE xid = $1",
                &[&id],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("markedRead".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn unread_count(
    pool: Extension<Pool>,
    axum::extract::Path(consume): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT COUNT(*) as cnt FROM X.MSG_MESSAGE WHERE xconsumer = $1 AND xconsumed = false",
            &[&consume],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = rows[0].get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
            ("consumer".to_string(), Value::String(consume)),
        ]),
    ))))
}

pub fn message_router() -> Router {
    Router::new()
        .route("/jaxrs/message/consume/list/{consume}/count/{count}", get(consume_list))
        .route("/jaxrs/message/consume/{id}/type/{type}", get(update_single))
        .route("/jaxrs/message/custom/create", post(custom_create))
        .route("/jaxrs/message/mark_read/{id}", post(mark_read))
        .route("/jaxrs/message/unread/count/{consume}", get(unread_count))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    message_router().layer(axum::extract::Extension(pool))
}

#[cfg(test)]
mod tests;
