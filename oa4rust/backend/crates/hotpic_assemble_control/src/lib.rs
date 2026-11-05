// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

#[allow(dead_code, non_snake_case)]
use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub const API_BASE: &str = "/api/hotpic_assemble_control";
pub mod routes;

/// 写面会话存在档：无会话先 401，在任何 `pool.get()` 之前（离线可证）。
fn require_session(
    session: Option<Extension<shared::session::Session>>,
) -> Result<shared::session::Session, AppError> {
    session.map(|Extension(s)| s).ok_or(AppError::Unauthorized)
}

#[cfg(test)]
mod security_gate_tests;
#[cfg(test)]
mod security_gate_tests_b72;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;

/// 内容管理写门禁（安全审计 2026-10 二轮）：轮播图（x_hotpic）与控制配置为
/// 管理台内容，此前 creator 恒写 "system" 且无角色校验，任一登录用户可增删改。
async fn require_admin(pool: &Pool, session: &shared::session::Session) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

pub fn hotpic_assemble_control_router(pool: Pool) -> axum::Router {
    routes::router(pool)
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT COUNT(*) as cnt FROM x_hotpic WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let count: i64 = row.map(|r| r.get("cnt")).unwrap_or(0);
    let enabled = count > 0;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("enabled".to_string(), Value::Bool(enabled)),
            ("cacheEnabled".to_string(), Value::Bool(count > 0)),
            (
                "defaultScale".to_string(),
                Value::Number(serde_json::Number::from_f64(1.0).unwrap()),
            ),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn list_control_panels(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT DISTINCT creator FROM x_hotpic WHERE deleted_at IS NULL ORDER BY creator ASC LIMIT 10",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let creator: String = row.get::<_, Option<String>>("creator").unwrap_or_default();
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(format!("panel-{}", i))),
                ("name".to_string(), Value::String(creator.clone())),
                ("enabled".to_string(), Value::Bool(!creator.is_empty())),
                ("type".to_string(), Value::String("hotpic".to_string())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn update_control_config(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 轮播控制配置写：仅 admin（安全审计 2026-10 二轮）。
    require_admin(&pool, &session).await?;
    let client = shared::db::acquire(&pool).await?;
    let config = body.0;
    tracing::info!("Updating hotpic assemble control config: {:?}", config);

    let id = uuid::Uuid::new_v4().to_string();
    let name = config
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let _enabled = config
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    // 归属键：空键对 = 部署级全局热点图；半截参数 → 400。
    let scope = HotpicScope::from_request(
        config.get("application").and_then(|v| v.as_str()),
        config.get("infoId").and_then(|v| v.as_str()),
    )?;

    let result = client
        .execute(
            "INSERT INTO x_hotpic (id, application, info_id, title, image_url, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[
                &id,
                &scope.application,
                &scope.info_id,
                &name,
                &"",
                &"system",
            ],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(result > 0)),
            ("config".to_string(), config),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn list_control_applications(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT DISTINCT creator as application FROM x_hotpic WHERE deleted_at IS NULL ORDER BY creator ASC",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            // O9：creator 可空（历史脏数据/探针写入），裸 get::<String> 遇 NULL 即
            // panic（arity_probe 实证）——按空串处理（enabled=false）。
            let application: String = row
                .get::<_, Option<String>>("application")
                .unwrap_or_default();
            Value::Object(serde_json::Map::from_iter([
                (
                    "application".to_string(),
                    Value::String(application.clone()),
                ),
                ("name".to_string(), Value::String(application.clone())),
                ("enabled".to_string(), Value::Bool(!application.is_empty())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::hotpic_assemble_control_router(pool)
}

#[derive(Debug, serde::Deserialize)]
#[allow(non_snake_case)]
pub struct HotpicRequest {
    pub title: Option<String>,
    pub imageUrl: Option<String>,
    /// 父实体引用：`x_hotpic.application`（NOT NULL、无 DEFAULT/CHECK）。
    pub application: Option<String>,
    /// 父实体引用：`x_hotpic.info_id`（NOT NULL、无 DEFAULT/CHECK）。
    pub infoId: Option<String>,
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn list_hotpics(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let rows = client
        .query(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 100",
            &[],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn get_hotpic(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

/// `x_hotpic` 的归属键 `(application, info_id)`。
///
/// **两列皆为空串 = 部署级全局热点图**。这个约定此前只存在于数据里（生产库 51 行空键、
/// 读路径不认），2026-11 起在代码里显式化：全局行在任何 application 视图下都可见。
pub(crate) const GLOBAL_APPLICATION: &str = "";
pub(crate) const GLOBAL_INFO_ID: &str = "";

/// 写路径的归属参数：要么全局（空键），要么具体 `(application, infoId)`。
/// 只给其中一个属于半截参数，直接 400——不许把半截当成全局，也不许编造另一半。
struct HotpicScope {
    application: String,
    info_id: String,
}

impl HotpicScope {
    fn from_request(application: Option<&str>, info_id: Option<&str>) -> Result<Self, AppError> {
        let application = application.unwrap_or(GLOBAL_APPLICATION).trim();
        let info_id = info_id.unwrap_or(GLOBAL_INFO_ID).trim();
        match (application.is_empty(), info_id.is_empty()) {
            (true, true) => Ok(Self::global()),
            (false, false) => Ok(Self {
                application: application.to_string(),
                info_id: info_id.to_string(),
            }),
            _ => Err(AppError::BadRequest(
                "application 与 infoId 必须同时提供或同时留空（空键 = 全局热点图）".to_string(),
            )),
        }
    }

    fn global() -> Self {
        Self {
            application: GLOBAL_APPLICATION.to_string(),
            info_id: GLOBAL_INFO_ID.to_string(),
        }
    }
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn create_hotpic(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Json(req): Json<HotpicRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 轮播图 = 管理台内容，仅 admin 可建（安全审计 2026-10 二轮）。
    require_admin(&pool, &session).await?;
    let client = shared::db::acquire(&pool).await?;
    let id = uuid::Uuid::new_v4().to_string();
    let title = req.title.unwrap_or_default();
    let image_url = req.imageUrl.unwrap_or_default();
    // 归属键：空键对 = 部署级全局热点图；半截参数 → 400。
    let scope = HotpicScope::from_request(req.application.as_deref(), req.infoId.as_deref())?;
    // 属主以会话登录人为事实源（原恒写 "system"，属主列形同虚设）。
    let creator = session.person_unique.clone();

    client
        .execute(
            "INSERT INTO x_hotpic (id, application, info_id, title, image_url, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[
                &id,
                &scope.application,
                &scope.info_id,
                &title,
                &image_url,
                &creator,
            ],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("title".to_string(), Value::String(title)),
        ("imageUrl".to_string(), Value::String(image_url)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn save_hotpic(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<HotpicRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 轮播图改 = 管理台内容写：仅 admin（安全审计 2026-10 二轮）。
    require_admin(&pool, &session).await?;
    let client = shared::db::acquire(&pool).await?;
    let title = req.title.unwrap_or_default();
    let image_url = req.imageUrl.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_hotpic SET title = $1, image_url = $2 WHERE id = $3 AND deleted_at IS NULL",
            &[&title, &image_url, &id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    if result == 0 {
        return Ok(Json(ActionResult::error("hotpic not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("title".to_string(), Value::String(title)),
            ("imageUrl".to_string(), Value::String(image_url)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn delete_hotpic(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 轮播图删 = 管理台内容写：仅 admin（安全审计 2026-10 二轮）。
    require_admin(&pool, &session).await?;
    let client = shared::db::acquire(&pool).await?;
    let result = client
        .execute(
            "UPDATE x_hotpic SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    if result == 0 {
        return Ok(Json(ActionResult::error(
            "hotpic not found or already deleted",
        )));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn cipher_hotpic_bbs_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn cipher_hotpic_cms_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn cipher_hotpic_filter_list_page_page_count_count(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let (count, offset) = shared::response::page_window(page, count)?;
    let rows = client
        .query(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET $1",
            &[&offset, &count],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[allow(non_snake_case)]
pub async fn cipher_hotpic_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn user_hotpic_changeTitle(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let Some(Extension(sess)) = session else {
        return Err(AppError::Unauthorized);
    };
    let client = shared::db::acquire(&pool).await?;
    let id = req
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let title = req
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if id.is_empty() {
        return Ok(Json(ActionResult::error("id is required")));
    }

    // 归属门：先取热图属主再判（admin 放行 / 本人放行，归一化比较）。
    let owner: Option<String> = client
        .query_opt(
            "SELECT creator FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
        .map(|r| r.get::<_, Option<String>>("creator").unwrap_or_default());
    let owner = match owner {
        Some(o) if !o.is_empty() => o,
        _ => return Ok(Json(ActionResult::error("hotpic not found"))),
    };
    shared::middleware::require_owner(&pool, &sess, &owner).await?;

    let result = client
        .execute(
            "UPDATE x_hotpic SET title = $1 WHERE id = $2 AND deleted_at IS NULL",
            &[&title, &id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    if result == 0 {
        return Ok(Json(ActionResult::error("hotpic not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("title".to_string(), Value::String(title)),
            ("saved".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn user_hotpic_exists_check(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let person_id = req.get("personId").and_then(|v| v.as_str()).unwrap_or("");
    let _application = req
        .get("application")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if person_id.is_empty() {
        return Ok(Json(ActionResult::success(Value::Bool(false))));
    }

    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM x_hotpic WHERE creator = $1 AND deleted_at IS NULL",
            &[&person_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let count: i64 = row.get("cnt");
    Ok(Json(ActionResult::success(Value::Bool(count > 0))))
}

#[allow(non_snake_case)]
pub async fn user_hotpic_filter_list_page_page_count_count(
    pool: Extension<Pool>,
    Path((page, count)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let (count, offset) = shared::response::page_window(page, count)?;
    let rows = client
        .query(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET $1",
            &[&offset, &count],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[allow(non_snake_case)]
pub async fn user_hotpic_application_infoId(
    pool: Extension<Pool>,
    Path((application, info_id)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            // 读路径：精确键 OR 全局（空键对）。全局行在任何 application 视图下可见；
            // 原实现按 `id = info_id` 查、完全忽略 application，已按归属键修正。
            "SELECT id, application, info_id, title, image_url, creator, \
                    create_time::text AS create_time \
             FROM x_hotpic \
             WHERE deleted_at IS NULL \
               AND ((application = $1 AND info_id = $2) OR (application = '' AND info_id = '')) \
             ORDER BY create_time DESC NULLS LAST LIMIT 1",
            &[&application, &info_id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => {
            let mut result = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]);
            result.insert(
                "application".to_string(),
                Value::String(row.get("application")),
            );
            result.insert("infoId".to_string(), Value::String(row.get("info_id")));
            Ok(Json(ActionResult::success(Value::Object(result))))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn user_hotpic_id(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Path(id): Path<String>,
    method: axum::http::Method,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if method == axum::http::Method::DELETE {
        // 归属门：无会话 401（先于读库）；仅热图属主或 admin 可软删（归一化比较）。
        let Some(Extension(sess)) = session else {
            return Err(AppError::Unauthorized);
        };
        let client = shared::db::acquire(&pool).await?;
        let owner: Option<String> = client
            .query_opt(
                "SELECT creator FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
                &[&id],
            )
            .await
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?
            .map(|r| r.get::<_, Option<String>>("creator").unwrap_or_default());
        let owner = match owner {
            Some(o) if !o.is_empty() => o,
            _ => {
                return Ok(Json(ActionResult::success(Value::Object(
                    serde_json::Map::from_iter([
                        ("id".to_string(), Value::String(id)),
                        ("deleted".to_string(), Value::Bool(false)),
                    ]),
                ))))
            }
        };
        shared::middleware::require_owner(&pool, &sess, &owner).await?;
        let result = client
            .execute(
                "UPDATE x_hotpic SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
                &[&id],
            )
            .await
            .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
        return Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("deleted".to_string(), Value::Bool(result > 0)),
            ]),
        ))));
    }

    let client = shared::db::acquire(&pool).await?;
    let row = client
        .query_opt(
            "SELECT id, title, image_url, creator, create_time::text AS create_time FROM x_hotpic WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                (
                    "imageUrl".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("image_url")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    row.get::<_, Option<String>>("creator").into(),
                ),
                (
                    "createTime".to_string(),
                    row.get::<_, Option<String>>("create_time").into(),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("hotpic not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn user_hotpic_delete_by_ids(
    pool: Extension<Pool>,
    session: Option<Extension<shared::session::Session>>,
    Path((id, id2)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 安全审计 round4-b72：会话存在档（沿用本 crate user_hotpic_changeTitle 惯例），
    // 门置于 pool.get() 之前，杜绝匿名软删 x_hotpic。
    require_session(session)?;
    let client = shared::db::acquire(&pool).await?;
    let ids = vec![id.clone(), id2.clone()];
    let n = client
        .execute(
            "UPDATE x_hotpic SET deleted_at = NOW() WHERE id = ANY($1) AND deleted_at IS NULL",
            &[&ids],
        )
        .await
        .map_err(|e| AppError::internal_with_source(concat!(file!(), ":", line!()), e))?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("id2".to_string(), Value::String(id2)),
            (
                "deleted".to_string(),
                Value::Number(serde_json::Number::from(n)),
            ),
        ]),
    ))))
}
