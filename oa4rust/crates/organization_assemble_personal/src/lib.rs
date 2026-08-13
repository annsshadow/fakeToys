use axum::{extract::{Extension, Path}, Json, Router, routing::get};
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

    let data = Value::Object(serde_json::Map::from_iter([
        ("mobile".to_string(), row.get::<_, Option<String>>("mobile").map(Value::String).unwrap_or(Value::Null)),
        ("email".to_string(), row.get::<_, Option<String>>("email").map(Value::String).unwrap_or(Value::Null)),
        ("icon".to_string(), row.get::<_, Option<String>>("icon").map(Value::String).unwrap_or(Value::Null)),
        ("theme".to_string(), row.get::<_, Option<String>>("theme").map(Value::String).unwrap_or(Value::Null)),
        ("lang".to_string(), row.get::<_, Option<String>>("lang").map(Value::String).unwrap_or(Value::Null)),
    ]));

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
}

pub fn router(pool: Pool) -> Router {
    routes::router(pool)
}
