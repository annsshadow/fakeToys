use axum::{extract::{Extension, Path}, Json, Router, routing::{get, post}};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


#[axum::debug_handler]
pub async fn user_setting(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT mobile, email, icon, theme, lang FROM auth_person WHERE id = $1 LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut map = serde_json::Map::new();
    if let Some(v) = row.get::<_, Option<String>>("mobile") {
        map.insert("mobile".to_string(), Value::String(v));
    }
    if let Some(v) = row.get::<_, Option<String>>("email") {
        map.insert("email".to_string(), Value::String(v));
    }
    if let Some(v) = row.get::<_, Option<String>>("icon") {
        map.insert("icon".to_string(), Value::String(v));
    }
    if let Some(v) = row.get::<_, Option<String>>("theme") {
        map.insert("theme".to_string(), Value::String(v));
    }
    if let Some(v) = row.get::<_, Option<String>>("lang") {
        map.insert("lang".to_string(), Value::String(v));
    }
    let data = Value::Object(map);

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn user_role_list(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT r.id, r.name FROM x_org_group_member m JOIN x_org_role r ON m.role_id = r.id WHERE m.person_id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let roles: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("roles".to_string(), Value::Array(roles)),
        ]),
    ))))
}

pub fn organization_assemble_personal_router() -> Router {
    Router::new()
        .route("/jaxrs/organization/assemble/personal/{id}/setting", get(user_setting))
        .route("/jaxrs/organization/assemble/personal/{id}/role/list", get(user_role_list))
        .route("/jaxrs/organization/assemble/personal/custom/{id}/mockputtopost", post(custom_mockputtopost))
}

pub async fn custom_mockputtopost(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let value = req.get("fieldValue").and_then(|v| v.as_str()).unwrap_or("");
    let n = client
        .execute(
            "UPDATE x_org_custom SET field_value = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&id, &value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "id": id, "updated": n
    }))))
}

pub fn router(pool: Pool) -> Router {
    routes::router(pool)
}
