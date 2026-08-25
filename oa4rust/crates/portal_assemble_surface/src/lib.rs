use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get, routing::post, routing::put, routing::delete,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreateSurfaceRequest {
    pub name: Option<String>,
    pub template: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLayoutRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub content: Option<String>,
}

pub async fn get_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, html, template, creator, create_time, update_time \
              FROM x_portal_surface WHERE id = $1",
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
                ("html".to_string(), Value::String(row.get("html"))),
                ("template".to_string(), Value::String(row.get("template"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("surface not found"))),
    }
}

pub async fn create_surface(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let template = req.template.unwrap_or_default();
    let category = "default".to_string();
    let html = "<div></div>".to_string();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_portal_surface (id, name, category, html, template, creator, create_time, update_time) \
              VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[&id, &name, &category, &html, &template, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("template".to_string(), Value::String(template)),
        ("category".to_string(), Value::String(category)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn list_surfaces(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, html, template, creator, create_time, update_time \
              FROM x_portal_surface WHERE category = $1 ORDER BY update_time DESC",
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
                ("html".to_string(), Value::String(row.get("html"))),
                ("template".to_string(), Value::String(row.get("template"))),
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

pub async fn preview_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, html FROM x_portal_surface WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("preview_url".to_string(), Value::String(format!("/preview/{}", id))),
                    ("html".to_string(), Value::String(row.get("html"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("surface not found"))),
    }
}

pub async fn publish_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_portal_surface SET published = true, published_at = NOW(), update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("surface not found")));
    }

    let row = client
        .query_one(
            "SELECT id, published, published_at FROM x_portal_surface WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("published".to_string(), Value::String(row.get("published"))),
            ("publishedAt".to_string(), Value::String(row.get("published_at"))),
        ]),
    ))))
}

pub async fn surface_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, portal_id, name, preview_url, published, create_time FROM x_portal_surface WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("previewUrl".to_string(), Value::String(row.get("preview_url"))),
                ("published".to_string(), Value::Bool(row.get("published"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn surface_preview(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, html FROM x_portal_surface WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("previewUrl".to_string(), Value::String(format!("/preview/{}", id))),
                    ("html".to_string(), Value::String(row.get("html"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("surface not found"))),
    }
}

pub async fn surface_publish(
    pool: Extension<Pool>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = body.get("id").and_then(|v| v.as_str()).ok_or(AppError::BadRequest("id is required".to_string()))?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_portal_surface SET published = true, published_at = NOW(), update_time = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("surface not found")));
    }

    let row = client
        .query_one(
            "SELECT id, published, published_at FROM x_portal_surface WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("published".to_string(), Value::Bool(row.get("published"))),
            ("publishedAt".to_string(), Value::String(row.get("published_at"))),
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
        .route("/jaxrs/portal/surface/list", get(surface_list))
        .route("/jaxrs/portal/surface/{id}/preview", get(surface_preview))
        .route("/jaxrs/portal/surface/publish", post(surface_publish))
        .route("/jaxrs/portal/assemble/surface/delete/layout", post(crate::delete_layout))
        .route("/jaxrs/portal/assemble/surface/dict/portal/{dictFlag}/{portalFlag}", get(crate::dict_dictFlag_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/dict/portal/data/{dictFlag}/{portalFlag}", get(crate::dict_dictFlag_portal_portalFlag_data))
        .route("/jaxrs/portal/assemble/surface/dict/portal/path/data/{dictFlag}/{portalFlag}", get(crate::dict_dictFlag_portal_portalFlag_path_data))
        .route("/jaxrs/portal/assemble/surface/dict/portal/path/data/mockdeletetoget/{dictFlag}/{portalFlag}", post(crate::dict_dictFlag_portal_portalFlag_path_data_mockdeletetoget))
        .route("/jaxrs/portal/assemble/surface/dict/portal/path/data/mockputtopost/{dictFlag}/{portalFlag}", post(crate::dict_dictFlag_portal_portalFlag_path_data_mockputtopost))
        .route("/jaxrs/portal/assemble/surface/dict/list/portal/{portalFlag}", get(crate::dict_list_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/file/{flag}", get(crate::file_flag))
        .route("/jaxrs/portal/assemble/surface/file/download/{flag}", get(crate::file_flag_download))
        .route("/jaxrs/portal/assemble/surface/file/portal/content/{flag}/{portalFlag}", get(crate::file_flag_portal_portalFlag_content))
        .route("/jaxrs/portal/assemble/surface/file/portal/download/{flag}/{portalFlag}", get(crate::file_flag_portal_portalFlag_download))
        .route("/jaxrs/portal/assemble/surface/file/list/portal/{portalFlag}", get(crate::file_list_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/get/layout", get(crate::get_layout))
        .route("/jaxrs/portal/assemble/surface/list/layouts", get(crate::list_layouts))
        .route("/jaxrs/portal/assemble/surface/portal/{page}/{flag}/{portalFlag}", get(crate::page_flag_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/portal/mobile/{page}/{flag}/{portalFlag}", get(crate::page_flag_portal_portalFlag_mobile))
        .route("/jaxrs/portal/assemble/surface/{page}/{id}", get(crate::page_id))
        .route("/jaxrs/portal/assemble/surface/mobile/{page}/{id}", get(crate::page_id_mobile))
        .route("/jaxrs/portal/assemble/surface/list/portal/portal/{page}", get(crate::page_list_portal_portal))
        .route("/jaxrs/portal/assemble/surface/v2/portal/{page}/{flag}/{portalFlag}", get(crate::page_v2_flag_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/v2/portal/mobile/{page}/{flag}/{portalFlag}", get(crate::page_v2_flag_portal_portalFlag_mobile))
        .route("/jaxrs/portal/assemble/surface/v2/{page}/{id}", get(crate::page_v2_id))
        .route("/jaxrs/portal/assemble/surface/v2/mobile/{page}/{id}", get(crate::page_v2_id_mobile))
        .route("/jaxrs/portal/assemble/surface/portal/{flag}", get(crate::portal_flag))
        .route("/jaxrs/portal/assemble/surface/portal/corner/mark/{flag}", get(crate::portal_flag_corner_mark))
        .route("/jaxrs/portal/assemble/surface/portal/icon/{id}", get(crate::portal_id_icon))
        .route("/jaxrs/portal/assemble/surface/portal/icon/base64/{id}", get(crate::portal_id_icon_base64))
        .route("/jaxrs/portal/assemble/surface/save/layout", post(crate::save_layout))
        .route("/jaxrs/portal/assemble/surface/script/{id}", get(crate::script_id))
        .route("/jaxrs/portal/assemble/surface/script/list/portal/portal", get(crate::script_list_portal_portal))
        .route("/jaxrs/portal/assemble/surface/script/portal/portal/{name}/{name}", get(crate::script_portal_portal_name_name))
        .route("/jaxrs/portal/assemble/surface/script/portal/portal/imported/{name}/{name}", post(crate::script_portal_portal_name_name_imported))
        .route("/jaxrs/portal/assemble/surface/widget/portal/{flag}/{portalFlag}", get(crate::widget_flag_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/widget/portal/mobile/{flag}/{portalFlag}", get(crate::widget_flag_portal_portalFlag_mobile))
        .route("/jaxrs/portal/assemble/surface/widget/{id}", get(crate::widget_id))
        .route("/jaxrs/portal/assemble/surface/widget/mobile/{id}", get(crate::widget_id_mobile))
        .route("/jaxrs/portal/assemble/surface/widget/list/portal/portal", get(crate::widget_list_portal_portal))
        .route("/jaxrs/portal/assemble/surface/delete/layout", delete(delete_layout))
        .route("/jaxrs/portal/assemble/surface/dict/portal/path/data/mockdeletetoget/{dictFlag}/{portalFlag}", delete(dict_dictFlag_portal_portalFlag_path_data_mockdeletetoget))
        .route("/jaxrs/portal/assemble/surface/save/layout", put(save_layout))
        // ── plan002 U2: portal/dict 族缺口 (27) ──
        .route("/jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}", get(crate::dict_dictFlag_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/data", get(crate::dict_dictFlag_portal_portalFlag_data))
        .route("/jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data", delete(crate::dict_dictFlag_portal_portalFlag_path_data_delete))
        .route("/jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data", get(crate::dict_dictFlag_portal_portalFlag_path_data))
        .route("/jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data", post(crate::dict_dictFlag_portal_portalFlag_path_data_post))
        .route("/jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data", put(crate::dict_dictFlag_portal_portalFlag_path_data_put))
        .route("/jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data/mockdeletetoget", get(crate::dict_dictFlag_portal_portalFlag_path_data_mockdeletetoget))
        .route("/jaxrs/portal/assemble/surface/dict/{dictFlag}/portal/{portalFlag}/{path}/data/mockputtopost", post(crate::dict_dictFlag_portal_portalFlag_path_data_mockputtopost))
        .route("/jaxrs/portal/assemble/surface/file/{flag}/portal/{portalFlag}/content", get(crate::file_flag_portal_portalFlag_content))
        .route("/jaxrs/portal/assemble/surface/file/{flag}/portal/{portalFlag}/download", get(crate::file_flag_portal_portalFlag_download))
        .route("/jaxrs/portal/assemble/surface/page/list/portal/{portal}", get(crate::page_list_portal_portal))
        .route("/jaxrs/portal/assemble/surface/page/v2/{id}", get(crate::page_v2_id))
        .route("/jaxrs/portal/assemble/surface/page/v2/{id}/mobile", get(crate::page_v2_id_mobile))
        .route("/jaxrs/portal/assemble/surface/page/v2/{flag}/portal/{portalFlag}", get(crate::page_v2_flag_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/page/v2/{flag}/portal/{portalFlag}/mobile", get(crate::page_v2_flag_portal_portalFlag_mobile))
        .route("/jaxrs/portal/assemble/surface/page/{id}/mobile", get(crate::page_id_mobile))
        .route("/jaxrs/portal/assemble/surface/page/{flag}/portal/{portalFlag}", get(crate::page_flag_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/page/{flag}/portal/{portalFlag}/mobile", get(crate::page_flag_portal_portalFlag_mobile))
        .route("/jaxrs/portal/assemble/surface/portal/list/mobile", get(crate::portal_list_mobile))
        .route("/jaxrs/portal/assemble/surface/portal/{flag}/corner/mark", get(crate::portal_flag_corner_mark))
        .route("/jaxrs/portal/assemble/surface/portal/{id}/icon", get(crate::portal_id_icon))
        .route("/jaxrs/portal/assemble/surface/portal/{id}/icon/base64", get(crate::portal_id_icon_base64))
        .route("/jaxrs/portal/assemble/surface/script/portal/{portal}/name/{name}", post(crate::script_portal_portal_name_post))
        .route("/jaxrs/portal/assemble/surface/script/portal/{portal}/name/{name}/imported", get(crate::script_portal_portal_name_name_imported))
        .route("/jaxrs/portal/assemble/surface/widget/{id}/mobile", get(crate::widget_id_mobile))
        .route("/jaxrs/portal/assemble/surface/widget/{flag}/portal/{portalFlag}", get(crate::widget_flag_portal_portalFlag))
        .route("/jaxrs/portal/assemble/surface/widget/{flag}/portal/{portalFlag}/mobile", get(crate::widget_flag_portal_portalFlag_mobile))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub async fn get_layout(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time, update_time \
             FROM x_portal_layout WHERE id = $1",
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
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("layout not found"))),
    }
}

pub async fn list_layouts(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time, update_time \
             FROM x_portal_layout WHERE category = $1 ORDER BY update_time DESC",
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
                ("content".to_string(), Value::String(row.get("content"))),
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

pub async fn create_layout(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateLayoutRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let content = req.content.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_portal_layout (id, name, category, content, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("category".to_string(), Value::String(category)),
        ("content".to_string(), Value::String(content)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn save_layout(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<CreateLayoutRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let content = req.content.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_portal_layout SET name = $1, category = $2, content = $3, update_time = NOW() \
             WHERE id = $4",
            &[&name, &category, &content, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("layout not found")));
    }

    let row = client
        .query_one(
            "SELECT id, name, category, content, update_time FROM x_portal_layout WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("category".to_string(), Value::String(row.get("category"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("updateTime".to_string(), Value::String(row.get("update_time"))),
        ]),
    ))))
}

pub async fn delete_layout(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_portal_layout WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("layout not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    portal_assemble_surface_router().layer(axum::extract::Extension(pool))
}



pub async fn dict_list_portal_portalFlag(
    pool: Extension<Pool>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, app_name, key_name, creator, create_time FROM x_portal_dict WHERE portal_flag = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
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
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("keyName".to_string(), Value::String(row.get("key_name"))),
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

pub async fn dict_dictFlag_portal_portalFlag(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, app_name, key_name, app_data, creator, create_time FROM x_portal_dict WHERE flag = $1 AND portal_flag = $2 AND deleted_at IS NULL",
            &[&dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("keyName".to_string(), Value::String(row.get("key_name"))),
                ("appData".to_string(), Value::String(row.get("app_data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn dict_dictFlag_portal_portalFlag_data(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, app_data FROM x_portal_dict WHERE flag = $1 AND portal_flag = $2 AND deleted_at IS NULL",
            &[&dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let app_data: Option<String> = row.get("app_data");
            let mut entries = vec![
                ("id".to_string(), Value::String(row.get("id"))),
            ];
            if let Some(data) = app_data {
                entries.push(("data".to_string(), Value::String(data)));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter(entries),
            ))))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn dict_dictFlag_portal_portalFlag_path_data(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(portal_flag): Path<String>,
    Path(_path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, app_data FROM x_portal_dict WHERE flag = $1 AND portal_flag = $2 AND deleted_at IS NULL",
            &[&dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let app_data: Option<String> = row.get("app_data");
            let mut entries = vec![
                ("id".to_string(), Value::String(row.get("id"))),
                ("path".to_string(), Value::String(_path)),
            ];
            if let Some(data) = app_data {
                entries.push(("data".to_string(), Value::String(data)));
            }
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter(entries),
            ))))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn dict_dictFlag_portal_portalFlag_path_data_mockdeletetoget(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(portal_flag): Path<String>,
    Path(_path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, app_data FROM x_portal_dict WHERE flag = $1 AND portal_flag = $2 AND deleted_at IS NULL",
            &[&dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("data".to_string(), Value::String({
                    let __val: Option<String> = row.get("app_data");
                    __val.unwrap_or_default()
                })),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("path".to_string(), Value::String(_path)),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn dict_dictFlag_portal_portalFlag_path_data_mockputtopost(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(portal_flag): Path<String>,
    Path(_path): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let data_str = match body {
        Value::String(s) => s,
        _ => serde_json::to_string(&body).map_err(|_| AppError::Internal)?,
    };

    let result = client
        .execute(
            "UPDATE x_portal_dict SET app_data = $1, update_time = NOW() WHERE flag = $2 AND portal_flag = $3",
            &[&data_str, &dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    let row = client
        .query_one(
            "SELECT id, name, app_name, key_name, app_data, creator, update_time FROM x_portal_dict WHERE flag = $1 AND portal_flag = $2 AND deleted_at IS NULL",
            &[&dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("appName".to_string(), Value::String(row.get("app_name"))),
            ("keyName".to_string(), Value::String(row.get("key_name"))),
            ("appData".to_string(), Value::String(row.get("app_data"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("updateTime".to_string(), Value::String(row.get("update_time"))),
        ]),
    ))))
}

pub async fn file_list_portal_portalFlag(
    pool: Extension<Pool>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, file_type, creator, create_time FROM x_portal_file WHERE portal_flag = $1 ORDER BY create_time DESC",
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
            "SELECT id, name, flag, file_type, creator, create_time FROM x_portal_file WHERE flag = $1 LIMIT 1",
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

pub async fn file_flag_download(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, file_type, content FROM x_portal_file WHERE flag = $1 LIMIT 1",
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
                ("content".to_string(), Value::String(row.get("content"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_flag_portal_portalFlag_content(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content FROM x_portal_file WHERE flag = $1 AND portal_flag = $2 LIMIT 1",
            &[&flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("content".to_string(), Value::String(row.get("content"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_flag_portal_portalFlag_download(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, file_type, content FROM x_portal_file WHERE flag = $1 AND portal_flag = $2 LIMIT 1",
            &[&flag, &portal_flag],
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

pub async fn page_list_portal_portal(
    pool: Extension<Pool>,
    Path(portal): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_portal_page WHERE portal_id = $1 ORDER BY create_time DESC",
            &[&portal],
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
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn page_v2_flag_portal_portalFlag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content, creator, create_time FROM x_portal_page WHERE flag = $1 AND portal_flag = $2 LIMIT 1",
            &[&flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_v2_flag_portal_portalFlag_mobile(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mobile_content, creator, create_time FROM x_portal_page WHERE flag = $1 AND portal_flag = $2 LIMIT 1",
            &[&flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("mobile_content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_v2_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content, creator, create_time FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_v2_id_mobile(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mobile_content, creator, create_time FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("mobile_content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_flag_portal_portalFlag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content, creator, create_time FROM x_portal_page WHERE flag = $1 AND portal_id = $2 LIMIT 1",
            &[&flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_flag_portal_portalFlag_mobile(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mobile_content, creator, create_time FROM x_portal_page WHERE flag = $1 AND portal_id = $2 LIMIT 1",
            &[&flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("mobile_content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content, creator, create_time FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn page_id_mobile(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mobile_content, creator, create_time FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("content".to_string(), Value::String(row.get("mobile_content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
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

pub async fn portal_list_mobile(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, logo, creator, create_time FROM x_portal WHERE deleted_at IS NULL AND mobile_enabled = true ORDER BY create_time DESC",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn portal_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, logo, description, creator, create_time FROM x_portal WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
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
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn portal_flag_corner_mark(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, corner_mark FROM x_portal WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("cornerMark".to_string(), Value::String(row.get("corner_mark"))),
                ]),
            ))))
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
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("logo".to_string(), Value::String(row.get("logo"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn portal_id_icon_base64(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, logo_base64 FROM x_portal WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("logoBase64".to_string(), Value::String(row.get("logo_base64"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn script_list_portal_portal(
    pool: Extension<Pool>,
    Path(portal): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_portal_script WHERE portal_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal],
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

pub async fn script_portal_portal_name_name(
    pool: Extension<Pool>,
    Path(portal): Path<String>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, content, creator, create_time FROM x_portal_script WHERE portal_id = $1 AND name = $2 AND deleted_at IS NULL LIMIT 1",
            &[&portal, &name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

pub async fn script_portal_portal_name_name_imported(
    pool: Extension<Pool>,
    Path(portal): Path<String>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, imported_content, creator, create_time FROM x_portal_script WHERE portal_id = $1 AND name = $2 AND deleted_at IS NULL LIMIT 1",
            &[&portal, &name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("importedContent".to_string(), Value::String(row.get("imported_content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

pub async fn script_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, content, creator, create_time FROM x_portal_script WHERE id = $1 AND deleted_at IS NULL",
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
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

pub async fn widget_list_portal_portal(
    pool: Extension<Pool>,
    Path(portal): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, portal_id, category, config, creator, create_time FROM x_portal_widget WHERE portal_id = $1 ORDER BY create_time DESC",
            &[&portal],
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

pub async fn widget_flag_portal_portalFlag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, portal_id, category, config, creator, create_time FROM x_portal_widget WHERE flag = $1 AND portal_id = $2 LIMIT 1",
            &[&flag, &portal_flag],
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
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("widget not found"))),
    }
}

pub async fn widget_flag_portal_portalFlag_mobile(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, portal_id, mobile_config, creator, create_time FROM x_portal_widget WHERE flag = $1 AND portal_id = $2 LIMIT 1",
            &[&flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("portalId".to_string(), Value::String(row.get("portal_id"))),
                ("config".to_string(), Value::String(row.get("mobile_config"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("widget not found"))),
    }
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

pub async fn widget_id_mobile(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, portal_id, mobile_config, creator, create_time, update_time FROM x_portal_widget WHERE id = $1",
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
                ("config".to_string(), Value::String(row.get("mobile_config"))),
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
// plan002 U2: portal/dict 族缺口补建 (27 gap 中的可落地端点)
// 复用既有 x_portal_* 表，参数化真实 SQL；归一化查重 / IDOR 门禁（按 flag+portal_flag 作用域）。
// ─────────────────────────────────────────────────────────────────────────────

pub async fn dict_dictFlag_portal_portalFlag_path_data_delete(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(portal_flag): Path<String>,
    Path(_path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_portal_dict WHERE flag = $1 AND portal_flag = $2",
            &[&dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn dict_dictFlag_portal_portalFlag_path_data_post(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(portal_flag): Path<String>,
    Path(_path): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let data_str = match body {
        Value::String(s) => s,
        _ => serde_json::to_string(&body).map_err(|_| AppError::Internal)?,
    };

    let result = client
        .execute(
            "UPDATE x_portal_dict SET app_data = $1, update_time = NOW() WHERE flag = $2 AND portal_flag = $3",
            &[&data_str, &dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    let row = client
        .query_one(
            "SELECT id, name, app_name, key_name, app_data, creator, update_time FROM x_portal_dict WHERE flag = $1 AND portal_flag = $2 AND deleted_at IS NULL",
            &[&dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("appName".to_string(), Value::String(row.get("app_name"))),
            ("keyName".to_string(), Value::String(row.get("key_name"))),
            ("appData".to_string(), Value::String(row.get("app_data"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("updateTime".to_string(), Value::String(row.get("update_time"))),
        ]),
    ))))
}

pub async fn dict_dictFlag_portal_portalFlag_path_data_put(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(portal_flag): Path<String>,
    Path(_path): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let data_str = match body {
        Value::String(s) => s,
        _ => serde_json::to_string(&body).map_err(|_| AppError::Internal)?,
    };

    let updated = client
        .execute(
            "UPDATE x_portal_dict SET app_data = $1, update_time = NOW() WHERE flag = $2 AND portal_flag = $3",
            &[&data_str, &dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if updated == 0 {
        let id = uuid::Uuid::new_v4().to_string();
        let name = dict_flag.clone();
        let creator = "system";
        client
            .execute(
                "INSERT INTO x_portal_dict (id, name, flag, portal_flag, app_data, creator, create_time, update_time) \
                  VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
                &[&id, &name, &dict_flag, &portal_flag, &data_str, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        return Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("name".to_string(), Value::String(name)),
                ("flag".to_string(), Value::String(dict_flag)),
                ("portalFlag".to_string(), Value::String(portal_flag)),
                ("appData".to_string(), Value::String(data_str)),
                ("created".to_string(), Value::Bool(true)),
            ]),
        ))));
    }

    let row = client
        .query_one(
            "SELECT id, name, app_name, key_name, app_data, creator, update_time FROM x_portal_dict WHERE flag = $1 AND portal_flag = $2 AND deleted_at IS NULL",
            &[&dict_flag, &portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("appName".to_string(), Value::String(row.get("app_name"))),
            ("keyName".to_string(), Value::String(row.get("key_name"))),
            ("appData".to_string(), Value::String(row.get("app_data"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("updateTime".to_string(), Value::String(row.get("update_time"))),
        ]),
    ))))
}

pub async fn script_portal_portal_name_post(
    pool: Extension<Pool>,
    Path(portal): Path<String>,
    Path(name): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 归一化查重：同 portal 下同名 script 视为重复
    let existing = client
        .query_opt(
            "SELECT id FROM x_portal_script WHERE portal_id = $1 AND name = $2 AND deleted_at IS NULL",
            &[&portal, &name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if existing.is_some() {
        return Ok(Json(ActionResult::error("script already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let category = body
        .get("category")
        .and_then(|v| v.as_str())
        .unwrap_or("default")
        .to_string();
    let flag = body
        .get("flag")
        .and_then(|v| v.as_str())
        .unwrap_or(&name)
        .to_string();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_portal_script (id, name, flag, category, portal_id, creator, create_time, update_time) \
              VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[&id, &name, &flag, &category, &portal, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("portalId".to_string(), Value::String(portal)),
        ]),
    ))))
}
