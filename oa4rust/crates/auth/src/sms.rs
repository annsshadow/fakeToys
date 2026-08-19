use axum::{routing::post, Json, Router};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;
use sms::{is_valid_phone, mock_sms_gateway, SmsGateway};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SmsSendRequest {
    pub phone: String,
}

/// 发送短信验证码（供 lib.rs 调用）
pub async fn sms_send(phone: &str) -> Result<ActionResult<Value>, AppError> {
    if phone.is_empty() {
        return Ok(ActionResult::error("phone is required"));
    }
    if !is_valid_phone(phone) {
        return Ok(ActionResult::error("invalid phone number"));
    }
    let plain_code = format!("{:06}", (Uuid::new_v4().as_u128() % 1_000_000) as u32);
    let gateway = mock_sms_gateway();
    let result = gateway
        .send_verification_code(phone, &plain_code, None)
        .await;
    match result {
        Ok(_) => Ok(ActionResult::success(json!({
            "message": "sms sent",
        }))),
        Err(e) => Ok(ActionResult::error(format!("sms send failed: {e}"))),
    }
}

/// POST /jaxrs/authentication/sms/send —— 发送短信验证码
pub async fn sms_send_handler(
    Json(payload): Json<SmsSendRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    sms_send(&payload.phone)
        .await
        .map(Json)
}

pub fn sms_router() -> Router {
    Router::new()
        .route("/jaxrs/authentication/sms/send", post(sms_send_handler))
}
