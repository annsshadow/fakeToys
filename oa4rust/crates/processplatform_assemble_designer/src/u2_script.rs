//! W7：process 脚本设计器写路径（u2 闭合）。
//!
//! 存储：PP_E_SCRIPT（parity 表，037 迁移补 deleted_at）+ PP_E_SCRIPTVERSION。
//! 每次 create/update 落一行版本快照（xcode 携带内容），版本历史面板经既有
//! GET scriptversion/list/script/{scriptId} 读取。

use axum::{extract::Extension, Json};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use uuid::Uuid;

fn body_str<'a>(body: &'a Value, key: &str) -> Option<&'a str> {
    body.get(key).and_then(Value::as_str)
}

/// POST /jaxrs/processplatform/assemble/designer/script —— 新建脚本并落 v1 版本快照
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
    let code = body_str(&body, "code").unwrap_or_default().to_string();
    let application = body_str(&body, "application")
        .unwrap_or_default()
        .to_string();
    let id = Uuid::new_v4().to_string();
    let creator = session.person_unique.clone();

    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    tx.execute(
        "INSERT INTO PP_E_SCRIPT (xid, xname, xapplication, xcode, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\", id, creator) \
         VALUES ($1, $2, NULLIF($3, ''), $4, $5, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), $1, $5)",
        &[&id, &name, &application, &code, &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.execute(
        "INSERT INTO PP_E_SCRIPTVERSION (xid, xscript, xname, xcode, xversion, \"xcreatorPerson\", \"xcreateTime\", id) \
         VALUES ($1, $2, $3, $4, 1, $5, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), $1)",
        &[&Uuid::new_v4().to_string(), &id, &name, &code, &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]),
    ))))
}

/// PUT /jaxrs/processplatform/assemble/designer/script/{id} —— 更新脚本并落新版本快照
pub async fn update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = body_str(&body, "name").map(str::to_string);
    let code = body_str(&body, "code").map(str::to_string);
    let creator = session.person_unique.clone();

    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let n = tx
        .execute(
            "UPDATE PP_E_SCRIPT SET xname = COALESCE($2, xname), xcode = COALESCE($3, xcode), \
             \"xupdateTime\" = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE xid = $1 AND deleted_at IS NULL",
            &[&id, &name, &code],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("script not found")));
    }
    let code_snapshot = code.unwrap_or_default();
    let name_snapshot = name.unwrap_or_default();
    let version = tx
        .query_one(
            "SELECT COALESCE(MAX(xversion), 0) + 1 AS next FROM PP_E_SCRIPTVERSION WHERE xscript = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get::<_, i64>("next");
    tx.execute(
        "INSERT INTO PP_E_SCRIPTVERSION (xid, xscript, xname, xcode, xversion, \"xcreatorPerson\", \"xcreateTime\", id) \
         VALUES ($1, $2, $3, $4, $5, $6, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), $1)",
        &[&Uuid::new_v4().to_string(), &id, &name_snapshot, &code_snapshot, &version, &creator],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// DELETE /jaxrs/processplatform/assemble/designer/script/{id} —— 软删除
pub async fn delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE PP_E_SCRIPT SET deleted_at = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE xid = $1 AND deleted_at IS NULL",
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
