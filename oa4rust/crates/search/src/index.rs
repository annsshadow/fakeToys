//! Tantivy-backed local full-text search indexes (plan002 U4).
//!
//! Each corpus (documents / subjects / messages) gets a local Tantivy index
//! under `SEARCH_INDEX_DIR` (default `data/search_index/<corpus>`). Indexes
//! are lazily ingested from the primary tables on first query. All fallible
//! operations return `Err`, letting callers fall back to the PostgreSQL
//! `to_tsvector` implementations.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use deadpool_postgres::Pool;
use serde::Deserialize;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, FAST, STORED, TEXT};
use tantivy::{doc, Index, ReloadPolicy};

const DEFAULT_INDEX_ROOT: &str = "data/search_index";
const MAX_INGEST_ROWS: i64 = 50_000;

fn index_root() -> PathBuf {
    PathBuf::from(std::env::var("SEARCH_INDEX_DIR").unwrap_or_else(|_| DEFAULT_INDEX_ROOT.to_string()))
}

#[derive(Deserialize, Clone)]
struct IngestRow {
    id: String,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    content: Option<String>,
}

struct CorpusIndex {
    index: Index,
    id_field: tantivy::schema::Field,
    title_field: Option<tantivy::schema::Field>,
    content_field: tantivy::schema::Field,
}

impl CorpusIndex {
    fn open(corpus: &str, with_title: bool) -> Result<Self, String> {
        let path = index_root().join(corpus);
        std::fs::create_dir_all(&path).map_err(|e| format!("create_dir {}: {e}", path.display()))?;
        let mut sb = Schema::builder();
        let id_field = sb.add_text_field("id", STORED | FAST);
        let title_field = with_title.then(|| sb.add_text_field("title", TEXT | STORED));
        let content_field = sb.add_text_field("content", TEXT);
        let schema = sb.build();
        let index = match Index::open_in_dir(&path) {
            Ok(idx) => idx,
            Err(_) => Index::create_in_dir(&path, schema).map_err(|e| format!("create index: {e}"))?,
        };
        Ok(Self { index, id_field, title_field, content_field })
    }

    fn replace_all(&self, rows: &[IngestRow], with_title: bool) -> Result<(), String> {
        let mut writer = self.index.writer(20_000_000).map_err(|e| format!("writer: {e}"))?;
        writer.delete_all_documents().map_err(|e| format!("delete_all: {e}"))?;
        for r in rows {
            let mut d = doc!(self.id_field => r.id.clone());
            if let (Some(tf), true) = (self.title_field, with_title) {
                d.add_text(tf, r.title.clone().unwrap_or_default());
            }
            d.add_text(self.content_field, r.content.clone().unwrap_or_default());
            writer.add_document(d).map_err(|e| format!("add_document: {e}"))?;
        }
        writer.commit().map_err(|e| format!("commit: {e}"))?;
        writer.wait_merging_threads().map_err(|e| format!("merge threads: {e}"))
    }

    fn search_ids(&self, query: &str, limit: usize) -> Result<Vec<(String, f32)>, String> {
        let reader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()
            .map_err(|e| format!("reader: {e}"))?;
        let searcher = reader.searcher();
        let mut fields = vec![self.content_field];
        if let Some(tf) = self.title_field {
            fields.push(tf);
        }
        let parser = QueryParser::for_index(&self.index, fields);
        let q = parser
            .parse_query(query)
            .map_err(|e| format!("parse_query '{query}': {e}"))?;
        let hits = searcher
            .search(&q, &TopDocs::with_limit(limit))
            .map_err(|e| format!("search: {e}"))?;
        let mut out = Vec::with_capacity(hits.len());
        for (_score, addr) in hits {
            let d = searcher
                .doc::<tantivy::TantivyDocument>(addr)
                .map_err(|e| format!("doc: {e}"))?;
            let id = d
                .get_first(self.id_field)
                .and_then(|v| match v {
                    tantivy::schema::OwnedValue::Str(s) => Some(s.clone()),
                    _ => None,
                })
                .unwrap_or_default();
            out.push((id, 0.0));
        }
        Ok(out)
    }
}

struct CorpusHandle {
    ci: Option<Arc<CorpusIndex>>,
    ingested: AtomicBool,
}

impl CorpusHandle {
    fn new() -> Self {
        Self { ci: None, ingested: AtomicBool::new(false) }
    }
}

fn corpus_slot(
    lock: &'static OnceLock<Mutex<CorpusHandle>>,
) -> &'static Mutex<CorpusHandle> {
    lock.get_or_init(|| Mutex::new(CorpusHandle::new()))
}

async fn fetch_ingest_rows(
    pool: &Pool,
    sql: &str,
) -> Result<Vec<IngestRow>, crate::AppError> {
    let client = pool.get().await.map_err(|_| crate::AppError::Internal)?;
    let rows = client.query(sql, &[&MAX_INGEST_ROWS]).await.map_err(|_| crate::AppError::Internal)?;
    Ok(rows
        .iter()
        .map(|r| IngestRow {
            id: r.get("id"),
            title: r.try_get("title").unwrap_or(None),
            content: r.try_get("content").unwrap_or(None),
        })
        .collect())
}

/// Ensure the corpus index exists+ingested, then run the query; returns ids in rank order.
async fn search_corpus(
    lock: &'static OnceLock<Mutex<CorpusHandle>>,
    corpus: &'static str,
    with_title: bool,
    sql: &'static str,
    pool: &Pool,
    query: &str,
    limit: usize,
) -> Result<Vec<String>, crate::AppError> {
    let slot = corpus_slot(lock);
    let needs_ingest = {
        let g = slot.lock().map_err(|_| crate::AppError::Internal)?;
        !g.ingested.load(Ordering::Relaxed) || g.ci.is_none()
    };
    let ci = if needs_ingest {
        let rows = fetch_ingest_rows(pool, sql).await?;
        let opened = Arc::new(CorpusIndex::open(corpus, with_title).map_err(|_| crate::AppError::Internal)?);
        let clone = opened.clone();
        let clone_rows = rows.clone();
        tokio::task::spawn_blocking(move || clone.replace_all(&clone_rows, with_title))
            .await
            .map_err(|_| crate::AppError::Internal)?
            .map_err(|_| crate::AppError::Internal)?;
        let mut g = slot.lock().map_err(|_| crate::AppError::Internal)?;
        g.ci = Some(opened);
        g.ingested.store(true, Ordering::Relaxed);
        drop(g);
        let g2 = slot.lock().map_err(|_| crate::AppError::Internal)?;
        g2.ci.clone().ok_or(crate::AppError::Internal)?
    } else {
        let g = slot.lock().map_err(|_| crate::AppError::Internal)?;
        g.ci.clone().ok_or(crate::AppError::Internal)?
    };

    let q = query.to_string();
    tokio::task::spawn_blocking(move || ci.search_ids(&q, limit))
        .await
        .map_err(|_| crate::AppError::Internal)?
        .map_err(|_| crate::AppError::Internal)
        .map(|hits| hits.into_iter().map(|(id, _)| id).collect())
}

static DOCUMENTS: OnceLock<Mutex<CorpusHandle>> = OnceLock::new();
static SUBJECTS: OnceLock<Mutex<CorpusHandle>> = OnceLock::new();
static MESSAGES: OnceLock<Mutex<CorpusHandle>> = OnceLock::new();

const DOCUMENTS_SQL: &str = "SELECT id, title, content FROM x_cms_document ORDER BY create_time DESC LIMIT $1";
const SUBJECTS_SQL: &str = "SELECT id, title, content FROM bbs_subject_info ORDER BY id LIMIT $1";
const MESSAGES_SQL: &str = "SELECT id, NULL::text AS title, content FROM x_message ORDER BY id LIMIT $1";

/// Tantivy search over x_cms_document; returns matching ids ranked.
pub async fn documents_search_ids(
    pool: &Pool,
    query: &str,
    limit: i32,
) -> Result<Vec<String>, crate::AppError> {
    search_corpus(&DOCUMENTS, "documents", true, DOCUMENTS_SQL, pool, query, limit as usize).await
}

/// Tantivy search over bbs_subject_info; returns matching ids ranked.
pub async fn subjects_search_ids(
    pool: &Pool,
    query: &str,
    limit: i32,
) -> Result<Vec<String>, crate::AppError> {
    search_corpus(&SUBJECTS, "subjects", true, SUBJECTS_SQL, pool, query, limit as usize).await
}

/// Tantivy search over x_message; returns matching ids ranked.
pub async fn messages_search_ids(
    pool: &Pool,
    query: &str,
    limit: i32,
) -> Result<Vec<String>, crate::AppError> {
    search_corpus(&MESSAGES, "messages", false, MESSAGES_SQL, pool, query, limit as usize).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_index(corpus: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("oa4rust-search-test-{}-{}", corpus, std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_index_ingest_and_search() {
        let path = tmp_index("unit");
        std::env::set_var("SEARCH_INDEX_DIR", &path);
        let ci = CorpusIndex::open("unit", true).unwrap();
        let rows = vec![
            IngestRow { id: "d1".into(), title: Some("Rust 全文检索".into()), content: Some("tantivy 引擎测试".into()) },
            IngestRow { id: "d2".into(), title: Some("无关文档".into()), content: Some("别的内容".into()) },
        ];
        ci.replace_all(&rows, true).unwrap();
        let hits = ci.search_ids("tantivy", 10).unwrap();
        let ids: Vec<_> = hits.into_iter().map(|(i, _)| i).collect();
        assert_eq!(ids, vec!["d1".to_string()]);
        std::fs::remove_dir_all(path).ok();
    }

    #[test]
    fn test_search_unknown_term_returns_empty() {
        let path = tmp_index("empty");
        std::env::set_var("SEARCH_INDEX_DIR", &path);
        let ci = CorpusIndex::open("empty", true).unwrap();
        ci.replace_all(
            &[IngestRow { id: "x".into(), title: Some("标题".into()), content: Some("内容".into()) }],
            true,
        )
        .unwrap();
        assert!(ci.search_ids("zzznotfound", 10).unwrap().is_empty());
        std::fs::remove_dir_all(path).ok();
    }
}
