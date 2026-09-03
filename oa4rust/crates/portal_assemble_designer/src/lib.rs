use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get, routing::post, routing::put, routing::delete,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::{option_to_json, ActionResult}};

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
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreatePortalRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let description = req.description.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_portal_design (id, name, description, creator, create_time, update_time) VALUES ($1, $2, $3, $4, NOW(), NOW())",
            &[&id, &name, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("description".to_string(), Value::String(description)),
    ]));
    Ok(Json(ActionResult::success(result)))
}

pub async fn get_design(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, description, content FROM x_portal_design WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("description".to_string(), Value::String(row.get("description")));
            if let Some(val) = option_to_json::<Value>(content.and_then(|s| serde_json::from_str(&s).ok())) {
                map.insert("components".to_string(), val);
            }
            let result = Value::Object(map);
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("design not found"))),
    }
}

pub async fn list_designs(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description, create_time, update_time FROM x_portal_design WHERE deleted_at IS NULL ORDER BY update_time DESC",
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
                ("description".to_string(), Value::String(row.get("description"))),
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

pub async fn save_design(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or("null");
    let content_str = content.to_string();

    let result = client
        .execute(
            "UPDATE x_portal_design SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("design not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("content".to_string(), Value::String(content.to_string())),
        ]),
    ))))
}

pub async fn list_pages_by_category(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("category".to_string(), Value::String(row.get("category")));
            if let Some(val) = option_to_json::<Value>(content.and_then(|s| serde_json::from_str(&s).ok())) {
                map.insert("content".to_string(), val);
            }
            map.insert("creator".to_string(), Value::String(row.get("creator")));
            map.insert("createTime".to_string(), Value::String(row.get("create_time")));
            map.insert("updateTime".to_string(), Value::String(row.get("update_time")));
            Value::Object(map)
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn get_page(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("category".to_string(), Value::String(row.get("category")));
            if let Some(val) = option_to_json::<Value>(content.and_then(|s| serde_json::from_str(&s).ok())) {
                map.insert("content".to_string(), val);
            }
            map.insert("creator".to_string(), Value::String(row.get("creator")));
            map.insert("createTime".to_string(), Value::String(row.get("create_time")));
            map.insert("updateTime".to_string(), Value::String(row.get("update_time")));
            let result = Value::Object(map);
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn create_page(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreatePageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<SavePageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
            ("saved".to_string(), Value::Bool(result > 0)),
            ("content".to_string(), Value::String(content.to_string())),
        ]),
    ))))
}

pub async fn delete_page(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn design_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_designs(pool).await
}

pub async fn design_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    get_design(pool, Path(id)).await
}

pub async fn design_save(
    pool: Extension<Pool>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = body.get("id").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("id is required".to_string()))?;
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or("null");
    let content_str = content.to_string();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_portal_design SET content = $1, update_time = NOW() WHERE id = $2 AND deleted_at IS NULL",
            &[&content_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("design not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("saved".to_string(), Value::Bool(result > 0)),
            ("content".to_string(), Value::String(content.to_string())),
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
        .route("/jaxrs/portal/design/list", get(design_list))
        .route("/jaxrs/portal/design/{id}", get(design_get))
        .route("/jaxrs/portal/design/save", post(design_save))
        .route("/jaxrs/portal/assemble/designer/dict/{id}", get(crate::dict_id))
        .route("/jaxrs/portal/assemble/designer/dict/list/paging/{page}/{size}/{size}", get(crate::dict_list_paging_page_size_size))
        .route("/jaxrs/portal/assemble/designer/dict/list/portal/{portalId}", get(crate::dict_list_portal_portalId))
        .route("/jaxrs/portal/assemble/designer/file/{flag}", get(crate::file_flag))
        .route("/jaxrs/portal/assemble/designer/file/download/{id}", get(crate::file_id_download))
        .route("/jaxrs/portal/assemble/designer/file/upload/{id}", post(crate::file_id_upload))
        .route("/jaxrs/portal/assemble/designer/file/list/application/{applicationFlag}", get(crate::file_list_application_applicationFlag))
        .route("/jaxrs/portal/assemble/designer/file/list/{id}/{next}/{count}", get(crate::file_list_id_next_count))
        .route("/jaxrs/portal/assemble/designer/{id}/{count}", get(crate::id_count))
        .route("/jaxrs/portal/assemble/designer/output/select/file/{flag}", get(crate::output_flag_select_file))
        .route("/jaxrs/portal/assemble/designer/output/select/{portalFlag}", get(crate::output_portalFlag_select))
        .route("/jaxrs/portal/assemble/designer/list/portal/{page}/{portalId}", get(crate::page_list_portal_portalId))
        .route("/jaxrs/portal/assemble/designer/pageversion/{id}", get(crate::pageversion_id))
        .route("/jaxrs/portal/assemble/designer/pageversion/list/{page}/{pageId}", get(crate::pageversion_list_page_pageId))
        .route("/jaxrs/portal/assemble/designer/portal/{id}", get(crate::portal_id))
        .route("/jaxrs/portal/assemble/designer/portal/icon/{id}", get(crate::portal_id_icon))
        .route("/jaxrs/portal/assemble/designer/portal/permission/{id}", get(crate::portal_id_permission))
        .route("/jaxrs/portal/assemble/designer/portal/list/portalcategory/{portalCategory}", get(crate::portal_list_portalcategory_portalCategory))
        .route("/jaxrs/portal/assemble/designer/portal/list/summary/portalcategory/{portalCategory}", get(crate::portal_list_summary_portalcategory_portalCategory))
        .route("/jaxrs/portal/assemble/designer/script/{id}", get(crate::script_id))
        .route("/jaxrs/portal/assemble/designer/script/list/paging/{page}/{size}/{size}", get(crate::script_list_paging_page_size_size))
        .route("/jaxrs/portal/assemble/designer/script/list/portal/{portalId}", get(crate::script_list_portal_portalId))
        .route("/jaxrs/portal/assemble/designer/scriptversion/{id}", get(crate::scriptversion_id))
        .route("/jaxrs/portal/assemble/designer/scriptversion/list/script/{scriptId}", get(crate::scriptversion_list_script_scriptId))
        .route("/jaxrs/portal/assemble/designer/templatepage/{id}", get(crate::templatepage_id))
        .route("/jaxrs/portal/assemble/designer/widget/{id}", get(crate::widget_id))
        .route("/jaxrs/portal/assemble/designer/widget/list/portal/{portalId}", get(crate::widget_list_portal_portalId))
        .route("/jaxrs/portal/assemble/designer/page/delete/{id}", delete(delete_page))
        .route("/jaxrs/portal/design/save", put(design_save))
        .route("/jaxrs/portal/assemble/designer/save/{id}", put(save_design))
        .route("/jaxrs/portal/assemble/designer/page/save/{id}", put(save_page))
        // ── plan002 U2: page/file/import 族 + 动词差 缺口 (20) ──
        .route("/jaxrs/portal/assemble/designer/page", post(crate::create_page))
        .route("/jaxrs/portal/assemble/designer/page/list/portal/{portalId}", get(crate::page_list_portal_portalId))
        .route("/jaxrs/portal/assemble/designer/page/{id}", delete(crate::delete_page))
        .route("/jaxrs/portal/assemble/designer/page/{id}", put(crate::save_page))
        .route("/jaxrs/portal/assemble/designer/pageversion/list/page/{pageId}", get(crate::pageversion_list_page_pageId))
        .route("/jaxrs/portal/assemble/designer/portal", post(crate::create_portal))
        .route("/jaxrs/portal/assemble/designer/portal/list/summary", get(crate::portal_list_summary))
        .route("/jaxrs/portal/assemble/designer/portal/list/summary/v2", post(crate::portal_list_summary_v2))
        .route("/jaxrs/portal/assemble/designer/portal/{id}", delete(crate::delete_portal))
        .route("/jaxrs/portal/assemble/designer/portal/{id}", put(crate::update_portal))
        .route("/jaxrs/portal/assemble/designer/portal/{id}/icon", put(crate::update_portal_icon))
        .route("/jaxrs/portal/assemble/designer/portal/{id}/permission", post(crate::portal_id_permission_post))
        .route("/jaxrs/portal/assemble/designer/templatepage", post(crate::create_templatepage))
        .route("/jaxrs/portal/assemble/designer/templatepage/list", get(crate::templatepage_list))
        .route("/jaxrs/portal/assemble/designer/templatepage/list/category", get(crate::templatepage_list_category))
        .route("/jaxrs/portal/assemble/designer/templatepage/list/category", put(crate::update_templatepage_category))
        .route("/jaxrs/portal/assemble/designer/templatepage/{id}", delete(crate::delete_templatepage))
        .route("/jaxrs/portal/assemble/designer/widget", post(crate::create_widget))
        .route("/jaxrs/portal/assemble/designer/widget/{id}", delete(crate::delete_widget))
        .route("/jaxrs/portal/assemble/designer/widget/{id}", put(crate::update_widget))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    portal_assemble_designer_router().layer(axum::extract::Extension(pool))
}



pub async fn designer_search(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, create_time, update_time FROM x_portal_design WHERE deleted_at IS NULL ORDER BY update_time DESC LIMIT 20",
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
                ("category".to_string(), Value::String(row.get("category"))),
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

pub async fn dict_list_paging_page_size_size(
    pool: Extension<Pool>,
    Path(_page): Path<i64>,
    Path(_size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, app_name, create_time FROM x_portal_dict WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2::bigint OFFSET ($1 - 1) * $2",
            &[&_page, &_size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn dict_list_portal_portalId(
    pool: Extension<Pool>,
    Path(portal_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, app_name, create_time FROM x_portal_dict WHERE portal_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn dict_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, app_name, app_data, creator, create_time FROM x_portal_dict WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("appData".to_string(), Value::String(row.get("app_data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn file_list_application_applicationFlag(
    pool: Extension<Pool>,
    Path(application_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, file_type, creator, create_time FROM x_portal_file WHERE application_flag = $1 ORDER BY create_time DESC",
            &[&application_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
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

pub async fn file_list_id_next_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, file_type, creator, create_time FROM x_portal_file WHERE id > $1 ORDER BY id ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
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

pub async fn file_list_id_prev_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, file_type, creator, create_time FROM x_portal_file WHERE id < $1 ORDER BY id DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
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

pub async fn file_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, file_type, content, creator, create_time FROM x_portal_file WHERE flag = $1 LIMIT 1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, file_type, content, creator, create_time FROM x_portal_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id_download(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, file_type, content FROM x_portal_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("content".to_string(), Value::String(row.get("content"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id_upload(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or("null");
    let content_str = content.to_string();

    let result = client
        .execute(
            "UPDATE x_portal_file SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("file not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("uploaded".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn id_count(
    pool: Extension<Pool>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one("SELECT COUNT(*) as cnt FROM x_portal", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let total: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("total".to_string(), Value::Number(serde_json::Number::from(total))),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn input_compare(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let content_str = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();

    let row = client
        .query_opt(
            "SELECT id, content FROM x_portal_input WHERE id = $1",
            &[&input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let old_content: Option<String> = row.get("content");
            let mut result_map = serde_json::Map::new();
            result_map.insert("id".to_string(), Value::String(input_id.to_string()));
            if let Some(val) = option_to_json(old_content.as_ref().map(|s| Value::String(s.to_string()))) {
                result_map.insert("oldContent".to_string(), val);
            }
            result_map.insert("newContent".to_string(), Value::String(content_str.to_string()));
            let compared = old_content.is_some();
            result_map.insert("compared".to_string(), Value::Bool(compared));
            let result = Value::Object(result_map);
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("input not found"))),
    }
}

pub async fn input_cover(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let content_str = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    let result = client
        .execute(
            "UPDATE x_portal_input SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content_str, &input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("input not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(input_id.to_string())),
            ("covered".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn input_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    let result = client
        .execute(
            "INSERT INTO x_portal_input (id, content, creator, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn input_prepare_cover(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let row = client
        .query_opt(
            "SELECT id, content FROM x_portal_input WHERE id = $1",
            &[&input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(input_id.to_string()));
            if let Some(val) = option_to_json(content.map(|s| Value::String(s))) {
                map.insert("content".to_string(), val);
            }
            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Ok(Json(ActionResult::error("input not found"))),
    }
}

pub async fn input_prepare_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "INSERT INTO x_portal_input (id, content, creator, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn output_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_portal_output WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
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

pub async fn output_flag_select_file(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, select_file FROM x_portal_output WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("selectFile".to_string(), Value::String(row.get("select_file"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("output not found"))),
    }
}

pub async fn output_portalFlag_select(
    pool: Extension<Pool>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_portal_output WHERE portal_flag = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
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

pub async fn page_list_portal_portalId(
    pool: Extension<Pool>,
    Path(portal_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time, update_time FROM x_portal_page WHERE portal_id = $1 ORDER BY update_time DESC",
            &[&portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
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

pub async fn page_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("category".to_string(), Value::String(row.get("category")));
            if let Some(val) = option_to_json::<Value>(content.and_then(|s| serde_json::from_str(&s).ok())) {
                map.insert("content".to_string(), val);
            }
            map.insert("creator".to_string(), Value::String(row.get("creator")));
            map.insert("createTime".to_string(), Value::String(row.get("create_time")));
            map.insert("updateTime".to_string(), Value::String(row.get("update_time")));
            let result = Value::Object(map);
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn pageversion_list_page_pageId(
    pool: Extension<Pool>,
    Path(page_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, page_id, version, creator, create_time FROM x_portal_page_version WHERE page_id = $1 ORDER BY create_time DESC",
            &[&page_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("pageId".to_string(), Value::String(row.get("page_id"))),
                ("version".to_string(), Value::String(row.get("version"))),
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

pub async fn pageversion_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, page_id, version, content, creator, create_time FROM x_portal_page_version WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("pageId".to_string(), Value::String(row.get("page_id"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page version not found"))),
    }
}

pub async fn portal_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, logo, creator, create_time FROM x_portal WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
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

pub async fn portal_list_portalcategory_portalCategory(
    pool: Extension<Pool>,
    Path(portal_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, logo, creator, create_time FROM x_portal WHERE category = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
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

pub async fn portal_list_summary(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, logo, creator, create_time FROM x_portal WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn portal_list_summary_portalcategory_portalCategory(
    pool: Extension<Pool>,
    Path(portal_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, logo FROM x_portal WHERE category = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn portal_list_summary_v2(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, logo, description, creator, create_time FROM x_portal WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
                ("description".to_string(), Value::String(row.get("description"))),
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

pub async fn portal_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, logo, description, creator, create_time, update_time FROM x_portal WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn portal_id_icon(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, logo FROM x_portal WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn portal_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_portal WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("permission".to_string(), Value::String(row.get("permission"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn portalcategory_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT DISTINCT category FROM x_portal WHERE deleted_at IS NULL AND category IS NOT NULL ORDER BY category",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("category".to_string(), Value::String(row.get("category"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn script_list_manager(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_portal_script WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
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

pub async fn script_list_paging_page_size_size(
    pool: Extension<Pool>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_portal_script WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2::bigint OFFSET ($1 - 1) * $2",
            &[&page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
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

pub async fn script_list_portal_portalId(
    pool: Extension<Pool>,
    Path(portal_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_portal_script WHERE portal_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
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

pub async fn script_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, category, content, creator, create_time FROM x_portal_script WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

pub async fn scriptversion_list_script_scriptId(
    pool: Extension<Pool>,
    Path(script_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, script_id, version, creator, create_time FROM x_portal_script_version WHERE script_id = $1 ORDER BY create_time DESC",
            &[&script_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("scriptId".to_string(), Value::String(row.get("script_id"))),
                ("version".to_string(), Value::String(row.get("version"))),
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

pub async fn scriptversion_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, script_id, version, content, creator, create_time FROM x_portal_script_version WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("scriptId".to_string(), Value::String(row.get("script_id"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script version not found"))),
    }
}

pub async fn templatepage_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time FROM x_portal_template_page WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), Value::String(row.get("content"))),
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

pub async fn templatepage_list_category(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT DISTINCT category FROM x_portal_template_page WHERE deleted_at IS NULL AND category IS NOT NULL ORDER BY category",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("category".to_string(), Value::String(row.get("category"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn templatepage_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time FROM x_portal_template_page WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(row.get("id")));
            map.insert("name".to_string(), Value::String(row.get("name")));
            map.insert("category".to_string(), Value::String(row.get("category")));
            if let Some(val) = option_to_json::<Value>(content.and_then(|s| serde_json::from_str(&s).ok())) {
                map.insert("content".to_string(), val);
            }
            map.insert("creator".to_string(), Value::String(row.get("creator")));
            map.insert("createTime".to_string(), Value::String(row.get("create_time")));
            let result = Value::Object(map);
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("template page not found"))),
    }
}

pub async fn widget_list_portal_portalId(
    pool: Extension<Pool>,
    Path(portal_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, portal_id, category, config, creator, create_time FROM x_portal_widget WHERE portal_id = $1 ORDER BY create_time DESC",
            &[&portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("portalId".to_string(), Value::String(row.get("portal_id"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("config".to_string(), Value::String(row.get("config"))),
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

pub async fn widget_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, portal_id, category, config, creator, create_time, update_time FROM x_portal_widget WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("portalId".to_string(), Value::String(row.get("portal_id"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("config".to_string(), Value::String(row.get("config"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("widget not found"))),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// plan002 U2: page/file/import 族 + 动词差 缺口补建 (20 gap)
// 复用既有 x_portal_* 表，参数化真实 SQL；归一化查重 / IDOR 门禁。
// ─────────────────────────────────────────────────────────────────────────────

pub async fn create_portal(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreatePortalRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = req.name.clone().unwrap_or_default();
    let description = req.description.clone().unwrap_or_default();
    let alias = name.clone();
    let portal_category = "default".to_string();
    let category = "default".to_string();
    let creator = "system";

    // 归一化查重：同名 portal 视为重复
    let existing = client
        .query_opt(
            "SELECT id FROM x_portal WHERE name = $1 AND deleted_at IS NULL",
            &[&name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if existing.is_some() {
        return Ok(Json(ActionResult::error("portal already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_portal (id, name, alias, description, portal_category, category, creator, create_time, update_time) \
              VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())",
            &[&id, &name, &alias, &description, &portal_category, &category, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]),
    ))))
}

pub async fn delete_portal(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_portal SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("portal not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn update_portal(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let description = body.get("description").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let category = body.get("category").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let logo = body.get("logo").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let result = client
        .execute(
            "UPDATE x_portal SET name = $1, description = $2, category = $3, logo = $4, update_time = NOW() WHERE id = $5 AND deleted_at IS NULL",
            &[&name, &description, &category, &logo, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("portal not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn update_portal_icon(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let logo = body.get("logo").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let result = client
        .execute(
            "UPDATE x_portal SET logo = $1, update_time = NOW() WHERE id = $2 AND deleted_at IS NULL",
            &[&logo, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("portal not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("logo".to_string(), Value::String(logo)),
        ]),
    ))))
}

pub async fn portal_id_permission_post(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let permission = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_portal SET permission = $1, update_time = NOW() WHERE id = $2 AND deleted_at IS NULL",
            &[&permission, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("portal not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("permission".to_string(), Value::String(permission)),
        ]),
    ))))
}

pub async fn create_templatepage(
    pool: Extension<Pool>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let category = body.get("category").and_then(|v| v.as_str()).unwrap_or("default").to_string();
    let content_str = body
        .get("content")
        .and_then(|v| serde_json::to_string(v).ok())
        .unwrap_or_else(|| "null".to_string());
    let creator = "system";

    let existing = client
        .query_opt(
            "SELECT id FROM x_portal_template_page WHERE name = $1 AND deleted_at IS NULL",
            &[&name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if existing.is_some() {
        return Ok(Json(ActionResult::error("template page already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_portal_template_page (id, name, category, content, creator, create_time, update_time) \
              VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &content_str, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]),
    ))))
}

pub async fn update_templatepage_category(
    pool: Extension<Pool>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let category = body.get("category").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time FROM x_portal_template_page WHERE category = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("category".to_string(), Value::String(category)),
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn delete_templatepage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_portal_template_page SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("template page not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn create_widget(
    pool: Extension<Pool>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let portal_id = body.get("portalId").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let category = body.get("category").and_then(|v| v.as_str()).unwrap_or("default").to_string();
    let config_str = body
        .get("config")
        .and_then(|v| serde_json::to_string(v).ok())
        .unwrap_or_else(|| "null".to_string());
    let creator = "system";

    let existing = client
        .query_opt(
            "SELECT id FROM x_portal_widget WHERE name = $1 AND portal_id = $2",
            &[&name, &portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if existing.is_some() {
        return Ok(Json(ActionResult::error("widget already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_portal_widget (id, name, portal_id, category, config, creator, create_time, update_time) \
              VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[&id, &name, &portal_id, &category, &config_str, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("portalId".to_string(), Value::String(portal_id)),
        ]),
    ))))
}

pub async fn delete_widget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_portal_widget SET deleted_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("widget not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn update_widget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let category = body.get("category").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let config_str = body
        .get("config")
        .and_then(|v| serde_json::to_string(v).ok())
        .unwrap_or_else(|| "null".to_string());
    let result = client
        .execute(
            "UPDATE x_portal_widget SET name = $1, category = $2, config = $3, update_time = NOW() WHERE id = $4",
            &[&name, &category, &config_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("widget not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}


