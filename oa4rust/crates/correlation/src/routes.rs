use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub fn correlation_router(pool: Pool) -> Router {
    Router::new()
        .route(
            "/jaxrs/correlation/type/cms/list",
            get(list_cms_correlations),
        )
        .route(
            "/jaxrs/correlation/type/processplatform/list",
            get(list_process_platform_correlations),
        )
        .route(
            "/jaxrs/correlation/type/cms/readable",
            get(check_cms_readable),
        )
        .layer(Extension(pool))
}

#[utoipa::path(
    get,
    path = "/jaxrs/correlation/type/cms/list",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "correlation"
)]
pub async fn list_cms_correlations(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, from_bundle, target_bundle, person, site, order_number FROM CORR_C_CORRELATION WHERE from_type = 'cms' ORDER BY order_number",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut obj = serde_json::Map::new();
            obj.insert("id".to_string(), Value::String(row.get("id")));
            obj.insert("fromBundle".to_string(), Value::String(row.get("from_bundle")));
            obj.insert("targetBundle".to_string(), Value::String(row.get("target_bundle")));
            obj.insert("person".to_string(), Value::String(row.get("person")));
            if let Some(site) = row.get::<_, Option<String>>("site") {
                obj.insert("site".to_string(), Value::String(site));
            }
            if let Some(n) = row.get::<_, Option<i32>>("order_number") {
                obj.insert("orderNumber".to_string(), Value::Number(serde_json::Number::from(n)));
            }
            Value::Object(obj)
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[utoipa::path(
    get,
    path = "/jaxrs/correlation/type/processplatform/list",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "correlation"
)]
pub async fn list_process_platform_correlations(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, from_bundle, target_bundle, person, site, order_number FROM CORR_C_CORRELATION WHERE from_type = 'processPlatform' ORDER BY order_number",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut obj = serde_json::Map::new();
            obj.insert("id".to_string(), Value::String(row.get("id")));
            obj.insert("fromBundle".to_string(), Value::String(row.get("from_bundle")));
            obj.insert("targetBundle".to_string(), Value::String(row.get("target_bundle")));
            obj.insert("person".to_string(), Value::String(row.get("person")));
            if let Some(site) = row.get::<_, Option<String>>("site") {
                obj.insert("site".to_string(), Value::String(site));
            }
            if let Some(n) = row.get::<_, Option<i32>>("order_number") {
                obj.insert("orderNumber".to_string(), Value::Number(serde_json::Number::from(n)));
            }
            Value::Object(obj)
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

#[utoipa::path(
    get,
    path = "/jaxrs/correlation/type/cms/readable",
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "correlation"
)]
pub async fn check_cms_readable(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM CORR_C_CORRELATION WHERE from_type = 'cms'",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");
    let readable = count > 0;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("readable".to_string(), Value::Bool(readable))]),
    ))))
}
