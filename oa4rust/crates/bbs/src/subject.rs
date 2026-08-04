use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub async fn top(
    pool: Extension<Pool>,
    Path(section_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, author_id, create_time, reply_count, view_count \
             FROM bbs_subject_info \
             WHERE section_id = $1 AND is_top = true AND disable = false \
             ORDER BY create_time DESC",
            &[&section_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("authorId".to_string(), Value::String(row.get("author_id"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
                ),
                (
                    "replyCount".to_string(),
                    Value::Number(serde_json::Number::from(
                        row.get::<_, i32>("reply_count")
                    )),
                ),
                (
                    "viewCount".to_string(),
                    Value::Number(serde_json::Number::from(
                        row.get::<_, i32>("view_count")
                    )),
                ),
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
