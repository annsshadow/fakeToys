use deadpool_postgres::Pool;
use serde::Serialize;
use shared::error::AppError;

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
    use crate::{search_documents, search_subjects, search_messages, Document, Subject, Message};
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
