use axum::{
    extract::Extension,
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreatePortalRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePageRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub content: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct SavePageRequest {
    pub content: Option<Value>,
}

pub async fn create_design(
    axum::extract::Json(req): Json<CreatePortalRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("description".to_string(), Value::String(req.description.unwrap_or_default())),
        ]),
    ))))
}

pub async fn get_design(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Portal Design".to_string())),
            ("components".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn list_designs() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("design-1".to_string())),
            ("name".to_string(), Value::String("Design 1".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn save_design(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("updated_at".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ]),
    ))))
}

pub async fn list_pages_by_category(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time, update_time FROM x_portal_page WHERE category = $1 ORDER BY update_time DESC",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let content: Option<String> = row.get("content");
            let content_value = content
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(Value::Null);
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), content_value),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn get_page(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time, update_time FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let content_value = content
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(Value::Null);
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), content_value),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn create_page(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): Json<CreatePageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let content = req.content.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    let content_str = serde_json::to_string(&content).map_err(|_| AppError::Internal)?;
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_portal_page (id, name, category, content, creator, create_time, update_time) VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &content_str, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("category".to_string(), Value::String(category)),
        ("content".to_string(), content),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn save_page(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<SavePageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let content = req.content.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    let content_str = serde_json::to_string(&content).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_portal_page SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("page not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("content".to_string(), content),
        ]),
    ))))
}

pub async fn delete_page(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "DELETE FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("page not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn portal_assemble_designer_router() -> Router {
    Router::new()
        .route("/jaxrs/portal/assemble/designer/page/list/{category}", get(list_pages_by_category))
        .route("/jaxrs/portal/assemble/designer/page/{id}", get(get_page))
        .route("/jaxrs/portal/assemble/designer/page/create", post(create_page))
        .route("/jaxrs/portal/assemble/designer/page/save/{id}", post(save_page))
        .route("/jaxrs/portal/assemble/designer/page/delete/{id}", post(delete_page))
        .route("/jaxrs/portal/assemble/designer/create", post(create_design))
        .route("/jaxrs/portal/assemble/designer/get/{id}", get(get_design))
        .route("/jaxrs/portal/assemble/designer/list", get(list_designs))
        .route("/jaxrs/portal/assemble/designer/save/{id}", post(save_design))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    portal_assemble_designer_router()
        .layer(Extension(pool))
        .route("/portal_assemble_designer/health", axum::routing::get(|| async { "ok" }))
}


/// Stub handler for /jaxrs/portal/assemble/designer/designer/search
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_designer_search() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/dict/list/paging/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_dict_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/dict/list/portal/{portalId}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_dict_list_portal_portalId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/dict/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_dict_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/file/list/application/{applicationFlag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_file_list_application_applicationFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/file/list/{id}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_file_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/file/list/{id}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_file_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/file/{flag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_file_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/file/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_file_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/file/{id}/download
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_file_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/file/{id}/upload
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_file_id_upload() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/id/{count}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_id_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/input/compare
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_input_compare() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/input/cover
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_input_cover() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/input/create
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_input_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/input/prepare/cover
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_input_prepare_cover() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/input/prepare/create
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_input_prepare_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/output/list
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_output_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/output/{flag}/select/file
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_output_flag_select_file() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/output/{portalFlag}/select
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_output_portalFlag_select() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/page/list/portal/{portalId}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_page_list_portal_portalId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/page/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_page_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/pageversion/list/page/{pageId}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_pageversion_list_page_pageId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/pageversion/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_pageversion_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portal/list
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portal_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portal/list/portalcategory/{portalCategory}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portal_list_portalcategory_portalCategory() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portal/list/summary
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portal_list_summary() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portal/list/summary/portalcategory/{portalCategory}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portal_list_summary_portalcategory_portalCategory() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portal/list/summary/v2
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portal_list_summary_v2() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portal/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portal_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portal/{id}/icon
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portal_id_icon() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portal/{id}/permission
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portal_id_permission() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/portalcategory/list
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_portalcategory_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/script/list/manager
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_script_list_manager() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/script/list/paging/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_script_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/script/list/portal/{portalId}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_script_list_portal_portalId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/script/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_script_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/scriptversion/list/script/{scriptId}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_scriptversion_list_script_scriptId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/scriptversion/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_scriptversion_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/templatepage/list
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_templatepage_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/templatepage/list/category
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_templatepage_list_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/templatepage/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_templatepage_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/widget/list/portal/{portalId}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_widget_list_portal_portalId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/portal/assemble/designer/widget/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_designer_widget_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}
