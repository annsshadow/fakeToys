use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[utoipa::path(
    get,
    path = "/jaxrs/bbs/section/viewforum/{forumId}",
    params(
        ("forumId" = String, Path, description = "Forum ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "bbs"
)]
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
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("forumId".to_string(), Value::String(row.get("forum_id")));
            map.insert(
                "sort".to_string(),
                Value::Number(serde_json::Number::from(row.get::<_, i32>("sort"))),
            );
            if let Some(v) = row.get::<_, Option<String>>("description") {
                map.insert("description".to_string(), Value::String(v));
            }
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

#[utoipa::path(
    get,
    path = "/jaxrs/bbs/section/view/all",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "bbs"
)]
pub async fn view_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, forum_id, sort, description FROM bbs_section_info ORDER BY sort",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("forumId".to_string(), Value::String(row.get("forum_id")));
            map.insert(
                "sort".to_string(),
                Value::Number(serde_json::Number::from(row.get::<_, i32>("sort"))),
            );
            if let Some(v) = row.get::<_, Option<String>>("description") {
                map.insert("description".to_string(), Value::String(v));
            }
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}
