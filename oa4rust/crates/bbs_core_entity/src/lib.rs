use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
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

pub fn bbs_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/core/entity/forum/list", get(forum_list))
        .route("/jaxrs/bbs/core/entity/section/list/{forumId}", get(section_list))
        .route("/jaxrs/bbs/core/entity/subject/top/{sectionId}", get(subject_top_list))
        .route("/jaxrs/bbs/core/entity/subject/list/{sectionId}", get(subject_list))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/bbs_core_entity/health", axum::routing::get(|| async { "TODO: bbs_core_entity - real implementation needed" }))
}