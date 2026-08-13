use axum::{
    extract::{Extension, Path},
    Json, Router,
};
use bcrypt::verify;
use chrono::Utc;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


thread_local! {
    static TOKEN_STORE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub credential: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTopicRequest {
    pub forum_id: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub creator: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReplyRequest {
    pub topic_id: Option<String>,
    pub content: Option<String>,
    pub creator: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ForumRow {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sort: i32,
    pub creator: String,
    pub create_time: String,
}

#[derive(Debug, Serialize)]
pub struct TopicRow {
    pub id: String,
    pub forum_id: String,
    pub title: String,
    pub content: String,
    pub creator: String,
    pub create_time: String,
}

#[derive(Debug, Serialize)]
pub struct ReplyRow {
    pub id: String,
    pub topic_id: String,
    pub content: String,
    pub creator: String,
    pub create_time: String,
}

fn row_to_forum(row: &deadpool_postgres::tokio_postgres::Row) -> ForumRow {
    ForumRow {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        sort: row.get("sort"),
        creator: row.get("creator"),
        create_time: row.get("create_time"),
    }
}

fn row_to_topic(row: &deadpool_postgres::tokio_postgres::Row) -> TopicRow {
    TopicRow {
        id: row.get("id"),
        forum_id: row.get("forum_id"),
        title: row.get("title"),
        content: row.get("content"),
        creator: row.get("creator"),
        create_time: row.get("create_time"),
    }
}

fn row_to_reply(row: &deadpool_postgres::tokio_postgres::Row) -> ReplyRow {
    ReplyRow {
        id: row.get("id"),
        topic_id: row.get("topic_id"),
        content: row.get("content"),
        creator: row.get("creator"),
        create_time: row.get("create_time"),
    }
}

fn verify_person_password(plain: &str, stored: &str) -> bool {
    const BCRYPT_PREFIX: &str = "{bcrypt}";
    if let Some(bcrypt_hash) = stored.strip_prefix(BCRYPT_PREFIX) {
        return verify(plain, bcrypt_hash).unwrap_or(false);
    }
    let md5_hash = format!("{:x}", md5::compute(plain.as_bytes()));
    md5_hash == stored
}

fn now_iso() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT enabled, max_forum_count, allow_anonymous FROM x_bbs_assemble_control_config LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(row.get("enabled"))),
        ("maxForumCount".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("max_forum_count")))),
        ("allowAnonymous".to_string(), Value::Bool(row.get("allow_anonymous"))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_sections(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, enabled FROM x_bbs_assemble_control_section ORDER BY sort",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let sections: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Array(sections))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled = body.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    let max_forum_count = body.get("maxForumCount").and_then(|v| v.as_i64()).unwrap_or(1000);
    let allow_anonymous = body.get("allowAnonymous").and_then(|v| v.as_bool()).unwrap_or(false);

    client
        .execute(
            "UPDATE x_bbs_assemble_control_config SET enabled = $1, max_forum_count = $2, allow_anonymous = $3 WHERE id = (SELECT id FROM x_bbs_assemble_control_config ORDER BY create_time LIMIT 1)",
            &[&enabled, &max_forum_count, &allow_anonymous],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), body.0),
        ]),
    ))))
}

/// GET /jaxrs/bbs/assemble/control/forum/list
pub async fn list_forums(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, sort, creator, create_time FROM x_bbs_forum ORDER BY sort ASC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(row_to_forum)
        .map(|f| serde_json::to_value(f).unwrap())
        .collect();

    Ok(Json(ActionResult::success(Value::Array(data))))
}

/// GET /jaxrs/bbs/assemble/control/forum/{id}
pub async fn get_forum(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, description, sort, creator, create_time FROM x_bbs_forum WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(serde_json::to_value(row_to_forum(&row)).unwrap()))),
        None => Ok(Json(ActionResult::error("forum not found"))),
    }
}

/// POST /jaxrs/bbs/assemble/control/topic/create
pub async fn create_topic(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateTopicRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let forum_id = req.forum_id.unwrap_or_default();
    let title = req.title.unwrap_or_default();
    let content = req.content.unwrap_or_default();
    let creator = req.creator.unwrap_or_else(|| "system".to_string());

    client
        .execute(
            "INSERT INTO x_bbs_topic (id, forum_id, title, content, creator, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &forum_id, &title, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("forumId".to_string(), Value::String(forum_id)),
        ("title".to_string(), Value::String(title)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// GET /jaxrs/bbs/assemble/control/topic/list/{forumId}
pub async fn list_topics_by_forum(
    pool: Extension<Pool>,
    axum::extract::Path(forum_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic WHERE forum_id = $1 ORDER BY create_time DESC",
            &[&forum_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();

    Ok(Json(ActionResult::success(Value::Array(data))))
}

/// POST /jaxrs/bbs/assemble/control/reply/create
pub async fn create_reply(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateReplyRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let topic_id = req.topic_id.unwrap_or_default();
    let content = req.content.unwrap_or_default();
    let creator = req.creator.unwrap_or_else(|| "system".to_string());

    client
        .execute(
            "INSERT INTO x_bbs_reply (id, topic_id, content, creator, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &topic_id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("topicId".to_string(), Value::String(topic_id)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub fn bbs_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::bbs_assemble_control_router(pool)
}



pub async fn forum_view_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, sort, creator, create_time FROM x_bbs_forum ORDER BY sort ASC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(row_to_forum)
        .map(|f| serde_json::to_value(f).unwrap())
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn forum_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, description, sort, creator, create_time FROM x_bbs_forum WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(serde_json::to_value(row_to_forum(&row)).unwrap()))),
        None => Ok(Json(ActionResult::error("forum not found"))),
    }
}

pub async fn mobile_view_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn permission_replyPublishable_subjectId(
    pool: Extension<Pool>,
    Path(subject_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id FROM x_bbs_topic WHERE id = $1 AND deleted_at IS NULL",
            &[&subject_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let publishable = row.is_some();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("replyPublishable".to_string(), Value::Bool(publishable)),
        ]),
    ))))
}

pub async fn permission_subjectPublishable_sectionId(
    pool: Extension<Pool>,
    Path(section_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let section_exists = client
        .query_opt("SELECT 1 FROM x_bbs_section WHERE id = $1 AND deleted_at IS NULL", &[&section_id])
        .await
        .map_err(|_| AppError::Internal)
        ?.is_some();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("subjectPublishable".to_string(), Value::Bool(section_exists)),
        ]),
    ))))
}

pub async fn reply_filter_list_page_page_count_count(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page.saturating_sub(1)) * count;

    let rows = client
        .query(
            "SELECT id, topic_id, content, creator, create_time FROM x_bbs_reply ORDER BY create_time DESC LIMIT $2 OFFSET $1",
            &[&offset, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(row_to_reply)
        .map(|r| serde_json::to_value(r).unwrap())
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn reply_list_sub_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, topic_id, content, creator, create_time FROM x_bbs_reply WHERE topic_id = $1 ORDER BY create_time ASC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(row_to_reply)
        .map(|r| serde_json::to_value(r).unwrap())
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn subject_view_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(serde_json::to_value(row_to_topic(&row)).unwrap()))),
        None => Ok(Json(ActionResult::error("subject not found"))),
    }
}

pub async fn subject_top_sectionId(
    pool: Extension<Pool>,
    Path(section_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic WHERE forum_id = $1 AND is_top = true ORDER BY create_time DESC",
            &[&section_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn permission_section_sectionId(
    pool: Extension<Pool>,
    Path(section_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let section_exists = client
        .query_opt("SELECT 1 FROM x_bbs_section WHERE id = $1 AND deleted_at IS NULL", &[&section_id])
        .await
        .map_err(|_| AppError::Internal)
        ?.is_some();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("publishable".to_string(), Value::Bool(section_exists)),
            ("replyPublishable".to_string(), Value::Bool(section_exists)),
        ]),
    ))))
}

pub async fn permission_subject_subjectId(
    pool: Extension<Pool>,
    Path(subject_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let topic_exists = client
        .query_opt("SELECT 1 FROM x_bbs_topic WHERE id = $1 AND deleted_at IS NULL", &[&subject_id])
        .await
        .map_err(|_| AppError::Internal)
        ?.is_some();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("editable".to_string(), Value::Bool(topic_exists)),
            ("deletable".to_string(), Value::Bool(topic_exists)),
        ]),
    ))))
}

pub async fn section_viewforum_forumId(
    pool: Extension<Pool>,
    Path(forum_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, forum_id, sort, description FROM x_bbs_section WHERE forum_id = $1 ORDER BY sort",
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

pub async fn delete_forum(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_bbs_forum SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn delete_reply(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_bbs_reply SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn delete_subject(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_bbs_topic SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn list_reply_filter(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page.saturating_sub(1)).saturating_mul(count);
    let rows = client
        .query(
            "SELECT id, topic_id, content, creator, create_time FROM x_bbs_reply \
             WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one("SELECT COUNT(*) FROM x_bbs_reply WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get(0);
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_reply)
        .map(|r| serde_json::to_value(r).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_topics_creamed(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page.saturating_sub(1)).saturating_mul(count);
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic \
             WHERE deleted_at IS NULL AND is_cream = true ORDER BY create_time DESC \
             LIMIT $1 OFFSET $2",
            &[&count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND is_cream = true", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get(0);
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_topics_recommended(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page.saturating_sub(1)).saturating_mul(count);
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic \
             WHERE deleted_at IS NULL AND is_recommend = true ORDER BY create_time DESC \
             LIMIT $1 OFFSET $2",
            &[&count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND is_recommend = true", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get(0);
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_subjects_filtered(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page.saturating_sub(1)).saturating_mul(count);
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic \
             WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get(0);
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_subjects_index(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page.saturating_sub(1)).saturating_mul(count);
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic \
             WHERE deleted_at IS NULL AND is_top = true ORDER BY create_time DESC \
             LIMIT $1 OFFSET $2",
            &[&count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND is_top = true", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get(0);
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn list_subjects_recommended_index(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page.saturating_sub(1)).saturating_mul(count);
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic \
             WHERE deleted_at IS NULL AND is_recommend = true ORDER BY create_time DESC \
             LIMIT $1 OFFSET $2",
            &[&count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND is_recommend = true", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get(0);
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn login(
    pool: Extension<Pool>,
    body: axum::extract::Json<LoginRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let credential = body.credential.trim();
    let password = body.password.trim();
    if credential.is_empty() || password.is_empty() {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, unique_id, name, password_hash, locked FROM auth_person \
             WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&credential],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("invalid credentials")));
    };
    if row.get::<_, bool>("locked") {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }
    let password_hash: String = row.get("password_hash");
    if !verify_person_password(password, &password_hash) {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }
    let person_id: String = row.get("id");
    let person_unique: String = row.get("unique_id");
    let person_name: String = row.get("name");
    let role_rows = client
        .query(
            "SELECT r.name FROM auth_role r JOIN auth_person_role pr ON r.id = pr.role_id \
             WHERE pr.person_id = $1 AND r.deleted_at IS NULL",
            &[&person_id],
        )
        .await
        .unwrap_or_default();
    let role_list: Vec<String> = role_rows.iter().map(|r| r.get::<_, String>("name")).collect();
    let token = Uuid::new_v4().to_string();
    TOKEN_STORE.with(|m| {
        m.lock().unwrap().insert(token.clone(), person_unique.clone());
    });
    let data = Value::Object(serde_json::Map::from_iter([
        ("token".to_string(), Value::String(token)),
        ("id".to_string(), Value::String(person_id)),
        ("unique".to_string(), Value::String(person_unique)),
        ("name".to_string(), Value::String(person_name)),
        ("roleList".to_string(), Value::Array(
            role_list.iter().map(|s| Value::String(s.clone())).collect(),
        )),
    ]));
    Ok(Json(ActionResult::success(data)))
}

pub async fn logout(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if let Some(token) = body.get("token").and_then(|v| v.as_str()) {
        TOKEN_STORE.with(|m| {
            m.lock().unwrap().remove(token);
        });
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn picture_list(
    pool: Extension<Pool>,
    Path(subject_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT content FROM x_bbs_topic WHERE id = $1 AND deleted_at IS NULL", &[&subject_id])
        .await
        .map_err(|_| AppError::Internal)?;
    let urls: Vec<Value> = match row {
        Some(r) => {
            let content: String = r.get("content");
            content
                .split_whitespace()
                .filter(|s| s.starts_with("http") && (s.ends_with(".jpg") || s.ends_with(".jpeg") || s.ends_with(".png") || s.ends_with(".gif") || s.contains(".svg") || s.contains(".webp")))
                .map(|s| Value::String(s.to_string()))
                .collect()
        }
        None => Vec::new(),
    };
    Ok(Json(ActionResult::success(Value::Array(urls))))
}

pub async fn shutup_create(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person = body.get("person").and_then(|v| v.as_str()).unwrap_or("");
    let reason = body.get("reason").and_then(|v| v.as_str()).unwrap_or("");
    let id = Uuid::new_v4().to_string();
    let ct = now_iso();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let _ = client
        .execute(
            "INSERT INTO x_bbs_shutup (id, person, reason, create_time) VALUES ($1, $2, $3, $4)",
            &[&id, &person, &reason, &ct],
        )
        .await;
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id.clone())),
        ("person".to_string(), Value::String(person.to_string())),
        ("reason".to_string(), Value::String(reason.to_string())),
        ("createTime".to_string(), Value::String(ct)),
    ]));
    Ok(Json(ActionResult::success(data)))
}

pub async fn shutup_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM x_bbs_shutup WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn shutup_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page.saturating_sub(1)).saturating_mul(count);
    let rows = client
        .query(
            "SELECT id, person, reason, create_time FROM x_bbs_shutup \
             ORDER BY create_time DESC LIMIT $1 OFFSET $2",
            &[&count, &offset],
        )
        .await
        .unwrap_or_default();
    let total_row = client
        .query_one("SELECT COUNT(*) FROM x_bbs_shutup", &[])
        .await
        .unwrap_or_else(|_| panic!("shutup count query failed"));
    let total: i64 = total_row.get(0);
    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("person".to_string(), Value::String(row.get("person"))),
            ("reason".to_string(), row.get::<_, Option<String>>("reason").map(Value::String).unwrap_or(Value::Null)),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ])))
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn subject_creamed_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_topics_creamed(pool, Path((page, count))).await
}

pub async fn subject_filter_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_subjects_filtered(pool, Path((page, count))).await
}

pub async fn subject_filter_listsubjectinfo(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let forum_id = body.get("forumId").and_then(|v| v.as_str()).unwrap_or("");
    let offset_val = body.get("page").and_then(|v| v.as_i64()).unwrap_or(1);
    let limit_val = body.get("count").and_then(|v| v.as_i64()).unwrap_or(20);
    let offset = (offset_val.saturating_sub(1)).saturating_mul(limit_val);
    let sql = if forum_id.is_empty() {
        "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $1 OFFSET $2"
    } else {
        "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic WHERE deleted_at IS NULL AND forum_id = $3 ORDER BY create_time DESC LIMIT $1 OFFSET $2"
    };
    let rows = if forum_id.is_empty() {
        client.query(sql, &[&limit_val, &offset]).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(sql, &[&limit_val, &offset, &forum_id]).await.map_err(|_| AppError::Internal)?
    };
    let total_row = if forum_id.is_empty() {
        client.query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL", &[])
            .await.map_err(|_| AppError::Internal)?
    } else {
        client.query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND forum_id = $1", &[&forum_id])
            .await.map_err(|_| AppError::Internal)?
    };
    let total: i64 = total_row.get(0);
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn subject_index_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_subjects_index(pool, Path((page, count))).await
}

pub async fn subject_search(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    let _page = page;
    let _count = count;
    Ok(Json(ActionResult::success(Value::Array(vec![]))))
}

pub async fn subject_statgrade(
    pool: Extension<Pool>,
    Path((_section_name, _subject_type)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let total_row = client.query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL", &[])
        .await.map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get(0);
    let cream_row = client.query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND is_cream = true", &[])
        .await.map_err(|_| AppError::Internal)?;
    let cream: i64 = cream_row.get(0);
    let rec_row = client.query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND is_recommend = true", &[])
        .await.map_err(|_| AppError::Internal)?;
    let recommended: i64 = rec_row.get(0);
    let top_row = client.query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND is_top = true", &[])
        .await.map_err(|_| AppError::Internal)?;
    let top: i64 = top_row.get(0);
    let data = Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("cream".to_string(), Value::Number(serde_json::Number::from(cream))),
        ("recommended".to_string(), Value::Number(serde_json::Number::from(recommended))),
        ("top".to_string(), Value::Number(serde_json::Number::from(top))),
    ]));
    Ok(Json(ActionResult::success(data)))
}

pub async fn topic_creamed_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_topics_creamed(pool, Path((page, count))).await
}

pub async fn topic_filter_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_subjects_filtered(pool, Path((page, count))).await
}

pub async fn topic_filter_listsubjectinfo(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    subject_filter_listsubjectinfo(pool, body).await
}

pub async fn topic_index_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_subjects_index(pool, Path((page, count))).await
}

pub async fn topic_recommended_index(
    pool: Extension<Pool>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic \
             WHERE deleted_at IS NULL AND is_recommend = true ORDER BY create_time DESC LIMIT $1",
            &[&count],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

pub async fn topic_recommended_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_topics_recommended(pool, Path((page, count))).await
}

pub async fn topic_search(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    let _page = page;
    let _count = count;
    Ok(Json(ActionResult::success(Value::Array(vec![]))))
}

pub async fn user_forum_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, sort, creator, create_time FROM x_bbs_forum \
             WHERE deleted_at IS NULL ORDER BY sort ASC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("description".to_string(), Value::String(row.get("description"))),
            ("sort".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("sort")))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ])))
        .collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

pub async fn user_info(
    pool: Extension<Pool>,
    Path(person): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile, email, icon, job, department, unit, position \
             FROM auth_person WHERE id = $1 AND deleted_at IS NULL",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let id: String = row.get("id");
            let unique: String = row.get("unique_id");
            let name: String = row.get("name");
            let role_rows = client
                .query(
                    "SELECT r.name FROM auth_role r JOIN auth_person_role pr ON r.id = pr.role_id \
                     WHERE pr.person_id = $1 AND r.deleted_at IS NULL",
                    &[&id],
                )
                .await
                .unwrap_or_default();
            let role_list: Vec<String> = role_rows.iter().map(|r| r.get::<_, String>("name")).collect();
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("unique".to_string(), Value::String(unique)),
                ("name".to_string(), Value::String(name)),
                ("mobile".to_string(), row.get::<_, Option<String>>("mobile").map(Value::String).unwrap_or(Value::Null)),
                ("email".to_string(), row.get::<_, Option<String>>("email").map(Value::String).unwrap_or(Value::Null)),
                ("icon".to_string(), row.get::<_, Option<String>>("icon").map(Value::String).unwrap_or(Value::Null)),
                ("roleList".to_string(), Value::Array(
                    role_list.iter().map(|s| Value::String(s.clone())).collect(),
                )),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("user not found"))),
    }
}

pub async fn user_reply_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, topic_id, content, creator, create_time FROM x_bbs_reply \
             WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_reply)
        .map(|r| serde_json::to_value(r).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

pub async fn user_role_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description FROM auth_role WHERE deleted_at IS NULL ORDER BY name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("description".to_string(), row.get::<_, Option<String>>("description").map(Value::String).unwrap_or(Value::Null)),
        ])))
        .collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

pub async fn user_section_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, forum_id, sort, description FROM x_bbs_section \
             WHERE deleted_at IS NULL ORDER BY sort",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("forumId".to_string(), Value::String(row.get("forum_id"))),
            ("sort".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("sort")))),
            ("description".to_string(), row.get::<_, Option<String>>("description").map(Value::String).unwrap_or(Value::Null)),
        ])))
        .collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

pub async fn user_setting(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT settings FROM auth_person WHERE settings IS NOT NULL LIMIT 1", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let data = match row {
        Some(r) => {
            let s: String = r.get("settings");
            serde_json::from_str(&s).unwrap_or(Value::Object(serde_json::Map::new()))
        }
        None => Value::Object(serde_json::Map::new()),
    };
    Ok(Json(ActionResult::success(data)))
}

pub async fn user_subject_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, forum_id, title, content, creator, create_time FROM x_bbs_topic \
             WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(row_to_topic)
        .map(|t| serde_json::to_value(t).unwrap())
        .collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

pub async fn uuid_generate() -> Result<Json<ActionResult<Value>>, AppError> {
    let uuid = uuid::Uuid::new_v4().to_string();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("uuid".to_string(), Value::String(uuid))]),
    ))))
}

pub async fn subjectattach_list(
    pool: Extension<Pool>,
    Path(_subject_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Array(vec![]))))
}

