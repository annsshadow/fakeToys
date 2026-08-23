//! plan002 U2 — calendar_assemble_control 端点闭合（对照 x_calendar_assemble_control
//! jaxrs 全集 31 条中缺失的 7 条补齐）。
//!
//! 分层约定（沿用 bbs/cms U2 先例）：
//! - 读操作公开（list/public、{id} 详情、setting 列表）；
//! - 个人资源（list/my）与管理判定（ismanager）需要会话；
//!   管理判定走 shared::middleware::is_admin。
//! - 表：既有表优先（cal_calendar/cal_event），setting 域新表 cal_setting
//!   由 migrations/074_u2_small_modules.sql 幂等创建。

use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::{error::AppError, response::ActionResult};

type ApiResult = Result<Json<ActionResult<Value>>, AppError>;

fn calendar_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    let mut map = serde_json::Map::new();
    map.insert("id".into(), Value::String(row.get("id")));
    map.insert("name".into(), Value::String(row.get("name")));
    map.insert("type".into(), Value::String(row.get("type")));
    map.insert(
        "target".into(),
        row.get::<_, Option<String>>("target")
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    map.insert(
        "color".into(),
        row.get::<_, Option<String>>("color")
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    map.insert(
        "description".into(),
        row.get::<_, Option<String>>("description")
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    map.insert("createor".into(), Value::String(row.get("createor")));
    map.insert("isPublic".into(), Value::Bool(row.get("is_public")));
    map.insert("status".into(), Value::String(row.get("status")));
    Value::Object(map)
}

async fn require_person_is_manager(
    pool: &Pool,
    person_unique: &str,
) -> Result<bool, AppError> {
    Ok(shared::middleware::is_admin(pool, person_unique).await)
}

/// GET /jaxrs/calendar_assemble_control/calendar/list/my —— 我能访问的日历（个人域）
pub async fn calendar_list_my(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // 本人创建的日历 + 创建人为空但 target 指向本人的开放日历
    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM cal_calendar \
             WHERE status = 'OPEN' AND (createor = $1 OR target = $1) \
             ORDER BY create_time DESC",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut my_calendars = Vec::new();
    let mut unit_calendars = Vec::new();
    for r in rows.iter() {
        let v = calendar_row_to_value(r);
        let is_unit = r
            .get::<_, String>("type")
            .eq_ignore_ascii_case("UNIT");
        if is_unit {
            unit_calendars.push(v);
        } else {
            my_calendars.push(v);
        }
    }

    Ok(Json(ActionResult::success(json!({
        "myCalendars": my_calendars,
        "unitCalendars": unit_calendars,
    }))))
}

/// GET /jaxrs/calendar_assemble_control/calendar/list/public —— 所有公开日历
pub async fn calendar_list_public(pool: Extension<Pool>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM cal_calendar WHERE status = 'OPEN' AND is_public = true \
             ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows.iter().map(calendar_row_to_value).collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

/// GET /jaxrs/calendar_assemble_control/calendar/{id} —— 按 ID 获取日历信息
pub async fn calendar_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM cal_calendar WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(calendar_row_to_value(&r)))),
        None => Ok(Json(ActionResult::error("calendar not found"))),
    }
}

/// GET /jaxrs/calendar_assemble_control/calendar/ismanager —— 当前用户是否管理员
pub async fn calendar_ismanager(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> ApiResult {
    let is_manager = require_person_is_manager(&pool, &session.person_unique).await?;
    Ok(Json(ActionResult::success(json!({ "value": is_manager }))))
}

/// GET /jaxrs/calendar_assemble_control/event/{id} —— 按 ID 获取日程事件
pub async fn event_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, calendar_id, title, content, location, start_time, end_time, \
             all_day, visibility, status, createor \
             FROM cal_event WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => {
            let data = json!({
                "id": r.get::<_, String>("id"),
                "calendarId": r.get::<_, String>("calendar_id"),
                "title": r.get::<_, String>("title"),
                "content": r.get::<_, Option<String>>("content"),
                "location": r.get::<_, Option<String>>("location"),
                "startTime": r.get::<_, Option<String>>("start_time"),
                "endTime": r.get::<_, Option<String>>("end_time"),
                "allDay": r.get::<_, bool>("all_day"),
                "visibility": r.get::<_, Option<String>>("visibility").unwrap_or_default(),
                "status": r.get::<_, Option<String>>("status").unwrap_or_default(),
                "createor": r.get::<_, Option<String>>("createor").unwrap_or_default(),
            });
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("event not found"))),
    }
}

/// GET /jaxrs/calendar_assemble_control/setting/list/all —— 日历设置列表
pub async fn setting_list_all(pool: Extension<Pool>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, code, name, description, value, order_no, person \
             FROM cal_setting ORDER BY order_no ASC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let items: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, String>("id"),
                "code": r.get::<_, String>("code"),
                "name": r.get::<_, Option<String>>("name").unwrap_or_default(),
                "description": r.get::<_, Option<String>>("description").unwrap_or_default(),
                "value": r.get::<_, Option<String>>("value").unwrap_or_default(),
                "order": r.get::<_, i32>("order_no"),
                "person": r.get::<_, Option<String>>("person").unwrap_or_default(),
            })
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

/// GET /jaxrs/calendar_assemble_control/setting/ismanager —— 当前用户是否设置管理员
pub async fn setting_ismanager(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> ApiResult {
    let is_manager = require_person_is_manager(&pool, &session.person_unique).await?;
    Ok(Json(ActionResult::success(json!({ "value": is_manager }))))
}
