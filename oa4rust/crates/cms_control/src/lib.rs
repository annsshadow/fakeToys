// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{extract::Extension, Json, Router};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT enabled, max_category_count, allow_anonymous FROM x_cms_control_config ORDER BY create_time LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        (
            "enabled".to_string(),
            Value::Bool(row.get::<_, Option<bool>>("enabled").unwrap_or(false)),
        ),
        (
            "maxCategoryCount".to_string(),
            Value::Number(serde_json::Number::from(
                row.get::<_, i64>("max_category_count"),
            )),
        ),
        (
            "allowAnonymous".to_string(),
            Value::Bool(
                row.get::<_, Option<bool>>("allow_anonymous")
                    .unwrap_or(false),
            ),
        ),
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
            "SELECT id, name, enabled FROM x_cms_control_section ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let sections: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "enabled".to_string(),
                    Value::Bool(row.get::<_, Option<bool>>("enabled").unwrap_or(false)),
                ),
            ]))
        })
        .collect();

    let total_sections = sections.len();
    Ok(Json(ActionResult::legacy_success(
        Value::Array(sections),
        total_sections as i64,
        0,
    )))
}

pub fn cms_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_control_router(pool)
}
