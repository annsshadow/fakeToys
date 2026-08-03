use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

use auth::password::verify_password;

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

pub async fn change(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<ChangePasswordRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, password_hash FROM auth_person WHERE locked = false LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");
    let stored_hash: String = row.get("password_hash");

    let valid = verify_password(&req.old_password, &stored_hash, "", None);
    if !valid {
        return Ok(Json(ActionResult::error("old password mismatch")));
    }

    let new_hash = format!("{:x}", md5::compute(req.new_password.as_bytes()));
    client
        .execute(
            "UPDATE auth_person SET password_hash = $1, updated_at = NOW() WHERE id = $2",
            &[&new_hash, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("success".to_string(), Value::Bool(true)),
    ])))))
}
