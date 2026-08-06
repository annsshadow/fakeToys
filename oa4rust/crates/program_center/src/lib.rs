use axum::{
    extract::{Extension, Path},
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

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
            ("indexPortal".to_string(), Value::Null),
            ("indexId".to_string(), Value::Null),
            ("portalList".to_string(), Value::Array(portal_list)),
        ]),
    ))))
}

pub async fn modules_all(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("name".to_string(), Value::String("Application".to_string())),
            ("className".to_string(), Value::String("com.x.organization.core.entity.Application".to_string())),
            ("entityCount".to_string(), Value::Number(serde_json::Number::from(12))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("name".to_string(), Value::String("Person".to_string())),
            ("className".to_string(), Value::String("com.x.organization.core.entity.Person".to_string())),
            ("entityCount".to_string(), Value::Number(serde_json::Number::from(8))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("name".to_string(), Value::String("Unit".to_string())),
            ("className".to_string(), Value::String("com.x.organization.core.entity.Unit".to_string())),
            ("entityCount".to_string(), Value::Number(serde_json::Number::from(5))),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("name".to_string(), Value::String("Process".to_string())),
            ("className".to_string(), Value::String("com.x.process.core.entity.Process".to_string())),
            ("entityCount".to_string(), Value::Number(serde_json::Number::from(15))),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn program_center_router() -> Router {
    Router::new()
        .route("/jaxrs/program/applications", get(applications))
        .route("/jaxrs/program/appstyle/current/style", get(current_style))
        .route("/jaxrs/program/datastructure/modules/all", get(modules_all))
}

pub async fn collect_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): Json<CollectAddRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn config_get(
    pool: Option<Extension<Pool>>,
    Path(key): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/program_center/health", axum::routing::get(|| async { "TODO: program_center - real implementation needed" }))
        .route("/jaxrs/program_center/collect/list", get(collect_list))
        .route("/jaxrs/program_center/collect/add", post(collect_add))
        .route("/jaxrs/program_center/collect/remove/{id}", post(collect_remove))
        .route("/jaxrs/program_center/config/get/{key}", get(config_get))
        .route("/jaxrs/program_center/config/save", post(config_save))
        .layer(axum::Extension(pool))
}


/// Stub handler for /jaxrs/program_center/agent/{flag}
/// TODO: Implement real business logic
pub async fn stub_program_center_agent_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/agent/{flag}/disable
/// TODO: Implement real business logic
pub async fn stub_program_center_agent_flag_disable() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/agent/{flag}/enable
/// TODO: Implement real business logic
pub async fn stub_program_center_agent_flag_enable() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/agent/{flag}/execute
/// TODO: Implement real business logic
pub async fn stub_program_center_agent_flag_execute() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/agent/{flag}/file
/// TODO: Implement real business logic
pub async fn stub_program_center_agent_flag_file() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/andfx/pull/sync
/// TODO: Implement real business logic
pub async fn stub_program_center_andfx_pull_sync() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/current/style
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_current_style() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/current/update
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_current_update() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/application/top
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_application_top() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/application/top/erase
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_application_top_erase() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/launch/logo
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_launch_logo() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/launch/logo/erase
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_launch_logo_erase() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/login/avatar
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_login_avatar() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/login/avatar/erase
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_login_avatar_erase() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/menu/logo/blur
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_menu_logo_blur() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/menu/logo/blur/erase
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_menu_logo_blur_erase() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/menu/logo/focus
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_menu_logo_focus() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/menu/logo/focus/erase
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_menu_logo_focus_erase() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/process/default
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_process_default() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/process/default/erase
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_process_default_erase() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/setup/about/logo
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_setup_about_logo() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/image/setup/about/logo/erase
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_image_setup_about_logo_erase() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/appstyle/index/portal
/// TODO: Implement real business logic
pub async fn stub_program_center_appstyle_index_portal() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/bar/create/mass/{from}/{count}
/// TODO: Implement real business logic
pub async fn stub_program_center_bar_create_mass_from_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/bar/select1/field/{field}/value/{value}/count/{count}
/// TODO: Implement real business logic
pub async fn stub_program_center_bar_select1_field_field_value_value_count_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/bar/select2/count/{count}
/// TODO: Implement real business logic
pub async fn stub_program_center_bar_select2_count_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/bar/select3/field/{field}/value/{value}/count/{count}
/// TODO: Implement real business logic
pub async fn stub_program_center_bar_select3_field_field_value_value_count_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/bar/select4/field/{field}/value/{value}/count/{count}
/// TODO: Implement real business logic
pub async fn stub_program_center_bar_select4_field_field_value_value_count_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/captcha/list
/// TODO: Implement real business logic
pub async fn stub_program_center_captcha_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/captcha/v2/create/width/{width}/height/{height}
/// TODO: Implement real business logic
pub async fn stub_program_center_captcha_v2_create_width_width_height_height() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/captcha/{id}/validate/answer/{answer}
/// TODO: Implement real business logic
pub async fn stub_program_center_captcha_id_validate_answer_answer() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/center/applications
/// TODO: Implement real business logic
pub async fn stub_program_center_center_applications() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/center/regist/applications
/// TODO: Implement real business logic
pub async fn stub_program_center_center_regist_applications() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/center/version
/// TODO: Implement real business logic
pub async fn stub_program_center_center_version() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/code/create/mobile/{mobile}
/// TODO: Implement real business logic
pub async fn stub_program_center_code_create_mobile_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/code/list
/// TODO: Implement real business logic
pub async fn stub_program_center_code_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/code/list/paging/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_program_center_code_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/code/validate/mobile/{mobile}/answer/{answer}
/// TODO: Implement real business logic
pub async fn stub_program_center_code_validate_mobile_mobile_answer_answer() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/code/validate/mobile/{mobile}/answer/{answer}/cascade
/// TODO: Implement real business logic
pub async fn stub_program_center_code_validate_mobile_mobile_answer_answer_cascade() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/code/mobile/{mobile}
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_code_mobile_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/connect
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_connect() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/controllebbs
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_controllebbs() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/controllermobile/name/{name}/mobile/{mobile}
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_controllermobile_name_name_mobile_mobile() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/disconnect
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_disconnect() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/login
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_login() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/mobile/check/connect
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_mobile_check_connect() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/name/{name}/exist
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_name_name_exist() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/name/{name}/mobile/{mobile}/code/{code}
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_name_name_mobile_mobile_code_code() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/person
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/resetpassword
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_resetpassword() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/sync/area
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_sync_area() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/updateUnit
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_updateUnit() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/urlMapping
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_urlMapping() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/validate
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_validate() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/validate/codeanswer
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_validate_codeanswer() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/validate/direct
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_validate_direct() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/collect/validate/password
/// TODO: Implement real business logic
pub async fn stub_program_center_collect_validate_password() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/command/execute
/// TODO: Implement real business logic
pub async fn stub_program_center_command_execute() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/command/list/node
/// TODO: Implement real business logic
pub async fn stub_program_center_command_list_node() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config-open/get/disable/export/enable
/// TODO: Implement real business logic
pub async fn stub_program_center_config_open_get_disable_export_enable() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/centerserver
/// TODO: Implement real business logic
pub async fn stub_program_center_config_centerserver() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/change/password
/// TODO: Implement real business logic
pub async fn stub_program_center_config_change_password() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/collect
/// TODO: Implement real business logic
pub async fn stub_program_center_config_collect() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/license
/// TODO: Implement real business logic
pub async fn stub_program_center_config_license() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/list
/// TODO: Implement real business logic
pub async fn stub_program_center_config_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/list/application
/// TODO: Implement real business logic
pub async fn stub_program_center_config_list_application() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/list/dump/data
/// TODO: Implement real business logic
pub async fn stub_program_center_config_list_dump_data() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/list/dump/data/current/node
/// TODO: Implement real business logic
pub async fn stub_program_center_config_list_dump_data_current_node() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/list/entity
/// TODO: Implement real business logic
pub async fn stub_program_center_config_list_entity() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/open
/// TODO: Implement real business logic
pub async fn stub_program_center_config_open() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/open/run/time/config
/// TODO: Implement real business logic
pub async fn stub_program_center_config_open_run_time_config() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/person
/// TODO: Implement real business logic
pub async fn stub_program_center_config_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/portal
/// TODO: Implement real business logic
pub async fn stub_program_center_config_portal() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/config/proxy
/// TODO: Implement real business logic
pub async fn stub_program_center_config_proxy() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn config_save(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): Json<ConfigSaveRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_config_ternary_management(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_config_token(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_datastructure_fileds_all(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_datastructure_modules_all(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_datastructure_tables_all(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_deploy_list_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, version, creator, create_time FROM x_program_deploy ORDER BY create_time DESC LIMIT $2 OFFSET ($1 - 1) * $2",
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

pub async fn stub_program_center_deploy_server_o2(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_deploy_server_resource(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_deploy_web_resource_as_new_asNew(
    pool: Option<Extension<Pool>>,
    Path(as_new): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_deploy_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_designer_search(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_dict_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_dict_list_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, key_name, app_name, creator, create_time FROM x_program_dict WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET ($1 - 1) * $2",
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

pub async fn stub_program_center_dict_dictFlag_data(
    pool: Option<Extension<Pool>>,
    Path(dict_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_dict_dictFlag_path_data(
    pool: Option<Extension<Pool>>,
    Path(dict_flag): Path<String>,
    Path(_path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_dict_dictFlag_path_data_mockdeletetoget(
    pool: Option<Extension<Pool>>,
    Path(dict_flag): Path<String>,
    Path(_path): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_dict_dictFlag_path_data_mockputtopost(
    pool: Option<Extension<Pool>>,
    Path(dict_flag): Path<String>,
    Path(_path): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_dict_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_dingding_get_callback_aes() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_dingding_pull_sync(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'dingding', 'pull', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
            ("affected".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn stub_program_center_dingding_request_pull_sync() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_dingding_sync_organization_callback(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let event_type = body.get("eventType").and_then(|v| v.as_str()).unwrap_or_default();
    let org_id = body.get("orgId").and_then(|v| v.as_str()).unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, org_id, event_type, create_time) VALUES ($1, 'dingding', 'callback', $2, $3, NOW())",
            &[&uuid::Uuid::new_v4().to_string(), &org_id, &event_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_dingding_sync_organization_register_callback_enable(
    Path(_enable): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("registered".to_string(), Value::Bool(true)),
            ("enable".to_string(), Value::String(_enable)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/distribute/assemble/source/{source}
/// TODO: Implement real business logic
pub async fn stub_program_center_distribute_assemble_source_source() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/distribute/webserver/assemble/source/{source}
/// TODO: Implement real business logic
pub async fn stub_program_center_distribute_webserver_assemble_source_source() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/foo/create/mass/{from}/{count}
/// TODO: Implement real business logic
pub async fn stub_program_center_foo_create_mass_from_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/input/compare
/// TODO: Implement real business logic
pub async fn stub_program_center_input_compare() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/input/cover
/// TODO: Implement real business logic
pub async fn stub_program_center_input_cover() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/input/create
/// TODO: Implement real business logic
pub async fn stub_program_center_input_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/input/prepare/cover
/// TODO: Implement real business logic
pub async fn stub_program_center_input_prepare_cover() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/input/prepare/create
/// TODO: Implement real business logic
pub async fn stub_program_center_input_prepare_create() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/invoke/list/category
/// TODO: Implement real business logic
pub async fn stub_program_center_invoke_list_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/invoke/list/with/category/{category}
/// TODO: Implement real business logic
pub async fn stub_program_center_invoke_list_with_category_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/invoke/token
/// TODO: Implement real business logic
pub async fn stub_program_center_invoke_token() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/invoke/{flag}
/// TODO: Implement real business logic
pub async fn stub_program_center_invoke_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/invoke/{flag}/client/{client}/token/{token}/execute
/// TODO: Implement real business logic
pub async fn stub_program_center_invoke_flag_client_client_token_token_execute() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/invoke/{flag}/execute
/// TODO: Implement real business logic
pub async fn stub_program_center_invoke_flag_execute() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/invoke/{flag}/execute/get
/// TODO: Implement real business logic
pub async fn stub_program_center_invoke_flag_execute_get() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/invoke/{flag}/file
/// TODO: Implement real business logic
pub async fn stub_program_center_invoke_flag_file() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/jest/center/list
/// TODO: Implement real business logic
pub async fn stub_program_center_jest_center_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/jest/clear/cache/{source}
/// TODO: Implement real business logic
pub async fn stub_program_center_jest_clear_cache_source() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/jest/list
/// TODO: Implement real business logic
pub async fn stub_program_center_jest_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/jest/version
/// TODO: Implement real business logic
pub async fn stub_program_center_jest_version() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/cloud/unit/is/vip
/// TODO: Implement real business logic
pub async fn stub_program_center_market_cloud_unit_is_vip() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/install/offline
/// TODO: Implement real business logic
pub async fn stub_program_center_market_install_offline() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/list/category
/// TODO: Implement real business logic
pub async fn stub_program_center_market_list_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/list/install/log/paging/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_program_center_market_list_install_log_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/list/paging/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_program_center_market_list_paging_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/list/paging/{page}/size/{size}/category/{category}
/// TODO: Implement real business logic
pub async fn stub_program_center_market_list_paging_page_size_size_category_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/list/top/three
/// TODO: Implement real business logic
pub async fn stub_program_center_market_list_top_three() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/{flag}
/// TODO: Implement real business logic
pub async fn stub_program_center_market_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/{flag}/cover/pic
/// TODO: Implement real business logic
pub async fn stub_program_center_market_flag_cover_pic() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/{flag}/install/log
/// TODO: Implement real business logic
pub async fn stub_program_center_market_flag_install_log() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/{flag}/install/or/update
/// TODO: Implement real business logic
pub async fn stub_program_center_market_flag_install_or_update() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/{flag}/installed/version
/// TODO: Implement real business logic
pub async fn stub_program_center_market_flag_installed_version() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/{flag}/uninstall
/// TODO: Implement real business logic
pub async fn stub_program_center_market_flag_uninstall() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/market/{id}/download
/// TODO: Implement real business logic
pub async fn stub_program_center_market_id_download() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/compare/upload
/// TODO: Implement real business logic
pub async fn stub_program_center_module_compare_upload() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/list
/// TODO: Implement real business logic
pub async fn stub_program_center_module_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/list/category
/// TODO: Implement real business logic
pub async fn stub_program_center_module_list_category() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/output
/// TODO: Implement real business logic
pub async fn stub_program_center_module_output() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/output/list/structure
/// TODO: Implement real business logic
pub async fn stub_program_center_module_output_list_structure() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/output/structure
/// TODO: Implement real business logic
pub async fn stub_program_center_module_output_structure() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/output/{flag}/file
/// TODO: Implement real business logic
pub async fn stub_program_center_module_output_flag_file() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/remove/structure/{id}
/// TODO: Implement real business logic
pub async fn stub_program_center_module_remove_structure_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/write/{flag}
/// TODO: Implement real business logic
pub async fn stub_program_center_module_write_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/module/{id}/compare
/// TODO: Implement real business logic
pub async fn stub_program_center_module_id_compare() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/mpweixin/check
/// TODO: Implement real business logic
pub async fn stub_program_center_mpweixin_check() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/mpweixin/media/add/forever
/// TODO: Implement real business logic
pub async fn stub_program_center_mpweixin_media_add_forever() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/mpweixin/menu/add
/// TODO: Implement real business logic
pub async fn stub_program_center_mpweixin_menu_add() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/mpweixin/menu/create/to/weixin
/// TODO: Implement real business logic
pub async fn stub_program_center_mpweixin_menu_create_to_weixin() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/mpweixin/menu/delete/{id}
/// TODO: Implement real business logic
pub async fn stub_program_center_mpweixin_menu_delete_id() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/program_center/mpweixin/menu/list/weixin
/// TODO: Implement real business logic
pub async fn stub_program_center_mpweixin_menu_list_weixin() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn stub_program_center_mpweixin_menu_subscribe() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_mpweixin_menu_update_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_mpweixin_message_template_send() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_output_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_output_appInfoFlag_select(
    pool: Option<Extension<Pool>>,
    Path(app_info_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_output_flag_select_file(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_prompterrorlog_count_exceptionclass(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_prompterrorlog_count_loggername(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_prompterrorlog_list_id_next_count(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id > $1 ORDER BY id ASC LIMIT $2",
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

pub async fn stub_program_center_prompterrorlog_list_id_next_count_date_date(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id > $1 AND DATE(create_time) = $2::date ORDER BY id ASC LIMIT $3",
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

pub async fn stub_program_center_prompterrorlog_list_id_next_count_exceptionclass_exceptionClass(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(exception_class): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id > $1 AND exception_class = $2 ORDER BY id ASC LIMIT $3",
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

pub async fn stub_program_center_prompterrorlog_list_id_next_count_loggername_loggerName(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(logger_name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id > $1 AND logger_name = $2 ORDER BY id ASC LIMIT $3",
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

pub async fn stub_program_center_prompterrorlog_list_id_prev_count(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id < $1 ORDER BY id DESC LIMIT $2",
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

pub async fn stub_program_center_prompterrorlog_list_id_prev_count_date_date(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id < $1 AND DATE(create_time) = $2::date ORDER BY id DESC LIMIT $3",
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

pub async fn stub_program_center_prompterrorlog_list_id_prev_count_exceptionclass_exceptionClass(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(exception_class): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id < $1 AND exception_class = $2 ORDER BY id DESC LIMIT $3",
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

pub async fn stub_program_center_prompterrorlog_list_id_prev_count_loggername_loggerName(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(logger_name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, exception_class, logger_name, message, create_time FROM x_program_prompt_error_log WHERE id < $1 AND logger_name = $2 ORDER BY id DESC LIMIT $3",
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

pub async fn stub_program_center_prompterrorlog_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_qiyeweixin_get_callback_aes() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_qiyeweixin_pull_sync(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'qiyeweixin', 'pull', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_qiyeweixin_request_pull_sync() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_qiyeweixin_send_getprivateinfo_message(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("sent".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_schedule_list_schedule(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_schedule_list_schedulelocal(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_schedule_list_schedulelog_application_application(
    pool: Option<Extension<Pool>>,
    Path(application): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_schedule_report(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_schedule_schedule_fire(
    pool: Option<Extension<Pool>>,
    Path(schedule_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("fired".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_script_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_script_list_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_program_script WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET ($1 - 1) * $2",
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

pub async fn stub_program_center_script_name_name(
    pool: Option<Extension<Pool>>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_script_name_name_imported(
    pool: Option<Extension<Pool>>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_script_flag(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_script_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_test_test1() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
            ("test".to_string(), Value::String("test1".to_string())),
        ]),
    ))))
}

pub async fn stub_program_center_test_test2() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
            ("test".to_string(), Value::String("test2".to_string())),
        ]),
    ))))
}

pub async fn stub_program_center_tokenthreshold_update(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_unexpectederrorlog_list_id_next_count(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id > $1 ORDER BY id ASC LIMIT $2",
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

pub async fn stub_program_center_unexpectederrorlog_list_id_next_count_date_date(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id > $1 AND DATE(create_time) = $2::date ORDER BY id ASC LIMIT $3",
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

pub async fn stub_program_center_unexpectederrorlog_list_id_prev_count(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id < $1 ORDER BY id DESC LIMIT $2",
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

pub async fn stub_program_center_unexpectederrorlog_list_id_prev_count_date_date(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
    Path(date): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, error_type, message, stack_trace, create_time FROM x_program_unexpected_error_log WHERE id < $1 AND DATE(create_time) = $2::date ORDER BY id DESC LIMIT $3",
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

pub async fn stub_program_center_unexpectederrorlog_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

pub async fn stub_program_center_validation_meta() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("valid".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_validation_scripting_benchmark() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
            ("benchmark".to_string(), Value::String("completed".to_string())),
        ]),
    ))))
}

pub async fn stub_program_center_validation_timeout_timeout(
    Path(_timeout): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
            ("timeout".to_string(), Value::Number(serde_json::Number::from(_timeout))),
        ]),
    ))))
}

pub async fn stub_program_center_zhengwudingding_pull_sync(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, create_time) VALUES ($1, 'zhengwudingding', 'pull', NOW())",
            &[&uuid::Uuid::new_v4().to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("synced".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_zhengwudingding_regist_callback(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
            ("registered".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_program_center_zhengwudingding_sync_organization_callback(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let event_type = body.get("eventType").and_then(|v| v.as_str()).unwrap_or_default();
    let org_id = body.get("orgId").and_then(|v| v.as_str()).unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_program_sync_log (id, source, action, org_id, event_type, create_time) VALUES ($1, 'zhengwudingding', 'callback', $2, $3, NOW())",
            &[&uuid::Uuid::new_v4().to_string(), &org_id, &event_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}
