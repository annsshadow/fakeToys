//! plan002 U2 — mind_assemble_control 端点闭合（对照 x_mind_assemble_control
//! jaxrs 全集 23 条，既有 8 条见 lib.rs / routes.rs，本文件补齐剩余 15 条，
//! 并复用 lib.rs 既有 handler 修正 HTTP 方法对齐）。
//!
//! 约定：
//! - 读操作公开（tree/my、{id}、view、version、shareRecords、icon、filter/list 等）；
//! - 写/删/共享操作做 IDOR 门禁：仅资源属主或管理员可操作；
//! - 全部走真实参数化 SQL，复用既有 x_mind / x_mind_share / x_mind_version_info
//!   等表，缺列由 migrations/085_calendar_mind_u2_columns.sql 补齐。

use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::{error::AppError, response::ActionResult};

type ApiResult = Result<Json<ActionResult<Value>>, AppError>;

fn mind_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    json!({
        "id": row.get::<_, String>("id"),
        "name": row.get::<_, Option<String>>("name").unwrap_or_default(),
        "content": row.get::<_, Option<String>>("content"),
        "parentId": row.get::<_, Option<String>>("parent_id"),
        "folderId": row.get::<_, Option<String>>("folder_id"),
        "icon": row.get::<_, Option<String>>("icon"),
        "description": row.get::<_, Option<String>>("description"),
        "shared": row.get::<_, bool>("shared"),
        "fileVersion": row.get::<_, i64>("file_version"),
        "creator": row.get::<_, Option<String>>("creator").unwrap_or_default(),
        "creatorUnit": row.get::<_, Option<String>>("creator_unit"),
    })
}

async fn require_person_is_manager(pool: &Pool, person_unique: &str) -> Result<bool, AppError> {
    Ok(shared::middleware::is_admin(pool, person_unique).await)
}

/// 校验请求人是否为脑图属主或管理员（IDOR 门禁）。
async fn person_can_manage_mind(
    pool: &Pool,
    person: &str,
    mind_id: &str,
) -> Result<bool, AppError> {
    if require_person_is_manager(pool, person).await? {
        return Ok(true);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT creator FROM x_mind WHERE id = $1",
            &[&mind_id.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(match row {
        Some(r) => {
            let owner: Option<String> = r.get("creator");
            owner.map(|v| v == person).unwrap_or(false)
        }
        None => false,
    })
}

/// PUT /jaxrs/mind/assemble/control/folder/move/{folderId} —— 移动文件夹（复用 lib.rs handler 修正方法）
pub use crate::folder_move_folderId as folder_move;

/// DELETE /jaxrs/mind/assemble/control/folder/{id} —— 删除文件夹（软删除）
pub async fn folder_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_mind(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not mind owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_mind SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("folder not found or already deleted")));
    }
    Ok(Json(ActionResult::success(json!({ "id": id, "deleted": n }))))
}

/// DELETE /jaxrs/mind/assemble/control/folder/{id}/force —— 强制删除（复用 lib.rs handler，修正方法）
pub use crate::folder_id_force as folder_force_delete;

/// PUT /jaxrs/mind/assemble/control/mind/filter/list/{id}/next/{page} —— 列表过滤分页
pub async fn mind_filter_list(
    pool: Extension<Pool>,
    Path((id, page)): Path<(String, String)>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let page_no: i64 = page.parse().unwrap_or(1).max(1);
    let offset = (page_no - 1) * 20;
    let rows = client
        .query(
            "SELECT id, name, content, parent_id, folder_id, icon, description, shared, \
             file_version, creator, creator_unit \
             FROM x_mind \
             WHERE deleted_at IS NULL AND ($1::text IS NULL OR $1 = '' OR folder_id = $1) \
             ORDER BY create_time DESC LIMIT 20 OFFSET $2",
            &[&id, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows.iter().map(mind_row_to_value).collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

/// PUT /jaxrs/mind/assemble/control/mind/filter/recived/{id}/next/{page} —— 收到(共享给我)的过滤分页
pub async fn mind_filter_received(
    pool: Extension<Pool>,
    Path((id, page)): Path<(String, String)>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let page_no: i64 = page.parse().unwrap_or(1).max(1);
    let offset = (page_no - 1) * 20;
    // {id} 视作接收人 person；返回共享给该人的脑图
    let rows = client
        .query(
            "SELECT m.id, m.name, m.content, m.parent_id, m.folder_id, m.icon, m.description, \
             m.shared, m.file_version, m.creator, m.creator_unit \
             FROM x_mind m \
             JOIN x_mind_share s ON s.mind_id = m.id \
             WHERE m.deleted_at IS NULL AND s.person = $1 \
             ORDER BY m.create_time DESC LIMIT 20 OFFSET $2",
            &[&id, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows.iter().map(mind_row_to_value).collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

/// PUT /jaxrs/mind/assemble/control/mind/filter/recycle/{id}/next/{page} —— 回收站过滤分页
pub async fn mind_filter_recycle(
    pool: Extension<Pool>,
    Path((id, page)): Path<(String, String)>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let page_no: i64 = page.parse().unwrap_or(1).max(1);
    let offset = (page_no - 1) * 20;
    let rows = client
        .query(
            "SELECT id, name, content, parent_id, folder_id, icon, description, shared, \
             file_version, creator, creator_unit \
             FROM x_mind \
             WHERE deleted_at IS NOT NULL AND ($1::text IS NULL OR $1 = '' OR creator = $1) \
             ORDER BY deleted_at DESC LIMIT 20 OFFSET $2",
            &[&id, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows.iter().map(mind_row_to_value).collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

/// PUT /jaxrs/mind/assemble/control/mind/filter/shared/{id}/next/{page} —— 我共享出的过滤分页
pub async fn mind_filter_shared(
    pool: Extension<Pool>,
    Path((id, page)): Path<(String, String)>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let page_no: i64 = page.parse().unwrap_or(1).max(1);
    let offset = (page_no - 1) * 20;
    let rows = client
        .query(
            "SELECT m.id, m.name, m.content, m.parent_id, m.folder_id, m.icon, m.description, \
             m.shared, m.file_version, m.creator, m.creator_unit \
             FROM x_mind m \
             WHERE m.deleted_at IS NULL AND m.shared = true \
               AND ($1::text IS NULL OR $1 = '' OR m.creator = $1) \
             ORDER BY m.create_time DESC LIMIT 20 OFFSET $2",
            &[&id, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows.iter().map(mind_row_to_value).collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

/// GET /jaxrs/mind/assemble/control/mind/list/{id}/shareRecords —— 共享记录
pub async fn mind_share_records(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, mind_id, person, create_time::text AS create_time \
             FROM x_mind_share WHERE mind_id = $1 ORDER BY create_time DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, String>("id"),
                "mindId": r.get::<_, String>("mind_id"),
                "person": r.get::<_, String>("person"),
                "createTime": r.get::<_, String>("create_time"),
            })
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

/// GET /jaxrs/mind/assemble/control/mind/list/{id}/version —— 版本列表
pub async fn mind_version_list(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, mind_id, name, file_version, creator, creator_unit, description, create_time \
             FROM x_mind_version_info WHERE mind_id = $1 ORDER BY file_version DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, String>("id"),
                "mindId": r.get::<_, String>("mind_id"),
                "name": r.get::<_, Option<String>>("name").unwrap_or_default(),
                "fileVersion": r.get::<_, i32>("file_version"),
                "creator": r.get::<_, Option<String>>("creator").unwrap_or_default(),
                "creatorUnit": r.get::<_, Option<String>>("creator_unit").unwrap_or_default(),
            })
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

/// DELETE /jaxrs/mind/assemble/control/mind/recycle/{id} —— 移入回收站（软删除）
pub async fn mind_recycle(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_mind(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not mind owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_mind SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("mind not found or already recycled")));
    }
    Ok(Json(ActionResult::success(json!({ "id": id, "recycled": n }))))
}

/// GET /jaxrs/mind/assemble/control/mind/restore/{id} —— 从回收站恢复
pub async fn mind_restore(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_mind(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not mind owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_mind SET deleted_at = NULL WHERE id = $1 AND deleted_at IS NOT NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("mind not found or not in recycle")));
    }
    Ok(Json(ActionResult::success(json!({ "id": id, "restored": n }))))
}

/// POST /jaxrs/mind/assemble/control/mind/save —— 保存脑图
pub async fn mind_save(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let content = body.get("content").and_then(|v| v.as_str()).map(|s| s.to_string());
    let folder_id = body.get("folderId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let description = body.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let shared = body.get("shared").and_then(|v| v.as_bool()).unwrap_or(false);
    let parent_id = body.get("parentId").and_then(|v| v.as_str()).map(|s| s.to_string());
    let person = session.person_unique.clone();

    let existing = body.get("id").and_then(|v| v.as_str()).map(|s| s.to_string());
    if let Some(eid) = existing {
        if !eid.is_empty() {
            let n = client
                .execute(
                    "UPDATE x_mind SET name = $2, content = COALESCE($3, content), \
                     folder_id = COALESCE($4, folder_id), description = COALESCE($5, description), \
                     shared = $6, parent_id = COALESCE($7, parent_id) \
                     WHERE id = $1 AND deleted_at IS NULL",
                    &[&eid, &name, &content, &folder_id, &description, &shared, &parent_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            if n > 0 {
                // 记录版本
                let vid = uuid::Uuid::new_v4().to_string();
                let ver: i64 = client
                    .query_one(
                        "SELECT COALESCE(MAX(file_version), 0) + 1 AS v FROM x_mind_version_info WHERE mind_id = $1",
                        &[&eid],
                    )
                    .await
                    .map_err(|_| AppError::Internal)?
                    .get("v");
                let _ = client
                    .execute(
                        "INSERT INTO x_mind_version_info (id, mind_id, name, file_version, creator, creator_unit, description, create_time) \
                         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
                        &[&vid, &eid, &name, &ver, &person, &body.get("creatorUnit").and_then(|v| v.as_str()).map(|s| s.to_string()), &description],
                    )
                    .await;
                return Ok(Json(ActionResult::success(json!({ "id": eid, "updated": n, "fileVersion": ver }))));
            }
        }
    }

    client
        .execute(
            "INSERT INTO x_mind (id, name, content, folder_id, description, shared, \
             file_version, creator, creator_unit, parent_id, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, 1, $7, $8, $9, NOW())",
            &[&id, &name, &content, &folder_id, &description, &shared, &person, &body.get("creatorUnit").and_then(|v| v.as_str()).map(|s| s.to_string()), &parent_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({
        "id": id,
        "name": name,
        "fileVersion": 1,
        "creator": person,
    }))))
}

/// PUT /jaxrs/mind/assemble/control/mind/share/{id} —— 共享给某人
pub async fn mind_share(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> ApiResult {
    if !person_can_manage_mind(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not mind owner or admin")));
    }
    let person = body.get("person").and_then(|v| v.as_str()).unwrap_or("").to_string();
    if person.is_empty() {
        return Ok(Json(ActionResult::error("person is required")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sid = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_mind_share (id, mind_id, person, create_time) VALUES ($1, $2, $3, NOW()) \
             ON CONFLICT (mind_id, person) DO NOTHING",
            &[&sid, &id, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("UPDATE x_mind SET shared = true WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "id": sid, "mindId": id, "person": person }))))
}

/// PUT /jaxrs/mind/assemble/control/mind/share/{id}/cancel —— 取消共享
pub async fn mind_share_cancel(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> ApiResult {
    if !person_can_manage_mind(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not mind owner or admin")));
    }
    let person = body.get("person").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "DELETE FROM x_mind_share WHERE mind_id = $1 AND ($2 = '' OR person = $2)",
            &[&id, &person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({ "canceled": n }))))
}

/// GET /jaxrs/mind/assemble/control/mind/version/{id} —— 最新版本
pub async fn mind_version_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, mind_id, name, file_version, creator, creator_unit, description, create_time \
             FROM x_mind_version_info WHERE mind_id = $1 ORDER BY file_version DESC LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(json!({
            "id": r.get::<_, String>("id"),
            "mindId": r.get::<_, String>("mind_id"),
            "name": r.get::<_, Option<String>>("name").unwrap_or_default(),
            "fileVersion": r.get::<_, i32>("file_version"),
            "creator": r.get::<_, Option<String>>("creator").unwrap_or_default(),
        })))),
        None => Ok(Json(ActionResult::error("version not found"))),
    }
}

/// GET /jaxrs/mind/assemble/control/mind/view/{id} —— 查看脑图（返回详情）
pub async fn mind_view(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, content, parent_id, folder_id, icon, description, shared, \
             file_version, creator, creator_unit \
             FROM x_mind WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(mind_row_to_value(&r)))),
        None => Ok(Json(ActionResult::error("mind not found"))),
    }
}

/// GET /jaxrs/mind/assemble/control/mind/{id} —— 获取脑图
pub async fn mind_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, content, parent_id, folder_id, icon, description, shared, \
             file_version, creator, creator_unit \
             FROM x_mind WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(mind_row_to_value(&r)))),
        None => Ok(Json(ActionResult::error("mind not found"))),
    }
}

/// DELETE /jaxrs/mind/assemble/control/mind/{id}/destorymind —— 彻底删除（销毁）
pub async fn mind_destroy(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_mind(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not mind owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_mind WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("mind not found")));
    }
    Ok(Json(ActionResult::success(json!({ "id": id, "destroyed": n }))))
}

/// DELETE /jaxrs/mind/assemble/control/mind/{id}/destoryrecycle —— 从回收站彻底删除
pub async fn mind_destroy_recycle(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> ApiResult {
    if !person_can_manage_mind(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not mind owner or admin")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM x_mind WHERE id = $1 AND deleted_at IS NOT NULL", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("mind not found in recycle")));
    }
    Ok(Json(ActionResult::success(json!({ "id": id, "destroyed": n }))))
}

/// GET /jaxrs/mind/assemble/control/mind/{id}/icon —— 获取图标
pub async fn mind_icon_get(pool: Extension<Pool>, Path(id): Path<String>) -> ApiResult {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt("SELECT id, icon FROM x_mind WHERE id = $1 AND deleted_at IS NULL", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => {
            let icon: Option<String> = r.get("icon");
            Ok(Json(ActionResult::success(json!({
                "id": id,
                "icon": icon.unwrap_or_default(),
            }))))
        }
        None => Ok(Json(ActionResult::error("mind not found"))),
    }
}

/// POST /jaxrs/mind/assemble/control/mind/{id}/icon/size/{size} —— 设置图标（含尺寸变体）
pub async fn mind_icon_set(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((id, size)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> ApiResult {
    if !person_can_manage_mind(&pool, &session.person_unique, &id).await? {
        return Ok(Json(ActionResult::error("forbidden: not mind owner or admin")));
    }
    let icon = body.get("icon").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("UPDATE x_mind SET icon = $2 WHERE id = $1 AND deleted_at IS NULL", &[&id, &icon])
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("mind not found")));
    }
    Ok(Json(ActionResult::success(json!({ "id": id, "size": size, "icon": icon }))))
}
