use axum::{
    extract::{Extension, Path},
    http::HeaderMap,
    Json,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use shared::error::AppError;
use shared::response::ActionResult;

use auth::SessionManager;

pub mod router;

// --- 数据模型 ---

#[derive(Debug, Deserialize)]
pub struct EmpowerCreateRequest {
    pub to_person: String,
    pub role_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmpowerInfo {
    pub id: String,
    pub from_person: String,
    pub to_person: String,
    pub role_id: Option<String>,
    pub enabled: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EmpowerUpdateRequest {
    pub role_id: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct EmpowerListResult {
    pub count: i64,
    pub data: Vec<EmpowerInfo>,
}

// --- 当前用户身份解析 ---

pub(crate) async fn resolve_current_person_unique(
    session_manager: &SessionManager,
    headers: &HeaderMap,
) -> Result<String, AppError> {
    let token = shared::middleware::extract_token_from_headers(headers).ok_or(AppError::Unauthorized)?;
    session_manager
        .validate_session(&token)
        .await
        .map(|session| session.person_unique)
        .ok_or(AppError::Unauthorized)
}

/// 内部辅助：从 session + pool 执行 require_owner 检查
async fn check_owner(pool: &Pool, session_manager: &SessionManager, headers: &HeaderMap, owner_id: &str) -> Result<(), AppError> {
    let token = shared::middleware::extract_token_from_headers(headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager.validate_session(&token).await.ok_or(AppError::Unauthorized)?;
    shared::middleware::require_owner(pool, &session, owner_id).await
}

// --- 处理器 ---

/// POST /jaxrs/person/empower — 创建授权
pub async fn create(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Json(req): Json<EmpowerCreateRequest>,
) -> Result<Json<ActionResult<EmpowerInfo>>, AppError> {
    let from_person = resolve_current_person_unique(&session_manager, &headers).await?;

    if req.to_person.is_empty() {
        return Ok(Json(ActionResult::error("to_person is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let to_exists = client
        .query_opt(
            "SELECT id FROM auth_person WHERE id = $1 AND locked = false AND deleted_at IS NULL",
            &[&req.to_person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if to_exists.is_none() {
        return Ok(Json(ActionResult::error("to_person not found")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let role_id = req.role_id.clone();

    client
        .execute(
            "INSERT INTO x_empower (id, from_person, to_person, role_id, enabled, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, true, NOW(), NOW())",
            &[&id, &from_person, &req.to_person, &role_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = EmpowerInfo {
        id: id.clone(),
        from_person: from_person.clone(),
        to_person: req.to_person.clone(),
        role_id,
        enabled: true,
        created_at: None,
        updated_at: None,
    };
    Ok(Json(ActionResult::success(info)))
}

/// GET /jaxrs/person/empower/{id} — 查询授权（需 owner 验证）
pub async fn get(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<EmpowerInfo>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, from_person, to_person, role_id, enabled, created_at, updated_at \
             FROM x_empower WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    check_owner(&pool, &session_manager, &headers, &row.get::<_, String>("from_person")).await?;

    let info = EmpowerInfo {
        id: row.get("id"),
        from_person: row.get("from_person"),
        to_person: row.get("to_person"),
        role_id: row.get("role_id"),
        enabled: row.get("enabled"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };
    Ok(Json(ActionResult::success(info)))
}

/// PUT /jaxrs/person/empower/{id} — 更新授权（需 owner 验证）
pub async fn update(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(req): Json<EmpowerUpdateRequest>,
) -> Result<Json<ActionResult<EmpowerInfo>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let from_person: String = row.get("from_person");
    check_owner(&pool, &session_manager, &headers, &from_person).await?;

    let new_role_id = req.role_id.or_else(|| row.get("role_id"));
    let new_enabled = req.enabled.unwrap_or(row.get("enabled"));

    client
        .execute(
            "UPDATE x_empower SET role_id = $1, enabled = $2, updated_at = NOW() WHERE id = $3",
            &[&new_role_id, &new_enabled, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = EmpowerInfo {
        id,
        from_person,
        to_person: row.get("to_person"),
        role_id: new_role_id,
        enabled: new_enabled,
        created_at: None,
        updated_at: None,
    };
    Ok(Json(ActionResult::success(info)))
}

/// DELETE /jaxrs/person/empower/{id} — 删除授权（需 owner 验证）
pub async fn delete(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, from_person FROM x_empower WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let from_person: String = row.get("from_person");
    check_owner(&pool, &session_manager, &headers, &from_person).await?;

    client
        .execute(
            "UPDATE x_empower SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(serde_json::json!({ "success": true }))))
}

/// POST /jaxrs/person/empower/{id}/enable — 启用授权（需 owner 验证）
pub async fn enable(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<EmpowerInfo>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let from_person: String = row.get("from_person");
    check_owner(&pool, &session_manager, &headers, &from_person).await?;

    client
        .execute(
            "UPDATE x_empower SET enabled = true, updated_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = EmpowerInfo {
        id: row.get("id"),
        from_person: row.get("from_person"),
        to_person: row.get("to_person"),
        role_id: row.get("role_id"),
        enabled: true,
        created_at: None,
        updated_at: None,
    };
    Ok(Json(ActionResult::success(info)))
}

/// POST /jaxrs/person/empower/{id}/disable — 禁用授权（需 owner 验证）
pub async fn disable(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<EmpowerInfo>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let from_person: String = row.get("from_person");
    check_owner(&pool, &session_manager, &headers, &from_person).await?;

    client
        .execute(
            "UPDATE x_empower SET enabled = false, updated_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = EmpowerInfo {
        id: row.get("id"),
        from_person: row.get("from_person"),
        to_person: row.get("to_person"),
        role_id: row.get("role_id"),
        enabled: false,
        created_at: None,
        updated_at: None,
    };
    Ok(Json(ActionResult::success(info)))
}

/// POST /jaxrs/person/empower/manager — 管理员创建授权
pub async fn manager_create(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Json(req): Json<EmpowerCreateRequest>,
) -> Result<Json<ActionResult<EmpowerInfo>>, AppError> {
    let current_person = resolve_current_person_unique(&session_manager, &headers).await?;

    if !shared::middleware::is_admin(&pool, &current_person).await {
        return Ok(Json(ActionResult::error("forbidden: admin required")));
    }

    if req.to_person.is_empty() {
        return Ok(Json(ActionResult::error("to_person is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let to_exists = client
        .query_opt(
            "SELECT id FROM auth_person WHERE id = $1 AND locked = false AND deleted_at IS NULL",
            &[&req.to_person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if to_exists.is_none() {
        return Ok(Json(ActionResult::error("to_person not found")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let role_id = req.role_id.clone();

    client
        .execute(
            "INSERT INTO x_empower (id, from_person, to_person, role_id, enabled, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, true, NOW(), NOW())",
            &[&id, &current_person, &req.to_person, &role_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = EmpowerInfo {
        id: id.clone(),
        from_person: current_person.clone(),
        to_person: req.to_person.clone(),
        role_id,
        enabled: true,
        created_at: None,
        updated_at: None,
    };
    Ok(Json(ActionResult::success(info)))
}

/// PUT /jaxrs/person/empower/manager/{id} — 管理员更新授权
pub async fn manager_update(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(req): Json<EmpowerUpdateRequest>,
) -> Result<Json<ActionResult<EmpowerInfo>>, AppError> {
    let current_person = resolve_current_person_unique(&session_manager, &headers).await?;

    if !shared::middleware::is_admin(&pool, &current_person).await {
        return Ok(Json(ActionResult::error("forbidden: admin required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, from_person, to_person, role_id, enabled FROM x_empower \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::NotFound)?;

    let from_person: String = row.get("from_person");
    let new_role_id = req.role_id.or_else(|| row.get("role_id"));
    let new_enabled = req.enabled.unwrap_or(row.get("enabled"));

    client
        .execute(
            "UPDATE x_empower SET role_id = $1, enabled = $2, updated_at = NOW() WHERE id = $3",
            &[&new_role_id, &new_enabled, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = EmpowerInfo {
        id,
        from_person,
        to_person: row.get("to_person"),
        role_id: new_role_id,
        enabled: new_enabled,
        created_at: None,
        updated_at: None,
    };
    Ok(Json(ActionResult::success(info)))
}

/// DELETE /jaxrs/person/empower/manager/{id} — 管理员删除授权
pub async fn manager_delete(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    let current_person = resolve_current_person_unique(&session_manager, &headers).await?;

    if !shared::middleware::is_admin(&pool, &current_person).await {
        return Ok(Json(ActionResult::error("forbidden: admin required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let deleted = client
        .execute(
            "UPDATE x_empower SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if deleted == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }

    Ok(Json(ActionResult::success(serde_json::json!({ "success": true }))))
}

/// POST /jaxrs/person/empower/manager/list/paging/{page}/size/{size} — 管理员分页查询
pub async fn manager_list_paging(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path((page, size)): Path<(i64, i64)>,
) -> Result<Json<ActionResult<EmpowerListResult>>, AppError> {
    let current_person = resolve_current_person_unique(&session_manager, &headers).await?;

    if !shared::middleware::is_admin(&pool, &current_person).await {
        return Ok(Json(ActionResult::error("forbidden: admin required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let offset = (page - 1).max(0) * size;

    let total_row = client
        .query_opt(
            "SELECT COUNT(*) AS cnt FROM x_empower WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
             "SELECT id, from_person, to_person, role_id, enabled, created_at, updated_at \
              FROM x_empower WHERE deleted_at IS NULL ORDER BY created_at DESC LIMIT $2::int OFFSET $1::int",
             &[&offset, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<EmpowerInfo> = rows
        .iter()
        .map(|row| EmpowerInfo {
            id: row.get("id"),
            from_person: row.get("from_person"),
            to_person: row.get("to_person"),
            role_id: row.get("role_id"),
            enabled: row.get("enabled"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(ActionResult::success(EmpowerListResult { count: total, data })))
}

/// GET /jaxrs/person/empower/list/currentperson — 我的授权列表
pub async fn list_current_person(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<EmpowerListResult>>, AppError> {
    let current_person = resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let total_row = client
        .query_opt(
            "SELECT COUNT(*) AS cnt FROM x_empower WHERE from_person = $1 AND deleted_at IS NULL",
            &[&current_person],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, from_person, to_person, role_id, enabled, created_at, updated_at \
             FROM x_empower WHERE from_person = $1 AND deleted_at IS NULL ORDER BY created_at DESC",
            &[&current_person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<EmpowerInfo> = rows
        .iter()
        .map(|row| EmpowerInfo {
            id: row.get("id"),
            from_person: row.get("from_person"),
            to_person: row.get("to_person"),
            role_id: row.get("role_id"),
            enabled: row.get("enabled"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(ActionResult::success(EmpowerListResult { count: total, data })))
}

/// GET /jaxrs/person/empower/list/currentperson/enable — 我的生效授权列表
pub async fn list_current_person_enable(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<EmpowerListResult>>, AppError> {
    let current_person = resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let total_row = client
        .query_opt(
            "SELECT COUNT(*) AS cnt FROM x_empower WHERE from_person = $1 AND enabled = true AND deleted_at IS NULL",
            &[&current_person],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, from_person, to_person, role_id, enabled, created_at, updated_at \
             FROM x_empower WHERE from_person = $1 AND enabled = true AND deleted_at IS NULL ORDER BY created_at DESC",
            &[&current_person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<EmpowerInfo> = rows
        .iter()
        .map(|row| EmpowerInfo {
            id: row.get("id"),
            from_person: row.get("from_person"),
            to_person: row.get("to_person"),
            role_id: row.get("role_id"),
            enabled: row.get("enabled"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(ActionResult::success(EmpowerListResult { count: total, data })))
}

/// GET /jaxrs/person/empower/list/to — 我拥有的被授权列表（我授权给他人的）
pub async fn list_to(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<EmpowerListResult>>, AppError> {
    let current_person = resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let total_row = client
        .query_opt(
            "SELECT COUNT(*) AS cnt FROM x_empower WHERE to_person = $1 AND deleted_at IS NULL",
            &[&current_person],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, from_person, to_person, role_id, enabled, created_at, updated_at \
             FROM x_empower WHERE to_person = $1 AND deleted_at IS NULL ORDER BY created_at DESC",
            &[&current_person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<EmpowerInfo> = rows
        .iter()
        .map(|row| EmpowerInfo {
            id: row.get("id"),
            from_person: row.get("from_person"),
            to_person: row.get("to_person"),
            role_id: row.get("role_id"),
            enabled: row.get("enabled"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(ActionResult::success(EmpowerListResult { count: total, data })))
}

/// GET /jaxrs/person/empower/list/to/enable — 我生效的被授权列表
pub async fn list_to_enable(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<EmpowerListResult>>, AppError> {
    let current_person = resolve_current_person_unique(&session_manager, &headers).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let total_row = client
        .query_opt(
            "SELECT COUNT(*) AS cnt FROM x_empower WHERE to_person = $1 AND enabled = true AND deleted_at IS NULL",
            &[&current_person],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .ok_or(AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let rows = client
        .query(
            "SELECT id, from_person, to_person, role_id, enabled, created_at, updated_at \
             FROM x_empower WHERE to_person = $1 AND enabled = true AND deleted_at IS NULL ORDER BY created_at DESC",
            &[&current_person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<EmpowerInfo> = rows
        .iter()
        .map(|row| EmpowerInfo {
            id: row.get("id"),
            from_person: row.get("from_person"),
            to_person: row.get("to_person"),
            role_id: row.get("role_id"),
            enabled: row.get("enabled"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(ActionResult::success(EmpowerListResult { count: total, data })))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

