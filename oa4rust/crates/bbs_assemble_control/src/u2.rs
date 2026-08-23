//! plan002 U2 — BBS 端点全量闭合（对照 jaxrs 静态提取的 106 条 Java 全集补齐）。
//!
//! 分层约定（沿用 cms_assemble_control U2 先例）：
//! - 读操作公开；写操作按 IDOR 门禁：
//!   · 个人资源（subject/reply/attachment 的改删、toggle）先校验所有者
//!     （gate_*_owner：admin 放行、非所有者 403、不存在走契约错误体）；
//!   · 管理资源（forum/section/role/setting/shutup 管理）一律 require_admin。
//! - 表：既有表优先（x_bbs_forum/x_bbs_section/x_bbs_topic/x_bbs_reply/x_bbs_shutup），
//!   缺列由 migrations/067_bbs_u2_tables.sql 幂等补齐；全新域建表见同 migration。
//! - 无法落地的端点（二进制上传下载依赖 shared::storage 接线、图片引擎、
//!   外部 x_program_center 同步）注册为显式 501 + tracing::warn。
//! - normalize_java_path 归一化类级+方法级路径并折叠相邻重复段，
//!   防止 `{page}/{page}` 型畸形与通配吞并。

use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{
    error::AppError,
    response::{row_opt_json, ActionResult},
};
use std::collections::HashMap;
use uuid::Uuid;

type ApiResult = Result<Json<ActionResult<Value>>, AppError>;
type PgClient = deadpool_postgres::tokio_postgres::Client;

// ══════════════════════════════════════════════════════════════════
// 路径归一化（防通配冲突 / 畸形重复段）
// ══════════════════════════════════════════════════════════════════

/// 拼接类级与方法级 @Path 并归一化：去空段与首尾斜杠；折叠相邻重复段
/// （历史事故形态 `reply/filter/list/{page}/{page}/{count}/{count}`），
/// 使同一参数名不会在同一路由中出现两次。
pub fn normalize_java_path(class_path: &str, method_path: &str) -> String {
    let raw = format!(
        "{}/{}",
        class_path.trim_matches('/'),
        method_path.trim_matches('/')
    );
    let mut segments: Vec<&str> = Vec::new();
    for seg in raw.split('/') {
        if seg.is_empty() {
            continue;
        }
        if segments.last() == Some(&seg) {
            continue;
        }
        segments.push(seg);
    }
    segments.join("/")
}

// ══════════════════════════════════════════════════════════════════
// IDOR 门禁（对齐 cms u2_check_owner 语义）
// ══════════════════════════════════════════════════════════════════

pub(crate) enum U2Gate {
    NotFound,
    Forbidden,
    Allowed,
}

async fn u2_gate_by_sql(pool: &Pool, sql: &str, id: &str, person_unique: &str) -> Result<U2Gate, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client.query_opt(sql, &[&id]).await.map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(U2Gate::NotFound),
        Some(r) => {
            let owner = r.get::<_, Option<String>>("owner").unwrap_or_default();
            if shared::middleware::is_admin(pool, person_unique).await
                || (!owner.is_empty() && owner == person_unique)
            {
                Ok(U2Gate::Allowed)
            } else {
                Ok(U2Gate::Forbidden)
            }
        }
    }
}

async fn gate_topic_owner(pool: &Pool, id: &str, person_unique: &str) -> Result<U2Gate, AppError> {
    u2_gate_by_sql(
        pool,
        "SELECT creator AS owner FROM x_bbs_topic WHERE id = $1 AND deleted_at IS NULL",
        id,
        person_unique,
    )
    .await
}

async fn gate_reply_owner(pool: &Pool, id: &str, person_unique: &str) -> Result<U2Gate, AppError> {
    u2_gate_by_sql(
        pool,
        "SELECT creator AS owner FROM x_bbs_reply WHERE id = $1 AND deleted_at IS NULL",
        id,
        person_unique,
    )
    .await
}

/// table 仅由本模块常量字面量传入，无注入面。
async fn gate_attachment_owner(
    pool: &Pool,
    table: &str,
    id: &str,
    person_unique: &str,
) -> Result<U2Gate, AppError> {
    let sql = format!(
        "SELECT creator AS owner FROM {} WHERE id = $1 AND deleted_at IS NULL",
        table
    );
    u2_gate_by_sql(pool, &sql, id, person_unique).await
}

async fn u2_require_admin(pool: &Pool, session: &shared::session::Session) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn body_str(body: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|k| {
        body.get(*k)
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    })
}

/// LIKE/ILIKE 通配符转义（配合 ESCAPE '\\' 使用），防关键词注入通配扫描。
pub fn like_escape(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

// ══════════════════════════════════════════════════════════════════
// base64（纯逻辑，无新增依赖）
// ══════════════════════════════════════════════════════════════════

const B64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64_ALPHABET[((n >> 18) & 63) as usize] as char);
        out.push(B64_ALPHABET[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            B64_ALPHABET[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            B64_ALPHABET[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

// ══════════════════════════════════════════════════════════════════
// 主题 flag 白名单（toggle 端点共用；杜绝动态列名注入）
// ══════════════════════════════════════════════════════════════════

pub const TOPIC_FLAG_COLUMNS: &[&str] = &[
    "is_cream",
    "is_original",
    "is_recommend",
    "top_to_bbs",
    "top_to_forum",
    "top_to_main_section",
    "top_to_section",
    "locked",
    "completed",
];

pub fn topic_flag_column(flag: &str) -> Option<&'static str> {
    TOPIC_FLAG_COLUMNS.iter().copied().find(|c| *c == flag)
}

// ══════════════════════════════════════════════════════════════════
// 测试辅助：与 migration 067 相同的幂等 DDL（live 测试自足性）
// ══════════════════════════════════════════════════════════════════

#[cfg(test)]
pub(crate) async fn ensure_u2_schema(client: &PgClient) {
    let ddl: &[&str] = &[
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS is_cream BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS is_original BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS is_recommend BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS top_to_bbs BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS top_to_forum BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS top_to_main_section BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS top_to_section BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS locked BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS completed BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS accept_reply_id VARCHAR(255)",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS vote_count INTEGER NOT NULL DEFAULT 0",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS section_name VARCHAR(255)",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS subject_type VARCHAR(255)",
        "ALTER TABLE x_bbs_topic ADD COLUMN IF NOT EXISTS subject_grade VARCHAR(255)",
        "ALTER TABLE x_bbs_reply ADD COLUMN IF NOT EXISTS accepted BOOLEAN NOT NULL DEFAULT false",
        "ALTER TABLE x_bbs_shutup ADD COLUMN IF NOT EXISTS reason TEXT",
        "ALTER TABLE x_bbs_section ADD COLUMN IF NOT EXISTS parent_id VARCHAR(255)",
        "CREATE TABLE IF NOT EXISTS x_bbs_attachment (id VARCHAR(255) PRIMARY KEY, subject_id VARCHAR(255), name TEXT, extension TEXT, url TEXT, description TEXT, content BYTEA, length BIGINT DEFAULT 0, creator TEXT, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, update_time TIMESTAMP, deleted_at TIMESTAMP)",
        "CREATE TABLE IF NOT EXISTS x_bbs_subject_attachment (id VARCHAR(255) PRIMARY KEY, subject_id VARCHAR(255), name TEXT, description TEXT, url TEXT, content BYTEA, length BIGINT DEFAULT 0, creator TEXT, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, update_time TIMESTAMP, deleted_at TIMESTAMP)",
        "CREATE TABLE IF NOT EXISTS x_bbs_config_setting (id VARCHAR(255) PRIMARY KEY, name TEXT, code VARCHAR(255), value TEXT, description TEXT, creator TEXT, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, update_time TIMESTAMP, deleted_at TIMESTAMP)",
        "CREATE TABLE IF NOT EXISTS x_bbs_role (id VARCHAR(255) PRIMARY KEY, name TEXT, code VARCHAR(255), description TEXT, forum_id VARCHAR(255), section_id VARCHAR(255), creator TEXT, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, update_time TIMESTAMP, deleted_at TIMESTAMP)",
        "CREATE TABLE IF NOT EXISTS x_bbs_role_bind (id VARCHAR(255) PRIMARY KEY, role_id VARCHAR(255), object_type VARCHAR(64), object_code VARCHAR(255), object_name TEXT, creator TEXT, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
        "CREATE TABLE IF NOT EXISTS x_bbs_permission (id VARCHAR(255) PRIMARY KEY, code VARCHAR(255), name TEXT, forum_id VARCHAR(255), section_id VARCHAR(255), role_code VARCHAR(255), max_reply BIGINT DEFAULT -1, publish BOOLEAN NOT NULL DEFAULT true, reply BOOLEAN NOT NULL DEFAULT true, visible BOOLEAN NOT NULL DEFAULT true, creator TEXT, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, deleted_at TIMESTAMP)",
        "CREATE TABLE IF NOT EXISTS x_bbs_vote_record (id VARCHAR(255) PRIMARY KEY, topic_id VARCHAR(255), person VARCHAR(255), option_id VARCHAR(255), option_name TEXT, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
        "CREATE TABLE IF NOT EXISTS x_bbs_user_info (id VARCHAR(255) PRIMARY KEY, person VARCHAR(255), nick_name TEXT, icon TEXT, signature TEXT, create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, update_time TIMESTAMP, deleted_at TIMESTAMP)",
        "INSERT INTO x_bbs_config_setting (id, name, code, value) VALUES ('bbs-setting-bbs-name', 'BBS名称', 'BBS_NAME', 'O2社区') ON CONFLICT (id) DO NOTHING",
    ];
    for stmt in ddl {
        let _ = client.execute(*stmt, &[]).await;
    }
}

// ══════════════════════════════════════════════════════════════════
// 显式 501（无法落地：二进制存储 / 图片引擎 / 外部同步依赖）
// ══════════════════════════════════════════════════════════════════

macro_rules! unimplemented_endpoint {
    ($name:ident, $reason:expr) => {
        pub async fn $name() -> ApiResult {
            tracing::warn!(
                endpoint = stringify!($name),
                reason = $reason,
                "bbs endpoint registered as explicit 501"
            );
            Err(AppError::NotImplemented)
        }
    };
}

// Java AttachmentAction.downloadWithSubject：附件二进制流下载，
// 依赖 shared::storage 接线（见 lib.rs U6b 注释），当前显式 501。
unimplemented_endpoint!(
    attachment_download_501,
    "binary download pending shared::storage wiring (plan002 U6b)"
);
unimplemented_endpoint!(
    attachment_download_stream_501,
    "binary streaming pending shared::storage wiring (plan002 U6b)"
);
unimplemented_endpoint!(
    attachment_upload_501,
    "multipart upload pending shared::storage wiring (plan002 U6b)"
);
unimplemented_endpoint!(
    attachment_upload_callback_501,
    "multipart upload pending shared::storage wiring (plan002 U6b)"
);
// Java PictureAction.pictureEncode：图片解码缩放后转 base64，需要图像引擎。
unimplemented_endpoint!(picture_encode_501, "image decode/resize engine not available");
unimplemented_endpoint!(
    picture_section_icon_501,
    "icon upload pending shared::storage wiring (plan002 U6b)"
);
// Java SectionInfoAction.syn（ActionSynApplicationsFromMarket）调用外部
// x_program_center market/list/paging 同步，属外部服务依赖。
unimplemented_endpoint!(
    section_syn_501,
    "depends on external x_program_center market sync service"
);

// ══════════════════════════════════════════════════════════════════
// user/subject 域（27 条）：18 个 flag toggle + acceptreply + get/save/
// change/section/vote/my-list/voterecord/软删
// ══════════════════════════════════════════════════════════════════

async fn set_topic_flag(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    id: String,
    flag: &'static str,
    value: bool,
) -> ApiResult {
    let pool = pool.0;
    match gate_topic_owner(&pool, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("subject not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let column = topic_flag_column(flag)
                .ok_or_else(|| AppError::BadRequest(format!("unknown flag {}", flag)))?;
            let sql = format!(
                "UPDATE x_bbs_topic SET {} = $1 WHERE id = $2 AND deleted_at IS NULL",
                column
            );
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(sql.as_str(), &[&value, &id])
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("flag".to_string(), Value::String(flag.to_string())),
                    ("value".to_string(), Value::Bool(value)),
                    ("updated".to_string(), Value::Bool(affected > 0)),
                ]),
            ))))
        }
    }
}

macro_rules! topic_toggle_endpoints {
    ($($fn_name:ident => ($flag:ident, $value:expr)),* $(,)?) => {$(
        #[axum::debug_handler]
        pub async fn $fn_name(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            Path(id): Path<String>,
        ) -> ApiResult {
            set_topic_flag(pool, session, id, stringify!($flag), $value).await
        }
    )*};
}

topic_toggle_endpoints! {
    u2_subject_set_cream => (is_cream, true),
    u2_subject_non_cream => (is_cream, false),
    u2_subject_set_original => (is_original, true),
    u2_subject_non_original => (is_original, false),
    u2_subject_set_recommend_index => (is_recommend, true),
    u2_subject_non_recommend_index => (is_recommend, false),
    u2_subject_top_to_bbs => (top_to_bbs, true),
    u2_subject_non_top_to_bbs => (top_to_bbs, false),
    u2_subject_top_to_forum => (top_to_forum, true),
    u2_subject_non_top_to_forum => (top_to_forum, false),
    u2_subject_top_to_main_section => (top_to_main_section, true),
    u2_subject_non_top_to_main_section => (top_to_main_section, false),
    u2_subject_top_to_section => (top_to_section, true),
    u2_subject_non_top_to_section => (top_to_section, false),
    u2_subject_lock => (locked, true),
    u2_subject_unlock => (locked, false),
    u2_subject_complete => (completed, true),
    u2_subject_uncomplete => (completed, false),
}

/// GET user/subject/{id} — 单条主题详情。
pub async fn u2_subject_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, content, creator, author_id, forum_id, section_id, \
             section_name, subject_type, is_top, is_cream, is_original, is_recommend, \
             locked, completed, view_count, reply_count \
             FROM x_bbs_topic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => {
            let map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                ("title".to_string(), Value::String(r.get("title"))),
                (
                    "content".to_string(),
                    r.get::<_, Option<String>>("content")
                        .map(Value::String)
                        .unwrap_or_else(|| Value::String(String::new())),
                ),
                (
                    "creator".to_string(),
                    r.get::<_, Option<String>>("creator").map(Value::String).unwrap_or(Value::Null),
                ),
                ("authorId".to_string(), Value::String(r.get("author_id"))),
                (
                    "forumId".to_string(),
                    r.get::<_, Option<String>>("forum_id").map(Value::String).unwrap_or(Value::Null),
                ),
                ("sectionId".to_string(), Value::String(r.get("section_id"))),
                (
                    "sectionName".to_string(),
                    r.get::<_, Option<String>>("section_name").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "subjectType".to_string(),
                    r.get::<_, Option<String>>("subject_type").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "isTop".to_string(),
                    Value::Bool(r.get::<_, Option<bool>>("is_top").unwrap_or(false)),
                ),
                ("isCream".to_string(), Value::Bool(r.get("is_cream"))),
                ("isOriginal".to_string(), Value::Bool(r.get("is_original"))),
                ("isRecommend".to_string(), Value::Bool(r.get("is_recommend"))),
                ("locked".to_string(), Value::Bool(r.get("locked"))),
                ("completed".to_string(), Value::Bool(r.get("completed"))),
                (
                    "viewCount".to_string(),
                    Value::Number(serde_json::Number::from(r.get::<_, i32>("view_count"))),
                ),
                (
                    "replyCount".to_string(),
                    Value::Number(serde_json::Number::from(r.get::<_, i32>("reply_count"))),
                ),
            ]);
            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Ok(Json(ActionResult::error("subject not found"))),
    }
}

/// POST user/subject — 发表主题（Java SubjectInfoManagerUserAction.save）。
pub async fn u2_subject_save(pool: Extension<Pool>, body: axum::extract::Json<Value>) -> ApiResult {
    let title = match body_str(&body, &["title"]) {
        Some(t) => t,
        None => return Err(AppError::BadRequest("title is required".to_string())),
    };
    let content = body_str(&body, &["content"]).unwrap_or_default();
    let creator = body_str(&body, &["creator", "person", "authorId"])
        .unwrap_or_else(|| "anonymous".to_string());
    let forum_id = body_str(&body, &["forumId"]).unwrap_or_default();
    let section_id = body_str(&body, &["sectionId"]).unwrap_or_default();
    let section_name = body_str(&body, &["sectionName"]).unwrap_or_default();
    let subject_type = body_str(&body, &["subjectType"]).unwrap_or_default();
    let id = Uuid::new_v4().to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_bbs_topic (id, title, content, author_id, creator, forum_id, \
             section_id, section_name, subject_type, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())",
            &[
                &id, &title, &content, &creator, &creator, &forum_id, &section_id,
                &section_name, &subject_type,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("sectionId".to_string(), Value::String(section_id)),
    ])))))
}

/// GET user/subject/acceptreply/{id}/{replyId} — 设定被采纳回复（owner 门禁）。
pub async fn u2_subject_accept_reply(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((id, reply_id)): Path<(String, String)>,
) -> ApiResult {
    let pool = pool.0;
    match gate_topic_owner(&pool, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("subject not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_bbs_topic SET accept_reply_id = $1 WHERE id = $2 AND deleted_at IS NULL",
                    &[&reply_id, &id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let _ = client
                .execute(
                    "UPDATE x_bbs_reply SET accepted = true WHERE id = $1 AND deleted_at IS NULL",
                    &[&reply_id],
                )
                .await;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("acceptReplyId".to_string(), Value::String(reply_id)),
                ("updated".to_string(), Value::Bool(affected > 0)),
            ])))))
        }
    }
}

/// GET user/subject/unacceptreply/{id} — 取消采纳。
pub async fn u2_subject_unaccept_reply(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    match gate_topic_owner(&pool, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("subject not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            client
                .execute(
                    "UPDATE x_bbs_topic SET accept_reply_id = NULL WHERE id = $1 AND deleted_at IS NULL",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("acceptReplyId".to_string(), Value::Null),
            ])))))
        }
    }
}

/// DELETE user/subject/{id} — 软删主题（owner 门禁）。
pub async fn u2_subject_soft_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    match gate_topic_owner(&pool, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("subject not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_bbs_topic SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("deleted".to_string(), Value::Bool(affected > 0)),
            ])))))
        }
    }
}

/// PUT user/subject/change/section — 调整主题所在版块（owner 门禁）。
pub async fn u2_subject_change_section(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let id = match body_str(&body, &["subjectId", "id"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("subjectId is required".to_string())),
    };
    let section_id = body_str(&body, &["sectionId"]).unwrap_or_default();
    let section_name = body_str(&body, &["sectionName"]).unwrap_or_default();
    match gate_topic_owner(&pool, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("subject not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_bbs_topic SET section_id = $1, section_name = $2 WHERE id = $3 AND deleted_at IS NULL",
                    &[&section_id, &section_name, &id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("sectionId".to_string(), Value::String(section_id)),
                ("updated".to_string(), Value::Bool(affected > 0)),
            ])))))
        }
    }
}

/// PUT user/subject — 投票提交（voteSubmit）：写投票记录并累加计数。
pub async fn u2_vote_submit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let topic_id = match body_str(&body, &["subjectId", "topicId", "id"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("subjectId is required".to_string())),
    };
    let option_id = body_str(&body, &["optionId"]).unwrap_or_default();
    let option_name = body_str(&body, &["optionName"]).unwrap_or_default();
    let person = session.person_unique.clone();
    let record_id = Uuid::new_v4().to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_bbs_vote_record (id, topic_id, person, option_id, option_name) \
             VALUES ($1, $2, $3, $4, $5)",
            &[&record_id, &topic_id, &person, &option_id, &option_name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE x_bbs_topic SET vote_count = vote_count + 1 WHERE id = $1 AND deleted_at IS NULL",
            &[&topic_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(record_id)),
        ("subjectId".to_string(), Value::String(topic_id)),
        ("voted".to_string(), Value::Bool(true)),
    ])))))
}

/// PUT user/subject/voterecord/list/page/{page}/count/{count} — 投票记录分页。
pub async fn u2_voterecord_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let topic_id = match body_str(&body, &["subjectId", "topicId"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("subjectId is required".to_string())),
    };
    let offset = page.saturating_sub(1).saturating_mul(count);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, person, option_id, option_name FROM x_bbs_vote_record \
             WHERE topic_id = $1 ORDER BY create_time DESC LIMIT $2 OFFSET $3",
            &[&topic_id, &count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one(
            "SELECT COUNT(*) FROM x_bbs_vote_record WHERE topic_id = $1",
            &[&topic_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                (
                    "person".to_string(),
                    r.get::<_, Option<String>>("person").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "optionId".to_string(),
                    r.get::<_, Option<String>>("option_id").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "optionName".to_string(),
                    r.get::<_, Option<String>>("option_name").map(Value::String).unwrap_or(Value::Null),
                ),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total_row.get::<_, i64>(0)))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// PUT user/subject/my/list/page/{page}/count/{count} — 我的主题分页（按会话人）。
pub async fn u2_my_subject_list(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((page, count)): Path<(i64, i64)>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    // 会话优先，兼容 body 显式传 creator 的管理查询。
    let person =
        body_str(&body, &["creator", "person"]).unwrap_or_else(|| session.person_unique.clone());
    let offset = page.saturating_sub(1).saturating_mul(count);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, content, creator, forum_id, create_time::text AS create_time \
             FROM x_bbs_topic WHERE deleted_at IS NULL AND (creator = $1 OR author_id = $1) \
             ORDER BY create_time DESC LIMIT $2 OFFSET $3",
            &[&person, &count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one(
            "SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL AND (creator = $1 OR author_id = $1)",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                ("title".to_string(), Value::String(r.get("title"))),
                (
                    "content".to_string(),
                    r.get::<_, Option<String>>("content")
                        .map(Value::String)
                        .unwrap_or_else(|| Value::String(String::new())),
                ),
                (
                    "creator".to_string(),
                    r.get::<_, Option<String>>("creator").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "forumId".to_string(),
                    r.get::<_, Option<String>>("forum_id").map(Value::String).unwrap_or(Value::Null),
                ),
                ("createTime".to_string(), Value::String(r.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total_row.get::<_, i64>(0)))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// PUT user/reply/my/list/page/{page}/count/{count} — 我的回复分页。
pub async fn u2_my_reply_list(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((page, count)): Path<(i64, i64)>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let person =
        body_str(&body, &["creator", "person"]).unwrap_or_else(|| session.person_unique.clone());
    let offset = page.saturating_sub(1).saturating_mul(count);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, topic_id, content, creator, create_time::text AS create_time \
             FROM x_bbs_reply WHERE deleted_at IS NULL AND (creator = $1 OR author_id = $1) \
             ORDER BY create_time DESC LIMIT $2 OFFSET $3",
            &[&person, &count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one(
            "SELECT COUNT(*) FROM x_bbs_reply WHERE deleted_at IS NULL AND (creator = $1 OR author_id = $1)",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                (
                    "topicId".to_string(),
                    r.get::<_, Option<String>>("topic_id").map(Value::String).unwrap_or(Value::Null),
                ),
                ("content".to_string(), Value::String(r.get("content"))),
                (
                    "creator".to_string(),
                    r.get::<_, Option<String>>("creator").map(Value::String).unwrap_or(Value::Null),
                ),
                ("createTime".to_string(), Value::String(r.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total_row.get::<_, i64>(0)))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

// ══════════════════════════════════════════════════════════════════
// reply 域（4 条缺口）
// ══════════════════════════════════════════════════════════════════

/// GET reply/{id} — 单条回复。
pub async fn u2_reply_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, topic_id, content, creator, accepted FROM x_bbs_reply \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(r.get("id"))),
            (
                "topicId".to_string(),
                r.get::<_, Option<String>>("topic_id").map(Value::String).unwrap_or(Value::Null),
            ),
            ("content".to_string(), Value::String(r.get("content"))),
            (
                "creator".to_string(),
                r.get::<_, Option<String>>("creator").map(Value::String).unwrap_or(Value::Null),
            ),
            (
                "accepted".to_string(),
                Value::Bool(r.get::<_, Option<bool>>("accepted").unwrap_or(false)),
            ),
        ]))))),
        None => Ok(Json(ActionResult::error("reply not found"))),
    }
}

/// PUT reply/filter/list/page/{page}/count/{count} — 回复分页（可按 topicId 过滤）。
pub async fn u2_reply_filter_list(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let offset = page.saturating_sub(1).saturating_mul(count);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let (rows, total) = match body_str(&body, &["topicId", "subjectId"]) {
        Some(topic_id) => {
            let rows = client
                .query(
                    "SELECT id, topic_id, content, creator FROM x_bbs_reply \
                     WHERE deleted_at IS NULL AND topic_id = $1 \
                     ORDER BY create_time DESC LIMIT $2 OFFSET $3",
                    &[&topic_id, &count, &offset],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let t = client
                .query_one(
                    "SELECT COUNT(*) FROM x_bbs_reply WHERE deleted_at IS NULL AND topic_id = $1",
                    &[&topic_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            (rows, t.get::<_, i64>(0))
        }
        None => {
            let rows = client
                .query(
                    "SELECT id, topic_id, content, creator FROM x_bbs_reply \
                     WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $1 OFFSET $2",
                    &[&count, &offset],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let t = client
                .query_one("SELECT COUNT(*) FROM x_bbs_reply WHERE deleted_at IS NULL", &[])
                .await
                .map_err(|_| AppError::Internal)?;
            (rows, t.get::<_, i64>(0))
        }
    };
    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                (
                    "topicId".to_string(),
                    r.get::<_, Option<String>>("topic_id").map(Value::String).unwrap_or(Value::Null),
                ),
                ("content".to_string(), Value::String(r.get("content"))),
                (
                    "creator".to_string(),
                    r.get::<_, Option<String>>("creator").map(Value::String).unwrap_or(Value::Null),
                ),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// POST user/reply — 发表回复（Java ReplyInfoManagerUserAction.save）。
pub async fn u2_user_reply_save(pool: Extension<Pool>, body: axum::extract::Json<Value>) -> ApiResult {
    let content = match body_str(&body, &["content"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("content is required".to_string())),
    };
    let topic_id = body_str(&body, &["topicId", "subjectId"]).unwrap_or_default();
    let creator =
        body_str(&body, &["creator", "person"]).unwrap_or_else(|| "anonymous".to_string());
    let id = Uuid::new_v4().to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_bbs_reply (id, topic_id, content, creator, create_time) \
             VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &topic_id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("topicId".to_string(), Value::String(topic_id)),
    ])))))
}

/// PUT user/reply/accept — 采纳回复（owner 门禁：仅主题所有者可采纳）。
pub async fn u2_user_reply_accept(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let subject_id = match body_str(&body, &["subjectId", "topicId"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("subjectId is required".to_string())),
    };
    let reply_id = match body_str(&body, &["replyId", "id"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("replyId is required".to_string())),
    };
    match gate_topic_owner(&pool, &subject_id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("subject not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            client
                .execute(
                    "UPDATE x_bbs_reply SET accepted = true WHERE id = $1 AND deleted_at IS NULL",
                    &[&reply_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            client
                .execute(
                    "UPDATE x_bbs_topic SET accept_reply_id = $1 WHERE id = $2 AND deleted_at IS NULL",
                    &[&reply_id, &subject_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("subjectId".to_string(), Value::String(subject_id)),
                ("acceptReplyId".to_string(), Value::String(reply_id)),
            ])))))
        }
    }
}

/// DELETE user/reply/{id} — 软删回复（owner 门禁）。
pub async fn u2_user_reply_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    match gate_reply_owner(&pool, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("reply not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_bbs_reply SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("deleted".to_string(), Value::Bool(affected > 0)),
            ])))))
        }
    }
}

// ══════════════════════════════════════════════════════════════════
// user/forum 域（管理资源：require_admin）
// ══════════════════════════════════════════════════════════════════

/// POST user/forum — 创建论坛（admin）。
pub async fn u2_user_forum_save(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let name = match body_str(&body, &["name", "forumName"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("name is required".to_string())),
    };
    let description = body_str(&body, &["description"]).unwrap_or_default();
    let creator = session.person_unique.clone();
    let id = Uuid::new_v4().to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_bbs_forum (id, name, description, creator, create_time) \
             VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &name, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

/// DELETE user/forum/{id} — 软删论坛（admin）。
pub async fn u2_user_forum_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute(
            "UPDATE x_bbs_forum SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Bool(affected > 0)),
    ])))))
}

// ══════════════════════════════════════════════════════════════════
// section 域（6 条缺口）
// ══════════════════════════════════════════════════════════════════

fn section_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("forumId".to_string(), Value::String(row.get("forum_id"))),
        (
            "parentId".to_string(),
            row.get::<_, Option<String>>("parent_id").map(Value::String).unwrap_or(Value::Null),
        ),
        (
            "sort".to_string(),
            Value::Number(serde_json::Number::from(
                row.get::<_, Option<i32>>("sort").unwrap_or(0),
            )),
        ),
        (
            "description".to_string(),
            row_opt_json::<String>(row, "description").unwrap_or(Value::Null),
        ),
    ]))
}

async fn query_sections(client: &PgClient, where_clause: &str, param: &str) -> Result<Vec<Value>, AppError> {
    // where_clause 仅由本模块常量字面量传入。
    let sql = format!(
        "SELECT id, name, forum_id, parent_id, sort, description FROM x_bbs_section \
         WHERE deleted_at IS NULL AND {} ORDER BY sort, create_time",
        where_clause
    );
    let rows = client.query(sql.as_str(), &[&param]).await.map_err(|_| AppError::Internal)?;
    Ok(rows.iter().map(section_row_to_value).collect())
}

/// GET section/{id} — 版块详情。
pub async fn u2_section_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = query_sections(&client, "id = $1", &id).await?;
    match data.into_iter().next() {
        Some(v) => Ok(Json(ActionResult::success(v))),
        None => Ok(Json(ActionResult::error("section not found"))),
    }
}

/// GET section/viewsub/{sectionId} — 主版块的子版块。
pub async fn u2_section_viewsub(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = query_sections(&client, "parent_id = $1", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// GET user/section/forum/{forumId} — 论坛下的版块。
pub async fn u2_user_section_forum(pool: Extension<Pool>, Path(forum_id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = query_sections(&client, "forum_id = $1", &forum_id).await?;
    Ok(Json(ActionResult::success(Value::Array(data))))
}

/// GET user/section/sub/{sectionId} — 子版块全量（管理视图）。
pub async fn u2_user_section_sub(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    u2_section_viewsub(pool, Path(id)).await
}

/// POST user/section — 创建版块（admin）。
pub async fn u2_user_section_save(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let name = match body_str(&body, &["name", "sectionName"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("name is required".to_string())),
    };
    let forum_id = match body_str(&body, &["forumId"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("forumId is required".to_string())),
    };
    let parent_id = body_str(&body, &["parentId", "mainSectionId"]).unwrap_or_default();
    let description = body_str(&body, &["description"]).unwrap_or_default();
    let creator = session.person_unique.clone();
    let id = Uuid::new_v4().to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_bbs_section (id, forum_id, name, parent_id, description, creator, create_time) \
             VALUES ($1, $2, $3, NULLIF($4, ''), $5, $6, NOW())",
            &[&id, &forum_id, &name, &parent_id, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("forumId".to_string(), Value::String(forum_id)),
    ])))))
}

/// DELETE user/section/{id} — 软删版块（admin）。
pub async fn u2_user_section_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute(
            "UPDATE x_bbs_section SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Bool(affected > 0)),
    ])))))
}

/// DELETE user/section/force/{id} — 物理删除版块（admin）。
pub async fn u2_user_section_delete_force(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute("DELETE FROM x_bbs_section WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Bool(affected > 0)),
    ])))))
}

// ══════════════════════════════════════════════════════════════════
// user/role 域（11 条；写操作 admin）
// ══════════════════════════════════════════════════════════════════

fn role_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), row_opt_json::<String>(row, "name").unwrap_or(Value::Null)),
        ("code".to_string(), row_opt_json::<String>(row, "code").unwrap_or(Value::Null)),
        ("description".to_string(), row_opt_json::<String>(row, "description").unwrap_or(Value::Null)),
        ("forumId".to_string(), row_opt_json::<String>(row, "forum_id").unwrap_or(Value::Null)),
        ("sectionId".to_string(), row_opt_json::<String>(row, "section_id").unwrap_or(Value::Null)),
    ]))
}

/// GET user/role/{id} — BBS 角色详情。
pub async fn u2_role_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, code, description, forum_id, section_id FROM x_bbs_role \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match rows.first() {
        Some(r) => Ok(Json(ActionResult::success(role_row_to_value(r)))),
        None => Ok(Json(ActionResult::error("role not found"))),
    }
}

/// GET user/role/all — BBS 角色全量（对齐 Java RoleInfoAction.listAll，读 x_bbs_role）。
pub async fn u2_role_all(pool: Extension<Pool>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, code, description, forum_id, section_id FROM x_bbs_role \
             WHERE deleted_at IS NULL ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Array(
        rows.iter().map(role_row_to_value).collect(),
    ))))
}

/// POST user/role — 创建角色（admin）。
pub async fn u2_role_save(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let name = match body_str(&body, &["name", "roleName"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("name is required".to_string())),
    };
    let code = body_str(&body, &["code", "roleCode"]).unwrap_or_else(|| name.clone());
    let description = body_str(&body, &["description"]).unwrap_or_default();
    let forum_id = body_str(&body, &["forumId"]).unwrap_or_default();
    let section_id = body_str(&body, &["sectionId"]).unwrap_or_default();
    let creator = session.person_unique.clone();
    let id = Uuid::new_v4().to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_bbs_role (id, name, code, description, forum_id, section_id, creator) \
             VALUES ($1, $2, $3, $4, NULLIF($5,''), NULLIF($6,''), $7)",
            &[&id, &name, &code, &description, &forum_id, &section_id, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("code".to_string(), Value::String(code)),
    ])))))
}

/// DELETE user/role/{id} — 软删角色（admin）。
pub async fn u2_role_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute(
            "UPDATE x_bbs_role SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Bool(affected > 0)),
    ])))))
}

async fn list_roles_by_column(pool: Extension<Pool>, column: &str, value: &str) -> ApiResult {
    // column 仅来自本文件两处常量调用（"forum_id" / "section_id"）。
    let sql = format!(
        "SELECT id, name, code, description, forum_id, section_id FROM x_bbs_role \
         WHERE deleted_at IS NULL AND {} = $1 ORDER BY create_time",
        column
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(sql.as_str(), &[&value]).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Array(
        rows.iter().map(role_row_to_value).collect(),
    ))))
}

/// PUT user/role/forum/{forumId} — 按论坛列角色。
pub async fn u2_role_list_by_forum(pool: Extension<Pool>, Path(forum_id): Path<String>) -> ApiResult {
    list_roles_by_column(pool, "forum_id", &forum_id).await
}

/// PUT user/role/section/{sectionId} — 按版块列角色。
pub async fn u2_role_list_by_section(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    list_roles_by_column(pool, "section_id", &id).await
}

async fn resolve_role_id(
    client: &PgClient,
    role_id: Option<String>,
    role_code: Option<String>,
) -> Result<Option<String>, AppError> {
    if let Some(id) = role_id {
        return Ok(Some(id));
    }
    if let Some(code) = role_code {
        let row = client
            .query_opt(
                "SELECT id FROM x_bbs_role WHERE code = $1 AND deleted_at IS NULL",
                &[&code],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        return Ok(row.map(|r| r.get::<_, String>("id")));
    }
    Ok(None)
}

async fn insert_bind(
    client: &PgClient,
    role_id: &str,
    object_type: &str,
    object_code: &str,
    object_name: &str,
    creator: &str,
) -> Result<(), AppError> {
    let id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_bbs_role_bind (id, role_id, object_type, object_code, object_name, creator) \
             VALUES ($1, $2, $3, $4, $5, $6)",
            &[&id, &role_id, &object_type, &object_code, &object_name, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(())
}

fn bind_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("roleId".to_string(), Value::String(row.get("role_id"))),
        ("objectType".to_string(), row_opt_json::<String>(row, "object_type").unwrap_or(Value::Null)),
        ("objectCode".to_string(), row_opt_json::<String>(row, "object_code").unwrap_or(Value::Null)),
        ("objectName".to_string(), row_opt_json::<String>(row, "object_name").unwrap_or(Value::Null)),
    ]))
}

/// PUT user/role/bind/object — 绑定对象到角色（admin）。
pub async fn u2_role_bind_object(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let role_id = {
        let client = pool.get().await.map_err(|_| AppError::Internal)?;
        resolve_role_id(&client, body_str(&body, &["roleId"]), body_str(&body, &["roleCode"])).await?
    };
    let Some(role_id) = role_id else {
        return Err(AppError::BadRequest("roleId or valid roleCode required".to_string()));
    };
    let objects = body
        .get("objects")
        .or_else(|| body.get("personList"))
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let mut bound = 0usize;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    for obj in objects {
        let object_type = obj
            .get("type")
            .or_else(|| obj.get("objectType"))
            .and_then(|v| v.as_str())
            .unwrap_or("person")
            .to_string();
        let object_code = obj
            .get("code")
            .or_else(|| obj.get("objectCode"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let object_name = obj
            .get("name")
            .or_else(|| obj.get("objectName"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        insert_bind(&client, &role_id, &object_type, &object_code, &object_name, &session.person_unique)
            .await?;
        bound += 1;
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("roleId".to_string(), Value::String(role_id)),
        ("bound".to_string(), Value::Number(serde_json::Number::from(bound as i64))),
    ])))))
}

/// PUT user/role/bind/role — 把人绑定到一组角色（admin）。
pub async fn u2_role_bind_user(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let person = match body_str(&body, &["personCode", "userCode", "person"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("personCode is required".to_string())),
    };
    let role_ids = body
        .get("roleIds")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let mut bound = 0usize;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    for rid in role_ids {
        let Some(rid) = rid.as_str().map(String::from) else { continue };
        insert_bind(&client, &rid, "person", &person, "", &session.person_unique).await?;
        bound += 1;
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("personCode".to_string(), Value::String(person)),
        ("bound".to_string(), Value::Number(serde_json::Number::from(bound as i64))),
    ])))))
}

async fn binds_for_role(client: &PgClient, role_id: &str) -> Result<Vec<Value>, AppError> {
    let rows = client
        .query(
            "SELECT id, role_id, object_type, object_code, object_name FROM x_bbs_role_bind WHERE role_id = $1",
            &[&role_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(rows.iter().map(bind_row_to_value).collect())
}

/// PUT user/role/rolecode/selected — 按 roleCode 列绑定对象。
pub async fn u2_role_selected_by_code(pool: Extension<Pool>, body: axum::extract::Json<Value>) -> ApiResult {
    let code = match body_str(&body, &["roleCode", "code"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("roleCode is required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let role_id = resolve_role_id(&client, None, Some(code)).await?;
    let Some(role_id) = role_id else {
        return Ok(Json(ActionResult::error("role not found")));
    };
    let data = binds_for_role(&client, &role_id).await?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("roleId".to_string(), Value::String(role_id)),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

async fn roles_bound_to(client: &PgClient, object_type: &str, object_code: &str) -> Result<Vec<Value>, AppError> {
    // object_type 仅由本模块常量字面量（"unit"/"person"）传入。
    let sql = format!(
        "SELECT r.id, r.name, r.code, r.description, r.forum_id, r.section_id \
         FROM x_bbs_role r JOIN x_bbs_role_bind b ON b.role_id = r.id \
         WHERE r.deleted_at IS NULL AND b.object_type = '{}' AND b.object_code = $1",
        object_type
    );
    let rows = client.query(sql.as_str(), &[&object_code]).await.map_err(|_| AppError::Internal)?;
    Ok(rows.iter().map(role_row_to_value).collect())
}

/// PUT user/role/unit/selected — 按组织列出已绑角色。
pub async fn u2_role_by_unit(pool: Extension<Pool>, body: axum::extract::Json<Value>) -> ApiResult {
    let unit = match body_str(&body, &["unitCode", "unit"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("unitCode is required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = roles_bound_to(&client, "unit", &unit).await?;
    Ok(Json(ActionResult::success(Value::Array(data))))
}

/// PUT user/role/user/selected — 按人列出已绑角色。
pub async fn u2_role_by_user(pool: Extension<Pool>, body: axum::extract::Json<Value>) -> ApiResult {
    let person = match body_str(&body, &["personCode", "userCode", "person"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("personCode is required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = roles_bound_to(&client, "person", &person).await?;
    Ok(Json(ActionResult::success(Value::Array(data))))
}

// ══════════════════════════════════════════════════════════════════
// permission 域（4 条新实现 + 2 条重挂既有 handler）
// ══════════════════════════════════════════════════════════════════

fn permission_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("code".to_string(), row_opt_json::<String>(row, "code").unwrap_or(Value::Null)),
        ("name".to_string(), row_opt_json::<String>(row, "name").unwrap_or(Value::Null)),
        ("forumId".to_string(), row_opt_json::<String>(row, "forum_id").unwrap_or(Value::Null)),
        ("sectionId".to_string(), row_opt_json::<String>(row, "section_id").unwrap_or(Value::Null)),
        ("roleCode".to_string(), row_opt_json::<String>(row, "role_code").unwrap_or(Value::Null)),
        ("maxReply".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("max_reply")))),
        ("publish".to_string(), Value::Bool(row.get("publish"))),
        ("reply".to_string(), Value::Bool(row.get("reply"))),
        ("visible".to_string(), Value::Bool(row.get("visible"))),
    ]))
}

/// GET permission — 全局权限概要（x_bbs_permission 聚合）。
pub async fn u2_permission_root(pool: Extension<Pool>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one(
            "SELECT COUNT(*) FROM x_bbs_permission WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(total_row.get::<_, i64>(0)))),
        // 无显式权限配置时与 Java 默认一致：登录即可发帖回帖。
        ("defaultPublishable".to_string(), Value::Bool(true)),
        ("defaultReplyPublishable".to_string(), Value::Bool(true)),
    ])))))
}

async fn permissions_by_column(pool: Extension<Pool>, column: &str, value: &str) -> ApiResult {
    // column 仅来自本文件常量调用（forum_id / section_id / role_code）。
    let sql = format!(
        "SELECT id, code, name, forum_id, section_id, role_code, max_reply, publish, reply, visible \
         FROM x_bbs_permission WHERE deleted_at IS NULL AND {} = $1",
        column
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(sql.as_str(), &[&value]).await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Array(
        rows.iter().map(permission_row_to_value).collect(),
    ))))
}

pub async fn u2_permission_admin_forum(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    permissions_by_column(pool, "forum_id", &id).await
}

pub async fn u2_permission_admin_section(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    permissions_by_column(pool, "section_id", &id).await
}

pub async fn u2_permission_admin_role(pool: Extension<Pool>, Path(code): Path<String>) -> ApiResult {
    permissions_by_column(pool, "role_code", &code).await
}

// ══════════════════════════════════════════════════════════════════
// setting 域（5 条；写操作 admin）
// ══════════════════════════════════════════════════════════════════

fn setting_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), row_opt_json::<String>(row, "name").unwrap_or(Value::Null)),
        ("code".to_string(), row_opt_json::<String>(row, "code").unwrap_or(Value::Null)),
        ("value".to_string(), row_opt_json::<String>(row, "value").unwrap_or(Value::Null)),
    ]))
}

async fn query_settings(client: &PgClient, filter: Option<(&str, &str)>) -> Result<Vec<Value>, AppError> {
    let rows = match filter {
        Some((col, val)) => {
            // col 仅由本模块常量字面量传入（"code"/"id"）。
            let sql = format!(
                "SELECT id, name, code, value FROM x_bbs_config_setting \
                 WHERE deleted_at IS NULL AND {} = $1 ORDER BY code",
                col
            );
            client.query(sql.as_str(), &[&val]).await
        }
        None => {
            client
                .query(
                    "SELECT id, name, code, value FROM x_bbs_config_setting \
                     WHERE deleted_at IS NULL ORDER BY code",
                    &[],
                )
                .await
        }
    }
    .map_err(|_| AppError::Internal)?;
    Ok(rows.iter().map(setting_row_to_value).collect())
}

/// GET setting/bbsName — 论坛名称（BBS_NAME 配置，缺省 O2社区）。
pub async fn u2_setting_bbs_name(pool: Extension<Pool>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = query_settings(&client, Some(("code", "BBS_NAME"))).await?;
    let name = data
        .first()
        .and_then(|v| v.get("value"))
        .and_then(|v| v.as_str())
        .unwrap_or("O2社区")
        .to_string();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("bbsName".to_string(), Value::String(name)),
    ])))))
}

/// GET user/setting/{id} — 单条配置。
pub async fn u2_setting_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = query_settings(&client, Some(("id", &id))).await?;
    match data.into_iter().next() {
        Some(v) => Ok(Json(ActionResult::success(v))),
        None => Ok(Json(ActionResult::error("setting not found"))),
    }
}

/// GET user/setting/all — 全量配置。
pub async fn u2_setting_all(pool: Extension<Pool>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = query_settings(&client, None).await?;
    Ok(Json(ActionResult::success(Value::Array(data))))
}

/// PUT user/setting — 更新/新增配置（admin；按 id 更新，未命中则插入）。
pub async fn u2_setting_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let code = match body_str(&body, &["code"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("code is required".to_string())),
    };
    let value = body_str(&body, &["value"]).unwrap_or_default();
    let name = body_str(&body, &["name"]).unwrap_or_else(|| code.clone());
    let creator = session.person_unique.clone();
    let id = body_str(&body, &["id"]).unwrap_or_else(|| Uuid::new_v4().to_string());

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute(
            "UPDATE x_bbs_config_setting SET value = $1, name = $2, update_time = NOW() \
             WHERE id = $3 AND deleted_at IS NULL",
            &[&value, &name, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if affected == 0 {
        client
            .execute(
                "INSERT INTO x_bbs_config_setting (id, name, code, value, creator) VALUES ($1, $2, $3, $4, $5)",
                &[&id, &name, &code, &value, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("code".to_string(), Value::String(code)),
        ("value".to_string(), Value::String(value)),
        ("updated".to_string(), Value::Bool(affected > 0)),
    ])))))
}

/// PUT user/setting/code — 按 code 查配置（Java getByCode）。
pub async fn u2_setting_get_by_code(pool: Extension<Pool>, body: axum::extract::Json<Value>) -> ApiResult {
    let code = match body_str(&body, &["code"]) {
        Some(v) => v,
        None => return Err(AppError::BadRequest("code is required".to_string())),
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = query_settings(&client, Some(("code", &code))).await?;
    Ok(Json(ActionResult::success(Value::Array(data))))
}

// ══════════════════════════════════════════════════════════════════
// userinfo 域（2 条）
// ══════════════════════════════════════════════════════════════════

/// GET userinfo/update/nick/name/{person}?nickname= — 更新 BBS 昵称（UPSERT）。
pub async fn u2_userinfo_update_nick(
    pool: Extension<Pool>,
    Path(person): Path<String>,
    Query(q): Query<HashMap<String, String>>,
) -> ApiResult {
    let nickname = q
        .get("nickname")
        .or_else(|| q.get("nickName"))
        .cloned()
        .filter(|s| !s.trim().is_empty());
    let Some(nickname) = nickname else {
        return Err(AppError::BadRequest("nickname query param is required".to_string()));
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let existing = client
        .query_opt(
            "SELECT id FROM x_bbs_user_info WHERE person = $1 AND deleted_at IS NULL",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match existing {
        Some(row) => {
            let id: String = row.get("id");
            client
                .execute(
                    "UPDATE x_bbs_user_info SET nick_name = $1, update_time = NOW() WHERE id = $2",
                    &[&nickname, &id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("person".to_string(), Value::String(person)),
                ("nickname".to_string(), Value::String(nickname)),
                ("updated".to_string(), Value::Bool(true)),
            ])))))
        }
        None => {
            let id = Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_bbs_user_info (id, person, nick_name) VALUES ($1, $2, $3)",
                    &[&id, &person, &nickname],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("person".to_string(), Value::String(person)),
                ("nickname".to_string(), Value::String(nickname)),
                ("updated".to_string(), Value::Bool(false)),
            ])))))
        }
    }
}

/// PUT userinfo/filterUserInfo — 按昵称模糊过滤 BBS 用户。
pub async fn u2_userinfo_filter(pool: Extension<Pool>, body: axum::extract::Json<Value>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = match body_str(&body, &["name", "nickName", "key"]) {
        Some(key) => {
            let pattern = format!("%{}%", like_escape(&key));
            client
                .query(
                    "SELECT id, person, nick_name, icon, signature FROM x_bbs_user_info \
                     WHERE deleted_at IS NULL AND nick_name ILIKE $1 ESCAPE '\\' \
                     ORDER BY create_time LIMIT 200",
                    &[&pattern],
                )
                .await
        }
        None => {
            client
                .query(
                    "SELECT id, person, nick_name, icon, signature FROM x_bbs_user_info \
                     WHERE deleted_at IS NULL ORDER BY create_time LIMIT 200",
                    &[],
                )
                .await
        }
    }
    .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                (
                    "person".to_string(),
                    r.get::<_, Option<String>>("person").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "nickname".to_string(),
                    r.get::<_, Option<String>>("nick_name").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "icon".to_string(),
                    r.get::<_, Option<String>>("icon").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "signature".to_string(),
                    r.get::<_, Option<String>>("signature").map(Value::String).unwrap_or(Value::Null),
                ),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

// ══════════════════════════════════════════════════════════════════
// attachment / subjectattach 域（元数据真实 SQL；二进制流 501）
// ══════════════════════════════════════════════════════════════════

const ATTACHMENT_TABLE: &str = "x_bbs_attachment";
const SUBJECT_ATTACHMENT_TABLE: &str = "x_bbs_subject_attachment";

fn attachment_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("subjectId".to_string(), row_opt_json::<String>(row, "subject_id").unwrap_or(Value::Null)),
        ("name".to_string(), row_opt_json::<String>(row, "name").unwrap_or(Value::Null)),
        ("extension".to_string(), row_opt_json::<String>(row, "extension").unwrap_or(Value::Null)),
        ("url".to_string(), row_opt_json::<String>(row, "url").unwrap_or(Value::Null)),
        ("description".to_string(), row_opt_json::<String>(row, "description").unwrap_or(Value::Null)),
        (
            "length".to_string(),
            Value::Number(serde_json::Number::from(
                row.get::<_, Option<i64>>("length").unwrap_or(0),
            )),
        ),
    ]))
}

/// GET attachment/{id} — 附件元数据。
pub async fn u2_attachment_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, subject_id, name, extension, url, description, length FROM x_bbs_attachment \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match rows.first() {
        Some(r) => Ok(Json(ActionResult::success(attachment_row_to_value(r)))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

/// GET attachment/list/subject/{subjectId} — 按主题列附件。
pub async fn u2_attachment_list_by_subject(
    pool: Extension<Pool>,
    Path(subject_id): Path<String>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, subject_id, name, extension, url, description, length FROM x_bbs_attachment \
             WHERE subject_id = $1 AND deleted_at IS NULL ORDER BY create_time",
            &[&subject_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Array(
        rows.iter().map(attachment_row_to_value).collect(),
    ))))
}

/// DELETE attachment/{id} — 软删附件（owner 门禁）。
pub async fn u2_attachment_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    match gate_attachment_owner(&pool, ATTACHMENT_TABLE, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("attachment not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_bbs_attachment SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("deleted".to_string(), Value::Bool(affected > 0)),
            ])))))
        }
    }
}

/// GET subjectattach/{id} — 主题附件元数据。
pub async fn u2_subjectattach_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, subject_id, name, extension, url, description, length FROM x_bbs_subject_attachment \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match rows.first() {
        Some(r) => Ok(Json(ActionResult::success(attachment_row_to_value(r)))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

/// GET subjectattach/list/subject/{id} — 真实查询（替换 unwrap_or_default 存根）。
pub async fn u2_subjectattach_list(pool: Extension<Pool>, Path(subject_id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, subject_id, name, extension, url, description, length FROM x_bbs_subject_attachment \
             WHERE subject_id = $1 AND deleted_at IS NULL ORDER BY create_time",
            &[&subject_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Array(
        rows.iter().map(attachment_row_to_value).collect(),
    ))))
}

/// GET subjectattach/{id}/binary/base64/{size} — 存量字节转 base64。
/// Java 版会按 size 缩放图片；无图像引擎时返回原始字节 base64（size 仅透传）。
pub async fn u2_subjectattach_base64(pool: Extension<Pool>, Path((id, size)): Path<(String, i64)>) -> ApiResult {
    if size <= 0 || size > 4096 {
        return Err(AppError::BadRequest("size must be within (0, 4096]".to_string()));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT content FROM x_bbs_subject_attachment WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => {
            let content: Option<Vec<u8>> = r.get("content");
            match content {
                Some(bytes) => Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("size".to_string(), Value::Number(serde_json::Number::from(size))),
                    ("base64".to_string(), Value::String(base64_encode(&bytes))),
                ]))))),
                None => Ok(Json(ActionResult::error("attachment has no binary content"))),
            }
        }
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

/// DELETE subjectattach/{id} — 软删主题附件（owner 门禁）。
pub async fn u2_subjectattach_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    match gate_attachment_owner(&pool, SUBJECT_ATTACHMENT_TABLE, &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("attachment not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let affected = client
                .execute(
                    "UPDATE x_bbs_subject_attachment SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("deleted".to_string(), Value::Bool(affected > 0)),
            ])))))
        }
    }
}

// ══════════════════════════════════════════════════════════════════
// shutup 补充 + subject 统计/搜索/过滤分页
// ══════════════════════════════════════════════════════════════════

/// GET shutup/get/shutup — 当前会话人的禁言记录。
pub async fn u2_shutup_get_mine(pool: Extension<Pool>, session: Extension<shared::session::Session>) -> ApiResult {
    let person = session.person_unique.clone();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, person, reason, create_time FROM x_bbs_shutup \
             WHERE person = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                (
                    "person".to_string(),
                    r.get::<_, Option<String>>("person").map(Value::String).unwrap_or(Value::Null),
                ),
                ("reason".to_string(), row_opt_json::<String>(r, "reason").unwrap_or(Value::Null)),
                ("createTime".to_string(), Value::String(r.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}

/// DELETE shutup/{id} — 解除禁言（admin 门禁；对齐 ShutupAction.delete 管理语义）。
pub async fn u2_shutup_delete_admin(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    let pool = pool.0;
    u2_require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let affected = client
        .execute("DELETE FROM x_bbs_shutup WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Bool(affected > 0)),
    ])))))
}

/// GET subject/statgrade/sectionName/{s}/subjectType/{t} — 按版块与类型统计等级分布。
pub async fn u2_statgrade(
    pool: Extension<Pool>,
    Path((section_name, subject_type)): Path<(String, String)>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT subject_grade, COUNT(*) AS total FROM x_bbs_topic \
             WHERE deleted_at IS NULL AND ($1 = '-' OR section_name = $1) \
             AND ($2 = '-' OR subject_type = $2) GROUP BY subject_grade",
            &[&section_name, &subject_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut grades = serde_json::Map::new();
    let mut total = 0i64;
    for r in &rows {
        let count: i64 = r.get("total");
        total += count;
        let grade = r
            .get::<_, Option<String>>("subject_grade")
            .unwrap_or_else(|| "none".to_string());
        grades.insert(grade, Value::Number(serde_json::Number::from(count)));
    }
    grades.insert("total".to_string(), Value::Number(serde_json::Number::from(total)));
    Ok(Json(ActionResult::success(Value::Object(grades))))
}

/// PUT subject/search/list/page/{page}/count/{count} — 关键词参数化搜索。
pub async fn u2_subject_search_page(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let keyword = body_str(&body, &["keyword", "key", "title", "search"]).unwrap_or_default();
    let offset = page.saturating_sub(1).saturating_mul(count);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    if keyword.is_empty() {
        let rows = client
            .query(
                "SELECT id, title, content, creator, forum_id FROM x_bbs_topic WHERE deleted_at IS NULL \
                 ORDER BY create_time DESC LIMIT $1 OFFSET $2",
                &[&count, &offset],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        let total_row = client
            .query_one("SELECT COUNT(*) FROM x_bbs_topic WHERE deleted_at IS NULL", &[])
            .await
            .map_err(|_| AppError::Internal)?;
        return search_response(&rows, total_row.get::<_, i64>(0));
    }

    // 关键词走参数化 ILIKE + 通配符转义，杜绝注入通配扫描。
    let pattern = format!("%{}%", like_escape(&keyword));
    let rows = client
        .query(
            "SELECT id, title, content, creator, forum_id FROM x_bbs_topic \
             WHERE deleted_at IS NULL AND (title ILIKE $1 ESCAPE '\\' OR content ILIKE $1 ESCAPE '\\') \
             ORDER BY create_time DESC LIMIT $2 OFFSET $3",
            &[&pattern, &count, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one(
            "SELECT COUNT(*) FROM x_bbs_topic \
             WHERE deleted_at IS NULL AND (title ILIKE $1 ESCAPE '\\' OR content ILIKE $1 ESCAPE '\\')",
            &[&pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    search_response(&rows, total_row.get::<_, i64>(0))
}

fn search_response(rows: &[deadpool_postgres::tokio_postgres::Row], total: i64) -> ApiResult {
    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                ("title".to_string(), Value::String(r.get("title"))),
                (
                    "content".to_string(),
                    r.get::<_, Option<String>>("content")
                        .map(Value::String)
                        .unwrap_or_else(|| Value::String(String::new())),
                ),
                (
                    "creator".to_string(),
                    r.get::<_, Option<String>>("creator").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "forumId".to_string(),
                    r.get::<_, Option<String>>("forum_id").map(Value::String).unwrap_or(Value::Null),
                ),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// POST subject/filter/listsubjectinfo/page/{page}/count/{count} — 带体过滤的分页。
pub async fn u2_subject_listsubjectinfo_page(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
    body: axum::extract::Json<Value>,
) -> ApiResult {
    let offset = page.saturating_sub(1).saturating_mul(count);
    let forum_id = body_str(&body, &["forumId"]);
    let subject_type = body_str(&body, &["subjectType"]);
    let creator = body_str(&body, &["creator", "person"]);

    // 占位符编号随过滤参数动态递增；page/count 为服务端解析的路径整数（可信），
    // LIMIT/OFFSET 直接内插，用户可控值一律占位符绑定。
    let mut conds: Vec<String> = vec!["deleted_at IS NULL".to_string()];
    let mut sparams: Vec<String> = Vec::new();
    if let Some(v) = forum_id {
        sparams.push(v);
        conds.push(format!("forum_id = ${}", sparams.len()));
    }
    if let Some(v) = subject_type {
        sparams.push(v);
        conds.push(format!("subject_type = ${}", sparams.len()));
    }
    if let Some(v) = creator {
        sparams.push(v);
        conds.push(format!("(creator = ${0} OR author_id = ${0})", sparams.len()));
    }
    let where_sql = conds.join(" AND ");
    let count_sql = format!("SELECT COUNT(*) FROM x_bbs_topic WHERE {}", where_sql);
    let select_sql = format!(
        "SELECT id, title, content, creator, forum_id, section_name, subject_type FROM x_bbs_topic \
         WHERE {} ORDER BY create_time DESC LIMIT {} OFFSET {}",
        where_sql, count, offset
    );

    let filter_params: Vec<&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)> =
        sparams.iter().map(|s| s as &(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)).collect();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let total_row = client
        .query_one(count_sql.as_str(), &filter_params[..])
        .await
        .map_err(|_| AppError::Internal)?;
    let rows = client
        .query(select_sql.as_str(), &filter_params[..])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|r| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(r.get("id"))),
                ("title".to_string(), Value::String(r.get("title"))),
                (
                    "content".to_string(),
                    r.get::<_, Option<String>>("content")
                        .map(Value::String)
                        .unwrap_or_else(|| Value::String(String::new())),
                ),
                (
                    "creator".to_string(),
                    r.get::<_, Option<String>>("creator").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "forumId".to_string(),
                    r.get::<_, Option<String>>("forum_id").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "sectionName".to_string(),
                    r.get::<_, Option<String>>("section_name").map(Value::String).unwrap_or(Value::Null),
                ),
                (
                    "subjectType".to_string(),
                    r.get::<_, Option<String>>("subject_type").map(Value::String).unwrap_or(Value::Null),
                ),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(total_row.get::<_, i64>(0)))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

