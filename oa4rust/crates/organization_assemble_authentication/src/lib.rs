use axum::{extract::{Extension, Path}, Json, Router, routing::get};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

#[axum::debug_handler]
pub async fn person_id_icon(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT icon_url FROM auth_person WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;
    let icon_url: String = row.get("icon_url");
    let data = Value::Object(serde_json::Map::from_iter([
        ("iconUrl".to_string(), Value::String(icon_url)),
        ("id".to_string(), Value::String(id)),
    ]));
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn identity_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, unit_id FROM x_org_identity WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("unit_id".to_string(), Value::String(row.get("unit_id"))),
    ]));
    Ok(Json(ActionResult::success(data)))
}

pub fn router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn organization_assemble_authentication_router() -> Router {
    Router::new()
        .route("/jaxrs/organization/assemble/authentication/person/{id}/icon", get(person_id_icon))
        .route("/jaxrs/organization/assemble/authentication/identity/{id}", get(identity_id))
}
