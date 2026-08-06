use axum::{
    extract::{Extension, Json, Path},
    routing::{get, post, delete},
    Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ForumInfo {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SectionInfo {
    pub id: String,
    pub name: String,
    pub forum_id: String,
    pub sort: i32,
    pub description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SubjectInfo {
    pub id: String,
    pub title: String,
    pub author_id: String,
    pub section_id: String,
    pub reply_count: i32,
    pub view_count: i32,
    pub is_top: bool,
}

pub async fn forum_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description FROM bbs_forum_info ORDER BY create_time LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
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

pub async fn section_list(
    pool: Extension<Pool>,
    axum::extract::Path(forum_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, forum_id, sort, description FROM bbs_section_info WHERE forum_id = $1 ORDER BY sort LIMIT 50",
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
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("sort"))),
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

pub async fn subject_top_list(
    pool: Extension<Pool>,
    axum::extract::Path(section_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, author_id, section_id, reply_count, view_count, is_top \
             FROM bbs_subject_info \
             WHERE section_id = $1 AND is_top = true AND disable = false \
             ORDER BY create_time DESC LIMIT 20",
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

pub async fn subject_list(
    pool: Extension<Pool>,
    axum::extract::Path(section_id): axum::extract::Path<String>,
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

#[axum::debug_handler]
pub async fn create_forum(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());

    client
        .execute(
            "INSERT INTO bbs_forum_info (id, name, description, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &name, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

#[axum::debug_handler]
pub async fn update_forum(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT name, description FROM bbs_forum_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("name"));
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()).or_else(|| row.get::<_, Option<String>>("description")).unwrap_or_default();

    client
        .execute(
            "UPDATE bbs_forum_info SET name = $1, description = $2 WHERE id = $3",
            &[&name, &description, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn delete_forum(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute("DELETE FROM bbs_forum_info WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("forum not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn create_section(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let forum_id = payload.get("forumId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let sort = payload.get("sort").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());

    client
        .execute(
            "INSERT INTO bbs_section_info (id, name, forum_id, sort, description) VALUES ($1, $2, $3, $4, $5)",
            &[&id, &name, &forum_id, &sort, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("forumId".to_string(), Value::String(forum_id)),
    ])))))
}

#[axum::debug_handler]
pub async fn update_section(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT name, forum_id, sort, description FROM bbs_section_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let name = payload.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("name"));
    let forum_id = payload.get("forumId").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("forum_id"));
    let sort = payload.get("sort").and_then(|v| v.as_i64()).map(|i| i as i32).unwrap_or_else(|| row.get::<_, i32>("sort"));
    let description = payload.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()).or_else(|| row.get::<_, Option<String>>("description")).unwrap_or_default();

    client
        .execute(
            "UPDATE bbs_section_info SET name = $1, forum_id = $2, sort = $3, description = $4 WHERE id = $5",
            &[&name, &forum_id, &sort, &description, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn delete_section(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute("DELETE FROM bbs_section_info WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("section not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn create_subject(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let section_id = payload.get("sectionId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let author_id = payload.get("authorId").and_then(|v| v.as_str()).unwrap_or_default().to_string();

    client
        .execute(
            "INSERT INTO bbs_subject_info (id, title, author_id, section_id, reply_count, view_count, is_top, disable, create_time) VALUES ($1, $2, $3, $4, 0, 0, false, false, NOW())",
            &[&id, &title, &author_id, &section_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("sectionId".to_string(), Value::String(section_id)),
    ])))))
}

#[axum::debug_handler]
pub async fn update_subject(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT title, section_id, is_top, disable FROM bbs_subject_info WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let title = payload.get("title").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("title"));
    let section_id = payload.get("sectionId").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| row.get("section_id"));
    let is_top = payload.get("isTop").and_then(|v| v.as_bool()).unwrap_or_else(|| row.get("is_top"));
    let disable = payload.get("disable").and_then(|v| v.as_bool()).unwrap_or_else(|| row.get("disable"));

    client
        .execute(
            "UPDATE bbs_subject_info SET title = $1, section_id = $2, is_top = $3, disable = $4 WHERE id = $5",
            &[&title, &section_id, &is_top, &disable, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn delete_subject(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute("DELETE FROM bbs_subject_info WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("subject not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

#[axum::debug_handler]
pub async fn create_reply(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let topic_id = payload.get("topicId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let content = payload.get("content").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let creator = payload.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    client
        .execute(
            "INSERT INTO bbs_subject_reply (id, topic_id, content, creator, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &topic_id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("topicId".to_string(), Value::String(topic_id)),
    ])))))
}

#[axum::debug_handler]
pub async fn search_subjects(
    pool: Extension<Pool>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let keyword = params.get("keyword").map(|s| s.as_str()).unwrap_or_default();
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

pub fn bbs_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/core/entity/forum/list", get(forum_list))
        .route("/jaxrs/bbs/core/entity/forum", post(create_forum))
        .route("/jaxrs/bbs/core/entity/forum/{id}", post(update_forum))
        .route("/jaxrs/bbs/core/entity/forum/{id}", delete(delete_forum))
        .route("/jaxrs/bbs/core/entity/section/list/{forumId}", get(section_list))
        .route("/jaxrs/bbs/core/entity/section", post(create_section))
        .route("/jaxrs/bbs/core/entity/section/{id}", post(update_section))
        .route("/jaxrs/bbs/core/entity/section/{id}", delete(delete_section))
        .route("/jaxrs/bbs/core/entity/subject/top/{sectionId}", get(subject_top_list))
        .route("/jaxrs/bbs/core/entity/subject/list/{sectionId}", get(subject_list))
        .route("/jaxrs/bbs/core/entity/subject", post(create_subject))
        .route("/jaxrs/bbs/core/entity/subject/{id}", post(update_subject))
        .route("/jaxrs/bbs/core/entity/subject/{id}", delete(delete_subject))
        .route("/jaxrs/bbs/core/entity/reply", post(create_reply))
        .route("/jaxrs/bbs/core/entity/subject/search", get(search_subjects))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/bbs_core_entity/health", axum::routing::get(|| async { "TODO: bbs_core_entity - real implementation needed" }))
}