use deadpool_postgres::{Client, Pool};
use serde_json::Value;
use shared::{
    error::AppError,
    response::{ActionResult, row_to_json},
};

pub const MAX_BATCH_IDS: usize = 100;
pub const MAX_PAGE_SIZE: i64 = 100;

pub fn normalize_key(raw: &str) -> String {
    raw.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn check_batch_len(len: usize) -> Result<(), AppError> {
    if len > MAX_BATCH_IDS {
        return Err(AppError::BadRequest(format!(
            "batch size {len} exceeds limit {MAX_BATCH_IDS}"
        )));
    }
    Ok(())
}

pub fn validate_password_policy(password: &str) -> bool {
    let len = password.chars().count();
    !(len < 6 || len > 64)
        && !password.chars().any(char::is_whitespace)
        && password.chars().any(char::is_alphabetic)
        && password.chars().any(|c| c.is_ascii_digit())
}

pub fn is_parseable_date(date: &str) -> bool {
    let trimmed = date.trim();
    let bare = trimmed.strip_suffix('Z').unwrap_or(trimmed);
    if bare.is_empty() {
        return false;
    }
    for fmt in ["%Y-%m-%dT%H:%M:%S%.f", "%Y-%m-%d %H:%M:%S"] {
        if chrono::NaiveDateTime::parse_from_str(bare, fmt).is_ok() {
            return true;
        }
    }
    chrono::NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").is_ok()
}

pub async fn require_admin(
    pool: &Pool,
    session: &shared::session::Session,
) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

pub async fn client_of(pool: &Pool) -> Result<Client, AppError> {
    pool.get().await.map_err(|_| AppError::Internal)
}

pub fn opt<'a>(body: &'a Value, keys: &[&str]) -> Option<&'a str> {
    for key in keys {
        if let Some(v) = body.get(*key).and_then(|v| v.as_str()) {
            return Some(v);
        }
    }
    None
}

pub fn json_str_list(body: &Value, keys: &[&str]) -> Vec<String> {
    for key in keys {
        if let Some(arr) = body.get(*key).and_then(|v| v.as_array()) {
            return arr
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect();
        }
    }
    Vec::new()
}

pub fn initials_from_body(body: &Value) -> Vec<String> {
    json_str_list(body, &["pinyinInitial", "pinyinInitialList"])
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

pub type HandlerResult = Result<Json<ActionResult<Value>>, AppError>;

use axum::Json;

pub fn ok(data: Value) -> HandlerResult {
    Ok(Json(ActionResult::success(data)))
}

pub fn err(msg: &str) -> HandlerResult {
    Ok(Json(ActionResult::error(msg)))
}

pub fn list_ok(rows: Vec<Value>) -> HandlerResult {
    ok(Value::Object(
        vec![
            ("count".to_string(), Value::Number((rows.len() as i64).into())),
            ("data".to_string(), Value::Array(rows)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn resolve_generic_id(
    client: &Client,
    table: &str,
    flag: &str,
) -> Result<Option<String>, AppError> {
    let sql =
        format!("SELECT id FROM {table} WHERE (id = $1 OR name = $1) AND deleted_at IS NULL");
    let row = client
        .query_opt(&sql, &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.map(|r| r.get::<_, String>(0)))
}

pub async fn soft_delete_generic(
    client: &Client,
    table: &str,
    flag: &str,
) -> Result<Option<String>, AppError> {
    let Some(id) = resolve_generic_id(client, table, flag).await? else {
        return Ok(None);
    };
    let sql =
        format!("UPDATE {table} SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL");
    client.execute(&sql, &[&id]).await.map_err(|_| AppError::Internal)?;
    Ok(Some(id))
}

pub async fn normalized_name_dup(
    client: &Client,
    table: &str,
    scope_col: &str,
    scope_val: &str,
    name: &str,
) -> Result<bool, AppError> {
    let sql = format!(
        "SELECT id FROM {table} WHERE LOWER(TRIM(name)) = LOWER($1) AND {scope_col} = NULLIF($2, '') AND deleted_at IS NULL"
    );
    let row = client
        .query_opt(&sql, &[&name, &scope_val])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.is_some())
}

pub fn camel_case(col: &str) -> String {
    let mut out = String::with_capacity(col.len());
    let mut upper_next = false;
    for ch in col.chars() {
        if ch == '_' {
            upper_next = true;
        } else if upper_next {
            out.extend(ch.to_uppercase());
            upper_next = false;
        } else {
            out.push(ch);
        }
    }
    out
}

type PgRow = deadpool_postgres::tokio_postgres::Row;

pub fn entity_row_json(row: &PgRow, snake_cols: &[&str]) -> Value {
    let mut base = row_to_json(row);
    if let Value::Object(map) = &mut base {
        for col in snake_cols {
            if let Some(v) = map.remove(*col) {
                map.insert(camel_case(col), v);
            }
        }
    }
    base
}

fn select_cols(extra_cols: &[&str]) -> String {
    ["id, name, creator, create_time::text"]
        .iter()
        .chain(extra_cols.iter())
        .copied()
        .collect::<Vec<_>>()
        .join(", ")
}

pub async fn generic_list_all(
    pool: &Pool,
    table: &str,
    extra_cols: &[&str],
) -> HandlerResult {
    let client = client_of(pool).await?;
    let cols = select_cols(extra_cols);
    let sql = format!(
        "SELECT {cols} FROM {table} WHERE deleted_at IS NULL ORDER BY create_time::text DESC"
    );
    let rows = client.query(&sql, &[]).await.map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(|r| entity_row_json(r, extra_cols)).collect())
}

pub async fn generic_like_search(
    pool: &Pool,
    table: &str,
    key: &str,
    pinyin_mode: bool,
    extra_cols: &[&str],
) -> HandlerResult {
    let client = client_of(pool).await?;
    let cols = select_cols(extra_cols);
    let key_norm = normalize_key(key);
    if key_norm.is_empty() {
        let sql = format!(
            "SELECT {cols} FROM {table} WHERE deleted_at IS NULL ORDER BY create_time::text DESC"
        );
        let rows = client.query(&sql, &[]).await.map_err(|_| AppError::Internal)?;
        return list_ok(rows.iter().map(|r| entity_row_json(r, extra_cols)).collect());
    }
    let (pattern, cond) = if pinyin_mode {
        (
            format!("{}%", key_norm.to_lowercase()),
            "(LOWER(name) LIKE $1 OR COALESCE(LOWER(pinyin_initial), '') LIKE $1)".to_string(),
        )
    } else {
        (format!("%{}%", key_norm), "name ILIKE $1".to_string())
    };
    let sql = format!(
        "SELECT {cols} FROM {table} WHERE deleted_at IS NULL AND {cond} ORDER BY create_time::text DESC"
    );
    let rows = client.query(&sql, &[&pattern]).await.map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(|r| entity_row_json(r, extra_cols)).collect())
}

pub async fn generic_pinyininitial_filter(
    pool: &Pool,
    table: &str,
    initials: &[String],
    extra_cols: &[&str],
) -> HandlerResult {
    check_batch_len(initials.len())?;
    let client = client_of(pool).await?;
    let cols = select_cols(extra_cols);
    if initials.is_empty() {
        let sql = format!(
            "SELECT {cols} FROM {table} WHERE deleted_at IS NULL ORDER BY create_time::text DESC"
        );
        let rows = client.query(&sql, &[]).await.map_err(|_| AppError::Internal)?;
        return list_ok(rows.iter().map(|r| entity_row_json(r, extra_cols)).collect());
    }
    let owned: Vec<String> = initials.to_vec();
    let sql = format!(
        "SELECT {cols} FROM {table} WHERE deleted_at IS NULL AND LEFT(LOWER(COALESCE(pinyin_initial, name)), 1) = ANY($1)"
    );
    let rows = client.query(&sql, &[&owned]).await.map_err(|_| AppError::Internal)?;
    list_ok(rows.iter().map(|r| entity_row_json(r, extra_cols)).collect())
}

pub const UNIT_TABLE: &str = "x_org_unit";
pub const UNIT_EXTRA: &[&str] = &["parent_id", "level", "sort"];
pub const IDENTITY_TABLE: &str = "x_org_identity";
pub const IDENTITY_EXTRA: &[&str] = &["unit_id", "person_id"];
pub const GROUP_TABLE: &str = "x_org_group";
pub const GROUP_EXTRA: &[&str] = &["unit_id", "type", "description"];
pub const ROLE_TABLE: &str = "x_org_role";
pub const ROLE_EXTRA: &[&str] = &["description"];
pub const DUTY_TABLE: &str = "x_org_duty";
pub const DUTY_EXTRA: &[&str] = &["unit_id", "identity_id"];
pub const PERM_TABLE: &str = "x_org_permission_setting";
pub const CARD_TABLE: &str = "x_org_personcard";
