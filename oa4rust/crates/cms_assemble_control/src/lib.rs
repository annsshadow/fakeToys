use axum::{
    extract::Extension,
    Json,
    Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

#[axum::debug_handler]
pub async fn get_control_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT enabled, max_category_count, allow_anonymous FROM x_cms_assemble_control_config ORDER BY create_time LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(row.get("enabled"))),
        ("maxCategoryCount".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i64>("max_category_count")))),
        ("allowAnonymous".to_string(), Value::Bool(row.get("allow_anonymous"))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_sections(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, enabled FROM x_cms_assemble_control_section ORDER BY create_time",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let sections: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("enabled".to_string(), Value::Bool(row.get("enabled"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Array(sections))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let enabled = body.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
    let max_category_count = body.get("maxCategoryCount").and_then(|v| v.as_i64()).unwrap_or(500);
    let allow_anonymous = body.get("allowAnonymous").and_then(|v| v.as_bool()).unwrap_or(false);

    client
        .execute(
            "UPDATE x_cms_assemble_control_config SET enabled = $1, max_category_count = $2, allow_anonymous = $3 WHERE id = (SELECT id FROM x_cms_assemble_control_config ORDER BY create_time LIMIT 1)",
            &[&enabled, &max_category_count, &allow_anonymous],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), body.0),
        ]),
    ))))
}

pub fn cms_assemble_control_router(pool: Pool) -> Router {
    routes::router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_assemble_control_router(pool)
}



pub async fn anonymous_document_filter_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn anonymous_document_filter_list_id_next_count_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn anonymous_document_filter_list_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn anonymous_document_filter_list_page_size_size_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn anonymous_document_id_view() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_fileinfo_download_document_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_fileinfo_download_document_id_stream() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_fileinfo_list_document_documentId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn anonymous_fileinfo_id_document_documentId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_form_v2_lookup_document_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_form_v2_lookup_document_docId_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_form_v2_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_form_v2_id_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_form_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_list_appInfo_appInfoFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn anonymous_surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appconfig_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appconfig_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_alias_alias() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_erase_app_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_erase_app_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_filter_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_filter_list_id_next_count_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_filter_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_filter_list_id_prev_count_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_get_user_publish_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_appType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_appType_manager() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_has_document() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_has_document_appType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_has_document_type_appType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_manage() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_manage_type_appType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_publish() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_publish_type_appType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_publish_with_process() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_view() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_view_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_view_all_type_appType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_view_article_type_appType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_view_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_list_user_view_data_type_appType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn appinfo_appId_icon_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_id_control() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn appinfo_id_permission() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_alias_alias() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_bind_categoryId_view() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_bind_categoryId_view_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_erase_category_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_erase_category_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_extContent() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_filter_list_id_next_count_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_filter_list_id_next_count_app_appId_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_filter_list_id_prev_count_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_filter_list_id_prev_count_app_appId_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_filter_list_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_filter_list_page_size_size_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_list_manage_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_list_objects() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_list_publish_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_list_view_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_list_view_app_appId_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_list_view_app_appId_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn categoryinfo_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_id_control() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_id_execute_projection() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn categoryinfo_id_permission() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn commend_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn commend_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn comment_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn comment_list_id_next_count_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn comment_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn comment_list_id_prev_count_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn comment_list_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn comment_list_page_size_size_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn comment_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn comment_id_commend() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn comment_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn comment_id_uncommend() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn correlation_doc_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn correlation_doc_docId_delete() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn correlation_list_doc_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn correlation_list_doc_docId_site_site() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn correlation_update_doc_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_array_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_path7() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn data_document_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn design_appdict_list_appInfo_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn design_appdict_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn design_appdict_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn design_appdict_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn design_appdict_id_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn designer_search() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn document_cipher_filter_list_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn document_cipher_filter_list_page_size_size_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn document_cipher_publish_content() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn document_cipher_publish_content_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn document_cipher_id_permission_read_person_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn document_cipher_id_persist_view_record() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_list_appInfo_appInfoFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn file_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_flag_appInfo_appInfoFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_flag_appInfo_appInfoFlag_content() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_flag_appInfo_appInfoFlag_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id_content() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn file_id_upload() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_batch_download_doc_docId_site_site() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_copy_to_doc_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_download_document_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_download_document_id_stream() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_download_transfer_flag_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_edit_id_doc_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_edit_id_doc_docId_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn fileinfo_list_document_documentId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn fileinfo_list_filter() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn fileinfo_replace_to_doc_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_update_document_docId_attachment_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_update_document_docId_attachment_id_callback_callback() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_update_id_content() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_upload_doc_docId_save_as_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_upload_document_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_upload_document_docId_callback_callback() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_upload_with_url() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_id_binary_base64_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_id_doc_docId_change_seqnumber_seqNumber() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_id_document_documentId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_id_online_info() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn fileinfo_id_preview_pdf() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_filter_list_id_next_count_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn form_filter_list_id_next_count_app_appId_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn form_filter_list_id_prev_count_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn form_filter_list_id_prev_count_app_appId_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn form_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn form_list_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn form_list_formfield_appInfo_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn form_list_id_formfield() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn form_v2_lookup_document_docId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_v2_lookup_document_docId_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_v2_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_v2_id_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_formFlag_appinfo_appFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn form_id_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn formversion_list_form_formId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn formversion_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn image_encode_base64() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn image_encode_base64_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn image_resize_id_id_width_width_height_height() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_compare() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_compare_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_cover() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_cover_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_create_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_prepare_cover() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_prepare_cover_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_prepare_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_prepare_create_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn log_filter_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn log_filter_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn log_list_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn log_list_category_categoryId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn log_list_document_documentId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn log_list_filter_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn log_list_level_operationLevel() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn log_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn output_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn output_appInfoFlag_select() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn output_appInfoFlag_select_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_appInfo_id_manageable() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_appInfo_id_managers() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_appInfo_id_publishers() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_appInfo_id_viewers() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_category_id_managers() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_category_id_publishers() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_category_id_viewers() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_categoryInfo_id_manageable() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_management_refresh_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_management_refresh_category_categoryId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_manager_appInfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_manager_categoryInfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_publisher_appInfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_publisher_categoryInfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_viewer_appInfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn permission_viewer_categoryInfo_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn review_v2_search() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn script_list_app_appId_name_name() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn script_list_app_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn script_list_manager() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn script_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn script_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn script_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn script_flag_appInfo_appInfoFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn script_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn script_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn script_id_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn script_uniqueName_app_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn script_uniqueName_app_flag_imported() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn scriptversion_list_script_scriptId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn scriptversion_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn searchfilter_list_archive_filter_category_categoryId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn searchfilter_list_draft_filter_category_categoryId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn searchfilter_list_publish_filter_category_categoryId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn surface_appdict_list_appInfo_appInfoFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_data_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_data_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_data_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn surface_appdict_appDictFlag_appInfo_appInfoFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn templateform_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn templateform_list_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn templateform_list_category_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn templateform_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn templateform_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn uuid_random() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn view_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn view_list_app_appId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn view_list_category_categoryId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn view_list_form_formId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn view_viewdata_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn view_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn view_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn view_id_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn viewcategory_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn viewcategory_list_category_categoryId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn viewcategory_list_view_viewId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn viewcategory_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn viewcategory_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn viewfieldconfig_list_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn viewfieldconfig_list_view_viewId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn viewfieldconfig_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn viewfieldconfig_id_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn viewfieldconfig_id_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn viewrecord_document_docId_filter_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn viewrecord_document_docId_has_view() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn viewrecord_list_install_log_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}
