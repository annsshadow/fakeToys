use super::u2_helpers::*;
use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::error::AppError;

fn unit_row_json(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    entity_row_json(row, UNIT_EXTRA)
}

pub async fn unit_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    let parent_flag = opt(&body, &["parentId", "superior"]).unwrap_or_default().to_string();
    let parent_id = if parent_flag.is_empty() {
        String::new()
    } else {
        match resolve_generic_id(&client, UNIT_TABLE, &parent_flag).await? {
            Some(id) => id,
            None => return Err(AppError::BadRequest("parent unit not found".to_string())),
        }
    };
    if normalized_name_dup(&client, UNIT_TABLE, "parent_id", &parent_id, &name).await? {
        return err("unit already exists");
    }
    let unit_type = opt(&body, &["type"]).unwrap_or_default().to_string();
    let sort = body.get("sort").and_then(|v| v.as_i64()).unwrap_or(0).to_string();
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_unit (id, name, parent_id, level, sort, type, creator)
             VALUES ($1, $2, NULLIF($3,''),
                     COALESCE((SELECT level + 1 FROM x_org_unit WHERE id = NULLIF($3,'')), 0),
                     $4::int, NULLIF($5,''), $6)",
            &[&id, &name, &parent_id, &sort, &unit_type, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn unit_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(uid) = resolve_generic_id(&client, UNIT_TABLE, &flag).await? else {
        return err("unit not found");
    };
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let unit_type = opt(&body, &["type"]).unwrap_or_default().to_string();
    let pinyin = opt(&body, &["pinyinInitial"]).unwrap_or_default().to_string();
    let sort = body
        .get("sort")
        .and_then(|v| v.as_i64())
        .map(|v| v.to_string())
        .unwrap_or_default();
    let updated = client
        .execute(
            "UPDATE x_org_unit SET
                name = CASE WHEN $2 = '' THEN name ELSE $2 END,
                type = CASE WHEN $3 = '' THEN type ELSE NULLIF($3, '') END,
                pinyin_initial = CASE WHEN $4 = '' THEN pinyin_initial ELSE $4 END,
                sort = CASE WHEN $5 = '' THEN sort ELSE $5::int END
             WHERE id = $1 AND deleted_at IS NULL",
            &[&uid, &name, &unit_type, &pinyin, &sort],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("unit not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(uid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn unit_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    unit_edit(pool, session, Path(flag), Json(body)).await
}

pub async fn unit_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, UNIT_TABLE, &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))].into_iter().collect(),
        )),
        None => err("unit not found"),
    }
}

pub async fn unit_mock_delete_to_get(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    unit_delete(pool, session, Path(flag)).await
}

async fn top_units(pool: &Pool, unit_type: Option<&str>, java_bare: bool) -> HandlerResult {
    let client = client_of(pool).await?;
    let rows = match unit_type {
        Some(t) => {
            client
                .query(
                    "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id IS NULL AND deleted_at IS NULL AND type = $1 ORDER BY sort ASC, create_time::text DESC",
                    &[&t.to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?
        }
        None => {
            client
                .query(
                    "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id IS NULL AND deleted_at IS NULL ORDER BY sort ASC, create_time::text DESC",
                    &[],
                )
                .await
                .map_err(|_| AppError::Internal)?
        }
    };
    if java_bare {
        list_ok_java(rows.iter().map(unit_row_json).collect())
    } else {
        list_ok(rows.iter().map(unit_row_json).collect())
    }
}

pub async fn unit_get_root(pool: Extension<Pool>) -> HandlerResult {
    top_units(&pool, None, false).await
}

pub async fn unit_list_top_root(pool: Extension<Pool>) -> HandlerResult {
    top_units(&pool, None, true).await
}

pub async fn unit_list_top_with_type(
    pool: Extension<Pool>,
    Path(unit_type): Path<String>,
) -> HandlerResult {
    top_units(&pool, Some(&unit_type), true).await
}

pub async fn unit_control_top(pool: Extension<Pool>) -> HandlerResult {
    top_units(&pool, None, true).await
}

pub async fn unit_list_types(pool: Extension<Pool>) -> HandlerResult {
    let client = client_of(&pool).await?;
    let rows = client
        .query(
            "SELECT DISTINCT type FROM x_org_unit WHERE type IS NOT NULL AND type <> '' AND deleted_at IS NULL ORDER BY type",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .filter_map(|r| r.try_get::<_, Option<String>>(0).ok().flatten())
        .map(Value::String)
        .collect();
    list_ok(data)
}

pub async fn unit_list_prev(
    pool: Extension<Pool>,
    Path((flag, count)): Path<(String, i64)>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let limit = count.clamp(1, MAX_BATCH_IDS as i64).to_string();
    let rows = if flag == "0" || flag == "(0)" {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id IS NULL AND deleted_at IS NULL ORDER BY sort ASC, create_time::text ASC LIMIT $1::bigint",
                &[&limit],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id = $1 AND deleted_at IS NULL ORDER BY sort ASC, create_time::text ASC LIMIT $2::bigint",
                &[&flag, &limit],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };
    list_ok(rows.iter().map(unit_row_json).collect())
}

pub async fn unit_list_sub_direct(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id = $1 AND deleted_at IS NULL ORDER BY sort ASC, create_time::text DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(unit_row_json).collect())
}

pub async fn unit_list_sub_direct_with_type(
    pool: Extension<Pool>,
    Path((flag, unit_type)): Path<(String, String)>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id = $1 AND type = $2 AND deleted_at IS NULL ORDER BY sort ASC, create_time::text DESC",
            &[&flag, &unit_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(unit_row_json).collect())
}

async fn identity_unit_id(client: &deadpool_postgres::Client, identity_flag: &str) -> Result<Option<String>, AppError> {
    let row = client
        .query_opt(
            "SELECT unit_id FROM x_org_identity WHERE (id = $1 OR name = $1) AND deleted_at IS NULL",
            &[&identity_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.and_then(|r| r.try_get::<_, Option<String>>(0).ok().flatten()))
}

const ANCESTOR_CTE: &str = "WITH RECURSIVE chain(id, depth) AS (
    SELECT id, 0 FROM x_org_unit WHERE id = $1 AND deleted_at IS NULL
    UNION ALL
    SELECT u.id, c.depth + 1 FROM x_org_unit u JOIN chain c ON u.parent_id = c.id WHERE c.depth < 32
)";

pub async fn unit_get_with_identity_level(
    pool: Extension<Pool>,
    Path((identity_flag, level)): Path<(String, i32)>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let Some(unit_id) = identity_unit_id(&client, &identity_flag).await? else {
        return err("identity not found");
    };
    let level_str = level.to_string();
    let sql = format!(
        "{ANCESTOR_CTE}
         SELECT u.id, u.name, u.parent_id, u.level, u.sort, u.creator, u.create_time::text
           FROM x_org_unit u JOIN chain c ON c.id = u.id
          WHERE u.level = $2::int ORDER BY c.depth ASC LIMIT 1"
    );
    match client
        .query_opt(&sql, &[&unit_id, &level_str])
        .await
        .map_err(|_| AppError::Internal)?
    {
        Some(row) => ok(unit_row_json(&row)),
        None => err("unit with level not found"),
    }
}

pub async fn unit_get_with_identity_type(
    pool: Extension<Pool>,
    Path((identity_flag, unit_type)): Path<(String, String)>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let Some(unit_id) = identity_unit_id(&client, &identity_flag).await? else {
        return err("identity not found");
    };
    let sql = format!(
        "{ANCESTOR_CTE}
         SELECT u.id, u.name, u.parent_id, u.level, u.sort, u.creator, u.create_time::text
           FROM x_org_unit u JOIN chain c ON c.id = u.id
          WHERE u.type = $2 ORDER BY c.depth ASC"
    );
    let rows = client
        .query(&sql, &[&unit_id, &unit_type])
        .await
        .map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(unit_row_json).collect())
}

pub async fn unit_get_sup_direct(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let row = client
        .query_opt(
            "SELECT u2.id, u2.name, u2.parent_id, u2.level, u2.sort, u2.creator, u2.create_time::text
               FROM x_org_unit u1 JOIN x_org_unit u2 ON u2.id = u1.parent_id
              WHERE u1.id = $1 AND u1.deleted_at IS NULL AND u2.deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => ok(unit_row_json(&row)),
        None => err("superior unit not found"),
    }
}

async fn units_by_flags(pool: &Pool, flags: &[String]) -> HandlerResult {
    check_batch_len(flags.len())?;
    let client = client_of(pool).await?;
    let mut data = Vec::new();
    for f in flags {
        if let Some(id) = resolve_generic_id(&client, UNIT_TABLE, f).await? {
            if let Some(row) = client
                .query_opt(
                    "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE id = $1 AND deleted_at IS NULL",
                    &[&id],
                )
                .await
                .map_err(|_| AppError::Internal)?
            {
                data.push(unit_row_json(&row));
            }
        }
    }
    list_ok_java(data)
}

pub async fn unit_list_by_body(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let flags = json_str_list(&body, &["unitList"]);
    if flags.is_empty() {
        return generic_list_all(&pool, UNIT_TABLE, UNIT_EXTRA).await;
    }
    units_by_flags(&pool, &flags).await
}

pub async fn unit_list_controller(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let flags = json_str_list(&body, &["unitList", "controllerList"]);
    if flags.is_empty() {
        return generic_list_all(&pool, UNIT_TABLE, UNIT_EXTRA).await;
    }
    units_by_flags(&pool, &flags).await
}

pub async fn unit_list_with_unit_type(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let unit_type = opt(&body, &["type"])
        .filter(|v| !v.trim().is_empty())
        .ok_or_else(|| AppError::BadRequest("type is required".to_string()))?
        .to_string();
    let flags = json_str_list(&body, &["unitList"]);
    check_batch_len(flags.len())?;
    let rows = if flags.is_empty() {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE type = $1 AND deleted_at IS NULL ORDER BY sort ASC",
                &[&unit_type],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        let mut ids = Vec::new();
        for f in &flags {
            if let Some(id) = resolve_generic_id(&client, UNIT_TABLE, f).await? {
                ids.push(id);
            }
        }
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE type = $1 AND id = ANY($2) AND deleted_at IS NULL ORDER BY sort ASC",
                &[&unit_type, &ids],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };
    list_ok(rows.iter().map(unit_row_json).collect())
}

pub async fn unit_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let initials = initials_from_body(&body);
    generic_pinyininitial_filter(&pool, UNIT_TABLE, &initials, UNIT_EXTRA, true).await
}

pub async fn unit_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, UNIT_TABLE, key, false, UNIT_EXTRA, true).await
}

pub async fn unit_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key"]).unwrap_or_default();
    generic_like_search(&pool, UNIT_TABLE, key, true, UNIT_EXTRA, true).await
}

// 鈹€鈹€ identity 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

pub async fn identity_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    let unit_flag = opt(&body, &["unitId"]).unwrap_or_default().to_string();
    let unit_id = if unit_flag.is_empty() {
        String::new()
    } else {
        resolve_generic_id(&client, UNIT_TABLE, &unit_flag)
            .await?
            .ok_or_else(|| AppError::BadRequest("unit not found".to_string()))?
    };
    if normalized_name_dup(&client, IDENTITY_TABLE, "unit_id", &unit_id, &name).await? {
        return err("identity already exists");
    }
    let person_flag = opt(&body, &["personId", "person"]).unwrap_or_default().to_string();
    let person_id = if person_flag.is_empty() {
        String::new()
    } else {
        super::u2_person::resolve_person_id(&client, &person_flag)
            .await?
            .ok_or_else(|| AppError::BadRequest("person not found".to_string()))?
    };
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_identity (id, name, unit_id, person_id, creator) VALUES ($1, $2, NULLIF($3,''), NULLIF($4,''), $5)",
            &[&id, &name, &unit_id, &person_id, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn identity_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(iid) = resolve_generic_id(&client, IDENTITY_TABLE, &flag).await? else {
        return err("identity not found");
    };
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let unit_id = opt(&body, &["unitId"]).unwrap_or_default().to_string();
    let updated = client
        .execute(
            "UPDATE x_org_identity SET
                name = CASE WHEN $2 = '' THEN name ELSE $2 END,
                unit_id = CASE WHEN $3 = '' THEN unit_id ELSE NULLIF($3, '') END
             WHERE id = $1 AND deleted_at IS NULL",
            &[&iid, &name, &unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("identity not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(iid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn identity_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    identity_edit(pool, session, Path(flag), Json(body)).await
}

pub async fn identity_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, IDENTITY_TABLE, &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))].into_iter().collect(),
        )),
        None => err("identity not found"),
    }
}

pub async fn identity_list_like(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, IDENTITY_TABLE, key, false, IDENTITY_EXTRA, false).await
}

pub async fn identity_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key"]).unwrap_or_default();
    generic_like_search(&pool, IDENTITY_TABLE, key, true, IDENTITY_EXTRA, false).await
}

pub async fn identity_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let initials = initials_from_body(&body);
    generic_pinyininitial_filter(&pool, IDENTITY_TABLE, &initials, IDENTITY_EXTRA, false).await
}

// 鈹€鈹€ group 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

pub async fn group_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    let unit_flag = opt(&body, &["unitId"]).unwrap_or_default().to_string();
    let unit_id = if unit_flag.is_empty() {
        String::new()
    } else {
        resolve_generic_id(&client, UNIT_TABLE, &unit_flag)
            .await?
            .ok_or_else(|| AppError::BadRequest("unit not found".to_string()))?
    };
    if normalized_name_dup(&client, GROUP_TABLE, "unit_id", &unit_id, &name).await? {
        return err("group already exists");
    }
    let group_type = opt(&body, &["type", "groupType"]).unwrap_or_default().to_string();
    let description = opt(&body, &["description"]).unwrap_or_default().to_string();
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_group (id, name, unit_id, type, description, creator) VALUES ($1, $2, NULLIF($3,''), NULLIF($4,''), $5, $6)",
            &[&id, &name, &unit_id, &group_type, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn group_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(gid) = resolve_generic_id(&client, GROUP_TABLE, &flag).await? else {
        return err("group not found");
    };
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let group_type = opt(&body, &["type"]).unwrap_or_default().to_string();
    let description = opt(&body, &["description"]).unwrap_or_default().to_string();
    let updated = client
        .execute(
            "UPDATE x_org_group SET
                name = CASE WHEN $2 = '' THEN name ELSE $2 END,
                type = CASE WHEN $3 = '' THEN type ELSE NULLIF($3, '') END,
                description = CASE WHEN $4 = '' THEN description ELSE $4 END
             WHERE id = $1 AND deleted_at IS NULL",
            &[&gid, &name, &group_type, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("group not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(gid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn group_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    group_edit(pool, session, Path(flag), Json(body)).await
}

pub async fn group_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(gid) = soft_delete_generic(&client, GROUP_TABLE, &flag).await? else {
        return err("group not found");
    };
    client
        .execute("DELETE FROM x_org_group_member WHERE group_id = $1", &[&gid])
        .await
        .map_err(|_| AppError::Internal)?;
    client
        .execute("DELETE FROM x_org_group_role WHERE group_id = $1", &[&gid])
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("id".to_string(), Value::String(gid))].into_iter().collect(),
    ))
}

pub async fn group_add_member(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(gid) = resolve_generic_id(&client, GROUP_TABLE, &flag).await? else {
        return err("group not found");
    };
    let mut members = Vec::new();
    for key in ["personList", "identityList", "unitList"] {
        members.extend(json_str_list(&body, &[key]));
    }
    check_batch_len(members.len())?;
    let mut added: i64 = 0;
    for m in &members {
        let pid = match super::u2_person::resolve_person_id(&client, m).await? {
            Some(p) => p,
            None => m.clone(),
        };
        added += client
            .execute(
                "INSERT INTO x_org_group_member (group_id, person_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                &[&gid, &pid],
            )
            .await
            .map_err(|_| AppError::Internal)? as i64;
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(gid)),
            ("added".to_string(), Value::Number(added.into())),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn group_add_member_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    group_add_member(pool, session, Path(flag), Json(body)).await
}

pub async fn group_delete_member(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(gid) = resolve_generic_id(&client, GROUP_TABLE, &flag).await? else {
        return err("group not found");
    };
    let mut members = Vec::new();
    for key in ["personList", "identityList", "unitList"] {
        members.extend(json_str_list(&body, &[key]));
    }
    check_batch_len(members.len())?;
    let mut removed: i64 = 0;
    for m in &members {
        removed += client
            .execute(
                "DELETE FROM x_org_group_member WHERE group_id = $1 AND person_id = $2",
                &[&gid, m],
            )
            .await
            .map_err(|_| AppError::Internal)? as i64;
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(gid)),
            ("removed".to_string(), Value::Number(removed.into())),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn group_delete_member_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    group_delete_member(pool, session, Path(flag), Json(body)).await
}

pub async fn group_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, GROUP_TABLE, key, false, GROUP_EXTRA, false).await
}

pub async fn group_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key"]).unwrap_or_default();
    generic_like_search(&pool, GROUP_TABLE, key, true, GROUP_EXTRA, false).await
}

pub async fn group_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let initials = initials_from_body(&body);
    generic_pinyininitial_filter(&pool, GROUP_TABLE, &initials, GROUP_EXTRA, false).await
}

// 鈹€鈹€ role 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

pub async fn role_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    if normalized_name_dup(&client, ROLE_TABLE, "creator", "", &name).await? {
        return err("role already exists");
    }
    let description = opt(&body, &["description"]).unwrap_or_default().to_string();
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_role (id, name, description, creator) VALUES ($1, $2, $3, $4)",
            &[&id, &name, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn role_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(rid) = resolve_generic_id(&client, ROLE_TABLE, &flag).await? else {
        return err("role not found");
    };
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let description = opt(&body, &["description"]).unwrap_or_default().to_string();
    let updated = client
        .execute(
            "UPDATE x_org_role SET
                name = CASE WHEN $2 = '' THEN name ELSE $2 END,
                description = CASE WHEN $3 = '' THEN description ELSE $3 END
             WHERE id = $1 AND deleted_at IS NULL",
            &[&rid, &name, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("role not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(rid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn role_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    role_edit(pool, session, Path(flag), Json(body)).await
}

pub async fn role_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(rid) = soft_delete_generic(&client, ROLE_TABLE, &flag).await? else {
        return err("role not found");
    };
    client
        .execute("DELETE FROM auth_person_role WHERE role_id = $1", &[&rid])
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("id".to_string(), Value::String(rid))].into_iter().collect(),
    ))
}

pub async fn role_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, ROLE_TABLE, key, false, ROLE_EXTRA, false).await
}

pub async fn role_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key"]).unwrap_or_default();
    generic_like_search(&pool, ROLE_TABLE, key, true, ROLE_EXTRA, false).await
}

pub async fn role_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let initials = initials_from_body(&body);
    generic_pinyininitial_filter(&pool, ROLE_TABLE, &initials, ROLE_EXTRA, false).await
}

// 鈹€鈹€ unitduty 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

pub async fn duty_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    let unit_flag = opt(&body, &["unitId", "unit"]).unwrap_or_default().to_string();
    let unit_id = if unit_flag.is_empty() {
        String::new()
    } else {
        resolve_generic_id(&client, UNIT_TABLE, &unit_flag)
            .await?
            .ok_or_else(|| AppError::BadRequest("unit not found".to_string()))?
    };
    if normalized_name_dup(&client, DUTY_TABLE, "unit_id", &unit_id, &name).await? {
        return err("unitduty already exists");
    }
    let identities = json_str_list(&body, &["identityList"]);
    check_batch_len(identities.len())?;
    let identity_first = identities.first().cloned().unwrap_or_default();
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_duty (id, name, unit_id, identity_id, creator) VALUES ($1, $2, NULLIF($3,''), NULLIF($4,''), $5)",
            &[&id, &name, &unit_id, &identity_first, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn duty_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(did) = resolve_generic_id(&client, DUTY_TABLE, &flag).await? else {
        return err("unitduty not found");
    };
    let identities = json_str_list(&body, &["identityList"]);
    check_batch_len(identities.len())?;
    let identity_first = identities.first().cloned().unwrap_or_default();
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let updated = client
        .execute(
            "UPDATE x_org_duty SET
                name = CASE WHEN $2 = '' THEN name ELSE $2 END,
                identity_id = CASE WHEN $3 = '' THEN identity_id ELSE NULLIF($3, '') END
             WHERE id = $1 AND deleted_at IS NULL",
            &[&did, &name, &identity_first],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("unitduty not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(did)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn duty_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    duty_edit(pool, session, Path(flag), Json(body)).await
}

pub async fn duty_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, DUTY_TABLE, &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))].into_iter().collect(),
        )),
        None => err("unitduty not found"),
    }
}

pub async fn duty_update_member(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let duty_name = normalize_key(opt(&body, &["unitDuty", "name"]).unwrap_or_default());
    if duty_name.is_empty() {
        return Err(AppError::BadRequest("unitDuty is required".to_string()));
    }
    let unit_flag = opt(&body, &["unit"]).unwrap_or_default().to_string();
    let unit_id = if unit_flag.is_empty() {
        String::new()
    } else {
        resolve_generic_id(&client, UNIT_TABLE, &unit_flag)
            .await?
            .ok_or_else(|| AppError::BadRequest("unit not found".to_string()))?
    };
    let did = client
        .query_opt(
            "SELECT id FROM x_org_duty WHERE name = $1 AND deleted_at IS NULL AND ($2 = '' OR unit_id = $2)",
            &[&duty_name, &unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .map(|r| r.get::<_, String>(0));
    let Some(did) = did else {
        return err("unitduty not found");
    };
    let identities = json_str_list(&body, &["identityList"]);
    check_batch_len(identities.len())?;
    let identity_first = identities.first().cloned().unwrap_or_default();
    let updated = client
        .execute(
            "UPDATE x_org_duty SET identity_id = NULLIF($2, '') WHERE id = $1",
            &[&did, &identity_first],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("unitduty not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(did)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn duty_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, DUTY_TABLE, key, false, DUTY_EXTRA, false).await
}
