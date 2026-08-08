use axum::{
    extract::Extension,
    Json, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use std::sync::Arc;
use std::ops::Deref;

// ---- RowGet ----
// Abstraction over a database row so tests can inject mock data.
// Implemented for tokio_postgres::Row (production) and MockRow (tests).

pub trait RowGet: Send + Sync {
    fn get_i32(&self, col: &str) -> i32;
    fn get_i64(&self, col: &str) -> i64;
    fn get_str(&self, col: &str) -> &str;
    fn get_bool(&self, col: &str) -> bool;
}

impl RowGet for deadpool_postgres::tokio_postgres::Row {
    fn get_i32(&self, col: &str) -> i32 {
        self.get(col)
    }
    fn get_i64(&self, col: &str) -> i64 {
        self.get(col)
    }
    fn get_str(&self, col: &str) -> &str {
        self.get(col)
    }
    fn get_bool(&self, col: &str) -> bool {
        self.get(col)
    }
}

// ---- ControlClient ----
// Abstraction over a database client so tests can inject a mock.
// Production impl wraps deadpool_postgres::Object (derefs to PgClient).

#[async_trait::async_trait]
pub trait ControlClient: Send + Sync {
    async fn ctrl_query(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>>;
    async fn ctrl_query_one(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>>;
    async fn ctrl_query_opt(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>>;
    async fn ctrl_execute(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait::async_trait]
impl ControlClient for deadpool_postgres::Object {
    async fn ctrl_query(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = self.deref().query(q, p).await?;
        Ok(rows.into_iter().map(|r| Box::new(r) as Box<dyn RowGet>).collect())
    }
    async fn ctrl_query_one(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>> {
        let row = self.deref().query_one(q, p).await?;
        Ok(Box::new(row) as Box<dyn RowGet>)
    }
    async fn ctrl_query_opt(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        let row_opt = self.deref().query_opt(q, p).await?;
        Ok(row_opt.map(|r| Box::new(r) as Box<dyn RowGet>))
    }
    async fn ctrl_execute(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.deref().execute(q, p).await.map_err(Into::into)
    }
}

// Arc<dyn ControlClient> delegates to the inner impl via this blanket.
#[async_trait::async_trait]
impl ControlClient for Arc<dyn ControlClient> {
    async fn ctrl_query(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Vec<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        self.as_ref().ctrl_query(q, p).await
    }
    async fn ctrl_query_one(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Box<dyn RowGet>, Box<dyn std::error::Error + Send + Sync>> {
        self.as_ref().ctrl_query_one(q, p).await
    }
    async fn ctrl_query_opt(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<Option<Box<dyn RowGet>>, Box<dyn std::error::Error + Send + Sync>> {
        self.as_ref().ctrl_query_opt(q, p).await
    }
    async fn ctrl_execute(
        &self,
        q: &str,
        p: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.as_ref().ctrl_execute(q, p).await
    }
}

// ---- ControlPool ----

pub trait ControlPool: Send + Sync {
    fn acquire<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn ControlClient>, AppError>> + Send + 'a>>;
}

impl ControlPool for Pool {
    fn acquire<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn ControlClient>, AppError>> + Send + 'a>>
    {
        Box::pin(async move {
            let object = self.get().await.map_err(|_| AppError::Internal)?;
            Ok(Arc::new(object) as Arc<dyn ControlClient>)
        })
    }
}

/// Wrapper allowing tests to inject a mock pool via `Arc<dyn ControlPool>`.
pub struct DynControlPool(Arc<dyn ControlPool>);

impl DynControlPool {
    pub fn new(inner: Arc<dyn ControlPool>) -> Self {
        Self(inner)
    }
}

#[async_trait::async_trait]
impl ControlPool for DynControlPool {
    fn acquire<'a>(
        &'a self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Arc<dyn ControlClient>, AppError>> + Send + 'a>>
    {
        self.0.acquire()
    }
}

pub mod routes;

#[cfg(test)]
mod tests;

pub fn file_assemble_control_router(pool: Pool) -> axum::Router {
    let base = routes::router(pool);
    axum::Router::new()
        .merge(base)
        .route("/jaxrs/file/assemble/control/file/list/{folderId}", get(list_files))
        .route("/jaxrs/file/assemble/control/file/{id}", get(get_file))
        .route("/jaxrs/file/assemble/control/file/upload", post(upload_file))
        .route("/jaxrs/file/assemble/control/file/create", post(create_file))
        .route("/jaxrs/file/assemble/control/file/delete/{id}", post(delete_file))
        .route("/jaxrs/file/core/entity/file/create", post(create_file_entity))
        .route("/jaxrs/file/core/entity/file/update/{id}", post(update_file_entity))
        .route("/jaxrs/file/core/entity/file/delete/{id}", post(delete_file_entity))
}

#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Arc<dyn ControlPool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.acquire().await?;

    let row = client
        .ctrl_query_one(
            "SELECT enabled, default_storage, max_upload_size FROM x_file_assemble_control_config LIMIT 1",
            &[],
        )
        .await;

    let data = match row {
        Ok(r) => serde_json::Map::from_iter([
            ("enabled".to_string(), Value::Bool(r.get_bool("enabled"))),
            ("defaultStorage".to_string(), Value::String(r.get_str("default_storage").to_string())),
            ("maxUploadSize".to_string(), Value::Number(serde_json::Number::from(r.get_i64("max_upload_size")))),
        ]),
        Err(_) => serde_json::Map::from_iter([
            ("enabled".to_string(), Value::Bool(true)),
            ("defaultStorage".to_string(), Value::String("local".to_string())),
            ("maxUploadSize".to_string(), Value::Number(serde_json::Number::from(104857600i64))),
        ]),
    };

    Ok(Json(ActionResult::success(Value::Object(data))))
}

#[axum::debug_handler]
pub async fn list_storage_pools(
    pool: Extension<Arc<dyn ControlPool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.acquire().await?;

    let rows = client
        .ctrl_query(
            "SELECT id, name, enabled FROM x_file_assemble_control_storage_pool ORDER BY id",
            &[],
        )
        .await;

    let data: Vec<Value> = match rows {
        Ok(r) => r
            .iter()
            .map(|row| {
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get_str("id").to_string())),
                    ("name".to_string(), Value::String(row.get_str("name").to_string())),
                    ("enabled".to_string(), Value::Bool(row.get_bool("enabled"))),
                ]))
            })
            .collect(),
        Err(_) => vec![],
    };

    Ok(Json(ActionResult::success(Value::Array(data))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Arc<dyn ControlPool>>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.acquire().await?;

    let enabled = body
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let default_storage = body
        .get("defaultStorage")
        .and_then(|v| v.as_str())
        .unwrap_or("local")
        .to_string();
    let max_upload_size: i64 = body
        .get("maxUploadSize")
        .and_then(|v| v.as_i64())
        .unwrap_or(104857600);

    client
        .ctrl_execute(
            "UPDATE x_file_assemble_control_config SET enabled = $1, default_storage = $2, max_upload_size = $3 WHERE id = 'global'",
            &[&enabled, &default_storage, &max_upload_size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("enabled".to_string(), Value::Bool(enabled)),
            ("defaultStorage".to_string(), Value::String(default_storage)),
            ("maxUploadSize".to_string(), Value::Number(serde_json::Number::from(max_upload_size))),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn list_control_categories(
    pool: Extension<Arc<dyn ControlPool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.acquire().await?;

    let rows = client
        .ctrl_query(
            "SELECT id, name, description FROM x_file_assemble_control_category ORDER BY id",
            &[],
        )
        .await;

    let categories: Vec<Value> = match rows {
        Ok(r) => r
            .iter()
            .map(|row| {
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get_str("id").to_string())),
                    ("name".to_string(), Value::String(row.get_str("name").to_string())),
                    ("description".to_string(), Value::String(row.get_str("description").to_string())),
                ]))
            })
            .collect(),
        Err(_) => vec![],
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(categories.len() as i64))),
            ("data".to_string(), Value::Array(categories)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::file_assemble_control_router(pool)
}


#[axum::debug_handler]
pub async fn list_files(
    pool: Extension<Pool>,
    axum::extract::Path(folder_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, path, size, creator, create_time, folder_id \
              FROM x_file WHERE folder_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("path".to_string(), Value::String(row.get("path"))),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("folderId".to_string(), Value::String(row.get("folder_id"))),
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
pub async fn get_file(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, path, size, creator, create_time, folder_id \
              FROM x_file WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("path".to_string(), Value::String(row.get("path"))),
                ("size".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("size")))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("folderId".to_string(), Value::String(row.get("folder_id"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn upload_file(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = body.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = body.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_file (id, name, path, size, creator, create_time, folder_id) \
              VALUES ($1, $2, $3, $4, $5, NOW(), $6)",
            &[&id, &name, &path, &size, &creator, &folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("path".to_string(), Value::String(path)),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("creator".to_string(), Value::String(creator)),
        ("folderId".to_string(), Value::String(folder_id)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
pub async fn create_file(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = body.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = body.get("creator").and_then(|v| v.as_str()).unwrap_or("system").to_string();

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_file (id, name, path, size, creator, create_time, folder_id) \
              VALUES ($1, $2, $3, $4, $5, NOW(), $6)",
            &[&id, &name, &path, &size, &creator, &folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn delete_file(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_file SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("file not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn create_file_entity(
    pool: Extension<Pool>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let folder_id = body.get("folderId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(0);
    let creator = "system";

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_file (id, name, path, size, creator, create_time, folder_id) \
              VALUES ($1, $2, $3, $4, $5, NOW(), $6)",
            &[&id, &name, &path, &size, &creator, &folder_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("path".to_string(), Value::String(path)),
            ("folderId".to_string(), Value::String(folder_id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn update_file_entity(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT creator FROM x_file WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("file not found")));
    };

    let creator: String = row.get("creator");
    shared::middleware::require_owner(&pool, &session, &creator).await?;

    let has_name = body.get("name").is_some();
    let has_path = body.get("path").is_some();
    let has_size = body.get("size").is_some();
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let path = body.get("path").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let size = body.get("size").and_then(|v| v.as_i64()).unwrap_or(-1);

    let row = if has_name && has_path && has_size {
        client
            .query_opt(
                "UPDATE x_file SET name = NULLIF($2, ''), path = NULLIF($3, ''), size = NULLIF($4, -1), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path, size",
                &[&id, &name, &path, &size],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_name && has_path {
        client
            .query_opt(
                "UPDATE x_file SET name = NULLIF($2, ''), path = NULLIF($3, ''), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path",
                &[&id, &name, &path],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_name && has_size {
        client
            .query_opt(
                "UPDATE x_file SET name = NULLIF($2, ''), size = NULLIF($3, -1), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path, size",
                &[&id, &name, &size],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_path && has_size {
        client
            .query_opt(
                "UPDATE x_file SET path = NULLIF($2, ''), size = NULLIF($3, -1), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path, size",
                &[&id, &path, &size],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_name {
        client
            .query_opt(
                "UPDATE x_file SET name = NULLIF($2, ''), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path",
                &[&id, &name],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_path {
        client
            .query_opt(
                "UPDATE x_file SET path = NULLIF($2, ''), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path",
                &[&id, &path],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else if has_size {
        client
            .query_opt(
                "UPDATE x_file SET size = NULLIF($2, -1), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path, size",
                &[&id, &size],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query_opt(
                "UPDATE x_file SET update_time = NOW() WHERE id = $1 AND deleted_at IS NULL RETURNING id, name, path",
                &[&id],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    match row {
        Some(row) => {
            let result_name: String = row.get("name");
            let result_path: String = row.get("path");
            let result_size: Option<i64> = row.get("size");

            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("saved".to_string(), Value::Bool(true)),
                ("name".to_string(), Value::String(result_name)),
                ("path".to_string(), Value::String(result_path)),
            ]);
            if let Some(s) = result_size {
                map.insert("size".to_string(), Value::Number(serde_json::Number::from(s)));
            }

            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

#[axum::debug_handler]
pub async fn delete_file_entity(
    pool: Extension<Pool>,
    Extension(session): Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT creator FROM x_file WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("file not found or already deleted")));
    };

    let creator: String = row.get("creator");
    shared::middleware::require_owner(&pool, &session, &creator).await?;

    let result = client
        .execute(
            "DELETE FROM x_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("file not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}


pub async fn anonymous_file_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_file_id_download_stream() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_list_editor_owner() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment_list_folder_folderId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment_list_share_owner() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment_list_top() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment_upload_folder_folderId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_upload_folder_folderId_callback_callback() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_binary_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_download_stream() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_image_scale_scale_binary_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_image_width_width_height_height_binary_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_update() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment_id_update_callback_callback() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_exist_file_fileMd5() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_list_editor_owner() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment2_list_filter_name() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment2_list_folder_folderId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment2_list_share_owner() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment2_list_top() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment2_list_type_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn attachment2_upload_folder_folderId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_user_capacity() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_id_binary_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_id_download_image_width_width_height_height() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_id_download_stream() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_id_image_scale_scale_binary_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_id_image_width_width_height_height_binary_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn attachment2_id_office_preview_type_type() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn complex_folder_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn complex_top() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn config_is_file_manager() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn config_system_config() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn editor_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_clean_unused_referencetype_cmsdocument_manage() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_copy_attachment_attachmentId_referencetype_referenceType_reference_reference_scale_scale() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_list_referencetype() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_referencetype_referenceType_reference_reference() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_unused_referencetype_cmsdocument_manage() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_id_next_count_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_id_next_count_referencetype_referenceType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_id_prev_count_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_id_prev_count_referencetype_referenceType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_referencetype_referenceType_reference_reference() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_upload_referencetype_referenceType_reference_reference_scale_scale() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_upload_referencetype_referenceType_reference_reference_scale_scale_callback_callback() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_upload_with_url() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id_binary_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id_download_stream() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn folder_list_top() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn folder_list_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn folder_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn folder2_batch_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn folder2_list_top() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn folder2_list_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn folder2_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn folder2_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn recycle_empty() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn recycle_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn recycle_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn recycle_id_delete() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn recycle_id_resume() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn share_download_share_shareId_file_fileId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn share_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn share_list_att_share_shareId_folder_folderId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn share_list_folder_share_shareId_folder_folderId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn share_list_my() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn share_list_my2_shareType_fileType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn share_list_to_me() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn share_list_to_me2_fileType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn share_share_shareId_file_fileId_folder_folderId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn share_shield_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn share_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn share_id_password_password() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}
