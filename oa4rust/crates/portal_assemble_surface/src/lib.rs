use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateSurfaceRequest {
    pub name: Option<String>,
    pub template: Option<String>,
}

pub async fn get_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Portal Surface".to_string())),
            ("html".to_string(), Value::String("<div></div>".to_string())),
        ]),
    ))))
}

pub async fn create_surface(
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("template".to_string(), Value::String(req.template.unwrap_or_default())),
        ]),
    ))))
}

pub async fn list_surfaces(
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("surface-1".to_string())),
            ("name".to_string(), Value::String("Surface 1".to_string())),
            ("category".to_string(), Value::String(category)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn preview_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("preview_url".to_string(), Value::String(format!("/preview/{}", id))),
            ("html".to_string(), Value::String("<div>Preview</div>".to_string())),
        ]),
    ))))
}

pub async fn publish_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("published".to_string(), Value::Bool(true)),
            ("published_at".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ]),
    ))))
}

pub fn portal_assemble_surface_router() -> Router {
    Router::new()
        .route("/jaxrs/portal/assemble/surface/get/{id}", get(get_surface))
        .route("/jaxrs/portal/assemble/surface/create", post(create_surface))
        .route("/jaxrs/portal/assemble/surface/list/{category}", get(list_surfaces))
        .route("/jaxrs/portal/assemble/surface/preview/{id}", get(preview_surface))
        .route("/jaxrs/portal/assemble/surface/publish/{id}", post(publish_surface))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/portal_assemble_surface/health", axum::routing::get(|| async { "TODO: portal_assemble_surface - real implementation needed" }))
}


/// Stub handler for /jaxrs/portal/assemble/surface/dict/list/portal/{portalFlag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_dict_list_portal_portalFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_dict_dictFlag_portal_portalFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/data
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_dict_dictFlag_portal_portalFlag_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_dict_dictFlag_portal_portalFlag_path_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_dict_dictFlag_portal_portalFlag_path_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_dict_dictFlag_portal_portalFlag_path_data_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/file/list/portal/{portalFlag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_file_list_portal_portalFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/file/{flag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_file_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/file/{flag}/download
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_file_flag_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/file/{flag}/portal/{portalFlag}/content
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_file_flag_portal_portalFlag_content() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/file/{flag}/portal/{portalFlag}/download
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_file_flag_portal_portalFlag_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/list/portal/{portal}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_list_portal_portal() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/v2/{flag}/portal/{portalFlag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_v2_flag_portal_portalFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/v2/{flag}/portal/{portalFlag}/mobile
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_v2_flag_portal_portalFlag_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/v2/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_v2_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/v2/{id}/mobile
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_v2_id_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/{flag}/portal/{portalFlag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_flag_portal_portalFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/{flag}/portal/{portalFlag}/mobile
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_flag_portal_portalFlag_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/page/{id}/mobile
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_page_id_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/portal/list
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_portal_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/portal/list/mobile
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_portal_list_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/portal/{flag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_portal_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/portal/{flag}/corner/mark
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_portal_flag_corner_mark() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/portal/{id}/icon
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_portal_id_icon() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/portal/{id}/icon/base64
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_portal_id_icon_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/script/list/portal/{portal}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_script_list_portal_portal() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/script/portal/{portal}/name/{name}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_script_portal_portal_name_name() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/script/portal/{portal}/name/{name}/imported
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_script_portal_portal_name_name_imported() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/script/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_script_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/widget/list/portal/{portal}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_widget_list_portal_portal() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/widget/{flag}/portal/{portalFlag}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_widget_flag_portal_portalFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/widget/{flag}/portal/{portalFlag}/mobile
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_widget_flag_portal_portalFlag_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/widget/{id}
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_widget_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}

/// Stub handler for /jaxrs/portal/assemble/surface/widget/{id}/mobile
/// TODO: Implement real business logic
pub async fn stub_portal_assemble_surface_widget_id_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Null)))
}
