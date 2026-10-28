// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

pub use crate::RowGet;

use crate::{error::AppError, ControlClient, ControlPool, DynControlPool};
use serde_json::Value;
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
        Arc::new(Mutex::new(vec![MockQueryResult::Row(vec![(
            "count",
            Value::Number(serde_json::Number::from(n)),
        )])]))
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
            Some(MockQueryResult::Rows(rows)) => Ok(rows
                .into_iter()
                .map(|v| Box::new(MockRow { values: v }) as Box<dyn RowGet>)
                .collect()),
            Some(MockQueryResult::Empty) => Ok(vec![]),
            Some(MockQueryResult::Error) => Err(Box::<dyn std::error::Error + Send + Sync>::from(
                "mock query error",
            )),
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
            Some(MockQueryResult::Error) => Err(Box::<dyn std::error::Error + Send + Sync>::from(
                "mock query error",
            )),
            _ => Err(Box::<dyn std::error::Error + Send + Sync>::from(
                "mock: no result",
            )),
        }
    }

    async fn ctrl_query_opt(
        &self,
        _q: &str,
        _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        match self.results.lock().await.pop() {
            Some(MockQueryResult::Row(values)) => {
                Ok(Some(Box::new(MockRow { values }) as Box<dyn RowGet>))
            }
            Some(MockQueryResult::Rows(rows)) if rows.len() == 1 => Ok(Some(Box::new(MockRow {
                values: rows[0].clone(),
            })
                as Box<dyn RowGet>)),
            Some(MockQueryResult::Empty) | None => Ok(None),
            Some(MockQueryResult::Error) => Err(Box::<dyn std::error::Error + Send + Sync>::from(
                "mock query error",
            )),
            _ => Ok(None),
        }
    }

    async fn ctrl_execute(
        &self,
        _q: &str,
        _p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        match self.results.lock().await.pop() {
            Some(MockQueryResult::Row(values)) => Ok(values
                .iter()
                .find(|(k, _)| *k == "count")
                .and_then(|(_, v)| v.as_u64())
                .unwrap_or(1)),
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
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<std::sync::Arc<dyn ControlClient>, AppError>>
                + Send
                + 'a,
        >,
    > {
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

#[cfg(test)]
mod tests {
    // mock 是全仓下游 crate 测试的地基：它们的「绿」都建立在
    // 这里的弹栈顺序与回退语义上。这组用例把这些语义钉死——
    // mock 自身行为漂移时，先红的必须是这里，而不是下游的假红。
    use super::*;

    fn err_of(r: Result<(), Box<dyn std::error::Error + Send + Sync>>) -> String {
        r.err().unwrap().to_string()
    }

    #[tokio::test]
    async fn rows_result_maps_to_rowget_with_defaults_on_missing_columns() {
        let results = MockControlClient::rows(vec![
            vec![
                ("id", serde_json::json!(7)),
                ("name", serde_json::json!("n")),
                ("flag", serde_json::json!(true)),
                ("big", serde_json::json!(1_000_000_000_i64)),
            ],
            vec![("id", serde_json::json!(8))],
        ]);
        let client = MockControlClient::new(results);
        let rows = client.ctrl_query("SELECT 1", &[]).await.unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].get_i32("id"), 7);
        assert_eq!(rows[0].get_i64("big"), 1_000_000_000);
        assert_eq!(rows[0].get_str("name"), "n");
        assert!(rows[0].get_bool("flag"));
        // 缺列时按类型给零值，而不是 panic——下游测试依赖这个宽松契约
        assert_eq!(rows[1].get_str("name"), "");
        assert_eq!(rows[1].get_i64("big"), 0);
        assert!(!rows[1].get_bool("flag"));
    }

    #[tokio::test]
    async fn query_results_pop_in_stack_order_and_empty_queue_yields_no_rows() {
        // Row 先入：query 消费的是**栈顶**（pop）——先压 Rows、后压 Row 时，
        // 第一次 query 拿到的是 Row 分支的回退（空），第二次才是 Rows。
        let results = Arc::new(Mutex::new(vec![
            MockQueryResult::Rows(vec![vec![("id", serde_json::json!(1))]]),
            MockQueryResult::Row(vec![("id", serde_json::json!(2))]),
        ]));
        let client = MockControlClient::new(results);
        // Row 不是 Rows/Empty/Error → `_ => Ok(vec![])`
        assert!(client.ctrl_query("q", &[]).await.unwrap().is_empty());
        let rows = client.ctrl_query("q", &[]).await.unwrap();
        assert_eq!(rows.len(), 1);
        // 队列耗尽 → `_ => Ok(vec![])`，不 panic
        assert!(client.ctrl_query("q", &[]).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn empty_and_error_results() {
        let client = MockControlClient::new(MockControlClient::empty());
        assert!(client.ctrl_query("q", &[]).await.unwrap().is_empty());
        let client = MockControlClient::new(Arc::new(Mutex::new(vec![MockQueryResult::Error])));
        assert_eq!(
            err_of(client.ctrl_query("q", &[]).await.map(|_| ())),
            "mock query error"
        );
    }

    #[tokio::test]
    async fn query_one_row_error_or_missing() {
        let client = MockControlClient::new(MockControlClient::single_row(vec![(
            "name",
            serde_json::json!("x"),
        )]));
        let row = client.ctrl_query_one("q", &[]).await.unwrap();
        assert_eq!(row.get_str("name"), "x");

        let client = MockControlClient::new(Arc::new(Mutex::new(vec![MockQueryResult::Error])));
        assert_eq!(
            err_of(client.ctrl_query_one("q", &[]).await.map(|_| ())),
            "mock query error"
        );

        // Empty / 耗尽 → "mock: no result"
        let client = MockControlClient::new(MockControlClient::empty());
        assert_eq!(
            err_of(client.ctrl_query_one("q", &[]).await.map(|_| ())),
            "mock: no result"
        );
        assert_eq!(
            err_of(client.ctrl_query_one("q", &[]).await.map(|_| ())),
            "mock: no result"
        );
    }

    #[tokio::test]
    async fn query_opt_all_branches() {
        // Row → Some
        let client = MockControlClient::new(MockControlClient::single_row(vec![(
            "id",
            serde_json::json!(3),
        )]));
        assert_eq!(
            client
                .ctrl_query_opt("q", &[])
                .await
                .unwrap()
                .unwrap()
                .get_i64("id"),
            3
        );
        // 单行 Rows → Some（下游常用 rows(...) 配 query_opt）
        let client = MockControlClient::new(MockControlClient::rows(vec![vec![(
            "id",
            serde_json::json!(4),
        )]]));
        assert_eq!(
            client
                .ctrl_query_opt("q", &[])
                .await
                .unwrap()
                .unwrap()
                .get_i64("id"),
            4
        );
        // Empty → None；耗尽 → None；多行 Rows → None（query_opt 语义：不取多行）
        let client = MockControlClient::new(MockControlClient::empty());
        assert!(client.ctrl_query_opt("q", &[]).await.unwrap().is_none());
        let client = MockControlClient::new(Arc::new(Mutex::new(vec![])));
        assert!(client.ctrl_query_opt("q", &[]).await.unwrap().is_none());
        let client = MockControlClient::new(MockControlClient::rows(vec![
            vec![("id", serde_json::json!(1))],
            vec![("id", serde_json::json!(2))],
        ]));
        assert!(client.ctrl_query_opt("q", &[]).await.unwrap().is_none());
        // Error → Err
        let client = MockControlClient::new(Arc::new(Mutex::new(vec![MockQueryResult::Error])));
        assert_eq!(
            err_of(client.ctrl_query_opt("q", &[]).await.map(|_| ())),
            "mock query error"
        );
    }

    #[tokio::test]
    async fn execute_returns_count_column_or_default_one() {
        let client = MockControlClient::new(MockControlClient::count(42));
        assert_eq!(client.ctrl_execute("UPDATE 1", &[]).await.unwrap(), 42);
        // 无 count 列 / Empty / 耗尽 → 默认 1（「至少影响 1 行」的乐观回退）
        let client = MockControlClient::new(MockControlClient::single_row(vec![(
            "id",
            serde_json::json!(1),
        )]));
        assert_eq!(client.ctrl_execute("UPDATE 1", &[]).await.unwrap(), 1);
        let client = MockControlClient::new(MockControlClient::empty());
        assert_eq!(client.ctrl_execute("UPDATE 1", &[]).await.unwrap(), 1);
        let client = MockControlClient::new(Arc::new(Mutex::new(vec![])));
        assert_eq!(client.ctrl_execute("UPDATE 1", &[]).await.unwrap(), 1);
    }

    #[tokio::test]
    async fn pool_acquire_hands_out_the_same_client() {
        let pool = mock_control_pool(MockControlClient::count(5));
        let client = pool.acquire().await.unwrap();
        assert_eq!(client.ctrl_execute("UPDATE 1", &[]).await.unwrap(), 5);
        // 同一 pool 反复 acquire 拿到同一 client（结果队列共享，弹栈语义才成立）
        let client2 = pool.acquire().await.unwrap();
        assert_eq!(client2.ctrl_execute("UPDATE 1", &[]).await.unwrap(), 1);
    }
}
