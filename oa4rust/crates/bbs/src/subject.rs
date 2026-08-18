use axum::extract::{Extension, Path, Query};
use axum::Json;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct SubjectSearchQuery {
    pub keyword: Option<String>,
}

#[utoipa::path(
    get,
    path = "/jaxrs/bbs/subject/top/{sectionId}",
    params(
        ("sectionId" = String, Path, description = "Section ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "bbs"
)]
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

#[utoipa::path(
    get,
    path = "/jaxrs/bbs/subject/list/{sectionId}",
    params(
        ("sectionId" = String, Path, description = "Section ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "bbs"
)]
pub async fn list(
    pool: Extension<Pool>,
    Path(section_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, title, author_id, section_id, reply_count, view_count, is_top, create_time \
             FROM bbs_subject_info \
             WHERE section_id = $1 AND disable = false \
             ORDER BY create_time DESC LIMIT 50",
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
                ("sectionId".to_string(), Value::String(row.get("section_id"))),
                (
                    "replyCount".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("reply_count"))),
                ),
                (
                    "viewCount".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("view_count"))),
                ),
                ("isTop".to_string(), Value::Bool(row.get("is_top"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
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

#[utoipa::path(
    get,
    path = "/jaxrs/bbs/subject/view/{id}",
    params(
        ("id" = String, Path, description = "Subject ID")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "bbs"
)]
pub async fn view(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, author_id, section_id, content, reply_count, view_count, is_top, create_time \
             FROM bbs_subject_info \
             WHERE id = $1 AND disable = false",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("authorId".to_string(), Value::String(row.get("author_id"))),
                ("sectionId".to_string(), Value::String(row.get("section_id"))),
                ("content".to_string(), Value::String(row.get("content"))),
                (
                    "replyCount".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("reply_count"))),
                ),
                (
                    "viewCount".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("view_count"))),
                ),
                ("isTop".to_string(), Value::Bool(row.get("is_top"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
                ),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("subject not found"))),
    }
}

#[utoipa::path(
    post,
    path = "/jaxrs/bbs/subject/create",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "bbs"
)]
pub async fn create(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let section_id = payload.get("sectionId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let author_id = payload.get("authorId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let content = payload.get("content").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    client
        .execute(
            "INSERT INTO bbs_subject_info (id, title, author_id, section_id, content, reply_count, view_count, is_top, disable, create_time) VALUES ($1, $2, $3, $4, $5, 0, 0, false, false, NOW())",
            &[&id, &title, &author_id, &section_id, &content],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("sectionId".to_string(), Value::String(section_id)),
    ])))))
}

#[utoipa::path(
    get,
    path = "/jaxrs/bbs/subject/search",
    params(
        ("keyword" = Option<String>, Query, description = "Search keyword")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "bbs"
)]
pub async fn search(
    pool: Extension<Pool>,
    Query(params): Query<SubjectSearchQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let keyword = params.keyword.unwrap_or_default();

    let pattern = format!("%{}%", keyword);
    let rows = client
        .query(
            "SELECT id, title, author_id, section_id, reply_count, view_count, is_top, create_time \
             FROM bbs_subject_info \
             WHERE title ILIKE $1 AND disable = false \
             ORDER BY create_time DESC LIMIT 20",
            &[&pattern],
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
                ("sectionId".to_string(), Value::String(row.get("section_id"))),
                (
                    "replyCount".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("reply_count"))),
                ),
                (
                    "viewCount".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("view_count"))),
                ),
                ("isTop".to_string(), Value::Bool(row.get("is_top"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
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
