//! W7：portal 脚本设计器写路径（u2 闭合）。
//!
//! 存储：x_portal_script（084 迁移补 content/portal_id/deleted_at 列）
//! + x_portal_script_version（092 迁移补 content 列）。
//! 每次 create/update 落一行版本快照（含 content），版本历史面板经既有
//! GET scriptversion/list/script/{scriptId} 读取。

use axum::{extract::Extension, Json};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use uuid::Uuid;

fn body_str<'a>(body: &'a Value, key: &str) -> Option<&'a str> {
    body.get(key).and_then(Value::as_str)
}

async fn next_version(
    tx: &deadpool_postgres::tokio_postgres::Transaction<'_>,
    script_id: &str,
) -> Result<i64, AppError> {
    let row = tx
        .query_one(
            "SELECT COALESCE(MAX(CAST(version AS INTEGER)), 0) + 1 AS next \
             FROM x_portal_script_version WHERE script_id = $1",
            &[&script_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.get::<_, i64>("next"))
}

/// POST /jaxrs/portal/assemble/designer/script —— 新建脚本并落 v1 版本快照
pub async fn create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = body_str(&body, "name")
        .unwrap_or_default()
        .trim()
        .to_string();
    if name.is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }
    let content = body_str(&body, "content").unwrap_or_default().to_string();
    let category = body_str(&body, "category").unwrap_or_default().to_string();
    let portal_id = body_str(&body, "portalId").unwrap_or_default().to_string();
    let id = Uuid::new_v4().to_string();
    let creator = session.person_unique.clone();

    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute(
        "INSERT INTO x_portal_script (id, name, flag, category, content, portal_id, creator, creator_person, create_time, update_time) \
         VALUES ($1, $2, $2, $3, $4, NULLIF($5, ''), $6, $6, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
        &[&id, &name, &category, &content, &portal_id, &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    let version = next_version(&tx, &id).await?;
    tx.execute(
        "INSERT INTO x_portal_script_version (id, script_id, version, content, creator, creator_person, create_time) \
         VALUES ($1, $2, $3, $4, $5, $5, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
        &[&Uuid::new_v4().to_string(), &id, &version.to_string(), &content, &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("version".to_string(), Value::Number(version.into())),
        ]),
    ))))
}

/// PUT /jaxrs/portal/assemble/designer/script/{id} —— 更新脚本并落新版本快照
pub async fn update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = body_str(&body, "name").map(str::to_string);
    let content = body_str(&body, "content").map(str::to_string);
    let category = body_str(&body, "category").map(str::to_string);
    let creator = session.person_unique.clone();

    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let n = tx
        .execute(
            "UPDATE x_portal_script SET \
             name = COALESCE($2, name), \
             content = COALESCE($3, content), \
             category = COALESCE($4, category), \
             update_person = $5, update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id, &name, &content, &category, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("script not found")));
    }
    let content_snapshot = content.unwrap_or_default();
    let version = next_version(&tx, &id).await?;
    tx.execute(
        "INSERT INTO x_portal_script_version (id, script_id, version, content, creator, creator_person, create_time) \
         VALUES ($1, $2, $3, $4, $5, $5, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
        &[&Uuid::new_v4().to_string(), &id, &version.to_string(), &content_snapshot, &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
            ("version".to_string(), Value::Number(version.into())),
        ]),
    ))))
}

/// DELETE /jaxrs/portal/assemble/designer/script/{id} —— 软删除
pub async fn delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_portal_script SET deleted_at = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("script not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}
