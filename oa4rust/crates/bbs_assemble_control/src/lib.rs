use axum::{
    extract::{Extension, Path},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

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

#[axum::debug_handler]
pub async fn get_control_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(true)),
        ("maxForumCount".to_string(), Value::Number(serde_json::Number::from(1000i64))),
        ("allowAnonymous".to_string(), Value::Bool(false)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_sections(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let sections = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("general".to_string())),
            ("name".to_string(), Value::String("General".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("moderation".to_string())),
            ("name".to_string(), Value::String("Moderation".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Array(sections))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating bbs assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
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

pub async fn permission_subjectPublishable_sectionId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn reply_filter_list_page_page_count_count(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page - 1) * count;

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
    Path(_section_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("publishable".to_string(), Value::Bool(true)),
            ("replyPublishable".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_subject_subjectId(
    pool: Extension<Pool>,
    Path(_subject_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("editable".to_string(), Value::Bool(true)),
            ("deletable".to_string(), Value::Bool(true)),
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

