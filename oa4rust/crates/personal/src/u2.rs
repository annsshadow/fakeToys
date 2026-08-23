//! plan002 U2 收尾：对齐 Java x_organization_assemble_personal 残余端点。
//!
//! 路径约定：沿用本仓库既有前缀 `/jaxrs/person/**`（Java war 前缀为
//! `/x_organization_assemble_personal/jaxrs/**`，类路径逐段映射）。
//!
//! 覆盖组：
//! - CustomAction        8 端点（个性化数据，x_custom）
//! - DefinitionAction    5 端点（全局自定义数据，x_org_definition，migration 077）
//! - EmpowerAction       补齐 list/{id}/next|prev、list/person/{flag} 与 mock 别名
//! - EmpowerLogAction    7 端点（x_org_empower_log，migration 077 补列）
//! - ExmailAction        6 端点（腾讯企业邮，配置驱动）
//! - Person/Password/Regist/Reset/Signature 的契约路径补齐

use axum::{
    extract::{Extension, Multipart, Path, Query},
    http::HeaderMap,
    Json,
};
use base64::Engine;
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::{
    error::AppError,
    middleware::is_admin,
    response::ActionResult,
};

use auth::SessionManager;

use crate::resolve_current_person_unique;
use crate::reset::ResetCodeStore;
use crate::reset::is_password_acceptable;

// ═════════════════════════════════════════════════════════════════════════════
// PersonAction 契约补齐：POST /person/mockputtopost、GET /person/icon、
// PUT /person/icon（multipart）、POST /person/icon（octet-stream）及其 mock 别名
// ═════════════════════════════════════════════════════════════════════════════

/// GET /jaxrs/person/icon —— 当前登录用户头像（Java ActionGetIcon）
pub async fn get_my_icon(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => {
            let icon: Option<String> = r.get("icon");
            let icon_str = icon.unwrap_or_default();
            Ok(Json(ActionResult::success(json!({
                "icon": icon_str,
                "exists": !icon_str.is_empty(),
            }))))
        }
        None => Ok(Json(ActionResult::error("用户不存在"))),
    }
}

/// POST /jaxrs/person/icon（application/octet-stream，Java ActionSetIconOctetStream）
///
/// 请求体即图片字节；base64 后写入当前用户 auth_person.icon。
pub async fn set_icon_octet_stream(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    bytes: axum::body::Bytes,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;
    if bytes.is_empty() {
        return Ok(Json(ActionResult::error("未提供头像内容")));
    }
    const MAX_ICON_SIZE: usize = 5 * 1024 * 1024;
    if bytes.len() > MAX_ICON_SIZE {
        return Ok(Json(ActionResult::error("文件大小超过限制（最大 5MB）")));
    }
    let encoded = base64::engine::general_purpose::STANDARD.encode(bytes.as_ref());
    write_icon(&pool, &person_unique, &encoded).await
}

/// PUT/POST multipart 头像设置复用 icon::upload（同一存储位置与校验规则）
pub async fn upload_multipart_alias(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    form: Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    crate::icon::upload(pool, session_manager, headers, form).await
}

async fn write_icon(
    pool: &Pool,
    person_unique: &str,
    encoded: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE auth_person SET icon = $1, updated_at = NOW() \
             WHERE unique_id = $2 AND locked = false AND deleted_at IS NULL",
            &[&encoded.to_string(), &person_unique.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({
        "icon": encoded,
        "exists": true,
    }))))
}

// ═════════════════════════════════════════════════════════════════════════════
// RegistAction 契约补齐：mode / captcha / code/mobile/{mobile} / check/password
// ═════════════════════════════════════════════════════════════════════════════

/// GET /jaxrs/person/regist/mode —— 注册开关（对齐 Config.person().getRegister()）
pub async fn regist_mode() -> Result<Json<ActionResult<String>>, AppError> {
    let enabled = std::env::var("PERSON_REGISTER")
        .map(|v| matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false);
    // Java Wo 为 WrapString："true"/"false"
    Ok(Json(ActionResult::success(enabled.to_string())))
}

/// GET /jaxrs/person/regist/code/mobile/{mobile} —— 发送注册验证码（短信渠道）
pub async fn regist_code_mobile(
    store: Extension<ResetCodeStore>,
    Path(mobile): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mobile = mobile.trim();
    if mobile.is_empty() || !mobile.chars().all(|c| c.is_ascii_digit()) {
        return Ok(Json(ActionResult::error("invalid mobile")));
    }
    let _plain = store.issue(mobile).await;
    Ok(Json(ActionResult::success(json!({ "message": "code sent" }))))
}

/// GET /jaxrs/person/regist/check/password/{password}
///
/// 对齐 Java：不满足密码策略时返回策略提示文案；满足时无 data。
pub async fn regist_check_password(
    Path(password): Path<String>,
) -> Result<Json<ActionResult<String>>, AppError> {
    if is_password_acceptable(&password) {
        return Ok(Json(ActionResult::success(String::new())));
    }
    Ok(Json(ActionResult::success(
        "密码需 6-64 位且至少包含一个字母和一个数字".to_string(),
    )))
}

/// GET /jaxrs/reset/mockputtopost 等 mock 别名直接注册既有 reset_password 处理器。

// ═════════════════════════════════════════════════════════════════════════════
// CustomAction —— 个性化数据（x_custom：name/person/value 列对应 Java name/person/data）
// ═════════════════════════════════════════════════════════════════════════════

const SIGNATURE_NAME_PREFIX: &str = "SIGNATURE_";

/// 归一化名称：trim；空名非法
fn normalize_name(name: &str) -> Option<String> {
    let n = name.trim();
    if n.is_empty() {
        None
    } else {
        Some(n.to_string())
    }
}

async fn custom_find(
    client: &deadpool_postgres::Client,
    person: &str,
    name: &str,
) -> Result<Option<(String, String)>, AppError> {
    let row = client
        .query_opt(
            "SELECT id, COALESCE(value, '') AS value FROM x_custom \
             WHERE person = $1 AND name = $2 AND deleted_at IS NULL \
             ORDER BY created_at DESC LIMIT 1",
            &[&person.to_string(), &name.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.map(|r| (r.get::<_, String>("id"), r.get::<_, String>("value"))))
}

/// GET /jaxrs/person/custom/{name} —— 当前用户指定名称数据
pub async fn custom_get(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<String>>, AppError> {
    let Some(name) = normalize_name(&name) else {
        return Ok(Json(ActionResult::error("name is required")));
    };
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    match custom_find(&client, &person, &name).await? {
        Some((_, value)) => Ok(Json(ActionResult::success(value))),
        None => Ok(Json(ActionResult::success(String::new()))),
    }
}

/// PUT /jaxrs/person/custom/{name} —— 更新（不存在则创建），返回 {id}
pub async fn custom_edit(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(name): Path<String>,
    body: String,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let Some(name) = normalize_name(&name) else {
        return Ok(Json(ActionResult::error("name is required")));
    };
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 归一化查重：同 (person,name) 仅保留一行（upsert）
    match custom_find(&client, &person, &name).await? {
        Some((id, _)) => {
            client
                .execute(
                    "UPDATE x_custom SET value = $1, updated_at = NOW() WHERE id = $2",
                    &[&body, &id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(json!({ "id": id }))))
        }
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_custom (id, name, person, value, created_at, updated_at) \
                     VALUES ($1, $2, $3, $4, NOW(), NOW())",
                    &[&id, &name, &person, &body],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(json!({ "id": id }))))
        }
    }
}

/// DELETE /jaxrs/person/custom/{name} —— 删除（软删），GET mockdeletetoget 同义
pub async fn custom_delete(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let Some(name) = normalize_name(&name) else {
        return Ok(Json(ActionResult::error("name is required")));
    };
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_custom SET deleted_at = NOW() \
             WHERE person = $1 AND name = $2 AND deleted_at IS NULL",
            &[&person, &name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "value": n > 0 }))))
}

/// 解析人员标识（unique_id / name / id）→ unique_id；不存在返回 None
async fn resolve_person_flag(
    client: &deadpool_postgres::Client,
    flag: &str,
) -> Result<Option<String>, AppError> {
    let row = client
        .query_opt(
            "SELECT unique_id FROM auth_person \
             WHERE (unique_id = $1 OR name = $1 OR id = $1) \
             AND locked = false AND deleted_at IS NULL",
            &[&flag.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.map(|r| r.get::<_, String>("unique_id")))
}

async fn require_admin(pool: &Pool, session_manager: &SessionManager, headers: &HeaderMap) -> Result<String, AppError> {
    let person = resolve_current_person_unique(session_manager, headers).await?;
    if !is_admin(pool, &person).await {
        return Err(AppError::Forbidden);
    }
    Ok(person)
}

/// GET /jaxrs/person/custom/manager/person/{person}/name/{name} —— 管理员读取
pub async fn custom_manager_get(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((person, name)): Path<(String, String)>,
) -> Result<Json<ActionResult<String>>, AppError> {
    require_admin(&pool, &session_manager, &headers).await?;
    let Some(name) = normalize_name(&name) else {
        return Ok(Json(ActionResult::error("name is required")));
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let Some(person_unique) = resolve_person_flag(&client, &person).await? else {
        return Ok(Json(ActionResult::error("person not found")));
    };
    match custom_find(&client, &person_unique, &name).await? {
        Some((_, value)) => Ok(Json(ActionResult::success(value))),
        None => Ok(Json(ActionResult::success(String::new()))),
    }
}

/// PUT /jaxrs/person/custom/manager/person/{person}/name/{name} —— 管理员更新
pub async fn custom_manager_edit(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((person, name)): Path<(String, String)>,
    body: String,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session_manager, &headers).await?;
    let Some(name) = normalize_name(&name) else {
        return Ok(Json(ActionResult::error("name is required")));
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let Some(person_unique) = resolve_person_flag(&client, &person).await? else {
        return Ok(Json(ActionResult::error("person not found")));
    };

    match custom_find(&client, &person_unique, &name).await? {
        Some((id, _)) => {
            client
                .execute(
                    "UPDATE x_custom SET value = $1, updated_at = NOW() WHERE id = $2",
                    &[&body, &id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(json!({ "id": id }))))
        }
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_custom (id, name, person, value, created_at, updated_at) \
                     VALUES ($1, $2, $3, $4, NOW(), NOW())",
                    &[&id, &name, &person_unique, &body],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(json!({ "id": id }))))
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════════
// DefinitionAction —— 全局自定义数据（x_org_definition）
// ═════════════════════════════════════════════════════════════════════════════

/// GET /jaxrs/person/definition/{name}
pub async fn definition_get(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<String>>, AppError> {
    resolve_current_person_unique(&session_manager, &headers).await?;
    let Some(name) = normalize_name(&name) else {
        return Ok(Json(ActionResult::error("name is required")));
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT COALESCE(data, '') FROM x_org_definition WHERE name = $1",
            &[&name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(
        row.map(|r| r.get::<_, String>(0)).unwrap_or_default(),
    )))
}

/// PUT /jaxrs/person/definition/{name} —— upsert（唯一索引保证同名单行）
pub async fn definition_edit(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(name): Path<String>,
    body: String,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    let Some(name) = normalize_name(&name) else {
        return Ok(Json(ActionResult::error("name is required")));
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // schema 漂移兼容：旧表可能存在本实现未覆盖的 NOT NULL 无默认值列；
    // 动态探测后以空串补位，保证 upsert 在新旧两种表形态下均可落库
    let extra_cols = client
        .query(
            "SELECT column_name FROM information_schema.columns \
             WHERE table_name = 'x_org_definition' \
               AND is_nullable = 'NO' AND column_default IS NULL \
               AND column_name NOT IN ('id', 'name', 'data', 'creator', \
                                       'create_time', 'update_time')",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let extra: Vec<String> = extra_cols.iter().map(|r| r.get::<_, String>(0)).collect();

    let id = uuid::Uuid::new_v4().to_string();
    let mut columns = vec![
        "id".to_string(),
        "name".to_string(),
        "data".to_string(),
        "creator".to_string(),
    ];
    let mut values: Vec<String> = vec![id.clone(), name.clone(), body.clone(), person.clone()];
    let mut placeholders = vec!["$1".to_string(), "$2".to_string(), "$3".to_string(), "$4".to_string()];
    for (i, col) in extra.iter().enumerate() {
        columns.push(col.clone());
        values.push(String::new());
        placeholders.push(format!("${}", i + 5));
    }
    let sql = format!(
        "INSERT INTO x_org_definition ({}) VALUES ({}) \
         ON CONFLICT (name) DO UPDATE SET data = EXCLUDED.data, update_time = NOW()",
        columns.join(", "),
        placeholders.join(", ")
    );
    let params: Vec<&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)> =
        values.iter().map(|v| v as _).collect();
    client
        .execute(&sql, &params)
        .await
        .map_err(|e| {
            let msg = e
                .as_db_error()
                .map(|d| d.message().to_string())
                .unwrap_or_else(|| e.to_string());
            AppError::BadRequest(format!("definition upsert failed: {msg}"))
        })?;
    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

/// DELETE /jaxrs/person/definition/{name}
pub async fn definition_delete(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    resolve_current_person_unique(&session_manager, &headers).await?;
    let Some(name) = normalize_name(&name) else {
        return Ok(Json(ActionResult::error("name is required")));
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_org_definition WHERE name = $1", &[&name])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "value": n > 0 }))))
}

// ═════════════════════════════════════════════════════════════════════════════
// EmpowerAction 残余：list next / prev / person + mock 别名（别名直连 empower crate）
// ═════════════════════════════════════════════════════════════════════════════

#[derive(Debug, serde::Serialize)]
struct EmpowerRow {
    id: String,
    from_person: String,
    to_person: String,
    role_id: Option<String>,
    enabled: bool,
}

async fn empower_anchor_exists(
    client: &deadpool_postgres::Client,
    anchor_id: &str,
) -> Result<bool, AppError> {
    let row = client
        .query_opt(
            "SELECT 1 AS ok FROM x_empower WHERE id = $1 AND deleted_at IS NULL",
            &[&anchor_id.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.is_some())
}

async fn empower_page(
    client: &deadpool_postgres::Client,
    dir_next: bool,
    anchor_id: Option<String>,
    count: i64,
) -> Result<Vec<EmpowerRow>, AppError> {
    let limit = count.clamp(1, 500);
    let rows = match anchor_id.as_deref() {
        None =>
            // id 为 "(0)" 或未找到锚点：从头取第一页
            if dir_next {
                client.query(
                    "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
                     WHERE deleted_at IS NULL ORDER BY created_at DESC, id DESC LIMIT $1",
                    &[&limit],
                ).await
            } else {
                client.query(
                    "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
                     WHERE deleted_at IS NULL ORDER BY created_at ASC, id ASC LIMIT $1",
                    &[&limit],
                ).await
            },
        Some(aid) => if dir_next {
            client.query(
                "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
                 WHERE deleted_at IS NULL \
                   AND (created_at, id) < (SELECT (created_at, id) FROM x_empower WHERE id = $1) \
                 ORDER BY created_at DESC, id DESC LIMIT $2",
                &[&aid.to_string(), &limit],
            ).await
        } else {
            client.query(
                "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
                 WHERE deleted_at IS NULL \
                   AND (created_at, id) > (SELECT (created_at, id) FROM x_empower WHERE id = $1) \
                 ORDER BY created_at ASC, id ASC LIMIT $2",
                &[&aid.to_string(), &limit],
            ).await
        },
    }
    .map_err(|_| AppError::Internal)?;

    Ok(rows
        .iter()
        .map(|r| EmpowerRow {
            id: r.get("id"),
            from_person: r.get("from_person"),
            to_person: r.get("to_person"),
            role_id: r.get("role_id"),
            enabled: r.get("enabled"),
        })
        .collect())
}

/// GET /jaxrs/person/empower/list/{id}/next/{count} —— 管理员下一页
pub async fn empower_list_next(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let anchor = match id.as_str() {
        "(0)" => None,
        _ if empower_anchor_exists(&client, &id).await? => Some(id.clone()),
        _ => None,
    };
    let rows = empower_page(&client, true, anchor, count).await?;
    let items: Vec<Value> = rows
        .iter()
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .collect();
    Ok(Json(ActionResult::success(json!({ "count": items.len() as i64, "data": items }))))
}

/// GET /jaxrs/person/empower/list/{id}/prev/{count} —— 管理员上一页
pub async fn empower_list_prev(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let anchor = match id.as_str() {
        "(0)" => None,
        _ if empower_anchor_exists(&client, &id).await? => Some(id.clone()),
        _ => None,
    };
    let mut rows = empower_page(&client, false, anchor, count).await?;
    // prev 页按逆序回传，保持与 next 相同的时间倒序展示
    rows.reverse();
    let items: Vec<Value> = rows
        .iter()
        .map(|r| serde_json::to_value(r).unwrap_or(Value::Null))
        .collect();
    Ok(Json(ActionResult::success(json!({ "count": items.len() as i64, "data": items }))))
}

/// GET /jaxrs/person/empower/list/person/{flag} —— 查询指定人员的授权
pub async fn empower_list_with_person(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let Some(person_unique) = resolve_person_flag(&client, &flag).await? else {
        return Ok(Json(ActionResult::error("person not found")));
    };
    let rows = client
        .query(
            "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
             WHERE from_person = $1 AND deleted_at IS NULL ORDER BY created_at DESC",
            &[&person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, String>("id"),
                "fromPerson": r.get::<_, String>("from_person"),
                "toPerson": r.get::<_, String>("to_person"),
                "roleId": r.get::<_, Option<String>>("role_id"),
                "enabled": r.get::<_, bool>("enabled"),
            })
        })
        .collect();
    Ok(Json(ActionResult::success(json!({ "count": items.len() as i64, "data": items }))))
}

// ═════════════════════════════════════════════════════════════════════════════
// EmpowerLogAction —— 授权日志（x_org_empower_log）
// ═════════════════════════════════════════════════════════════════════════════

const LOG_COLUMNS: &str =
    "id, COALESCE(from_person,'') AS from_person, COALESCE(to_person,'') AS to_person, \
     COALESCE(from_identity,'') AS from_identity, COALESCE(to_identity,'') AS to_identity, \
     COALESCE(application,'') AS application, COALESCE(title,'') AS title";

fn log_row_json(r: &deadpool_postgres::tokio_postgres::Row) -> Value {
    json!({
        "id": r.get::<_, String>("id"),
        "fromPerson": r.get::<_, String>("from_person"),
        "toPerson": r.get::<_, String>("to_person"),
        "fromIdentity": r.get::<_, String>("from_identity"),
        "toIdentity": r.get::<_, String>("to_identity"),
        "application": r.get::<_, String>("application"),
        "title": r.get::<_, String>("title"),
    })
}

/// GET /jaxrs/person/empowerlog/list/{id}/next|prev/{count} —— 管理员翻页
pub async fn log_list_next(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let limit = count.clamp(1, 500);
    let rows = if id == "(0)" || id.trim().is_empty() {
        client
            .query(
                &format!("SELECT {LOG_COLUMNS} FROM x_org_empower_log ORDER BY created_at DESC LIMIT $1"),
                &[&limit],
            )
            .await
    } else {
        client
            .query(
                &format!(
                    "SELECT {LOG_COLUMNS} FROM x_org_empower_log \
                     WHERE created_at < (SELECT created_at FROM x_org_empower_log WHERE id = $1) \
                     ORDER BY created_at DESC LIMIT $2"
                ),
                &[&id, &limit],
            )
            .await
    }
    .map_err(|_| AppError::Internal)?;

    let items: Vec<Value> = rows.iter().map(log_row_json).collect();
    Ok(Json(ActionResult::success(json!({ "count": items.len() as i64, "data": items }))))
}

pub async fn log_list_prev(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let limit = count.clamp(1, 500);
    let rows = if id == "(0)" || id.trim().is_empty() {
        client
            .query(
                &format!("SELECT {LOG_COLUMNS} FROM x_org_empower_log ORDER BY created_at ASC LIMIT $1"),
                &[&limit],
            )
            .await
    } else {
        client
            .query(
                &format!(
                    "SELECT {LOG_COLUMNS} FROM x_org_empower_log \
                     WHERE created_at > (SELECT created_at FROM x_org_empower_log WHERE id = $1) \
                     ORDER BY created_at ASC LIMIT $2"
                ),
                &[&id, &limit],
            )
            .await
    }
    .map_err(|_| AppError::Internal)?;

    let mut items: Vec<Value> = rows.iter().map(log_row_json).collect();
    items.reverse();
    Ok(Json(ActionResult::success(json!({ "count": items.len() as i64, "data": items }))))
}

#[derive(Debug, Deserialize)]
pub struct LogPagingWi {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(rename = "fromPerson", default)]
    pub from_person: Option<String>,
    #[serde(default)]
    pub startTime: Option<String>,
    #[serde(default)]
    pub endTime: Option<String>,
}

async fn log_paging(
    pool: &Pool,
    person: &str,
    admin_view: bool,
    wi: &LogPagingWi,
    page: i64,
    size: i64,
    to_scope: bool,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let limit = size.clamp(1, 200);
    let offset = (page - 1).clamp(0, i64::MAX) * limit;

    // 参数化谓词拼装（值全部走绑定参数，LIKE 关键字转义）
    let mut where_parts: Vec<String> = Vec::new();
    let mut params: Vec<String> = Vec::new();

    // 对齐 Java ActionManagerListPaging：管理员可按 fromPerson 过滤；非管理员仅见本人
    if admin_view && is_admin(pool, person).await {
        if let Some(fp) = wi.from_person.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            params.push(format!("%{}%", escape_like_key(fp)));
            where_parts.push(format!("COALESCE(from_person,'') LIKE ${}", params.len()));
        }
    } else {
        params.push(person.to_string());
        let col = if to_scope { "to_person" } else { "from_person" };
        where_parts.push(format!("COALESCE({col},'') = ${}", params.len()));
    }

    if let Some(start) = wi.startTime.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        params.push(start.to_string());
        where_parts.push(format!("created_at > ${}", params.len()));
    }
    if let Some(end) = wi.endTime.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        params.push(end.to_string());
        where_parts.push(format!("created_at < ${}", params.len()));
    }
    if let Some(key) = wi.key.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        params.push(format!("%{}%", escape_like_key(key)));
        where_parts.push(format!("COALESCE(title,'') LIKE ${}", params.len()));
    }

    let where_clause = if where_parts.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_parts.join(" AND "))
    };

    let count_row = client
        .query_one(
            &format!(
                "SELECT COUNT(*) AS c FROM x_org_empower_log {where_clause}"
            ),
            &params.iter().map(|s| s as &(dyn ToSqlSync + Sync)).collect::<Vec<_>>(),
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = count_row.get("c");

    let rows = client
        .query(
            &format!(
                "SELECT {LOG_COLUMNS} FROM x_org_empower_log {where_clause} \
                 ORDER BY created_at DESC LIMIT {} OFFSET {}",
                limit, offset
            ),
            &params.iter().map(|s| s as &(dyn ToSqlSync + Sync)).collect::<Vec<_>>(),
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let items: Vec<Value> = rows.iter().map(log_row_json).collect();
    Ok(Json(ActionResult::success(json!({
        "count": total,
        "data": items,
    }))))
}

/// LIKE 关键字转义（% _ \），对齐 Java StringTools.escapeSqlLikeKey
fn escape_like_key(key: &str) -> String {
    key.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")
}

/// POST /jaxrs/person/empowerlog/list/currentperson/paging/{page}/size/{size}
pub async fn log_currentperson_paging(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((page, size)): Path<(i64, i64)>,
    Json(wi): Json<LogPagingWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    log_paging(&pool, &person, false, &wi, page, size, false).await
}

/// POST /jaxrs/person/empowerlog/list/to/currentperson/paging/{page}/size/{size}
pub async fn log_to_currentperson_paging(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((page, size)): Path<(i64, i64)>,
    Json(wi): Json<LogPagingWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    log_paging(&pool, &person, false, &wi, page, size, true).await
}

/// POST /jaxrs/person/empowerlog/manager/list/paging/{page}/size/{size}
pub async fn log_manager_paging(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((page, size)): Path<(i64, i64)>,
    Json(wi): Json<LogPagingWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    log_paging(&pool, &person, true, &wi, page, size, false).await
}

/// DELETE /jaxrs/person/empowerlog/{id} —— 管理员删除（Java isNotManager 拒绝）
pub async fn log_delete(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_org_empower_log WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "value": n > 0 }))))
}

// ═════════════════════════════════════════════════════════════════════════════
// ExmailAction —— 腾讯企业邮（配置驱动：EXMAIL_API_BASE / EXMAIL_NEW_COUNT_URL /
//                 EXMAIL_SSO_URL / EXMAIL_CALLBACK_TOKEN）
// ═════════════════════════════════════════════════════════════════════════════

const EXMAIL_EXTEND_TYPE: &str = "exmail";

async fn read_exmail_extend(
    client: &deadpool_postgres::Client,
    person: &str,
) -> Result<Option<Value>, AppError> {
    let row = client
        .query_opt(
            "SELECT extend::text AS extend FROM x_org_person_extend WHERE person = $1 AND type = $2",
            &[&person.to_string(), &EXMAIL_EXTEND_TYPE.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.and_then(|r| {
        let raw: String = r.get("extend");
        serde_json::from_str(&raw).ok()
    }))
}

/// GET /jaxrs/person/exmail/new/count —— 即时获取（需配置取数地址）
pub async fn exmail_new_count(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    let url = match std::env::var("EXMAIL_NEW_COUNT_URL") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return Ok(Json(ActionResult::error(
                "exmail active count requires EXMAIL_NEW_COUNT_URL",
            )))
        }
    };

    let resp: Value = reqwest::Client::new()
        .get(&url)
        .query(&[("userid", person.as_str())])
        .send()
        .await
        .map_err(|e| AppError::BadRequest(format!("exmail upstream error: {e}")))?
        .json()
        .await
        .map_err(|e| AppError::BadRequest(format!("exmail upstream decode error: {e}")))?;

    let count = resp.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
    Ok(Json(ActionResult::success(json!({ "value": count }))))
}

/// GET /jaxrs/person/exmail/new/count/passive —— 读回调写入的未读数
pub async fn exmail_new_count_passive(
    pool: Extension<Pool>,

    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token_ok = extract_token_from_headers_pub(&headers);
    let person_opt = match token_ok {
        Some(token) => session_manager.validate_session(&token).await.map(|s| s.person_unique),
        None => None,
    };

    let mut count = 0i64;
    if let Some(person) = person_opt {
        let client = pool.get().await.map_err(|_| AppError::Internal)?;
        if let Some(ext) = read_exmail_extend(&client, &person).await? {
            count = ext.get("unreadCount").and_then(|v| v.as_i64()).unwrap_or(0);
        }
    }
    Ok(Json(ActionResult::success(json!({ "value": count }))))
}

/// GET /jaxrs/person/exmail/list/title/passive —— 读回调写入的邮件标题列表
pub async fn exmail_list_title_passive(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token_ok = extract_token_from_headers_pub(&headers);
    let person_opt = match token_ok {
        Some(token) => session_manager.validate_session(&token).await.map(|s| s.person_unique),
        None => None,
    };

    let mut titles: Vec<Value> = Vec::new();
    if let Some(person) = person_opt {
        let client = pool.get().await.map_err(|_| AppError::Internal)?;
        if let Some(ext) = read_exmail_extend(&client, &person).await? {
            if let Some(list) = ext.get("titleList").and_then(|v| v.as_array()) {
                titles = list
                    .iter()
                    .map(|t| json!({ "value": t.as_str().unwrap_or_default() }))
                    .collect();
            }
        }
    }
    Ok(Json(ActionResult::success(Value::Array(titles))))
}

/// GET /jaxrs/person/exmail/sso —— 单点登录地址（模板注入 userid）
pub async fn exmail_sso(
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<String>>, AppError> {
    let person = resolve_current_person_unique(&session_manager, &headers).await?;
    let template = match std::env::var("EXMAIL_SSO_URL") {
        Ok(v) if v.contains("{userid}") => v,
        Ok(_) | Err(_) => {
            return Ok(Json(ActionResult::error(
                "exmail sso requires EXMAIL_SSO_URL with {userid} placeholder",
            )))
        }
    };
    Ok(Json(ActionResult::success(template.replace("{userid}", &person))))
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    #[serde(default)]
    msg_signature: Option<String>,
    #[serde(default)]
    timestamp: Option<String>,
    #[serde(default)]
    nonce: Option<String>,
    #[serde(default)]
    echostr: Option<String>,
}

fn sha1_hex(input: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

/// 回调签名校验：sha1(sort(token, timestamp, nonce, encrypt))
fn verify_callback_signature(
    q: &CallbackQuery,
    encrypt_text: Option<&str>,
) -> Result<(), String> {
    let token = std::env::var("EXMAIL_CALLBACK_TOKEN")
        .map_err(|_| "callback not configured".to_string())?;
    let signature = q
        .msg_signature
        .as_deref()
        .filter(|s| !s.is_empty())
        .ok_or("msg_signature is required")?;
    let timestamp = q.timestamp.as_deref().unwrap_or("");
    let nonce = q.nonce.as_deref().ok_or("nonce is required")?;
    let payload = encrypt_text.ok_or("echostr is required")?;

    let mut parts = vec![token.as_str(), timestamp, nonce, payload];
    parts.sort();
    let computed = sha1_hex(&parts.concat());
    if !computed.eq_ignore_ascii_case(signature) {
        return Err("signature mismatch".to_string());
    }
    Ok(())
}

/// GET /jaxrs/person/exmail?msg_signature=..&timestamp=..&nonce=..&echostr=..
///
/// 验证回调签名。解密 echostr 需要 AES-CBC（依赖未引入），验签通过时显式返回
/// 不支持错误而非伪造成功。
pub async fn exmail_callback_get(
    Query(q): Query<CallbackQuery>,
) -> Result<Json<ActionResult<String>>, AppError> {
    let echostr = q.echostr.clone().unwrap_or_default();
    verify_callback_signature(&q, Some(&echostr)).map_err(AppError::BadRequest)?;
    Ok(Json(ActionResult::error(
        "callback decrypt requires AES-CBC support; signature verified",
    )))
}

/// POST /jaxrs/person/exmail —— 接收加密事件推送（同样先验签）
pub async fn exmail_callback_post(
    Query(q): Query<CallbackQuery>,
    body: String,
) -> Result<Json<ActionResult<String>>, AppError> {
    verify_callback_signature(&q, Some(body.trim())).map_err(AppError::BadRequest)?;
    Ok(Json(ActionResult::error(
        "callback decrypt requires AES-CBC support; signature verified",
    )))
}

use deadpool_postgres::tokio_postgres::types::ToSql as ToSqlSync;

// 从 shared 提取 Bearer/Cookie token 的轻量包装（避免依赖 auth crate 私有项）
fn extract_token_from_headers_pub(headers: &HeaderMap) -> Option<String> {
    shared::middleware::extract_token_from_headers(headers)
}

/// GET /jaxrs/person/signature/list/person/{flag}
///
/// 对齐 Java ActionManagerList：管理员查看指定人员的电子签名列表
/// （x_custom 中 name LIKE 'SIGNATURE_%' 的行）。
pub async fn signature_list_person(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let Some(person_unique) = resolve_person_flag(&client, &flag).await? else {
        return Ok(Json(ActionResult::error("person not found")));
    };

    let pattern = format!("{SIGNATURE_NAME_PREFIX}%");
    let rows = client
        .query(
            "SELECT id, name, person, value, created_at FROM x_custom \
             WHERE person = $1 AND name LIKE $2 AND deleted_at IS NULL \
             ORDER BY created_at DESC",
            &[&person_unique, &pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let signatures: Vec<Value> = rows
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<_, String>("id"),
                "name": row.get::<_, String>("name"),
                "person": row.get::<_, String>("person"),
                "value": row.get::<_, String>("value"),
            })
        })
        .collect();

    Ok(Json(ActionResult::success(json!({ "signatures": signatures }))))
}

