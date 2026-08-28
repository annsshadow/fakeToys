use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[utoipa::path(
    get,
    path = "/jaxrs/bbs/forum/view/all",
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
            "SELECT id, name, description FROM bbs_forum_info ORDER BY create_time",
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
            if let Some(v) = row.get::<_, Option<String>>("description") {
                map.insert("description".to_string(), Value::String(v));
            }
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[utoipa::path(
    get,
    path = "/jaxrs/bbs/forum/view/{id}",
    params(
        ("id" = String, Path, description = "Forum ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "bbs"
)]
pub async fn view_one(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, description FROM bbs_forum_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            if let Some(v) = row.get::<_, Option<String>>("description") {
                map.insert("description".to_string(), Value::String(v));
            }
            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Ok(Json(ActionResult::error("forum not found"))),
    }
}
