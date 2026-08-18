use axum::{
    extract::Extension,
    extract::Query,
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;

use shared::{error::AppError, response::ActionResult};

pub mod routes;
pub mod batch_query;

pub use batch_query::{
    express_person_list, express_unit_list, express_identity_list, express_group_list,
    express_role_list, express_person_with_unit, express_person_with_identity,
};

#[derive(Debug, Deserialize)]
pub struct ExpressQuery {
    pub code: Option<String>,
    pub company: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExpressSubscribeRequest {
    pub code: Option<String>,
    pub company: Option<String>,
    pub callback: Option<String>,
}

pub async fn get_express_info(
    pool: Extension<Pool>,
    Query(params): Query<ExpressQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = params.code.unwrap_or_default();
    let company = params.company.unwrap_or_default();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT \"xtrackingNumber\", xstatus, xcompany FROM x_express_info WHERE \"xtrackingNumber\" = $1 AND xcompany = $2",
            &[&code, &company],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = if rows.is_empty() {
        Value::Object(serde_json::Map::from_iter([
            ("code".to_string(), Value::String(code)),
            ("company".to_string(), Value::String(company)),
            ("status".to_string(), Value::String("not_found".to_string())),
            ("traces".to_string(), Value::Array(vec![])),
        ]))
    } else {
        let row = &rows[0];
        Value::Object(serde_json::Map::from_iter([
            ("code".to_string(), Value::String(code)),
            ("company".to_string(), Value::String(company)),
            ("status".to_string(), Value::String(row.get("xstatus"))),
            ("traces".to_string(), Value::Array(vec![])),
        ]))
    };

    Ok(Json(ActionResult::success(data)))
}

pub async fn list_express_companies(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xcode, xname FROM x_express_company ORDER BY xname LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let companies: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("code".to_string(), Value::String(row.get("xcode"))),
                ("name".to_string(), Value::String(row.get("xname"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(companies.len() as i64))),
            ("data".to_string(), Value::Array(companies)),
        ]),
    ))))
}

pub async fn subscribe_express(
    pool: Extension<Pool>,
    Json(payload): Json<ExpressSubscribeRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = payload.code.unwrap_or_default();
    let company = payload.company.unwrap_or_default();
    let callback = payload.callback.unwrap_or_default();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO x_express_subscribe (xid, \"xtrackingNumber\", xcompany, xcallback) VALUES ($1, $2, $3, $4)",
            &[&id, &code, &company, &callback],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("code".to_string(), Value::String(code)),
            ("company".to_string(), Value::String(company)),
            ("callback".to_string(), Value::String(callback)),
            ("subscribed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

