use axum::{
    extract::{Extension, Path},
    routing::{delete, get, post, put},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

#[derive(Debug, Serialize, Clone)]
pub struct Document {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub rank: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Subject {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub rank: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub id: String,
    pub content: Option<String>,
    pub rank: Option<f64>,
}

pub async fn search_documents(
    pool: &Pool,
    query: &str,
    limit: i32,
) -> Result<Vec<Document>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            r#"
            SELECT
                id,
                title,
                content,
                ts_rank(
                    to_tsvector('simple', COALESCE(title, '') || ' ' || COALESCE(content, '')),
                    websearch_to_tsquery('simple', $1)
                ) AS rank
            FROM x_cms_document
            WHERE to_tsvector('simple', COALESCE(title, '') || ' ' || COALESCE(content, '')) @@ websearch_to_tsquery('simple', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#,
            &[&query, &limit],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(rows
        .iter()
        .map(|row| Document {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            rank: row.get("rank"),
        })
        .collect())
}

pub async fn search_subjects(
    pool: &Pool,
    query: &str,
    limit: i32,
) -> Result<Vec<Subject>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            r#"
            SELECT
                id,
                title,
                content,
                ts_rank(
                    to_tsvector('simple', COALESCE(title, '') || ' ' || COALESCE(content, '')),
                    websearch_to_tsquery('simple', $1)
                ) AS rank
            FROM bbs_subject_info
            WHERE to_tsvector('simple', COALESCE(title, '') || ' ' || COALESCE(content, '')) @@ websearch_to_tsquery('simple', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#,
            &[&query, &limit],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(rows
        .iter()
        .map(|row| Subject {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            rank: row.get("rank"),
        })
        .collect())
}

pub async fn search_messages(
    pool: &Pool,
    query: &str,
    limit: i32,
) -> Result<Vec<Message>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            r#"
            SELECT
                id,
                content,
                ts_rank(
                    to_tsvector('simple', COALESCE(content, '')),
                    websearch_to_tsquery('simple', $1)
                ) AS rank
            FROM x_message
            WHERE to_tsvector('simple', COALESCE(content, '')) @@ websearch_to_tsquery('simple', $1)
            ORDER BY rank DESC
            LIMIT $2
            "#,
            &[&query, &limit],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(rows
        .iter()
        .map(|row| Message {
            id: row.get("id"),
            content: row.get("content"),
            rank: row.get("rank"),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::{search_documents, search_messages, search_subjects, Document, Message, Subject};
    use deadpool_postgres::{Manager, Pool};

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_search_documents_returns_error_without_db() {
        let pool = build_test_pool();
        let result = search_documents(&pool, "测试", 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_documents_smart_falls_back_to_empty_without_db() {
        let pool = build_test_pool();
        let result = crate::search_documents_smart(&pool, "测试", 10).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_search_subjects_returns_error_without_db() {
        let pool = build_test_pool();
        let result = search_subjects(&pool, "测试", 10).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_messages_returns_error_without_db() {
        let pool = build_test_pool();
        let result = search_messages(&pool, "测试", 10).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_document_struct_serialization() {
        let doc = Document {
            id: "doc-1".to_string(),
            title: Some("测试标题".to_string()),
            content: Some("测试内容".to_string()),
            rank: Some(0.5),
        };
        let json = serde_json::to_value(&doc).unwrap();
        assert_eq!(json["id"], "doc-1");
        assert_eq!(json["title"], "测试标题");
        assert_eq!(json["rank"], 0.5);
    }

    #[test]
    fn test_subject_struct_serialization() {
        let subject = Subject {
            id: "subj-1".to_string(),
            title: Some("主题标题".to_string()),
            content: Some("主题内容".to_string()),
            rank: Some(0.8),
        };
        let json = serde_json::to_value(&subject).unwrap();
        assert_eq!(json["id"], "subj-1");
        assert_eq!(json["title"], "主题标题");
    }

    #[test]
    fn test_message_struct_serialization() {
        let msg = Message {
            id: "msg-1".to_string(),
            content: Some("消息内容".to_string()),
            rank: Some(0.3),
        };
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["id"], "msg-1");
        assert_eq!(json["content"], "消息内容");
    }
}

/// PostgreSQL full-text document search that preserves the endpoint's
/// historical empty-list fallback when the database query is unavailable.
pub async fn search_documents_smart(pool: &Pool, query: &str, limit: i32) -> Vec<Document> {
    search_documents(pool, query, limit)
        .await
        .unwrap_or_default()
}

// ── ftsearch/list（桌面 FtSearchApp「全文搜索引擎」配置串引用，查 x_ftsearch_document 096）──
#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn ftsearch_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, source, score::text AS score, creator, create_time::text AS create_time \
             FROM x_ftsearch_document WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 200",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                (
                    "title".to_string(),
                    Value::String(row.get::<_, Option<String>>("title").unwrap_or_default()),
                ),
                (
                    "source".to_string(),
                    Value::String(row.get::<_, Option<String>>("source").unwrap_or_default()),
                ),
                (
                    "score".to_string(),
                    Value::String(row.get::<_, Option<String>>("score").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

// ── ftsearch 家族 CRUD（x_ftsearch_document 096，通用参数化写；score 为 DOUBLE 不映射）──
fn ftsearch_spec() -> shared::crud::CrudSpec {
    shared::crud::CrudSpec {
        table: "x_ftsearch_document",
        columns: &[("title", "title"), ("body", "body"), ("source", "source")],
        soft_delete: true,
    }
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn ftsearch_create(
    pool: Extension<Pool>,
    body: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = shared::crud_create(&pool, &ftsearch_spec(), &body.0).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn ftsearch_save(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    body: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let saved = shared::crud_save(&pool, &ftsearch_spec(), &id, &body.0).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(saved)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn ftsearch_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let deleted = shared::crud_delete(&pool, &ftsearch_spec(), &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(deleted)),
        ]),
    ))))
}

/// 全文检索 HTTP 路由（供 create_app 挂载；此前 search crate 无 HTTP 面）。
pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/api/ftsearch/list", get(ftsearch_list))
        // ── ftsearch 家族 CRUD ──
        .route("/api/ftsearch/create", post(ftsearch_create))
        .route("/api/ftsearch/save/{id}", put(ftsearch_save))
        .route("/api/ftsearch/save/{id}", post(ftsearch_save))
        .route("/api/ftsearch/delete/{id}", delete(ftsearch_delete))
        .route("/api/ftsearch/delete/{id}", post(ftsearch_delete))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests_generated;
