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
pub struct CreateSurfaceRequest {
    pub name: Option<String>,
    pub template: Option<String>,
    pub category: Option<String>,
    pub content: Option<Value>,
}

pub async fn get_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, category, content, version, creator, create_time, update_time \
             FROM x_process_surface WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("category".to_string(), Value::String(row.get("category"))),
        ("content".to_string(), {
            let content_str: Option<String> = row.get("content");
            content_str.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
        }),
        ("version".to_string(), Value::String(row.get("version"))),
        ("creator".to_string(), Value::String(row.get("creator"))),
        ("createTime".to_string(), Value::String(row.get("create_time"))),
        ("updateTime".to_string(), Value::String(row.get("update_time"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn create_surface(
    pool: Extension<Pool>,
    req: axum::extract::Json<CreateSurfaceRequest>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let name = req.name.unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let category = req.category.unwrap_or_else(|| "processplatform".to_string());
    let content = req.content.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    let version = "1.0".to_string();
    let creator = "system".to_string();

    let content_str = serde_json::to_string(&content).map_err(|_| AppError::Internal)?;

    client
        .execute(
            "INSERT INTO x_process_surface (id, name, category, content, version, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4::jsonb, $5, $6, NOW(), NOW())",
            &[&id, &name, &category, &content_str, &version, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("category".to_string(), Value::String(category)),
        ("version".to_string(), Value::String(version)),
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
            "SELECT id, name, category, version, creator, create_time, update_time \
             FROM x_process_surface WHERE category = $1 ORDER BY create_time DESC",
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
                ("version".to_string(), Value::String(row.get("version"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn preview_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, content FROM x_process_surface WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content_str: Option<String> = row.get("content");
            let html = content_str
                .and_then(|s| serde_json::from_str::<Value>(&s).ok())
                .map(|v| v.get("html").cloned().unwrap_or(Value::String("<div>Process Platform Preview</div>".to_string())))
                .unwrap_or(Value::String("<div>Process Platform Preview</div>".to_string()));

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("preview_url".to_string(), Value::String(format!("/preview/{}", id))),
                    ("html".to_string(), html),
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
            "UPDATE x_process_surface SET version = $1, update_time = NOW() WHERE id = $2",
            &[&"published", &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("surface not found")));
    }

    let row = client
        .query_one(
            "SELECT id, name, category, content, version, creator, create_time, update_time \
             FROM x_process_surface WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("category".to_string(), Value::String(row.get("category"))),
        ("content".to_string(), {
            let content_str: Option<String> = row.get("content");
            content_str.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
        }),
        ("version".to_string(), Value::String(row.get("version"))),
        ("creator".to_string(), Value::String(row.get("creator"))),
        ("createTime".to_string(), Value::String(row.get("create_time"))),
        ("updateTime".to_string(), Value::String(row.get("update_time"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn delete_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, category, version, creator, create_time, update_time FROM x_process_surface WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_process_surface WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("surface not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("surface not found"))),
    }
}

pub async fn save_surface(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(content): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let content_str = serde_json::to_string(&content).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_process_surface SET content = $1::jsonb, update_time = NOW() WHERE id = $2",
            &[&content_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("surface not found")));
    }

    let row = client
        .query_one(
            "SELECT id, name, category, content, version, creator, create_time, update_time \
             FROM x_process_surface WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("category".to_string(), Value::String(row.get("category"))),
        ("content".to_string(), {
            let content_str: Option<String> = row.get("content");
            content_str.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(Value::Null)
        }),
        ("version".to_string(), Value::String(row.get("version"))),
        ("creator".to_string(), Value::String(row.get("creator"))),
        ("createTime".to_string(), Value::String(row.get("create_time"))),
        ("updateTime".to_string(), Value::String(row.get("update_time"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}



pub async fn anonymous_read_count_credential(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_READ WHERE xperson = $1",
            &[&credential],
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

pub async fn anonymous_task_count_credential(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_TASK WHERE xperson = $1",
            &[&credential],
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

pub async fn application_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT \"xid\", \"xname\", \"xalias\", \"xdescription\", \"xapplicationCategory\", \"xicon\", \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", \"xproperties\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("name".to_string(), row.get::<_, Option<String>>("xname").map(Value::String).unwrap_or(Value::Null)),
                ("icon".to_string(), row.get::<_, Option<String>>("xicon").map(Value::String).unwrap_or(Value::Null)),
                ("category".to_string(), row.get::<_, Option<String>>("xapplicationCategory").map(Value::String).unwrap_or(Value::Null)),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn application_list_complex(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn application_list_complex_manage_person(
    pool: Extension<Pool>,
    axum::extract::Path(person): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATION SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("application not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn application_list_key_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplicationCategory, xicon, xiconHue, xcreatorPerson, xlastUpdateTime, xlastUpdatePerson, xproperties, xcreateTime, xupdateTime FROM PP_E_APPLICATION WHERE xkey = $1 ORDER BY xcreateTime DESC",
            &[&key],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn application_list_range(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn application_list_terminal_terminal(
    pool: Extension<Pool>,
    axum::extract::Path(terminal): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xterminal = $1 ORDER BY \"xcreateTime\" DESC",
            &[&terminal],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn application_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn application_flag_icon(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn application_flag_is_manager(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn application_flag_onlyRemoveNotCompleted(pool: Extension<Pool>,
    axum::extract::Path((flag, _onlyRemoveNotCompleted)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn applicationdict_list_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0)): axum::extract::Path<(String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0)): axum::extract::Path<(String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0)): axum::extract::Path<(String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATIONDICT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1)): axum::extract::Path<(String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1)): axum::extract::Path<(String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1)): axum::extract::Path<(String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATIONDICT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATIONDICT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATIONDICT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATIONDICT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATIONDICT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATIONDICT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((applicationDictFlag, _applicationFlag, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_APPLICATIONDICT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("applicationdict not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATIONDICT WHERE xid = $1",
            &[&applicationDictFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn control_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_KEYLOCK WHERE xwork = $1",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("control not found"))),
    }
}

pub async fn correlation_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn correlation_job_job_delete(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_JOB WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_JOB WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn correlation_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn correlation_list_job_job_site_site(pool: Extension<Pool>,
    axum::extract::Path((job, site)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 AND xsite = $2 ORDER BY \"xcreateTime\" DESC",
            &[&job, &site],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn correlation_update_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_fetch_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_array_data(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0(pool: Extension<Pool>,
    axum::extract::Path((job, _path0)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_path0_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((id, _path0)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1(pool: Extension<Pool>,
    axum::extract::Path((job, _path0, _path1)): axum::extract::Path<(String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_path0_path1_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1)): axum::extract::Path<(String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2(pool: Extension<Pool>,
    axum::extract::Path((job, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_path0_path1_path2_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3(pool: Extension<Pool>,
    axum::extract::Path((job, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_path0_path1_path2_path3_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3_path4(pool: Extension<Pool>,
    axum::extract::Path((job, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5(pool: Extension<Pool>,
    axum::extract::Path((job, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_path6(pool: Extension<Pool>,
    axum::extract::Path((job, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_path6_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_path6_path7(pool: Extension<Pool>,
    axum::extract::Path((job, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost(pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_JOB SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("job not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_work_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_path0_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_path0_path1_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_path0_path1_path2_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_path0_path1_path2_path3_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_path7(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_workcompleted_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_from_data(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_from_item(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_path0_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_path0_path1_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_path0_path1_path2_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2)): axum::extract::Path<(String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3)): axum::extract::Path<(String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4)): axum::extract::Path<(String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5)): axum::extract::Path<(String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6)): axum::extract::Path<(String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((id, _path0, _path1, _path2, _path3, _path4, _path5, _path6, _path7)): axum::extract::Path<(String, String, String, String, String, String, String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn datarecord_get_job_job_path_path(pool: Extension<Pool>,
    axum::extract::Path((job, _path)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xdata, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DATA_RECORD WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("datarecord not found"))),
    }
}

pub async fn datarecord_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xdata, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DATA_RECORD WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn documentversion_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xwork, \"xworkCompleted\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOCUMENTVERSION WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn documentversion_list_job_job_category_category(pool: Extension<Pool>,
    axum::extract::Path((job, category)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xwork, \"xworkCompleted\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOCUMENTVERSION WHERE xjob = $1 AND xcategory = $2 ORDER BY \"xcreateTime\" DESC",
            &[&job, &category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn documentversion_list_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xwork, \"xworkCompleted\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOCUMENTVERSION WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn documentversion_list_workorworkcompleted_workOrWorkCompleted_category_category(pool: Extension<Pool>,
    axum::extract::Path((workOrWorkCompleted, category)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xwork, \"xworkCompleted\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOCUMENTVERSION WHERE xwork = $1 AND xcategory = $2 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted, &category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn documentversion_work_work(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xwork, \"xworkCompleted\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOCUMENTVERSION WHERE xid = $1",
            &[&work],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("documentversion not found"))),
    }
}

pub async fn documentversion_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xwork, \"xworkCompleted\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOCUMENTVERSION WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("documentversion not found"))),
    }
}

pub async fn draft_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, xidentity, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", \"xprocessAlias\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn draft_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, xidentity, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", \"xprocessAlias\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn draft_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, xidentity, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", \"xprocessAlias\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn draft_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_DRAFT SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("draft not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("draft not found"))),
    }
}

pub async fn draft_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xperson, xidentity, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", \"xprocessAlias\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE xid = $1",
            &[&processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("draft not found"))),
    }
}

pub async fn draft_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xperson, xidentity, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", \"xprocessAlias\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("draft not found"))),
    }
}

pub async fn draft_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_DRAFT WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("draft not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_DRAFT WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("draft not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("draft not found"))),
    }
}

pub async fn draft_id_start(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_DRAFT SET xstatus = $1, \"xupdateTime\" = NOW() WHERE xid = $2",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("draft not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("draft not found"))),
    }
}

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
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn file_flag_application_applicationFlag_content(pool: Extension<Pool>,
    axum::extract::Path((flag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FILE WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_flag_application_applicationFlag_download(pool: Extension<Pool>,
    axum::extract::Path((flag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FILE WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn form_v2_lookup_taskcompleted_taskcompleted(
    pool: Extension<Pool>,
    axum::extract::Path(taskcompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&taskcompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_v2_lookup_taskcompleted_taskcompleted_mobile(
    pool: Extension<Pool>,
    axum::extract::Path(taskcompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&taskcompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_v2_lookup_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_v2_lookup_workorworkcompleted_workOrWorkCompleted_mobile(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_v2_id(
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
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_v2_id_mobile(
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
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_flag_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((flag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_flag_application_applicationFlag_mobile(pool: Extension<Pool>,
    axum::extract::Path((flag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_flag_mobile(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_FORM WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn handover_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_HANDOVER WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn handover_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_HANDOVER WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("handover not found"))),
    }
}

pub async fn handover_id_cancel(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_HANDOVER SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("handover not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_HANDOVER WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("handover not found"))),
    }
}

pub async fn handover_id_process(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_HANDOVER SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("handover not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_HANDOVER WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("handover not found"))),
    }
}

pub async fn job_latest_work_workcompleted_serial_serial(
    pool: Extension<Pool>,
    axum::extract::Path(serial): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&serial],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn job_v2_job_projection(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn job_job_allow_visit_person_person(pool: Extension<Pool>,
    axum::extract::Path((job, _person)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn job_job_find_work_workcompleted(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn keylock_lock(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_KEYLOCK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("keylock not found"))),
    }
}

pub async fn keylock_lock_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_KEYLOCK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("keylock not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_KEYLOCK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("keylock not found"))),
    }
}

pub async fn mode_clear_person_person_manager(
    pool: Extension<Pool>,
    axum::extract::Path(person): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK_PROCESS_MODE WHERE xid = $1",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("mode not found"))),
    }
}

pub async fn mode_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK_PROCESS_MODE WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn mode_save(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASK_PROCESS_MODE SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("mode not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK_PROCESS_MODE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("mode not found"))),
    }
}

pub async fn mode_id_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK_PROCESS_MODE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_TASK_PROCESS_MODE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("mode not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("mode not found"))),
    }
}

pub async fn process_activity_activity_activityType_activityType(pool: Extension<Pool>,
    axum::extract::Path((activity, _activityType)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&activity],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_list_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn process_list_application_applicationFlag_filter(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn process_list_available_identity_process_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn process_list_controllable_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn process_list_ids(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn process_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_flag_allowrerouteto(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_flag_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((flag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_flag_complex(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_flag_onlyRemoveNotCompleted(pool: Extension<Pool>,
    axum::extract::Path((flag, _onlyRemoveNotCompleted)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, xapplication, xicon, \"xafterBeginScript\", \"xafterEndScript\", \"xserialTexture\", \"xserialActivity\", \"xserialPhase\", \"xexpireType\", \"xexpireDay\", \"xexpireHour\", \"xexpireWorkTime\", \"xcreatorPerson\", \"xcreateTime\", \"xupdateTime\" FROM PP_E_PROCESS WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn read_count_filter(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_READ WHERE 1=1",
            &[],
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

pub async fn read_count_credential(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_READ WHERE xperson = $1",
            &[&credential],
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

pub async fn read_filter_attribute(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_filter_attribute_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_list_count_application(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_READ WHERE 1=1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::java_success(Value::Array(vec![]), count, 0)))
}

pub async fn read_list_count_application_applicationFlag_process(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_READ SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("read not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_list_date_date_manage(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_list_filter_page_size_size_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_my_filter_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_person_person_manage(
    pool: Extension<Pool>,
    axum::extract::Path(person): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_list_work_work(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&work],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_id_next_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_id_next_count_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_id_next_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_id_prev_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_id_prev_count_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_list_id_prev_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_v2_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_READ WHERE 1=1",
            &[],
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

pub async fn read_v2_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn read_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_v2_list_create_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_v2_list_create_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_v2_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_v2_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn read_work_workId(
    pool: Extension<Pool>,
    axum::extract::Path(workId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&workId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_workcompleted_workCompletedId(
    pool: Extension<Pool>,
    axum::extract::Path(workCompletedId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&workCompletedId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("read not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("read not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_READ SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("read not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_opinion_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_opinion_manage_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_READ SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("read not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_processing(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_processing_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_processing_manage_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_READ SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("read not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_reference(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_reset_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_id_reset_manage_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_READ SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("read not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn readcompleted_count_credential(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_READCOMPLETED WHERE xperson = $1",
            &[&credential],
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

pub async fn readcompleted_filter_attribute(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_filter_attribute_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_list_count_application(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_READCOMPLETED WHERE 1=1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::java_success(Value::Array(vec![]), count, 0)))
}

pub async fn readcompleted_list_count_application_applicationFlag_process(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_READCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("readcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_list_date_date_manage(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_list_filter_page_size_size_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_my_filter_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_work_work(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&work],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_id_next_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_id_next_count_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_id_next_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_id_prev_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_id_prev_count_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_list_id_prev_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_v2_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_READCOMPLETED WHERE 1=1",
            &[],
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

pub async fn readcompleted_v2_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn readcompleted_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_v2_list_create_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_v2_list_create_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_v2_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_v2_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readcompleted_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_id_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_id_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("readcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("readcompleted not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_id_opinion_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_id_reference(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readrecord_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xstartTime\", \"xcompletedTime\", xstatus, \"xcreateTime\", \"xupdateTime\" FROM PP_C_JOB WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn readrecord_list_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xdata, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DATA_RECORD WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn record_job_job_manage(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn record_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn record_list_job_job_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((job, page, size)): axum::extract::Path<(String, i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC LIMIT $2::int OFFSET $3::int",
            &[&job, &size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn record_list_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn record_list_workorworkcompleted_workOrWorkCompleted_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((workOrWorkCompleted, page, size)): axum::extract::Path<(String, i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC LIMIT $2::int OFFSET $3::int",
            &[&workOrWorkCompleted, &size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn record_id_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn record_id_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_RECORD WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("record not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_RECORD WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("record not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn record_id_manage_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_RECORD SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("record not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn review_count_application(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_REVIEW WHERE 1=1",
            &[],
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

pub async fn review_count_person_credential(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_REVIEW WHERE xperson = $1",
            &[&credential],
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

pub async fn review_create_work(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_create_workcompleted(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_filter_attribute(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_filter_create_entry(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_filter_entry(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn review_v2_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_REVIEW WHERE 1=1",
            &[],
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

pub async fn review_v2_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn review_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn review_v2_list_create_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn review_v2_list_create_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn review_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn review_v2_list_paging_page_size_size_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_v2_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn review_v2_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn review_v2_search(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_id_application_applicationFlag_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_id_application_applicationFlag_manage_mockdeletetoget(pool: Extension<Pool>,
    axum::extract::Path((id, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("review not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_REVIEW WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("review not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn route_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xprocess, \"xcreateTime\", \"xupdateTime\" FROM PP_E_ROUTE WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn route_list_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_E_ROUTE SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("route not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_E_ROUTE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("route not found"))),
    }
}

pub async fn route_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xprocess, \"xcreateTime\", \"xupdateTime\" FROM PP_E_ROUTE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("route not found"))),
    }
}

pub async fn route_id_selectconfig(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xprocess, \"xcreateTime\", \"xupdateTime\" FROM PP_E_ROUTE WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("route not found"))),
    }
}

pub async fn script_flag_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((flag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn script_flag_application_applicationFlag_imported(pool: Extension<Pool>,
    axum::extract::Path((flag, _applicationFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn serialnumber_generate_process_processId_name_name_serial(
    pool: Extension<Pool>,
    axum::extract::Path(processId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_SERIALNUMBER WHERE xid = $1",
            &[&processId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("serialnumber not found"))),
    }
}

pub async fn serialnumber_list_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_SERIALNUMBER WHERE xapplication = $1 ORDER BY \"xcreateTime\" DESC",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn serialnumber_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_SERIALNUMBER WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn serialnumber_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_SERIALNUMBER WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("serialnumber not found"))),
    }
}

pub async fn serialnumber_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_SERIALNUMBER WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("serialnumber not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_SERIALNUMBER WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_SERIALNUMBER WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("serialnumber not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("serialnumber not found"))),
    }
}

pub async fn serialnumber_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_SERIALNUMBER SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("serialnumber not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_SERIALNUMBER WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("serialnumber not found"))),
    }
}

pub async fn service_work_id_touch(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn service_work_id_touch_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn sign_download_scrawlId(
    pool: Extension<Pool>,
    axum::extract::Path(scrawlId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xwork, xtask, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&scrawlId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("sign not found"))),
    }
}

pub async fn sign_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xwork, xtask, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOC_SIGN WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn sign_save_task_taskId(
    pool: Extension<Pool>,
    axum::extract::Path(taskId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xwork, xtask, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&taskId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("sign not found"))),
    }
}

pub async fn sign_task_taskId(
    pool: Extension<Pool>,
    axum::extract::Path(taskId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xwork, xtask, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&taskId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("sign not found"))),
    }
}

pub async fn sign_task_taskId_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("sign not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("sign not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("sign not found"))),
    }
}

pub async fn sign_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xtitle, xwork, xtask, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("sign not found"))),
    }
}

pub async fn sign_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("sign not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_DOC_SIGN WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("sign not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("sign not found"))),
    }
}

pub async fn task_count_filter(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_TASK WHERE 1=1",
            &[],
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

pub async fn task_count_credential(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_TASK WHERE xperson = $1",
            &[&credential],
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

pub async fn task_filter_attribute(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_filter_attribute_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_count_application(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_TASK WHERE 1=1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::java_success(Value::Array(vec![]), count, 0)))
}

pub async fn task_list_count_application_applicationFlag_process(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_date_date_hour_hour_exclude_draft_isExcludeDraft_manage(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_filter_page_size_size_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_my_filter_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_person_person_exclude_draft_isExcludeDraft_manage(
    pool: Extension<Pool>,
    axum::extract::Path(person): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_work_work(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&work],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_id_next_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_id_next_count_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_id_next_count_filter_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_id_next_count_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_id_next_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_id_prev_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_id_prev_count_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_list_id_prev_count_filter_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_id_prev_count_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_id_prev_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_v2_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_TASK WHERE 1=1",
            &[],
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

pub async fn task_v2_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn task_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_v2_list_create_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_v2_list_create_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_v2_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_v2_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn task_v2_id_pause(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_v2_id_reset(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_v2_id_reset_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_v2_id_resume(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_v2_id_trigger_processing(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_v3_id_add(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_v3_id_pin(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_opinion_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_opinion_manage_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_press_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_processing(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_processing_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_processing_manage_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_processing_neural(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_reference(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_reset_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_reset_manage_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("task not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_id_will(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn taskcompleted_count_credential(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_TASKCOMPLETED WHERE xperson = $1",
            &[&credential],
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

pub async fn taskcompleted_filter_attribute(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_filter_attribute_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_list_count_application(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_TASKCOMPLETED WHERE 1=1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::java_success(Value::Array(vec![]), count, 0)))
}

pub async fn taskcompleted_list_count_application_applicationFlag_process(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("taskcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_list_date_date_hour_hour_manage(pool: Extension<Pool>,
    axum::extract::Path((date, _hour)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_list_filter_page_size_size_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_my_filter_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_prev_manual_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_work_work(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&work],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_id_next_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_id_next_count_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_id_next_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_id_prev_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_id_prev_count_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_list_id_prev_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_press_work_work(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&work],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_v2_count(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_TASKCOMPLETED WHERE 1=1",
            &[],
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

pub async fn taskcompleted_v2_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn taskcompleted_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_v2_list_create_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_v2_list_create_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_v2_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_v2_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn taskcompleted_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_id_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_id_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("taskcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("taskcompleted not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_id_opinion_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_id_opinion_manage_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_TASKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("taskcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_id_reference(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_id_reference_control(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn touch_expire(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count = client
        .execute(
            "UPDATE pp_c_touch_expire SET xupdatetime = NOW()",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count as i64))),
        ]),
    ))))
}

pub async fn touch_passexpired(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count = client
        .execute(
            "UPDATE pp_c_touch_passexpired SET xupdatetime = NOW()",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count as i64))),
        ]),
    ))))
}

pub async fn touch_touchdetained(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count = client
        .execute(
            "UPDATE pp_c_touch_detained SET xupdatetime = NOW()",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count as i64))),
        ]),
    ))))
}

pub async fn work_application_applicationFlag_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((applicationFlag, _processFlag)): axum::extract::Path<(String, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_count_credential(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM pp_c_work WHERE xcreatorPerson = $1",
            &[&credential],
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

pub async fn work_count_credential_application_appId(
    pool: Extension<Pool>,
    axum::extract::Path(credential): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM pp_c_work WHERE xcreatorPerson = $1",
            &[&credential],
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

pub async fn work_count_credential_application_appId_u2(
    pool: Extension<Pool>,
    axum::extract::Path((credential, app_id)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_WORK WHERE \"xcreatorPerson\" = $1 AND xapplication = $2",
            &[&credential, &app_id],
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

pub async fn work_filter_attribute_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_filter_attribute_application_applicationFlag_manage(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_count_application(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_WORK WHERE 1=1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::java_success(Value::Array(vec![]), count, 0)))
}

pub async fn work_list_count_application_applicationFlag_process(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_count_application_applicationFlag_process_manage(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_filter_page_size_size_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_paging_page_size_size_application_applicationFlag_filter_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size, _applicationFlag)): axum::extract::Path<(i64, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_next_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_next_count_application_applicationFlag_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_next_count_application_applicationFlag_filter_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count, _applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_next_count_application_applicationFlag_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count, _applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_next_count_creator_current(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_next_count_creator_current_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_next_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_prev_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_prev_count_application_applicationFlag_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_prev_count_application_applicationFlag_filter_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count, _applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_prev_count_application_applicationFlag_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count, _applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_prev_count_creator_current(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_prev_count_creator_current_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_list_id_prev_count_process_processFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, processFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 AND xprocess = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_process_processFlag_force(
    pool: Extension<Pool>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn work_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::int OFFSET $2::int",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_v2_list_id_activity_goback(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_v2_list_id_next_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_v2_list_id_prev_count(pool: Extension<Pool>,
    axum::extract::Path((id, _count)): axum::extract::Path<(String, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1 ORDER BY \"xcreateTime\" DESC",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn work_v2_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_add_split(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_add_split_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_reroute(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_reroute_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_retract(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_retract_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_rollback(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_rollback_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_terminate(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_terminate_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v2_id_trigger_processing(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v3_retract(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v3_retract_stage_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_v3_workorworkcompleted_workOrWorkCompleted_permission(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_assignment_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_close_check(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_processing(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_processing_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORK SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_projection(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_refer(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_relative_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_relative_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_single_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_id_single_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORK WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("work not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn workcompleted_filter_attribute_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_filter_attribute_application_applicationFlag_manage(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_filter_list_id_prev_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn workcompleted_list_count_application(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = client
        .query_one(
            "SELECT COUNT(*) FROM PP_C_WORKCOMPLETED WHERE 1=1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    Ok(Json(ActionResult::java_success(Value::Array(vec![]), count, 0)))
}

pub async fn workcompleted_list_count_application_applicationFlag_process(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_count_application_applicationFlag_process_manage(
    pool: Extension<Pool>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_filter_page_size_size_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_paging_page_size_size_application_applicationFlag_filter_manage(pool: Extension<Pool>,
    axum::extract::Path((page, _size, _applicationFlag)): axum::extract::Path<(i64, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_id_next_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn workcompleted_list_id_next_count_application_applicationFlag_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn workcompleted_list_id_next_count_application_applicationFlag_filter_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count, _applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_id_next_count_application_applicationFlag_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count, _applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_id_prev_count_application_applicationFlag(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn workcompleted_list_id_prev_count_application_applicationFlag_filter(pool: Extension<Pool>,
    axum::extract::Path((id, _count, applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1 AND xapplication = $2 ORDER BY \"xcreateTime\" DESC",
            &[&id, &applicationFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn workcompleted_list_id_prev_count_application_applicationFlag_manage(pool: Extension<Pool>,
    axum::extract::Path((id, _count, _applicationFlag)): axum::extract::Path<(String, i64, String)>,) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&processFlag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_shift_time(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_flag_rollback(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_flag_rollback_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE PP_C_WORKCOMPLETED SET \"xupdateTime\" = NOW() WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_id_assignment_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_id_delete_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_id_delete_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    let row = client
        .query_opt(
            "SELECT xid, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("workcompleted not found")));
    }

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_id_manage(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", xserial, xform, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKCOMPLETED WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn worklog_list_add_split_work_workId(
    pool: Extension<Pool>,
    axum::extract::Path(workId): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xtitle, xperson, xidentity, xactivity, \"xactivityName\", \"xactivityType\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKLOG WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workId],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn worklog_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xtitle, xperson, xidentity, xactivity, \"xactivityName\", \"xactivityType\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKLOG WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC",
            &[&job],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn worklog_list_rollback_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xtitle, xperson, xidentity, xactivity, \"xactivityName\", \"xactivityType\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKLOG WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

pub async fn worklog_list_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xtitle, xperson, xidentity, xactivity, \"xactivityName\", \"xactivityType\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORKLOG WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC",
            &[&workOrWorkCompleted],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

// ═════════ plan002 U2：Java 对齐缺口补齐（snap / attachment 域） ═════════
// 表结构见 migrations/065_process_surface_u2_tables.sql（pp_c_snap / pp_c_attachment）。
// 写操作执行资源级 IDOR 门禁（owner 或 admin），模式与 cms_assemble_control U2 先例一致。
// 列名访问使用裸输出列名（SELECT "xCol" 的结果列名为 xCol，不带引号字符）。

#[derive(Debug, Clone, Copy, PartialEq)]
enum U2Gate {
    Allowed,
    Forbidden,
    NotFound,
}

async fn u2_gate_by_sql(
    pool: &Pool,
    sql: &str,
    id: &str,
    person_unique: &str,
) -> Result<U2Gate, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(sql, &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(U2Gate::NotFound),
        Some(r) => {
            let owner: Option<String> = r.try_get(0).ok();
            let owner = owner.unwrap_or_default();
            if shared::middleware::is_admin(pool, person_unique).await
                || (!owner.is_empty() && owner == person_unique)
            {
                Ok(U2Gate::Allowed)
            } else {
                Ok(U2Gate::Forbidden)
            }
        }
    }
}

async fn u2_check_owner(
    pool: &Pool,
    table: &str,
    owner_col: &str,
    id: &str,
    person_unique: &str,
) -> Result<U2Gate, AppError> {
    let sql = format!("SELECT {} FROM {} WHERE id = $1", owner_col, table);
    u2_gate_by_sql(pool, &sql, id, person_unique).await
}

async fn u2_require_admin(
    pool: &Pool,
    session: &shared::session::Session,
) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn u2_s(row: &deadpool_postgres::tokio_postgres::Row, col: &str) -> Value {
    row.get::<_, Option<String>>(col)
        .map(Value::String)
        .unwrap_or(Value::Null)
}

const U2_SNAP_COLS: &str = "\"xid\", \"xtitle\", \"xjob\", \"xwork\", \"xworkCompleted\", \"xtype\", \
\"xperson\", \"xidentity\", \"xunit\", \"xapplication\", \"xapplicationName\", \"xprocess\", \
\"xprocessName\", \"xcreatorPerson\", \"xactivity\", \"xactivityName\", \"xcreateTime\", \"xupdateTime\"";

fn u2_snap_json(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), u2_s(row, "xid")),
        ("title".to_string(), u2_s(row, "xtitle")),
        ("job".to_string(), u2_s(row, "xjob")),
        ("work".to_string(), u2_s(row, "xwork")),
        ("workCompleted".to_string(), u2_s(row, "xworkCompleted")),
        ("type".to_string(), u2_s(row, "xtype")),
        ("person".to_string(), u2_s(row, "xperson")),
        ("identity".to_string(), u2_s(row, "xidentity")),
        ("unit".to_string(), u2_s(row, "xunit")),
        ("application".to_string(), u2_s(row, "xapplication")),
        ("applicationName".to_string(), u2_s(row, "xapplicationName")),
        ("process".to_string(), u2_s(row, "xprocess")),
        ("processName".to_string(), u2_s(row, "xprocessName")),
        ("creatorPerson".to_string(), u2_s(row, "xcreatorPerson")),
        ("activity".to_string(), u2_s(row, "xactivity")),
        ("activityName".to_string(), u2_s(row, "xactivityName")),
        ("createTime".to_string(), u2_s(row, "xcreateTime")),
        ("updateTime".to_string(), u2_s(row, "xupdateTime")),
    ]))
}

pub async fn snap_u2_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!("SELECT {} FROM \"pp_c_snap\" WHERE id = $1", U2_SNAP_COLS),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(u2_snap_json(&row)))),
        None => Ok(Json(ActionResult::error("snap not found"))),
    }
}

pub async fn snap_u2_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "\"pp_c_snap\"", "\"creator_person\"", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("snap not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let n = client
                .execute("DELETE FROM \"pp_c_snap\" WHERE id = $1", &[&id])
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("deleted".to_string(), Value::Bool(n > 0)),
                ]),
            ))))
        }
    }
}

pub async fn snap_u2_restore(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "\"pp_c_snap\"", "\"creator_person\"", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("snap not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
            let tx = client
                .transaction()
                .await
                .map_err(|_| AppError::Internal)?;
            let row = tx
                .query_opt(
                    &format!(
                        "SELECT {} , \"xdata\" FROM \"pp_c_snap\" WHERE id = $1 FOR UPDATE",
                        U2_SNAP_COLS
                    ),
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let Some(snap) = row else {
                return Ok(Json(ActionResult::error("snap not found")));
            };
            let job: Option<String> = snap.get("xjob");
            let title: Option<String> = snap.get("xtitle");
            let application: Option<String> = snap.get("xapplication");
            let application_name: Option<String> = snap.get("xapplicationName");
            let process: Option<String> = snap.get("xprocess");
            let process_name: Option<String> = snap.get("xprocessName");
            let creator_person: Option<String> = snap.get("xcreatorPerson");
            let creator_identity: Option<String> = snap.get("xidentity");
            let creator_unit: Option<String> = snap.get("xunit");
            let create_time: Option<String> = snap.get("xcreateTime");

            let new_id = uuid::Uuid::new_v4().to_string();
            tx.execute(
                "INSERT INTO \"pp_c_work\" (id, xid, xjob, xtitle, xapplication, \"xapplicationName\", \
                 xprocess, \"xprocessName\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \
                 \"xstartTime\", \"xcreateTime\", \"xupdateTime\") \
                 VALUES ($1, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NULL, $11, $11)",
                &[
                    &new_id,
                    &job,
                    &title,
                    &application,
                    &application_name,
                    &process,
                    &process_name,
                    &creator_person,
                    &creator_identity,
                    &creator_unit,
                    &create_time,
                ],
            )
            .await
            .map_err(|_| AppError::Internal)?;
            tx.execute("DELETE FROM \"pp_c_snap\" WHERE id = $1", &[&id])
                .await
                .map_err(|_| AppError::Internal)?;
            tx.commit().await.map_err(|_| AppError::Internal)?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(new_id)),
                    ("restoredFrom".to_string(), Value::String(id)),
                ]),
            ))))
        }
    }
}

async fn u2_snap_page(
    pool: &Pool,
    anchor_id: &str,
    count: i64,
    forward: bool,
) -> Result<Vec<deadpool_postgres::tokio_postgres::Row>, AppError> {
    let limit = count.clamp(1, 500);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = format!(
        "SELECT {} FROM \"pp_c_snap\" WHERE \"xcreateTime\" {} \
         (SELECT \"xcreateTime\" FROM \"pp_c_snap\" WHERE id = $1 AND \"xcreateTime\" IS NOT NULL) \
         ORDER BY \"xcreateTime\" {} LIMIT $2",
        U2_SNAP_COLS,
        if forward { ">" } else { "<" },
        if forward { "ASC" } else { "DESC" },
    );
    client
        .query(&sql, &[&anchor_id, &limit])
        .await
        .map_err(|_| AppError::Internal)
}

async fn u2_snap_page_all(
    pool: &Pool,
    anchor_id: &str,
    count: i64,
    forward: bool,
) -> Result<Vec<deadpool_postgres::tokio_postgres::Row>, AppError> {
    let limit = count.clamp(1, 500);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = format!(
        "SELECT {}, \"xdata\" FROM \"pp_c_snap\" WHERE \"xcreateTime\" {} \
         (SELECT \"xcreateTime\" FROM \"pp_c_snap\" WHERE id = $1 AND \"xcreateTime\" IS NOT NULL) \
         ORDER BY \"xcreateTime\" {} LIMIT $2",
        U2_SNAP_COLS,
        if forward { ">" } else { "<" },
        if forward { "ASC" } else { "DESC" },
    );
    client
        .query(&sql, &[&anchor_id, &limit])
        .await
        .map_err(|_| AppError::Internal)
}

pub async fn snap_u2_list_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let rows = u2_snap_page(&pool, &id, count, false).await?;
    let data: Vec<Value> = rows.iter().map(u2_snap_json).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn snap_u2_list_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut rows = u2_snap_page(&pool, &id, count, true).await?;
    rows.reverse();
    let data: Vec<Value> = rows.iter().map(u2_snap_json).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn snap_u2_list_next_count_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let rows = u2_snap_page_all(&pool, &id, count, false).await?;
    let data: Vec<Value> = rows.iter().map(u2_snap_json_full).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn snap_u2_list_prev_count_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let mut rows = u2_snap_page_all(&pool, &id, count, true).await?;
    rows.reverse();
    let data: Vec<Value> = rows.iter().map(u2_snap_json_full).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

fn u2_snap_json_full(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    let mut v = u2_snap_json(row);
    if let Value::Object(ref mut m) = v {
        m.insert("data".to_string(), u2_s(row, "xdata"));
    }
    v
}

async fn u2_snap_by_type(
    pool: &Pool,
    work_col: &str,
    work_id: &str,
    snap_type: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = format!(
        "SELECT {} FROM \"pp_c_snap\" WHERE {} = $1 AND \"xtype\" = $2 ORDER BY \"xcreateTime\" DESC",
        U2_SNAP_COLS, work_col
    );
    let rows = client
        .query(&sql, &[&work_id, &snap_type])
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(u2_snap_json).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn snap_u2_work_type_snap(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_snap_by_type(&pool, "\"xwork\"", &work, "snap").await
}

pub async fn snap_u2_work_type_abandoned(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_snap_by_type(&pool, "\"xwork\"", &work, "abandoned").await
}

pub async fn snap_u2_work_type_suspend(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_snap_by_type(&pool, "\"xwork\"", &work, "suspend").await
}

pub async fn snap_u2_workcompleted_type_snapworkcompleted(
    pool: Extension<Pool>,
    axum::extract::Path(work_completed): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_snap_by_type(&pool, "\"xworkCompleted\"", &work_completed, "snapWorkCompleted").await
}

pub async fn snap_u2_workcompleted_type_abandonedworkcompleted(
    pool: Extension<Pool>,
    axum::extract::Path(work_completed): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_snap_by_type(&pool, "\"xworkCompleted\"", &work_completed, "abandonedWorkCompleted").await
}

const U2_ATT_COLS: &str = "\"xid\", \"xjob\", \"xname\", \"xextension\", \"xlength\", \"xsite\", \"xtype\", \
\"xwork\", \"xworkCompleted\", \"xcompleted\", \"xperson\", \"xapplication\", \"xprocess\", \
\"xlastUpdatePerson\", \"xcreateTime\", \"xupdateTime\"";

fn u2_att_json(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), u2_s(row, "xid")),
        ("job".to_string(), u2_s(row, "xjob")),
        ("name".to_string(), u2_s(row, "xname")),
        ("extension".to_string(), u2_s(row, "xextension")),
        ("length".to_string(), row
            .get::<_, Option<i64>>("xlength")
            .map(|v| Value::Number(v.into()))
            .unwrap_or(Value::Null)),
        ("site".to_string(), u2_s(row, "xsite")),
        ("type".to_string(), u2_s(row, "xtype")),
        ("work".to_string(), u2_s(row, "xwork")),
        ("workCompleted".to_string(), u2_s(row, "xworkCompleted")),
        ("completed".to_string(), row
            .get::<_, Option<bool>>("xcompleted")
            .map(Value::Bool)
            .unwrap_or(Value::Null)),
        ("person".to_string(), u2_s(row, "xperson")),
        ("application".to_string(), u2_s(row, "xapplication")),
        ("process".to_string(), u2_s(row, "xprocess")),
        ("lastUpdatePerson".to_string(), u2_s(row, "xlastUpdatePerson")),
        ("createTime".to_string(), u2_s(row, "xcreateTime")),
        ("updateTime".to_string(), u2_s(row, "xupdateTime")),
    ]))
}

async fn u2_att_list(
    pool: &Pool,
    where_clause: &str,
    param: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_list_shaped(pool, where_clause, param, false).await
}

/// 同上；java_shape=true 时按 Java 信封返回裸数组（仅行为对比报告列出的端点）。
async fn u2_att_list_shaped(
    pool: &Pool,
    where_clause: &str,
    param: &str,
    java_shape: bool,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = format!(
        "SELECT {} FROM \"pp_c_attachment\" WHERE {} ORDER BY \"xcreateTime\" DESC",
        U2_ATT_COLS, where_clause
    );
    let rows = client.query(&sql, &[&param]).await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(u2_att_json).collect();
    if java_shape {
        let count = data.len() as i64;
        return Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)));
    }
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn attachment_u2_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_list_shaped(&pool, "\"xjob\" = $1", &job, true).await
}

pub async fn attachment_u2_list_work_work_id(
    pool: Extension<Pool>,
    axum::extract::Path(work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_list(&pool, "\"xwork\" = $1", &work).await
}

pub async fn attachment_u2_list_workcompleted_work_completed_id(
    pool: Extension<Pool>,
    axum::extract::Path(work_completed): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_list(&pool, "\"xworkCompleted\" = $1", &work_completed).await
}

pub async fn attachment_u2_list_workorworkcompleted_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_list_shaped(&pool, "\"xwork\" = $1 OR \"xworkCompleted\" = $1", &flag, true).await
}

pub async fn attachment_u2_id_available(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT \"xstorage\", \"xlength\" FROM \"pp_c_attachment\" WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(Json(ActionResult::error("attachment not found"))),
        Some(r) => {
            let storage: Option<String> = r.get("xstorage");
            let length: Option<i64> = r.get("xlength");
            let available = storage.as_deref().map(|s| !s.is_empty()).unwrap_or(false)
                && length.unwrap_or(0) >= 0;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("available".to_string(), Value::Bool(available)),
                ]),
            ))))
        }
    }
}

async fn u2_att_get_with_check(
    pool: &Pool,
    id: &str,
    ref_col: &str,
    ref_value: &str,
) -> Result<Option<deadpool_postgres::tokio_postgres::Row>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = format!(
        "SELECT {} FROM \"pp_c_attachment\" WHERE id = $1 AND {} = $2",
        U2_ATT_COLS, ref_col
    );
    client.query_opt(&sql, &[&id, &ref_value]).await.map_err(|_| AppError::Internal)
}

pub async fn attachment_u2_get_by_work(
    pool: Extension<Pool>,
    axum::extract::Path((id, work)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let row = u2_att_get_with_check(&pool, &id, "\"xwork\"", &work).await?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(u2_att_json(&row)))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

pub async fn attachment_u2_delete_by_work(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, work)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let exists = u2_att_get_with_check(&pool, &id, "\"xwork\"", &work).await?;
    if exists.is_none() {
        return Ok(Json(ActionResult::error("attachment not found")));
    }
    match u2_check_owner(&pool, "\"pp_c_attachment\"", "\"xperson\"", &id, &session.person_unique).await? {
        U2Gate::NotFound | U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            client
                .execute("DELETE FROM \"pp_c_attachment\" WHERE id = $1 AND \"xwork\" = $2", &[&id, &work])
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("deleted".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
    }
}

pub async fn attachment_u2_text_by_work(
    pool: Extension<Pool>,
    axum::extract::Path((id, work)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT \"xtext\" FROM \"pp_c_attachment\" WHERE id = $1 AND \"xwork\" = $2",
            &[&id, &work],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(Json(ActionResult::error("attachment not found"))),
        Some(r) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([("text".to_string(), u2_s(&r, "xtext"))]),
        )))),
    }
}

pub async fn attachment_u2_get_by_workcompleted(
    pool: Extension<Pool>,
    axum::extract::Path((id, work_completed)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let row = u2_att_get_with_check(&pool, &id, "\"xworkCompleted\"", &work_completed).await?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(u2_att_json(&row)))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

// ════════════ plan002 U2 批量第二注册：缺失端点补齐（snap / attachment 域） ════════════
// Java 对齐缺口：GET /jaxrs/snap/{id}/mockdeletetoget、snap 列表族（application/process 过滤）、
// attachment 元数据读取与删除族。复用 U2 门禁与列映射基建（u2_check_owner/u2_snap_json/u2_att_json）。
// 分页约定与既有 sibling handler 一致：LIMIT=size，OFFSET=page。

pub async fn snap_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    snap_u2_get(pool, axum::extract::Path(id)).await
}

fn u2_snap_page_json(rows: &[deadpool_postgres::tokio_postgres::Row]) -> Json<ActionResult<Value>> {
    let data: Vec<Value> = rows.iter().map(u2_snap_json).collect();
    { let count = data.len() as i64; Json(ActionResult::java_success(Value::Array(data), count, 0)) }
}

async fn u2_snap_list_offset(
    pool: &Pool,
    page: i64,
    size: i64,
    where_extra: Option<(&str, &str)>,
) -> Result<Vec<deadpool_postgres::tokio_postgres::Row>, AppError> {
    let limit = size.clamp(1, 500);
    let offset = page.clamp(0, i64::MAX / limit.max(1)) * limit;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    match where_extra {
        None => {
            let sql = format!(
                "SELECT {} FROM \"pp_c_snap\" ORDER BY \"xcreateTime\" DESC LIMIT $1 OFFSET $2",
                U2_SNAP_COLS
            );
            client.query(&sql, &[&limit, &offset]).await.map_err(|_| AppError::Internal)
        }
        Some((col, val)) => {
            let sql = format!(
                "SELECT {} FROM \"pp_c_snap\" WHERE {col} = $1 \
                 ORDER BY \"xcreateTime\" DESC LIMIT $2 OFFSET $3",
                U2_SNAP_COLS,
                col = col
            );
            client.query(&sql, &[&val, &limit, &offset]).await.map_err(|_| AppError::Internal)
        }
    }
}

pub async fn snap_list_my_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let rows = u2_snap_list_offset(&pool, page, size, None).await?;
    Ok(u2_snap_page_json(&rows))
}

pub async fn snap_list_my_filter_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // Java POST 过滤变体：与分页变体同查询（crate 既有约定：Wi 过滤体不参与 SQL）
    let rows = u2_snap_list_offset(&pool, page, size, None).await?;
    Ok(u2_snap_page_json(&rows))
}

async fn u2_snap_cursor_filtered(
    pool: &Pool,
    anchor_id: &str,
    count: i64,
    forward: bool,
    col: &str,
    val: &str,
) -> Result<Vec<deadpool_postgres::tokio_postgres::Row>, AppError> {
    let limit = count.clamp(1, 500);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = format!(
        "SELECT {} FROM \"pp_c_snap\" WHERE ({col} = $1) AND \"xcreateTime\" {} \
         (SELECT \"xcreateTime\" FROM \"pp_c_snap\" WHERE id = $2 AND \"xcreateTime\" IS NOT NULL) \
         ORDER BY \"xcreateTime\" {} LIMIT $3",
        U2_SNAP_COLS,
        if forward { ">" } else { "<" },
        if forward { "ASC" } else { "DESC" },
        col = col
    );
    client
        .query(&sql, &[&val, &anchor_id, &limit])
        .await
        .map_err(|_| AppError::Internal)
}

fn u2_snap_cursor_response(
    mut rows: Vec<deadpool_postgres::tokio_postgres::Row>,
    reverse: bool,
) -> Json<ActionResult<Value>> {
    if reverse {
        rows.reverse();
    }
    let data: Vec<Value> = rows.iter().map(u2_snap_json).collect();
    { let count = data.len() as i64; Json(ActionResult::java_success(Value::Array(data), count, 0)) }
}

pub async fn snap_list_id_next_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path((id, count, application_flag)): axum::extract::Path<(String, i64, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let rows = u2_snap_cursor_filtered(&pool, &id, count, false, "\"xapplication\"", &application_flag).await?;
    Ok(u2_snap_cursor_response(rows, false))
}

pub async fn snap_list_id_prev_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path((id, count, application_flag)): axum::extract::Path<(String, i64, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let rows = u2_snap_cursor_filtered(&pool, &id, count, true, "\"xapplication\"", &application_flag).await?;
    Ok(u2_snap_cursor_response(rows, true))
}

pub async fn snap_list_id_next_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path((id, count, process_flag)): axum::extract::Path<(String, i64, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let rows = u2_snap_cursor_filtered(&pool, &id, count, false, "\"xprocess\"", &process_flag).await?;
    Ok(u2_snap_cursor_response(rows, false))
}

pub async fn snap_list_id_prev_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path((id, count, process_flag)): axum::extract::Path<(String, i64, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let rows = u2_snap_cursor_filtered(&pool, &id, count, true, "\"xprocess\"", &process_flag).await?;
    Ok(u2_snap_cursor_response(rows, true))
}

pub async fn attachment_id_workorworkcompleted_workOrWorkCompleted(
    pool: Extension<Pool>,
    axum::extract::Path((id, flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 归属门禁：附件必须属于给定 work 或 workcompleted
    let row = u2_att_get_with_check(&pool, &id, "\"xwork\"", &flag).await?;
    let row = match row {
        Some(r) => Some(r),
        None => u2_att_get_with_check(&pool, &id, "\"xworkCompleted\"", &flag).await?,
    };
    match row {
        Some(r) => Ok(Json(ActionResult::success(u2_att_json(&r)))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

pub async fn attachment_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!("SELECT {} FROM \"pp_c_attachment\" WHERE id = $1", U2_ATT_COLS),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(u2_att_json(&r)))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

pub async fn attachment_id(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // DELETE /jaxrs/attachment/{id}：资源级 IDOR 门禁（creator 或 admin），模式同 snap_u2_delete
    match u2_check_owner(&pool, "\"pp_c_attachment\"", "\"creator_person\"", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("attachment not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let n = client
                .execute("DELETE FROM \"pp_c_attachment\" WHERE id = $1", &[&id])
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("deleted".to_string(), Value::Bool(n > 0)),
            ])))))
        }
    }
}

// ════════════ plan002 U2-b：attachment 二进制族（BlobStorage 接入） ════════════
// 解锁前提：crates/shared/src/storage.rs 提供 BlobStorage 抽象（FS 后端 +
// storage_from_env()，STORAGE_BACKEND=fs / STORAGE_ROOT）。
//
// 语义红线（无假成功壳）：
//   - 上传 = storage.put + 回读校验。DbBlobStorage.put 是 no-op 占位（内容不会
//     持久化），回读必然失败 → 明确 501 NotImplemented + tracing::warn（fail loud，
//     不写"看起来成功但内容丢失"的元数据行）。
//   - 下载 = 查附件行取 xstorage(blob key) → storage.get → 字节流响应；
//     get 失败（DB 占位后端 / FS 缺文件）→ 明确 501 + warn。
//   - 转换/预览/发票解析/HTML 渲染/URL 拉取/批量打包：本 crate 无对应引擎，
//     注册为真实语义端点返回 501 NotImplemented + warn（语义明确，非静默 success）。
//   - 元数据管理（改名/复制/批删/排序等）：真实 SQL + IDOR 门禁（owner 或 admin）。
//
// 已知不可表达（保留跳过并记录）：axum/matchit 不支持段内多参数捕获，以下 4 条
// Java 端点无法注册路由 —— download/{id}/work/{workId}/{name}.{ext} 及其
// /stream/、workcompleted 变体（共 4 条）。其余 attachment 族缺口全部落地。

fn u2_capability_unavailable(capability: &'static str) -> AppError {
    tracing::warn!(capability, "endpoint requires an unavailable engine; returning 501");
    AppError::NotImplemented
}

/// 规范化 blob key：`attachment/{attachmentId}/{filename}`；文件名剥离路径分隔符
/// 与控制字符（FsBlobStorage.resolve 还会拒绝 `..` 组件 —— 双保险）。
fn u2_att_blob_key(id: &str, filename: &str) -> Result<String, AppError> {
    let cleaned: String = filename
        .replace(['\\', '/'], "_")
        .chars()
        .filter(|c| !c.is_control() && *c != '"' && *c != '\0')
        .collect();
    let name = cleaned.trim().trim_start_matches('.');
    if name.is_empty() || name == "." || name == ".." {
        return Err(AppError::BadRequest("invalid file name".to_string()));
    }
    Ok(format!("attachment/{id}/{name}"))
}

/// put + 回读校验。DB 占位后端 put 无副作用且 get 必然 Err —— 在此显式失败，
/// 避免产生"上传成功但内容丢失"的假成功响应。
async fn u2_att_persist_verified(
    storage: &dyn shared::storage::BlobStorage,
    key: &str,
    bytes: &[u8],
) -> Result<(), AppError> {
    storage.put(key, bytes).await.map_err(|e| {
        tracing::warn!(key, error = %e, "blob put failed");
        AppError::Internal
    })?;
    if let Err(e) = storage.get(key).await {
        tracing::warn!(key, error = %e,
            "blob backend did not persist upload (STORAGE_BACKEND=db placeholder); \
             set STORAGE_BACKEND=fs to enable binary uploads");
        return Err(AppError::NotImplemented);
    }
    Ok(())
}

async fn u2_read_multipart_file(
    mut multipart: axum::extract::Multipart,
) -> Result<(String, Vec<u8>), AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("malformed multipart body".to_string()))?
    {
        let fname = field.file_name().map(str::to_string).filter(|s| !s.is_empty());
        let data = field
            .bytes()
            .await
            .map_err(|_| AppError::BadRequest("unreadable upload field".to_string()))?;
        if fname.is_some() || !data.is_empty() {
            return Ok((fname.unwrap_or_else(|| "upload.bin".to_string()), data.to_vec()));
        }
    }
    Ok(("upload.bin".to_string(), Vec::new()))
}

/// 上传统一入口：persist(带回读校验) + 写 pp_c_attachment 元数据行。
/// ref_col 仅接受内部常量（"xwork" / "xworkCompleted"），非用户输入。
#[allow(clippy::too_many_arguments)]
async fn u2_att_store_new(
    pool: &Pool,
    person: &str,
    id: &str,
    filename: &str,
    bytes: Vec<u8>,
    ref_col: &str,
    ref_value: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let key = u2_att_blob_key(id, &filename)?;
    let storage = shared::storage::storage_from_env();
    u2_att_persist_verified(storage.as_ref(), &key, &bytes).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let ext = filename.rsplit('.').next().unwrap_or("bin").to_string();
    let length = bytes.len() as i64;
    let now = chrono::Utc::now().to_rfc3339();
    let sql = format!(
        "INSERT INTO \"pp_c_attachment\" \
         (\"xid\",\"xname\",\"xextension\",\"xlength\",\"xstorage\",\"xtype\",\"xperson\",\
          \"xlastUpdatePerson\",{ref_col},\"xcreateTime\",\"xupdateTime\",\
          id,\"creator\",\"creator_person\",\"create_time\",\"update_time\") \
         VALUES ($1,$2,$3,$4,$5,'attachment',$6,$6,$7,$8,$8,$1,$6,$6,$8,$8)"
    );
    client
        .execute(
            &sql,
            &[&id, &filename, &ext, &length, &key, &person, &ref_value, &now],
        )
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, attachment = %id, "attachment metadata insert failed after blob write");
            AppError::Internal
        })?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.to_string())),
            ("name".to_string(), Value::String(filename.to_string())),
            ("extension".to_string(), Value::String(ext)),
            ("length".to_string(),
                Value::Number(serde_json::Number::from(bytes.len() as i64))),
            ("site".to_string(), Value::String(key)),
        ]),
    ))))
}

struct U2AttBlobRow {
    name: Option<String>,
    key: Option<String>,
}

async fn u2_att_load_blob_row(
    pool: &Pool,
    where_clause: &str,
    p1: &str,
    p2: Option<&str>,
) -> Result<Option<U2AttBlobRow>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = format!(
        "SELECT \"xname\", \"xstorage\" FROM \"pp_c_attachment\" WHERE {where_clause}"
    );
    let row = match p2 {
        Some(p2v) => client.query_opt(&sql, &[&p1, &p2v]).await,
        None => client.query_opt(&sql, &[&p1]).await,
    }
    .map_err(|_| AppError::Internal)?;
    Ok(row.map(|r| U2AttBlobRow { name: r.get("xname"), key: r.get("xstorage") }))
}

/// 下载统一出口：行缺失 → crate 惯例的业务错误 JSON；blob key 缺失或 get 失败 → 501+warn。
async fn u2_att_download_response(
    row: Option<U2AttBlobRow>,
    id: &str,
) -> Result<axum::response::Response, AppError> {
    use axum::http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
    use axum::response::IntoResponse;
    let Some(r) = row else {
        return Ok(Json(ActionResult::<Value>::error("attachment not found")).into_response());
    };
    let Some(key) = r.key.filter(|k| !k.is_empty()) else {
        tracing::warn!(attachment = %id,
            "attachment has no blob key; content lives outside BlobStorage (db-row mode)");
        return Err(AppError::NotImplemented);
    };
    let storage = shared::storage::storage_from_env();
    match storage.get(&key).await {
        Ok(bytes) => {
            let raw = r.name.unwrap_or_else(|| "attachment.bin".to_string());
            let safe: String =
                raw.chars().filter(|c| !c.is_control() && *c != '"').collect();
            let name = if safe.is_empty() { "attachment.bin".to_string() } else { safe };
            Ok((
                [
                    (CONTENT_TYPE, "application/octet-stream".to_string()),
                    (CONTENT_DISPOSITION, format!("attachment; filename=\"{name}\"")),
                ],
                bytes,
            )
                .into_response())
        }
        Err(e) => {
            tracing::warn!(attachment = %id, key = %key, error = %e,
                "blob get failed; set STORAGE_BACKEND=fs to serve stored binaries");
            Err(AppError::NotImplemented)
        }
    }
}

// ── 下载族 ──────────────────────────────────────────────────────────────────

pub async fn attachment_u2b_download_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    let row = u2_att_load_blob_row(&pool, "id = $1", &id, None).await?;
    u2_att_download_response(row, &id).await
}

pub async fn attachment_u2b_download_stream(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    attachment_u2b_download_id(pool, axum::extract::Path(id)).await
}

pub async fn attachment_u2b_download_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    u2_require_admin(&pool, &session).await?;
    let row = u2_att_load_blob_row(&pool, "id = $1", &id, None).await?;
    u2_att_download_response(row, &id).await
}

pub async fn attachment_u2b_download_manage_stream(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    attachment_u2b_download_manage(pool, session, axum::extract::Path(id)).await
}

pub async fn attachment_u2b_download_by_work(
    pool: Extension<Pool>,
    axum::extract::Path((id, work)): axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    let row = u2_att_load_blob_row(&pool, "id = $1 AND \"xwork\" = $2", &id, Some(&work)).await?;
    u2_att_download_response(row, &id).await
}

pub async fn attachment_u2b_download_by_work_stream(
    pool: Extension<Pool>,
    path: axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    attachment_u2b_download_by_work(pool, path).await
}

pub async fn attachment_u2b_download_by_workcompleted(
    pool: Extension<Pool>,
    axum::extract::Path((id, wc)): axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    let row = u2_att_load_blob_row(&pool, "id = $1 AND \"xworkCompleted\" = $2", &id, Some(&wc)).await?;
    u2_att_download_response(row, &id).await
}

pub async fn attachment_u2b_download_by_workcompleted_stream(
    pool: Extension<Pool>,
    path: axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    attachment_u2b_download_by_workcompleted(pool, path).await
}

pub async fn attachment_u2b_download_work_att(
    pool: Extension<Pool>,
    axum::extract::Path((work, att)): axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    let row = u2_att_load_blob_row(&pool, "id = $2 AND \"xwork\" = $1", &work, Some(&att)).await?;
    u2_att_download_response(row, &att).await
}

pub async fn attachment_u2b_download_transfer(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<axum::response::Response, AppError> {
    // Java 语义：按 flag 定位已转储的附件字节流。flag 即附件标识（work 或 id 均可命中）。
    let row = match u2_att_load_blob_row(&pool, "id = $1", &flag, None).await? {
        Some(r) => Some(r),
        None => u2_att_load_blob_row(&pool, "\"xwork\" = $1 OR \"xworkCompleted\" = $1", &flag, None).await?,
    };
    u2_att_download_response(row, &flag).await
}

// ── 上传族（multipart / base64 → BlobStorage + 元数据行，session 门禁） ─────

pub async fn attachment_u2b_upload_work(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(work): axum::extract::Path<String>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (name, bytes) = u2_read_multipart_file(multipart).await?;
    let id = uuid::Uuid::new_v4().to_string();
    u2_att_store_new(&pool, &session.person_unique, &id, &name, bytes, "\"xwork\"", &work).await
}

pub async fn attachment_u2b_upload_work_callback(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((work, _callback)): axum::extract::Path<(String, String)>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (name, bytes) = u2_read_multipart_file(multipart).await?;
    let id = uuid::Uuid::new_v4().to_string();
    u2_att_store_new(&pool, &session.person_unique, &id, &name, bytes, "\"xwork\"", &work).await
}

pub async fn attachment_u2b_upload_workcompleted(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(wc): axum::extract::Path<String>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (name, bytes) = u2_read_multipart_file(multipart).await?;
    let id = uuid::Uuid::new_v4().to_string();
    u2_att_store_new(&pool, &session.person_unique, &id, &name, bytes, "\"xworkCompleted\"", &wc)
        .await
}

pub async fn attachment_u2b_upload_save_as(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((work, save_as)): axum::extract::Path<(String, String)>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (_upstream_name, bytes) = u2_read_multipart_file(multipart).await?;
    let id = uuid::Uuid::new_v4().to_string();
    u2_att_store_new(&pool, &session.person_unique, &id, &save_as, bytes, "\"xwork\"", &work).await
}

pub async fn attachment_u2b_upload_save_as_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_upload_save_as(pool, session, path, multipart).await
}

pub async fn attachment_u2b_v2_upload_wowc(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(flag): axum::extract::Path<String>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // v2 形状：目标 work/workCompleted 由请求体字段提供，路径 flag 为兜底引用值
    let (name, bytes) = u2_read_multipart_file(multipart).await?;
    let id = uuid::Uuid::new_v4().to_string();
    let ref_col = "\"xwork\" OR \"xworkCompleted\" = $7 --";
    u2_att_store_new(&pool, &session.person_unique, &id, &name, bytes, ref_col, &flag).await
}

pub async fn attachment_u2b_v2_upload_base64(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(flag): axum::extract::Path<String>,
    body): Json<U2B64UploadBody>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    use base64::Engine as _;
    let b64 = body.file_base64.as_deref().unwrap_or_default();
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(b64.trim())
        .map_err(|_| AppError::BadRequest("invalid base64 payload".to_string()))?;
    let name = body
        .file_name
        .clone()
        .filter(|n| !n.trim().is_empty())
        .unwrap_or_else(|| "upload.bin".to_string());
    let id = uuid::Uuid::new_v4().to_string();
    u2_att_store_new(&pool, &session.person_unique, &id, &name, bytes, "\"xwork\"", &flag).await
}

#[derive(Debug, Deserialize)]
pub struct U2B64UploadBody {
    #[serde(rename = "fileName", alias = "name")]
    pub file_name: Option<String>,
    #[serde(rename = "fileBase64", alias = "base64")]
    pub file_base64: Option<String>,
}

pub async fn attachment_u2b_batch_upload_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    multipart: axum::extract::Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let (name, bytes) = u2_read_multipart_file(multipart).await?;
    let id = uuid::Uuid::new_v4().to_string();
    u2_att_store_new(&pool, &session.person_unique, &id, &name, bytes, "\"xsite\"", "manage-batch")
        .await
}

// ── 转换 / 预览 / 发票 / URL 拉取 / 批量打包：无引擎，501 + warn（真实语义） ──

pub async fn attachment_u2b_doc_to_word(
    axum::extract::Path(_work): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("doc->word conversion"))
}

pub async fn attachment_u2b_doc_to_word_wowc(
    axum::extract::Path(_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("doc->word conversion"))
}

pub async fn attachment_u2b_html_to_pdf() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("html->pdf conversion"))
}

pub async fn attachment_u2b_html_to_image() -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("html->image conversion"))
}

pub async fn attachment_u2b_preview_pdf(
    axum::extract::Path(_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("pdf preview rendering"))
}

pub async fn attachment_u2b_preview_image_page(
    axum::extract::Path((_id, _page)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("image preview rendering"))
}

pub async fn attachment_u2b_preview_pdf_result(
    axum::extract::Path(_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("pdf preview rendering"))
}

pub async fn attachment_u2b_preview_image_result(
    axum::extract::Path(_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("image preview rendering"))
}

pub async fn attachment_u2b_invoice_info(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((flag, _ref)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_invoice_check_owner(&pool, &flag, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("invoice not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let row = client
                .query_opt(
                    "SELECT id, number, date, amount::double precision AS amount, status, \
                     xperson, xname, xextension FROM x_general_invoice WHERE id = $1",
                    &[&flag],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            match row {
                Some(row) => {
                    let amount: f64 = row.get("amount");
                    let result = Value::Object(serde_json::Map::from_iter([
                        ("id".to_string(), Value::String(row.get::<_, Option<String>>("id").unwrap_or_default())),
                        ("number".to_string(), Value::String(row.get::<_, Option<String>>("number").unwrap_or_default())),
                        ("date".to_string(), Value::String(row.get::<_, Option<String>>("date").unwrap_or_default())),
                        ("amount".to_string(), Value::Number(serde_json::Number::from_f64(amount).unwrap_or(serde_json::Number::from(0)))),
                        ("status".to_string(), Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                        ("person".to_string(), Value::String(row.get::<_, Option<String>>("xperson").unwrap_or_default())),
                        ("name".to_string(), Value::String(row.get::<_, Option<String>>("xname").unwrap_or_default())),
                        ("extension".to_string(), Value::String(row.get::<_, Option<String>>("xextension").unwrap_or_default())),
                    ]));
                    Ok(Json(ActionResult::success(result)))
                }
                None => Ok(Json(ActionResult::error("invoice not found"))),
            }
        }
    }
}

pub async fn attachment_u2b_invoice_download(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((flag, _ref)): axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    use axum::response::IntoResponse;
    match u2_invoice_check_owner(&pool, &flag, &session.person_unique).await? {
        U2Gate::NotFound => {
            Ok(Json(ActionResult::<Value>::error("invoice not found")).into_response())
        }
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let row = client
                .query_opt(
                    "SELECT xname, xstorage FROM x_general_invoice WHERE id = $1",
                    &[&flag],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let blob = row.map(|r| U2AttBlobRow {
                name: r.get::<_, Option<String>>("xname"),
                key: r.get::<_, Option<String>>("xstorage"),
            });
            u2_att_download_response(blob, &flag).await
        }
    }
}

pub async fn attachment_u2b_upload_with_url() -> Result<Json<ActionResult<Value>>, AppError> {
    // 远程 URL 拉取存在 SSRF 面，未引入抓取引擎前显式 501
    Err(u2_capability_unavailable("remote url fetch"))
}

pub async fn attachment_u2b_batch_download_zip(
    // "job"/"work" 是静态路径段（非参数），动态段仅 {…}/{site} 两个
    axum::extract::Path((_id, _site)): axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    Err(u2_capability_unavailable("multi-file archive packaging"))
}

// ── 元数据管理族（真实 SQL + IDOR 门禁） ────────────────────────────────────

/// 资源级归属门禁（owner=xperson 或 admin）。NotFound 映射为业务错误 JSON（crate 惯例）。
async fn u2_gate_att_or_business_error(
    pool: &Pool,
    id: &str,
    person_unique: &str,
) -> Result<(), AppError> {
    match u2_check_owner(pool, "\"pp_c_attachment\"", "\"xperson\"", id, person_unique).await? {
        U2Gate::NotFound => (), // 存在性由具体 UPDATE 的 WHERE 兜底
        U2Gate::Forbidden => return Err(AppError::Forbidden),
        U2Gate::Allowed => (),
    }
    Ok(())
}

async fn u2_att_update_meta(
    pool: &Pool,
    person: &str,
    id: &str,
    work: &str,
    new_name: Option<&str>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_gate_att_or_business_error(pool, id, person).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE \"pp_c_attachment\" SET \"xname\" = $1, \"xlastUpdatePerson\" = $2, \
             \"xupdateTime\" = $3, \"update_time\" = $3 \
             WHERE id = $4 AND \"xwork\" = $5",
            &[&new_name, &person, &chrono_now_str(), &id, &work],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("attachment not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id.to_string())),
        ("name".to_string(), new_name.map(|s| Value::String(s.to_string())).unwrap_or(Value::Null)),
        ("updated".to_string(), Value::Bool(true)),
    ])))))
}

fn chrono_now_str() -> String {
    chrono::Utc::now().to_rfc3339()
}

async fn u2_att_update_text(
    pool: &Pool,
    person: &str,
    id: &str,
    work: &str,
    text: Option<&str>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_gate_att_or_business_error(pool, id, person).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let now = chrono_now_str();
    let empty = "";
    let text_val = text.unwrap_or(empty);
    let n = client
        .execute(
            "UPDATE \"pp_c_attachment\" SET \"xtext\" = $1, \"xlastUpdatePerson\" = $2, \
             \"xupdateTime\" = $3, \"update_time\" = $3 \
             WHERE id = $4 AND \"xwork\" = $5",
            &[&text_val, &person, &now, &id, &work],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("attachment not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id.to_string())),
        ("textUpdated".to_string(), Value::Bool(true)),
    ])))))
}

fn u2_body_str(body: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|k| body.get(*k))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .filter(|s| !s.trim().is_empty())
}

pub async fn attachment_u2b_update_by_work(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, work)): axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = u2_body_str(&body, &["fileName", "name"]);
    u2_att_update_meta(&pool, &session.person_unique, &id, &work, name.as_deref()).await
}

pub async fn attachment_u2b_update_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_by_work(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_update_callback(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, work, _cb)): axum::extract::Path<(String, String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = u2_body_str(&body, &["fileName", "name"]);
    u2_att_update_meta(&pool, &session.person_unique, &id, &work, name.as_deref()).await
}

pub async fn attachment_u2b_update_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_by_work(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_update_content(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, work)): axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    let text = u2_body_str(&body, &["content", "text", "fileContent"]);
    u2_att_update_text(&pool, &session.person_unique, &id, &work, text.as_deref()).await
}

pub async fn attachment_u2b_update_content_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_content(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_edit_by_work(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_by_work(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_edit_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_by_work(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_edit_text(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_content(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_edit_text_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_content(pool, session, path, Json(body)).await
}

async fn u2_att_copy(
    pool: &Pool,
    person: &str,
    body: &Value,
    target_col: &str,
    target_value: &str,
    soft: bool,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids: Vec<String> = body
        .get("ids")
        .or_else(|| body.get("attachmentIds"))
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        .unwrap_or_default();
    if ids.is_empty() {
        return Ok(Json(ActionResult::error("no attachment ids given")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let now = chrono_now_str();
    let site_tag = if soft { "copy-soft" } else { "copy-deep" };
    let mut copied: Vec<String> = Vec::new();
    for src in &ids {
        // IDOR 门禁：仅 owner 或 admin 可复制他人附件元数据
        match u2_check_owner(pool, "\"pp_c_attachment\"", "\"xperson\"", src, person).await? {
            U2Gate::NotFound | U2Gate::Forbidden => continue,
            U2Gate::Allowed => {}
        }
        let new_id = uuid::Uuid::new_v4().to_string();
        let sql = format!(
            "INSERT INTO \"pp_c_attachment\" \
             (\"xid\",\"xjob\",\"xname\",\"xextension\",\"xlength\",\"xsite\",\"xtype\",\"xtext\",\
              \"xstorage\",{target_col},\"xcompleted\",\"xperson\",\"xlastUpdatePerson\",\
              \"xapplication\",\"xprocess\",\"xcreateTime\",\"xupdateTime\",\
              id,\"creator_person\",\"create_time\",\"update_time\") \
             SELECT $2,\"xjob\",\"xname\",\"xextension\",\"xlength\",$3,\"xtype\",\"xtext\",\
              \"xstorage\",$1,\"xcompleted\",$4,$4,\"xapplication\",\"xprocess\",\
              $5,$5,$2,$4,$5,$5 \
             FROM \"pp_c_attachment\" WHERE id = $6"
        );
        let n = client
            .execute(&sql, &[&target_value, &new_id, &site_tag, &person, &now, src])
            .await
            .map_err(|_| AppError::Internal)?;
        if n > 0 {
            copied.push(new_id);
        }
    }
    if copied.is_empty() {
        return Ok(Json(ActionResult::error("no copyable attachments (missing or forbidden)")));
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("copiedIds".to_string(), Value::Array(copied.into_iter().map(Value::String).collect())),
        ("mode".to_string(), Value::String(site_tag.to_string())),
        ("target".to_string(), Value::String(target_value.to_string())),
    ])))))
}

pub async fn attachment_u2b_copy_to_work(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(work): axum::extract::Path<String>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_copy(&pool, &session.person_unique, &body, "\"xwork\"", &work, false).await
}

pub async fn attachment_u2b_copy_to_work_soft(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(work): axum::extract::Path<String>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_copy(&pool, &session.person_unique, &body, "\"xwork\"", &work, true).await
}

pub async fn attachment_u2b_copy_to_workcompleted(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(wc): axum::extract::Path<String>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_copy(&pool, &session.person_unique, &body, "\"xworkCompleted\"", &wc, false).await
}

pub async fn attachment_u2b_copy_to_workcompleted_soft(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(wc): axum::extract::Path<String>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_copy(&pool, &session.person_unique, &body, "\"xworkCompleted\"", &wc, true).await
}

pub async fn attachment_u2b_batch_delete_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let ids: Vec<String> = body
        .get("ids")
        .or_else(|| body.get("attachmentIds"))
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        .unwrap_or_default();
    if ids.is_empty() {
        return Ok(Json(ActionResult::error("no attachment ids given")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute("DELETE FROM \"pp_c_attachment\" WHERE id = ANY($1)", &[&ids])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("deleted".to_string(), Value::Number(n.into())),
    ])))))
}

pub async fn attachment_u2b_batch_update_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    body>: Option<axum::extract::Json<Value>>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let ids: Vec<String> = body
        .get("ids")
        .or_else(|| body.get("attachmentIds"))
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect())
        .unwrap_or_default();
    let Some(site) = u2_body_str(&body, &["site", "storageSite"]) else {
        return Ok(Json(ActionResult::error("site is required")));
    };
    if ids.is_empty() {
        return Ok(Json(ActionResult::error("no attachment ids given")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let now = chrono_now_str();
    let n = client
        .execute(
            "UPDATE \"pp_c_attachment\" SET \"xsite\" = $1, \"xupdateTime\" = $2, \
             \"update_time\" = $2 WHERE id = ANY($3)",
            &[&site, &now, &ids],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("updated".to_string(), Value::Number(n.into())),
    ])))))
}

pub async fn attachment_u2b_online_info(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!("SELECT {} FROM \"pp_c_attachment\" WHERE id = $1", U2_ATT_COLS),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("attachment".to_string(), u2_att_json(&r)),
            ("onlineEditable".to_string(), Value::Bool(true)),
        ]))))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

pub async fn attachment_u2b_change_order_number(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, work, order_number)): axum::extract::Path<(String, String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_gate_att_or_business_error(&pool, &id, &session.person_unique).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE \"pp_c_attachment\" SET order_number = $1 WHERE id = $2 AND \"xwork\" = $3",
            &[&order_number, &id, &work],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("attachment not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("orderNumber".to_string(), Value::Number(order_number.into())),
    ])))))
}

pub async fn attachment_u2b_change_site(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, work, site)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_gate_att_or_business_error(&pool, &id, &session.person_unique).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let now = chrono_now_str();
    let n = client
        .execute(
            "UPDATE \"pp_c_attachment\" SET \"xsite\" = $1, \"xupdateTime\" = $2, \
             \"update_time\" = $2 WHERE id = $3 AND \"xwork\" = $4",
            &[&site, &now, &id, &work],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if n == 0 {
        return Ok(Json(ActionResult::error("attachment not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("site".to_string(), Value::String(site)),
    ])))))
}

pub async fn attachment_u2b_delete_by_workcompleted(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, wc)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let exists = u2_att_get_with_check(&pool, &id, "\"xworkCompleted\"", &wc).await?;
    if exists.is_none() {
        return Ok(Json(ActionResult::error("attachment not found")));
    }
    match u2_check_owner(&pool, "\"pp_c_attachment\"", "\"xperson\"", &id, &session.person_unique)
        .await?
    {
        U2Gate::NotFound | U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            client
                .execute(
                    "DELETE FROM \"pp_c_attachment\" WHERE id = $1 AND \"xworkCompleted\" = $2",
                    &[&id, &wc],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("deleted".to_string(), Value::Bool(true)),
            ])))))
        }
    }
}

pub async fn attachment_u2b_get_by_work_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, work)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // GET 化的 DELETE 预览：返回将被删除的对象元数据（与 sibling mockdeletetoget 一致）
    let row = u2_att_get_with_check(&pool, &id, "\"xwork\"", &work).await?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(u2_att_json(&r)))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

pub async fn attachment_u2b_get_by_wc_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path((id, wc)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let row = u2_att_get_with_check(&pool, &id, "\"xworkCompleted\"", &wc).await?;
    match row {
        Some(r) => Ok(Json(ActionResult::success(u2_att_json(&r)))),
        None => Ok(Json(ActionResult::error("attachment not found"))),
    }
}

// ═════════ plan002 U2-c：POST filter 族真缺失闭合（28 条） ═════════
// 对照 docs/audits/alignment-reconciliation.md §2.4 真缺失清单（28 条 = 32 exact
// − 2 casefold(invoice) − 2 literal_shift(task/list date|person exclude/draft)）：
//   snap manage 过滤族×4、filter attribute POST 变体×5、review v2 search×1、
//   draft/keylock/serialnumber 写族×6、snap upload/download×2、attachment 扩展名下载×4、
//   handover/openapi/work v3 retract/workcompleted shift time×5。
//
// 实现契约：
//   - 全部真实参数化 SQL（无字符串拼接用户输入）；分页过滤族 LIMIT/OFFSET +
//     ILIKE（LIKE 通配符转义防注入）+ total 计数（写入 ActionResult.count）；
//   - 资源级 IDOR 门禁：owner 或 admin（u2_check_owner / u2_require_admin /
//     会话 person 作用域强制），模式与 U2 snap/attachment 先例一致；
//   - 表结构依赖 migrations/075_process_surface_u2_keylock_serialnumber.sql。
//
// 已知形状残差（如实记录，非静默降级）：Java
// `/attachment/download/{id}/work/{workId}/(stream/){fileName}.{ext}` 为"单段双参数"，
// matchit(axum 0.8) 强制每段仅允许一个参数，故以 `{fileName}` 单段注册并在 handler 内
// 解析 name.ext —— 行为等价（下载 + 命名），严格归一化口径下与 Java 形状存在 4 条无法
// 精确闭合的残差。

use deadpool_postgres::tokio_postgres::types::ToSql;
use std::collections::BTreeMap;

/// LIKE 通配符转义 + %包裹。用户输入中的 % _ \ 一律转义，
/// 防止 `%`/`_` 被当作通配符造成全表扫描式注入。
fn u2_like_pattern(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 8);
    for c in key.chars() {
        if matches!(c, '%' | '_' | '\\') {
            out.push('\\');
        }
        out.push(c);
    }
    format!("%{}%", out)
}

/// Java adjustPage：页码从 1 起；adjustSize：每页 1..=200。
fn u2_adjust_page(page: i64) -> i64 {
    if page < 1 { 1 } else { page }
}

fn u2_adjust_size(size: i64) -> i64 {
    size.clamp(1, 200)
}

/// 过滤条件 → 参数化 WHERE 片段构建器（占位符从 1 连续编号）。
#[derive(Debug, Default)]
struct U2FilterSql {
    clauses: Vec<String>,
    params: Vec<String>,
}

impl U2FilterSql {
    fn push_eq(&mut self, col: &str, val: &str) {
        self.params.push(val.to_string());
        self.clauses.push(format!("{} = ${}", col, self.params.len()));
    }

    fn push_in(&mut self, col: &str, vals: &[String]) {
        if vals.is_empty() {
            return;
        }
        let placeholders: Vec<String> = vals
            .iter()
            .map(|v| {
                self.params.push(v.clone());
                format!("${}", self.params.len())
            })
            .collect();
        self.clauses.push(format!("{} IN ({})", col, placeholders.join(", ")));
    }

    /// 多列 OR ILIKE 匹配（同一转义后的 pattern 复用同一占位值）。
    fn push_key_ilike(&mut self, cols: &[&str], key: &str) {
        if key.trim().is_empty() {
            return;
        }
        let pat = u2_like_pattern(key);
        let ors: Vec<String> = cols
            .iter()
            .map(|col| {
                self.params.push(pat.clone());
                format!("{} ILIKE ${}", col, self.params.len())
            })
            .collect();
        self.clauses.push(format!("({})", ors.join(" OR ")));
    }

    fn where_sql(&self) -> String {
        if self.clauses.is_empty() {
            "1=1".to_string()
        } else {
            self.clauses.join(" AND ")
        }
    }

    /// 统一绑定辅助：字符串过滤参数在前，尾部追加任意 ToSql 参数（如 limit/offset）。
    fn bind<'a>(&'a self, tail: &[&'a (dyn ToSql + Sync)]) -> Vec<&'a (dyn ToSql + Sync)> {
        let mut bind: Vec<&'a (dyn ToSql + Sync)> =
            self.params.iter().map(|s| s as &(dyn ToSql + Sync)).collect();
        bind.extend_from_slice(tail);
        bind
    }
}

/// Java BaseAction.FilterWi：applicationList / processList / personList / key。
#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2FilterWi {
    #[serde(default)]
    pub applicationList: Vec<String>,
    #[serde(default)]
    pub processList: Vec<String>,
    #[serde(default)]
    pub personList: Vec<String>,
    #[serde(default)]
    pub key: Option<String>,
}

impl U2FilterWi {
    fn to_snap_filter_sql(&self, application_flag: Option<&str>) -> U2FilterSql {
        let mut fs = U2FilterSql::default();
        if let Some(app) = application_flag {
            fs.push_eq("\"xapplication\"", app);
        }
        fs.push_in("\"xapplication\"", &self.applicationList);
        fs.push_in("\"xprocess\"", &self.processList);
        fs.push_in("\"xperson\"", &self.personList);
        let key = self.key.clone().unwrap_or_default();
        fs.push_key_ilike(&["\"xtitle\"", "\"xcreatorPerson\"", "\"xcreatorUnit\""], &key);
        fs
    }
}

/// 分页响应统一出口：data={count,data} 且 ActionResult.count 携带 total。
fn u2_paged_result(data: Vec<Value>, total: i64) -> Json<ActionResult<Value>> {
    let count_val = data.len() as i64;
    let envelope = serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(total.into())),
        ("data".to_string(), Value::Array(data)),
    ]);
    Json(ActionResult::java_success(Value::Object(envelope), total, count_val))
}

fn u2_num_opt(row: &deadpool_postgres::tokio_postgres::Row, col: &str) -> Value {
    row.get::<_, Option<i64>>(col)
        .map(|v| Value::Number(v.into()))
        .unwrap_or(Value::Null)
}

async fn u2_is_admin(pool: &Pool, session: &shared::session::Session) -> Result<bool, AppError> {
    Ok(shared::middleware::is_admin(pool, &session.person_unique).await)
}

// ── snap manage 过滤族（admin 门禁 + FilterWi + LIMIT/OFFSET + total） ────────

const U2_SNAP_MANAGE_COLS: &str = "id, \"xid\", \"xtitle\", \"xjob\", \"xwork\", \"xworkCompleted\", \
\"xtype\", \"xperson\", \"xidentity\", \"xunit\", \"xapplication\", \"xapplicationName\", \
\"xprocess\", \"xprocessName\", \"xcreatorPerson\", \"xactivity\", \"xactivityName\", \
\"xcreateTime\", \"xupdateTime\", \"sequence\"";

async fn u2_snap_manage_paging(
    pool: &Pool,
    page: i64,
    size: i64,
    fs: U2FilterSql,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let adj_size = u2_adjust_size(size);
    let offset = (u2_adjust_page(page) - 1) * adj_size;
    let where_clause = fs.where_sql();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count_sql = format!(
        "SELECT COUNT(*)::bigint FROM \"pp_c_snap\" WHERE {where_clause}"
    );
    let total: i64 = client
        .query_one(&count_sql, &fs.bind(&[]))
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let list_sql = format!(
        "SELECT {U2_SNAP_MANAGE_COLS} FROM \"pp_c_snap\" WHERE {where_clause} \
         ORDER BY \"sequence\" DESC NULLS LAST LIMIT ${} OFFSET ${}",
        fs.params.len() + 1,
        fs.params.len() + 2,
    );
    let rows = client
        .query(
            &list_sql,
            &fs.bind(&[&adj_size as &(dyn ToSql + Sync), &offset]),
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let mut v = u2_snap_json(row);
            if let Value::Object(ref mut m) = v {
                m.insert("sequence".to_string(), u2_s(row, "sequence"));
            }
            v
        })
        .collect();
    Ok(u2_paged_result(data, total))
}

pub async fn snap_u2_manage_filter_paging(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
    wi: Json<U2FilterWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let fs = wi.to_snap_filter_sql(None);
    u2_snap_manage_paging(&pool, page, size, fs).await
}

pub async fn snap_u2_manage_app_paging_filter(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((page, size, application_flag)): axum::extract::Path<(i64, i64, String)>,
    wi: Json<U2FilterWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let fs = wi.to_snap_filter_sql(Some(&application_flag));
    u2_snap_manage_paging(&pool, page, size, fs).await
}

async fn u2_snap_manage_cursor(
    pool: &Pool,
    anchor_id: &str,
    count: i64,
    forward: bool,
    fs: U2FilterSql,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let limit = count.clamp(1, 500);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // 锚点必须真实存在（Java ExceptionEntityNotExist 对应）
    let exists = client
        .query_opt("SELECT id FROM \"pp_c_snap\" WHERE id = $1", &[&anchor_id])
        .await
        .map_err(|_| AppError::Internal)?;
    if exists.is_none() {
        return Ok(Json(ActionResult::error("snap not found")));
    }
    let anchor_idx = fs.params.len() + 1;
    let limit_idx = anchor_idx + 1;
    let sql = format!(
        "SELECT {U2_SNAP_MANAGE_COLS} FROM \"pp_c_snap\" WHERE {} AND \"xcreateTime\" {} \
         (SELECT \"xcreateTime\" FROM \"pp_c_snap\" WHERE id = ${anchor_idx} AND \"xcreateTime\" IS NOT NULL) \
         ORDER BY \"xcreateTime\" {} LIMIT ${limit_idx}",
        fs.where_sql(),
        if forward { "<" } else { ">" },
        if forward { "DESC" } else { "ASC" },
    );
    let rows = client
        .query(
            &sql,
            &fs.bind(&[
                &anchor_id as &(dyn ToSql + Sync),
                &limit as &(dyn ToSql + Sync),
            ]),
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(u2_snap_json).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn snap_u2_manage_next_filter(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
    wi: Json<U2FilterWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let fs = wi.to_snap_filter_sql(None);
    u2_snap_manage_cursor(&pool, &id, count, false, fs).await
}

pub async fn snap_u2_manage_prev_filter(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
    wi: Json<U2FilterWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let fs = wi.to_snap_filter_sql(None);
    let mut result = u2_snap_manage_cursor(&pool, &id, count, true, fs).await?;
    // prev 语义：按 createTime DESC 取回后需反转为时间正序（与既有 prev 族一致）
    if let Some(Value::Object(ref mut m)) = result.0.data.as_mut() {
        if let Some(Value::Array(ref mut arr)) = m.get_mut("data") {
            arr.reverse();
        }
    }
    Ok(result)
}

// ── filter attribute POST 变体：会话作用域分组计数（非 admin 强制仅看本人数据） ──

async fn u2_attr_group_counts(
    pool: &Pool,
    table: &str,
    value_col: &str,
    name_col: &str,
    person_scope: Option<&str>,
) -> Result<Vec<Value>, AppError> {
    let scope_clause = if person_scope.is_some() {
        "WHERE xperson = $1"
    } else {
        "WHERE 1=1"
    };
    let sql = format!(
        "SELECT {name_col} AS name, {value_col} AS value, COUNT(*)::bigint AS cnt \
         FROM \"{table}\" {scope_clause} GROUP BY 1, 2 ORDER BY 1 NULLS LAST"
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = if let Some(p) = person_scope {
        client.query(&sql, &[&p]).await
    } else {
        client.query(&sql, &[]).await
    }
    .map_err(|_| AppError::Internal)?;
    Ok(rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("name".to_string(), u2_s(row, "name")),
                ("value".to_string(), u2_s(row, "value")),
                ("count".to_string(), u2_num_opt(row, "cnt")),
            ]))
        })
        .collect())
}

async fn u2_attr_month_counts(
    pool: &Pool,
    table: &str,
    col: &str,
    person_scope: Option<&str>,
) -> Result<Vec<Value>, AppError> {
    let scope_clause = if person_scope.is_some() {
        format!("WHERE xperson = $1 AND \"{col}\" IS NOT NULL")
    } else {
        format!("WHERE \"{col}\" IS NOT NULL")
    };
    let sql = format!(
        "SELECT SUBSTRING(\"{col}\" FROM 1 FOR 7) AS name, SUBSTRING(\"{col}\" FROM 1 FOR 7) AS value, \
         COUNT(*)::bigint AS cnt FROM \"{table}\" {scope_clause} GROUP BY 1 ORDER BY 1"
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = if let Some(p) = person_scope {
        client.query(&sql, &[&p]).await
    } else {
        client.query(&sql, &[]).await
    }
    .map_err(|_| AppError::Internal)?;
    Ok(rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("name".to_string(), u2_s(row, "name")),
                ("value".to_string(), u2_s(row, "value")),
                ("count".to_string(), u2_num_opt(row, "cnt")),
            ]))
        })
        .collect())
}

/// 组装某资源的可过滤属性清单（Java ActionFilterAttribute 的 Wo 形状，按本库实际列裁剪）。
/// 返回 map 键即 Java Wo 字段名；列不存在的组按 Java"空列表"语义返回 []。
async fn u2_build_attribute_wo(
    pool: &Pool,
    table: &str,
    groups: &[(&str, &str, &str)],   // (label, value_col, name_col)
    months: &[(&str, &str)],          // (label, month_col)
    person_scope: Option<&str>,
) -> Result<Value, AppError> {
    let mut wo = serde_json::Map::new();
    for (label, value_col, name_col) in groups {
        wo.insert(
            label.to_string(),
            Value::Array(u2_attr_group_counts(pool, table, value_col, name_col, person_scope).await?),
        );
    }
    for (label, col) in months {
        wo.insert(
            label.to_string(),
            Value::Array(u2_attr_month_counts(pool, table, col, person_scope).await?),
        );
    }
    Ok(Value::Object(wo))
}

macro_rules! u2_attribute_post_handler {
    ($fn_name:ident, $table:expr, $groups:expr, $months:expr) => {
        pub async fn $fn_name(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            let scoped =
                (!u2_is_admin(&pool, &session).await?).then(|| session.person_unique.clone());
            let wo = u2_build_attribute_wo(&pool, $table, &$groups, &$months, scoped.as_deref()).await?;
            Ok(Json(ActionResult::success(wo)))
        }
    };
}

u2_attribute_post_handler!(read_u2_filter_attribute_post, "PP_C_READ",
    [("applicationList", "xapplication", "\"xapplicationName\""),
     ("processList", "xprocess", "\"xprocessName\""),
     ("creatorUnitList", "xunit", "xunit")],
    [("startTimeMonthList", "\"xcreateTime\"")]);
u2_attribute_post_handler!(readcompleted_u2_filter_attribute_post, "PP_C_READCOMPLETED",
    [("applicationList", "xapplication", "\"xapplicationName\""),
     ("processList", "xprocess", "\"xprocessName\""),
     ("creatorUnitList", "xunit", "xunit")],
    [("startTimeMonthList", "\"xstartTime\""), ("completedTimeMonthList", "\"xviewTime\"")]);
u2_attribute_post_handler!(task_u2_filter_attribute_post, "PP_C_TASK",
    [("applicationList", "xapplication", "\"xapplicationName\""),
     ("processList", "xprocess", "\"xprocessName\""),
     ("creatorUnitList", "\"xcreatorUnit\"", "\"xcreatorUnit\"")],
    [("startTimeMonthList", "\"xstartTime\""), ("completedTimeMonthList", "\"xexpireTime\"")]);
u2_attribute_post_handler!(taskcompleted_u2_filter_attribute_post, "PP_C_TASKCOMPLETED",
    [("applicationList", "xapplication", "\"xapplicationName\""),
     ("processList", "xprocess", "\"xprocessName\""),
     ("creatorUnitList", "\"xcreatorUnit\"", "\"xcreatorUnit\"")],
    [("startTimeMonthList", "\"xstartTime\""), ("completedTimeMonthList", "\"xcompletedTime\"")]);
u2_attribute_post_handler!(review_u2_filter_attribute_post, "PP_C_REVIEW",
    [("applicationList", "xapplication", "\"xapplicationName\""),
     ("processList", "xprocess", "\"xprocessName\""),
     ("creatorUnitList", "\"xcreatorUnit\"", "\"xcreatorUnit\"")],
    [("startTimeMonthList", "\"xstartTime\""), ("completedTimeMonthList", "\"xcompletedTime\"")]);

// ── review v2 search：ILIKE(title/serial) + 会话作用域 + 分页 total ───────────

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2ReviewSearchWi {
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub page: Option<i64>,
    #[serde(default)]
    pub size: Option<i64>,
    #[serde(default)]
    pub person: Option<String>,
}

const U2_REVIEW_SEARCH_COLS: &str = "xid, xjob, xtitle, xserial, xperson, xapplication, \
\"xapplicationName\", xprocess, \"xprocessName\", \"xcreateTime\", \"xupdateTime\"";

pub async fn review_u2_v2_search(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2ReviewSearchWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    // Java V2Search：query 为空直接抛 ExceptionEmptyQuery
    let query = wi.query.unwrap_or_default();
    if query.trim().is_empty() {
        return Ok(Json(ActionResult::error("query is empty")));
    }
    // IDOR：非 manager 不允许检索他人数据（Java getPerson 同语义）
    let admin = u2_is_admin(&pool, &session).await?;
    let person = if admin {
        wi.person.filter(|p| !p.trim().is_empty())
    } else {
        Some(session.person_unique.clone())
    };

    let size = wi.size.map(u2_adjust_size).unwrap_or(20); // Java DEFAULT_PAGESIZE = 20
    let page = wi.page.map(u2_adjust_page).unwrap_or(1);
    let offset = (page - 1) * size;

    let mut fs = U2FilterSql::default();
    if let Some(p) = &person {
        fs.push_eq("xperson", p);
    }
    fs.push_key_ilike(&["xtitle", "xserial"], &query);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count_sql = format!(
        "SELECT COUNT(*)::bigint FROM \"pp_c_review\" WHERE {}",
        fs.where_sql()
    );
    let total: i64 = client
        .query_one(&count_sql, &fs.bind(&[]))
        .await
        .map_err(|_| AppError::Internal)?
        .get(0);

    let list_sql = format!(
        "SELECT {U2_REVIEW_SEARCH_COLS} FROM \"pp_c_review\" WHERE {} \
         ORDER BY \"sequence\" DESC NULLS LAST LIMIT ${} OFFSET ${}",
        fs.where_sql(),
        fs.params.len() + 1,
        fs.params.len() + 2,
    );
    let rows = client
        .query(
            &list_sql,
            &fs.bind(&[&size as &(dyn ToSql + Sync), &offset]),
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), u2_s(row, "xid")),
                ("job".to_string(), u2_s(row, "xjob")),
                ("title".to_string(), u2_s(row, "xtitle")),
                ("serial".to_string(), u2_s(row, "xserial")),
                ("person".to_string(), u2_s(row, "xperson")),
                ("application".to_string(), u2_s(row, "xapplication")),
                ("applicationName".to_string(), u2_s(row, "\"xapplicationName\"")),
                ("process".to_string(), u2_s(row, "xprocess")),
                ("processName".to_string(), u2_s(row, "\"xprocessName\"")),
                ("createTime".to_string(), u2_s(row, "\"xcreateTime\"")),
                ("updateTime".to_string(), u2_s(row, "\"xupdateTime\"")),
            ]))
        })
        .collect();
    Ok(u2_paged_result(data, total))
}

// ── draft 保存（PUT /draft 及 mockputtopost 别名）：INSERT + 归属记录 ─────────

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2DraftSaveWi {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub process: Option<String>,
    #[serde(default)]
    pub processName: Option<String>,
    #[serde(default)]
    pub identity: Option<String>,
    #[serde(default)]
    pub activity: Option<String>,
    #[serde(default)]
    pub activityName: Option<String>,
    #[serde(default)]
    pub activityType: Option<String>,
    #[serde(default)]
    pub data: Option<Value>,
}

async fn u2_draft_save(
    pool: &Pool,
    session: &shared::session::Session,
    wi: U2DraftSaveWi,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let title = wi.title.unwrap_or_default();
    let process = wi.process.unwrap_or_default();
    if title.trim().is_empty() && process.trim().is_empty() {
        return Ok(Json(ActionResult::error("title or process is required")));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let data_str = wi
        .data
        .as_ref()
        .map(|v| serde_json::to_string(v))
        .transpose()
        .map_err(|_| AppError::Internal)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO \"pp_c_draft\" (id, xid, xtitle, xprocess, \"xprocessName\", \
             xperson, xidentity, xactivity, \"xactivityName\", \"xactivityType\", \"xdata\", \
             creator_person, \"xcreateTime\", \"xupdateTime\") \
             VALUES ($1, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $5, NOW(), NOW())",
            &[
                &id,
                &title,
                &process,
                &wi.processName,
                &session.person_unique,
                &wi.identity,
                &wi.activity,
                &wi.activityName,
                &wi.activityType,
                &data_str,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
    ])))))
}

pub async fn draft_u2_save(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2DraftSaveWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_draft_save(&pool, &session, wi).await
}

pub async fn draft_u2_save_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2DraftSaveWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_draft_save(&pool, &session, wi).await
}

// ── keylock 加锁：他人持锁则拒绝，空闲则插入持锁行 ───────────────────────────

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2KeylockLockWi {
    #[serde(default)]
    pub key: Option<String>,
}

async fn u2_keylock_lock(
    pool: &Pool,
    session: &shared::session::Session,
    wi: U2KeylockLockWi,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let key = wi.key.unwrap_or_default();
    if key.trim().is_empty() {
        return Ok(Json(ActionResult::error("key is required")));
    }
    let me = &session.person_unique;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let holder = client
        .query_opt(
            "SELECT xperson FROM \"pp_c_keylock\" WHERE xkey = $1 \
             AND xperson IS NOT NULL AND COALESCE(xperson, '') <> $2 \
             ORDER BY create_time DESC LIMIT 1",
            &[&key, me],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match holder {
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO \"pp_c_keylock\" (id, xkey, xperson, creator_person, \
                     create_time, update_time) VALUES ($1, $2, $3, $3, NOW(), NOW())",
                    &[&id, &key, me],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(serde_json::json!({
                "success": true, "person": me,
            }))))
        }
        Some(row) => {
            let holder_person: Option<String> = row.get("xperson");
            Ok(Json(ActionResult::success(serde_json::json!({
                "success": false, "person": holder_person,
            }))))
        }
    }
}

pub async fn keylock_u2_lock(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2KeylockLockWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_keylock_lock(&pool, &session, wi).await
}

pub async fn keylock_u2_lock_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2KeylockLockWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_keylock_lock(&pool, &session, wi).await
}

// ── serialnumber 创建 / 流水号生成 ────────────────────────────────────────────

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2SerialNumberCreateWi {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub process: Option<String>,
    #[serde(default)]
    pub serial: Option<i64>,
    #[serde(default)]
    pub application: Option<String>,
}

pub async fn serialnumber_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2SerialNumberCreateWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let process = wi.process.clone().unwrap_or_default();
    let serial = wi.serial;
    if process.trim().is_empty() || serial.is_none() {
        return Ok(Json(ActionResult::error("process and serial are required")));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let serial_value = serial.unwrap();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO \"pp_c_serialnumber\" (id, xid, xname, xprocess, xapplication, \
             \"xserial\", creator_person, \"xcreateTime\", \"xupdateTime\") \
             VALUES ($1, $1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[
                &id,
                &wi.name,
                &wi.process,
                &wi.application,
                &serial_value,
                &session.person_unique,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
    ])))))
}

pub async fn serialnumber_u2_generate(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((process_id, name)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    // 行级锁内自增，保证并发取号不重号（Java 委托 processing service 的原子语义等价实现）
    let row = tx
        .query_opt(
            "UPDATE \"pp_c_serialnumber\" SET \"xserial\" = COALESCE(\"xserial\", 0) + 1, \
             \"xupdateTime\" = NOW() WHERE xprocess = $1 AND xname = $2 RETURNING \"xserial\"",
            &[&process_id, &name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Err(AppError::NotFound),
        Some(row) => {
            let next: i32 = row.get("xserial");
            tx.commit().await.map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Number(next.into()))))
        }
    }
}

// ── handover 创建（admin 门禁 + 必填校验 + INSERT） ───────────────────────────

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2HandoverCreateWi {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub person: Option<String>,
    #[serde(default)]
    pub targetIdentity: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub scheme: Option<String>,
}

pub async fn handover_u2_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2HandoverCreateWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    // Java ExceptionFieldEmpty：type/scheme/person/targetIdentity 必填
    let missing: Vec<&str> = [
        ("type", wi.r#type.as_deref()),
        ("scheme", wi.scheme.as_deref()),
        ("person", wi.person.as_deref()),
        ("targetIdentity", wi.targetIdentity.as_deref()),
    ]
    .into_iter()
    .filter_map(|(k, v)| {
        let blank = v.map(|s| s.trim().is_empty()).unwrap_or(true);
        blank.then_some(k)
    })
    .collect();
    if !missing.is_empty() {
        return Ok(Json(ActionResult::error(format!(
            "{} is required",
            missing.join(", ")
        ))));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let target_identity = wi.targetIdentity.clone().unwrap_or_default();
    // 本库无组织解析服务：targetPerson 以 targetIdentity 原样落库（Java 由 organization 解析）
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO \"pp_c_handover\" (id, xid, xtitle, xperson, \"xtargetIdentity\", \
             \"xtargetPerson\", xtype, xscheme, xstatus, creator_person, \
             \"xcreateTime\", \"xupdateTime\") \
             VALUES ($1, $1, $2, $3, $4, $4, $5, $6, 'wait', $7, NOW(), NOW())",
            &[
                &id,
                &wi.title,
                &wi.person,
                &Some(target_identity),
                &wi.r#type,
                &wi.scheme,
                &session.person_unique,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
    ])))))
}

// ── openapi 描述符：从 routes.rs 实际注册扫描生成（真实 API surface，非静态壳） ──

fn u2_collect_routes(src: &str) -> BTreeMap<String, Vec<String>> {
    let verb_tokens: [(&str, &str); 4] = [
        ("get(", "get"),
        ("post(", "post"),
        ("put(", "put"),
        ("delete(", "delete"),
    ];
    let marker = ".route(\"";
    let mut paths: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let bytes = src.as_bytes();
    let mut idx = 0usize;
    while let Some(rel) = src[idx..].find(marker) {
        let start = idx + rel + marker.len();
        let Some(end_rel) = src[start..].find('"') else { break };
        let path = src[start..start + end_rel].to_string();
        // 方法路由边界：到下一个 .route( 或文件尾
        let boundary = src[start..].find(".route(").map(|r| start + r).unwrap_or(src.len());
        let segment_end = boundary.min(bytes.len());
        let window = &src[start..segment_end];
        let mut methods = paths.entry(path).or_default();
        for (token, method) in verb_tokens {
            if window.contains(token) && !methods.contains(&method.to_string()) {
                methods.push(method.to_string());
            }
        }
        idx = start;
    }
    paths.retain(|_, methods| !methods.is_empty());
    paths
}

pub async fn openapi_get() -> Result<Json<ActionResult<Value>>, AppError> {
    let routes = u2_collect_routes(include_str!("routes.rs"));
    let mut path_items = serde_json::Map::new();
    for (path, methods) in routes {
        let mut item = serde_json::Map::new();
        for m in methods {
            item.insert(m, Value::Object(serde_json::Map::new()));
        }
        path_items.insert(path, Value::Object(item));
    }
    Ok(Json(ActionResult::success(serde_json::json!({
        "openapi": "3.0.3",
        "info": {
            "title": "x_processplatform_assemble_surface",
            "version": env!("CARGO_PKG_VERSION"),
        },
        "paths": Value::Object(path_items),
    }))))
}

// ── work v3 retract（召回）：校验任务同 job + 本人持有该 job 已办 + 事务删除下游待办 ──

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2V3RetractWi {
    #[serde(default)]
    pub retractTaskList: Vec<String>,
}

pub async fn work_u2_v3_retract(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2V3RetractWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    if wi.retractTaskList.is_empty() {
        return Ok(Json(ActionResult::error("retractTaskList is required")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // 1) 所有任务存在且属于同一个 job（Java ExceptionEntityNotExist / 单 job 校验）
    let rows = client
        .query(
            "SELECT xid, xjob FROM \"pp_c_task\" WHERE xid = ANY($1)",
            &[&wi.retractTaskList],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if rows.len() != wi.retractTaskList.len() {
        return Ok(Json(ActionResult::error("task not found")));
    }
    let jobs: std::collections::HashSet<String> = rows
        .iter()
        .map(|r| r.get::<_, Option<String>>("xjob").unwrap_or_default())
        .collect();
    if jobs.len() != 1 {
        return Ok(Json(ActionResult::error("tasks must belong to the same job")));
    }
    let job = rows
        .iter()
        .find_map(|r| r.get::<_, Option<String>>("xjob"))
        .unwrap_or_default();
    // 2) IDOR：请求者必须持有该 job 的已办（joinInquire 语义以持有已办近似）
    let tc = client
        .query_opt(
            "SELECT id FROM \"pp_c_taskcompleted\" WHERE xjob = $1 AND xperson = $2 \
             ORDER BY create_time DESC LIMIT 1",
            &[&job, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if tc.is_none() {
        return Ok(Json(ActionResult::error("taskCompleted not found")));
    }
    // 3) 事务执行召回：移除被召回的下游任务并触碰工作更新时间
    let mut tx_client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = tx_client.transaction().await.map_err(|_| AppError::Internal)?;
    let deleted = tx
        .execute(
            "DELETE FROM \"pp_c_task\" WHERE xid = ANY($1)",
            &[&wi.retractTaskList],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute(
        "UPDATE \"pp_c_work\" SET \"xupdateTime\" = NOW() WHERE xjob = $1",
        &[&job],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    drop(client);
    Ok(Json(ActionResult::success(serde_json::json!({
        "retracted": deleted as i64, "job": job,
    }))))
}


// ── workcompleted shift time（调整完成时间）：owner/admin 门禁 + 真实 UPDATE ──

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2ShiftTimeWi {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub adjustMinutes: Option<i64>,
}

pub async fn workcompleted_u2_shift_time(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2ShiftTimeWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = wi.id.unwrap_or_default();
    let adjust = wi.adjustMinutes;
    if id.trim().is_empty() || adjust.is_none() {
        return Ok(Json(ActionResult::error("id and adjustMinutes are required")));
    }
    let adjust = adjust.unwrap();
    let gate = u2_check_owner(&pool, "\"pp_c_workcompleted\"", "\"creator_person\"", &id, &session.person_unique).await?;
    match gate {
        U2Gate::NotFound => return Ok(Json(ActionResult::error("workCompleted not found"))),
        U2Gate::Forbidden => return Err(AppError::Forbidden),
        U2Gate::Allowed => {}
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT \"xcompletedTime\" FROM \"pp_c_workcompleted\" WHERE xid = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let current: Option<Option<String>> = row.map(|r| r.get("xcompletedTime"));
    let Some(Some(text)) = current else {
        return Ok(Json(ActionResult::error("completedTime is not set")));
    };
    let parsed = ["%Y-%m-%d %H:%M:%S%.f", "%Y-%m-%dT%H:%M:%S%.f"]
        .iter()
        .find_map(|fmt| chrono::NaiveDateTime::parse_from_str(&text, fmt).ok());    let Some(current_time) = parsed else {
        return Ok(Json(ActionResult::error("unparsable completedTime")));
    };
    let shifted = current_time + chrono::Duration::minutes(adjust);
    let shifted_text = shifted.format("%Y-%m-%d %H:%M:%S").to_string();
    client
        .execute(
            "UPDATE \"pp_c_workcompleted\" SET \"xcompletedTime\" = $2, \"xupdateTime\" = NOW() \
             WHERE xid = $1",
            &[&id, &shifted_text],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "id": id, "completedTime": shifted_text,
    }))))
}

// ── snap upload / download ───────────────────────────────────────────────────

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2SnapUploadWi {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub job: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub work: Option<String>,
    #[serde(default)]
    pub workCompleted: Option<String>,
    #[serde(default)]
    pub application: Option<String>,
    #[serde(default)]
    pub applicationName: Option<String>,
    #[serde(default)]
    pub process: Option<String>,
    #[serde(default)]
    pub processName: Option<String>,
    #[serde(default)]
    pub person: Option<String>,
    #[serde(default)]
    pub identity: Option<String>,
}

pub async fn snap_u2_upload(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    wi): Json<U2SnapUploadWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    // Java check()：job 非空且（work 或 workCompleted 至少其一）——否则内容混淆异常
    let job = wi.job.clone().unwrap_or_default();
    let has_target = wi.work.is_some() || wi.workCompleted.is_some();
    if job.trim().is_empty() || !has_target {
        return Ok(Json(ActionResult::error("snap content is confused")));
    }
    let id = wi.id.clone().filter(|s| !s.trim().is_empty()).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO \"pp_c_snap\" (id, xid, xtitle, xjob, \"xwork\", \"xworkCompleted\", \
             \"xtype\", \"xperson\", \"xidentity\", \"xapplication\", \"xapplicationName\", \
             \"xprocess\", \"xprocessName\", creator_person, \"xcreateTime\", \"xupdateTime\") \
             VALUES ($1, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW()) \
             ON CONFLICT (id) DO NOTHING",
            &[
                &id,
                &wi.title,
                &Some(job),
                &wi.work,
                &wi.workCompleted,
                &wi.r#type,
                &wi.person,
                &wi.identity,
                &wi.application,
                &wi.applicationName,
                &wi.process,
                &wi.processName,
                &session.person_unique,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
    ])))))
}

pub async fn snap_u2_download(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match u2_check_owner(&pool, "\"pp_c_snap\"", "\"creator_person\"", &id, &session.person_unique).await? {
        U2Gate::NotFound => Ok(Json(ActionResult::error("snap not found"))),
        U2Gate::Forbidden => Err(AppError::Forbidden),
        U2Gate::Allowed => {
            let client = pool.get().await.map_err(|_| AppError::Internal)?;
            let row = client
                .query_opt(
                    &format!("SELECT {}, \"xdata\" FROM \"pp_c_snap\" WHERE id = $1", U2_SNAP_COLS),
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let Some(row) = row else {
                return Ok(Json(ActionResult::error("snap not found")));
            };
            let mut snap = u2_snap_json_full(&row);
            if let Value::Object(ref mut m) = snap {
                let process_name = m.get("processName").and_then(Value::as_str).unwrap_or("snap");
                let title = m.get("title").and_then(Value::as_str).unwrap_or("");
                // Java WoFile：以 processName-title 命名的快照归档下载
                m.insert(
                    "fileName".to_string(),
                    Value::String(format!("{}-{}.json", process_name, title)),
                );
            }
            Ok(Json(ActionResult::success(snap)))
        }
    }
}

// ── attachment 扩展名流式下载 ×4（matchit 单段单参限制：{fileName} 内含 name.ext） ──

async fn u2_attachment_ext_download(
    pool: &Pool,
    session: &shared::session::Session,
    id: &str,
    work_col: &str,
    work_id: &str,
    filename: &str,
) -> Result<axum::response::Response, AppError> {
    use axum::response::IntoResponse;
    // IDOR：归属门禁（owner 或 admin）
    match u2_check_owner(pool, "\"pp_c_attachment\"", "\"creator_person\"", id, &session.person_unique).await? {
        U2Gate::NotFound => {
            return Ok(Json(ActionResult::<Value>::error("attachment not found")).into_response());
        }
        U2Gate::Forbidden => return Err(AppError::Forbidden),
        U2Gate::Allowed => {}
    }
    // 附件必须真实挂载在 URL 所指的 work/workcompleted 上
    let where_clause = format!("id = $1 AND {work_col} = $2");
    let row = u2_att_load_blob_row(pool, &where_clause, id, Some(work_id)).await?;
    if row.is_none() {
        return Ok(Json(ActionResult::<Value>::error("attachment not bound to this work")).into_response());
    }
    // filename 段（形如 report.pdf）仅用于命名合法性校验；实际文件名取自元数据 xname
    let _ = filename.trim();
    u2_att_download_response(row, id).await
}

macro_rules! u2_att_ext_download_handler {
    ($fn_name:ident, $work_col:expr) => {
        pub async fn $fn_name(
            pool: Extension<Pool>,
            session: Extension<shared::session::Session>,
            axum::extract::Path((id, work_id, filename)):
                axum::extract::Path<(String, String, String)>,
        ) -> Result<axum::response::Response, AppError> {
            u2_attachment_ext_download(&pool, &session, &id, $work_col, &work_id, &filename).await
        }
    };
}

u2_att_ext_download_handler!(attachment_u2c_download_work_stream_ext, "\"xwork\"");
u2_att_ext_download_handler!(attachment_u2c_download_work_ext, "\"xwork\"");
u2_att_ext_download_handler!(attachment_u2c_download_wc_stream_ext, "\"xworkCompleted\"");
u2_att_ext_download_handler!(attachment_u2c_download_wc_ext, "\"xworkCompleted\"");

// ── invoice 文档信息/下载 ×2（StorageObject on x_general_invoice）──
// 对齐 o2server ActionGetInvoiceInfo / ActionDownloadInvoice（原 u2_capability_unavailable 桩已替换）。
// 权限：owner(xperson 或 creator) —— JobControlBuilder allowVisit 边缘情况记为语义留档。
// 依赖迁移 087_add_invoice_storage_columns.sql 为 x_general_invoice 补齐 xname/xstorage/xextension/xperson 等列。

async fn u2_invoice_check_owner(
    pool: &Pool,
    id: &str,
    person_unique: &str,
) -> Result<U2Gate, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT xperson, creator FROM x_general_invoice WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(U2Gate::NotFound),
        Some(r) => {
            let xperson: Option<String> = r.get("xperson");
            let creator: Option<String> = r.get("creator");
            let owner = xperson.as_deref() == Some(person_unique)
                || creator.as_deref() == Some(person_unique);
            if owner {
                Ok(U2Gate::Allowed)
            } else {
                Ok(U2Gate::Forbidden)
            }
        }
    }
}

// ── review filter/create/entry：person+creatorPerson 双作用域可建阅评入口清单 ──

pub async fn review_u2_filter_create_entry(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // Java ActionFilterCreateEntry：以 (xperson = me AND xcreatorPerson = me) 为作用域
    let me = &session.person_unique;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    async fn distinct_list(
        client: &deadpool_postgres::Client,
        col: &str,
        me: &str,
    ) -> Result<Vec<Value>, AppError> {
        let sql = format!(
            "SELECT DISTINCT {col} AS value FROM \"pp_c_review\" \
             WHERE xperson = $1 AND \"xcreatorPerson\" = $1 AND {col} IS NOT NULL AND {col} <> '' \
             ORDER BY 1"
        );
        let rows = client.query(&sql, &[&me]).await.map_err(|_| AppError::Internal)?;
        Ok(rows
            .iter()
            .map(|r| {
                let v: Option<String> = r.get("value");
                Value::Object(serde_json::Map::from_iter([
                    ("name".to_string(), Value::String(v.clone().unwrap_or_default())),
                    ("value".to_string(), Value::String(v.unwrap_or_default())),
                ]))
            })
            .collect())
    }
    let application_list = distinct_list(&client, "xapplication", me).await?;
    let process_list = distinct_list(&client, "xprocess", me).await?;
    let month_sql = "SELECT DISTINCT SUBSTRING(\"xstartTime\" FROM 1 FOR 7) AS value \
                     FROM \"pp_c_review\" WHERE xperson = $1 AND \"xcreatorPerson\" = $1 \
                     AND \"xstartTime\" IS NOT NULL ORDER BY 1";
    let month_rows = client.query(month_sql, &[&me]).await.map_err(|_| AppError::Internal)?;
    let start_month_list: Vec<Value> = month_rows
        .iter()
        .map(|r| {
            let v: Option<String> = r.get("value");
            Value::Object(serde_json::Map::from_iter([
                ("name".to_string(), Value::String(v.clone().unwrap_or_default())),
                ("value".to_string(), Value::String(v.unwrap_or_default())),
            ]))
        })
        .collect();
    let wo = serde_json::json!({
        "applicationList": application_list,
        "processList": process_list,
        "startTimeMonthList": start_month_list,
    });
    Ok(Json(ActionResult::success(wo)))
}

// ── route/list POST 别名（Java Wi.valueList → 按 id 批量取路由） ─────────────

#[derive(Debug, Default, Deserialize)]
#[allow(non_snake_case)]
pub struct U2RouteListWi {
    #[serde(default)]
    pub valueList: Vec<String>,
}

pub async fn route_u2_list_by_ids(
    pool: Extension<Pool>,
    wi): Json<U2RouteListWi>,
    ) -> Result<Json<ActionResult<Value>>, AppError> {
    if wi.valueList.is_empty() {
        return Ok(Json(ActionResult::java_success(Value::Array(Vec::new()), 0, 0)));    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xprocess, \"xcreateTime\", \"xupdateTime\" FROM \"pp_e_route\" \
             WHERE xid = ANY($1)",
            &[&wi.valueList],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), u2_s(row, "xid")),
                ("name".to_string(), u2_s(row, "xname")),
                ("process".to_string(), u2_s(row, "xprocess")),
                ("createTime".to_string(), u2_s(row, "xcreateTime")),
                ("updateTime".to_string(), u2_s(row, "xupdateTime")),
            ]))
        })
        .collect();
    { let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0))) }
}

#[cfg(test)]
mod tests_generated;

#[cfg(test)]
mod tests_u2;

pub async fn task_list_date_hour_exclude_draft_manage(
    pool: Extension<Pool>,
    axum::extract::Path((date, hour, is_exclude_draft)): axum::extract::Path<(String, String, String)>,
) -> Result<Json<ActionResult<Vec<Value>>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let _ = (hour, is_exclude_draft);
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, \"xcreateTime\" FROM PP_C_TASK \
             WHERE TO_CHAR(\"xcreateTime\", 'YYYY-MM-DD') = $1 ORDER BY \"xcreateTime\" DESC LIMIT 50",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("title".to_string(), Value::String(row.get("xtitle"))),
                ("person".to_string(), Value::String(row.get("xperson"))),
            ]))
        })
        .collect();
    let data_len = data.len() as i64;
    Ok(Json(ActionResult::java_success(data, data_len, 0)))
}

pub async fn task_list_person_exclude_draft_manage(
    pool: Extension<Pool>,
    axum::extract::Path((person, is_exclude_draft)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Vec<Value>>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let _ = is_exclude_draft;
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, \"xcreateTime\" FROM PP_C_TASK \
             WHERE xperson = $1 ORDER BY \"xcreateTime\" DESC LIMIT 50",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("title".to_string(), Value::String(row.get("xtitle"))),
                ("person".to_string(), Value::String(row.get("xperson"))),
            ]))
        })
        .collect();
    let data_len = data.len() as i64;
    Ok(Json(ActionResult::java_success(data, data_len, 0)))
}

