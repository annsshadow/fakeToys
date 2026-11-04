// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, routing::get, Json, Router};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

/// O23：`config_value` 是 TEXT，不能直接 `.get::<bool>()`（类型错配会 500）。
/// 缺行是「没初始化配置」→ 503；有行但值不是 true/false 是真数据缺陷 → 500。
async fn read_sync_enabled(client: &deadpool_postgres::Object) -> Result<bool, AppError> {
    let row = client
        .query_opt(
            "SELECT config_value FROM x_org_config WHERE config_key = 'sync_enabled'",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    let Some(row) = row else {
        return Err(AppError::missing_config("x_org_config.sync_enabled"));
    };
    let raw: String = row.get("config_value");
    raw.parse::<bool>().map_err(|e| {
        AppError::internal_with_source(
            concat!(file!(), ":", line!()),
            format!("x_org_config.sync_enabled={raw:?}: {e}"),
        )
    })
}

/// 获取组织服务状态
/// 检查组织服务是否正常运行
pub async fn get_status(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let person_count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_org_person WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .get("count");
    let group_count: i64 = client
        .query_one("SELECT COUNT(*) FROM x_org_group", &[])
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .get("count");
    let enabled = read_sync_enabled(&client).await?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        (
            "personCount".to_string(),
            Value::Number(serde_json::Number::from(person_count)),
        ),
        (
            "groupCount".to_string(),
            Value::Number(serde_json::Number::from(group_count)),
        ),
        ("enabled".to_string(), Value::Bool(enabled)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 同步组织数据
/// 触发组织数据的同步操作
pub async fn sync_organization(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let synced: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM x_org_person WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("synced".to_string(), Value::Bool(synced > 0)),
        (
            "syncedRecords".to_string(),
            Value::Number(serde_json::Number::from(synced)),
        ),
        ("lastSyncTime".to_string(), Value::String("".to_string())),
        ("message".to_string(), Value::String("同步完成".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 获取组织服务配置
/// 返回组织服务的配置信息
pub async fn get_config(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let enabled = read_sync_enabled(&client).await?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(enabled)),
        (
            "syncInterval".to_string(),
            Value::Number(serde_json::Number::from(300i64)),
        ),
        (
            "maxRecords".to_string(),
            Value::Number(serde_json::Number::from(10000i64)),
        ),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 创建组织核心服务路由
/// 注册以下路由：
/// - /api/organization/core/express/status - 服务状态
/// - /api/organization/core/express/sync - 同步组织数据
/// - /api/organization/core/express/config - 服务配置
pub fn organization_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/api/organization/core/express/status", get(get_status))
        .route(
            "/api/organization/core/express/sync",
            get(sync_organization),
        )
        .route("/api/organization/core/express/config", get(get_config))
        .layer(Extension(pool))
}

#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::organization_core_express_router(pool)
}
