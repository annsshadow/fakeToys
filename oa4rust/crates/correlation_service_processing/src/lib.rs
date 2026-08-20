use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

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
        .route("/jaxrs/correlation/service/processing/correlation/delete/type/cms/document/{document}", post(correlation_delete_type_cms_document_document))
        .route("/jaxrs/correlation/service/processing/correlation/delete/type/processplatform/job/{job}", post(correlation_delete_type_processplatform_job_job))
        .route("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/{document}", get(correlation_list_type_cms_document_document))
        .route("/jaxrs/correlation/service/processing/correlation/list/type/cms/document/{document}/site/{site}", get(correlation_list_type_cms_document_document_site_site))
        .route("/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/{job}", get(correlation_list_type_processplatform_job_job))
        .route("/jaxrs/correlation/service/processing/correlation/list/type/processplatform/job/{job}/site/{site}", get(correlation_list_type_processplatform_job_job_site_site))
        .route("/jaxrs/correlation/service/processing/correlation/readable/type/cms", get(correlation_readable_type_cms))
        .route("/jaxrs/correlation/service/processing/correlation/readable/type/processplatform", get(correlation_readable_type_processplatform))
        .route("/jaxrs/correlation/service/processing/correlation/type/cms/document/{document}", get(correlation_type_cms_document_document))
        .route("/jaxrs/correlation/service/processing/correlation/type/processplatform/job/{job}", get(correlation_type_processplatform_job_job))
        .route("/jaxrs/correlation/service/processing/correlation/update/type/cms/document/{document}", post(correlation_update_type_cms_document_document))
        .route("/jaxrs/correlation/service/processing/correlation/update/type/processplatform/job/{job}", post(correlation_update_type_processplatform_job_job))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    correlation_service_processing_router().layer(axum::extract::Extension(pool))
}


pub async fn correlation_delete_type_cms_document_document(
    pool: Extension<Pool>,
    Path(document): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            r#"DELETE FROM x_correlation WHERE "type" = 'cms/document' AND target_id = $1"#,
            &[&document],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("correlation not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("document".to_string(), Value::String(document)),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn correlation_delete_type_processplatform_job_job(
    pool: Extension<Pool>,
    Path(job): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            r#"DELETE FROM x_correlation WHERE "type" = 'processplatform/job' AND target_id = $1"#,
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("correlation not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("job".to_string(), Value::String(job)),
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn correlation_list_type_cms_document_document(
    pool: Extension<Pool>,
    Path(document): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            r#"SELECT id, person_id, target_id, "type", creator, create_time FROM x_correlation WHERE "type" = 'cms/document' AND target_id = $1 ORDER BY create_time DESC"#,
            &[&document],
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

pub async fn correlation_list_type_cms_document_document_site_site(
    pool: Extension<Pool>,
    Path((document, _site)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            r#"SELECT id, person_id, target_id, "type", creator, create_time FROM x_correlation WHERE "type" = 'cms/document' AND target_id = $1 ORDER BY create_time DESC"#,
            &[&document],
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

pub async fn correlation_list_type_processplatform_job_job(
    pool: Extension<Pool>,
    Path(job): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            r#"SELECT id, person_id, target_id, "type", creator, create_time FROM x_correlation WHERE "type" = 'processplatform/job' AND target_id = $1 ORDER BY create_time DESC"#,
            &[&job],
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

pub async fn correlation_list_type_processplatform_job_job_site_site(
    pool: Extension<Pool>,
    Path((job, _site)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            r#"SELECT id, person_id, target_id, "type", creator, create_time FROM x_correlation WHERE "type" = 'processplatform/job' AND target_id = $1 ORDER BY create_time DESC"#,
            &[&job],
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

pub async fn correlation_readable_type_cms(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            r#"SELECT COUNT(*) as cnt FROM x_correlation WHERE "type" LIKE 'cms/%'"#,
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("readable".to_string(), Value::Bool(count > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn correlation_readable_type_processplatform(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            r#"SELECT COUNT(*) as cnt FROM x_correlation WHERE "type" LIKE 'processplatform/%'"#,
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("readable".to_string(), Value::Bool(count > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn correlation_type_cms_document_document(
    pool: Extension<Pool>,
    Path(document): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            r#"SELECT id, person_id, target_id, "type", creator, create_time FROM x_correlation WHERE "type" = 'cms/document' AND target_id = $1 LIMIT 1"#,
            &[&document],
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

pub async fn correlation_type_processplatform_job_job(
    pool: Extension<Pool>,
    Path(job): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            r#"SELECT id, person_id, target_id, "type", creator, create_time FROM x_correlation WHERE "type" = 'processplatform/job' AND target_id = $1 LIMIT 1"#,
            &[&job],
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

pub async fn correlation_update_type_cms_document_document(
    pool: Extension<Pool>,
    Path(document): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let person_id = req.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let r#type = req.get("type").and_then(|v| v.as_str()).unwrap_or("cms/document").to_string();

    let result = client
        .execute(
            r#"UPDATE x_correlation SET person_id = $1, "type" = $2 WHERE "type" = 'cms/document' AND target_id = $3"#,
            &[&person_id, &r#type, &document],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("correlation not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("document".to_string(), Value::String(document)),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("personId".to_string(), Value::String(person_id)),
            ("type".to_string(), Value::String(r#type)),
        ]),
    ))))
}

pub async fn correlation_update_type_processplatform_job_job(
    pool: Extension<Pool>,
    Path(job): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let person_id = req.get("personId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let r#type = req.get("type").and_then(|v| v.as_str()).unwrap_or("processplatform/job").to_string();

    let result = client
        .execute(
            r#"UPDATE x_correlation SET person_id = $1, "type" = $2 WHERE "type" = 'processplatform/job' AND target_id = $3"#,
            &[&person_id, &r#type, &job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("correlation not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("job".to_string(), Value::String(job)),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("personId".to_string(), Value::String(person_id)),
            ("type".to_string(), Value::String(r#type)),
        ]),
    ))))
}

