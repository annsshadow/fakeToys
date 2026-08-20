use axum::{
    extract::{Extension, Query},
    Json,
};
use deadpool_postgres::Pool;
use deadpool_postgres::tokio_postgres::Row;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use shared::session::Session;

/// 流程平台设计器组装模块
/// 提供流程设计器相关的接口
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

/// 创建流程定义
/// 接收请求创建新的流程定义
pub async fn create_flow(
    pool: Extension<Pool>,
    session: Extension<Session>,
    axum::extract::Json(req): Json<CreateFlowRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = req.name.unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let category = req.category.unwrap_or_default();
    let description = req.description.unwrap_or_default();
    let version = 1i32;
    let creator = session.person_unique.clone();

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

/// 获取流程定义
/// 根据指定ID获取流程定义
pub async fn get_flow(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, category, process_definition, version, creator, create_time, update_time \
             FROM x_process_definition WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let mut map = serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("category".to_string(), Value::String(row.get::<_, Option<String>>("category").unwrap_or_default())),
        ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("version")))),
        ("creator".to_string(), Value::String(row.get("creator"))),
        ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ("updateTime".to_string(), Value::String(row.get::<_, Option<String>>("update_time").unwrap_or_default())),
    ]);
    if let Some(pd) = row.get::<_, Option<String>>("process_definition").and_then(|s| serde_json::from_str(&s).ok()) {
        map.insert("processDefinition".to_string(), pd);
    }
    let result = Value::Object(map);

    Ok(Json(ActionResult::success(result)))
}

/// 获取流程列表
/// 根据指定分类获取流程定义列表
pub async fn list_flows(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
    Query(params): Query<ListQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let page = params.page.unwrap_or(1).max(1);
    let size = params.size.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * size;

    let total: i64 = if category.is_empty() || category == "all" {
        client
            .query_one(
                "SELECT COUNT(*) as count FROM x_process_definition WHERE 1=1",
                &[],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .get("count")
    } else {
        client
            .query_one(
                "SELECT COUNT(*) as count FROM x_process_definition WHERE category = $1",
                &[&category],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .get("count")
    };

    let rows = if category.is_empty() || category == "all" {
        client
            .query(
                "SELECT id, name, category, version, creator, create_time FROM x_process_definition \
                 WHERE 1=1 ORDER BY create_time DESC LIMIT $1::bigint OFFSET $2::bigint",
                &[&size, &offset],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, category, version, creator, create_time FROM x_process_definition \
                 WHERE category = $1 ORDER BY create_time DESC LIMIT $2::bigint OFFSET $3::bigint",
                &[&category, &size, &offset],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

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

/// 保存流程定义
/// 根据指定ID更新流程定义到数据库
pub async fn save_flow(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let process_definition: Option<Value> = body.get("processDefinition")
        .or_else(|| body.get("process_definition"))
        .cloned();
    let process_definition_str = process_definition
        .map(|v| serde_json::to_string(&v))
        .transpose()
        .map_err(|_| AppError::Internal)?;

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
            ("saved".to_string(), Value::Bool(result > 0)),
            ("updatedAt".to_string(), Value::String(chrono::Utc::now().to_rfc3339())),
        ]),
    ))))
}

/// 删除流程定义
/// 根据ID删除指定的流程定义
pub async fn delete_flow(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
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
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

/// 预览流程定义
/// 返回流程定义的预览信息
pub async fn preview_flow(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, process_definition FROM x_process_definition WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let process_definition: Option<String> = row.get("process_definition");
            let nodes = process_definition
                .as_ref()
                .and_then(|s| serde_json::from_str::<Value>(s).ok())
                .map(|v| v.get("nodes").cloned().unwrap_or(Value::Array(vec![])))
                .unwrap_or(Value::Array(vec![]));
            let edges = process_definition
                .as_ref()
                .and_then(|s| serde_json::from_str::<Value>(s).ok())
                .map(|v| v.get("edges").cloned().unwrap_or(Value::Array(vec![])))
                .unwrap_or(Value::Array(vec![]));

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("preview_url".to_string(), Value::String(format!("/preview/flow/{}", id))),
                    ("nodes".to_string(), nodes),
                    ("edges".to_string(), edges),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("process definition not found"))),
    }
}

/// 流程平台设计器组装路由
/// 路由前缀: /jaxrs/processplatform/assemble/designer/*
pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}

// ──────────────────────────────────────────────────────────────────────────────
// Helper: generic row_to_data for PP_E_* tables
// ──────────────────────────────────────────────────────────────────────────────
fn row_to_app_data(row: &Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("xid"))),
        ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
        ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
    ]))
}

fn row_to_basic_data(row: &Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("xid"))),
        ("name".to_string(), Value::String(row.get("xname"))),
        ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
        ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
    ]))
}

fn row_to_form_data(row: &Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("xid"))),
        ("name".to_string(), Value::String(row.get("xname"))),
        ("application".to_string(), Value::String(row.get::<_, Option<String>>("xapplication").unwrap_or_default())),
        ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
        ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
    ]))
}

fn row_to_process_data(row: &Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("xid"))),
        ("name".to_string(), Value::String(row.get("xname"))),
        ("application".to_string(), Value::String(row.get::<_, Option<String>>("xapplication").unwrap_or_default())),
        ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
        ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
    ]))
}

// ──────────────────────────────────────────────────────────────────────────────
// application 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn application_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplicationCategory, xicon, xiconHue, xcreatorPerson, xlastUpdateTime, xlastUpdatePerson, xproperties, \"\"xcreateTime\"\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn application_list_applicationcategory_applicationCategory(
    pool: Extension<Pool>,
    axum::extract::Path(applicationCategory): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE \"xapplicationCategory\" = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationCategory],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn application_list_summary(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(0.into())),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn application_list_summary_applicationcategory_applicationCategory(
    pool: Extension<Pool>,
    axum::extract::Path(applicationCategory): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(0.into())),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn application_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_app_data(&row)))),
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn application_id_icon(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xicon FROM PP_E_APPLICATION WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let icon: Option<String> = row.get("xicon");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("icon".to_string(), Value::String(icon.unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn application_id_permission(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xproperties FROM PP_E_APPLICATION WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let props: Option<String> = row.get("xproperties");
            let permissions: Value = props
                .as_ref()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(Value::Object(serde_json::Map::new()));
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("permissions".to_string(), permissions),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn application_id_onlyRemoveNotCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_app_data(&row)))),
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// applicationcategory 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn applicationcategory_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xdescription, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONCATEGORY WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_basic_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

// ──────────────────────────────────────────────────────────────────────────────
// applicationdict 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn applicationdict_list_application_applicationId(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn applicationdict_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let size = size.clamp(1, 100);
    let offset = (page - 1).max(0) * size;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
    ])))))
}

pub async fn applicationdict_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_app_data(&row)))),
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// elementtool 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn elementtool_applicationdict_orphan(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT d.xid, d.xname, d.xapplication, d.\"xapplicationName\", d.\"xcreatorPerson\", d.\"xcreateTime\", d.\"xupdateTime\" FROM PP_E_APPLICATIONDICT d LEFT JOIN PP_E_APPLICATION a ON d.xapplication = a.xid WHERE a.xid IS NULL ORDER BY d.\"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn elementtool_form_orphan(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT f.xid, f.xname, f.xapplication, f.\"xapplicationName\", f.\"xcreatorPerson\", f.\"xcreateTime\", f.\"xupdateTime\" FROM PP_E_FORM f LEFT JOIN PP_E_APPLICATION a ON f.xapplication = a.xid WHERE a.xid IS NULL ORDER BY f.\"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_form_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn elementtool_process_orphan(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT p.xid, p.xname, p.xapplication, p.\"xapplicationName\", p.\"xcreatorPerson\", p.\"xcreateTime\", p.\"xupdateTime\" FROM PP_E_PROCESS p LEFT JOIN PP_E_APPLICATION a ON p.xapplication = a.xid WHERE a.xid IS NULL ORDER BY p.\"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_process_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn elementtool_script_orphan(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT s.xid, s.xname, s.xapplication, s.\"xapplicationName\", s.\"xcreatorPerson\", s.\"xcreateTime\", s.\"xupdateTime\" FROM PP_E_SCRIPT s LEFT JOIN PP_E_APPLICATION a ON s.xapplication = a.xid WHERE a.xid IS NULL ORDER BY s.\"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

// ──────────────────────────────────────────────────────────────────────────────
// file 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn file_list_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FILE WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn file_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_FILE WHERE xid > $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn file_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_FILE WHERE xid < $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn file_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FILE WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_flag_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FILE WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FILE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id_content(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xcontent, \"xcreateTime\", \"xupdateTime\" FROM PP_E_FILE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("xcontent");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("content".to_string(), Value::String(content.unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id_download(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xcontent, \"xcreateTime\", \"xupdateTime\" FROM PP_E_FILE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("xcontent");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("downloadUrl".to_string(), Value::String(format!("/file/download/{}", id))),
                    ("content".to_string(), Value::String(content.unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id_upload(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let result = client
        .execute(
            "UPDATE PP_E_FILE SET xcontent = $1, \"xupdateTime\" = NOW() WHERE xid = $2",
            &[&content, &id],
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

// ──────────────────────────────────────────────────────────────────────────────
// form 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn form_list_application_applicationId(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_form_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn form_list_formfield_application_applicationId(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_form_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn form_list_id_formfield(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_form_data(&row)))),
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_FORM WHERE xid > $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn form_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_FORM WHERE xid < $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn form_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_form_data(&row)))),
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn formversion_list_form_formId(
    pool: Extension<Pool>,
    axum::extract::Path(formId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xform, xname, xcontent, xversion, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORMVERSION WHERE xform = $1 ORDER BY xversion DESC",
            &[&formId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("form".to_string(), Value::String(row.get("xform"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("xversion")))),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn formversion_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xform, xname, xcontent, xversion, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORMVERSION WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("xcontent");
            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("form".to_string(), Value::String(row.get("xform"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
                ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("xversion")))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]);
            if let Some(c) = content.as_ref().and_then(|s| serde_json::from_str(s).ok()) {
                map.insert("content".to_string(), c);
            }
            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Ok(Json(ActionResult::error("formversion not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// id_count
// ──────────────────────────────────────────────────────────────────────────────

pub async fn id_count(
    pool: Extension<Pool>,
    axum::extract::Path(entity): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let table = match entity.as_str() {
        "application" => "PP_E_APPLICATION",
        "form" => "PP_E_FORM",
        "process" => "PP_E_PROCESS",
        "script" => "PP_E_SCRIPT",
        _ => "PP_E_APPLICATION",
    };
    let count: i64 = client
        .query_one(
            format!("SELECT COUNT(*) FROM {}", table).as_str(),
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
            ("entity".to_string(), Value::String(entity)),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// input 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn input_compare(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("matched".to_string(), Value::Bool(false)),
        ]),
    ))))
}

pub async fn input_cover(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(0.into())),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn input_create(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(0.into())),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn input_prepare_cover(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(0.into())),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn input_prepare_create(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(0.into())),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

// ──────────────────────────────────────────────────────────────────────────────
// item_access 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn item_access_bach_save(
    pool: Extension<Pool>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let items: Vec<Value> = body.get("items").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let count = items.len();
    for item in &items {
        let id = item.get("id").and_then(|v| v.as_str()).unwrap_or("");
        if !id.is_empty() {
            let _ = client
                .execute(
                    "UPDATE PP_E_ITEM_ACCESS SET \"xupdateTime\" = NOW() WHERE xid = $1",
                    &[&id],
                )
                .await;
        }
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(count > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn item_access_delete_process_processId_path_path(
    pool: Extension<Pool>,
    axum::extract::Path(processId): axum::extract::Path<String>,
    axum::extract::Path(path): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_ITEM_ACCESS WHERE xprocess = $1 AND xpath = $2",
            &[&processId, &path],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(result > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(result))),
        ]),
    ))))
}

pub async fn item_access_path_path(
    pool: Extension<Pool>,
    axum::extract::Path(path): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xprocess, xpath, \"xcreateTime\", \"xupdateTime\" FROM PP_E_ITEM_ACCESS WHERE xpath = $1 ORDER BY \"xcreateTime\" DESC",
            &[&path],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("process".to_string(), Value::String(row.get("xprocess"))),
            ("path".to_string(), Value::String(row.get("xpath"))),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn item_access_process_processId(
    pool: Extension<Pool>,
    axum::extract::Path(processId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xprocess, xpath, \"xcreateTime\", \"xupdateTime\" FROM PP_E_ITEM_ACCESS WHERE xprocess = $1 ORDER BY \"xcreateTime\" DESC",
            &[&processId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("process".to_string(), Value::String(row.get("xprocess"))),
            ("path".to_string(), Value::String(row.get("xpath"))),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn item_access_process_processId_path_path(
    pool: Extension<Pool>,
    axum::extract::Path(processId): axum::extract::Path<String>,
    axum::extract::Path(path): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xprocess, xpath, \"xcreateTime\", \"xupdateTime\" FROM PP_E_ITEM_ACCESS WHERE xprocess = $1 AND xpath = $2 ORDER BY \"xcreateTime\" DESC",
            &[&processId, &path],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("process".to_string(), Value::String(row.get("xprocess"))),
            ("path".to_string(), Value::String(row.get("xpath"))),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn item_access_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xprocess, xpath, \"xcreateTime\", \"xupdateTime\" FROM PP_E_ITEM_ACCESS WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
                ("process".to_string(), Value::String(row.get("xprocess"))),
                ("path".to_string(), Value::String(row.get("xpath"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("item_access not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// mapping 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn mapping_list_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xsource, xtarget, \"xcreateTime\", \"xupdateTime\" FROM PP_E_MAPPING WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("application".to_string(), Value::String(row.get::<_, Option<String>>("xapplication").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn mapping_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_MAPPING WHERE xid > $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn mapping_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_MAPPING WHERE xid < $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn mapping_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xsource, xtarget, \"xcreateTime\", \"xupdateTime\" FROM PP_E_MAPPING WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
                ("application".to_string(), Value::String(row.get::<_, Option<String>>("xapplication").unwrap_or_default())),
                ("source".to_string(), Value::String(row.get::<_, Option<String>>("xsource").unwrap_or_default())),
                ("target".to_string(), Value::String(row.get::<_, Option<String>>("xtarget").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("mapping not found"))),
    }
}

pub async fn mapping_flag_execute(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xsource, xtarget, \"xcreateTime\", \"xupdateTime\" FROM PP_E_MAPPING WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let found = row.is_some();
    match row {
        Some(row) => {
            let source: String = row.get("xsource");
            let target: String = row.get("xtarget");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("execute".to_string(), Value::Bool(found)),
                    ("source".to_string(), Value::String(source)),
                    ("target".to_string(), Value::String(target)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("mapping not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// mergeitemplan 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn mergeitemplan_estimate(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_MERGEITEMPLAN",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("estimated".to_string(), Value::Bool(count > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn mergeitemplan_list_application_applicationId_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let size = size.clamp(1, 100);
    let offset = (page - 1).max(0) * size;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_MERGEITEMPLAN WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC LIMIT $2::bigint OFFSET $3::bigint",
            &[&applicationId, &size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
    ])))))
}

pub async fn mergeitemplan_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let size = size.clamp(1, 100);
    let offset = (page - 1).max(0) * size;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_MERGEITEMPLAN WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
    ])))))
}

pub async fn mergeitemplan_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_MERGEITEMPLAN WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_app_data(&row)))),
        None => Ok(Json(ActionResult::error("mergeitemplan not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// output 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn output_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xprocess, xoutput, \"xcreateTime\", \"xupdateTime\" FROM PP_E_OUTPUT WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("process".to_string(), Value::String(row.get("xprocess"))),
            ("output".to_string(), Value::String(row.get::<_, Option<String>>("xoutput").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn output_applicationFlag_select(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xprocess, xoutput, \"xcreateTime\", \"xupdateTime\" FROM PP_E_OUTPUT WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("process".to_string(), Value::String(row.get("xprocess"))),
            ("output".to_string(), Value::String(row.get::<_, Option<String>>("xoutput").unwrap_or_default())),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

// ──────────────────────────────────────────────────────────────────────────────
// process_activity 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn process_activity_flag_activityType_activityType(
    pool: Extension<Pool>,
    axum::extract::Path(activityType): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, \"xactivityType\", xdescription, \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS_ACTIVITY WHERE xactivitytype = $1 ORDER BY \"xcreateTime\" DESC",
            &[&activityType],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get("xname"))),
            ("activityType".to_string(), Value::String(row.get("\"xactivityType\""))),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

// ──────────────────────────────────────────────────────────────────────────────
// process 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn process_application_applicationId(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_process_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn process_application_applicationId_disable_edition(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xapplication = $1 AND xstatus = 'disabled' ORDER BY \"xcreateTime\" DESC",
            &[&applicationId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get("xname"))),
            ("edition".to_string(), Value::String("disabled".to_string())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn process_application_applicationId_edition_edition(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
    axum::extract::Path(edition): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xapplication = $1 AND xedition = $2 ORDER BY \"xcreateTime\" DESC",
            &[&applicationId, &edition],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get("xname"))),
            ("edition".to_string(), Value::String(edition.clone())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn process_form_formId(
    pool: Extension<Pool>,
    axum::extract::Path(formId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xformId, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xformid = $1",
            &[&formId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_process_data(&row)))),
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_upgrade_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_PROCESS SET \"xupdateTime\" = NOW(), xversion = xversion + 1 WHERE 1=1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("upgraded".to_string(), Value::Bool(result > 0)),
            ("count".to_string(), Value::Number(serde_json::Number::from(result))),
        ]),
    ))))
}

pub async fn process_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_process_data(&row)))),
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_id_disable(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_PROCESS SET xstatus = 'disabled', \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("process not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("disabled".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn process_id_enable(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_PROCESS SET xstatus = 'enabled', \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("process not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("enabled".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn process_id_enabled(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, xstatus FROM PP_E_PROCESS WHERE xid = $1 AND xstatus = 'enabled'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("name".to_string(), Value::String(row.get("xname"))),
                ("enabled".to_string(), Value::Bool(row.get::<_, Option<String>>("xstatus").map(|s| s == "enabled").unwrap_or(false))),
            ]),
        )))),
        None => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("enabled".to_string(), Value::Bool(false)),
            ]),
        )))),
    }
}

pub async fn process_id_execute_projection(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, \"xserialTexture\", \"xserialActivity\", \"xserialPhase\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let texture: Option<String> = row.get("\"xserialTexture\"");
            let activity: Option<String> = row.get("\"xserialActivity\"");
            let phase: Option<String> = row.get("\"xserialPhase\"");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("projection".to_string(), Value::Object(serde_json::Map::from_iter([
                        ("texture".to_string(), Value::String(texture.unwrap_or_default())),
                        ("activity".to_string(), Value::String(activity.unwrap_or_default())),
                        ("phase".to_string(), Value::String(phase.unwrap_or_default())),
                    ]))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_id_lead_out(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let process_def: Option<String> = row.get("xname");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("exportUrl".to_string(), Value::String(format!("/export/process/{}", id))),
                    ("name".to_string(), Value::String(row.get("xname"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_id_list_element(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, \"xprocessId\", \"xelementType\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS_ELEMENT WHERE xprocessid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get("xname"))),
            ("processId".to_string(), Value::String(row.get("\"xprocessId\""))),
            ("elementType".to_string(), Value::String(row.get::<_, Option<String>>("\"xelementType\"").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn process_id_permission(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xproperties FROM PP_E_PROCESS WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let props: Option<String> = row.get("xproperties");
            let permissions: Value = props
                .as_ref()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or(Value::Object(serde_json::Map::new()));
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("permissions".to_string(), permissions),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_id_process(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_process_data(&row)))),
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_id_upgrade(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_PROCESS SET \"xupdateTime\" = NOW(), xversion = COALESCE(xversion, 0) + 1 WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("process not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("upgraded".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn process_id_onlyRemoveNotCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_process_data(&row)))),
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_id_onlyRemoveNotCompleted_edition(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, xedition, \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let edition: Option<String> = row.get("xedition");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("name".to_string(), Value::String(row.get("xname"))),
                    ("edition".to_string(), Value::String(edition.unwrap_or_default())),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// processversion 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn processversion_list_process_processId(
    pool: Extension<Pool>,
    axum::extract::Path(processId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xprocess, xname, xcontent, xversion, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESSVERSION WHERE xprocess = $1 ORDER BY xversion DESC",
            &[&processId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("process".to_string(), Value::String(row.get("xprocess"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("xversion")))),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn processversion_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xprocess, xname, xcontent, xversion, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESSVERSION WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("xcontent");
            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("process".to_string(), Value::String(row.get("xprocess"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
                ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("xversion")))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]);
            if let Some(c) = content.as_ref().and_then(|s| serde_json::from_str(s).ok()) {
                map.insert("content".to_string(), c);
            }
            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Ok(Json(ActionResult::error("processversion not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// script 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn script_application_applicationId(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_SCRIPT WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn script_application_applicationId_name_name(
    pool: Extension<Pool>,
    axum::extract::Path(applicationId): axum::extract::Path<String>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_SCRIPT WHERE xapplication = $1 AND xname = $2 ORDER BY \"xcreateTime\" DESC",
            &[&applicationId, &name],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
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
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_SCRIPT WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn script_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let size = size.clamp(1, 100);
    let offset = (page - 1).max(0) * size;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_SCRIPT WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| row_to_app_data(row))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
    ])))))
}

pub async fn script_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_SCRIPT WHERE xid > $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn script_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_E_SCRIPT WHERE xid < $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn script_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xcode, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_SCRIPT WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(row_to_app_data(&row)))),
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

pub async fn scriptversion_list_script_scriptId(
    pool: Extension<Pool>,
    axum::extract::Path(scriptId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xscript, xname, xcode, xversion, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_SCRIPTVERSION WHERE xscript = $1 ORDER BY xversion DESC",
            &[&scriptId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("script".to_string(), Value::String(row.get("xscript"))),
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
            ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("xversion")))),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn scriptversion_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xscript, xname, xcode, xversion, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_SCRIPTVERSION WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let code: Option<String> = row.get("xcode");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("xid"))),
                    ("script".to_string(), Value::String(row.get("xscript"))),
                    ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
                    ("version".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("xversion")))),
                    ("code".to_string(), Value::String(code.unwrap_or_default())),
                    ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                    ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("scriptversion not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// templateform 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn templateform_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xcategory, xcontent, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_TEMPLATEFORM WHERE 1=1 ORDER BY xname",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get("xname"))),
            ("category".to_string(), Value::String(row.get::<_, Option<String>>("xcategory").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn templateform_list_category(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xcategory, xcontent, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_TEMPLATEFORM WHERE xcategory = $1 ORDER BY xname",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("name".to_string(), Value::String(row.get("xname"))),
            ("category".to_string(), Value::String(row.get::<_, Option<String>>("xcategory").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
            ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn templateform_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xcategory, xcontent, \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_TEMPLATEFORM WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("xcontent");
            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("name".to_string(), Value::String(row.get("xname"))),
                ("category".to_string(), Value::String(row.get::<_, Option<String>>("xcategory").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]);
            if let Some(c) = content.as_ref().and_then(|s| serde_json::from_str(s).ok()) {
                map.insert("content".to_string(), c);
            }
            Ok(Json(ActionResult::success(Value::Object(map))))
        }
        None => Ok(Json(ActionResult::error("templateform not found"))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// workcompleted 管理
// ──────────────────────────────────────────────────────────────────────────────

pub async fn workcompleted_application_applicationFlag_merge_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, \"xworkId\", \"xcompletedTime\", xcreator, \"xcreateTime\" FROM PP_E_WORKCOMPLETED wc JOIN PP_E_PROCESS p ON wc.xworkid = p.xid WHERE p.xapplication = $1 ORDER BY wc.\"xcompletedTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("workId".to_string(), Value::String(row.get("\"xworkId\""))),
            ("\"completedTime\"".to_string(), Value::String(row.get("\"xcompletedTime\""))),
            ("creator".to_string(), Value::String(row.get::<_, Option<String>>("xcreator").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn workcompleted_process_processFlag_merge_data(
    pool: Extension<Pool>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, \"xworkId\", \"xcompletedTime\", xcreator, \"xcreateTime\" FROM PP_E_WORKCOMPLETED wc JOIN PP_E_PROCESS p ON wc.xworkid = p.xid WHERE p.xid = $1 ORDER BY wc.\"xcompletedTime\" DESC",
            &[&processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("xid"))),
            ("workId".to_string(), Value::String(row.get("\"xworkId\""))),
            ("\"completedTime\"".to_string(), Value::String(row.get("\"xcompletedTime\""))),
            ("creator".to_string(), Value::String(row.get::<_, Option<String>>("xcreator").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
        ])))
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

#[cfg(test)]
mod tests_generated;
