// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[allow(dead_code, non_snake_case)]
use axum::{extract::Extension, Json, Router};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;
pub mod u2;

#[cfg(test)]
mod security_gate_tests_b70;
#[cfg(test)]
mod security_gate_tests_r6;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

#[cfg(test)]
mod tests_manual;
#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;

    let row = client
        .query_one(
            "SELECT config_key, config_value FROM cal_control_config WHERE config_key = 'global' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .ok();

    let (enabled, default_time_zone, allow_sharing) = match row {
        Some(r) => {
            let cfg: Option<String> = r.get("config_value");
            match cfg {
                Some(ref s) if s.contains("\"enabled\":true") => (true, "UTC+8".to_string(), true),
                Some(ref s) if s.contains("\"enabled\":false") => {
                    (false, "UTC+8".to_string(), false)
                }
                _ => (true, "UTC+8".to_string(), true),
            }
        }
        None => (true, "UTC+8".to_string(), true),
    };

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(enabled)),
        (
            "defaultTimeZone".to_string(),
            Value::String(default_time_zone),
        ),
        ("allowSharing".to_string(), Value::Bool(allow_sharing)),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_calendars(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;

    let rows = client
        .query(
            "SELECT id, name, type, is_public, status FROM CAL_CALENDAR WHERE status = 'OPEN' ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let mut calendars = Vec::new();
    for row in rows.iter() {
        let cal_type: String = row.get::<_, Option<String>>("type").unwrap_or_default();
        let is_public: bool = row.get("is_public");
        calendars.push(Value::Object(serde_json::Map::from_iter([
            (
                "id".to_string(),
                Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
            ),
            (
                "name".to_string(),
                Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
            ),
            ("type".to_string(), Value::String(cal_type)),
            ("enabled".to_string(), Value::Bool(is_public)),
        ])));
    }

    let count = calendars.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(calendars),
        count,
        0,
    )))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 全局日历配置写（enabled / 时区 / 共享开关）属管理面：无会话 401、非 admin 403。
    match session.as_deref() {
        None => return Err(AppError::Unauthorized),
        Some(s) if !shared::middleware::is_admin(&pool, &s.person_unique).await => {
            return Err(AppError::Forbidden)
        }
        Some(_) => {}
    }
    let config = body.0;
    let client = shared::db::acquire(&pool).await?;

    let enabled = config
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let default_time_zone = config
        .get("defaultTimeZone")
        .and_then(|v| v.as_str())
        .unwrap_or("UTC+8")
        .to_string();
    let allow_sharing = config
        .get("allowSharing")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let config_value = serde_json::json!({
        "enabled": enabled,
        "defaultTimeZone": default_time_zone,
        "allowSharing": allow_sharing,
    });

    // `cal_control_config` 只有 `id` 主键，没有 `config_key` 唯一索引；
    // 旧 SQL 的 `ON CONFLICT (config_key)` → E42P10，且未写 NOT NULL 的 id。
    // 改为 UPDATE-then-INSERT（`config_key='global'` 单行幂等）；`update_time` 是 text，
    // 必须 `NOW()::text`（timestamptz 不能隐式赋给 text）。
    let config_key = "global";
    let config_text = config_value.to_string();
    let updated = client
        .execute(
            "UPDATE cal_control_config SET config_value = $1, update_time = NOW()::text WHERE config_key = $2",
            &[&config_text, &config_key],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    let result = if updated > 0 {
        updated
    } else {
        client
            .execute(
                "INSERT INTO cal_control_config (id, config_key, config_value, create_time, update_time) \
                 VALUES ($1, $2, $3, NOW()::text, NOW()::text)",
                &[&uuid::Uuid::new_v4().to_string(), &config_key, &config_text],
            )
            .await
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
    };

    tracing::info!(
        "Updated calendar assemble control config: {:?}",
        config_value
    );

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(result > 0)),
            ("config".to_string(), config_value),
        ]),
    ))))
}

pub async fn get_calendar_detail(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;

    let calendar_row = client
        .query_one(
            "SELECT id, name, type, target, color, description, source, createor, is_public, status \
             FROM CAL_CALENDAR WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    // O9：日历详情含全部事件列表——私有日历仅属主/管理员可读
    // （原无校验且本路由长期因接线缺陷 500；接线修复后泄露面暴露，矩阵实证）
    let is_public: bool = calendar_row
        .get::<_, Option<bool>>("is_public")
        .unwrap_or(false);
    if !is_public {
        let createor: String = calendar_row
            .get::<_, Option<String>>("createor")
            .unwrap_or_default();
        shared::middleware::require_owner(&pool, &session, &createor).await?;
    }

    let event_rows = client
        .query(
            "SELECT id, calendar_id, title, content, location, start_time, end_time, all_day, visibility, status, createor \
             FROM CAL_EVENT WHERE calendar_id = $1 AND status = 'OPEN' ORDER BY start_time ASC LIMIT 100",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let events: Vec<Value> = event_rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            // O9：NULL 安全——id/all_day 等列可空，裸 get::<String> 遇 NULL 即 panic
            // （matrix run10 实证：fixture 未设 target/all_day → handler 崩）
            map.insert(
                "id".to_string(),
                Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
            );
            map.insert(
                "calendarId".to_string(),
                Value::String(
                    row.get::<_, Option<String>>("calendar_id")
                        .unwrap_or_default(),
                ),
            );
            map.insert(
                "title".to_string(),
                Value::String(row.get::<_, Option<String>>("title").unwrap_or_default()),
            );
            if let Some(content) = row.get::<_, Option<String>>("content") {
                map.insert("content".to_string(), Value::String(content));
            }
            if let Some(location) = row.get::<_, Option<String>>("location") {
                map.insert("location".to_string(), Value::String(location));
            }
            map.insert(
                "startTime".to_string(),
                Value::String(
                    row.get::<_, Option<String>>("start_time")
                        .unwrap_or_default(),
                ),
            );
            map.insert(
                "endTime".to_string(),
                Value::String(row.get::<_, Option<String>>("end_time").unwrap_or_default()),
            );
            map.insert(
                "allDay".to_string(),
                Value::Bool(row.get::<_, Option<bool>>("all_day").unwrap_or(false)),
            );
            map.insert(
                "visibility".to_string(),
                Value::String(
                    row.get::<_, Option<String>>("visibility")
                        .unwrap_or_default(),
                ),
            );
            map.insert(
                "status".to_string(),
                Value::String(row.get::<_, Option<String>>("status").unwrap_or_default()),
            );
            Value::Object(map)
        })
        .collect();

    let mut data_map = serde_json::Map::new();
    data_map.insert(
        "id".to_string(),
        Value::String(
            calendar_row
                .get::<_, Option<String>>("id")
                .unwrap_or_default(),
        ),
    );
    data_map.insert(
        "name".to_string(),
        Value::String(
            calendar_row
                .get::<_, Option<String>>("name")
                .unwrap_or_default(),
        ),
    );
    data_map.insert(
        "type".to_string(),
        Value::String(
            calendar_row
                .get::<_, Option<String>>("type")
                .unwrap_or_default(),
        ),
    );
    data_map.insert(
        "target".to_string(),
        Value::String(
            calendar_row
                .get::<_, Option<String>>("target")
                .unwrap_or_default(),
        ),
    );
    data_map.insert(
        "color".to_string(),
        Value::String(
            calendar_row
                .get::<_, Option<String>>("color")
                .unwrap_or_default(),
        ),
    );
    if let Some(description) = calendar_row.get::<_, Option<String>>("description") {
        data_map.insert("description".to_string(), Value::String(description));
    }
    data_map.insert(
        "createor".to_string(),
        Value::String(
            calendar_row
                .get::<_, Option<String>>("createor")
                .unwrap_or_default(),
        ),
    );
    data_map.insert(
        "isPublic".to_string(),
        Value::Bool(calendar_row.get("is_public")),
    );
    data_map.insert(
        "status".to_string(),
        Value::String(
            calendar_row
                .get::<_, Option<String>>("status")
                .unwrap_or_default(),
        ),
    );
    data_map.insert(
        "eventCount".to_string(),
        Value::Number(serde_json::Number::from(events.len() as i64)),
    );
    data_map.insert("events".to_string(), Value::Array(events));
    let data = Value::Object(data_map);

    Ok(Json(ActionResult::success(data)))
}

pub fn calendar_assemble_control_router(pool: Pool) -> Router {
    // plan002 U2 的 7 条新路由注册于 routes::router 内部（Extension 层之前），
    // 确保共享连接池扩展对全部路由可见。
    // O9：detail 路由已挪入 routes::router 的 layer 之前——此前在 layer 之后
    // 追加注册，Extension<Pool> 对 detail 不可见，生产恒定 500。
    crate::routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::calendar_assemble_control_router(pool)
}
