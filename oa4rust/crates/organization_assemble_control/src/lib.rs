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
use uuid::Uuid;

mod u2_helpers;
mod u2_misc;
mod u2_org;
mod u2_person;
mod u2_router;

#[derive(Debug, Deserialize)]
pub struct PersonLikeRequest {
    pub name: Option<String>,
}

pub async fn organization_assemble_control_role_list_flag_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client
            .query(
                "SELECT id, name, description, creator, create_time::text FROM x_org_role ORDER BY create_time::text DESC LIMIT $1::bigint",
                &[&count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, description, creator, create_time::text FROM x_org_role WHERE id > $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn organization_assemble_control_role_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, description, creator, create_time::text FROM x_org_role WHERE id = $1",
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
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id IS NULL ORDER BY sort ASC, create_time::text DESC LIMIT $1::bigint",
                &[&count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id = $1 ORDER BY sort ASC, create_time::text DESC LIMIT $2::bigint",
                &[&flag, &count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let parent_id: Option<String> = row.get("parent_id");
            Value::Object(serde_json::Map::from_iter(
                [
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("level".to_string(), Value::String(row.get("level"))),
                    ("sort".to_string(), Value::String(row.get("sort"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]
                .into_iter()
                .chain(parent_id.map(|v| ("\"parentId\"".to_string(), Value::String(v)))),
            ))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn organization_assemble_control_unit_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let parent_id: Option<String> = row.get("parent_id");
            let result = Value::Object(serde_json::Map::from_iter(
                [
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("level".to_string(), Value::String(row.get("level"))),
                    ("sort".to_string(), Value::String(row.get("sort"))),
                    ("creator".to_string(), Value::String(row.get("creator"))),
                    ("createTime".to_string(), Value::String(row.get("create_time"))),
                ]
                .into_iter()
                .chain(parent_id.map(|v| ("\"parentId\"".to_string(), Value::String(v)))),
            ));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("unit not found"))),
    }
}

pub async fn organization_assemble_control_unit_list_flag_sub_nested(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "WITH RECURSIVE sub AS (
                SELECT id FROM x_org_unit WHERE id = $1 AND deleted_at IS NULL
                UNION ALL
                SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL
            )
            SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit
            WHERE id IN (SELECT id FROM sub) AND deleted_at IS NULL
            ORDER BY sort ASC, create_time DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        let parent_id: Option<String> = row.get("parent_id");
        let level: i32 = row.get("level");
        let sort: i32 = row.get("sort");
        Value::Object(serde_json::Map::from_iter(
            [
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("level".to_string(), Value::Number(serde_json::Number::from(level))),
                ("sort".to_string(), Value::Number(serde_json::Number::from(sort))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]
            .into_iter()
            .chain(parent_id.map(|v| ("\"parentId\"".to_string(), Value::String(v)))),
        ))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn organization_assemble_control_unit_list_flag_sup_nested(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "WITH RECURSIVE sup AS (
                SELECT id, parent_id FROM x_org_unit WHERE id = $1 AND deleted_at IS NULL
                UNION ALL
                SELECT u.id, u.parent_id FROM x_org_unit u JOIN sup s ON u.id = s.parent_id WHERE u.deleted_at IS NULL
            )
            SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit
            WHERE id IN (SELECT id FROM sup) AND deleted_at IS NULL
            ORDER BY level ASC, sort ASC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        let parent_id: Option<String> = row.get("parent_id");
        let level: i32 = row.get("level");
        let sort: i32 = row.get("sort");
        Value::Object(serde_json::Map::from_iter(
            [
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("level".to_string(), Value::Number(serde_json::Number::from(level))),
                ("sort".to_string(), Value::Number(serde_json::Number::from(sort))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]
            .into_iter()
            .chain(parent_id.map(|v| ("\"parentId\"".to_string(), Value::String(v)))),
        ))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn organization_assemble_control_unit_list_flag_sup_nested_type_type(
    pool: Extension<Pool>,
    axum::extract::Path((flag, _ty)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    tracing::warn!("unit sup/nested/type: type param ignored (x_org_unit has no type column)");
    organization_assemble_control_unit_list_flag_sup_nested(pool, axum::extract::Path(flag)).await
}

pub async fn organization_assemble_control_person_list_like(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<PersonLikeRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name_pattern = req.name.unwrap_or_default();
    let like_pattern = format!("%{}%", name_pattern);

    let rows = client
        .query(
            "SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person WHERE name ILIKE $1 ORDER BY create_time::text DESC",
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn export_export_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let export_id = uuid::Uuid::new_v4().to_string();
    client
        .execute("INSERT INTO x_org_export (id, type, status, create_time) VALUES ($1, 'all', 'pending', NOW())", &[&export_id])
        .await
        .map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, type, status, create_time::text FROM x_org_export WHERE id = $1", &[&export_id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("export record not found after creation"))),
    }
}

pub async fn export_result_flag_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, type, status, file_url, create_time::text FROM x_org_export WHERE id = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("\"fileUrl\"".to_string(), Value::String(row.get("file_url"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("export not found"))),
    }
}

pub async fn export_zhengwudingding_person(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let export_id = uuid::Uuid::new_v4().to_string();
    client
        .execute("INSERT INTO x_org_export (id, type, status, create_time) VALUES ($1, 'zhengwudingding', 'pending', NOW())", &[&export_id])
        .await
        .map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, type, status, create_time::text FROM x_org_export WHERE id = $1", &[&export_id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("export record not found after creation"))),
    }
}



pub async fn group_list_like_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn group_list_like_pinyin(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn group_list_like_pinyin_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}




pub async fn group_list_pinyininitial(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn group_list_pinyininitial_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}






pub async fn group_list_flag_sub_direct(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT g.id, g.name, g.unit_id, g.type, g.creator, g.create_time::text FROM x_org_group g WHERE g.unit_id = $1 AND g.id != $1 AND g.deleted_at IS NULL ORDER BY g.create_time::text DESC",
            &[&flag],
        )
        .await
        .map_err(|e| { eprintln!("DIAG org_sub_direct query err: {:?}", e); AppError::Internal })?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn group_list_flag_sub_nested(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "WITH RECURSIVE sub AS (SELECT id FROM x_org_unit WHERE id = $1 AND deleted_at IS NULL UNION ALL SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) SELECT g.id, g.name, g.unit_id, g.type, g.creator, g.create_time::text FROM x_org_group g JOIN sub s ON g.unit_id = s.id WHERE g.id != $2 AND g.deleted_at IS NULL ORDER BY g.create_time::text DESC",
            &[&flag, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn group_list_flag_sup_direct(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT g2.id, g2.name, g2.unit_id, g2.type, g2.creator, g2.create_time::text \
             FROM x_org_group g1 \
             JOIN x_org_unit u1 ON g1.unit_id = u1.id AND u1.deleted_at IS NULL \
             JOIN x_org_unit pu ON pu.id = u1.parent_id AND pu.deleted_at IS NULL \
             JOIN x_org_group g2 ON g2.unit_id = pu.id AND g2.deleted_at IS NULL \
             WHERE g1.id = $1 AND g1.deleted_at IS NULL AND g2.id != $1 \
             ORDER BY g2.create_time DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn group_list_flag_sup_nested(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "WITH RECURSIVE sup AS (\
             SELECT id, parent_id FROM x_org_unit WHERE id = $1 AND deleted_at IS NULL \
             UNION ALL \
             SELECT u.id, u.parent_id FROM x_org_unit u JOIN sup s ON u.id = s.parent_id WHERE u.deleted_at IS NULL \
             ) \
             SELECT g.id, g.name, g.unit_id, g.type, g.creator, g.create_time::text \
             FROM x_org_group g JOIN sup s ON g.unit_id = s.id \
             WHERE g.id != $1 AND g.deleted_at IS NULL \
             ORDER BY g.create_time DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn group_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("group not found"))),
    }
}

pub async fn group_flag_add_member(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = flag.clone();
    let result = client
        .execute(
            "INSERT INTO x_org_group_member (group_id, person_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            &[&flag, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn group_flag_add_member_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = flag.clone();
    let result = client
        .execute(
            "INSERT INTO x_org_group_member (group_id, person_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            &[&flag, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("saved".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn group_flag_delete_member(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = flag.clone();
    let result = client
        .execute(
            "DELETE FROM x_org_group_member WHERE group_id = $1 AND person_id = $2",
            &[&flag, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn group_flag_delete_member_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person_id = flag.clone();
    let result = client
        .execute(
            "DELETE FROM x_org_group_member WHERE group_id = $1 AND person_id = $2",
            &[&flag, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("deleted".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn group_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = row {
        client
            .execute("UPDATE x_org_group SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&flag])
            .await
            .map_err(|_| AppError::Internal)?;

        let result = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]));
        Ok(Json(ActionResult::success(result)))
    } else {
        Ok(Json(ActionResult::error("group not found")))
    }
}

pub async fn group_flag_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("group not found"))),
    }
}



pub async fn identity_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, name, unit_id FROM x_org_identity WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("identity not found"))),
    }
}

pub async fn identity_list_like_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn identity_list_like_pinyin(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn identity_list_like_pinyin_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}


pub async fn identity_list_pinyininitial(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn identity_list_pinyininitial_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}










pub async fn identity_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("identity not found"))),
    }
}

pub async fn identity_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = row {
        client
            .execute("UPDATE x_org_identity SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&flag])
            .await
            .map_err(|_| AppError::Internal)?;

        let result = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]));
        Ok(Json(ActionResult::success(result)))
    } else {
        Ok(Json(ActionResult::error("identity not found")))
    }
}

pub async fn identity_flag_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("identity not found"))),
    }
}





pub async fn inputperson_template(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, person_id, status, message, create_time::text FROM x_org_import_result ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn inputperson_wipe(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, person_id, status, message, create_time::text FROM x_org_import_result ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("message".to_string(), Value::String(row.get("message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}


pub async fn permissionsetting_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, creator, create_time::text FROM x_org_permission_setting ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn permissionsetting_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, creator, create_time::text FROM x_org_permission_setting WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("permission setting not found"))),
    }
}

pub async fn permissionsetting_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, creator, create_time::text FROM x_org_permission_setting WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = row {
        client
            .execute("UPDATE x_org_permission_setting SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&flag])
            .await
            .map_err(|_| AppError::Internal)?;

        let result = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]));
        Ok(Json(ActionResult::success(result)))
    } else {
        Ok(Json(ActionResult::error("permission setting not found")))
    }
}

pub async fn permissionsetting_flag_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, creator, create_time::text FROM x_org_permission_setting WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("permission setting not found"))),
    }
}







pub async fn personattribute_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_person_attribute WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
                ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("person attribute not found"))),
    }
}

pub async fn personattribute_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_person_attribute WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = row {
        client
            .execute("UPDATE x_org_person_attribute SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&flag])
            .await
            .map_err(|_| AppError::Internal)?;

        let result = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("personId".to_string(), Value::String(row.get("person_id"))),
            ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
            ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]));
        Ok(Json(ActionResult::success(result)))
    } else {
        Ok(Json(ActionResult::error("person attribute not found")))
    }
}

pub async fn personattribute_flag_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, person_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_person_attribute WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("personId".to_string(), Value::String(row.get("person_id"))),
                ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
                ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("person attribute not found"))),
    }
}









pub async fn personcard_listgrouptypes(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mobile".to_string(), Value::String(row.get("mobile"))),
                ("email".to_string(), Value::String(row.get("email"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}


pub async fn personcard_listpaging_page_page_size_size_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);

    let rows = client
        .query("SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person WHERE deleted_at IS NULL ORDER BY create_time::text DESC LIMIT $1::bigint OFFSET $2::bigint", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), Value::String(row.get("mobile"))),
            ("email".to_string(), Value::String(row.get("email"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}


pub async fn personcard_listpagingwithgroup_page_page_size_size_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);

    let rows = client
        .query("SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person WHERE deleted_at IS NULL ORDER BY create_time::text DESC LIMIT $1::bigint OFFSET $2::bigint", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), Value::String(row.get("mobile"))),
            ("email".to_string(), Value::String(row.get("email"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}

pub async fn personcard_mylist(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mobile".to_string(), Value::String(row.get("mobile"))),
                ("email".to_string(), Value::String(row.get("email"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn personcard_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person WHERE id = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mobile".to_string(), Value::String(row.get("mobile"))),
                ("email".to_string(), Value::String(row.get("email"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("person not found"))),
    }
}

pub async fn personcard_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person WHERE id = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = row {
        client
            .execute("UPDATE x_org_person SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&flag])
            .await
            .map_err(|_| AppError::Internal)?;

        let result = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), Value::String(row.get("mobile"))),
            ("email".to_string(), Value::String(row.get("email"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]));
        Ok(Json(ActionResult::success(result)))
    } else {
        Ok(Json(ActionResult::error("person not found")))
    }
}





pub async fn role_list_like_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, description, creator, create_time::text FROM x_org_role ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn role_list_like_pinyin(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, description, creator, create_time::text FROM x_org_role ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn role_list_like_pinyin_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, description, creator, create_time::text FROM x_org_role ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}


pub async fn role_list_pinyininitial(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, description, creator, create_time::text FROM x_org_role ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}
pub async fn role_list_pinyininitial_mockputtopost(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, description, creator, create_time::text FROM x_org_role ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}


pub async fn role_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, description, creator, create_time::text FROM x_org_role WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = row {
        client
            .execute("UPDATE x_org_role SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&flag])
            .await
            .map_err(|_| AppError::Internal)?;

        let result = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("description".to_string(), Value::String(row.get("description"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]));
        Ok(Json(ActionResult::success(result)))
    } else {
        Ok(Json(ActionResult::error("role not found")))
    }
}

pub async fn role_flag_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, description, creator, create_time::text FROM x_org_role WHERE id = $1",
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







pub async fn unitattribute_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, unit_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_unit_attribute WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
                ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("unit attribute not found"))),
    }
}

pub async fn unitattribute_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, unit_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_unit_attribute WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = row {
        client
            .execute("UPDATE x_org_unit_attribute SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL", &[&flag])
            .await
            .map_err(|_| AppError::Internal)?;

        let result = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
            ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]));
        Ok(Json(ActionResult::success(result)))
    } else {
        Ok(Json(ActionResult::error("unit attribute not found")))
    }
}

pub async fn unitattribute_flag_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, unit_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_unit_attribute WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
                ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("unit attribute not found"))),
    }
}

pub async fn unitduty_distinct_name(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT DISTINCT name FROM x_org_duty ORDER BY name", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([("name".to_string(), Value::String(row.get("name")))]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}











pub async fn unitduty_flag_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_org_duty SET sort = sort + 1 WHERE id = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn unitduty_flag_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_org_duty WHERE id = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn unitduty_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty WHERE id = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("identityId".to_string(), Value::String(row.get("identity_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("unit duty not found"))),
    }
}

pub async fn unitduty_update_member(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty ORDER BY create_time::text DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitduty_list_flag_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty WHERE id < $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitduty_list_flag_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty WHERE id > $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitduty_list_unit_unitFlag(
    pool: Extension<Pool>,
    axum::extract::Path(unit_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty WHERE unit_id = $1 ORDER BY create_time::text DESC", &[&unit_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitduty_list_name_name(
    pool: Extension<Pool>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", name);
    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty WHERE name ILIKE $1 ORDER BY create_time::text DESC", &[&pattern])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitduty_list_like(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let pattern = format!("%{}%", name);
    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty WHERE name ILIKE $1 ORDER BY create_time::text DESC", &[&pattern])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitduty_list_identity_identityFlag(
    pool: Extension<Pool>,
    axum::extract::Path(identity_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_duty WHERE identity_id = $1 ORDER BY create_time::text DESC", &[&identity_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitduty_distinct_name_like_key(
    pool: Extension<Pool>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let pattern = format!("%{}%", key);
    let rows = client
        .query("SELECT DISTINCT name FROM x_org_duty WHERE name ILIKE $1 ORDER BY name", &[&pattern])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([("name".to_string(), Value::String(row.get("name")))]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}





pub async fn unitattribute_list_flag_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, unit_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_unit_attribute ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, unit_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_unit_attribute WHERE id < $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
            ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitattribute_list_flag_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, unit_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_unit_attribute ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, unit_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_unit_attribute WHERE id > $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
            ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn unitattribute_list_unit_flag(
    pool: Extension<Pool>,
    axum::extract::Path(unit_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, unit_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_unit_attribute WHERE unit_id = $1 ORDER BY create_time::text DESC", &[&unit_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
            ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}



pub async fn role_list_flag_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, name, description, creator, create_time::text FROM x_org_role ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, name, description, creator, create_time::text FROM x_org_role WHERE id < $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("description".to_string(), Value::String(row.get("description"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}



pub async fn role_list_person_personFlag(
    pool: Extension<Pool>,
    axum::extract::Path(person_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, description, creator, create_time::text FROM x_org_role WHERE creator = $1 ORDER BY create_time::text DESC", &[&person_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("description".to_string(), Value::String(row.get("description"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}




pub async fn role_list_like(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let pattern = format!("%{}%", name);
    let rows = client
        .query("SELECT id, name, description, creator, create_time::text FROM x_org_role WHERE name ILIKE $1 ORDER BY create_time::text DESC", &[&pattern])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("description".to_string(), Value::String(row.get("description"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn role_list_group_groupFlag(
    pool: Extension<Pool>,
    axum::extract::Path(group_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT r.id, r.name, r.description, r.creator, r.create_time::text FROM x_org_role r JOIN x_org_group_role gr ON gr.role_id = r.id WHERE gr.group_id = $1 AND r.deleted_at IS NULL ORDER BY r.create_time::text DESC", &[&group_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("description".to_string(), Value::String(row.get("description"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}





pub async fn personcard_listpagingwithgroup_page_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);

    let rows = client
        .query("SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person ORDER BY create_time::text DESC LIMIT $1::bigint OFFSET $2::bigint", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), Value::String(row.get("mobile"))),
            ("email".to_string(), Value::String(row.get("email"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}


pub async fn personcard_listpaging_page_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);

    let rows = client
        .query("SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person ORDER BY create_time::text DESC LIMIT $1::bigint OFFSET $2::bigint", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), Value::String(row.get("mobile"))),
            ("email".to_string(), Value::String(row.get("email"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}


pub async fn personcard_listVCf_idList(
    pool: Extension<Pool>,
    axum::extract::Path(id_list): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let ids: Vec<&str> = id_list.split(',').collect();
    let rows = client
        .query("SELECT id, name, mobile, email, unit_id FROM x_org_person WHERE id = ANY($1) AND deleted_at IS NULL", &[&ids])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), Value::String(row.get("mobile"))),
            ("email".to_string(), Value::String(row.get("email"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn personcard_listPersonalVCf_idList(
    pool: Extension<Pool>,
    axum::extract::Path(id_list): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let ids: Vec<&str> = id_list.split(',').collect();
    let rows = client
        .query("SELECT id, name, mobile, email, unit_id FROM x_org_person WHERE id = ANY($1) AND deleted_at IS NULL", &[&ids])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), Value::String(row.get("mobile"))),
            ("email".to_string(), Value::String(row.get("email"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn personcard_createQR_cardId(
    pool: Extension<Pool>,
    axum::extract::Path(card_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let qr_code = format!("https://api.example.com/personcard/qr/{}", card_id);
    client
        .execute("UPDATE x_org_person SET qr_code = $1 WHERE id = $2", &[&qr_code, &card_id])
        .await
        .map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person WHERE id = $1",
            &[&card_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mobile".to_string(), Value::String(row.get("mobile"))),
                ("email".to_string(), Value::String(row.get("email"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("person not found"))),
    }
}

pub async fn personcard_createCode_cardId(
    pool: Extension<Pool>,
    axum::extract::Path(card_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let code = uuid::Uuid::new_v4().to_string();
    client
        .execute("UPDATE x_org_person SET card_code = $1 WHERE id = $2", &[&code, &card_id])
        .await
        .map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, mobile, email, unit_id, creator, create_time::text FROM x_org_person WHERE id = $1",
            &[&card_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mobile".to_string(), Value::String(row.get("mobile"))),
                ("email".to_string(), Value::String(row.get("email"))),
                ("unitId".to_string(), Value::String(row.get("unit_id"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("person not found"))),
    }
}




pub async fn personattribute_list_flag_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, person_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_person_attribute ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, person_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_person_attribute WHERE id < $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("personId".to_string(), Value::String(row.get("person_id"))),
            ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
            ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn personattribute_list_flag_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, person_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_person_attribute ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, person_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_person_attribute WHERE id > $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("personId".to_string(), Value::String(row.get("person_id"))),
            ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
            ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn personattribute_list_person_personFlag(
    pool: Extension<Pool>,
    axum::extract::Path(person_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, person_id, attribute_key, attribute_value, creator, create_time::text FROM x_org_person_attribute WHERE person_id = $1 AND deleted_at IS NULL ORDER BY create_time::text DESC", &[&person_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("personId".to_string(), Value::String(row.get("person_id"))),
            ("attributeKey".to_string(), Value::String(row.get("attribute_key"))),
            ("attributeValue".to_string(), Value::String(row.get("attribute_value"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}





pub async fn loginrecord_stream(
    pool: Extension<Pool>,
    axum::extract::Path(stream): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, person_id, login_time, ip, device FROM x_org_login_record WHERE stream = $1 ORDER BY login_time DESC LIMIT 100", &[&stream])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("personId".to_string(), Value::String(row.get("person_id"))),
            ("loginTime".to_string(), Value::String(row.get("login_time"))),
            ("ip".to_string(), Value::String(row.get("ip"))),
            ("device".to_string(), Value::String(row.get("device"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}



pub async fn inputperson_result_flag_flag(
    pool: Extension<Pool>,
    axum::extract::Path(flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, person_id, status, message, create_time::text FROM x_org_import_result WHERE import_id = $1 ORDER BY create_time::text DESC", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("personId".to_string(), Value::String(row.get("person_id"))),
            ("status".to_string(), Value::String(row.get("status"))),
            ("message".to_string(), Value::String(row.get("message"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn identity_flag_order_before_followFlag(
    pool: Extension<Pool>,
    axum::extract::Path((flag, follow_flag)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let identity_row = client
        .query_opt("SELECT id, unit_id FROM x_org_identity WHERE id = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let identity_row = match identity_row {
        Some(row) => row,
        None => return Ok(Json(ActionResult::error("identity not found"))),
    };

    let unit_id: String = identity_row.get("unit_id");

    let follow_identity = if follow_flag != "(0)" {
        client
            .query_opt("SELECT id FROM x_org_identity WHERE id = $1", &[&follow_flag])
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        None
    };

    let rows = client
        .query("SELECT id FROM x_org_identity WHERE unit_id = $1 ORDER BY create_time::text DESC", &[&unit_id])
        .await
        .map_err(|_| AppError::Internal)?;

    let mut ids: Vec<String> = rows.iter().map(|row| row.get("id")).collect();

    if follow_identity.is_none() {
        ids.retain(|id| id != &flag);
        ids.push(flag);
    } else {
        let follow_id: String = follow_identity.map(|row| row.get("id")).unwrap_or_default();
        let mut new_ids = Vec::new();
        for id in ids {
            if id == follow_id {
                new_ids.push(flag.clone());
            }
            if id != flag {
                new_ids.push(id);
            }
        }
        ids = new_ids;
    }

    let mut order_result: u64 = 0;
    for (index, id) in ids.iter().enumerate() {
        let order_num = (index + 1) as i32;
        order_result += client
            .execute("UPDATE x_org_identity SET order_number = $1 WHERE id = $2", &[&order_num, id])
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(order_result > 0))]),
    ))))
}




pub async fn identity_list_flag_unitduty_name_unitDutyName(
    pool: Extension<Pool>,
    axum::extract::Path((flag, unit_duty_name)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE id > $1 AND unit_id IN (SELECT id FROM x_org_duty WHERE name = $2) ORDER BY create_time::text DESC LIMIT 10", &[&flag, &unit_duty_name])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn identity_list_flag_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE id < $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn identity_list_flag_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE id > $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn identity_list_unitduty_name_unitDutyName(
    pool: Extension<Pool>,
    axum::extract::Path(unit_duty_name): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE unit_id IN (SELECT id FROM x_org_duty WHERE name = $1) ORDER BY create_time::text DESC", &[&unit_duty_name])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn identity_list_unit_unitFlag(
    pool: Extension<Pool>,
    axum::extract::Path(unit_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE unit_id = $1 AND deleted_at IS NULL ORDER BY create_time::text DESC", &[&unit_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}



pub async fn identity_list_person_personFlag(
    pool: Extension<Pool>,
    axum::extract::Path(person_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE person_id = $1 AND deleted_at IS NULL ORDER BY create_time::text DESC", &[&person_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}




pub async fn identity_list_like(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let pattern = format!("%{}%", name);
    let rows = client
        .query("SELECT id, name, unit_id, identity_id, creator, create_time::text FROM x_org_identity WHERE name ILIKE $1 ORDER BY create_time::text DESC", &[&pattern])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("identityId".to_string(), Value::String(row.get("identity_id"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}












pub async fn group_list_flag_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group WHERE id < $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn group_list_flag_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((flag, count_str)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = count_str.parse().unwrap_or(10);
    let rows = if flag == "0" {
        client.query(
            "SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group ORDER BY create_time::text DESC LIMIT $1::bigint",
            &[&count],
        ).await.map_err(|_| AppError::Internal)?
    } else {
        client.query(
            "SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group WHERE id > $1 ORDER BY create_time::text DESC LIMIT $2::bigint",
            &[&flag, &count],
        ).await.map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn group_list_role_roleFlag(
    pool: Extension<Pool>,
    axum::extract::Path(role_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group WHERE id IN (SELECT group_id FROM x_org_group_role WHERE role_id = $1) AND deleted_at IS NULL ORDER BY create_time::text DESC", &[&role_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}



pub async fn group_list_person_personFlag_sup_nested(
    pool: Extension<Pool>,
    axum::extract::Path(person_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("WITH RECURSIVE sup AS (SELECT id, name, unit_id, parent_id FROM x_org_unit WHERE id IN (SELECT unit_id FROM x_org_group WHERE id IN (SELECT group_id FROM x_org_group_member WHERE person_id = $1) AND deleted_at IS NULL) AND deleted_at IS NULL UNION ALL SELECT u.id, u.name, u.unit_id, u.parent_id FROM x_org_unit u JOIN sup s ON u.id = s.parent_id WHERE u.deleted_at IS NULL) SELECT DISTINCT g.id, g.name, g.unit_id, g.type, g.creator, g.create_time::text FROM x_org_group g JOIN sup s ON g.unit_id = s.id WHERE g.deleted_at IS NULL ORDER BY g.create_time::text DESC", &[&person_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub async fn group_list_person_personFlag_sup_direct(
    pool: Extension<Pool>,
    axum::extract::Path(person_flag): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group WHERE unit_id IN (SELECT unit_id FROM x_org_group WHERE id IN (SELECT group_id FROM x_org_group_member WHERE person_id = $1) AND deleted_at IS NULL) AND deleted_at IS NULL ORDER BY create_time::text DESC", &[&person_flag])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}




pub async fn group_list_like(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let pattern = format!("%{}%", name);
    let rows = client
        .query("SELECT id, name, unit_id, type, creator, create_time::text FROM x_org_group WHERE name ILIKE $1 ORDER BY create_time::text DESC", &[&pattern])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("unitId".to_string(), Value::String(row.get("unit_id"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("creator".to_string(), Value::String(row.get("creator"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

pub fn router(pool: deadpool_postgres::Pool) -> Router {
    let router = Router::new()
    .route("/jaxrs/organization/assemble/control/export/export/all", get(export_export_all))
    .route("/jaxrs/organization/assemble/control/export/result/flag/{flag}", get(export_result_flag_flag))
    .route("/jaxrs/organization/assemble/control/export/zhengwudingding/person", get(export_zhengwudingding_person))
    .route("/jaxrs/organization/assemble/control/group/list/like", get(group_list_like))
    .route("/jaxrs/organization/assemble/control/group/list/like/mockputtopost", get(group_list_like_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/list/like/pinyin", get(group_list_like_pinyin))
    .route("/jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost", get(group_list_like_pinyin_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/direct", get(group_list_person_personFlag_sup_direct))
    .route("/jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/nested", get(group_list_person_personFlag_sup_nested))
    .route("/jaxrs/organization/assemble/control/group/list/pinyininitial", get(group_list_pinyininitial))
    .route("/jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost", get(group_list_pinyininitial_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/list/role/{roleFlag}", get(group_list_role_roleFlag))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/next/{count}", get(group_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/prev/{count}", get(group_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/sub/direct", get(group_list_flag_sub_direct))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/sub/nested", get(group_list_flag_sub_nested))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/sup/direct", get(group_list_flag_sup_direct))
    .route("/jaxrs/organization/assemble/control/group/list/{flag}/sup/nested", get(group_list_flag_sup_nested))
    .route("/jaxrs/organization/assemble/control/group/{flag}", get(group_flag))
    .route("/jaxrs/organization/assemble/control/group/{flag}/add/member", get(group_flag_add_member))
    .route("/jaxrs/organization/assemble/control/group/{flag}/add/member/mockputtopost", get(group_flag_add_member_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/{flag}/delete/member", get(group_flag_delete_member))
    .route("/jaxrs/organization/assemble/control/group/{flag}/delete/member/mockputtopost", get(group_flag_delete_member_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/{flag}/mockdeletetoget", get(group_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/group/{flag}/mockputtopost", get(group_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/list/like", get(identity_list_like))
    .route("/jaxrs/organization/assemble/control/identity/list/like/mockputtopost", get(identity_list_like_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/list/like/pinyin", get(identity_list_like_pinyin))
    .route("/jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost", get(identity_list_like_pinyin_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/list/person/{personFlag}", get(identity_list_person_personFlag))
    .route("/jaxrs/organization/assemble/control/identity/list/pinyininitial", get(identity_list_pinyininitial))
    .route("/jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost", get(identity_list_pinyininitial_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/list/unit/{unitFlag}", get(identity_list_unit_unitFlag))
    .route("/jaxrs/organization/assemble/control/identity/list/unitduty/name/{unitDutyName}", get(identity_list_unitduty_name_unitDutyName))
    .route("/jaxrs/organization/assemble/control/identity/list/{flag}/next/{count}", get(identity_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/identity/list/{flag}/prev/{count}", get(identity_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/identity/list/{flag}/unitduty/name/{unitDutyName}", get(identity_list_flag_unitduty_name_unitDutyName))
    .route("/jaxrs/organization/assemble/control/identity/{flag}", get(identity_flag))
    .route("/jaxrs/organization/assemble/control/identity/{flag}/mockdeletetoget", get(identity_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/identity/{flag}/mockputtopost", get(identity_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/identity/{flag}/order/before/{followFlag}", get(identity_flag_order_before_followFlag))
    .route("/jaxrs/organization/assemble/control/inputperson/result/flag/{flag}", get(inputperson_result_flag_flag))
    .route("/jaxrs/organization/assemble/control/inputperson/template", get(inputperson_template))
    .route("/jaxrs/organization/assemble/control/inputperson/wipe", get(inputperson_wipe))
    .route("/jaxrs/organization/assemble/control/loginrecord/{stream}", get(loginrecord_stream))
    .route("/jaxrs/organization/assemble/control/permissionsetting/list", get(permissionsetting_list))
    .route("/jaxrs/organization/assemble/control/permissionsetting/{flag}", get(permissionsetting_flag))
    .route("/jaxrs/organization/assemble/control/permissionsetting/{flag}/mockdeletetoget", get(permissionsetting_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/permissionsetting/{flag}/mockputtopost", get(permissionsetting_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/personattribute/list/person/{personFlag}", get(personattribute_list_person_personFlag))
    .route("/jaxrs/organization/assemble/control/personattribute/list/{flag}/next/{count}", get(personattribute_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/personattribute/list/{flag}/prev/{count}", get(personattribute_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/personattribute/{flag}", get(personattribute_flag))
    .route("/jaxrs/organization/assemble/control/personattribute/{flag}/mockdeletetoget", get(personattribute_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/personattribute/{flag}/mockputtopost", get(personattribute_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/personcard/createCode/{cardId}", get(personcard_createCode_cardId))
    .route("/jaxrs/organization/assemble/control/personcard/createQR/{cardId}", get(personcard_createQR_cardId))
    .route("/jaxrs/organization/assemble/control/personcard/listPersonalVCf/{idList}", get(personcard_listPersonalVCf_idList))
    .route("/jaxrs/organization/assemble/control/personcard/listVCf/{idList}", get(personcard_listVCf_idList))
    .route("/jaxrs/organization/assemble/control/personcard/listgrouptypes", get(personcard_listgrouptypes))
    .route("/jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}", get(personcard_listpaging_page_page_size_size))
    .route("/jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}/mockputtopost", get(personcard_listpaging_page_page_size_size_mockputtopost))
    .route("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}", get(personcard_listpagingwithgroup_page_page_size_size))
    .route("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}/mockputtopost", get(personcard_listpagingwithgroup_page_page_size_size_mockputtopost))
    .route("/jaxrs/organization/assemble/control/personcard/mylist", get(personcard_mylist))
    .route("/jaxrs/organization/assemble/control/personcard/{flag}", get(personcard_flag))
    .route("/jaxrs/organization/assemble/control/personcard/{flag}/mockdeletetoget", get(personcard_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/role/list/group/{groupFlag}", get(role_list_group_groupFlag))
    .route("/jaxrs/organization/assemble/control/role/list/like", get(role_list_like))
    .route("/jaxrs/organization/assemble/control/role/list/like/mockputtopost", get(role_list_like_mockputtopost))
    .route("/jaxrs/organization/assemble/control/role/list/like/pinyin", get(role_list_like_pinyin))
    .route("/jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost", get(role_list_like_pinyin_mockputtopost))
    .route("/jaxrs/organization/assemble/control/role/list/person/{personFlag}", get(role_list_person_personFlag))
    .route("/jaxrs/organization/assemble/control/role/list/pinyininitial", get(role_list_pinyininitial))
    .route("/jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost", get(role_list_pinyininitial_mockputtopost))
    .route("/jaxrs/organization/assemble/control/role/list/{flag}/next/{count}", get(organization_assemble_control_role_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/role/list/{flag}/prev/{count}", get(role_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/role/{flag}", get(organization_assemble_control_role_flag))
    .route("/jaxrs/organization/assemble/control/role/{flag}/mockdeletetoget", get(role_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/role/{flag}/mockputtopost", get(role_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/unitattribute/list/unit/{flag}", get(unitattribute_list_unit_flag))
    .route("/jaxrs/organization/assemble/control/unitattribute/list/{flag}/next/{count}", get(unitattribute_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/unitattribute/list/{flag}/prev/{count}", get(unitattribute_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/unitattribute/{flag}", get(unitattribute_flag))
    .route("/jaxrs/organization/assemble/control/unitattribute/{flag}/mockdeletetoget", get(unitattribute_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/unitattribute/{flag}/mockputtopost", get(unitattribute_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/unitduty/distinct/name", get(unitduty_distinct_name))
    .route("/jaxrs/organization/assemble/control/unitduty/distinct/name/like/{key}", get(unitduty_distinct_name_like_key))
    .route("/jaxrs/organization/assemble/control/unitduty/list/identity/{identityFlag}", get(unitduty_list_identity_identityFlag))
    .route("/jaxrs/organization/assemble/control/unitduty/list/like", get(unitduty_list_like))
    .route("/jaxrs/organization/assemble/control/unitduty/list/name/{name}", get(unitduty_list_name_name))
    .route("/jaxrs/organization/assemble/control/unitduty/list/unit/{unitFlag}", get(unitduty_list_unit_unitFlag))
    .route("/jaxrs/organization/assemble/control/unitduty/list/{flag}/next/{count}", get(unitduty_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/unitduty/list/{flag}/prev/{count}", get(unitduty_list_flag_prev_count))
    .route("/jaxrs/organization/assemble/control/unitduty/update/member", get(unitduty_update_member))
    .route("/jaxrs/organization/assemble/control/unitduty/{flag}", get(unitduty_flag))
    .route("/jaxrs/organization/assemble/control/unitduty/{flag}/mockdeletetoget", get(unitduty_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/unitduty/{flag}/mockputtopost", get(unitduty_flag_mockputtopost))
    .route("/jaxrs/organization/assemble/control/unit/list/{flag}/next/{count}", get(organization_assemble_control_unit_list_flag_next_count))
    .route("/jaxrs/organization/assemble/control/unit/list/{flag}/sub/nested", get(organization_assemble_control_unit_list_flag_sub_nested))
    .route("/jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested", get(organization_assemble_control_unit_list_flag_sup_nested))
    .route("/jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested/type/{type}", get(organization_assemble_control_unit_list_flag_sup_nested_type_type))
    .route("/jaxrs/organization/assemble/control/unit/{flag}", get(organization_assemble_control_unit_flag))
    .route("/jaxrs/organization/assemble/control/person/list/like", post(organization_assemble_control_person_list_like))
    .route("/jaxrs/identity/{id}", get(identity_id))
    .route("/jaxrs/organization/assemble/control/group/{flag}/delete/member", delete(group_flag_delete_member))
    .route("/jaxrs/organization/assemble/control/group/{flag}/delete/member/mockputtopost", delete(group_flag_delete_member_mockputtopost))
    .route("/jaxrs/organization/assemble/control/group/{flag}/mockdeletetoget", delete(group_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/identity/{flag}/mockdeletetoget", delete(identity_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/permissionsetting/{flag}/mockdeletetoget", delete(permissionsetting_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/personattribute/{flag}/mockdeletetoget", delete(personattribute_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/personcard/{flag}/mockdeletetoget", delete(personcard_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/role/{flag}/mockdeletetoget", delete(role_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/unitattribute/{flag}/mockdeletetoget", delete(unitattribute_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/unitduty/{flag}/mockdeletetoget", delete(unitduty_flag_mockdeletetoget))
    .route("/jaxrs/organization/assemble/control/unitduty/update/member", put(unitduty_update_member))
    .merge(u2_router::router())
    .layer(Extension(pool));
    router
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;
#[cfg(test)]
mod tests_u2;




