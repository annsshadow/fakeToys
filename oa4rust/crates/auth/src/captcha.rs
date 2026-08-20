use axum::{
    extract::{Json, Path},
    routing::{get, post},
    Router,
};
use base64::Engine;
use captcha_store::{captcha_store, generate as captcha_generate, verify as captcha_verify, CaptchaError, VerifyResult};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;

// ──────────────────────────────────────────────────────────────────────────────
// captcha — 验证码
//
// 使用 captcha_store 库生成 PNG 验证码并存储校验状态：
// 5 分钟有效、一次性（校验通过即删除）、尝试次数上限（超过即删除）。
// ──────────────────────────────────────────────────────────────────────────────

const DEFAULT_WIDTH: u32 = 120;
const DEFAULT_HEIGHT: u32 = 40;
const MAX_WIDTH: u32 = 500;
const MAX_HEIGHT: u32 = 200;

fn image_data_uri(png: &[u8]) -> String {
    format!(
        "data:image/png;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(png)
    )
}

fn validate_dimensions(width: u32, height: u32) -> Result<(), AppError> {
    if width == 0 || height == 0 || width > MAX_WIDTH || height > MAX_HEIGHT {
        return Err(AppError::BadRequest(format!(
            "invalid captcha dimensions: {width}x{height}"
        )));
    }
    Ok(())
}

/// GET /jaxrs/authentication/captcha —— 默认尺寸验证码
pub async fn captcha_default() -> Result<Json<ActionResult<Value>>, AppError> {
    generate_captcha(DEFAULT_WIDTH, DEFAULT_HEIGHT)
}

/// GET /jaxrs/authentication/captcha/width/{width}/height/{height}
pub async fn captcha_with_size(
    Path((width, height)): Path<(u32, u32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    generate_captcha(width, height)
}

fn generate_captcha(width: u32, height: u32) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_dimensions(width, height)?;
    let (id, png) = captcha_generate(width, height).map_err(|e| match e {
        CaptchaError::NotFound => AppError::Internal,
        _ => AppError::Internal,
    })?;
    Ok(Json(ActionResult::success(json!({
        "captchaId": id,
        "image": image_data_uri(&png),
    }))))
}

#[derive(Debug, Deserialize)]
pub struct VerifyCaptchaRequest {
    pub captchaId: String,
    pub answer: String,
}

/// POST /jaxrs/secret/captcha/verify —— 验证码校验
pub async fn verify(
    Json(req): Json<VerifyCaptchaRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match captcha_verify(&req.captchaId, &req.answer) {
        VerifyResult::Ok => Ok(Json(ActionResult::success(json!({ "passed": true })))),
        VerifyResult::TooManyAttempts => Ok(Json(ActionResult::error(
            "too many attempts, captcha invalidated",
        ))),
        VerifyResult::Expired => Ok(Json(ActionResult::error("captcha expired"))),
        _ => Ok(Json(ActionResult::error("invalid captcha"))),
    }
}

pub fn captcha_router() -> Router {
    Router::new()
        .route("/jaxrs/authentication/captcha", get(captcha_default))
        .route(
            "/jaxrs/authentication/captcha/width/{width}/height/{height}",
            get(captcha_with_size),
        )
        .route("/jaxrs/secret/captcha/verify", post(verify))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_captcha_ok_and_one_time() {
        let id = captcha_store().insert("Ab3D".to_string());
        assert_eq!(captcha_store().verify(&id, "ab3d"), VerifyResult::Ok);
        assert_eq!(captcha_store().verify(&id, "ab3d"), VerifyResult::NotFound);
    }

    #[test]
    fn test_captcha_wrong_answer_attempts() {
        let id = captcha_store().insert("1234".to_string());
        assert_eq!(captcha_store().verify(&id, "0000"), VerifyResult::WrongAnswer);
        assert_eq!(captcha_store().verify(&id, "0000"), VerifyResult::WrongAnswer);
        assert_eq!(captcha_store().verify(&id, "0000"), VerifyResult::WrongAnswer);
        assert_eq!(captcha_store().verify(&id, "0000"), VerifyResult::WrongAnswer);
        assert_eq!(captcha_store().verify(&id, "0000"), VerifyResult::TooManyAttempts);
        assert_eq!(captcha_store().verify(&id, "1234"), VerifyResult::NotFound);
    }

    #[test]
    fn test_captcha_unknown_id() {
        assert_eq!(captcha_store().verify("nope", "1234"), VerifyResult::NotFound);
    }

    #[test]
    fn test_captcha_expired() {
        captcha_store().force_insert("expired-id", "1234", chrono::Utc::now() - chrono::Duration::minutes(1));
        assert_eq!(captcha_store().verify("expired-id", "1234"), VerifyResult::Expired);
    }

    #[test]
    fn test_dimension_validation() {
        assert!(validate_dimensions(120, 40).is_ok());
        assert!(validate_dimensions(0, 40).is_err());
        assert!(validate_dimensions(120, 0).is_err());
        assert!(validate_dimensions(501, 40).is_err());
        assert!(validate_dimensions(120, 201).is_err());
    }
}
