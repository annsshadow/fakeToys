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
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
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
            "SELECT xid, xname, xalias, xdescription, xapplicationCategory, xicon, xiconHue, xcreatorPerson, xlastUpdateTime, xlastUpdatePerson, xproperties, \"\"xcreateTime\"\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE 1=1 ORDER BY \"xcreateTime\" DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
            "SELECT xid, xname, xalias, xdescription, \"xapplicationCategory\", xicon, \"xiconHue\", \"xcreatorPerson\", \"xlastUpdateTime\", \"xlastUpdatePerson\", xproperties, \"xcreateTime\", \"xupdateTime\" FROM PP_E_APPLICATION WHERE xkey = $1 ORDER BY \"xcreateTime\" DESC",
            &[&key],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn application_flag_onlyRemoveNotCompleted(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("applicationdict not found"))),
    }
}

pub async fn applicationdict_applicationDictFlag_application_applicationFlag_path0_path1_path2_path3_path4_path5_path6_path7_data_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(applicationDictFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn correlation_list_job_job_site_site(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
    axum::extract::Path(site): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_job_job_path0_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_job_job_path0_path1_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_job_job_path0_path1_path2_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_job_job_path0_path1_path2_path3_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3_path4(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_job_job_path0_path1_path2_path3_path4_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_path6(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_path6_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_path6_path7(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_job_job_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_work_id_path0_mockdeletetoget(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_work_id_path0_path1_mockdeletetoget(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_work_id_path0_path1_path2_mockdeletetoget(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_work_id_path0_path1_path2_path3_mockdeletetoget(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_work_id_path0_path1_path2_path3_path4_mockdeletetoget(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_mockdeletetoget(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_mockdeletetoget(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_path7(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockdeletetoget(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn data_work_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_workcompleted_id_path0_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_workcompleted_id_path0_path1_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_workcompleted_id_path0_path1_path2_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn data_workcompleted_id_path0_path1_path2_path3_path4_path5_path6_path7_mockputtopost(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn datarecord_get_job_job_path_path(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn documentversion_list_job_job_category_category(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn documentversion_list_workorworkcompleted_workOrWorkCompleted_category_category(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("documentversion not found"))),
    }
}

pub async fn draft_list_my_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, xidentity, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", \"xprocessAlias\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_DRAFT WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn draft_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn draft_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn file_flag_application_applicationFlag_content(
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
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_flag_application_applicationFlag_download(
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
        Some(row) => {
            let data = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_flag_application_applicationFlag(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn form_flag_application_applicationFlag_mobile(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("form not found"))),
    }
}

pub async fn handover_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xtitle, xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_HANDOVER WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("job not found"))),
    }
}

pub async fn job_job_allow_visit_person_person(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("mode not found"))),
    }
}

pub async fn process_activity_activity_activityType_activityType(
    pool: Extension<Pool>,
    axum::extract::Path(activity): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_flag_application_applicationFlag(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("process not found"))),
    }
}

pub async fn process_flag_onlyRemoveNotCompleted(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("read not found"))),
    }
}

pub async fn read_list_filter_page_size_size_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_my_filter_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_my_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_id_next_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_id_next_count_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_id_next_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_id_prev_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_id_prev_count_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_id_prev_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_v2_list_create_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_v2_list_create_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_v2_list_create_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_v2_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xread, xtitle, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, \"xcreateTime\", \"xupdateTime\" FROM PP_C_READ WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_v2_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_v2_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("readcompleted not found"))),
    }
}

pub async fn readcompleted_list_filter_page_size_size_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_my_filter_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_my_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_id_next_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_id_next_count_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_id_next_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_id_prev_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_id_prev_count_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_id_prev_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_v2_list_create_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_v2_list_create_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_v2_list_create_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_v2_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, \"xstartTime\", \"xviewTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, xidentity, xunit, xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_READCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_v2_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_v2_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn record_list_job_job_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xjob = $1 ORDER BY \"xcreateTime\" DESC LIMIT $2::bigint OFFSET $3::bigint",
            &[&job, &size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn record_list_workorworkcompleted_workOrWorkCompleted_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(workOrWorkCompleted): axum::extract::Path<String>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xcreateTime\", \"xupdateTime\" FROM PP_C_RECORD WHERE xwork = $1 ORDER BY \"xcreateTime\" DESC LIMIT $2::bigint OFFSET $3::bigint",
            &[&workOrWorkCompleted, &size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn review_v2_list_create_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn review_v2_list_create_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn review_v2_list_create_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn review_v2_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xwork, \"xworkCompleted\", xcompleted, xtitle, xserial, \"xstartTime\", \"xcompletedTime\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xperson, \"xactivityUnique\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xopinion, \"xopinionLob\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_REVIEW WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn review_v2_list_paging_page_size_size_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_v2_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn review_v2_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_id_application_applicationFlag_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("review not found"))),
    }
}

pub async fn review_id_application_applicationFlag_manage_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("route not found"))),
    }
}

pub async fn script_flag_application_applicationFlag(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}

pub async fn script_flag_application_applicationFlag_imported(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn serialnumber_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xname, xapplication, \"xapplicationName\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", xperson, xidentity, \"xcreateTime\", \"xupdateTime\" FROM PP_C_SERIALNUMBER WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_filter_page_size_size_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_my_filter_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_my_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_id_next_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_id_next_count_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_id_next_count_filter_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_id_next_count_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_id_next_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_id_prev_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_id_prev_count_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_id_prev_count_filter_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_id_prev_count_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("task not found"))),
    }
}

pub async fn task_list_id_prev_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_v2_list_create_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_v2_list_create_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_v2_list_create_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_v2_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", xwork, xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityName\", \"xactivityType\", \"xactivityToken\", xperson, xidentity, xunit, \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", \"xexpireTime\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_v2_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_v2_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_list_date_date_hour_hour_manage(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("taskcompleted not found"))),
    }
}

pub async fn taskcompleted_list_filter_page_size_size_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_my_filter_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_my_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_id_next_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_id_next_count_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_id_next_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_id_prev_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_id_prev_count_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_id_prev_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_v2_list_create_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_v2_list_create_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_v2_list_create_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_v2_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcompletedTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xserial, xperson, \"xactivityUnique\", \"xcreateTime\", \"xupdateTime\" FROM PP_C_TASKCOMPLETED WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_v2_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_v2_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
            "UPDATE PP_C_TOUCH_EXPIRE SET \"xupdateTime\" = NOW()",
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
            "UPDATE PP_C_TOUCH_PASSEXPIRED SET \"xupdateTime\" = NOW()",
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
            "UPDATE PP_C_TOUCH_DETAINED SET \"xupdateTime\" = NOW()",
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

pub async fn work_application_applicationFlag_process_processFlag(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
            "SELECT COUNT(*) FROM PP_C_WORK WHERE xperson = $1",
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
            "SELECT COUNT(*) FROM PP_C_WORK WHERE xperson = $1",
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_filter_page_size_size_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_my_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_paging_page_size_size_application_applicationFlag_filter_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_next_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_next_count_application_applicationFlag_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_next_count_application_applicationFlag_filter_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_next_count_application_applicationFlag_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_next_count_creator_current(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_next_count_creator_current_filter(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_next_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_prev_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_prev_count_application_applicationFlag_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_prev_count_application_applicationFlag_filter_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_prev_count_application_applicationFlag_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("work not found"))),
    }
}

pub async fn work_list_id_prev_count_creator_current(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_prev_count_creator_current_filter(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_list_id_prev_count_process_processFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(processFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_v2_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path(size): axum::extract::Path<i64>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT xid, xjob, xtitle, \"xstartTime\", \"xcreatorPerson\", \"xcreatorIdentity\", \"xcreatorUnit\", xapplication, \"xapplicationName\", \"xapplicationAlias\", xprocess, \"xprocessName\", xactivity, \"xactivityType\", \"xactivityName\", \"xactivityAlias\", \"xactivityDescription\", \"xactivityToken\", \"xactivityUnique\", \"xactivityArrivedTime\", xserial, \"xcreateTime\", \"xupdateTime\" FROM PP_C_WORK WHERE 1=1 ORDER BY \"xcreateTime\" DESC LIMIT $1::bigint OFFSET $2::bigint",
            &[&size, &page],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("xid"))),
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_v2_list_id_next_count(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_v2_list_id_prev_count(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_filter_list_id_prev_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_filter_page_size_size_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_paging_page_size_size_application_applicationFlag_filter_manage(
    pool: Extension<Pool>,
    axum::extract::Path(page): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_id_next_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn workcompleted_list_id_next_count_application_applicationFlag_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn workcompleted_list_id_next_count_application_applicationFlag_filter_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_id_next_count_application_applicationFlag_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]));
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("workcompleted not found"))),
    }
}

pub async fn workcompleted_list_id_prev_count_application_applicationFlag(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn workcompleted_list_id_prev_count_application_applicationFlag_filter(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Path(applicationFlag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn workcompleted_list_id_prev_count_application_applicationFlag_manage(
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("\"xcreateTime\""))),
                ("updateTime".to_string(), Value::String(row.get("\"xupdateTime\""))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

#[cfg(test)]
mod tests_generated;
