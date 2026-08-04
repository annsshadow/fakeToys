use axum::{
    extract::Extension,
    routing::{get, post, put},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use shared::error::AppError;
use shared::response::ActionResult;

pub mod password;
pub mod reset;

// --- Models ---

#[derive(Debug, Deserialize)]
pub struct EditPersonRequest {
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PersonInfo {
    pub id: String,
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub icon: Option<String>,
}

// --- Handlers ---

pub async fn get_person(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person WHERE locked = false LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let info = PersonInfo {
        id: row.get("id"),
        unique: row.get("unique_id"),
        name: row.get("name"),
        mobile: row.get("mobile"),
        email: row.get("email"),
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(info)))
}

pub async fn edit_person(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<EditPersonRequest>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email FROM auth_person WHERE locked = false LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");

    let name = req.name.unwrap_or_else(|| row.get("name"));
    let db_mobile: Option<String> = row.get("mobile");
    let db_email: Option<String> = row.get("email");
    let mobile = req.mobile.or(db_mobile);
    let email = req.email.or(db_email);

    client
        .execute(
            "UPDATE auth_person SET name = $1, mobile = $2, email = $3, updated_at = NOW() WHERE id = $4",
            &[&name, &mobile, &email, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let updated = PersonInfo {
        id: person_id,
        unique: row.get("unique_id"),
        name,
        mobile,
        email,
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(updated)))
}

// --- Router ---

pub fn router(pool: Pool) -> Router {
    let reset_store = reset::ResetCodeStore::new();

    Router::new()
        .route("/jaxrs/person", get(get_person))
        .route("/jaxrs/person", put(edit_person))
        .route("/jaxrs/person/mockputtopost", post(edit_person))
        .route("/jaxrs/password", put(password::change))
        .route("/jaxrs/password/mockputtopost", post(password::change))
        .route("/jaxrs/reset/code", post(reset::send_code))
        .route("/jaxrs/reset/check", post(reset::check_code))
        .route("/jaxrs/reset/set", post(reset::reset_password))
        .layer(Extension(pool))
        .layer(Extension(reset_store))
}

#[cfg(test)]
mod tests;
