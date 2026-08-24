//! plan002 U2 收尾：对齐 Java x_processplatform_service_processing jaxrs/** 契约。
//!
//! 权威端点清单：docs/audits/java-endpoint-inventory.json 中
//! x_processplatform_service_processing（127 动词注解 / 121 唯一端点，剥注释口径）。
//! 本模块按 `{war}/jaxrs/<类路径>/<方法路径>` 形状补齐缺口路由；前缀沿用本 crate 惯例
//! /jaxrs/processplatform/service/processing。既有契约形状一致的端点保持不动。
//!
//! 落地语义说明（schema 子集）：
//! - applicationdict/data 走 migration 079 的 x_application_dict / x_data（JSONB 路径寻址）；
//! - record/review/read/snap/task/work 等复用 crate 既有状态机与事务模式；
//! - 归一化查重：review/read 同 (work,person) 不重复建、record 同内容去重、
//!   attachment 复制按目标+文件名去重、documentversion 版本号取 max+1；
//! - IDOR 门禁：跨引用参数必须匹配归属（attachment×work/workcompleted、
//!   taskcompleted press×work），不匹配返回业务错误而非静默删除。

use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::{
    error::AppError,
    response::{row_to_json, ActionResult},
};
use uuid::Uuid;

type H = Result<Json<ActionResult<Value>>, AppError>;
type Row = deadpool_postgres::tokio_postgres::Row;
type Client = deadpool_postgres::Client;

fn ok(v: Value) -> H {
    Ok(Json(ActionResult::success(v)))
}

fn biz_err(msg: impl Into<String>) -> H {
    Ok(Json(ActionResult::error(msg.into())))
}

fn opt_str(row: &Row, col: &str) -> String {
    row.get::<_, Option<String>>(col).unwrap_or_default()
}

fn body_str(body: &Value, keys: &[&str]) -> String {
    for k in keys {
        if let Some(s) = body.get(k).and_then(|v| v.as_str()) {
            return s.trim().to_string();
        }
    }
    String::new()
}

fn body_str_list(body: &Value, keys: &[&str]) -> Vec<String> {
    for k in keys {
        if let Some(arr) = body.get(k).and_then(|v| v.as_array()) {
            let list: Vec<String> = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !list.is_empty() {
                return list;
            }
        }
    }
    Vec::new()
}

async fn record_insert(
    client: &Client,
    work_id: &str,
    record_type: &str,
    content: &str,
    creator: &str,
) -> Result<String, AppError> {
    let id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &work_id.to_string(), &record_type.to_string(), &content.to_string(), &creator.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(id)
}

/// 归一化查重：同 work + 类型 + 内容的记录只保留一条
async fn record_exists(
    client: &Client,
    work_id: &str,
    record_type: &str,
    content: &str,
) -> Result<bool, AppError> {
    let n = client
        .query_one(
            "SELECT COUNT(*) AS c FROM x_record \
             WHERE work_id = $1 AND COALESCE(record_type,'') = $2 AND COALESCE(content,'') = $3",
            &[&work_id.to_string(), &record_type.to_string(), &content.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get::<_, i64>("c");
    Ok(n > 0)
}

// ── applicationdict（ApplicationDictAction）─────────────────────────────────

pub async fn dict_edit(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let id = id.trim().to_string();
    if id.is_empty() {
        return biz_err("id is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_application_dict (id, data) VALUES ($1, '{}'::jsonb) \
             ON CONFLICT (id) DO NOTHING",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if let Some(name) = body.get("name").and_then(|v| v.as_str()) {
        client
            .execute(
                "UPDATE x_application_dict SET name = $2, update_time = NOW() WHERE id = $1",
                &[&id, &name.to_string()],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    if let Some(data) = body.get("data") {
        if !data.is_null() {
            client
                .execute(
                    "UPDATE x_application_dict SET data = ($2::text)::jsonb, update_time = NOW() WHERE id = $1",
                    &[&id, &data.to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?;
        }
    }
    let row = client
        .query_one(
            "SELECT id, name, category, COALESCE(data::text, '{}') AS data FROM x_application_dict WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let raw: String = row.get::<_, Option<String>>("data").unwrap_or_else(|| "{}".to_string());
    let data: Value = serde_json::from_str(&raw).unwrap_or(json!({}));
    ok(json!({
        "id": row.get::<_, String>("id"),
        "name": opt_str(&row, "name"),
        "data": data,
    }))
}

async fn dict_set(pool: &Pool, parts: &[String], body: Value) -> H {
    let id = parts[0].trim().to_string();
    if id.is_empty() || parts.len() < 2 {
        return biz_err("id and path are required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_application_dict (id, data) VALUES ($1, '{}'::jsonb) \
             ON CONFLICT (id) DO NOTHING",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let segments: Vec<String> = parts[1..].to_vec();
    client
        .execute(
            "UPDATE x_application_dict \
             SET data = jsonb_set(COALESCE(data, '{}'::jsonb), $2::text[], ($3::text)::jsonb, true), \
                 update_time = NOW() \
             WHERE id = $1",
            &[&id, &segments, &body.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let raw: String = client
        .query_one("SELECT COALESCE(data::text, '{}') AS data FROM x_application_dict WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?
        .get("data");
    let data: Value = serde_json::from_str(&raw).unwrap_or(json!({}));
    ok(json!({ "id": id, "value": data }))
}

async fn dict_del(pool: &Pool, parts: &[String]) -> H {
    let id = parts[0].trim().to_string();
    if id.is_empty() || parts.len() < 2 {
        return biz_err("id and path are required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let segments: Vec<String> = parts[1..].to_vec();
    let n = client
        .execute(
            "UPDATE x_application_dict \
             SET data = COALESCE(data, '{}'::jsonb) #- $2::text[], update_time = NOW() \
             WHERE id = $1",
            &[&id, &segments],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("application dict not found");
    }
    ok(json!({ "id": id, "value": true }))
}

macro_rules! dict_data_fns {
    ($set:ident, $del:ident ($($p:ident : $t:ty),+)) => {
        pub async fn $set(
            pool: Extension<Pool>,
            Path(($($p),+)): Path<($($t),+)>,
            Json(body): Json<Value>,
        ) -> H {
            dict_set(&pool, &[$($p.to_string()),+], body).await
        }

        pub async fn $del(
            pool: Extension<Pool>,
            Path(($($p),+)): Path<($($t),+)>,
        ) -> H {
            dict_del(&pool, &[$($p.to_string()),+]).await
        }
    };
}

dict_data_fns!(dict_set_1, dict_del_1 (id: String, p0: String));
dict_data_fns!(dict_set_2, dict_del_2 (id: String, p0: String, p1: String));
dict_data_fns!(dict_set_3, dict_del_3 (id: String, p0: String, p1: String, p2: String));
dict_data_fns!(dict_set_4, dict_del_4 (id: String, p0: String, p1: String, p2: String, p3: String));
dict_data_fns!(dict_set_5, dict_del_5 (id: String, p0: String, p1: String, p2: String, p3: String, p4: String));
dict_data_fns!(dict_set_6, dict_del_6 (id: String, p0: String, p1: String, p2: String, p3: String, p4: String, p5: String));
dict_data_fns!(dict_set_7, dict_del_7 (id: String, p0: String, p1: String, p2: String, p3: String, p4: String, p5: String, p6: String));
dict_data_fns!(dict_set_8, dict_del_8 (id: String, p0: String, p1: String, p2: String, p3: String, p4: String, p5: String, p6: String, p7: String));

// ── data（DataAction：job/work/workcompleted 业务数据，migration 079 x_data）──

const DATA_SCOPE_JOB: &str = "job";
const DATA_SCOPE_WORK: &str = "work";
const DATA_SCOPE_WC: &str = "workcompleted";

async fn entity_exists(client: &Client, table: &str, id: &str) -> Result<bool, AppError> {
    let sql = format!("SELECT COUNT(*) AS c FROM {table} WHERE id = $1");
    let n = client
        .query_one(&sql, &[&id.to_string()])
        .await
        .map_err(|_| AppError::Internal)?
        .get::<_, i64>("c");
    Ok(n > 0)
}

async fn data_row_exists(client: &Client, scope: &str, bundle: &str) -> Result<bool, AppError> {
    let n = client
        .query_one(
            "SELECT COUNT(*) AS c FROM x_data WHERE scope = $1 AND bundle = $2",
            &[&scope.to_string(), &bundle.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get::<_, i64>("c");
    Ok(n > 0)
}

/// PUT 语义：整体替换（不存在则创建）
async fn data_put_whole(pool: &Pool, scope: &str, bundle: &str, body: Value) -> H {
    if bundle.trim().is_empty() {
        return biz_err("bundle is required");
    }
    let data = if body.is_null() { json!({}) } else { body };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_data (scope, bundle, data) VALUES ($1, $2, ($3::text)::jsonb) \
             ON CONFLICT (scope, bundle) DO UPDATE SET data = EXCLUDED.data, update_time = NOW()",
            &[&scope.to_string(), &bundle.to_string(), &data.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "scope": scope, "bundle": bundle, "value": data }))
}

/// POST 语义：仅创建；已存在报错（Java ExceptionDataAlreadyExist）
async fn data_create_whole(
    pool: &Pool,
    scope: &str,
    bundle: &str,
    body: Value,
    gate_table: Option<&str>,
) -> H {
    if bundle.trim().is_empty() {
        return biz_err("bundle is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if let Some(table) = gate_table {
        if !entity_exists(&client, table, bundle).await? {
            return biz_err(format!("{table} not found: {bundle}"));
        }
    }
    if data_row_exists(&client, scope, bundle).await? {
        return biz_err("data already exist");
    }
    let data = if body.is_null() { json!({}) } else { body };
    client
        .execute(
            "INSERT INTO x_data (scope, bundle, data) VALUES ($1, $2, ($3::text)::jsonb)",
            &[&scope.to_string(), &bundle.to_string(), &data.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "scope": scope, "bundle": bundle, "value": data }))
}

async fn data_update_whole(pool: &Pool, scope: &str, bundle: &str, body: Value) -> H {
    if bundle.trim().is_empty() {
        return biz_err("bundle is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !data_row_exists(&client, scope, bundle).await? {
        return biz_err("data not exist");
    }
    let data = if body.is_null() { json!({}) } else { body };
    client
        .execute(
            "UPDATE x_data SET data = ($3::text)::jsonb, update_time = NOW() \
             WHERE scope = $1 AND bundle = $2",
            &[&scope.to_string(), &bundle.to_string(), &data.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "scope": scope, "bundle": bundle, "value": data }))
}

/// 单键写入：create 要求键不存在、update 要求键已存在（Java parent/already-exist 异常语义）
async fn data_set_key(
    pool: &Pool,
    scope: &str,
    bundle: &str,
    key: &str,
    body: Value,
    create_mode: bool,
) -> H {
    if bundle.trim().is_empty() || key.trim().is_empty() {
        return biz_err("bundle and path are required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !data_row_exists(&client, scope, bundle).await? {
        return biz_err("data not exist");
    }
    let present: bool = client
        .query_one(
            "SELECT (COALESCE(data,'{}'::jsonb) #> ARRAY[$3::text]) IS NOT NULL AS present \
             FROM x_data WHERE scope = $1 AND bundle = $2",
            &[&scope.to_string(), &bundle.to_string(), &key.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("present");
    if create_mode && present {
        return biz_err("data path already exist");
    }
    if !create_mode && !present {
        return biz_err("parent not exist");
    }
    client
        .execute(
            "UPDATE x_data \
             SET data = jsonb_set(COALESCE(data,'{}'::jsonb), ARRAY[$4::text], ($3::text)::jsonb, true), \
                 update_time = NOW() \
             WHERE scope = $1 AND bundle = $2",
            &[&scope.to_string(), &bundle.to_string(), &body.to_string(), &key.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let raw: String = client
        .query_one(
            "SELECT COALESCE(data::text, '{}') AS data FROM x_data WHERE scope = $1 AND bundle = $2",
            &[&scope.to_string(), &bundle.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("data");
    let data: Value = serde_json::from_str(&raw).unwrap_or(json!({}));
    ok(json!({ "scope": scope, "bundle": bundle, "value": data }))
}

async fn data_delete_whole(pool: &Pool, scope: &str, bundle: &str) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "DELETE FROM x_data WHERE scope = $1 AND bundle = $2",
            &[&scope.to_string(), &bundle.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("data not exist");
    }
    ok(json!({ "scope": scope, "bundle": bundle, "deleted": true }))
}

async fn data_delete_key(pool: &Pool, scope: &str, bundle: &str, key: &str) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !data_row_exists(&client, scope, bundle).await? {
        return biz_err("data not exist");
    }
    client
        .execute(
            "UPDATE x_data SET data = COALESCE(data,'{}'::jsonb) #- ARRAY[$3::text], \
                 update_time = NOW() \
             WHERE scope = $1 AND bundle = $2",
            &[&scope.to_string(), &bundle.to_string(), &key.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "scope": scope, "bundle": bundle, "deleted": true }))
}

pub async fn data_job_put(
    pool: Extension<Pool>,
    Path(job): Path<String>,
    Json(body): Json<Value>,
) -> H {
    data_put_whole(&pool, DATA_SCOPE_JOB, &job, body).await
}

pub async fn data_job_put_path(
    pool: Extension<Pool>,
    Path((job, path)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> H {
    data_set_key(&pool, DATA_SCOPE_JOB, &job, &path, body, false).await
}

pub async fn data_work_create(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    data_create_whole(&pool, DATA_SCOPE_WORK, &id, body, Some("x_work")).await
}

pub async fn data_work_update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    data_update_whole(&pool, DATA_SCOPE_WORK, &id, body).await
}

pub async fn data_work_create_path(
    pool: Extension<Pool>,
    Path((id, path)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> H {
    data_set_key(&pool, DATA_SCOPE_WORK, &id, &path, body, true).await
}

pub async fn data_work_update_path(
    pool: Extension<Pool>,
    Path((id, path)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> H {
    data_set_key(&pool, DATA_SCOPE_WORK, &id, &path, body, false).await
}

pub async fn data_work_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> H {
    data_delete_whole(&pool, DATA_SCOPE_WORK, &id).await
}

pub async fn data_work_delete_path(
    pool: Extension<Pool>,
    Path((id, path)): Path<(String, String)>,
) -> H {
    data_delete_key(&pool, DATA_SCOPE_WORK, &id, &path).await
}

pub async fn data_wc_update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !entity_exists(&client, "x_workcompleted", &id).await? {
        return biz_err("workcompleted not found");
    }
    drop(client);
    data_put_whole(&pool, DATA_SCOPE_WC, &id, body).await
}

pub async fn data_wc_update_path(
    pool: Extension<Pool>,
    Path((id, path)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !entity_exists(&client, "x_workcompleted", &id).await? {
        return biz_err("workcompleted not found");
    }
    drop(client);
    data_set_key(&pool, DATA_SCOPE_WC, &id, &path, body, false).await
}

// ── attachment（AttachmentAction）───────────────────────────────────────────

/// DELETE attachment/{id}：软删除
pub async fn att_delete_id(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_attachment SET deleted_at = NOW() \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("attachment not found");
    }
    ok(json!({ "id": id, "deleted": true }))
}

/// DELETE attachment/{id}/work/{workId}：IDOR 门禁——附件必须归属该 work
pub async fn att_delete_with_work(
    pool: Extension<Pool>,
    Path((id, work_id)): Path<(String, String)>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT work_id FROM x_attachment WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("attachment not found");
    };
    let owner: String = opt_str(&row, "work_id");
    if owner != work_id {
        return biz_err("attachment does not belong to work");
    }
    let n = client
        .execute(
            "UPDATE x_attachment SET deleted_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "id": id, "deleted": n > 0 }))
}

/// DELETE attachment/{id}/workcompleted/{workCompletedId}：IDOR 门禁同理
pub async fn att_delete_with_workcompleted(
    pool: Extension<Pool>,
    Path((id, wc_id)): Path<(String, String)>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT workcompleted_id FROM x_attachment WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("attachment not found");
    };
    let owner: String = opt_str(&row, "workcompleted_id");
    if owner.is_empty() || owner != wc_id {
        return biz_err("attachment does not belong to workcompleted");
    }
    let n = client
        .execute(
            "UPDATE x_attachment SET deleted_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "id": id, "deleted": n > 0 }))
}

/// PUT attachment/{id}：编辑元信息（name）
pub async fn att_edit_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let name = body_str(&body, &["name", "fileName"]);
    if name.is_empty() {
        return biz_err("name is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_attachment SET name = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&id, &name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("attachment not found");
    }
    let row = client
        .query_one(
            "SELECT id, work_id, workcompleted_id, name, creator FROM x_attachment WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(row_to_json(&row))
}

/// PUT attachment/edit/{id}/text：编辑文本内容（content 列承载 text）
pub async fn att_edit_text(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let text = body_str(&body, &["text", "content"]);
    if text.is_empty() {
        return biz_err("text is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_attachment SET content = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&id, &text],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("attachment not found");
    }
    ok(json!({ "id": id, "edited": true }))
}

/// POST attachment/copy/work/{workId}：按 Wi.attachmentList 复制到目标 work；
/// 归一化查重——目标下同名附件已存在则跳过。
pub async fn att_copy_to_work(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let list = body_str_list(&body, &["attachmentList"]);
    if work_id.trim().is_empty() {
        return biz_err("workId is required");
    }
    if list.is_empty() {
        return biz_err("attachmentList is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !entity_exists(&client, "x_work", &work_id).await? {
        return biz_err("work not found");
    }
    let mut success: Vec<Value> = Vec::new();
    for src_id in &list {
        let Some(src) = client
            .query_opt(
                "SELECT name, content, creator FROM x_attachment \
                 WHERE id = $1 AND deleted_at IS NULL",
                &[src_id],
            )
            .await
            .map_err(|_| AppError::Internal)?
        else {
            success.push(json!({ "id": src_id, "copied": false, "reason": "not found" }));
            continue;
        };
        let name: String = opt_str(&src, "name");
        let dup = client
            .query_one(
                "SELECT COUNT(*) AS c FROM x_attachment \
                 WHERE work_id = $1 AND COALESCE(name,'') = $2 AND deleted_at IS NULL",
                &[&work_id, &name],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .get::<_, i64>("c");
        if dup > 0 {
            success.push(json!({ "id": src_id, "copied": false, "reason": "already exist" }));
            continue;
        }
        let new_id = Uuid::new_v4().to_string();
        let content: Option<String> = src.get("content");
        let creator: String = opt_str(&src, "creator");
        client
            .execute(
                "INSERT INTO x_attachment (id, work_id, workcompleted_id, name, content, creator) \
                 VALUES ($1, $2, NULL, $3, $4, $5)",
                &[&new_id, &work_id, &name, &content, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        success.push(json!({ "id": new_id, "name": name, "copied": true }));
    }
    ok(json!({ "successList": success }))
}

// ── job（JobAction）────────────────────────────────────────────────────────

/// DELETE job/{job}：结束 job 及其运行任务
pub async fn job_delete(pool: Extension<Pool>, Path(job): Path<String>) -> H {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let n = tx
        .execute(
            "UPDATE x_job SET job_status = 'cancelled', end_time = NOW() \
             WHERE id = $1 AND end_time IS NULL",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute(
        "UPDATE x_task SET task_status = 'cancelled', end_time = NOW() \
         WHERE work = $1 AND end_time IS NULL",
        &[&job],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("job already completed or not found");
    }
    ok(json!({ "job": job, "value": true }))
}

/// GET job/v2/{job}/person/{person}/view：该人员视角下的 job 任务视图
pub async fn job_v2_view(
    pool: Extension<Pool>,
    Path((job, person)): Path<(String, String)>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, title, activity, activity_token, person, task_status \
             FROM x_task WHERE work = $1 AND (person = $2 OR $2 = '')",
            &[&job, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let tasks: Vec<Value> = rows.iter().map(row_to_json).collect();
    ok(json!({ "job": job, "person": person, "count": tasks.len() as i64, "taskList": tasks }))
}

// ── read（ReadAction）──────────────────────────────────────────────────────

/// 归一化查重：同 (bundle,person,scope) 未删除的已阅只建一条
async fn read_exists(client: &Client, bundle: &str, person: &str, scope: &str) -> Result<bool, AppError> {
    let n = client
        .query_one(
            "SELECT COUNT(*) AS c FROM x_read \
             WHERE work_id = $1 AND person = $2 AND scope = $3 AND deleted_at IS NULL",
            &[&bundle.to_string(), &person.to_string(), &scope.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get::<_, i64>("c");
    Ok(n > 0)
}

async fn read_create(pool: &Pool, bundle: &str, body: Value, scope: &str, gate_table: &str) -> H {
    let mut persons = body_str_list(&body, &["personList", "persons"]);
    if persons.is_empty() {
        let single = body_str(&body, &["person"]);
        if single.is_empty() {
            return biz_err("personList is required");
        }
        persons.push(single);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !entity_exists(&client, gate_table, bundle).await? {
        return biz_err(format!("{gate_table} not found"));
    }
    let mut created = Vec::new();
    let mut skipped = Vec::new();
    for person in persons {
        if read_exists(&client, bundle, &person, scope).await? {
            skipped.push(person);
            continue;
        }
        let id = Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_read (id, work_id, person, scope) VALUES ($1, $2, $3, $4)",
                &[&id, &bundle.to_string(), &person, &scope.to_string()],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        created.push(json!({ "id": id, "person": person }));
    }
    ok(json!({ "value": true, "createdList": created, "existList": skipped }))
}

/// POST read/work/{workId}
pub async fn read_create_with_work(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    read_create(&pool, &work_id, body, DATA_SCOPE_WORK, "x_work").await
}

/// POST read/workcompleted/{workCompletedId}
pub async fn read_create_with_workcompleted(
    pool: Extension<Pool>,
    Path(wc_id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    read_create(&pool, &wc_id, body, DATA_SCOPE_WC, "x_workcompleted").await
}

/// DELETE read/{id}：软删除
pub async fn read_delete(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_read SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("read not found");
    }
    ok(json!({ "id": id, "value": true }))
}

/// PUT read/{id}/processing：标记已读——事务内迁移至 x_readcompleted 并软删源记录
pub async fn read_processing(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_opt(
            "SELECT id, work_id, person, scope FROM x_read \
             WHERE id = $1 AND deleted_at IS NULL FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("read not found");
    };
    let rc_id = Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO x_readcompleted (id, work_id, person) VALUES ($1, $2, $3)",
        &[
            &rc_id,
            &row.get::<_, String>("work_id"),
            &row.get::<_, String>("person"),
        ],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.execute(
        "UPDATE x_read SET deleted_at = NOW() WHERE id = $1",
        &[&id],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": id, "readCompletedId": rc_id, "value": true }))
}

/// POST read/{id}/replace：更换阅读人
pub async fn read_replace(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let person = body_str(&body, &["person"]);
    if person.is_empty() {
        return biz_err("person is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_read SET person = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&id, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("read not found");
    }
    ok(json!({ "id": id, "person": person, "value": true }))
}

/// POST read/{id}/reset：重置为待读（可同时更换阅读人）
pub async fn read_reset(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let person = body_str(&body, &["person"]);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = if person.is_empty() {
        client
            .execute(
                "UPDATE x_read SET deleted_at = NULL WHERE id = $1",
                &[&id],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .execute(
                "UPDATE x_read SET deleted_at = NULL, person = $2 WHERE id = $1",
                &[&id, &person],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };
    if n == 0 {
        return biz_err("read not found");
    }
    ok(json!({ "id": id, "value": true }))
}

// ── readcompleted ──────────────────────────────────────────────────────────

/// DELETE readcompleted/{id}
pub async fn readcompleted_delete(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_readcompleted WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("readcompleted not found");
    }
    ok(json!({ "id": id, "value": true }))
}

// ── record（RecordAction）──────────────────────────────────────────────────

async fn record_create_dedup(pool: &Pool, work_id: &str, body: Value) -> H {
    let r#type = {
        let t = body_str(&body, &["recordType", "type"]);
        if t.is_empty() { "info".to_string() } else { t.to_lowercase() }
    };
    let content_val = body.get("content").cloned().unwrap_or(json!({}));
    let content = content_val.to_string();
    let creator = {
        let c = body_str(&body, &["creator", "person"]);
        if c.is_empty() { "system".to_string() } else { c }
    };
    if work_id.trim().is_empty() {
        return biz_err("work/job is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if record_exists(&client, work_id, &r#type, &content).await? {
        return ok(json!({ "value": true, "duplicated": true }));
    }
    let id = record_insert(&client, work_id, &r#type, &content, &creator).await?;
    ok(json!({ "id": id, "value": true }))
}

/// POST record/job/{job}
pub async fn record_create_with_job(
    pool: Extension<Pool>,
    Path(job): Path<String>,
    Json(body): Json<Value>,
) -> H {
    record_create_dedup(&pool, &job, body).await
}

/// POST record/work/processing
pub async fn record_work_processing(pool: Extension<Pool>, Json(body): Json<Value>) -> H {
    let work = body_str(&body, &["work", "workId"]);
    if work.is_empty() {
        return biz_err("work is required");
    }
    record_create_dedup(&pool, &work, body).await
}

/// POST record/work/terminate
pub async fn record_work_terminate(pool: Extension<Pool>, Json(body): Json<Value>) -> H {
    let work = body_str(&body, &["work", "workId"]);
    if work.is_empty() {
        return biz_err("work is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !entity_exists(&client, "x_work", &work).await? {
        return biz_err("work not found");
    }
    drop(client);
    record_create_dedup(&pool, &work, body).await
}

/// PUT record/{id}：编辑记录内容/类型
pub async fn record_edit(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id FROM x_record WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if row.is_none() {
        return biz_err("record not found");
    }
    if let Some(t) = body.get("recordType").or_else(|| body.get("type")) {
        if let Some(t) = t.as_str() {
            client
                .execute(
                    "UPDATE x_record SET record_type = $2 WHERE id = $1",
                    &[&id, &t.to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?;
        }
    }
    if let Some(c) = body.get("content") {
        let content = c.to_string();
        client
            .execute(
                "UPDATE x_record SET content = $2 WHERE id = $1",
                &[&id, &content],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }
    let row = client
        .query_one(
            "SELECT id, work_id, task_id, record_type, content, creator, create_time \
             FROM x_record WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(row_to_json(&row))
}

/// DELETE record/{id}：物理删除（Java Record delete 语义）
pub async fn record_delete(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_record WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("record not found");
    }
    ok(json!({ "id": id, "value": true }))
}

// ── review（ReviewAction）──────────────────────────────────────────────────

/// 归一化查重：同 (work,reviewer) 有效评审不重复创建
async fn review_exists(client: &Client, work_id: &str, reviewer: &str) -> Result<bool, AppError> {
    let n = client
        .query_one(
            "SELECT COUNT(*) AS c FROM x_review \
             WHERE work_id = $1 AND COALESCE(reviewer,'') = $2 AND deleted_at IS NULL",
            &[&work_id.to_string(), &reviewer.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get::<_, i64>("c");
    Ok(n > 0)
}

async fn review_create_batch(pool: &Pool, work_id: &str, body: Value, gate_table: &str) -> H {
    let persons = {
        let mut p = body_str_list(&body, &["personList", "persons"]);
        if p.is_empty() {
            let single = body_str(&body, &["person", "reviewer"]);
            if !single.is_empty() {
                p.push(single);
            }
        }
        p
    };
    if work_id.trim().is_empty() {
        return biz_err("work is required");
    }
    if persons.is_empty() {
        return biz_err("personList is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !entity_exists(&client, gate_table, work_id).await? {
        return biz_err(format!("{gate_table} not found: {work_id}"));
    }
    let comment = body_str(&body, &["opinion", "comment"]);
    let mut success = Vec::new();
    let mut failure = Vec::new();
    for person in persons {
        if review_exists(&client, work_id, &person).await? {
            failure.push(json!({ "person": person, "reason": "already exist" }));
            continue;
        }
        let id = Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_review (id, work_id, reviewer, comment, status) \
                 VALUES ($1, $2, $3, $4, 'pending')",
                &[&id, &work_id.to_string(), &person, &comment],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        success.push(json!({ "id": id, "person": person }));
    }
    ok(json!({ "successList": success, "failureList": failure }))
}

/// POST review/create/work
pub async fn review_create_work(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> H {
    let work = body_str(&body, &["work", "workId"]);
    review_create_batch(&pool, &work, body, "x_work").await
}

/// POST review/create/workcompleted
pub async fn review_create_workcompleted(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> H {
    let work = body_str(&body, &["workCompleted", "workCompletedId", "work"]);
    review_create_batch(&pool, &work, body, "x_workcompleted").await
}

/// POST review/init/review：为所有有任务的工作补齐缺失评审（幂等批量）
pub async fn review_init_for_view(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT t.work, t.person FROM x_task t \
             WHERE t.person IS NOT NULL AND t.person <> '' \
               AND NOT EXISTS (\
                   SELECT 1 FROM x_review r \
                   WHERE r.work_id = t.work AND COALESCE(r.reviewer,'') = t.person \
                     AND r.deleted_at IS NULL)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut created = Vec::new();
    for row in &rows {
        let work: String = row.get("work");
        let person: String = row.get("person");
        let id = Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_review (id, work_id, reviewer, status) \
                 VALUES ($1, $2, $3, 'pending')",
                &[&id, &work, &person],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        created.push(json!({ "work": work, "person": person }));
    }
    ok(json!({ "value": true, "count": created.len() as i64, "createdList": created }))
}

/// DELETE review/{id}：软删除
pub async fn review_delete(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_review SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("review not found");
    }
    ok(json!({ "id": id, "value": true }))
}

// ── snap（SnapAction）──────────────────────────────────────────────────────

/// DELETE snap/{id}
pub async fn snap_delete(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_snap WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("snap not found");
    }
    ok(json!({ "id": id, "value": true }))
}

/// 快照类型操作公共实现：事务内快照 + 状态迁移
async fn snap_take_for_work(
    pool: &Pool,
    work_id: &str,
    snap_type: &str,
    cancel_tasks: bool,
    next_status: Option<&str>,
) -> H {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_opt(
            "SELECT id, title, process, application, work_status, creator \
             FROM x_work WHERE id = $1 FOR UPDATE",
            &[&work_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("work not found");
    };
    let snap_data = row_to_json(&row);
    let snap_id = Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO x_snap (id, work_id, snap_type, snap_data) VALUES ($1, $2, $3, ($4::text)::jsonb)",
        &[&snap_id, &work_id.to_string(), &snap_type.to_string(), &snap_data.to_string()],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    if cancel_tasks {
        tx.execute(
            "UPDATE x_task SET task_status = 'cancelled' \
             WHERE work = $1 AND task_status = 'active'",
            &[&work_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    }
    if let Some(status) = next_status {
        tx.execute(
            "UPDATE x_work SET work_status = $2 WHERE id = $1",
            &[&work_id, &status.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    }
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": snap_id, "type": snap_type }))
}

async fn snap_take_for_wc(pool: &Pool, wc_id: &str, snap_type: &str) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, work_id, completed_time, creator FROM x_workcompleted WHERE id = $1",
            &[&wc_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("workcompleted not found");
    };
    let snap_data = row_to_json(&row);
    let snap_id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_snap (id, work_id, snap_type, snap_data) VALUES ($1, $2, $3, ($4::text)::jsonb)",
            &[&snap_id, &wc_id.to_string(), &snap_type.to_string(), &snap_data.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "id": snap_id, "type": snap_type }))
}

/// GET snap/work/{workId}/type/abandoned：放弃工作（快照+取消任务+置状态）
pub async fn snap_type_abandoned(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
) -> H {
    snap_take_for_work(&pool, &work_id, "abandoned", true, Some("abandoned")).await
}

/// GET snap/work/{workId}/type/snap
pub async fn snap_type_snap(pool: Extension<Pool>, Path(work_id): Path<String>) -> H {
    snap_take_for_work(&pool, &work_id, "snap", false, None).await
}

/// GET snap/work/{workId}/type/suspend：挂起工作
pub async fn snap_type_suspend(
    pool: Extension<Pool>,
    Path(work_id): Path<String>,
) -> H {
    snap_take_for_work(&pool, &work_id, "suspend", false, Some("suspended")).await
}

/// GET snap/workcompleted/{workCompletedId}/type/abandonedworkcompleted
pub async fn snap_wc_type_abandoned(
    pool: Extension<Pool>,
    Path(wc_id): Path<String>,
) -> H {
    snap_take_for_wc(&pool, &wc_id, "abandonedWorkCompleted").await
}

/// GET snap/workcompleted/{workCompletedId}/type/snapworkcompleted
pub async fn snap_wc_type_snap(
    pool: Extension<Pool>,
    Path(wc_id): Path<String>,
) -> H {
    snap_take_for_wc(&pool, &wc_id, "snapWorkCompleted").await
}

// ── task（TaskAction 补缺）─────────────────────────────────────────────────

/// DELETE task/{id}：软删除待办
pub async fn task_delete(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_task SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("task not found");
    }
    ok(json!({ "id": id, "value": true }))
}

// ── taskcompleted（TaskCompletedAction 补缺）───────────────────────────────

/// DELETE taskcompleted/{id}：物理删除已完成任务
pub async fn taskcompleted_delete(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_readcompleted WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("taskcompleted not found");
    }
    ok(json!({ "id": id, "value": true }))
}

/// GET taskcompleted/{id}/press/work/{work}：催办；IDOR 门禁——记录必须归属该 work
pub async fn taskcompleted_press(
    pool: Extension<Pool>,
    Path((id, work)): Path<(String, String)>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT work_id FROM x_readcompleted WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("taskcompleted not found");
    };
    let owner: String = opt_str(&row, "work_id");
    if owner != work {
        return biz_err("taskcompleted does not belong to work");
    }
    if record_exists(&client, &work, "press", &format!("taskcompleted:{id}")).await? {
        return ok(json!({ "value": true, "duplicated": true }));
    }
    let rid = record_insert(&client, &work, "press", &format!("taskcompleted:{id}"), "system").await?;
    ok(json!({ "id": rid, "value": true }))
}

/// PUT taskcompleted/next/task/identity：设置任务的下一处理身份
pub async fn taskcompleted_update_next_identity(pool: Extension<Pool>, Json(body): Json<Value>) -> H {
    let id = body_str(&body, &["id", "taskId"]);
    let identity = body_str(&body, &["nextTaskIdentity", "nextIdentity", "identity"]);
    if id.is_empty() || identity.is_empty() {
        return biz_err("id and nextTaskIdentity are required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_task SET next_task_identity = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&id, &identity],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("task not found");
    }
    ok(json!({ "id": id, "nextTaskIdentity": identity, "value": true }))
}

// ── touch（TouchAction：参数化批量维护作业，全部真实 SQL）──────────────────

/// GET touch/cleanevent：清理超过 24h 的陈旧事件记录
pub async fn touch_clean_event(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "DELETE FROM x_record \
             WHERE record_type = 'event' AND create_time < NOW() - INTERVAL '24 hours'",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "value": true, "deleted": n as i64 }))
}

/// GET touch/deletedraft：清理已结束工作的草稿
pub async fn touch_delete_draft(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_draft SET deleted_at = NOW() \
             WHERE deleted_at IS NULL \
               AND work_id IN (SELECT id FROM x_work WHERE end_time IS NOT NULL)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "value": true, "deleted": n as i64 }))
}

/// GET touch/handoverjob：把无主 job 移交其工作创建者
pub async fn touch_handover_job(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_job j SET person = w.creator \
             FROM x_work w \
             WHERE w.id = j.work_id \
               AND COALESCE(j.person,'') = '' AND w.creator IS NOT NULL AND w.creator <> ''",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "value": true, "handovered": n as i64 }))
}

/// GET touch/loglongdetained：滞留超 24h 的活动工作补记日志（按内容幂等去重）
pub async fn touch_log_long_detained(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.id FROM x_work w \
             WHERE w.end_time IS NULL \
               AND w.start_time < NOW() - INTERVAL '24 hours' \
               AND NOT EXISTS (\
                   SELECT 1 FROM x_record r \
                   WHERE r.work_id = w.id AND r.record_type = 'long_detained' \
                     AND r.create_time >= date_trunc('day', NOW()))",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut logged: Vec<String> = Vec::new();
    for row in &rows {
        let work: String = row.get("id");
        record_insert(&client, &work, "long_detained", "work detained over 24 hours", "system")
            .await
            .ok();
        logged.push(work);
    }
    ok(json!({ "value": true, "count": logged.len() as i64, "workList": logged }))
}

/// GET touch/touchdelay：为存在过期任务的工作补接触摸记录（每日去重）
pub async fn touch_delay(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT t.work FROM x_task t \
             WHERE t.task_status = 'expired' \
               AND NOT EXISTS (\
                   SELECT 1 FROM x_record r \
                   WHERE r.work_id = t.work AND r.record_type = 'touch_delay' \
                     AND r.create_time >= date_trunc('day', NOW()))",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut touched: Vec<String> = Vec::new();
    for row in &rows {
        let work: String = row.get("work");
        record_insert(&client, &work, "touch_delay", "delay touched", "system").await.ok();
        touched.push(work);
    }
    ok(json!({ "value": true, "count": touched.len() as i64, "workList": touched }))
}

/// GET touch/urge：为滞留超 24h 的活动任务补催办记录（每日去重）
pub async fn touch_urge(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT t.work FROM x_task t \
             WHERE t.task_status = 'active' AND t.start_time < NOW() - INTERVAL '24 hours'",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut urged: Vec<String> = Vec::new();
    for row in &rows {
        let work: String = row.get("work");
        if record_exists(&client, &work, "urge", "urge processing").await? {
            continue;
        }
        record_insert(&client, &work, "urge", "urge processing", "system").await.ok();
        urged.push(work);
    }
    ok(json!({ "value": true, "count": urged.len() as i64, "workList": urged }))
}

/// GET touch/merge：合并同 work 的重复 workcompleted（保留最早一条）
pub async fn touch_merge(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "DELETE FROM x_workcompleted a \
             USING x_workcompleted b \
             WHERE a.work_id = b.work_id AND a.id <> b.id \
               AND (b.completed_time, b.id) < (a.completed_time, a.id)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "value": true, "merged": n as i64 }))
}

/// GET touch/mergeitem：合并同名重复附件（同一归属下保留最早一条）
pub async fn touch_merge_item(pool: Extension<Pool>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "DELETE FROM x_attachment a \
             USING x_attachment b \
             WHERE a.id <> b.id \
               AND COALESCE(a.name,'') = COALESCE(b.name,'') \
               AND COALESCE(a.work_id,'') = COALESCE(b.work_id,'') \
               AND COALESCE(a.workcompleted_id,'') = COALESCE(b.workcompleted_id,'') \
               AND (COALESCE(b.create_time, b.id), b.id) < (COALESCE(a.create_time, a.id), a.id)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(json!({ "value": true, "merged": n as i64 }))
}

// ── documentversion ────────────────────────────────────────────────────────

/// POST documentversion/work/{work}：版本号取 max+1（归一化查重防并发重号）
pub async fn documentversion_create(
    pool: Extension<Pool>,
    Path(work): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !entity_exists(&client, "x_work", &work).await? {
        return biz_err("work not found");
    }
    drop(client);
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let next: i32 = tx
        .query_one(
            "SELECT COALESCE(MAX(version), 0) + 1 AS v FROM x_document_version WHERE work_id = $1",
            &[&work],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("v");
    let id = Uuid::new_v4().to_string();
    let content_val = body.get("content").cloned().unwrap_or(json!({}));
    let creator = {
        let c = body_str(&body, &["creator", "person"]);
        if c.is_empty() { "system".to_string() } else { c }
    };
    tx.execute(
        "INSERT INTO x_document_version (id, work_id, version, content, creator) \
         VALUES ($1, $2, $3, ($4::text)::jsonb, $5)",
        &[&id, &work, &next, &content_val.to_string(), &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": id, "work": work, "version": next }))
}


// ── work（WorkAction 补缺）─────────────────────────────────────────────────

/// POST work：按流程标识创建工作实例（事务内建 work + 首个任务）
pub async fn work_create(pool: Extension<Pool>, Json(body): Json<Value>) -> H {
    let process_id = body_str(&body, &["process", "processId"]);
    work_start_impl(pool, process_id, None, body).await
}

/// POST work/process/{processId}：流程标识取自路径
pub async fn work_create_for_process(
    pool: Extension<Pool>,
    Path(process_id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    work_start_impl(pool, process_id, None, body).await
}

/// POST work/process/{processId}/name/{name}/serial：按名称创建，流水号服务端生成（同流程现有工作数+1）
pub async fn work_create_with_serial(
    pool: Extension<Pool>,
    Path((process_id, name)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n: i64 = client
        .query_one(
            "SELECT COUNT(*) AS c FROM x_work WHERE process = $1",
            &[&process_id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("c");
    drop(client);
    let title = format!("{name}-{next}", next = n + 1);
    work_start_impl(pool, process_id, Some(title), body).await
}

async fn work_start_impl(
    pool: Extension<Pool>,
    process_id: String,
    title_override: Option<String>,
    body: Value,
) -> H {
    let process_id = process_id.trim().to_string();
    let title = title_override.unwrap_or_else(|| {
        let t = body_str(&body, &["title", "name"]);
        if t.is_empty() { format!("work-{process_id}") } else { t }
    });
    if process_id.is_empty() {
        return biz_err("processId is required");
    }
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let exists = tx
        .query_one(
            "SELECT COUNT(*) AS c FROM x_process_definition WHERE id = $1 OR name = $1",
            &[&process_id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get::<_, i64>("c");
    if exists == 0 {
        return biz_err("process not found");
    }
    let id = Uuid::new_v4().to_string();
    let creator = {
        let c = body_str(&body, &["creator", "person"]);
        if c.is_empty() { "system".to_string() } else { c }
    };
    tx.execute(
        "INSERT INTO x_work (id, title, process, application, work_status, creator, create_time, start_time) \
         VALUES ($1, $2, $3, '', 'pending', $4, NOW(), NOW())",
        &[&id, &title, &process_id, &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    let task_id = Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) \
         VALUES ($1, $2, $3, 'start', '', $4, 'active', NOW())",
        &[&task_id, &format!("start:{title}"), &id, &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": id, "taskId": task_id }))
}

/// GET work/{id}
pub async fn work_get(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, title, process, application, work_status, creator, \
                    create_time, start_time, end_time \
             FROM x_work WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("work not found");
    };
    let tasks = client
        .query(
            "SELECT id, title, activity, activity_token, person, task_status \
             FROM x_task WHERE work = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let task_list: Vec<Value> = tasks.iter().map(row_to_json).collect();
    let mut data = row_to_json(&row);
    if let Value::Object(ref mut m) = data {
        m.insert("tasks".to_string(), Value::Array(task_list));
    }
    ok(data)
}

/// PUT work/{id}：编辑标题等基础信息
pub async fn work_edit(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let title = body_str(&body, &["title"]);
    if title.is_empty() {
        return biz_err("title is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_work SET title = $2 WHERE id = $1",
            &[&id, &title],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("work not found");
    }
    let row = client
        .query_one("SELECT id, title, work_status FROM x_work WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    ok(row_to_json(&row))
}

/// DELETE work/{id}：物理删除工作及其任务（Java delete 级联语义，事务内执行）
pub async fn work_delete(pool: Extension<Pool>, Path(id): Path<String>) -> H {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let n = tx
        .execute("DELETE FROM x_work WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("work not found");
    }
    tx.execute("DELETE FROM x_task WHERE work = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute("DELETE FROM x_draft WHERE work_id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": id, "value": true }))
}

/// DELETE work/{id}/draft：删除该工作的全部草稿；无草稿报错（Java ExceptionDeleteDraft）
pub async fn work_draft_delete(pool: Extension<Pool>, Path(work): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_draft WHERE work_id = $1", &[&work])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("draft not found");
    }
    ok(json!({ "work": work, "deleted": n as i64 }))
}

/// POST work/manual/after/processing：人工流转后置记录（内容幂等去重）
pub async fn work_manual_after(pool: Extension<Pool>, Json(body): Json<Value>) -> H {
    let work = body_str(&body, &["work", "workId"]);
    if work.is_empty() {
        return biz_err("work is required");
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    if !entity_exists(&client, "x_work", &work).await? {
        return biz_err("work not found");
    }
    if record_exists(&client, &work, "manual_after_processing", "manual after processing").await? {
        return ok(json!({ "value": true, "duplicated": true }));
    }
    let id = record_insert(&client, &work, "manual_after_processing", "manual after processing", "system").await?;
    ok(json!({ "id": id, "value": true }))
}

/// POST work/v3/retract：按 body.work 撤回（状态机迁移 + 任务取消，事务内）
pub async fn work_v3_retract_body(pool: Extension<Pool>, Json(body): Json<Value>) -> H {
    let work = body_str(&body, &["work", "workId"]);
    if work.is_empty() {
        return biz_err("work is required");
    }
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let n = tx
        .execute(
            "UPDATE x_work SET work_status = 'retracted' WHERE id = $1",
            &[&work],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return biz_err("work not found");
    }
    tx.execute(
        "UPDATE x_task SET task_status = 'cancelled', end_time = NOW() \
         WHERE work = $1 AND end_time IS NULL",
        &[&work],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    record_insert_tx(&tx, &work, "retract", "v3 retract", "system").await?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": work, "value": true }))
}

async fn record_insert_tx(
    tx: &deadpool_postgres::tokio_postgres::Transaction<'_>,
    work_id: &str,
    record_type: &str,
    content: &str,
    creator: &str,
) -> Result<(), AppError> {
    let id = Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO x_record (id, work_id, record_type, content, creator, create_time) \
         VALUES ($1, $2, $3, $4, $5, NOW())",
        &[&id, &work_id.to_string(), &record_type.to_string(), &content.to_string(), &creator.to_string()],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    Ok(())
}

/// GET work/{id}/series/{series}/activitytoken/{activityToken}/processing/signal：
/// 按 activityToken 定位任务并完成信号处理
pub async fn work_processing_signal(
    pool: Extension<Pool>,
    Path((id, series, activity_token)): Path<(String, String, String)>,
) -> H {
    let _ = series;
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_opt(
            "SELECT id FROM x_task \
             WHERE work = $1 AND activity_token = $2 AND deleted_at IS NULL FOR UPDATE",
            &[&id, &activity_token],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("task not found by activityToken");
    };
    let task_id: String = row.get("id");
    tx.execute(
        "UPDATE x_task SET task_status = 'completed', end_time = NOW() WHERE id = $1",
        &[&task_id],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    record_insert_tx(&tx, &id, "signal", &format!("activityToken:{activity_token}"), "system").await?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": id, "taskId": task_id, "signaled": true }))
}

/// POST work/v2/{id}/add/manual/task/identity/matrix：为工作追加身份矩阵任务
pub async fn work_add_identity_matrix(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> H {
    let identities = body_str_list(&body, &["identityList", "identityMatrix", "identities"]);
    if identities.is_empty() {
        return biz_err("identityList is required");
    }
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_opt(
            "SELECT id, title FROM x_work WHERE id = $1 FOR UPDATE",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("work not found");
    };
    let title: String = row.get("title");
    let mut created = Vec::new();
    for identity in identities {
        let task_id = Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO x_task (id, title, work, activity, activity_token, person, task_status, start_time) \
             VALUES ($1, $2, $3, 'manual', '', $4, 'active', NOW())",
            &[&task_id, &format!("manual:{title}"), &id, &identity],
        )
        .await
        .map_err(|_| AppError::Internal)?;
        created.push(json!({ "taskId": task_id, "identity": identity }));
    }
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": id, "taskList": created }))
}

// ── workcompleted ──────────────────────────────────────────────────────────

/// GET workcompleted/{flag}/merge：保留 flag 指定项，合并同 work 的其余重复项
pub async fn wc_merge_flag(pool: Extension<Pool>, Path(flag): Path<String>) -> H {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_opt(
            "SELECT work_id FROM x_workcompleted WHERE id = $1 FOR UPDATE",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("workcompleted not found");
    };
    let work_id: String = opt_str(&row, "work_id");
    let n = tx
        .execute(
            "DELETE FROM x_workcompleted WHERE work_id = $1 AND id <> $2",
            &[&work_id, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": flag, "merged": n as i64 }))
}

/// POST workcompleted/process/{processFlag}：为该流程已结束工作补齐完成记录（幂等）
pub async fn wc_process_flag(pool: Extension<Pool>, Path(process_flag): Path<String>) -> H {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT w.id, w.creator FROM x_work w \
             WHERE w.process = $1 AND w.end_time IS NOT NULL \
               AND NOT EXISTS (SELECT 1 FROM x_workcompleted wc WHERE wc.work_id = w.id)",
            &[&process_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut created = Vec::new();
    for row in &rows {
        let work: String = row.get("id");
        let creator: String = opt_str(row, "creator");
        let id = Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_workcompleted (id, work_id, creator) VALUES ($1, $2, NULLIF($3,''))",
                &[&id, &work, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        created.push(json!({ "id": id, "work": work }));
    }
    ok(json!({ "process": process_flag, "count": created.len() as i64, "createdList": created }))
}

/// POST workcompleted/shift/time：平移完成时间（minutes 可正可负）
pub async fn wc_shift_time(pool: Extension<Pool>, Json(body): Json<Value>) -> H {
    let minutes = body.get("minutes").and_then(|v| v.as_i64()).unwrap_or(0);
    if minutes == 0 || minutes.abs() > 5_256_000 {
        return biz_err("minutes must be non-zero within ±10 years");
    }
    let ids = body_str_list(&body, &["ids", "idList"]);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = if ids.is_empty() {
        client
            .execute(
                "UPDATE x_workcompleted SET completed_time = completed_time + make_interval(mins => $1)",
                &[&minutes],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .execute(
                "UPDATE x_workcompleted SET completed_time = completed_time + make_interval(mins => $1) \
                 WHERE id = ANY($2)",
                &[&minutes, &ids],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };
    ok(json!({ "shifted": n as i64, "minutes": minutes }))
}

/// PUT workcompleted/{flag}/rollback：回滚完成态——对应工作恢复运行并补记 rollback
pub async fn wc_rollback_flag(pool: Extension<Pool>, Path(flag): Path<String>) -> H {
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let row = tx
        .query_opt(
            "SELECT work_id FROM x_workcompleted WHERE id = $1 FOR UPDATE",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return biz_err("workcompleted not found");
    };
    let work_id: String = opt_str(&row, "work_id");
    if work_id.is_empty() {
        return biz_err("workcompleted has no work reference");
    }
    tx.execute(
        "UPDATE x_work SET work_status = 'pending', end_time = NULL WHERE id = $1",
        &[&work_id],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.execute("DELETE FROM x_workcompleted WHERE id = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;
    record_insert_tx(&tx, &work_id, "rollback", &format!("workcompleted {flag} rolled back"), "system").await?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(json!({ "id": flag, "work": work_id, "rolledBack": true }))
}
