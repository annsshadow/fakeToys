pub use crate::RowGet;

use crate::{
    ControlClient, ControlPool, DynControlPool, error::AppError,
};
use axum::extract::Extension;
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::Value;
use crate::response::ActionResult;
use std::sync::Arc;
use tokio::sync::Mutex;

// ---- Mock types ----

#[derive(Clone)]
pub struct MockRow {
    pub values: Vec<(&'static str, Value)>,
}

impl RowGet for MockRow {
    fn get_i32(&self, col: &str) -> i32 {
        self.values
            .iter()
            .find(|(k, _)| *k == col)
            .and_then(|(_, v)| v.as_i64())
            .unwrap_or(0) as i32
    }
    fn get_i64(&self, col: &str) -> i64 {
        self.values
            .iter()
            .find(|(k, _)| *k == col)
            .and_then(|(_, v)| v.as_i64())
            .unwrap_or(0)
    }
    fn get_str(&self, col: &str) -> &str {
        self.values
            .iter()
            .find(|(k, _)| *k == col)
            .and_then(|(_, v)| v.as_str())
            .unwrap_or("")
    }
    fn get_bool(&self, col: &str) -> bool {
        self.values
            .iter()
            .find(|(k, _)| *k == col)
            .and_then(|(_, v)| v.as_bool())
            .unwrap_or(false)
    }
}

pub enum MockQueryResult {
    Row(Vec<(&'static str, Value)>),
    Rows(Vec<Vec<(&'static str, Value)>>),
    Empty,
    Error,
}

pub struct MockControlClient {
    pub results: Arc<Mutex<Vec<MockQueryResult>>>,
}

impl MockControlClient {
    pub fn new(results: Arc<Mutex<Vec<MockQueryResult>>>) -> Self {
        Self { results }
    }

    pub fn single_row(values: Vec<(&'static str, Value)>) -> Arc<Mutex<Vec<MockQueryResult>>> {
        Arc::new(Mutex::new(vec![MockQueryResult::Row(values)]))
    }

    pub fn rows(values: Vec<Vec<(&'static str, Value)>>) -> Arc<Mutex<Vec<MockQueryResult>>> {
        Arc::new(Mutex::new(vec![MockQueryResult::Rows(values)]))
    }

    pub fn empty() -> Arc<Mutex<Vec<MockQueryResult>>> {
        Arc::new(Mutex::new(vec![MockQueryResult::Empty]))
    }

    pub fn count(n: u64) -> Arc<Mutex<Vec<MockQueryResult>>> {
        Arc::new(Mutex::new(vec![MockQueryResult::Row(vec![("count", Value::Number(serde_json::Number::from(n)))])]))
    }
}

#[async_trait::async_trait]
impl ControlClient for MockControlClient {
    async fn ctrl_query(
        &self,
        _q: &str,
        _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        match self.results.lock().await.pop() {
            Some(MockQueryResult::Rows(rows)) => Ok(rows.into_iter().map(|v| Box::new(MockRow { values: v }) as Box<dyn RowGet>).collect()),
            Some(MockQueryResult::Empty) => Ok(vec![]),
            Some(MockQueryResult::Error) => Err(Box::<dyn std::error::Error + Send + Sync>::from("mock query error")),
            _ => Ok(vec![]),
        }
    }

    async fn ctrl_query_one(
        &self,
        _q: &str,
        _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>> {
        match self.results.lock().await.pop() {
            Some(MockQueryResult::Row(values)) => {
                Ok(Box::new(MockRow { values }) as Box<dyn RowGet>)
            }
            Some(MockQueryResult::Error) => Err(Box::<dyn std::error::Error + Send + Sync>::from("mock query error")),
            _ => Err(Box::<dyn std::error::Error + Send + Sync>::from("mock: no result")),
        }
    }

    async fn ctrl_query_opt(
        &self,
        _q: &str,
        _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    async fn ctrl_execute(
        &self,
        _q: &str,
        _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        match self.results.lock().await.pop() {
            Some(MockQueryResult::Row(values)) => {
                values.iter().find(|(k, _)| *k == "count").and_then(|(_, v)| v.as_u64()).unwrap_or(1);
                Ok(values.iter().find(|(k, _)| *k == "count").and_then(|(_, v)| v.as_u64()).unwrap_or(1))
            }
            _ => Ok(1),
        }
    }
}

// ---- MockControlPool ----

pub struct MockControlPool {
    client: Arc<MockControlClient>,
}

impl MockControlPool {
    pub fn new(client: Arc<MockControlClient>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl ControlPool for MockControlPool {
    fn acquire<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<std::sync::Arc<dyn ControlClient>, AppError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(self.client.clone() as std::sync::Arc<dyn ControlClient>) })
    }
}

// ---- Helpers ----

pub fn mock_control_pool(
    results: Arc<Mutex<Vec<MockQueryResult>>>,
) -> std::sync::Arc<dyn ControlPool> {
    let client = Arc::new(MockControlClient::new(results));
    let pool = MockControlPool::new(client);
    std::sync::Arc::new(DynControlPool::new(std::sync::Arc::new(pool)))
}
