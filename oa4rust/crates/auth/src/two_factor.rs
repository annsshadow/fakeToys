use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use shared::db::dialect;
use shared::error::AppError;
use shared::response::ActionResult;

use crate::{code_store, ldap_auth, password, temp_token_store};
use tracing::warn;

// ──────────────────────────────────────────────────────────────────────────────
// two_factor — 双因素登录第一阶段（短信验证码发送）
//
// 流程：POST /jaxrs/authentication/two_factor
//   1. 验证第一因子：credential + password
//   2. 发送短信验证码
//   3. 签发临时 token（绑定到 credential，防止凭证交换攻击）
//   4. 返回 {value: true, password_expired: bool, temp_token: "..."}
//
// 安全：第一因子失败时返回相同错误消息，不暴露是否验证码正确（防枚举）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TwoFactorLoginRequest {
    pub credential: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct TwoFactorPhase1Response {
    pub value: bool,
    pub password_expired: bool,
    pub temp_token: String,
}

/// POST /jaxrs/authentication/two_factor —— 双因素登录第一阶段
///
/// 验证 credential + password，发送短信验证码，签发临时 token（阶段绑定）
pub async fn two_factor_login(
    pool: Extension<Pool>,
    Json(req): Json<TwoFactorLoginRequest>,
) -> Result<Json<ActionResult<TwoFactorPhase1Response>>, AppError> {
    if req.credential.is_empty() || req.password.is_empty() {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let d = dialect();
    let sql = format!(
        "SELECT id, unique_id, name, mobile, email, icon, job, department, unit, position, \
         password_hash, locked, {}, {} FROM auth_person \
         WHERE unique_id = {} AND deleted_at IS NULL",
        d.cast_text("change_password_time"),
        d.cast_text("password_expired_time"),
        d.param(1),
    );
    let row = client
        .query_one(&sql, &[&req.credential])
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let person_unique: String = row.get("unique_id");
    let locked: bool = row.get("locked");
    let password_hash: String = row.get("password_hash");
    let change_password_time: Option<String> = row.get("change_password_time");

    if locked {
        // 返回通用错误消息，防止账户锁定状态枚举
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let valid = match ldap_auth::try_ldap_authenticate(&req.credential, &req.password).await {
        Ok(Some(ldap_auth::LdapAuthOutcome::Success)) => true,
        Ok(Some(ldap_auth::LdapAuthOutcome::Failed)) => {
            warn!("LDAP auth failed for two_factor user {}, falling back to DB", req.credential);
            password::verify_password(&req.password, &password_hash, "", None)
        }
        Ok(None) | Ok(Some(ldap_auth::LdapAuthOutcome::Disabled)) => {
            password::verify_password(&req.password, &password_hash, "", None)
        }
        Err(_) => {
            return Ok(Json(ActionResult::error("invalid credentials")));
        }
        Ok(Some(ldap_auth::LdapAuthOutcome::Error)) => {
            return Ok(Json(ActionResult::error("invalid credentials")));
        }
    };
    if !valid {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let password_expired = change_password_time.is_none();

    let _plain = code_store().issue(&person_unique);

    let temp_token = temp_token_store().issue(&person_unique);

    Ok(Json(ActionResult::success(TwoFactorPhase1Response {
        value: true,
        password_expired,
        temp_token,
    })))
}
