use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

pub async fn portal_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, alias, description, portal_category FROM x_portal ORDER BY name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("portalCategory".to_string(), Value::String(row.get("portal_category"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn widget_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, alias, category, portal FROM x_widget ORDER BY name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("portal".to_string(), Value::String(row.get("portal"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn page_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, portal_id, name, content, status, create_time FROM x_portal_page WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("portalId".to_string(), Value::String(row.get("portal_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn page_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, portal_id, name, content, status, create_time FROM x_portal_page WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("portalId".to_string(), Value::String(row.get("portal_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get::<_, Option<String>>("content").unwrap_or_default())),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_create(
    pool: Extension<Pool>,
    axum::extract::Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let portal_id = payload.get("portalId").and_then(|v| v.as_str()).unwrap_or_default();
    let name = payload.get("name").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("name is required".to_string()))?;
    let content = payload.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("active");
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_portal_page (id, portal_id, name, content, status, create_time) VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &portal_id, &name, &content, &status],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name.to_string())),
        ("status".to_string(), Value::String(status.to_string())),
    ])))))
}

pub async fn page_update(
    pool: Extension<Pool>,
    axum::extract::Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = payload.get("id").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("id is required".to_string()))?;
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let content = payload.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("active");
    let result = client
        .execute(
            "UPDATE x_portal_page SET name = $1, content = $2, status = $3 WHERE id = $4 AND deleted_at IS NULL",
            &[&name, &content, &status, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("page not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn page_remove(
    pool: Extension<Pool>,
    axum::extract::Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = payload.get("id").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("id is required".to_string()))?;
    let result = client
        .execute(
            "UPDATE x_portal_page SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("page not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn script_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, alias, validated FROM x_script ORDER BY name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("alias".to_string(), Value::String(row.get("alias"))),
                ("validated".to_string(), Value::Bool(row.get("validated"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn portal_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/portal/portal/list", get(portal_list))
        .route("/jaxrs/portal/widget/list", get(widget_list))
        .route("/jaxrs/portal/page/list", get(page_list))
        .route("/jaxrs/portal/page/{id}", get(page_get))
        .route("/jaxrs/portal/page/create", post(page_create))
        .route("/jaxrs/portal/page/update", post(page_update))
        .route("/jaxrs/portal/page/remove", post(page_remove))
        .route("/jaxrs/portal/script/list", get(script_list))
        .layer(Extension(pool))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::portal_core_entity_router(pool)
}
