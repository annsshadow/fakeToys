use super::u2_helpers::*;
use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::error::AppError;

type PgRow = deadpool_postgres::tokio_postgres::Row;

const PERSON_TABLE: &str = "x_org_person";
const PERSON_COLS: &str =
    "id, name, mobile, email, unit_id, icon, status, status_des, creator, create_time::text";

pub async fn resolve_person_id(client: &deadpool_postgres::Client, flag: &str) -> Result<Option<String>, AppError> {
    resolve_generic_id(client, PERSON_TABLE, flag).await
}

fn person_row_json(row: &PgRow) -> Value {
    let mut map = serde_json::Map::new();
    for (key, col) in [
        ("id", "id"),
        ("name", "name"),
        ("mobile", "mobile"),
        ("email", "email"),
        ("unitId", "unit_id"),
        ("icon", "icon"),
        ("status", "status"),
        ("statusDes", "status_des"),
        ("creator", "creator"),
        ("createTime", "create_time"),
    ] {
        let v: Option<String> = row.try_get(col).ok().flatten();
        if let Some(v) = v {
            map.insert(key.to_string(), Value::String(v));
        }
    }
    Value::Object(map)
}

pub async fn person_get(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    let sql = format!("SELECT {PERSON_COLS} FROM {PERSON_TABLE} WHERE id = $1");
    match client.query_opt(&sql, &[&pid]).await.map_err(|_| AppError::Internal)? {
        Some(row) => ok(person_row_json(&row)),
        None => err("person not found"),
    }
}

pub async fn person_create(
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
    let dup = client
        .query_opt(
            "SELECT id FROM x_org_person WHERE LOWER(TRIM(name)) = LOWER($1) AND deleted_at IS NULL",
            &[&name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if dup.is_some() {
        return err("person already exists");
    }
    let mobile = opt(&body, &["mobile"]).unwrap_or_default().to_string();
    let email = opt(&body, &["email"]).unwrap_or_default().to_string();
    let unit_id = opt(&body, &["unitId"]).unwrap_or_default().to_string();
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_person (id, name, mobile, email, unit_id, creator) VALUES ($1, $2, NULLIF($3,''), NULLIF($4,''), NULLIF($5,''), $6)",
            &[&id, &name, &mobile, &email, &unit_id, &creator],
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

pub async fn person_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let mobile = opt(&body, &["mobile"]).unwrap_or_default().to_string();
    let email = opt(&body, &["email"]).unwrap_or_default().to_string();
    let unit_id = opt(&body, &["unitId"]).unwrap_or_default().to_string();
    let pinyin = opt(&body, &["pinyinInitial"]).unwrap_or_default().to_string();
    let updated = client
        .execute(
            "UPDATE x_org_person SET
                name = CASE WHEN $2 = '' THEN name ELSE $2 END,
                mobile = CASE WHEN $3 = '' THEN mobile ELSE NULLIF($3, '') END,
                email = CASE WHEN $4 = '' THEN email ELSE NULLIF($4, '') END,
                unit_id = CASE WHEN $5 = '' THEN unit_id ELSE NULLIF($5, '') END,
                pinyin_initial = CASE WHEN $6 = '' THEN pinyin_initial ELSE $6 END
             WHERE id = $1 AND deleted_at IS NULL",
            &[&pid, &name, &mobile, &email, &unit_id, &pinyin],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("person not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(pid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn person_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    person_edit(pool, session, Path(flag), Json(body)).await
}

async fn soft_delete_person(client: &deadpool_postgres::Client, flag: &str) -> Result<Option<String>, AppError> {
    let Some(pid) = resolve_person_id(client, flag).await? else {
        return Ok(None);
    };
    client
        .execute(
            "UPDATE x_org_person SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Some(pid))
}

pub async fn person_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_person(&client, &flag).await? {
        Some(pid) => ok(Value::Object(
            vec![("id".to_string(), Value::String(pid))].into_iter().collect(),
        )),
        None => err("person not found"),
    }
}

pub async fn person_mock_delete_to_get(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    person_delete(pool, session, Path(flag)).await
}

pub async fn person_reserve_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    for sql in [
        "DELETE FROM x_org_identity WHERE person_id = $1",
        "DELETE FROM x_org_group_member WHERE person_id = $1",
        "DELETE FROM x_org_person_attribute WHERE person_id = $1",
    ] {
        client.execute(sql, &[&pid]).await.map_err(|_| AppError::Internal)?;
    }
    client
        .execute(
            "UPDATE x_org_person SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("id".to_string(), Value::String(pid))].into_iter().collect(),
    ))
}

pub async fn person_reserve_mock_delete_to_get(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    person_reserve_delete(pool, session, Path(flag)).await
}

async fn cursor_page(pool: &Pool, flag: &str, count: i64, next: bool) -> HandlerResult {
    let client = client_of(pool).await?;
    let limit = count.clamp(1, MAX_BATCH_IDS as i64).to_string();
    let rows = if flag == "0" || flag == "(0)" {
        let sql = format!(
            "SELECT {PERSON_COLS} FROM {PERSON_TABLE} WHERE deleted_at IS NULL ORDER BY create_time::text DESC LIMIT $1::bigint"
        );
        client.query(&sql, &[&limit]).await.map_err(|_| AppError::Internal)?
    } else {
        let op = if next { ">" } else { "<" };
        let sql = format!(
            "SELECT {PERSON_COLS} FROM {PERSON_TABLE} WHERE deleted_at IS NULL AND id {op} $1 ORDER BY create_time::text DESC LIMIT $2::bigint"
        );
        client
            .query(&sql, &[&flag.to_string(), &limit])
            .await
            .map_err(|_| AppError::Internal)?
    };
    list_ok(rows.iter().map(person_row_json).collect())
}

pub async fn person_list_next(
    pool: Extension<Pool>,
    Path((flag, count)): Path<(String, i64)>,
) -> HandlerResult {
    cursor_page(&pool, &flag, count, true).await
}

pub async fn person_list_prev(
    pool: Extension<Pool>,
    Path((flag, count)): Path<(String, i64)>,
) -> HandlerResult {
    cursor_page(&pool, &flag, count, false).await
}

async fn persons_in_group(pool: &Pool, group_flag: &str, nested: bool) -> HandlerResult {
    let client = client_of(pool).await?;
    let edge = if nested {
        "UNION SELECT m.person_id FROM x_org_group_member m JOIN sub_groups sg ON m.group_id = sg.id WHERE m.type = 'group'"
    } else {
        ""
    };
    let sql = format!(
        "WITH RECURSIVE sub_groups(id) AS (
            SELECT id FROM x_org_group WHERE (id = $1 OR name = $1) AND deleted_at IS NULL
            {edge}
        )
        SELECT DISTINCT {PERSON_COLS} FROM x_org_person p
          JOIN x_org_group_member gm ON gm.person_id = p.id
          JOIN sub_groups sg ON gm.group_id = sg.id
         WHERE p.deleted_at IS NULL"
    );
    let rows = client
        .query(&sql, &[&group_flag.to_string()])
        .await
        .map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(person_row_json).collect())
}

pub async fn person_list_group_sub_direct(
    pool: Extension<Pool>,
    Path(group_flag): Path<String>,
) -> HandlerResult {
    persons_in_group(&pool, &group_flag, false).await
}

pub async fn person_list_group_sub_nested(
    pool: Extension<Pool>,
    Path(group_flag): Path<String>,
) -> HandlerResult {
    persons_in_group(&pool, &group_flag, true).await
}

pub async fn person_list_with_role(
    pool: Extension<Pool>,
    Path(role_flag): Path<String>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let sql = format!(
        "SELECT DISTINCT {PERSON_COLS} FROM x_org_person p
          JOIN auth_person_role pr ON pr.person_id = p.id
          JOIN x_org_role r ON r.id = pr.role_id
         WHERE (r.id = $1 OR r.name = $1) AND p.deleted_at IS NULL"
    );
    let rows = client.query(&sql, &[&role_flag]).await.map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(person_row_json).collect())
}

pub async fn person_list_pinyininitial(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    check_batch_len(initials_from_body(&body).len())?;
    let initials = initials_from_body(&body);
    let client = client_of(&pool).await?;
    if initials.is_empty() {
        let sql = format!("SELECT {PERSON_COLS} FROM {PERSON_TABLE} WHERE deleted_at IS NULL");
        let rows = client.query(&sql, &[]).await.map_err(|_| AppError::Internal)?;
        return list_ok(rows.iter().map(person_row_json).collect());
    }
    let sql = format!(
        "SELECT {PERSON_COLS} FROM {PERSON_TABLE}
          WHERE deleted_at IS NULL AND (LEFT(LOWER(pinyin_initial), 1) = ANY($1) OR LEFT(LOWER(name), 1) = ANY($1))"
    );
    let rows = client.query(&sql, &[&initials]).await.map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(person_row_json).collect())
}

pub async fn person_list_like(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let key = normalize_key(opt(&body, &["key", "name"]).unwrap_or_default());
    if key.is_empty() {
        let sql = format!("SELECT {PERSON_COLS} FROM {PERSON_TABLE} WHERE deleted_at IS NULL");
        let rows = client.query(&sql, &[]).await.map_err(|_| AppError::Internal)?;
        return list_ok(rows.iter().map(person_row_json).collect());
    }
    let pattern = format!("%{key}%");
    let sql = format!(
        "SELECT {PERSON_COLS} FROM {PERSON_TABLE} WHERE deleted_at IS NULL AND name ILIKE $1"
    );
    let rows = client.query(&sql, &[&pattern]).await.map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(person_row_json).collect())
}

pub async fn person_list_like_pinyin(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let key = normalize_key(opt(&body, &["key"]).unwrap_or_default());
    if key.is_empty() {
        let sql = format!("SELECT {PERSON_COLS} FROM {PERSON_TABLE} WHERE deleted_at IS NULL");
        let rows = client.query(&sql, &[]).await.map_err(|_| AppError::Internal)?;
        return list_ok(rows.iter().map(person_row_json).collect());
    }
    let pattern = format!("{}%", key.to_lowercase());
    let sql = format!(
        "SELECT {PERSON_COLS} FROM {PERSON_TABLE}
          WHERE deleted_at IS NULL AND (LOWER(pinyin_initial) LIKE $1 OR LOWER(name) LIKE $1)"
    );
    let rows = client.query(&sql, &[&pattern]).await.map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(person_row_json).collect())
}

pub async fn person_set_password(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let value = opt(&body, &["value", "password"]).unwrap_or_default().to_string();
    if value.is_empty() {
        return Err(AppError::BadRequest("password must not be empty".to_string()));
    }
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    client
        .execute("UPDATE x_org_person SET password = $2 WHERE id = $1", &[&pid, &value])
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Bool(true))
}

pub async fn person_set_password_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    person_set_password(pool, session, Path(flag), Json(body)).await
}

pub async fn person_reset_password(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    client
        .execute("UPDATE x_org_person SET password = '' WHERE id = $1", &[&pid])
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("value".to_string(), Value::Bool(true))].into_iter().collect(),
    ))
}

pub async fn person_check_password(Path(password): Path<String>) -> HandlerResult {
    ok(Value::Object(
        vec![("value".to_string(), Value::Bool(validate_password_policy(&password)))]
            .into_iter()
            .collect(),
    ))
}

pub async fn person_get_icon(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    match client
        .query_opt("SELECT icon FROM x_org_person WHERE id = $1", &[&pid])
        .await
        .map_err(|_| AppError::Internal)?
    {
        Some(row) => {
            let icon: Option<String> = row.try_get(0).ok().flatten();
            ok(icon.map(Value::String).unwrap_or(Value::Null))
        }
        None => err("person not found"),
    }
}

pub async fn person_set_icon(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let icon = opt(&body, &["icon"]).unwrap_or_default().to_string();
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    let updated = client
        .execute(
            "UPDATE x_org_person SET icon = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&pid, &icon],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("person not updated");
    }
    ok(Value::Bool(true))
}

pub async fn person_set_icon_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    person_set_icon(pool, session, Path(flag), Json(body)).await
}

pub async fn person_lock(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let desc = opt(&body, &["desc", "description"]).unwrap_or_default().to_string();
    let lock_expired_time = opt(&body, &["lockExpiredTime"]).unwrap_or_default().to_string();
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    client
        .execute(
            "UPDATE x_org_person SET status = 'lock', status_des = $2, lock_expired_time = NULLIF($3, '')::timestamp WHERE id = $1",
            &[&pid, &desc, &lock_expired_time],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Bool(true))
}

pub async fn person_unlock(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    client
        .execute(
            "UPDATE x_org_person SET status = 'active', status_des = '', lock_expired_time = NULL WHERE id = $1",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Bool(true))
}

pub async fn person_ban(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    client
        .execute(
            "UPDATE x_org_person SET status = 'ban', status_des = 'banned' WHERE id = $1",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Bool(true))
}

pub async fn person_unban(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    client
        .execute(
            "UPDATE x_org_person SET status = 'active', status_des = '' WHERE id = $1",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Bool(true))
}

pub async fn person_set_password_expired_time(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path((flag, date)): Path<(String, String)>,
) -> HandlerResult {
    if !is_parseable_date(&date) {
        return Err(AppError::BadRequest(format!("invalid date: {date}")));
    }
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_person_id(&client, &flag).await? else {
        return err("person not found");
    };
    let date_value = date.trim().trim_end_matches('Z').to_string();
    client
        .execute(
            "UPDATE x_org_person SET password_expired_time = $2::timestamp WHERE id = $1",
            &[&pid, &date_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Bool(true))
}

pub async fn person_list_filter_paging(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    Json(body): Json<Value>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let page = page.max(1);
    let size = size.clamp(1, MAX_PAGE_SIZE);
    let offset = ((page - 1) * size).to_string();
    let size_str = size.to_string();

    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let mobile = normalize_key(opt(&body, &["mobile"]).unwrap_or_default());
    let email = normalize_key(opt(&body, &["email"]).unwrap_or_default());
    let status = normalize_key(opt(&body, &["status"]).unwrap_or_default());
    let unit_flag = normalize_key(opt(&body, &["unitFlag", "unitId"]).unwrap_or_default());

    let cond = "deleted_at IS NULL
                AND ($1 = '' OR name ILIKE '%'||$1||'%')
                AND ($2 = '' OR mobile ILIKE '%'||$2||'%')
                AND ($3 = '' OR email ILIKE '%'||$3||'%')
                AND ($4 = '' OR status = $4)
                AND ($5 = '' OR unit_id = $5)";
    let total_sql = format!("SELECT COUNT(*)::bigint AS cnt FROM x_org_person WHERE {cond}");
    let total_row = client
        .query_one(&total_sql, &[&name, &mobile, &email, &status, &unit_flag])
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_row.get("cnt");

    let data_sql = format!(
        "SELECT {PERSON_COLS} FROM x_org_person WHERE {cond} ORDER BY create_time::text DESC LIMIT $6::bigint OFFSET $7::bigint"
    );
    let rows = client
        .query(&data_sql, &[&name, &mobile, &email, &status, &unit_flag, &size_str, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(person_row_json).collect();
    ok(Value::Object(
        vec![
            ("count".to_string(), Value::Number(total.into())),
            ("page".to_string(), Value::Number(page.into())),
            ("size".to_string(), Value::Number(size.into())),
            ("data".to_string(), Value::Array(data)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn person_list_delete_paging(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
) -> HandlerResult {
    let client = client_of(&pool).await?;
    let page = page.max(1);
    let size = size.clamp(1, MAX_PAGE_SIZE);
    let offset = ((page - 1) * size).to_string();
    let size_str = size.to_string();
    let sql = format!(
        "SELECT {PERSON_COLS} FROM {PERSON_TABLE} WHERE deleted_at IS NOT NULL ORDER BY create_time::text DESC LIMIT $1::bigint OFFSET $2::bigint"
    );
    let rows = client
        .query(&sql, &[&size_str, &offset])
        .await
        .map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(person_row_json).collect())
}
