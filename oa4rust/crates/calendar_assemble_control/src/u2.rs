//! plan002 U2 — calendar_assemble_control 端点闭合（对照 x_calendar_assemble_control
//! jaxrs 全集 31 条，第 1 批 7 条见下文，本文件补齐剩余 24 条）。
//!
//! 分层约定（沿用 bbs/cms U2 先例）：
//! - 读操作公开（list/public、{id} 详情、setting 列表、follow 查询）；
//! - 个人资源（list/my、follow、manager）需要会话；管理判定走 shared::middleware::is_admin；
//! - 写/删操作做 IDOR 门禁：仅资源属主或管理员可操作；
//! - 全部走真实参数化 SQL，复用既有 cal_calendar / cal_event / cal_setting 等表，
//!   关注关系、留言、事件重复分组由 migrations/085_calendar_mind_u2_columns.sql 补齐。

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

fn event_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    json!({
        "id": row.get::<_, String>("id"),
        "calendarId": row.get::<_, Option<String>>("calendar_id").unwrap_or_default(),
        "title": row.get::<_, Option<String>>("title").unwrap_or_default(),
        "content": row.get::<_, Option<String>>("content"),
        "location": row.get::<_, Option<String>>("location"),
        "startTime": row.get::<_, Option<String>>("start_time"),
        "endTime": row.get::<_, Option<String>>("end_time"),
        "allDay": row.get::<_, bool>("all_day"),
        "visibility": row.get::<_, Option<String>>("visibility").unwrap_or_default(),
        "status": row.get::<_, Option<String>>("status").unwrap_or_default(),
        "createor": row.get::<_, Option<String>>("createor").unwrap_or_default(),
    })
}

async fn require_person_is_manager(pool: &Pool, person_unique: &str) -> Result<bool, AppError> {
    Ok(shared::middleware::is_admin(pool, person_unique).await)
}

/// 校验请求人是否为日历属主或管理员（IDOR 门禁）。
async fn person_can_manage_calendar(
    pool: &Pool,
    person: &str,
    calendar_id: &str,
) -> Result<bool, AppError> {
    if require_person_is_manager(pool, person).await? {
        return Ok(true);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT createor FROM cal_calendar WHERE id = $1",
            &[&calendar_id.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(match row {
        Some(r) => {
            let owner: Option<String> = r.get("createor");
            owner.map(|v| v == person).unwrap_or(false)
        }
        None => false,
    })
}

/// 校验请求人是否为事件属主或管理员（IDOR 门禁）。
async fn person_can_manage_event(
    pool: &Pool,
    person: &str,
    event_id: &str,
) -> Result<bool, AppError> {
    if require_person_is_manager(pool, person).await? {
        return Ok(true);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT createor FROM cal_event WHERE id = $1",
            &[&event_id.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(match row {
        Some(r) => {
            let owner: Option<String> = r.get("createor");
            owner.map(|v| v == person).unwrap_or(false)
        }
        None => false,
    })
}

// ═══════════════════════════ 第 1 批（既有 7 条）═════════════════════════

/// GET /jaxrs/calendar_assemble_control/calendar/list/my —— 我能访问的日历（个人域）
pub async fn calendar_list_my(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
        let is_unit = r.get::<_, String>("type").eq_ignore_ascii_case("UNIT");
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
    let total_items = items.len();
    Ok(Json(ActionResult::java_success(Value::Array(items), total_items as i64, 0)))
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
        Some(r) => Ok(Json(ActionResult::success(event_row_to_value(&r)))),
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
    let total_items = items.len();
    Ok(Json(ActionResult::java_success(Value::Array(items), total_items as i64, 0)))
}

/// GET /jaxrs/calendar_assemble_control/setting/ismanager —— 当前用户是否设置管理员
pub async fn setting_ismanager(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> ApiResult {
    let is_manager = require_person_is_manager(&pool, &session.person_unique).await?;
    Ok(Json(ActionResult::success(json!({ "value": is_manager }))))
}

// ═══════════════════════════ 第 2 批（补齐 24 条）═════════════════════════

/// POST /jaxrs/calendar_assemble_control/calendar —— 创建日历
pub async fn calendar_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let cal_type = body.get("type").and_then(|v| v.as_str()).unwrap_or("PERSONAL").to_string();
    let target = body.get("target").and_then(|v| v.as_str()).map(|s| s.to_string());
    let color = body.get("color").and_then(|v| v.as_str()).map(|s| s.to_string());
    let description = body.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let is_public = body.get("isPublic").and_then(|v| v.as_bool()).unwrap_or(false);
    let person = session.person_unique.clone();

    client
        .execute(
            "INSERT INTO cal_calendar (id, name, type, is_public, status, target, color, \
             description, createor, create_time) \
             VALUES ($1, $2, $3, $4, 'OPEN', $5, $6, $7, $8, NOW())",
            &[
                &id,
                &name,
                &cal_type,
                &is_public,
                &target,
                &color,
                &description,
                &person,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({
        "id": id,
        "name": name,
        "type": cal_type,
        "isPublic": is_public,
        "createor": person,
    }))))
}

/// GET /jaxrs/calendar_assemble_control/calendar/follow/{id} —— 当前用户是否关注该日历
pub async fn calendar_follow_get(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) AS c FROM cal_calendar_follow WHERE calendar_id = $1 AND person = $2",
            &[&id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("c");
    Ok(Json(ActionResult::success(json!({ "followed": count > 0 }))))
}

/// GET /jaxrs/calendar_assemble_control/calendar/follow/{id}/cancel —— 取消关注
pub async fn calendar_follow_cancel(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "DELETE FROM cal_calendar_follow WHERE calendar_id = $1 AND person = $2",
            &[&id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "canceled": n > 0 }))))
}

/// GET /jaxrs/calendar_assemble_control/calendar/ismanager/calendar/{id} —— 当前用户是否某日历管理员
pub async fn calendar_ismanager_calendar(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let allowed = person_can_manage_calendar(&pool, &session.person_unique, &id).await?;
    Ok(Json(ActionResult::success(json!({ "value": allowed }))))
}

/// PUT /jaxrs/calendar_assemble_control/calendar/list/filter —— 按条件过滤日历
pub async fn calendar_list_filter(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name: Option<String> = body.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let cal_type: Option<String> = body.get("type").and_then(|v| v.as_str()).map(|s| s.to_string());
    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM cal_calendar \
             WHERE status = 'OPEN' \
               AND ($1::text IS NULL OR name ILIKE '%' || $1 || '%') \
               AND ($2::text IS NULL OR type = $2) \
             ORDER BY create_time DESC LIMIT 200",
            &[&name, &cal_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows.iter().map(calendar_row_to_value).collect();
    let total_items = items.len();
    Ok(Json(ActionResult::java_success(Value::Array(items), total_items as i64, 0)))
}

/// GET /jaxrs/calendar_assemble_control/calendar/manager/list/with/person/{id} —— 含某人的日历管理员列表
pub async fn calendar_manager_list_with_person(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, createor, creator_person FROM cal_calendar WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let managers: Vec<Value> = match row {
        Some(r) => vec![json!({
            "calendarId": r.get::<_, String>("id"),
            "calendarName": r.get::<_, String>("name"),
            "person": r.get::<_, Option<String>>("createor").unwrap_or_default(),
            "creatorPerson": r.get::<_, Option<String>>("creator_person").unwrap_or_default(),
        })],
        None => vec![],
    };
    let total_managers = managers.len();
    Ok(Json(ActionResult::java_success(Value::Array(managers), total_managers as i64, 0)))
}

/// DELETE /jaxrs/calendar_assemble_control/calendar/{id} —— 删除日历（IDOR 门禁）
pub async fn calendar_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_calendar(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not calendar owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM cal_calendar WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("calendar not found")));
    }
    Ok(Json(ActionResult::success(json!({ "deleted": n }))))
}

/// POST /jaxrs/calendar_assemble_control/event —— 创建事件
pub async fn event_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let calendar_id = body.get("calendarId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let title = body.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let content = body.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let location = body.get("location").and_then(|v| v.as_str()).map(|s| s.to_string());
    let start_time = body.get("startTime").and_then(|v| v.as_str()).map(|s| s.to_string());
    let end_time = body.get("endTime").and_then(|v| v.as_str()).map(|s| s.to_string());
    let all_day = body.get("allDay").and_then(|v| v.as_bool()).unwrap_or(false);
    let visibility = body.get("visibility").and_then(|v| v.as_str()).map(|s| s.to_string());
    let master_id = body.get("masterId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let person = session.person_unique.clone();

    client
        .execute(
            "INSERT INTO cal_event (id, calendar_id, title, content, location, start_time, \
             end_time, all_day, visibility, status, createor, master_id, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'OPEN', $10, $11, NOW())",
            &[
                &id,
                &calendar_id,
                &title,
                &content,
                &location,
                &start_time,
                &end_time,
                &all_day,
                &visibility,
                &person,
                &master_id,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({
        "id": id,
        "calendarId": calendar_id,
        "title": title,
        "createor": person,
    }))))
}

/// DELETE /jaxrs/calendar_assemble_control/event/after/{id} —— 删除该事件之后（含）的所有重复实例
pub async fn event_delete_after(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_event(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not event owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let anchor = client
        .query_opt(
            "SELECT master_id, start_time FROM cal_event WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let (master_id, start_time): (Option<String>, Option<String>) = match anchor {
        Some(r) => (r.get("master_id"), r.get("start_time")),
        None => return Ok(Json(ActionResult::error("event not found"))),
    };
    let n = client
        .execute(
            "DELETE FROM cal_event \
             WHERE status = 'OPEN' \
               AND ($1::text IS NULL OR master_id = $1) \
               AND ($2::text IS NULL OR start_time >= $2)",
            &[&master_id, &start_time],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "deleted": n }))))
}

/// DELETE /jaxrs/calendar_assemble_control/event/all/{id} —— 删除该事件全部重复实例
pub async fn event_delete_all(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_event(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not event owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let master_id: Option<String> = client
        .query_opt("SELECT master_id FROM cal_event WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?
        .and_then(|r| r.get("master_id"));
    let n = client
        .execute(
            "DELETE FROM cal_event WHERE status = 'OPEN' AND ($1::text IS NULL OR master_id = $1)",
            &[&master_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "deleted": n }))))
}

/// PUT /jaxrs/calendar_assemble_control/event/list/filter —— 事件过滤列表
pub async fn event_list_filter(pool: Extension<Pool>, Json(body): Json<Value>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let calendar_id: Option<String> = body.get("calendarId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let title: Option<String> = body.get("title").and_then(|v| v.as_str()).map(|s| s.to_string());
    let rows = client
        .query(
            "SELECT id, calendar_id, title, content, location, start_time, end_time, \
             all_day, visibility, status, createor \
             FROM cal_event \
             WHERE status = 'OPEN' \
               AND ($1::text IS NULL OR calendar_id = $1) \
               AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%') \
             ORDER BY start_time ASC LIMIT 200",
            &[&calendar_id, &title],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows.iter().map(event_row_to_value).collect();
    let total_items = items.len();
    Ok(Json(ActionResult::java_success(Value::Array(items), total_items as i64, 0)))
}

/// PUT /jaxrs/calendar_assemble_control/event/list/filter/sample —— 事件过滤样例（限量）
pub async fn event_list_filter_sample(pool: Extension<Pool>, Json(body): Json<Value>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let calendar_id: Option<String> = body.get("calendarId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let rows = client
        .query(
            "SELECT id, calendar_id, title, start_time, status, createor \
             FROM cal_event \
             WHERE status = 'OPEN' AND ($1::text IS NULL OR calendar_id = $1) \
             ORDER BY start_time ASC LIMIT 10",
            &[&calendar_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, String>("id"),
                "calendarId": r.get::<_, Option<String>>("calendar_id").unwrap_or_default(),
                "title": r.get::<_, Option<String>>("title").unwrap_or_default(),
                "startTime": r.get::<_, Option<String>>("start_time"),
            })
        })
        .collect();
    let total_items = items.len();
    Ok(Json(ActionResult::java_success(Value::Array(items), total_items as i64, 0)))
}

/// POST /jaxrs/calendar_assemble_control/event/list/filter/sample/manager —— 管理视角的事件样例
pub async fn event_list_filter_sample_manager(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> ApiResult {
    let is_manager = require_person_is_manager(&pool, &session.person_unique).await?;
    if !is_manager {
        return Ok(Json(ActionResult::error("forbidden: manager only")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let calendar_id: Option<String> = body.get("calendarId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let rows = client
        .query(
            "SELECT id, calendar_id, title, start_time, createor \
             FROM cal_event \
             WHERE status = 'OPEN' AND ($1::text IS NULL OR calendar_id = $1) \
             ORDER BY start_time ASC LIMIT 10",
            &[&calendar_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, String>("id"),
                "calendarId": r.get::<_, Option<String>>("calendar_id").unwrap_or_default(),
                "title": r.get::<_, Option<String>>("title").unwrap_or_default(),
                "createor": r.get::<_, Option<String>>("createor").unwrap_or_default(),
            })
        })
        .collect();
    let total_items = items.len();
    Ok(Json(ActionResult::java_success(Value::Array(items), total_items as i64, 0)))
}

/// POST /jaxrs/calendar_assemble_control/event/manage —— 事件管理（更新状态/可见性）
pub async fn event_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> ApiResult {
    let id = body.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    if id.is_empty() {
        return Ok(Json(ActionResult::error("id is required")));
    }
    if !person_can_manage_event(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not event owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let status = body.get("status").and_then(|v| v.as_str()).map(|s| s.to_string());
    let visibility = body.get("visibility").and_then(|v| v.as_str()).map(|s| s.to_string());
    let n = client
        .execute(
            "UPDATE cal_event SET status = COALESCE($2, status), visibility = COALESCE($3, visibility) \
             WHERE id = $1 AND status = 'OPEN'",
            &[&id, &status, &visibility],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("event not found")));
    }
    Ok(Json(ActionResult::success(json!({ "updated": n }))))
}

/// GET /jaxrs/calendar_assemble_control/event/rfc/{id} —— 返回事件 RFC2445(iCal) 文本
pub async fn event_rfc(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, start_time, end_time, location, rfc_text \
             FROM cal_event WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => {
            let rfc: Option<String> = r.get("rfc_text");
            let rfc = rfc.unwrap_or_else(|| {
                let title: Option<String> = r.get("title");
                let start: Option<String> = r.get("start_time");
                let end: Option<String> = r.get("end_time");
                let loc: Option<String> = r.get("location");
                format!(
                    "BEGIN:VCALENDAR\r\nBEGIN:VEVENT\r\nSUMMARY:{}\r\nDTSTART:{}\r\nDTEND:{}\r\nLOCATION:{}\r\nEND:VEVENT\r\nEND:VCALENDAR",
                    title.unwrap_or_default(),
                    start.unwrap_or_default(),
                    end.unwrap_or_default(),
                    loc.unwrap_or_default()
                )
            });
            Ok(Json(ActionResult::success(json!({ "id": id, "rfc": rfc }))))
        }
        None => Ok(Json(ActionResult::error("event not found"))),
    }
}

/// DELETE /jaxrs/calendar_assemble_control/event/single/{id} —— 删除单个事件
pub async fn event_delete_single(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_event(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not event owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM cal_event WHERE id = $1 AND status = 'OPEN'", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("event not found")));
    }
    Ok(Json(ActionResult::success(json!({ "deleted": n }))))
}

/// PUT /jaxrs/calendar_assemble_control/event/update/after/{id} —— 更新该事件之后（含）的重复实例
pub async fn event_update_after(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> ApiResult {
    if !person_can_manage_event(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not event owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let anchor = client
        .query_opt("SELECT master_id, start_time FROM cal_event WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    let (master_id, start_time): (Option<String>, Option<String>) = match anchor {
        Some(r) => (r.get("master_id"), r.get("start_time")),
        None => return Ok(Json(ActionResult::error("event not found"))),
    };
    let title = body.get("title").and_then(|v| v.as_str()).map(|s| s.to_string());
    let location = body.get("location").and_then(|v| v.as_str()).map(|s| s.to_string());
    let n = client
        .execute(
            "UPDATE cal_event SET title = COALESCE($3, title), location = COALESCE($4, location) \
             WHERE status = 'OPEN' \
               AND ($1::text IS NULL OR master_id = $1) \
               AND ($2::text IS NULL OR start_time >= $2)",
            &[&master_id, &start_time, &title, &location],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "updated": n }))))
}

/// PUT /jaxrs/calendar_assemble_control/event/update/all/{id} —— 更新该事件全部重复实例
pub async fn event_update_all(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> ApiResult {
    if !person_can_manage_event(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not event owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let master_id: Option<String> = client
        .query_opt("SELECT master_id FROM cal_event WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?
        .and_then(|r| r.get("master_id"));
    let title = body.get("title").and_then(|v| v.as_str()).map(|s| s.to_string());
    let location = body.get("location").and_then(|v| v.as_str()).map(|s| s.to_string());
    let n = client
        .execute(
            "UPDATE cal_event SET title = COALESCE($2, title), location = COALESCE($3, location) \
             WHERE status = 'OPEN' AND ($1::text IS NULL OR master_id = $1)",
            &[&master_id, &title, &location],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "updated": n }))))
}

/// PUT /jaxrs/calendar_assemble_control/event/update/single/{id} —— 更新单个事件
pub async fn event_update_single(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> ApiResult {
    if !person_can_manage_event(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not event owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let title = body.get("title").and_then(|v| v.as_str()).map(|s| s.to_string());
    let content = body.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let location = body.get("location").and_then(|v| v.as_str()).map(|s| s.to_string());
    let start_time = body.get("startTime").and_then(|v| v.as_str()).map(|s| s.to_string());
    let end_time = body.get("endTime").and_then(|v| v.as_str()).map(|s| s.to_string());
    let n = client
        .execute(
            "UPDATE cal_event SET title = COALESCE($2, title), content = COALESCE($3, content), \
             location = COALESCE($4, location), start_time = COALESCE($5, start_time), \
             end_time = COALESCE($6, end_time) WHERE id = $1 AND status = 'OPEN'",
            &[&id, &title, &content, &location, &start_time, &end_time],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("event not found")));
    }
    Ok(Json(ActionResult::success(json!({ "updated": n }))))
}

/// POST /jaxrs/calendar_assemble_control/message —— 创建留言
pub async fn message_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let calendar_id = body.get("calendarId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let title = body.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let msg_body = body.get("body").and_then(|v| v.as_str()).map(|s| s.to_string());
    let person = session.person_unique.clone();
    client
        .execute(
            "INSERT INTO cal_message (id, calendar_id, title, body, person, create_time) \
             VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &calendar_id, &title, &msg_body, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({
        "id": id,
        "calendarId": calendar_id,
        "title": title,
        "person": person,
    }))))
}

/// POST /jaxrs/calendar_assemble_control/setting —— 创建设置
pub async fn setting_create(pool: Extension<Pool>, Json(body): Json<Value>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let code = body.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let name = body.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let description = body.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let value = body.get("value").and_then(|v| v.as_str()).map(|s| s.to_string());
    let order_no = body.get("order").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let person = body.get("person").and_then(|v| v.as_str()).map(|s| s.to_string());
    client
        .execute(
            "INSERT INTO cal_setting (id, code, name, description, value, order_no, person, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&id, &code, &name, &description, &value, &order_no, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({
        "id": id,
        "code": code,
        "name": name,
    }))))
}

/// GET /jaxrs/calendar_assemble_control/setting/code/{code} —— 按 code 查询设置
pub async fn setting_get_by_code(pool: Extension<Pool>, Path(code): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, code, name, description, value, order_no, person \
             FROM cal_setting WHERE code = $1",
            &[&code],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(json!({
            "id": r.get::<_, String>("id"),
            "code": r.get::<_, String>("code"),
            "name": r.get::<_, Option<String>>("name").unwrap_or_default(),
            "description": r.get::<_, Option<String>>("description").unwrap_or_default(),
            "value": r.get::<_, Option<String>>("value").unwrap_or_default(),
            "order": r.get::<_, i32>("order_no"),
        })))),
        None => Ok(Json(ActionResult::error("setting not found"))),
    }
}

/// GET /jaxrs/calendar_assemble_control/setting/{id} —— 按 id 查询设置
pub async fn setting_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, code, name, description, value, order_no, person \
             FROM cal_setting WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(json!({
            "id": r.get::<_, String>("id"),
            "code": r.get::<_, String>("code"),
            "name": r.get::<_, Option<String>>("name").unwrap_or_default(),
            "description": r.get::<_, Option<String>>("description").unwrap_or_default(),
            "value": r.get::<_, Option<String>>("value").unwrap_or_default(),
            "order": r.get::<_, i32>("order_no"),
        })))),
        None => Ok(Json(ActionResult::error("setting not found"))),
    }
}

/// GET /jaxrs/calendar_assemble_control/test/1 —— 连通性自检
pub async fn test_1(pool: Extension<Pool>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let ok: bool = client
        .query_one("SELECT 1 AS v", &[])
        .await
        .map(|r| r.get::<_, i32>("v") == 1)
        .unwrap_or(false);
    Ok(Json(ActionResult::success(json!({ "ok": ok }))))
}
