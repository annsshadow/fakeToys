use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub async fn view_forum(
    pool: Extension<Pool>,
    Path(forum_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, forum_id, sort, description FROM bbs_section_info WHERE forum_id = $1 ORDER BY sort",
            &[&forum_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("forumId".to_string(), Value::String(row.get("forum_id"))),
                (
                    "sort".to_string(),
                    Value::Number(serde_json::Number::from(
                        row.get::<_, i32>("sort")
                    )),
                ),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
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
