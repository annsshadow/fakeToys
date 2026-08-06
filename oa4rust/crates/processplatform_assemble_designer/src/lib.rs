use axum::{
    extract::{Extension, Query},
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

/// 流程平台设计器装配模块
/// 提供流程设计器相关的装配服务
pub mod routes;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFlowRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
}

/// 创建流程设计
/// 根据请求创建新的流程设计
pub async fn create_flow(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): Json<CreateFlowRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = req.name.unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let category = req.category.unwrap_or_default();
    let description = req.description.unwrap_or_default();
    let version = 1i32;
    let creator = "system".to_string();

    client
        .execute(
            "INSERT INTO x_process_definition (id, name, category, process_definition, version, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4::jsonb, $5, $6, NOW(), NOW())",
            &[&id, &name, &category, &description, &version, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 获取流程设计
/// 返回指定ID的流程设计详情
pub async fn get_flow(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_one(
            "SELECT id, name, category, process_definition, version, creator, create_time, update_time \
             FROM x_process_definition WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("category".to_string(), Value::String(row.get::<_, Option<String>>("category").unwrap_or_default())),
        ("processDefinition".to_string(), {
            let pd: Option<String> = row.get("process_definition");
            pd.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
        }),
        ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("version")))),
        ("creator".to_string(), Value::String(row.get("creator"))),
        ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ("updateTime".to_string(), Value::String(row.get::<_, Option<String>>("update_time").unwrap_or_default())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 列出流程设计
/// 返回指定类别下的所有流程设计列表
pub async fn list_flows(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(category): axum::extract::Path<String>,
    Query(params): Query<ListQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let page = params.page.unwrap_or(1).max(1);
    let size = params.size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * size;

    let category_filter = if category == "all" || category.is_empty() {
        "".to_string()
    } else {
        format!("AND category = '{}'", category.replace("'", "''"))
    };

    let total: i64 = client
        .query_one(
            &format!("SELECT COUNT(*) as count FROM x_process_definition WHERE 1=1 {}", category_filter),
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let rows = client
        .query(
            &format!(
                "SELECT id, name, category, version, creator, create_time FROM x_process_definition \
                 WHERE 1=1 {} ORDER BY create_time DESC LIMIT $1 OFFSET $2",
                category_filter
            ),
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get::<_, Option<String>>("category").unwrap_or_default())),
                ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("version")))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(total))),
        ("size".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// 保存流程设计
/// 保存指定的流程设计到数据库
pub async fn save_flow(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let process_definition = body.get("processDefinition")
        .or_else(|| body.get("process_definition"))
        .cloned()
        .unwrap_or(Value::Null);

    let process_definition_str = serde_json::to_string(&process_definition).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_process_definition SET process_definition = $1::jsonb, update_time = NOW() WHERE id = $2",
            &[&process_definition_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("process definition not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("updatedAt".to_string(), Value::String(chrono::Utc::now().to_rfc3339())),
        ]),
    ))))
}

/// 删除流程设计
/// 根据ID删除指定的流程设计
pub async fn delete_flow(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "DELETE FROM x_process_definition WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("process definition not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 预览流程设计
/// 返回流程设计的预览信息
pub async fn preview_flow(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("preview_url".to_string(), Value::String(format!("/preview/flow/{}", id))),
            ("nodes".to_string(), Value::Array(vec![])),
            ("edges".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// 流程平台设计器装配路由
/// 路由前缀: /jaxrs/processplatform/assemble/designer/*
pub fn processplatform_assemble_designer_router() -> Router {
    Router::new()
        .route("/jaxrs/processplatform/assemble/designer/create", post(create_flow))
        .route("/jaxrs/processplatform/assemble/designer/get/{id}", get(get_flow))
        .route("/jaxrs/processplatform/assemble/designer/list/{category}", get(list_flows))
        .route("/jaxrs/processplatform/assemble/designer/save/{id}", post(save_flow))
        .route("/jaxrs/processplatform/assemble/designer/delete/{id}", post(delete_flow))
        .route("/jaxrs/processplatform/assemble/designer/preview/{id}", get(preview_flow))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}


/// Stub handler for /jaxrs/processplatform/assemble/designer/application/list
pub async fn stub_processplatform_assemble_designer_application_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/application/list/applicationcategory/{applicationCategory}
pub async fn stub_processplatform_assemble_designer_application_list_applicationcategory_applicationCategory() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/application/list/summary
pub async fn stub_processplatform_assemble_designer_application_list_summary() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/application/list/summary/applicationcategory/{applicationCategory}
pub async fn stub_processplatform_assemble_designer_application_list_summary_applicationcategory_applicationCategory() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/application/{id}
pub async fn stub_processplatform_assemble_designer_application_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/application/{id}/icon
pub async fn stub_processplatform_assemble_designer_application_id_icon() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/application/{id}/permission
pub async fn stub_processplatform_assemble_designer_application_id_permission() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/application/{id}/{onlyRemoveNotCompleted}
pub async fn stub_processplatform_assemble_designer_application_id_onlyRemoveNotCompleted() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/applicationcategory/list
pub async fn stub_processplatform_assemble_designer_applicationcategory_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/applicationdict/list/application/{applicationId}
pub async fn stub_processplatform_assemble_designer_applicationdict_list_application_applicationId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/applicationdict/list/paging/{page}/size/{size}
pub async fn stub_processplatform_assemble_designer_applicationdict_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/applicationdict/{id}
pub async fn stub_processplatform_assemble_designer_applicationdict_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/elementtool/applicationdict/orphan
pub async fn stub_processplatform_assemble_designer_elementtool_applicationdict_orphan() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/elementtool/form/orphan
pub async fn stub_processplatform_assemble_designer_elementtool_form_orphan() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/elementtool/process/orphan
pub async fn stub_processplatform_assemble_designer_elementtool_process_orphan() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/elementtool/script/orphan
pub async fn stub_processplatform_assemble_designer_elementtool_script_orphan() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/list/application/{applicationFlag}
pub async fn stub_processplatform_assemble_designer_file_list_application_applicationFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/list/{id}/next/{count}
pub async fn stub_processplatform_assemble_designer_file_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/list/{id}/prev/{count}
pub async fn stub_processplatform_assemble_designer_file_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/{flag}
pub async fn stub_processplatform_assemble_designer_file_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/{flag}/application/{applicationFlag}
pub async fn stub_processplatform_assemble_designer_file_flag_application_applicationFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/{id}
pub async fn stub_processplatform_assemble_designer_file_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/{id}/content
pub async fn stub_processplatform_assemble_designer_file_id_content() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/{id}/download
pub async fn stub_processplatform_assemble_designer_file_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/file/{id}/upload
pub async fn stub_processplatform_assemble_designer_file_id_upload() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/form/list/application/{applicationId}
pub async fn stub_processplatform_assemble_designer_form_list_application_applicationId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/form/list/formfield/application/{applicationId}
pub async fn stub_processplatform_assemble_designer_form_list_formfield_application_applicationId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/form/list/{id}/formfield
pub async fn stub_processplatform_assemble_designer_form_list_id_formfield() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/form/list/{id}/next/{count}
pub async fn stub_processplatform_assemble_designer_form_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/form/list/{id}/prev/{count}
pub async fn stub_processplatform_assemble_designer_form_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/form/{id}
pub async fn stub_processplatform_assemble_designer_form_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/formversion/list/form/{formId}
pub async fn stub_processplatform_assemble_designer_formversion_list_form_formId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/formversion/{id}
pub async fn stub_processplatform_assemble_designer_formversion_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/id/{count}
pub async fn stub_processplatform_assemble_designer_id_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/input/compare
pub async fn stub_processplatform_assemble_designer_input_compare() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/input/cover
pub async fn stub_processplatform_assemble_designer_input_cover() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/input/create
pub async fn stub_processplatform_assemble_designer_input_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/input/prepare/cover
pub async fn stub_processplatform_assemble_designer_input_prepare_cover() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/input/prepare/create
pub async fn stub_processplatform_assemble_designer_input_prepare_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/item-access/bach/save
pub async fn stub_processplatform_assemble_designer_item_access_bach_save() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/item-access/delete/process/{processId}/path/{path}
pub async fn stub_processplatform_assemble_designer_item_access_delete_process_processId_path_path() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/item-access/path/{path}
pub async fn stub_processplatform_assemble_designer_item_access_path_path() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/item-access/process/{processId}
pub async fn stub_processplatform_assemble_designer_item_access_process_processId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/item-access/process/{processId}/path/{path}
pub async fn stub_processplatform_assemble_designer_item_access_process_processId_path_path() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/item-access/{id}
pub async fn stub_processplatform_assemble_designer_item_access_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mapping/list/application/{applicationFlag}
pub async fn stub_processplatform_assemble_designer_mapping_list_application_applicationFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mapping/list/{id}/next/{count}
pub async fn stub_processplatform_assemble_designer_mapping_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mapping/list/{id}/prev/{count}
pub async fn stub_processplatform_assemble_designer_mapping_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mapping/{flag}
pub async fn stub_processplatform_assemble_designer_mapping_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mapping/{flag}/execute
pub async fn stub_processplatform_assemble_designer_mapping_flag_execute() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mergeitemplan/estimate
pub async fn stub_processplatform_assemble_designer_mergeitemplan_estimate() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mergeitemplan/list/application/{applicationId}/paging/{page}/size/{size}
pub async fn stub_processplatform_assemble_designer_mergeitemplan_list_application_applicationId_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mergeitemplan/list/paging/{page}/size/{size}
pub async fn stub_processplatform_assemble_designer_mergeitemplan_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/mergeitemplan/{id}
pub async fn stub_processplatform_assemble_designer_mergeitemplan_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/output/list
pub async fn stub_processplatform_assemble_designer_output_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/output/{applicationFlag}/select
pub async fn stub_processplatform_assemble_designer_output_applicationFlag_select() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/activity/{flag}/activityType/{activityType}
pub async fn stub_processplatform_assemble_designer_process_activity_flag_activityType_activityType() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/application/{applicationId}
pub async fn stub_processplatform_assemble_designer_process_application_applicationId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/application/{applicationId}/disable/edition
pub async fn stub_processplatform_assemble_designer_process_application_applicationId_disable_edition() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/application/{applicationId}/edition/{edition}
pub async fn stub_processplatform_assemble_designer_process_application_applicationId_edition_edition() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/form/{formId}
pub async fn stub_processplatform_assemble_designer_process_form_formId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/upgrade/all
pub async fn stub_processplatform_assemble_designer_process_upgrade_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}
pub async fn stub_processplatform_assemble_designer_process_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/disable
pub async fn stub_processplatform_assemble_designer_process_id_disable() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/enable
pub async fn stub_processplatform_assemble_designer_process_id_enable() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/enabled
pub async fn stub_processplatform_assemble_designer_process_id_enabled() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/execute/projection
pub async fn stub_processplatform_assemble_designer_process_id_execute_projection() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/lead/out
pub async fn stub_processplatform_assemble_designer_process_id_lead_out() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/list/element
pub async fn stub_processplatform_assemble_designer_process_id_list_element() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/permission
pub async fn stub_processplatform_assemble_designer_process_id_permission() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/process
pub async fn stub_processplatform_assemble_designer_process_id_process() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/upgrade
pub async fn stub_processplatform_assemble_designer_process_id_upgrade() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/{onlyRemoveNotCompleted}
pub async fn stub_processplatform_assemble_designer_process_id_onlyRemoveNotCompleted() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/process/{id}/{onlyRemoveNotCompleted}/edition
pub async fn stub_processplatform_assemble_designer_process_id_onlyRemoveNotCompleted_edition() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/processversion/list/process/{processId}
pub async fn stub_processplatform_assemble_designer_processversion_list_process_processId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/processversion/{id}
pub async fn stub_processplatform_assemble_designer_processversion_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/script/application/{applicationId}
pub async fn stub_processplatform_assemble_designer_script_application_applicationId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/script/application/{applicationId}/name/{name}
pub async fn stub_processplatform_assemble_designer_script_application_applicationId_name_name() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/script/list/manager
pub async fn stub_processplatform_assemble_designer_script_list_manager() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/script/list/paging/{page}/size/{size}
pub async fn stub_processplatform_assemble_designer_script_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/script/list/{id}/next/{count}
pub async fn stub_processplatform_assemble_designer_script_list_id_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/script/list/{id}/prev/{count}
pub async fn stub_processplatform_assemble_designer_script_list_id_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/script/{id}
pub async fn stub_processplatform_assemble_designer_script_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/scriptversion/list/script/{scriptId}
pub async fn stub_processplatform_assemble_designer_scriptversion_list_script_scriptId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/scriptversion/{id}
pub async fn stub_processplatform_assemble_designer_scriptversion_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/templateform/list
pub async fn stub_processplatform_assemble_designer_templateform_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/templateform/list/category
pub async fn stub_processplatform_assemble_designer_templateform_list_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/templateform/{id}
pub async fn stub_processplatform_assemble_designer_templateform_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/workcompleted/application/{applicationFlag}/merge/data
pub async fn stub_processplatform_assemble_designer_workcompleted_application_applicationFlag_merge_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/processplatform/assemble/designer/workcompleted/process/{processFlag}/merge/data
pub async fn stub_processplatform_assemble_designer_workcompleted_process_processFlag_merge_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}
