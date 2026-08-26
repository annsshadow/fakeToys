use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

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
        .query_one("SELECT COUNT(*) as cnt FROM x_ai_clue", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, title, person, create_time FROM x_ai_clue ORDER BY create_time DESC LIMIT $1::bigint OFFSET $2::bigint",
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

    Ok(Json(ActionResult::java_success(Value::Array(data), total, size)))
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
        .query_one("SELECT COUNT(*) as cnt FROM x_ai_completion WHERE \"clueId\" = $1", &[&clue_id])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, person, \"clueId\", input, content, \"generateType\", create_time FROM x_ai_completion WHERE \"clueId\" = $1 ORDER BY create_time DESC LIMIT $2::bigint OFFSET $3::bigint",
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
                ("\"clueId\"".to_string(), Value::String(row.get("\"clueId\""))),
                ("input".to_string(), Value::String(row.get("input"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("\"generateType\"".to_string(), Value::String(row.get("\"generateType\""))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::java_success(Value::Array(data), total, size)))
}

#[axum::debug_handler]
pub async fn chat_delete(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    Path(clue_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT person FROM x_ai_clue WHERE id = $1", &[&clue_id])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("clue not found")));
    };

    let clue_person: String = row.get("person");
    shared::middleware::require_owner(&pool, &session, &clue_person).await?;

    let mut tx = client.transaction().await.map_err(|_| AppError::Internal)?;

    tx.execute("DELETE FROM x_ai_completion WHERE \"clueId\" = $1", &[&clue_id])
        .await
        .map_err(|_| AppError::Internal)?;

    let result = tx
        .execute("DELETE FROM x_ai_clue WHERE id = $1", &[&clue_id])
        .await
        .map_err(|_| AppError::Internal)?;

    tx.commit().await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}
