// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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

#[allow(non_snake_case)]
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
    let parent_flag = opt(&body, &["parentId", "superior"])
        .unwrap_or_default()
        .to_string();
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
    let sort = body
        .get("sort")
        .and_then(|v| v.as_i64())
        .unwrap_or(0)
        .to_string();
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

#[allow(non_snake_case)]
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
    let pinyin = opt(&body, &["pinyinInitial"])
        .unwrap_or_default()
        .to_string();
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

#[allow(non_snake_case)]
pub async fn unit_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    unit_edit(pool, session, Path(flag), Json(body)).await
}

#[allow(non_snake_case)]
pub async fn unit_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, UNIT_TABLE, &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))]
                .into_iter()
                .collect(),
        )),
        None => err("unit not found"),
    }
}

#[allow(non_snake_case)]
pub async fn unit_mock_delete_to_get(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    unit_delete(pool, session, Path(flag)).await
}

async fn top_units(pool: &Pool, unit_type: Option<&str>, legacy_bare: bool) -> HandlerResult {
    let client = client_of(pool).await?;
    let rows = match unit_type {
        Some(t) => {
            client
                .query(
                    "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id IS NULL AND deleted_at IS NULL AND type = $1 ORDER BY sort ASC, create_time DESC",
                    &[&t.to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?
        }
        None => {
            client
                .query(
                    "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id IS NULL AND deleted_at IS NULL ORDER BY sort ASC, create_time DESC",
                    &[],
                )
                .await
                .map_err(|_| AppError::Internal)?
        }
    };
    if legacy_bare {
        list_ok_legacy(rows.iter().map(unit_row_json).collect())
    } else {
        list_ok(rows.iter().map(unit_row_json).collect())
    }
}

#[allow(non_snake_case)]
pub async fn unit_get_root(pool: Extension<Pool>) -> HandlerResult {
    top_units(&pool, None, false).await
}

#[allow(non_snake_case)]
pub async fn unit_list_top_root(pool: Extension<Pool>) -> HandlerResult {
    top_units(&pool, None, true).await
}

#[allow(non_snake_case)]
pub async fn unit_list_top_with_type(
    pool: Extension<Pool>,
    Path(unit_type): Path<String>,
) -> HandlerResult {
    top_units(&pool, Some(&unit_type), true).await
}

#[allow(non_snake_case)]
pub async fn unit_control_top(pool: Extension<Pool>) -> HandlerResult {
    top_units(&pool, None, true).await
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub async fn unit_list_prev(
    pool: Extension<Pool>,
    Path((flag, count)): Path<(String, i64)>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let limit = count.clamp(1, MAX_BATCH_IDS as i64).to_string();
    let rows = if flag == "0" || flag == "(0)" {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id IS NULL AND deleted_at IS NULL ORDER BY sort ASC, create_time ASC LIMIT $1",
                &[&limit],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id = $1 AND deleted_at IS NULL ORDER BY sort ASC, create_time ASC LIMIT $2",
                &[&flag, &limit],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };
    list_ok(rows.iter().map(unit_row_json).collect())
}

#[allow(non_snake_case)]
pub async fn unit_list_sub_direct(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id = $1 AND deleted_at IS NULL ORDER BY sort ASC, create_time DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(unit_row_json).collect())
}

#[allow(non_snake_case)]
pub async fn unit_list_sub_direct_with_type(
    pool: Extension<Pool>,
    Path((flag, unit_type)): Path<(String, String)>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE parent_id = $1 AND type = $2 AND deleted_at IS NULL ORDER BY sort ASC, create_time DESC",
            &[&flag, &unit_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(unit_row_json).collect())
}

async fn identity_unit_id(
    client: &deadpool_postgres::Client,
    identity_flag: &str,
) -> Result<Option<String>, AppError> {
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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub async fn unit_get_sup_direct(pool: Extension<Pool>, Path(flag): Path<String>) -> HandlerResult {
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
    // Single query instead of 2N serial round-trips: the old loop ran, per flag,
    // a resolve_generic_id (SELECT id WHERE id/name) plus a full-row fetch by id —
    // both against x_org_unit. Fetch every flagged unit at once, then assemble in
    // flag order so duplicates (same flag repeated) are preserved. More robust
    // than the old query_opt resolve, which 500s when a name matches >1 row.
    let rows = client
        .query(
            "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE (id = ANY($1) OR name = ANY($1)) AND deleted_at IS NULL",
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    use std::collections::HashMap;
    type Row = deadpool_postgres::tokio_postgres::Row;
    let mut by_id: HashMap<&str, &Row> = HashMap::new();
    let mut by_name: HashMap<&str, &Row> = HashMap::new();
    for row in &rows {
        if let Some(id) = row.get::<_, Option<&str>>("id") {
            by_id.entry(id).or_insert(row);
        }
        if let Some(name) = row.get::<_, Option<&str>>("name") {
            by_name.entry(name).or_insert(row);
        }
    }
    let mut data = Vec::new();
    for f in flags {
        if let Some(&row) = by_id.get(f.as_str()).or_else(|| by_name.get(f.as_str())) {
            data.push(unit_row_json(row));
        }
    }
    list_ok_legacy(data)
}

#[allow(non_snake_case)]
pub async fn unit_list_by_body(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let flags = json_str_list(&body, &["unitList"]);
    if flags.is_empty() {
        return generic_list_all(&pool, UNIT_TABLE, UNIT_EXTRA).await;
    }
    units_by_flags(&pool, &flags).await
}

#[allow(non_snake_case)]
pub async fn unit_list_controller(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let flags = json_str_list(&body, &["unitList", "controllerList"]);
    if flags.is_empty() {
        return generic_list_all(&pool, UNIT_TABLE, UNIT_EXTRA).await;
    }
    units_by_flags(&pool, &flags).await
}

#[allow(non_snake_case)]
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
        // Single query instead of per-flag resolve_generic_id (N+1): filter the
        // typed units directly by (id or name) in the flag batch. Equivalent to
        // the old resolve-then-id=ANY path, and robust to a name that matches
        // multiple units (query_opt in resolve_generic_id would 500 on that).
        client
            .query(
                "SELECT id, name, parent_id, level, sort, creator, create_time::text FROM x_org_unit WHERE type = $1 AND (id = ANY($2) OR name = ANY($2)) AND deleted_at IS NULL ORDER BY sort ASC",
                &[&unit_type, &flags],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };
    list_ok(rows.iter().map(unit_row_json).collect())
}

#[allow(non_snake_case)]
pub async fn unit_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let initials = initials_from_body(&body);
    generic_pinyininitial_filter(&pool, UNIT_TABLE, &initials, UNIT_EXTRA, true).await
}

#[allow(non_snake_case)]
pub async fn unit_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, UNIT_TABLE, key, false, UNIT_EXTRA, true).await
}

#[allow(non_snake_case)]
pub async fn unit_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key"]).unwrap_or_default();
    generic_like_search(&pool, UNIT_TABLE, key, true, UNIT_EXTRA, true).await
}

// 鈹€鈹€ identity 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[allow(non_snake_case)]
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
    let person_flag = opt(&body, &["personId", "person"])
        .unwrap_or_default()
        .to_string();
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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub async fn identity_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    identity_edit(pool, session, Path(flag), Json(body)).await
}

#[allow(non_snake_case)]
pub async fn identity_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, IDENTITY_TABLE, &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))]
                .into_iter()
                .collect(),
        )),
        None => err("identity not found"),
    }
}

#[allow(non_snake_case)]
pub async fn identity_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, IDENTITY_TABLE, key, false, IDENTITY_EXTRA, false).await
}

#[allow(non_snake_case)]
pub async fn identity_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key"]).unwrap_or_default();
    generic_like_search(&pool, IDENTITY_TABLE, key, true, IDENTITY_EXTRA, false).await
}

#[allow(non_snake_case)]
pub async fn identity_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let initials = initials_from_body(&body);
    generic_pinyininitial_filter(&pool, IDENTITY_TABLE, &initials, IDENTITY_EXTRA, false).await
}

// 鈹€鈹€ group 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[allow(non_snake_case)]
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
    let group_type = opt(&body, &["type", "groupType"])
        .unwrap_or_default()
        .to_string();
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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub async fn group_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    group_edit(pool, session, Path(flag), Json(body)).await
}

#[allow(non_snake_case)]
pub async fn group_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    // 软删群组 + 级联清成员/角色三写必须原子：中途失败留孤儿成员/角色。
    let mut client = client_of(&pool).await?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let find_sql =
        format!("SELECT id FROM {GROUP_TABLE} WHERE (id = $1 OR name = $1) AND deleted_at IS NULL");
    let Some(gid) = tx
        .query_opt(&find_sql, &[&flag])
        .await
        .map_err(|_| AppError::Internal)?
        .map(|r| r.get::<_, String>(0))
    else {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return err("group not found");
    };
    let del_sql =
        format!("UPDATE {GROUP_TABLE} SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL");
    tx.execute(&del_sql, &[&gid])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute(
        "DELETE FROM x_org_group_member WHERE group_id = $1",
        &[&gid],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    tx.execute("DELETE FROM x_org_group_role WHERE group_id = $1", &[&gid])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("id".to_string(), Value::String(gid))]
            .into_iter()
            .collect(),
    ))
}

#[allow(non_snake_case)]
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
    // 三个字面量各自调用（非变量键循环）：键名显式可见，静态契约审计可解析。
    let mut members = json_str_list(&body, &["personList"]);
    members.extend(json_str_list(&body, &["identityList"]));
    members.extend(json_str_list(&body, &["unitList"]));
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

#[allow(non_snake_case)]
pub async fn group_add_member_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    group_add_member(pool, session, Path(flag), Json(body)).await
}

#[allow(non_snake_case)]
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
    // 三个字面量各自调用（非变量键循环）：键名显式可见，静态契约审计可解析。
    let mut members = json_str_list(&body, &["personList"]);
    members.extend(json_str_list(&body, &["identityList"]));
    members.extend(json_str_list(&body, &["unitList"]));
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

#[allow(non_snake_case)]
pub async fn group_delete_member_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    group_delete_member(pool, session, Path(flag), Json(body)).await
}

#[allow(non_snake_case)]
pub async fn group_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, GROUP_TABLE, key, false, GROUP_EXTRA, false).await
}

#[allow(non_snake_case)]
pub async fn group_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key"]).unwrap_or_default();
    generic_like_search(&pool, GROUP_TABLE, key, true, GROUP_EXTRA, false).await
}

#[allow(non_snake_case)]
pub async fn group_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let initials = initials_from_body(&body);
    generic_pinyininitial_filter(&pool, GROUP_TABLE, &initials, GROUP_EXTRA, false).await
}

// 鈹€鈹€ role 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub async fn role_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    role_edit(pool, session, Path(flag), Json(body)).await
}

#[allow(non_snake_case)]
pub async fn role_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    // 软删角色 + 清授权两写必须原子：中途失败留「已删角色仍在授权表生效」的权限残留。
    let mut client = client_of(&pool).await?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;
    let find_sql =
        format!("SELECT id FROM {ROLE_TABLE} WHERE (id = $1 OR name = $1) AND deleted_at IS NULL");
    let Some(rid) = tx
        .query_opt(&find_sql, &[&flag])
        .await
        .map_err(|_| AppError::Internal)?
        .map(|r| r.get::<_, String>(0))
    else {
        tx.commit().await.map_err(|_| AppError::Internal)?;
        return err("role not found");
    };
    let del_sql =
        format!("UPDATE {ROLE_TABLE} SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL");
    tx.execute(&del_sql, &[&rid])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.execute("DELETE FROM auth_person_role WHERE role_id = $1", &[&rid])
        .await
        .map_err(|_| AppError::Internal)?;
    tx.commit().await.map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("id".to_string(), Value::String(rid))]
            .into_iter()
            .collect(),
    ))
}

#[allow(non_snake_case)]
pub async fn role_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, ROLE_TABLE, key, false, ROLE_EXTRA, false).await
}

#[allow(non_snake_case)]
pub async fn role_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let key = opt(&body, &["key"]).unwrap_or_default();
    generic_like_search(&pool, ROLE_TABLE, key, true, ROLE_EXTRA, false).await
}

#[allow(non_snake_case)]
pub async fn role_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let initials = initials_from_body(&body);
    generic_pinyininitial_filter(&pool, ROLE_TABLE, &initials, ROLE_EXTRA, false).await
}

// 鈹€鈹€ unitduty 鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€鈹€

#[allow(non_snake_case)]
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
    let unit_flag = opt(&body, &["unitId", "unit"])
        .unwrap_or_default()
        .to_string();
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

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub async fn duty_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    duty_edit(pool, session, Path(flag), Json(body)).await
}

#[allow(non_snake_case)]
pub async fn duty_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, DUTY_TABLE, &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))]
                .into_iter()
                .collect(),
        )),
        None => err("unitduty not found"),
    }
}

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
pub async fn duty_list_like(pool: Extension<Pool>, Json(body): Json<Value>) -> HandlerResult {
    let key = opt(&body, &["key", "name"]).unwrap_or_default();
    generic_like_search(&pool, DUTY_TABLE, key, false, DUTY_EXTRA, false).await
}

// ═══ 表单设计器数据源示例默认值所指的两条通用读端点（KNOWN_BACKEND_GAPS 最后两条，实装清零）═══

/// GET /api/users/list — 人员清单（未删行）。
#[allow(non_snake_case)]
pub async fn users_list(pool: Extension<Pool>) -> HandlerResult {
    let client = client_of(&pool).await?;
    let rows = client
        .query(
            "SELECT id, name, mobile, email, unit_id FROM x_org_person WHERE deleted_at IS NULL ORDER BY name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get::<_, String>("id"))),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, String>("name")),
                ),
                (
                    "mobile".to_string(),
                    Value::String(row.get::<_, Option<String>>("mobile").unwrap_or_default()),
                ),
                (
                    "email".to_string(),
                    Value::String(row.get::<_, Option<String>>("email").unwrap_or_default()),
                ),
                (
                    "unitId".to_string(),
                    Value::String(row.get::<_, Option<String>>("unit_id").unwrap_or_default()),
                ),
            ]))
        })
        .collect();
    list_ok(data)
}

/// GET /api/departments/tree — 部门树（未删行，按 parent_id 组装 children，
/// 自底向上挂接避免浅拷贝丢深层节点；父不在集合内/缺失视作根）。
#[allow(non_snake_case)]
pub async fn departments_tree(pool: Extension<Pool>) -> HandlerResult {
    let client = client_of(&pool).await?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, level, sort FROM x_org_unit WHERE deleted_at IS NULL ORDER BY level, sort, name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut nodes: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get::<_, String>("id"))),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, String>("name")),
                ),
                (
                    "parentId".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("parent_id")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "level".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("level"))),
                ),
                ("children".to_string(), Value::Array(Vec::new())),
            ]))
        })
        .collect();
    use std::collections::HashMap;
    let index: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n["id"].as_str().unwrap_or_default().to_string(), i))
        .collect();
    // 自底向上：level 深的先挂进父，孙子先并入子支，再整支上挂
    let mut order: Vec<usize> = (0..nodes.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(nodes[i]["level"].as_i64().unwrap_or(0)));
    for &c in &order {
        let parent = nodes[c]["parentId"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        let self_id = nodes[c]["id"].as_str().unwrap_or_default().to_string();
        if parent.is_empty() || parent == self_id {
            continue;
        }
        if let Some(&p) = index.get(parent.as_str()) {
            if p != c {
                let child = nodes[c].clone();
                if let Some(arr) = nodes[p].get_mut("children").and_then(Value::as_array_mut) {
                    arr.push(child);
                }
            }
        }
    }
    let tree: Vec<Value> = nodes
        .iter()
        .enumerate()
        .filter(|(i, n)| {
            let parent = n["parentId"].as_str().unwrap_or_default();
            parent.is_empty() || index.get(parent).copied() != Some(*i)
        })
        .map(|(_, n)| n.clone())
        .collect();
    let count = tree.len() as i64;
    Ok(Json(shared::response::ActionResult::legacy_success(
        Value::Array(tree),
        count,
        0,
    )))
}
