use axum::{
    extract::Extension,
    Json, Router,
    routing::get,
    routing::post,
    routing::put,
    routing::delete,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

#[derive(Debug, Deserialize)]
pub struct PersonLikeRequest {
    pub name: Option<String>,
}

pub async fn organization_assemble_control_role_list_flag_next_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let count: i32 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client
            .query(
                "SELECT id, name, description, creator, create_time FROM x_org_role ORDER BY create_time DESC LIMIT $1",
                &[&count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, description, creator, create_time FROM x_org_role WHERE id > $1 ORDER BY create_time DESC LIMIT $2",
                &[&flag, &count],
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
                ("description".to_string(), Value::String(row.get("description"))),
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

pub async fn organization_assemble_control_role_flag(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, description, creator, create_time FROM x_org_role WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("role not found"))),
    }
}

pub async fn organization_assemble_control_unit_list_flag_next_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let count: i32 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time FROM x_org_unit WHERE parent_id IS NULL ORDER BY sort ASC, create_time DESC LIMIT $1",
                &[&count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time FROM x_org_unit WHERE parent_id = $1 ORDER BY sort ASC, create_time DESC LIMIT $2",
                &[&flag, &count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let parent_id: Option<String> = row.get("parent_id");
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), parent_id.map(Value::String).unwrap_or(Value::Null)),
                ("level".to_string(), Value::String(row.get("level"))),
                ("sort".to_string(), Value::String(row.get("sort"))),
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

pub async fn organization_assemble_control_unit_flag(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, parent_id, level, sort, creator, create_time FROM x_org_unit WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let parent_id: Option<String> = row.get("parent_id");
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), parent_id.map(Value::String).unwrap_or(Value::Null)),
                ("level".to_string(), Value::String(row.get("level"))),
                ("sort".to_string(), Value::String(row.get("sort"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("unit not found"))),
    }
}

pub async fn organization_assemble_control_person_list_like(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): axum::extract::Json<PersonLikeRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let name_pattern = req.name.unwrap_or_default();
    let like_pattern = format!("%{}%", name_pattern);

    let rows = client
        .query(
            "SELECT id, name, mobile, email, unit_id, creator, create_time FROM x_org_person WHERE name ILIKE $1 ORDER BY create_time DESC",
            &[&like_pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mobile".to_string(), Value::String(row.get("mobile"))),
                ("email".to_string(), Value::String(row.get("email"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
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

/// Stub handler for /jaxrs/organization/assemble/control/export/export/all
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_export_export_all() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/export/result/flag/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_export_result_flag_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/export/zhengwudingding/person
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_export_zhengwudingding_person() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/like
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_like() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/like/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_like_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/like/pinyin
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_like_pinyin() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_like_pinyin_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/direct
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_person_personFlag_sup_direct() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/nested
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_person_personFlag_sup_nested() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/pinyininitial
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_pinyininitial() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_pinyininitial_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/role/{roleFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_role_roleFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/{flag}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_flag_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/{flag}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_flag_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/{flag}/sub/direct
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_flag_sub_direct() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/{flag}/sub/nested
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_flag_sub_nested() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/{flag}/sup/direct
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_flag_sup_direct() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/list/{flag}/sup/nested
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_list_flag_sup_nested() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/{flag}/add/member
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_flag_add_member() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/{flag}/add/member/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_flag_add_member_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/{flag}/delete/member
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_flag_delete_member() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/{flag}/delete/member/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_flag_delete_member_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/{flag}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/group/{flag}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_group_flag_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/like
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_like() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/like/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_like_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/like/pinyin
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_like_pinyin() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_like_pinyin_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/person/{personFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_person_personFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/pinyininitial
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_pinyininitial() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_pinyininitial_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/unit/{unitFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_unit_unitFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/unitduty/name/{unitDutyName}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_unitduty_name_unitDutyName() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/{flag}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_flag_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/{flag}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_flag_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/list/{flag}/unitduty/name/{unitDutyName}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_list_flag_unitduty_name_unitDutyName() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/{flag}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/{flag}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_flag_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/identity/{flag}/order/before/{followFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_identity_flag_order_before_followFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/inputperson/result/flag/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_inputperson_result_flag_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/inputperson/template
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_inputperson_template() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/inputperson/wipe
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_inputperson_wipe() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/loginrecord/{stream}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_loginrecord_stream() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/permissionsetting/list
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_permissionsetting_list() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/permissionsetting/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_permissionsetting_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/permissionsetting/{flag}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_permissionsetting_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/permissionsetting/{flag}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_permissionsetting_flag_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personattribute/list/person/{personFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personattribute_list_person_personFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personattribute/list/{flag}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personattribute_list_flag_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personattribute/list/{flag}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personattribute_list_flag_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personattribute/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personattribute_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personattribute/{flag}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personattribute_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personattribute/{flag}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personattribute_flag_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/createCode/{cardId}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_createCode_cardId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/createQR/{cardId}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_createQR_cardId() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/listPersonalVCf/{idList}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_listPersonalVCf_idList() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/listVCf/{idList}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_listVCf_idList() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/listgrouptypes
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_listgrouptypes() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_listpaging_page_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_listpaging_page_page_size_size_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_listpagingwithgroup_page_page_size_size() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_listpagingwithgroup_page_page_size_size_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/mylist
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_mylist() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/personcard/{flag}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_personcard_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/group/{groupFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_group_groupFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/like
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_like() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/like/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_like_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/like/pinyin
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_like_pinyin() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_like_pinyin_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/person/{personFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_person_personFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/pinyininitial
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_pinyininitial() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_pinyininitial_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/list/{flag}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_list_flag_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/{flag}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/role/{flag}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_role_flag_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitattribute/list/unit/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitattribute_list_unit_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitattribute/list/{flag}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitattribute_list_flag_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitattribute/list/{flag}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitattribute_list_flag_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitattribute/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitattribute_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitattribute/{flag}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitattribute_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitattribute/{flag}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitattribute_flag_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/distinct/name
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_distinct_name() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/distinct/name/like/{key}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_distinct_name_like_key() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/list/identity/{identityFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_list_identity_identityFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/list/like
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_list_like() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/list/name/{name}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_list_name_name() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/list/unit/{unitFlag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_list_unit_unitFlag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/list/{flag}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_list_flag_next_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/list/{flag}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_list_flag_prev_count() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(0i64))),
            ("data".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/update/member
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_update_member() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/{flag}
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_flag() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/{flag}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_flag_mockdeletetoget() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// Stub handler for /jaxrs/organization/assemble/control/unitduty/{flag}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_organization_assemble_control_unitduty_flag_mockputtopost() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    let router = Router::new()
    .route("/jaxrs/organization/assemble/control/export/export/all", get(stub_organization_assemble_control_export_export_all))
    .route("/jaxrs/organization/assemble/control/export/result/flag/{flag}", get(stub_organization_assemble_control_export_result_flag_flag))
    .route("/jaxrs/organization/assemble/control/export/zhengwudingding/person", get(stub_organization_assemble_control_export_zhengwudingding_person))
    .route("/jaxrs/organization/assemble/control/group/list/like", get(stub_organization_assemble_control_group_list_like))
    .route("/jaxrs/organization/assemble/control/group/list/like/mockputtopost", get(stub_organization_assemble_control_group_list_like_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/list/like/pinyin", get(stub_organization_assemble_control_group_list_like_pinyin))
    .route("/jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost", get(stub_organization_assemble_control_group_list_like_pinyin_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/direct", get(stub_organization_assemble_control_group_list_person_personFlag_sup_direct))
    .route("/jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/nested", get(stub_organization_assemble_control_group_list_person_personFlag_sup_nested))
    .route("/jaxrs/organization/assemble/control/group/list/pinyininitial", get(stub_organization_assemble_control_group_list_pinyininitial))
    .route("/jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost", get(stub_organization_assemble_control_group_list_pinyininitial_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/list/role/{roleFlag}", get(stub_organization_assemble_control_group_list_role_roleFlag))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/next/{count}", get(stub_organization_assemble_control_group_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/prev/{count}", get(stub_organization_assemble_control_group_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/sub/direct", get(stub_organization_assemble_control_group_list_flag_sub_direct))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/sub/nested", get(stub_organization_assemble_control_group_list_flag_sub_nested))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/sup/direct", get(stub_organization_assemble_control_group_list_flag_sup_direct))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/sup/nested", get(stub_organization_assemble_control_group_list_flag_sup_nested))
    .route("/jaxrs/organization/assemble/control/group/{flag}", get(stub_organization_assemble_control_group_flag))
    .route("/jaxrs/organization/assemble/control/group/{flag}/add/member", get(stub_organization_assemble_control_group_flag_add_member))
    .route("/jaxrs/organization/assemble/control/group/{flag}/add/member/mockputtopost", get(stub_organization_assemble_control_group_flag_add_member_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/{flag}/delete/member", get(stub_organization_assemble_control_group_flag_delete_member))
    .route("/jaxrs/organization/assemble/control/group/{flag}/delete/member/mockputtopost", get(stub_organization_assemble_control_group_flag_delete_member_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/{flag}/mockdeletetoget", get(stub_organization_assemble_control_group_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/group/{flag}/mockputtopost", get(stub_organization_assemble_control_group_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/list/like", get(stub_organization_assemble_control_identity_list_like))
    .route("/jaxrs/organization/assemble/control/identity/list/like/mockputtopost", get(stub_organization_assemble_control_identity_list_like_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/list/like/pinyin", get(stub_organization_assemble_control_identity_list_like_pinyin))
    .route("/jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost", get(stub_organization_assemble_control_identity_list_like_pinyin_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/list/person/{personFlag}", get(stub_organization_assemble_control_identity_list_person_personFlag))
    .route("/jaxrs/organization/assemble/control/identity/list/pinyininitial", get(stub_organization_assemble_control_identity_list_pinyininitial))
    .route("/jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost", get(stub_organization_assemble_control_identity_list_pinyininitial_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/list/unit/{unitFlag}", get(stub_organization_assemble_control_identity_list_unit_unitFlag))
    .route("/jaxrs/organization/assemble/control/identity/list/unitduty/name/{unitDutyName}", get(stub_organization_assemble_control_identity_list_unitduty_name_unitDutyName))
    .route("/jaxrs/organization/assemble/control/identity/list/{flag}/next/{count}", get(stub_organization_assemble_control_identity_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/identity/list/{flag}/prev/{count}", get(stub_organization_assemble_control_identity_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/identity/list/{flag}/unitduty/name/{unitDutyName}", get(stub_organization_assemble_control_identity_list_flag_unitduty_name_unitDutyName))
    .route("/jaxrs/organization/assemble/control/identity/{flag}", get(stub_organization_assemble_control_identity_flag))
    .route("/jaxrs/organization/assemble/control/identity/{flag}/mockdeletetoget", get(stub_organization_assemble_control_identity_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/identity/{flag}/mockputtopost", get(stub_organization_assemble_control_identity_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/{flag}/order/before/{followFlag}", get(stub_organization_assemble_control_identity_flag_order_before_followFlag))
    .route("/jaxrs/organization/assemble/control/inputperson/result/flag/{flag}", get(stub_organization_assemble_control_inputperson_result_flag_flag))
    .route("/jaxrs/organization/assemble/control/inputperson/template", get(stub_organization_assemble_control_inputperson_template))
    .route("/jaxrs/organization/assemble/control/inputperson/wipe", get(stub_organization_assemble_control_inputperson_wipe))
    .route("/jaxrs/organization/assemble/control/loginrecord/{stream}", get(stub_organization_assemble_control_loginrecord_stream))
    .route("/jaxrs/organization/assemble/control/permissionsetting/list", get(stub_organization_assemble_control_permissionsetting_list))
    .route("/jaxrs/organization/assemble/control/permissionsetting/{flag}", get(stub_organization_assemble_control_permissionsetting_flag))
    .route("/jaxrs/organization/assemble/control/permissionsetting/{flag}/mockdeletetoget", get(stub_organization_assemble_control_permissionsetting_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/permissionsetting/{flag}/mockputtopost", get(stub_organization_assemble_control_permissionsetting_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/personattribute/list/person/{personFlag}", get(stub_organization_assemble_control_personattribute_list_person_personFlag))
    .route("/jaxrs/organization/assemble/control/personattribute/list/{flag}/next/{count}", get(stub_organization_assemble_control_personattribute_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/personattribute/list/{flag}/prev/{count}", get(stub_organization_assemble_control_personattribute_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/personattribute/{flag}", get(stub_organization_assemble_control_personattribute_flag))
    .route("/jaxrs/organization/assemble/control/personattribute/{flag}/mockdeletetoget", get(stub_organization_assemble_control_personattribute_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/personattribute/{flag}/mockputtopost", get(stub_organization_assemble_control_personattribute_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/personcard/createCode/{cardId}", get(stub_organization_assemble_control_personcard_createCode_cardId))
    .route("/jaxrs/organization/assemble/control/personcard/createQR/{cardId}", get(stub_organization_assemble_control_personcard_createQR_cardId))
    .route("/jaxrs/organization/assemble/control/personcard/listPersonalVCf/{idList}", get(stub_organization_assemble_control_personcard_listPersonalVCf_idList))
    .route("/jaxrs/organization/assemble/control/personcard/listVCf/{idList}", get(stub_organization_assemble_control_personcard_listVCf_idList))
    .route("/jaxrs/organization/assemble/control/personcard/listgrouptypes", get(stub_organization_assemble_control_personcard_listgrouptypes))
    .route("/jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}", get(stub_organization_assemble_control_personcard_listpaging_page_page_size_size))
    .route("/jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}/mockputtopost", get(stub_organization_assemble_control_personcard_listpaging_page_page_size_size_mockputtopost))
    .route("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}", get(stub_organization_assemble_control_personcard_listpagingwithgroup_page_page_size_size))
    .route("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}/mockputtopost", get(stub_organization_assemble_control_personcard_listpagingwithgroup_page_page_size_size_mockputtopost))
    .route("/jaxrs/organization/assemble/control/personcard/mylist", get(stub_organization_assemble_control_personcard_mylist))
    .route("/jaxrs/organization/assemble/control/personcard/{flag}", get(stub_organization_assemble_control_personcard_flag))
    .route("/jaxrs/organization/assemble/control/personcard/{flag}/mockdeletetoget", get(stub_organization_assemble_control_personcard_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/role/list/group/{groupFlag}", get(stub_organization_assemble_control_role_list_group_groupFlag))
    .route("/jaxrs/organization/assemble/control/role/list/like", get(stub_organization_assemble_control_role_list_like))
    .route("/jaxrs/organization/assemble/control/role/list/like/mockputtopost", get(stub_organization_assemble_control_role_list_like_mockputtopost))
    .route("/jaxrs/organization/assemble/control/role/list/like/pinyin", get(stub_organization_assemble_control_role_list_like_pinyin))
    .route("/jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost", get(stub_organization_assemble_control_role_list_like_pinyin_mockputtopost))
    .route("/jaxrs/organization/assemble/control/role/list/person/{personFlag}", get(stub_organization_assemble_control_role_list_person_personFlag))
    .route("/jaxrs/organization/assemble/control/role/list/pinyininitial", get(stub_organization_assemble_control_role_list_pinyininitial))
    .route("/jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost", get(stub_organization_assemble_control_role_list_pinyininitial_mockputtopost))
    .route("/jaxrs/organization/assemble/control/role/list/{flag}/next/{count}", get(organization_assemble_control_role_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/role/list/{flag}/prev/{count}", get(stub_organization_assemble_control_role_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/role/{flag}", get(organization_assemble_control_role_flag))
    .route("/jaxrs/organization/assemble/control/role/{flag}/mockdeletetoget", get(stub_organization_assemble_control_role_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/role/{flag}/mockputtopost", get(stub_organization_assemble_control_role_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/unitattribute/list/unit/{flag}", get(stub_organization_assemble_control_unitattribute_list_unit_flag))
    .route("/jaxrs/organization/assemble/control/unitattribute/list/{flag}/next/{count}", get(stub_organization_assemble_control_unitattribute_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/unitattribute/list/{flag}/prev/{count}", get(stub_organization_assemble_control_unitattribute_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/unitattribute/{flag}", get(stub_organization_assemble_control_unitattribute_flag))
    .route("/jaxrs/organization/assemble/control/unitattribute/{flag}/mockdeletetoget", get(stub_organization_assemble_control_unitattribute_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/unitattribute/{flag}/mockputtopost", get(stub_organization_assemble_control_unitattribute_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/unitduty/distinct/name", get(stub_organization_assemble_control_unitduty_distinct_name))
    .route("/jaxrs/organization/assemble/control/unitduty/distinct/name/like/{key}", get(stub_organization_assemble_control_unitduty_distinct_name_like_key))
    .route("/jaxrs/organization/assemble/control/unitduty/list/identity/{identityFlag}", get(stub_organization_assemble_control_unitduty_list_identity_identityFlag))
    .route("/jaxrs/organization/assemble/control/unitduty/list/like", get(stub_organization_assemble_control_unitduty_list_like))
    .route("/jaxrs/organization/assemble/control/unitduty/list/name/{name}", get(stub_organization_assemble_control_unitduty_list_name_name))
    .route("/jaxrs/organization/assemble/control/unitduty/list/unit/{unitFlag}", get(stub_organization_assemble_control_unitduty_list_unit_unitFlag))
    .route("/jaxrs/organization/assemble/control/unitduty/list/{flag}/next/{count}", get(stub_organization_assemble_control_unitduty_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/unitduty/list/{flag}/prev/{count}", get(stub_organization_assemble_control_unitduty_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/unitduty/update/member", get(stub_organization_assemble_control_unitduty_update_member))
    .route("/jaxrs/organization/assemble/control/unitduty/{flag}", get(stub_organization_assemble_control_unitduty_flag))
    .route("/jaxrs/organization/assemble/control/unitduty/{flag}/mockdeletetoget", get(stub_organization_assemble_control_unitduty_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/unitduty/{flag}/mockputtopost", get(stub_organization_assemble_control_unitduty_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/unit/list/{flag}/next/{count}", get(organization_assemble_control_unit_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/unit/{flag}", get(organization_assemble_control_unit_flag))
    .route("/jaxrs/organization/assemble/control/person/list/like", post(organization_assemble_control_person_list_like))
    .layer(Extension(pool));
    router
}
