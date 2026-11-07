// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, routing::get, Json, Router};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

/// 读门结果：区分「不存在」（保持原 NotFound 语义）与「存在但无权」（403）。
enum ReadGate {
    NotFound,
    Forbidden,
    Allowed,
}

fn opt_session(
    session: &Option<Extension<shared::session::Session>>,
) -> Option<&shared::session::Session> {
    session.as_ref().map(|e| &e.0)
}

/// 内容读门（cms §3）：`published` 全员（含匿名）可读；其余状态仅 `creator`
/// 本人与管理员。参数固定为 ($1 id, $2 会话人, $3 是否管理员)。
const CONTENT_READ_ALLOWED_SQL: &str = "SELECT 1 AS hit FROM x_cms_content \
     WHERE id = $1 AND deleted_at IS NULL \
     AND (status = 'published' \
     OR ($2::text IS NOT NULL AND creator = $2::text) \
     OR $3::boolean)";
const CONTENT_READ_EXISTS_SQL: &str =
    "SELECT 1 AS hit FROM x_cms_content WHERE id = $1 AND deleted_at IS NULL";

async fn content_read_gate(
    pool: &Pool,
    id: &str,
    session: Option<&shared::session::Session>,
) -> Result<ReadGate, AppError> {
    let client = shared::db::acquire(pool).await?;
    let person = session.map(|s| s.person_unique.clone());
    let admin = match session {
        Some(s) => shared::middleware::is_admin(pool, &s.person_unique).await,
        None => false,
    };
    let hit = client
        .query_opt(CONTENT_READ_ALLOWED_SQL, &[&id, &person, &admin])
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    if hit.is_some() {
        return Ok(ReadGate::Allowed);
    }
    let exists = client
        .query_opt(CONTENT_READ_EXISTS_SQL, &[&id])
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    Ok(if exists.is_some() {
        ReadGate::Forbidden
    } else {
        ReadGate::NotFound
    })
}

// CMS 内容实体
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CmsContent {
    pub id: String,
    pub title: String,
    pub category_id: String,
    pub status: String,
}

/// 获取 CMS 内容列表
/// 从数据库查询 x_cms_content 表
pub async fn content_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT id, title, category_id, status FROM x_cms_content ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "title".to_string(),
                    Value::String(row.get::<_, Option<String>>("title").unwrap_or_default()),
                ),
                (
                    "categoryId".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("category_id")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "status".to_string(),
                    Value::String(row.get::<_, Option<String>>("status").unwrap_or_default()),
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

/// 获取 CMS 内容详情
/// 从数据库查询 x_cms_content 表，按 ID 查询
pub async fn content_detail(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match content_read_gate(&pool, &id, opt_session(&session)).await? {
        ReadGate::NotFound => return Err(AppError::NotFound),
        ReadGate::Forbidden => return Err(AppError::Forbidden),
        ReadGate::Allowed => {}
    }
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_one(
            "SELECT id, title, category_id, status, content FROM x_cms_content WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        (
            "id".to_string(),
            Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
        ),
        (
            "title".to_string(),
            Value::String(row.get::<_, Option<String>>("title").unwrap_or_default()),
        ),
        (
            "categoryId".to_string(),
            Value::String(
                row.get::<_, Option<String>>("category_id")
                    .unwrap_or_default(),
            ),
        ),
        (
            "status".to_string(),
            Value::String(row.get::<_, Option<String>>("status").unwrap_or_default()),
        ),
        (
            "content".to_string(),
            row.get::<_, Option<String>>("content")
                .map(Value::String)
                .unwrap_or_default(),
        ),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 创建 CMS 核心服务路由
/// 注册以下路由：
/// - /api/cms/core/express/content/list - 内容列表
/// - /api/cms/core/express/content/detail/{id} - 内容详情
pub fn cms_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/cms/core/express/content/list", get(content_list))
        .route(
            "/api/cms/core/express/content/detail/{id}",
            get(content_detail),
        )
        .layer(Extension(pool))
}

#[cfg(test)]
mod security_gate_tests_read_a_b4;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

#[cfg(test)]
mod tests_manual;
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_core_express_router(pool)
}
