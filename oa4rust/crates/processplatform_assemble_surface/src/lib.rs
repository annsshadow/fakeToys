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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_my_filter_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn read_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_my_filter_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn readcompleted_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn record_list_job_job_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((job, page, size)): axum::extract::Path<(String, i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn record_list_workorworkcompleted_workOrWorkCompleted_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((workOrWorkCompleted, page, size)): axum::extract::Path<(String, i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn review_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn review_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn serialnumber_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_my_filter_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn task_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_my_filter_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_list_my_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_v2_list_create_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn taskcompleted_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn work_v2_list_paging_page_size_size(pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,) -> Result<Json<ActionResult<Value>>, AppError> {
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
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
                ("createTime".to_string(), Value::String(row.get("xcreateTime"))),
                ("updateTime".to_string(), Value::String(row.get("xupdateTime"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn snap_u2_list_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let mut rows = u2_snap_page(&pool, &id, count, true).await?;
    rows.reverse();
    let data: Vec<Value> = rows.iter().map(u2_snap_json).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn snap_u2_list_next_count_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_require_admin(&pool, &session).await?;
    let rows = u2_snap_page_all(&pool, &id, count, false).await?;
    let data: Vec<Value> = rows.iter().map(u2_snap_json_full).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
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
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
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
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = format!(
        "SELECT {} FROM \"pp_c_attachment\" WHERE {} ORDER BY \"xcreateTime\" DESC",
        U2_ATT_COLS, where_clause
    );
    let rows = client.query(&sql, &[&param]).await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(u2_att_json).collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn attachment_u2_list_job_job(
    pool: Extension<Pool>,
    axum::extract::Path(job): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_list(&pool, "\"xjob\" = $1", &job).await
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
    u2_att_list(&pool, "\"xwork\" = $1 OR \"xworkCompleted\" = $1", &flag).await
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
    Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ]))))
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
    Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ]))))
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
    Json(body): Json<U2B64UploadBody>,
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
    axum::extract::Path((_flag, _ref)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Err(u2_capability_unavailable("invoice parsing"))
}

pub async fn attachment_u2b_invoice_download(
    axum::extract::Path((_flag, _ref)): axum::extract::Path<(String, String)>,
) -> Result<axum::response::Response, AppError> {
    Err(u2_capability_unavailable("invoice parsing"))
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
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = u2_body_str(&body, &["fileName", "name"]);
    u2_att_update_meta(&pool, &session.person_unique, &id, &work, name.as_deref()).await
}

pub async fn attachment_u2b_update_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_by_work(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_update_callback(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, work, _cb)): axum::extract::Path<(String, String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = u2_body_str(&body, &["fileName", "name"]);
    u2_att_update_meta(&pool, &session.person_unique, &id, &work, name.as_deref()).await
}

pub async fn attachment_u2b_update_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_by_work(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_update_content(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path((id, work)): axum::extract::Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let text = u2_body_str(&body, &["content", "text", "fileContent"]);
    u2_att_update_text(&pool, &session.person_unique, &id, &work, text.as_deref()).await
}

pub async fn attachment_u2b_update_content_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_content(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_edit_by_work(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_by_work(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_edit_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_by_work(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_edit_text(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    attachment_u2b_update_content(pool, session, path, Json(body)).await
}

pub async fn attachment_u2b_edit_text_mockputtopost(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    path: axum::extract::Path<(String, String)>,
    Json(body): Json<Value>,
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
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_copy(&pool, &session.person_unique, &body, "\"xwork\"", &work, false).await
}

pub async fn attachment_u2b_copy_to_work_soft(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(work): axum::extract::Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_copy(&pool, &session.person_unique, &body, "\"xwork\"", &work, true).await
}

pub async fn attachment_u2b_copy_to_workcompleted(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(wc): axum::extract::Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_copy(&pool, &session.person_unique, &body, "\"xworkCompleted\"", &wc, false).await
}

pub async fn attachment_u2b_copy_to_workcompleted_soft(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(wc): axum::extract::Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_att_copy(&pool, &session.person_unique, &body, "\"xworkCompleted\"", &wc, true).await
}

pub async fn attachment_u2b_batch_delete_manage(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
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
    Json(body): Json<Value>,
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

#[cfg(test)]
mod tests_generated;

#[cfg(test)]
mod tests_u2;
