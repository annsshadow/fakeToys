use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;
pub mod u2;

#[derive(Debug, Deserialize)]
pub struct LinkRequest {
    pub source_type: Option<String>,
    pub source_id: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CorrelationRequest {
    pub person_id: Option<String>,
    pub target_id: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CorrelationUpdateRequest {
    pub target_id: Option<String>,
    pub r#type: Option<String>,
}

pub async fn link_service(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<LinkRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let source_type = req.source_type.unwrap_or_default();
    let source_id = req.source_id.unwrap_or_default();
    let target_type = req.target_type.unwrap_or_default();
    let target_id = req.target_id.unwrap_or_default();

    let row = client
        .query_opt(
            r#"SELECT id FROM x_correlation WHERE "type" = $1 AND person_id = $2 AND target_id = $3 LIMIT 1"#,
            &[&source_type, &source_id, &target_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let linked = row.is_some();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("linked".to_string(), Value::Bool(linked)),
            ("source_type".to_string(), Value::String(source_type)),
            ("source_id".to_string(), Value::String(source_id)),
            ("target_type".to_string(), Value::String(target_type)),
            ("target_id".to_string(), Value::String(target_id)),
        ]),
    ))))
}

pub async fn get_link(
    pool: Extension<Pool>,
    axum::extract::Path((source_type, source_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            r#"SELECT id, target_id, "type" FROM x_correlation WHERE "type" = $1 AND person_id = $2 LIMIT 1"#,
            &[&source_type, &source_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("sourceType".to_string(), Value::String(source_type)),
                    ("sourceId".to_string(), Value::String(source_id)),
                    ("targetType".to_string(), Value::String(row.get("type"))),
                    ("targetId".to_string(), Value::String(row.get("target_id"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("link not found"))),
    }
}

pub async fn list_correlations(
    pool: Extension<Pool>,
    axum::extract::Path(person_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            r#"SELECT id, person_id, target_id, "type", creator, create_time FROM x_correlation
             WHERE person_id = $1 ORDER BY create_time DESC"#,
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("targetId".to_string(), Value::String(row.get("target_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn get_correlation(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            r#"SELECT id, person_id, target_id, "type", creator, create_time FROM x_correlation
             WHERE id = $1"#,
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("targetId".to_string(), Value::String(row.get("target_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("correlation not found"))),
    }
}

pub async fn create_correlation(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CorrelationRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let person_id = req.person_id.unwrap_or_default();
    let target_id = req.target_id.unwrap_or_default();
    let r#type = req.r#type.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            r#"INSERT INTO x_correlation (id, person_id, target_id, "type", creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, NOW())"#,
            &[&id, &person_id, &target_id, &r#type, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("personId".to_string(), Value::String(person_id)),
        ("targetId".to_string(), Value::String(target_id)),
        ("type".to_string(), Value::String(r#type)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn save_correlation(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<CorrelationUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let target_id = req.target_id.unwrap_or_default();
    let r#type = req.r#type.unwrap_or_default();

    let result = client
        .execute(
            r#"UPDATE x_correlation SET target_id = $1, "type" = $2 WHERE id = $3"#,
            &[&target_id, &r#type, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("correlation not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("targetId".to_string(), Value::String(target_id)),
            ("type".to_string(), Value::String(r#type)),
        ]),
    ))))
}

pub async fn delete_correlation(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM x_correlation WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("correlation not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn unlink_service(
    pool: Extension<Pool>,
    axum::extract::Path((source_type, source_id, target_type, target_id)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            r#"DELETE FROM x_correlation WHERE "type" = $1 AND person_id = $2 AND target_id = $3"#,
            &[&source_type, &source_id, &target_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("correlation not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("unlinked".to_string(), Value::Bool(result > 0)),
            ("sourceType".to_string(), Value::String(source_type)),
            ("sourceId".to_string(), Value::String(source_id)),
            ("targetType".to_string(), Value::String(target_type)),
            ("targetId".to_string(), Value::String(target_id)),
        ]),
    ))))
}

pub fn correlation_service_processing_router() -> Router {
    Router::new()
        .route("/jaxrs/correlation/service/processing/list/{personId}", get(list_correlations))
        .route("/jaxrs/correlation/service/processing/{id}", get(get_correlation))
        .route("/jaxrs/correlation/service/processing/create", post(create_correlation))
        .route("/jaxrs/correlation/service/processing/save/{id}", post(save_correlation))
        .route("/jaxrs/correlation/service/processing/delete/{id}", post(delete_correlation))
        .route("/jaxrs/correlation/service/processing/link/{sourceType}/{sourceId}", get(get_link))
        .route("/jaxrs/correlation/service/processing/link", post(link_service))
        .route("/jaxrs/correlation/service/processing/unlink/{sourceType}/{sourceId}/{targetType}/{targetId}", post(unlink_service))
        // ── Java CorrelationAction 契约（u2）────────────────────────────────
        // GET 为仓库既有扩展（单条查询），POST 对齐 Java 创建语义
        .route(
            "/jaxrs/correlation/service/processing/correlation/type/processplatform/job/{job}",
            get(correlation_type_processplatform_job_job).post(u2::create_pp),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/type/cms/document/{document}",
            get(correlation_type_cms_document_document).post(u2::create_cms),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/update/type/processplatform/job/{job}",
            post(u2::update_pp),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/update/type/cms/document/{document}",
            post(u2::update_cms),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/{job}",
            post(u2::delete_pp),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/delete/type/cms/document/{document}",
            post(u2::delete_cms),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/readable/type/processplatform",
            post(u2::readable_pp),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/readable/type/cms",
            post(u2::readable_cms),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/{job}",
            get(u2::list_pp),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/{job}/site/{site}",
            get(u2::list_pp_site),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/list/type/cms/document/{document}",
            get(u2::list_cms),
        )
        .route(
            "/jaxrs/correlation/service/processing/correlation/list/type/cms/document/{document}/site/{site}",
            get(u2::list_cms_site),
        )
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    correlation_service_processing_router().layer(axum::extract::Extension(pool))
}

/// GET correlation/type/cms/document/{document}（仓库既有扩展：按目标取单条关联）
#[axum::debug_handler]
pub async fn correlation_type_cms_document_document(
    pool: Extension<Pool>,
    Path(document): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            r#"SELECT id FROM x_correlation WHERE "type" = 'cms/document' AND target_id = $1 LIMIT 1"#,
            &[&document],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("document".to_string(), Value::String(document)),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("correlation not found"))),
    }
}

/// GET correlation/type/processplatform/job/{job}（仓库既有扩展：按目标取单条关联）
#[axum::debug_handler]
pub async fn correlation_type_processplatform_job_job(
    pool: Extension<Pool>,
    Path(job): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            r#"SELECT id FROM x_correlation WHERE "type" = 'processplatform/job' AND target_id = $1 LIMIT 1"#,
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("job".to_string(), Value::String(job)),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("correlation not found"))),
    }
}


