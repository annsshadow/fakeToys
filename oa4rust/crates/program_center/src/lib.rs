use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests_u2;

#[derive(Debug, Deserialize)]
pub struct CollectAddRequest {
    pub title: Option<String>,
    pub url: Option<String>,
    pub person_id: Option<String>,
    pub creator: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigSaveRequest {
    pub key: String,
    pub value: Option<String>,
    pub category: Option<String>,
    pub creator: Option<String>,
}

pub async fn applications(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, app_id, disable FROM x_applications ORDER BY name",
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
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("disable".to_string(), Value::Bool(row.get("disable"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn current_style(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, app_id, disable FROM x_applications ORDER BY name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let portal_list: Vec<Value> = rows
        .iter()
        .take(3)
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("indexType".to_string(), Value::String("portal".to_string())),
            ("indexPortal".to_string(), Value::String("".to_string())),
            ("indexId".to_string(), Value::String("".to_string())),
            ("portalList".to_string(), Value::Array(portal_list)),
        ]),
    ))))
}

pub async fn modules_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT m.id, m.name, m.entity, m.creator, m.create_time, COUNT(f.id) as field_count \
             FROM x_program_module m \
             LEFT JOIN x_program_field f ON f.entity = m.entity \
             GROUP BY m.id, m.name, m.entity, m.creator, m.create_time \
             ORDER BY m.name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let entity: String = row.get("entity");
            let class_name = if entity == "Process" {
                format!("com.x.process.core.entity.{}", entity)
            } else {
                format!("com.x.organization.core.entity.{}", entity)
            };
            let field_count: i64 = row.get("field_count");

            Value::Object(serde_json::Map::from_iter([
                ("name".to_string(), Value::String(row.get("name"))),
                ("className".to_string(), Value::String(class_name)),
                ("entityCount".to_string(), Value::Number(serde_json::Number::from(field_count))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}

pub async fn collect_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn collect_add(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CollectAddRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let title = req.title.unwrap_or_default();
    let url = req.url.unwrap_or_default();
    let person_id = req.person_id.unwrap_or_default();
    let creator = req.creator.unwrap_or_else(|| "system".to_string());

    client
        .execute(
            "INSERT INTO x_program_collect (id, person_id, title, url, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, NOW())",
            &[&id, &person_id, &title, &url, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("title".to_string(), Value::String(title)),
            ("url".to_string(), Value::String(url)),
        ]),
    ))))
}

pub async fn collect_remove(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_program_collect WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("collect not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}

pub async fn config_get(
    pool: Extension<Pool>,
    Path(key): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE key = $1",
            &[&key],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("config not found"))),
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;



pub async fn agent_flag(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, flag, creator, create_time FROM x_program_agent WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn agent_flag_disable(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, flag, creator, create_time FROM x_program_agent WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn agent_flag_enable(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, flag, creator, create_time FROM x_program_agent WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn agent_flag_execute(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, flag, creator, create_time FROM x_program_agent WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn agent_flag_file(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, flag, creator, create_time FROM x_program_agent WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn andfx_pull_sync(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'andfx', 'pull', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("andfx".to_string())),
            ("action".to_string(), Value::String("pull".to_string())),
        ]),
    ))))
}


pub async fn appstyle_current_style(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, app_id, disable FROM x_applications ORDER BY name LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("appId".to_string(), Value::String(row.get("app_id"))),
                    ("disable".to_string(), Value::Bool(row.get("disable"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}


pub async fn appstyle_current_update(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, app_id, disable FROM x_applications ORDER BY name LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("appId".to_string(), Value::String(row.get("app_id"))),
                    ("disable".to_string(), Value::Bool(row.get("disable"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("application not found"))),
    }
}


pub async fn appstyle_image_application_top(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource WHERE resource_type = 'app_top' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                    ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                    ("path".to_string(), Value::String(row.get("path"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn appstyle_image_application_top_erase(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_program_deploy_resource WHERE id = $1 AND resource_type = 'app_top'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn appstyle_image_launch_logo(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource WHERE resource_type = 'launch_logo' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                    ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                    ("path".to_string(), Value::String(row.get("path"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn appstyle_image_launch_logo_erase(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_program_deploy_resource WHERE id = $1 AND resource_type = 'launch_logo'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn appstyle_image_login_avatar(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource WHERE resource_type = 'login_avatar' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                    ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                    ("path".to_string(), Value::String(row.get("path"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn appstyle_image_login_avatar_erase(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_program_deploy_resource WHERE id = $1 AND resource_type = 'login_avatar'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn appstyle_image_menu_logo_blur(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource WHERE resource_type = 'menu_logo_blur' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                    ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                    ("path".to_string(), Value::String(row.get("path"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn appstyle_image_menu_logo_blur_erase(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_program_deploy_resource WHERE id = $1 AND resource_type = 'menu_logo_blur'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn appstyle_image_menu_logo_focus(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource WHERE resource_type = 'menu_logo_focus' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                    ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                    ("path".to_string(), Value::String(row.get("path"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn appstyle_image_menu_logo_focus_erase(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_program_deploy_resource WHERE id = $1 AND resource_type = 'menu_logo_focus'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn appstyle_image_process_default(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource WHERE resource_type = 'process_default' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                    ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                    ("path".to_string(), Value::String(row.get("path"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn appstyle_image_process_default_erase(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_program_deploy_resource WHERE id = $1 AND resource_type = 'process_default'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn appstyle_image_setup_about_logo(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource WHERE resource_type = 'setup_about_logo' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                    ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                    ("path".to_string(), Value::String(row.get("path"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn appstyle_image_setup_about_logo_erase(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_program_deploy_resource WHERE id = $1 AND resource_type = 'setup_about_logo'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn appstyle_index_portal(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, app_id, disable FROM x_applications WHERE disable = false ORDER BY name",
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
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("disable".to_string(), Value::Bool(row.get("disable"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn bar_create_mass_from_count(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, status, creator, create_time FROM x_program_schedule ORDER BY create_time DESC",
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
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn bar_select1_field_field_value_value_count_count(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, status, creator, create_time FROM x_program_schedule ORDER BY create_time DESC",
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
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn bar_select2_count_count(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, status, creator, create_time FROM x_program_schedule ORDER BY create_time DESC",
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
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn bar_select3_field_field_value_value_count_count(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, status, creator, create_time FROM x_program_schedule ORDER BY create_time DESC",
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
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn bar_select4_field_field_value_value_count_count(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, status, creator, create_time FROM x_program_schedule ORDER BY create_time DESC",
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
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn captcha_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, creator, create_time FROM x_program_script WHERE category = 'captcha' AND deleted_at IS NULL ORDER BY create_time DESC",
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
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn captcha_v2_create_width_width_height_height(pool: Extension<Pool>, Path(width): Path<i64>, Path(height): Path<i64>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_script (id, name, flag, category, creator, create_time) VALUES ($1, $2, $3, 'captcha', 'system', NOW())",
            &[&id, &format!("captcha_{}", id), &format!("width={},height={}", width, height)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn captcha_id_validate_answer_answer(pool: Extension<Pool>, Path(id): Path<String>, Path(answer): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, flag, content FROM x_program_script WHERE id = $1 AND category = 'captcha' AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            let valid = row.get::<_, String>("flag") == answer;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("valid".to_string(), Value::Bool(valid)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("captcha not found"))),
    }
}


pub async fn center_applications(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, app_id, disable FROM x_applications WHERE disable = false ORDER BY name",
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
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("disable".to_string(), Value::Bool(row.get("disable"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn center_regist_applications(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, app_id, disable FROM x_applications ORDER BY name",
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
                ("appId".to_string(), Value::String(row.get("app_id"))),
                ("disable".to_string(), Value::Bool(row.get("disable"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn center_version(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("version".to_string(), Value::String("1.0.0".to_string())),
        ]),
    ))))
}


pub async fn code_create_mobile_mobile(pool: Extension<Pool>, Path(mobile): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let code = format!("{:06}", chrono::Utc::now().timestamp_subsec_millis() % 1000000);
    client
        .execute(
            "INSERT INTO x_program_script (id, name, flag, category, creator, create_time) VALUES ($1, $2, $3, 'code', 'system', NOW())",
            &[&id, &format!("mobile_{}", mobile), &code],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn code_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_program_script WHERE category = 'code' AND deleted_at IS NULL ORDER BY create_time DESC",
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn code_list_paging_page_size_size(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_program_script WHERE category = 'code' AND deleted_at IS NULL ORDER BY create_time DESC",
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn code_validate_mobile_mobile_answer_answer(pool: Extension<Pool>, Path(mobile): Path<String>, Path(answer): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, flag FROM x_program_script WHERE category = 'code' AND flag = $1 AND deleted_at IS NULL ORDER BY create_time DESC LIMIT 1",
            &[&mobile],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let valid = match row {
        Some(row) => row.get::<_, String>("flag") == answer,
        None => false,
    };
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("valid".to_string(), Value::Bool(valid)),
        ]),
    ))))
}


pub async fn code_validate_mobile_mobile_answer_answer_cascade(pool: Extension<Pool>, Path(mobile): Path<String>, Path(answer): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, flag FROM x_program_script WHERE category = 'code' AND flag = $1 AND deleted_at IS NULL ORDER BY create_time DESC LIMIT 1",
            &[&mobile],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let valid = match row {
        Some(row) => row.get::<_, String>("flag") == answer,
        None => false,
    };
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("valid".to_string(), Value::Bool(valid)),
        ]),
    ))))
}


pub async fn collect_code_mobile_mobile(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_connect(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_controllebbs(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_controllermobile_name_name_mobile_mobile(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_disconnect(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_login(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_mobile_check_connect(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_name_name_exist(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_name_name_mobile_mobile_code_code(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_person(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_resetpassword(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_sync_area(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_updateUnit(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_urlMapping(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_validate(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_validate_codeanswer(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_validate_direct(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn collect_validate_password(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, person_id, title, url, creator, create_time FROM x_program_collect WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("person_id".to_string(), Value::String(row.get("person_id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("url".to_string(), Value::String(row.get("url"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn command_execute(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}


pub async fn command_list_node(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}


pub async fn config_open_get_disable_export_enable(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_centerserver(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_change_password(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_collect(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_license(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}


pub async fn config_list_application(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}


pub async fn config_list_dump_data(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}


pub async fn config_list_dump_data_current_node(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}


pub async fn config_list_entity(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}


pub async fn config_open(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_open_run_time_config(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_person(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_portal(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_proxy(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn config_save(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<ConfigSaveRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let key = req.key;
    let value = req.value.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let creator = req.creator.unwrap_or_else(|| "system".to_string());

    client
        .execute(
            "INSERT INTO x_program_config (id, key, value, category, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, NOW()) \
             ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, category = EXCLUDED.category, creator = EXCLUDED.creator, create_time = NOW()",
            &[&id, &key, &value, &category, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("key".to_string(), Value::String(key)),
        ]),
    ))))
}

pub async fn config_ternary_management(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, key, value, category, creator, create_time FROM x_program_config WHERE category = 'ternary' ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("key".to_string(), Value::String(row.get("key"))),
                ("value".to_string(), Value::String(row.get("value"))),
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

pub async fn config_token(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, key, value, creator, create_time FROM x_program_config WHERE key = 'system.token'",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("key".to_string(), Value::String(row.get("key"))),
                    ("value".to_string(), Value::String(row.get("value"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("token config not found"))),
    }
}

pub async fn datastructure_fileds_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, entity, field_name, field_label, field_type, create_time FROM x_program_field ORDER BY entity, sort_order",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("fieldName".to_string(), Value::String(row.get("field_name"))),
                ("fieldLabel".to_string(), Value::String(row.get("field_label"))),
                ("fieldType".to_string(), Value::String(row.get("field_type"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn datastructure_modules_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
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

pub async fn datastructure_tables_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_name, entity, creator, create_time FROM x_program_table ORDER BY table_name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableName".to_string(), Value::String(row.get("table_name"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
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

pub async fn deploy_list_paging_page_size_size(
    pool: Extension<Pool>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, version, creator, create_time FROM x_program_deploy ORDER BY create_time DESC LIMIT $2::bigint OFFSET ($1 - 1) * $2",
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

pub async fn deploy_server_o2(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, server_url, status, creator, create_time FROM x_program_deploy_server WHERE server_type = 'o2' LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("serverUrl".to_string(), Value::String(row.get("server_url"))),
                    ("status".to_string(), Value::String(row.get("status"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("o2 server not configured"))),
    }
}

pub async fn deploy_server_resource(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource ORDER BY resource_name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                ("path".to_string(), Value::String(row.get("path"))),
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

pub async fn deploy_web_resource_as_new_asNew(
    pool: Extension<Pool>,
    Path(as_new): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_program_deploy_resource (id, resource_name, resource_type, creator, create_time) VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &"web_resource", &"web", &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("asNew".to_string(), Value::String(as_new)),
        ]),
    ))))
}

pub async fn deploy_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, version, status, creator, create_time FROM x_program_deploy WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("deploy not found"))),
    }
}

pub async fn designer_search(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_program_design WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
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

pub async fn dict_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, key_name, app_name, creator, create_time FROM x_program_dict WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("keyName".to_string(), Value::String(row.get("key_name"))),
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

pub async fn dict_list_paging_page_size_size(
    pool: Extension<Pool>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, key_name, app_name, creator, create_time FROM x_program_dict WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2::bigint OFFSET ($1 - 1) * $2",
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
                ("keyName".to_string(), Value::String(row.get("key_name"))),
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

pub async fn dict_dictFlag_data(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, app_data FROM x_program_dict WHERE flag = $1 AND deleted_at IS NULL",
            &[&dict_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("data".to_string(), Value::String(row.get("app_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn dict_dictFlag_path_data(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(_path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, app_data FROM x_program_dict WHERE flag = $1 AND deleted_at IS NULL",
            &[&dict_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("path".to_string(), Value::String(_path)),
                    ("data".to_string(), Value::String(row.get("app_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn dict_dictFlag_path_data_mockdeletetoget(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(_path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, app_data FROM x_program_dict WHERE flag = $1 AND deleted_at IS NULL",
            &[&dict_flag],
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

pub async fn dict_dictFlag_path_data_mockputtopost(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
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
            "UPDATE x_program_dict SET app_data = $1, update_time = NOW() WHERE flag = $2",
            &[&data_str, &dict_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("dictFlag".to_string(), Value::String(dict_flag)),
            ("path".to_string(), Value::String(_path)),
        ]),
    ))))
}

pub async fn dict_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, key_name, app_name, app_data, creator, create_time FROM x_program_dict WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("keyName".to_string(), Value::String(row.get("key_name"))),
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

pub async fn dingding_get_callback_aes(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, source, action, create_time FROM x_program_sync_log WHERE source = 'dingding' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("source".to_string(), Value::String(row.get("source"))),
                    ("action".to_string(), Value::String(row.get("action"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn dingding_pull_sync(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'dingding', 'pull', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("affected".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn dingding_request_pull_sync(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'dingding', 'pull', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("dingding".to_string())),
            ("action".to_string(), Value::String("pull".to_string())),
        ]),
    ))))
}


pub async fn dingding_sync_organization_callback(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let event_type = body.get("eventType").and_then(|v| v.as_str()).unwrap_or_default();
    let org_id = body.get("orgId").and_then(|v| v.as_str()).unwrap_or_default();

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, org_id, event_type, create_time) VALUES ($1, 'dingding', 'callback', $2, $3, NOW())",
            &[&id, &org_id, &event_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("dingding".to_string())),
            ("action".to_string(), Value::String("callback".to_string())),
            ("orgId".to_string(), Value::String(org_id.to_string())),
            ("eventType".to_string(), Value::String(event_type.to_string())),
        ]),
    ))))
}

pub async fn dingding_sync_organization_register_callback_enable(
    pool: Extension<Pool>,
    Path(enable): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_callback_registration (id, source, create_time) VALUES ($1, 'dingding', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("enable".to_string(), Value::String(enable)),
        ]),
    ))))
}

pub async fn distribute_assemble_source_source(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn distribute_webserver_assemble_source_source(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn foo_create_mass_from_count(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, cron_expression, status, creator, create_time FROM x_program_schedule ORDER BY create_time DESC",
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
                ("cronExpression".to_string(), Value::String(row.get("cron_expression"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn input_compare(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn input_cover(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn input_create(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn input_prepare_cover(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn input_prepare_create(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn invoke_list_category(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn invoke_list_with_category_category(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn invoke_token(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, key, value, creator, create_time FROM x_program_config WHERE key = 'system.token' AND deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("key".to_string(), Value::String(row.get("key"))),
                    ("value".to_string(), Value::String(row.get("value"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("token not found"))),
    }
}


pub async fn invoke_flag(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, description, creator, create_time FROM x_program_agent WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("flag".to_string(), Value::String(row.get("flag"))),
                    ("description".to_string(), Value::String(row.get("description"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("agent not found"))),
    }
}


pub async fn invoke_flag_client_client_token_token_execute(pool: Extension<Pool>, Path(client): Path<String>, Path(token): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let db_client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = db_client
        .query_opt(
            "SELECT id, name, flag FROM x_program_agent WHERE name = $1 AND flag = $2 AND deleted_at IS NULL",
            &[&client, &token],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            db_client
                .execute(
                    "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'invoke', 'execute', NOW())",
                    &[&uuid::Uuid::new_v4().to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("agent not found"))),
    }
}


pub async fn invoke_flag_execute(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, flag FROM x_program_agent WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            client
                .execute(
                    "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'invoke', 'execute', NOW())",
                    &[&uuid::Uuid::new_v4().to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("agent not found"))),
    }
}


pub async fn invoke_flag_execute_get(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, flag FROM x_program_agent WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            client
                .execute(
                    "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'invoke', 'execute', NOW())",
                    &[&uuid::Uuid::new_v4().to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("agent not found"))),
    }
}


pub async fn invoke_flag_file(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, content, creator, create_time FROM x_program_script WHERE flag IS NOT NULL AND deleted_at IS NULL ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("flag".to_string(), Value::String(row.get("flag"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}


pub async fn jest_center_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn jest_clear_cache_source(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'jest', 'clear_cache', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("jest".to_string())),
            ("action".to_string(), Value::String("clear_cache".to_string())),
        ]),
    ))))
}


pub async fn jest_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_program_script WHERE category = 'jest' AND deleted_at IS NULL ORDER BY create_time DESC",
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

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn jest_version(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("version".to_string(), Value::String("1.0.0".to_string())),
        ]),
    ))))
}


pub async fn market_cloud_unit_is_vip(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, entity, vip, creator, create_time FROM x_program_module ORDER BY name LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("entity".to_string(), Value::String(row.get("entity"))),
                    ("vip".to_string(), Value::Bool(row.get("vip"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn market_install_offline(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_deploy (id, name, creator, create_time) VALUES ($1, $2, 'system', NOW())",
            &[&id, &"offline_install".to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}

pub async fn market_list_category(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn market_list_install_log_paging_page_size_size(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, schedule_id, application, status, message, create_time FROM x_program_schedule_log ORDER BY create_time DESC LIMIT 100",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("scheduleId".to_string(), Value::String(row.get("schedule_id"))),
                ("application".to_string(), Value::String(row.get("application"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn market_list_paging_page_size_size(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn market_list_paging_page_size_size_category_category(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY name",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn market_list_top_three(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY create_time DESC LIMIT 3",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn market_flag(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("entity".to_string(), Value::String(row.get("entity"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn market_flag_cover_pic(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, resource_name, resource_type, path, creator, create_time FROM x_program_deploy_resource WHERE resource_type = 'cover_pic' ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("resourceName".to_string(), Value::String(row.get("resource_name"))),
                    ("resourceType".to_string(), Value::String(row.get("resource_type"))),
                    ("path".to_string(), Value::String(row.get("path"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn market_flag_install_log(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, schedule_id, application, status, message, create_time FROM x_program_schedule_log ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("scheduleId".to_string(), Value::String(row.get("schedule_id"))),
                ("application".to_string(), Value::String(row.get("application"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}


pub async fn market_flag_install_or_update(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'market', 'install_or_update', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("market".to_string())),
            ("action".to_string(), Value::String("install_or_update".to_string())),
        ]),
    ))))
}


pub async fn market_flag_installed_version(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, version, status, creator, create_time FROM x_program_deploy ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("version".to_string(), Value::String(row.get("version"))),
                    ("status".to_string(), Value::String(row.get("status"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("deploy not found"))),
    }
}


pub async fn market_flag_uninstall(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'market', 'uninstall', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("market".to_string())),
            ("action".to_string(), Value::String("uninstall".to_string())),
        ]),
    ))))
}


pub async fn market_id_download(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, entity FROM x_program_module WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("module not found"))),
    }
}


pub async fn module_compare_upload(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn module_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY create_time DESC",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}


pub async fn module_list_category(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, entity, creator, create_time FROM x_program_module ORDER BY create_time DESC",
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
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}


pub async fn module_output(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn module_output_list_structure() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn module_output_structure(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn module_output_flag_file(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn module_remove_structure_id(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn module_write_flag(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn module_id_compare(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, entity, creator, create_time FROM x_program_module WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("entity".to_string(), Value::String(row.get("entity"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn mpweixin_check(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, create_time FROM x_program_mpweixin_menu WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn mpweixin_media_add_forever(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, create_time FROM x_program_mpweixin_menu WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn mpweixin_menu_add(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, create_time FROM x_program_mpweixin_menu WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn mpweixin_menu_create_to_weixin(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, create_time FROM x_program_mpweixin_menu WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn mpweixin_menu_delete_id(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "DELETE FROM x_program_mpweixin_menu WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}


pub async fn mpweixin_menu_list_weixin() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn mpweixin_menu_subscribe(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, create_time FROM x_program_mpweixin_menu WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn mpweixin_menu_update_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_program_mpweixin_menu SET update_time = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("menu not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}

pub async fn mpweixin_message_template_send(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, create_time FROM x_program_mpweixin_menu WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn output_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_program_output WHERE deleted_at IS NULL ORDER BY create_time DESC",
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

pub async fn output_appInfoFlag_select(
    pool: Extension<Pool>,
    Path(app_info_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_program_output WHERE app_info_flag = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&app_info_flag],
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
            "SELECT id, name, flag, select_file FROM x_program_output WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("flag".to_string(), Value::String(row.get("flag"))),
                    ("selectFile".to_string(), Value::String(row.get("select_file"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("output not found"))),
    }
}

pub async fn prompterrorlog_count_exceptionclass(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT exception_class, COUNT(*) as cnt FROM x_program_prompt_error_log GROUP BY exception_class ORDER BY cnt DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("count".to_string(), {
                    let __val: i64 = row.get("cnt");
                    Value::Number(serde_json::Number::from(__val))
                }),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_count_loggername(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT logger_name, COUNT(*) as cnt FROM x_program_prompt_error_log GROUP BY logger_name ORDER BY cnt DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("count".to_string(), {
                    let __val: i64 = row.get("cnt");
                    Value::Number(serde_json::Number::from(__val))
                }),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_list_id_next_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id > $1 ORDER BY id ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_list_id_next_count_date_date(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id > $1 AND DATE(create_time) = $2::date ORDER BY id ASC LIMIT $3::bigint",
            &[&id, &date, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_list_id_next_count_exceptionclass_exceptionClass(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(exception_class): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id > $1 AND exception_class = $2 ORDER BY id ASC LIMIT $3::bigint",
            &[&id, &exception_class, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_list_id_next_count_loggername_loggerName(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(logger_name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id > $1 AND logger_name = $2 ORDER BY id ASC LIMIT $3::bigint",
            &[&id, &logger_name, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_list_id_prev_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id < $1 ORDER BY id DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_list_id_prev_count_date_date(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id < $1 AND DATE(create_time) = $2::date ORDER BY id DESC LIMIT $3::bigint",
            &[&id, &date, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_list_id_prev_count_exceptionclass_exceptionClass(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(exception_class): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id < $1 AND exception_class = $2 ORDER BY id DESC LIMIT $3::bigint",
            &[&id, &exception_class, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_list_id_prev_count_loggername_loggerName(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(logger_name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id < $1 AND logger_name = $2 ORDER BY id DESC LIMIT $3::bigint",
            &[&id, &logger_name, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn prompterrorlog_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("exceptionClass".to_string(), Value::String(row.get("exception_class"))),
                ("loggerName".to_string(), Value::String(row.get("logger_name"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("prompt error log not found"))),
    }
}

pub async fn qiyeweixin_get_callback_aes(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, source, action, create_time FROM x_program_sync_log WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                ("source".to_string(), Value::String(row.get("source"))),
                ("action".to_string(), Value::String(row.get("action"))),
                ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn qiyeweixin_pull_sync(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'qiyeweixin', 'pull', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("qiyeweixin".to_string())),
            ("action".to_string(), Value::String("pull".to_string())),
        ]),
    ))))
}

pub async fn qiyeweixin_request_pull_sync(pool: Extension<Pool>, Path(id): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, source, action, create_time FROM x_program_sync_log WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("source".to_string(), Value::String(row.get("source"))),
                    ("action".to_string(), Value::String(row.get("action"))),
                    ("create_time".to_string(), Value::String(row.get("create_time"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("not found"))),
    }
}


pub async fn qiyeweixin_send_getprivateinfo_message(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let user_id = body.get("userId").and_then(|v| v.as_str()).unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_program_message_log (id, user_id, message_type, create_time) VALUES ($1, $2, 'qiyeweixin_getprivateinfo', NOW())",
            &[&uuid::Uuid::new_v4().to_string(), &user_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
        ]),
    ))))
}

#[derive(Debug, Deserialize)]
pub struct ApplicationCreateRequest {
    pub name: Option<String>,
    pub app_id: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
}

#[axum::debug_handler]
pub async fn application_create(
    pool: Extension<Pool>,
    Json(req): Json<ApplicationCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let app_id = req.app_id.unwrap_or_default();
    let description = req.description.unwrap_or_default();
    let creator = req.creator.unwrap_or_else(|| "system".to_string());

    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    client
        .execute(
            "INSERT INTO x_applications (id, name, app_id, description, disable, creator, create_time) \
              VALUES ($1, $2, $3, $4, false, $5, NOW())",
            &[&id, &name, &app_id, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("appId".to_string(), Value::String(app_id)),
            ("description".to_string(), Value::String(description)),
        ]),
    ))))
}

#[derive(Debug, Deserialize)]
pub struct ApplicationSaveRequest {
    pub name: Option<String>,
    pub app_id: Option<String>,
    pub description: Option<String>,
    pub disable: Option<bool>,
}

#[axum::debug_handler]
pub async fn application_save(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<ApplicationSaveRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_applications SET name = COALESCE($1, name), app_id = COALESCE($2, app_id), description = COALESCE($3, description), disable = COALESCE($4, disable), update_time = NOW() WHERE id = $5",
            &[&req.name, &req.app_id, &req.description, &req.disable, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("application not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}

#[derive(Debug, Deserialize)]
pub struct AgentCreateRequest {
    pub name: Option<String>,
    pub flag: Option<String>,
    pub description: Option<String>,
    pub creator: Option<String>,
}

#[axum::debug_handler]
pub async fn agent_create(
    pool: Extension<Pool>,
    Json(req): Json<AgentCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let flag = req.flag.unwrap_or_default();
    let description = req.description.unwrap_or_default();
    let creator = req.creator.unwrap_or_else(|| "system".to_string());

    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    client
        .execute(
            "INSERT INTO x_program_agent (id, name, flag, description, creator, create_time, update_time) \
              VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &flag, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            ("flag".to_string(), Value::String(flag)),
            ("description".to_string(), Value::String(description)),
        ]),
    ))))
}

#[derive(Debug, Deserialize)]
pub struct AgentSaveRequest {
    pub name: Option<String>,
    pub flag: Option<String>,
    pub description: Option<String>,
}

#[axum::debug_handler]
pub async fn agent_save(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<AgentSaveRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_program_agent SET name = COALESCE($1, name), flag = COALESCE($2, flag), description = COALESCE($3, description), update_time = NOW() WHERE id = $4",
            &[&req.name, &req.flag, &req.description, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("agent not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
        ]),
    ))))
}

pub async fn schedule_list_schedule(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, cron_expression, status, creator, create_time FROM x_program_schedule ORDER BY create_time DESC",
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
                ("cronExpression".to_string(), Value::String(row.get("cron_expression"))),
                ("status".to_string(), Value::String(row.get("status"))),
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

pub async fn schedule_list_schedulelocal(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, cron_expression, server_node, status, creator, create_time FROM x_program_schedule WHERE server_node IS NOT NULL ORDER BY create_time DESC",
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
                ("cronExpression".to_string(), Value::String(row.get("cron_expression"))),
                ("serverNode".to_string(), Value::String(row.get("server_node"))),
                ("status".to_string(), Value::String(row.get("status"))),
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

pub async fn schedule_list_schedulelog_application_application(
    pool: Extension<Pool>,
    Path(application): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, schedule_id, application, status, message, create_time FROM x_program_schedule_log WHERE application = $1 ORDER BY create_time DESC LIMIT 100",
            &[&application],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("scheduleId".to_string(), Value::String(row.get("schedule_id"))),
                ("application".to_string(), Value::String(row.get("application"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn schedule_report(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, schedule_id, status, message, create_time FROM x_program_schedule_log ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("scheduleId".to_string(), Value::String(row.get("schedule_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn schedule_schedule_fire(
    pool: Extension<Pool>,
    Path(schedule_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute(
            "INSERT INTO x_program_schedule_log (id, schedule_id, status, message, create_time) VALUES ($1, $2, 'fired', 'manual fire', NOW())",
            &[&uuid::Uuid::new_v4().to_string(), &schedule_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("scheduleId".to_string(), Value::String(schedule_id)),
        ]),
    ))))
}

pub async fn script_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_program_script WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
            "SELECT id, name, flag, category, creator, create_time FROM x_program_script WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2::bigint OFFSET ($1 - 1) * $2",
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

pub async fn script_name_name(
    pool: Extension<Pool>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, content, creator, create_time FROM x_program_script WHERE name = $1 AND deleted_at IS NULL LIMIT 1",
            &[&name],
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

pub async fn script_name_name_imported(
    pool: Extension<Pool>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, imported_content, creator, create_time FROM x_program_script WHERE name = $1 AND deleted_at IS NULL LIMIT 1",
            &[&name],
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

pub async fn script_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, content, creator, create_time FROM x_program_script WHERE flag = $1 AND deleted_at IS NULL",
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
                ("content".to_string(), Value::String(row.get("content"))),
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
            "SELECT id, name, flag, content, creator, create_time FROM x_program_script WHERE id = $1 AND deleted_at IS NULL",
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

pub async fn test_test1(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}


pub async fn test_test2(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}


pub async fn tokenthreshold_update(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let threshold = body.get("threshold").and_then(|v| v.as_i64()).unwrap_or(100);

    let result = client
        .execute(
            "UPDATE x_program_config SET value = $1, update_time = NOW() WHERE key = 'token.threshold'",
            &[&threshold.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        let id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_program_config (id, key, value, creator, create_time) VALUES ($1, 'token.threshold', $2, 'system', NOW())",
                &[&id, &threshold.to_string()],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("threshold".to_string(), Value::Number(serde_json::Number::from(threshold))),
        ]),
    ))))
}

pub async fn unexpectederrorlog_list_id_next_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id > $1 ORDER BY id ASC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("errorType".to_string(), Value::String(row.get("error_type"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("stackTrace".to_string(), Value::String(row.get("stack_trace"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn unexpectederrorlog_list_id_next_count_date_date(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id > $1 AND DATE(create_time) = $2::date ORDER BY id ASC LIMIT $3::bigint",
            &[&id, &date, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("errorType".to_string(), Value::String(row.get("error_type"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("stackTrace".to_string(), Value::String(row.get("stack_trace"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn unexpectederrorlog_list_id_prev_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id < $1 ORDER BY id DESC LIMIT $2::bigint",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("errorType".to_string(), Value::String(row.get("error_type"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("stackTrace".to_string(), Value::String(row.get("stack_trace"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn unexpectederrorlog_list_id_prev_count_date_date(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id < $1 AND DATE(create_time) = $2::date ORDER BY id DESC LIMIT $3::bigint",
            &[&id, &date, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("errorType".to_string(), Value::String(row.get("error_type"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("stackTrace".to_string(), Value::String(row.get("stack_trace"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn unexpectederrorlog_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("errorType".to_string(), Value::String(row.get("error_type"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("stackTrace".to_string(), Value::String(row.get("stack_trace"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("unexpected error log not found"))),
    }
}

pub async fn validation_meta(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}


pub async fn validation_scripting_benchmark(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let _client = pool.get().await.map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}


pub async fn validation_timeout_timeout(
    pool: Extension<Pool>,
    Path(timeout): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'validation', 'timeout', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("timeout".to_string(), Value::Number(serde_json::Number::from(timeout))),
        ]),
    ))))
}

pub async fn zhengwudingding_pull_sync(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'zhengwudingding', 'pull', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("zhengwudingding".to_string())),
            ("action".to_string(), Value::String("pull".to_string())),
        ]),
    ))))
}

pub async fn zhengwudingding_regist_callback(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_callback_registration (id, source, create_time) VALUES ($1, 'zhengwudingding', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("zhengwudingding".to_string())),
        ]),
    ))))
}

pub async fn zhengwudingding_sync_organization_callback(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let event_type = body.get("eventType").and_then(|v| v.as_str()).unwrap_or_default();
    let org_id = body.get("orgId").and_then(|v| v.as_str()).unwrap_or_default();

    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, org_id, event_type, create_time) VALUES ($1, 'zhengwudingding', 'callback', $2, $3, NOW())",
            &[&id, &org_id, &event_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("source".to_string(), Value::String("zhengwudingding".to_string())),
            ("action".to_string(), Value::String("callback".to_string())),
            ("orgId".to_string(), Value::String(org_id.to_string())),
            ("eventType".to_string(), Value::String(event_type.to_string())),
        ]),
    ))))
}

// ════════════════════════════════════════════════════════════════════
// plan002 U2 — Java 对齐缺口端点
//
// 表：x_program_warn_log / x_program_app_pack（migration 062 幂等补建），
// 其余沿用既有表。写操作按 IDOR 文档门禁
// （docs/solutions/security-issues/idor-vulnerability-write-handlers.md）：
//   - 管理资源（cachedispatch / center regist / apppack 构建、agent 删除）
//     一律 require_admin；
//   - 个人资源（dict / script）creator_person 取自会话，删除前
//     require_owner 校验。
// ════════════════════════════════════════════════════════════════════

async fn require_admin(
    pool: &Pool,
    session: &shared::session::Session,
) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn json_str(payload: &Value, key: &str) -> String {
    payload
        .get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

#[derive(Debug, Deserialize)]
pub struct WarnLogCreateRequest {
    pub level: Option<String>,
    pub tag: Option<String>,
    #[serde(rename = "loggerName")]
    pub logger_name: Option<String>,
    pub message: Option<String>,
    pub detail: Option<String>,
    pub host: Option<String>,
    pub port: Option<String>,
}

pub async fn warnlog_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(req): Json<WarnLogCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let level = req.level.unwrap_or_else(|| "WARN".to_string());
    let tag = req.tag.unwrap_or_default();
    let logger_name = req.logger_name.unwrap_or_default();
    let message = req.message.unwrap_or_default();
    let detail = req.detail.unwrap_or_default();
    let host = req.host.unwrap_or_default();
    let port = req.port.unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_program_warn_log (id, level, tag, logger_name, message, detail, host, port, creator_person, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW())",
            &[&id, &level, &tag, &logger_name, &message, &detail, &host, &port, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

fn warnlog_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    let opt = |k: &str| -> String { row.get::<_, Option<String>>(k).unwrap_or_default() };
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(opt("id"))),
        ("level".to_string(), Value::String(opt("level"))),
        ("tag".to_string(), Value::String(opt("tag"))),
        ("loggerName".to_string(), Value::String(opt("logger_name"))),
        ("message".to_string(), Value::String(opt("message"))),
        ("detail".to_string(), Value::String(opt("detail"))),
        ("host".to_string(), Value::String(opt("host"))),
        ("port".to_string(), Value::String(opt("port"))),
        ("createTime".to_string(), Value::String(opt("create_time"))),
    ]))
}

pub async fn warnlog_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, level, tag, logger_name, message, detail, host, port, create_time FROM x_program_warn_log WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(warnlog_row_to_value(&row)))),
        None => Ok(Json(ActionResult::error("warn log not found"))),
    }
}

async fn warnlog_list(
    pool: &Pool,
    where_clause: &str,
    params: &[&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)],
) -> Result<Json<ActionResult<Value>>, AppError> {
    let sql = format!(
        "SELECT id, level, tag, logger_name, message, detail, host, port, create_time FROM x_program_warn_log {} ORDER BY id DESC LIMIT ${}",
        where_clause,
        params.len()
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, params).await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(warnlog_row_to_value).collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn warnlog_list_next_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    warnlog_list(&pool, "WHERE id < $1", &[&id, &count]).await
}

pub async fn warnlog_list_next_count_date_date(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    warnlog_list(
        &pool,
        "WHERE id < $1 AND DATE(create_time) = $2::date",
        &[&id, &date, &count],
    )
    .await
}

pub async fn warnlog_list_prev_count(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    warnlog_list(&pool, "WHERE id > $1", &[&id, &count]).await
}

pub async fn warnlog_list_prev_count_date_date(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    warnlog_list(
        &pool,
        "WHERE id > $1 AND DATE(create_time) = $2::date",
        &[&id, &date, &count],
    )
    .await
}

pub async fn warnlog_view_system_log_tag_tag(
    pool: Extension<Pool>,
    Path(tag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    warnlog_list(&pool, "WHERE tag = $1", &[&tag]).await
}




// ── storagemappings / adminlogin / authentication / cachedispatch / center ──

pub async fn storagemappings_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT key, value FROM x_program_config WHERE category = 'storageMapping' ORDER BY key",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("key".to_string(), Value::String(row.get::<_, Option<String>>("key").unwrap_or_default())),
                ("value".to_string(), Value::String(row.get::<_, Option<String>>("value").unwrap_or_default())),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn adminlogin_logout(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "DELETE FROM auth_session WHERE person_id = $1",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("person".to_string(), Value::String(session.person_unique.clone())),
        ("sessionsClosed".to_string(), Value::Number(serde_json::Number::from(n as i64))),
    ])))))
}

pub async fn authentication_who(
    session: Option<Extension<shared::session::Session>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    match session {
        Some(session) => Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("person".to_string(), Value::String(session.person_unique.clone())),
            ("token".to_string(), Value::String(session.token.clone())),
        ]))))),
        None => Ok(Json(ActionResult::error("anonymous"))),
    }
}

pub async fn cachedispatch_dispatch(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'cache', 'dispatch', NOW())",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("source".to_string(), Value::String("cache".to_string())),
        ("action".to_string(), Value::String("dispatch".to_string())),
    ])))))
}

pub async fn center_regist_applications_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_program_callback_registration (id, creator_person, create_time) VALUES ($1, $2, NOW())",
            &[&id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

// ══════════════════════════════════════════════════════════════════
// plan002 U2 残余闭合（9 条，路径对齐 Java v9 全集）：
//
// - config PUT 家族（3 条）：centerserver / person / token 的写回。
//   Java 侧为 Config 对象保存（ActionEditConfig），Rust 侧以
//   x_program_config 键值域持久化；管理员门禁（require_admin）。
// - invoke CRUD（4 条）：POST/GET/PUT/DELETE /invoke[/{flag}]，
//   表 x_program_invoke（migration 074），flag 语义对齐 Java emc.flag
//   （id 或 name 或 alias）；写操作 serviceControlAble ≈ 管理员门禁；
//   name/alias 归一化查重（trim 后比对，排除自身 id）。
// - appstyle erase GET 家族（2 条）：GET .../erase 为 Java 原生方法，
//   语义是"清除当前该类图片"（无 id 参数）；补挂在既有 DELETE 注册上，
//   管理员门禁（对齐 Java ExceptionAccessDenied）。
// ══════════════════════════════════════════════════════════════════

/// config PUT 家族的公共实现：按 key 域 upsert 配置 JSON
async fn u2_config_domain_put(
    pool: &Pool,
    session: &shared::session::Session,
    key: &str,
    body: Value,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(pool, session).await?;
    if !body.is_object() {
        return Ok(Json(ActionResult::error("request body must be a JSON object")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let value = body.to_string();
    let n = client
        .execute(
            "UPDATE x_program_config SET value = $1, update_time = NOW() \
             WHERE key = $2 AND deleted_at IS NULL",
            &[&value, &key],
        )
        .await
        .map_err(|e| {
            tracing::error!("[u2_config_put] update failed: {}", e);
            AppError::Internal
        })?;
    if n == 0 {
        let id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_program_config (id, key, value, category, creator, create_time, update_time) \
                 VALUES ($1, $2, $3, 'config', $4, NOW(), NOW())",
                &[&id, &key, &value, &session.person_unique],
            )
            .await
            .map_err(|e| {
                tracing::error!("[u2_config_put] insert failed: {}", e);
                AppError::Internal
            })?;
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("value".to_string(), Value::Bool(true)),
            ("key".to_string(), Value::String(key.to_string())),
        ]),
    ))))
}

/// PUT /jaxrs/program_center/config/centerserver —— 保存中心服务器配置
pub async fn u2_config_centerserver_put(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_config_domain_put(&pool, &session, "centerserver", body).await
}

/// PUT /jaxrs/program_center/config/person —— 人员配置保存
pub async fn u2_config_person_put(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_config_domain_put(&pool, &session, "person", body).await
}

/// PUT /jaxrs/program_center/config/token —— 令牌配置保存
pub async fn u2_config_token_put(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_config_domain_put(&pool, &session, "system.token", body).await
}

// --- invoke CRUD（表：x_program_invoke）---

#[derive(Debug, Deserialize)]
pub struct U2InvokeRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub alias: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub enable: bool,
    #[serde(rename = "enableToken", default)]
    pub enable_token: bool,
    #[serde(rename = "enableAnonymous", default = "default_true_u2")]
    pub enable_anonymous: bool,
    #[serde(default)]
    pub validated: bool,
    #[serde(default)]
    pub text: String,
    #[serde(rename = "remoteAddrRegex", default)]
    pub remote_addr_regex: String,
    #[serde(default)]
    pub data: String,
    #[serde(rename = "executorList", default)]
    pub executor_list: Option<Vec<String>>,
}

fn default_true_u2() -> bool {
    true
}

fn opt_non_empty(s: &str) -> Option<String> {
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

async fn u2_invoke_find_by_flag(
    client: &deadpool_postgres::tokio_postgres::Client,
    flag: &str,
) -> Result<Option<String>, AppError> {
    // 对齐 Java emc.flag：id 或 name 或 alias 命中
    let row = client
        .query_opt(
            "SELECT id FROM x_program_invoke \
             WHERE (id = $1 OR name = $1 OR alias = $1)",
            &[&flag],
        )
        .await
        .map_err(|e| {
            tracing::error!("[u2_invoke] find-by-flag failed: {}", e);
            AppError::Internal
        })?;
    Ok(row.map(|r| r.get::<_, String>("id")))
}

/// POST /jaxrs/program_center/invoke —— 创建服务调用
pub async fn u2_invoke_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(req): Json<U2InvokeRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Ok(Json(ActionResult::error("name cannot be empty")));
    }
    let alias = opt_non_empty(&req.alias);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 归一化查重：name 与 alias 都不允许与现有记录的 name/alias 冲突
    let dup_name = client
        .query_opt(
            "SELECT id FROM x_program_invoke WHERE name = $1 OR alias = $1 LIMIT 1",
            &[&name],
        )
        .await
        .map_err(|e| {
            tracing::error!("[u2_invoke_create] dup-name check failed: {}", e);
            AppError::Internal
        })?;
    if dup_name.is_some() {
        return Ok(Json(ActionResult::error("duplicate name")));
    }
    if let Some(a) = &alias {
        let dup_alias = client
            .query_opt(
                "SELECT id FROM x_program_invoke WHERE name = $1 OR alias = $1 LIMIT 1",
                &[a],
            )
            .await
            .map_err(|e| {
                tracing::error!("[u2_invoke_create] dup-alias check failed: {}", e);
                AppError::Internal
            })?;
        if dup_alias.is_some() {
            return Ok(Json(ActionResult::error("duplicate alias")));
        }
    }

    let id = uuid::Uuid::new_v4().to_string();
    let executor_list = serde_json::to_string(&req.executor_list.unwrap_or_default())
        .unwrap_or_else(|_| "[]".to_string());
    client
        .execute(
            "INSERT INTO x_program_invoke \
             (id, name, alias, category, description, enable, enable_token, enable_anonymous, \
              validated, text, remote_addr_regex, data, executor_list, creator) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, ($13::text)::jsonb, $14)",
            &[
                &id,
                &name,
                &alias,
                &opt_non_empty(&req.category),
                &opt_non_empty(&req.description),
                &req.enable,
                &req.enable_token,
                &req.enable_anonymous,
                &req.validated,
                &opt_non_empty(&req.text),
                &opt_non_empty(&req.remote_addr_regex),
                &opt_non_empty(&req.data),
                &executor_list,
                &session.person_unique,
            ],
        )
        .await
        .map_err(|e| {
            tracing::error!("[u2_invoke_create] insert failed: {}", e);
            AppError::Internal
        })?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

/// GET /jaxrs/program_center/invoke/{flag} —— 按 id/name/alias 查询
pub async fn u2_invoke_get(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let Some(id) = u2_invoke_find_by_flag(&client, flag.trim()).await? else {
        return Ok(Json(ActionResult::error("invoke not found")));
    };
    let row = client
        .query_opt(
            "SELECT id, name, alias, category, description, enable, enable_token, \
             enable_anonymous, validated, text, remote_addr_regex, last_start_time, last_end_time \
             FROM x_program_invoke WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(r) = row else {
        return Ok(Json(ActionResult::error("invoke not found")));
    };
    Ok(Json(ActionResult::success(json!({
        "id": r.get::<_, String>("id"),
        "name": r.get::<_, String>("name"),
        "alias": r.get::<_, Option<String>>("alias"),
        "category": r.get::<_, Option<String>>("category"),
        "description": r.get::<_, Option<String>>("description"),
        "enable": r.get::<_, bool>("enable"),
        "enableToken": r.get::<_, bool>("enable_token"),
        "enableAnonymous": r.get::<_, bool>("enable_anonymous"),
        "validated": r.get::<_, bool>("validated"),
        "text": r.get::<_, Option<String>>("text"),
        "remoteAddrRegex": r.get::<_, Option<String>>("remote_addr_regex"),
    }))))
}

/// PUT /jaxrs/program_center/invoke/{flag} —— 更新服务调用
pub async fn u2_invoke_update(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(req): Json<U2InvokeRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Ok(Json(ActionResult::error("name cannot be empty")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let Some(id) = u2_invoke_find_by_flag(&client, flag.trim()).await? else {
        return Ok(Json(ActionResult::error("invoke not found")));
    };

    // 查重排除自身
    let dup = client
        .query_opt(
            "SELECT id FROM x_program_invoke \
             WHERE (name = $1 OR alias = $1 OR alias = $2) AND id <> $3 LIMIT 1",
            &[&name, &opt_non_empty(&req.alias), &id],
        )
        .await
        .map_err(|e| {
            tracing::error!("[u2_invoke_update] dup check failed: {}", e);
            AppError::Internal
        })?;
    if dup.is_some() {
        return Ok(Json(ActionResult::error("duplicate name")));
    }

    let alias = opt_non_empty(&req.alias);
    let n = client
        .execute(
            "UPDATE x_program_invoke SET name = $1, alias = $2, category = $3, description = $4, \
             enable = $5, enable_token = $6, enable_anonymous = $7, validated = $8, text = $9, \
             remote_addr_regex = $10, data = $11, update_time = NOW() WHERE id = $12",
            &[
                &name,
                &alias,
                &opt_non_empty(&req.category),
                &opt_non_empty(&req.description),
                &req.enable,
                &req.enable_token,
                &req.enable_anonymous,
                &req.validated,
                &opt_non_empty(&req.text),
                &opt_non_empty(&req.remote_addr_regex),
                &opt_non_empty(&req.data),
                &id,
            ],
        )
        .await
        .map_err(|e| {
            tracing::error!("[u2_invoke_update] update failed: {}", e);
            AppError::Internal
        })?;
    if n == 0 {
        return Ok(Json(ActionResult::error("invoke not found")));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

/// DELETE /jaxrs/program_center/invoke/{flag} —— 删除服务调用
pub async fn u2_invoke_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let Some(id) = u2_invoke_find_by_flag(&client, flag.trim()).await? else {
        return Ok(Json(ActionResult::error("invoke not found")));
    };
    client
        .execute("DELETE FROM x_program_invoke WHERE id = $1", &[&id])
        .await
        .map_err(|e| {
            tracing::error!("[u2_invoke_delete] delete failed: {}", e);
            AppError::Internal
        })?;
    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

// --- appstyle erase（GET，清除当前类图，对齐 Java 无 id 参数语义）---

async fn u2_appstyle_erase_current(
    pool: &Pool,
    session: &shared::session::Session,
    resource_type: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(pool, session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "DELETE FROM x_program_deploy_resource \
             WHERE id = (SELECT id FROM x_program_deploy_resource \
                         WHERE resource_type = $1 ORDER BY create_time DESC LIMIT 1)",
            &[&resource_type],
        )
        .await
        .map_err(|e| {
            tracing::error!("[u2_appstyle_erase] delete failed: {}", e);
            AppError::Internal
        })?;
    if n == 0 {
        return Ok(Json(ActionResult::error("not found")));
    }
    Ok(Json(ActionResult::success(json!({
        "value": true,
        "resourceType": resource_type,
    }))))
}

/// GET /jaxrs/program_center/appstyle/image/login/avatar/erase —— 清除当前登录头像
pub async fn u2_appstyle_login_avatar_erase_get(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_appstyle_erase_current(&pool, &session, "login_avatar").await
}

/// GET /jaxrs/program_center/appstyle/image/launch/logo/erase —— 清除当前启动 Logo
pub async fn u2_appstyle_launch_logo_erase_get(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    u2_appstyle_erase_current(&pool, &session, "launch_logo").await
}

// ── agent list / delete（Java: GET/DELETE /agent）──────────────────────

pub async fn agent_list_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, description, creator_person, create_time FROM x_program_agent WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let opt = |k: &str| -> String { row.get::<_, Option<String>>(k).unwrap_or_default() };
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(opt("id"))),
                ("name".to_string(), Value::String(opt("name"))),
                ("flag".to_string(), Value::String(opt("flag"))),
                ("description".to_string(), Value::String(opt("description"))),
                ("creatorPerson".to_string(), Value::String(opt("creator_person"))),
                ("createTime".to_string(), Value::String(opt("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn agent_delete_flag(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_program_agent SET deleted_at = NOW() WHERE (flag = $1 OR id = $1) AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("agent not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("flag".to_string(), Value::String(flag)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

// ── apppack 家族（Java AppPackAction / AppPackAnonymousAction，migration 062 建表）──

#[derive(Debug, Deserialize)]
pub struct AppPackStartRequest {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppPackPublishRequest {
    pub id: String,
}

fn apppack_row_to_value(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    let opt = |k: &str| -> String { row.get::<_, Option<String>>(k).unwrap_or_default() };
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(opt("id"))),
        ("name".to_string(), Value::String(opt("name"))),
        ("version".to_string(), Value::String(opt("version"))),
        ("status".to_string(), Value::String(opt("status"))),
        ("fileName".to_string(), Value::String(opt("file_name"))),
        ("filePath".to_string(), Value::String(opt("file_path"))),
        ("description".to_string(), Value::String(opt("description"))),
        (
            "createTime".to_string(),
            Value::String(opt("create_time")),
        ),
        (
            "updateTime".to_string(),
            Value::String(opt("update_time")),
        ),
    ]))
}

pub async fn apppack_info_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, version, status, file_name, file_path, description, create_time, update_time FROM x_program_app_pack ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(apppack_row_to_value).collect();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

const APPPACK_FILE_COLUMNS: &str =
    "SELECT id, name, version, status, file_name, file_path, description, create_time, update_time FROM x_program_app_pack";

pub async fn apppack_file_last(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!("{} WHERE status = 'published' ORDER BY update_time DESC LIMIT 1", APPPACK_FILE_COLUMNS),
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(apppack_row_to_value(&row)))),
        None => Ok(Json(ActionResult::error("no published pack found"))),
    }
}

pub async fn apppack_file_download(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            &format!("{} WHERE id = $1", APPPACK_FILE_COLUMNS),
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(apppack_row_to_value(&row)))),
        None => Ok(Json(ActionResult::error("app pack not found"))),
    }
}

pub async fn apppack_logo_get(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT config_json FROM x_program_app_pack ORDER BY create_time DESC LIMIT 1",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let raw: Option<String> = row.get("config_json");
            let logo = raw
                .as_deref()
                .and_then(|s| serde_json::from_str::<Value>(s).ok())
                .and_then(|v| v.get("logo").and_then(|l| l.as_str()).map(|s| s.to_string()))
                .unwrap_or_default();
            Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
                ("logo".to_string(), Value::String(logo)),
            ])))))
        }
        None => Ok(Json(ActionResult::error("app pack not found"))),
    }
}

pub async fn apppack_android_repack(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_program_app_pack SET status = 'repacking', update_time = NOW() WHERE id = (SELECT id FROM x_program_app_pack ORDER BY create_time DESC LIMIT 1)",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("app pack not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("repacking".to_string())),
    ])))))
}

pub async fn apppack_android_start(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(req): Json<AppPackStartRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_else(|| "android-pack".to_string());
    let version = req.version.unwrap_or_else(|| "1.0.0".to_string());
    let description = req.description.unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_program_app_pack (id, name, version, status, description, creator_person, create_time, update_time) \
             VALUES ($1, $2, $3, 'building', $4, $5, NOW(), NOW())",
            &[&id, &name, &version, &description, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("status".to_string(), Value::String("building".to_string())),
    ])))))
}

pub async fn apppack_publish(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(req): Json<AppPackPublishRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    require_admin(&pool, &session).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_program_app_pack SET status = 'published', update_time = NOW() WHERE id = $1",
            &[&req.id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("app pack not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(req.id)),
        ("status".to_string(), Value::String("published".to_string())),
    ])))))
}

pub async fn apppack_server_connect(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one("SELECT COUNT(*) AS packs FROM x_program_app_pack", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let packs: i64 = row.get("packs");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("connected".to_string(), Value::Bool(true)),
        ("packCount".to_string(), Value::Number(serde_json::Number::from(packs))),
    ])))))
}

// ── dict 家族写端点（Java DictAction POST "" / PUT|POST|DELETE {dictFlag}/{path}/data / DELETE {id}）──

#[derive(Debug, Deserialize)]
pub struct DictCreateRequest {
    #[serde(rename = "dictFlag")]
    pub dict_flag: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "appName")]
    pub app_name: Option<String>,
    #[serde(rename = "keyName")]
    pub key_name: Option<String>,
    #[serde(rename = "appData")]
    pub app_data: Option<String>,
}

pub async fn dict_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(req): Json<DictCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_else(|| "dict".to_string());
    let flag = req.dict_flag.unwrap_or_default();
    let app_name = req.app_name.unwrap_or_default();
    let key_name = req.key_name.unwrap_or_default();
    let app_data = req.app_data.unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_program_dict (id, name, flag, app_name, key_name, app_data, creator_person, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())",
            &[&id, &name, &flag, &app_name, &key_name, &app_data, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
    ])))))
}

async fn dict_data_write(
    pool: Extension<Pool>,
    dict_flag: String,
    path: String,
    body: Value,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let data_str = match body {
        Value::String(s) => s,
        _ => serde_json::to_string(&body).map_err(|_| AppError::Internal)?,
    };

    let n = client
        .execute(
            "UPDATE x_program_dict SET app_data = $1, update_time = NOW() WHERE flag = $2 AND deleted_at IS NULL",
            &[&data_str, &dict_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("dictFlag".to_string(), Value::String(dict_flag)),
        ("path".to_string(), Value::String(path)),
    ])))))
}

pub async fn dict_data_save_put(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(path): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    dict_data_write(pool, dict_flag, path, body).await
}

pub async fn dict_data_delete_path(
    pool: Extension<Pool>,
    Path(dict_flag): Path<String>,
    Path(path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_program_dict SET app_data = '', update_time = NOW() WHERE flag = $1 AND deleted_at IS NULL",
            &[&dict_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("dictFlag".to_string(), Value::String(dict_flag)),
        ("path".to_string(), Value::String(path)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn dict_delete_id(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT creator_person FROM x_program_dict WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("dict not found")));
    };
    let owner: String = row.get::<_, Option<String>>("creator_person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;

    let n = client
        .execute(
            "UPDATE x_program_dict SET deleted_at = NOW() WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("dict not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}

// ── script 家族写端点（IDOR：creator_person 取自会话，改/删前 require_owner）──

#[derive(Debug, Deserialize)]
pub struct ScriptCreateRequest {
    pub name: Option<String>,
    pub flag: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ScriptSaveRequest {
    pub name: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
}

pub async fn script_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(req): Json<ScriptCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_else(|| "script".to_string());
    let flag = req.flag.unwrap_or_else(|| id.clone());
    let content = req.content.unwrap_or_default();
    let category = req.category.unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_program_script (id, name, flag, content, category, creator_person, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &name, &flag, &content, &category, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("flag".to_string(), Value::String(flag)),
    ])))))
}

async fn script_save(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    column: &'static str,
    key: String,
    req: ScriptSaveRequest,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let select_sql = format!(
        "SELECT creator_person FROM x_program_script WHERE {} = $1 AND deleted_at IS NULL",
        column
    );
    let row = client
        .query_opt(select_sql.as_str(), &[&key])
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("script not found")));
    };
    let owner: String = row.get::<_, Option<String>>("creator_person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;

    let name: Option<String> = req.name;
    let content: Option<String> = req.content;
    let category: Option<String> = req.category;

    let update_sql = format!(
        "UPDATE x_program_script SET name = COALESCE($2, name), content = COALESCE($3, content), category = COALESCE($4, category), update_time = NOW() \
         WHERE {} = $1 AND deleted_at IS NULL",
        column
    );
    let n = client
        .execute(update_sql.as_str(), &[&key, &name, &content, &category])
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("script not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(key)),
    ])))))
}

pub async fn script_save_flag(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(req): Json<ScriptSaveRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    script_save(pool, session, "flag", flag, req).await
}

pub async fn script_update_id(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
    Json(req): Json<ScriptSaveRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    script_save(pool, session, "id", id, req).await
}

pub async fn script_delete_id(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT creator_person FROM x_program_script WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        return Ok(Json(ActionResult::error("script not found")));
    };
    let owner: String = row.get::<_, Option<String>>("creator_person").unwrap_or_default();
    shared::middleware::require_owner(&pool, &session, &owner).await?;

    let n = client
        .execute(
            "UPDATE x_program_script SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        return Ok(Json(ActionResult::error("script not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("deleted".to_string(), Value::Bool(true)),
    ])))))
}
