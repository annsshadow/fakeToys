use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult, session::Session};

#[axum::debug_handler]
pub async fn file_get(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT xid, xname, xlength, xstorage, xcreator, \"xcreateTime\" FROM x_ai_file WHERE xid = $1 OR xname = $1",
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn file_download(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT xid, xname FROM x_ai_file WHERE xid = $1 OR xname = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(_) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id.clone())),
                ("name".to_string(), Value::String(format!("{}.bin", id))),
                ("contentType".to_string(), Value::String("application/octet-stream".to_string())),
                ("contentDisposition".to_string(), Value::String(format!("attachment; filename=\"{}.bin\"", id))),
                ("fastETag".to_string(), Value::String(format!("{}-0", id))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("file not found"))),
    }
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
    Extension(session): Extension<Session>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT xcreator FROM x_ai_file WHERE xid = $1 OR xname = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("file not found")));
    };

    let file_creator: String = row.get("xcreator");
    shared::middleware::require_owner(&pool, &session, &file_creator).await?;

    client
        .execute("DELETE FROM x_ai_file WHERE xid = $1 OR xname = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag)),
        ]),
    ))))
}
