//! plan002 U2: organization_assemble_express endpoint completion.
//!
//! Aligns with Java `x_organization_assemble_express` (132 endpoints) for the
//! high-value person/unit/identity/group/role/unitduty batch query endpoints.
//! Conventions follow crates/express/src/batch_query.rs precedent:
//! - no auth gate (intranet direct-call contract, do not add auth)
//! - batch ID list capped at 100 per request (BadRequest beyond)
//! - PII fields (mobile/email) excluded by default; explicit includePii=true
//! - all handlers run real parameterized SQL against existing x_org_* tables

use axum::{
    extract::{Extension, Json, Path},
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub const ID_COUNT_LIMIT: usize = 100;

/// Extract a string-array field from the request body
/// (loose equivalent of the Java Wi xxxList Gson contract).
pub fn string_list(body: &Value, key: &str) -> Vec<String> {
    body.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Batch cap check: max 100 IDs per request (existing convention).
pub fn capped(ids: &[String]) -> Result<(), AppError> {
    if ids.len() > ID_COUNT_LIMIT {
        return Err(AppError::BadRequest(format!(
            "batch id list exceeds limit of {} items",
            ID_COUNT_LIMIT
        )));
    }
    Ok(())
}

/// Explicit PII parameter: exclude mobile/email unless includePii=true.
pub fn include_pii(body: &Value) -> bool {
    body.get("includePii")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

pub(crate) fn ok_json(data: Value) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    Ok(AxumJson(ActionResult::success(data)))
}

pub(crate) fn count_data(count: usize, data: Vec<Value>) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number((count as i64).into())),
        ("data".to_string(), Value::Array(data)),
    ]))
}

/// Java 裸数组契约（行为对齐）：data 为数组、count 入信封、size 恒 0。
pub(crate) fn ok_java_list(
    count: usize,
    data: Vec<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    Ok(AxumJson(ActionResult::java_success(
        Value::Array(data),
        count as i64,
        0,
    )))
}

pub(crate) fn named_list(key: &str, items: &[String]) -> Value {
    Value::Object(serde_json::Map::from_iter([(
        key.to_string(),
        Value::Array(items.iter().map(|s| Value::String(s.clone())).collect()),
    )]))
}

/// Java WrapBoolean Wo 序列化形态：{"value": true|false}。
pub(crate) fn wrap_bool(v: bool) -> Value {
    Value::Object(serde_json::Map::from_iter([("value".to_string(), Value::Bool(v))]))
}

/// 单值字符串字段（缺失/非字符串返回 None）。
pub(crate) fn string_field(body: &Value, key: &str) -> Option<String> {
    body.get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(str::to_string)
}

/// 布尔字段，默认 default（Java BooleanUtils.isNotFalse / isNotTrue 语义的 loose 版）。
pub(crate) fn bool_field(body: &Value, key: &str, default: bool) -> bool {
    body.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

/// 整型数组字段（levelList 等），同样受批量上限约束。
pub(crate) fn int_list(body: &Value, key: &str) -> Result<Vec<i32>, AppError> {
    let raw: Vec<i32> = body
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_i64().map(|n| n as i32)).collect())
        .unwrap_or_default();
    capped(&raw.iter().map(|i| i.to_string()).collect::<Vec<_>>())?;
    Ok(raw)
}

/// 归一化查重：trim、去空、保序去重（o2 ListTools.trim 的等价实现）。
/// 所有批量入口在 capped() 之后调用，保证重复 flag 不会产生重复行。
pub(crate) fn normalize_flags(flags: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    flags
        .into_iter()
        .map(|f| f.trim().to_string())
        .filter(|f| !f.is_empty())
        .filter(|f| seen.insert(f.clone()))
        .collect()
}

pub(crate) async fn named_list_response(
    pool: &Pool,
    key: &'static str,
    sql: &str,
    flags: &[String],
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(sql, &[&flags])
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
    Ok(AxumJson(ActionResult::java_success(
        named_list(key, &list),
        list.len() as i64,
        0,
    )))
}

/// Flag resolution: match id OR name (simplified o2 pick over available columns).
pub(crate) const PICK_ANY: &str = "(id = ANY($1) OR name = ANY($1))";

type Cols = &'static [&'static str];

fn cols_sql(cols: Cols) -> String {
    cols.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
}

pub(crate) fn row_to_map(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    use deadpool_postgres::tokio_postgres::types::Type;
    let mut map = serde_json::Map::new();
    for col in row.columns() {
        let name = col.name();
        if name == "level" {
            if let Some(v) = row.get::<_, Option<i32>>(name) {
                map.insert("level".to_string(), Value::Number((v as i64).into()));
            }
            continue;
        }
        if name == "member_count" {
            let v: i64 = row.get(name);
            map.insert("memberCount".to_string(), Value::Number(v.into()));
            continue;
        }
        let val: Option<Value> = match *col.type_() {
            Type::BOOL => row.get::<_, Option<bool>>(name).map(Value::Bool),
            _ => row.get::<_, Option<String>>(name).map(Value::String),
        };
        if let Some(v) = val {
            map.insert(name.to_string(), v);
        }
    }
    Value::Object(map)
}

// ── Person ────────────────────────────────────────────────────────────────────

pub(crate) fn person_cols(pii: bool) -> Cols {
    if pii {
        &["id", "name", "unit_id", "mobile", "email"]
    } else {
        &["id", "name", "unit_id"]
    }
}

pub(crate) async fn resolve_person_ids(
    client: &deadpool_postgres::Client,
    flags: &[String],
) -> Result<Vec<String>, AppError> {
    let rows = client
        .query(
            &format!(
                "SELECT id FROM x_org_person WHERE deleted_at IS NULL AND {}",
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(rows.iter().map(|r| r.get("id")).collect())
}

/// GET /jaxrs/person/auth/info/{flag}
/// Person plus identity/group/role lists (Java ActionGetAuthInfo).
pub async fn person_auth_info_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let persons = resolve_person_ids(&client, std::slice::from_ref(&flag)).await?;
    let Some(pid) = persons.first() else {
        return ok_json(Value::Object(serde_json::Map::new()));
    };

    let identity_rows = client
        .query(
            "SELECT id FROM x_org_identity WHERE deleted_at IS NULL AND person_id = $1 ORDER BY id",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let group_rows = client
        .query(
            "SELECT g.id FROM x_org_group g JOIN x_org_group_member m ON m.group_id = g.id \
             WHERE g.deleted_at IS NULL AND m.person_id = $1 ORDER BY g.id",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let role_rows = client
        .query(
            "SELECT DISTINCT r.id FROM x_org_role r \
             JOIN x_org_group_role gr ON gr.role_id = r.id \
             JOIN x_org_group_member m ON m.group_id = gr.group_id \
             WHERE r.deleted_at IS NULL AND m.person_id = $1 ORDER BY r.id",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let person_rows = client
        .query(
            "SELECT id, name, unit_id FROM x_org_person WHERE deleted_at IS NULL AND id = $1",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if person_rows.is_empty() {
        return ok_json(Value::Object(serde_json::Map::new()));
    }

    let identity_list: Vec<String> = identity_rows.iter().map(|r| r.get("id")).collect();
    let group_list: Vec<String> = group_rows.iter().map(|r| r.get("id")).collect();
    let role_list: Vec<String> = role_rows.iter().map(|r| r.get("id")).collect();

    let mut data = row_to_map(&person_rows[0]);
    if let Value::Object(ref mut map) = data {
        map.insert("identityList".to_string(), {
            Value::Array(identity_list.into_iter().map(Value::String).collect())
        });
        map.insert("groupList".to_string(), {
            Value::Array(group_list.into_iter().map(Value::String).collect())
        });
        map.insert("roleList".to_string(), {
            Value::Array(role_list.into_iter().map(Value::String).collect())
        });
    }
    ok_json(data)
}

/// GET /jaxrs/person/nick/name/{flag}: nick name (no dedicated column; returns name).
pub async fn person_nick_name_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name FROM x_org_person WHERE deleted_at IS NULL AND (id = $1 OR name = $1)",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match rows.first() {
        None => ok_json(Value::Object(serde_json::Map::new())),
        Some(row) => ok_json(row_to_map(row)),
    }
}

/// GET /jaxrs/person/mobile/{flag}: explicit single PII lookup (Java ActionGetMobile).
pub async fn person_mobile_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, mobile FROM x_org_person WHERE deleted_at IS NULL AND (id = $1 OR name = $1)",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    match rows.first() {
        None => ok_json(Value::Object(serde_json::Map::new())),
        Some(row) => ok_json(row_to_map(row)),
    }
}

async fn person_batch(
    pool: Extension<Pool>,
    body: Value,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "personList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let pii = include_pii(&body);
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT {} FROM x_org_person WHERE deleted_at IS NULL AND {} ORDER BY id",
                cols_sql(person_cols(pii)),
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/person/list: batch person flag lookup (Java ActionList).
pub async fn person_list(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    person_batch(pool, body).await
}

/// POST /jaxrs/person/list/object: batch person objects (Java ActionListObject);
/// mobile/email excluded by default, enabled explicitly via includePii=true.
pub async fn person_list_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    person_batch(pool, body).await
}

/// GET /jaxrs/person/list/all: all person ids (PII-free).
pub async fn person_list_all(
    pool: Extension<Pool>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id FROM x_org_person WHERE deleted_at IS NULL ORDER BY id",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows
        .iter()
        .map(|r| Value::String(r.get::<_, String>("id")))
        .collect();
    ok_java_list(list.len(), list)
}

/// GET /jaxrs/person/list/all/object: all person objects (PII-free).
pub async fn person_list_all_object(
    pool: Extension<Pool>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, unit_id FROM x_org_person WHERE deleted_at IS NULL ORDER BY id",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/person/has/role: does a person hold any of the given roles
/// (via the group-member/group-role join chain).
pub async fn person_has_role(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let persons = string_list(&body, "personList");
    let roles = string_list(&body, "roleList");
    capped(&persons)?;
    capped(&roles)?;
    if persons.is_empty() || roles.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT p.id FROM x_org_person p \
             JOIN x_org_group_member m ON m.person_id = p.id \
             JOIN x_org_group_role gr ON gr.group_id = m.group_id \
             JOIN x_org_role r ON r.id = gr.role_id AND r.deleted_at IS NULL \
             WHERE p.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) \
               AND (r.id = ANY($2) OR r.name = ANY($2)) ORDER BY p.id",
            &[&persons, &roles],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows
        .iter()
        .map(|r| Value::String(r.get::<_, String>("id")))
        .collect();
    ok_java_list(list.len(), list)
}

async fn identities_of_persons(pool: &Pool, flags: &[String]) -> Result<Vec<String>, AppError> {
    if flags.is_empty() {
        return Ok(vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT i.id FROM x_org_identity i \
             JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
             WHERE i.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY i.id",
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(rows.iter().map(|r| r.get("id")).collect())
}

/// POST /jaxrs/person/list/identity: identities of persons -> {identityList}.
pub async fn person_list_identity(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "personList");
    capped(&flags)?;
    let list = identities_of_persons(&pool, &flags).await?;
    Ok(AxumJson(ActionResult::java_success(
        named_list("identityList", &list),
        list.len() as i64,
        0,
    )))
}

/// POST /jaxrs/person/list/group: groups containing persons -> {groupList}.
pub async fn person_list_group(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT g.id FROM x_org_group g \
         JOIN x_org_group_member m ON m.group_id = g.id \
         JOIN x_org_person p ON p.id = m.person_id AND p.deleted_at IS NULL \
         WHERE g.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY g.id";
    let flags = string_list(&body, "personList");
    capped(&flags)?;
    named_list_response(&pool, "groupList", SQL, &flags).await
}

/// POST /jaxrs/person/list/role: roles held by persons -> {roleList}.
pub async fn person_list_role(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT r.id FROM x_org_role r \
         JOIN x_org_group_role gr ON gr.role_id = r.id \
         JOIN x_org_group_member m ON m.group_id = gr.group_id \
         JOIN x_org_person p ON p.id = m.person_id AND p.deleted_at IS NULL \
         WHERE r.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY r.id";
    let flags = string_list(&body, "personList");
    capped(&flags)?;
    named_list_response(&pool, "roleList", SQL, &flags).await
}

/// POST /jaxrs/person/list/filter/{page}/size/{size}: filtered paging (PII-free).
pub async fn person_list_filter_page_size(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let page = page.max(1);
    let size = size.clamp(1, 200);
    let name_like = body
        .get("name")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string);
    let unit_id = body
        .get("unitId")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let where_clause =
        "WHERE deleted_at IS NULL AND ($1::text IS NULL OR name ILIKE '%' || $1 || '%') \
         AND ($2::text IS NULL OR unit_id = $2)";
    let total_rows = client
        .query_one(
            &format!("SELECT COUNT(*) AS cnt FROM x_org_person {}", where_clause),
            &[&name_like, &unit_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let total: i64 = total_rows.get("cnt");

    let rows = client
        .query(
            &format!(
                "SELECT id, name, unit_id FROM x_org_person {} ORDER BY id LIMIT $3 OFFSET $4",
                where_clause
            ),
            &[&name_like, &unit_id, &size, &((page - 1) * size)],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    Ok(AxumJson(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(total.into())),
            ("page".to_string(), Value::Number(page.into())),
            ("size".to_string(), Value::Number(size.into())),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ── Identity ──────────────────────────────────────────────────────────────────

async fn identity_batch(
    pool: Extension<Pool>,
    body: Value,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "identityList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT id, name, unit_id, person_id FROM x_org_identity \
                 WHERE deleted_at IS NULL AND {} ORDER BY id",
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/identity/list: batch identity lookup (Java IdentityAction#list).
pub async fn identity_list(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    identity_batch(pool, body).await
}

/// POST /jaxrs/identity/list/object: batch identity objects.
pub async fn identity_list_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    identity_batch(pool, body).await
}

/// POST /jaxrs/identity/list/person: identities of persons -> {identityList}.
pub async fn identity_list_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "personList");
    capped(&flags)?;
    let list = identities_of_persons(&pool, &flags).await?;
    Ok(AxumJson(ActionResult::java_success(
        named_list("identityList", &list),
        list.len() as i64,
        0,
    )))
}

async fn identity_list_with_units(
    pool: Extension<Pool>,
    body: Value,
    nested: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const NESTED_SQL: &str = "WITH RECURSIVE sub AS (\
         SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
         UNION \
         SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) \
         SELECT DISTINCT i.id FROM x_org_identity i JOIN sub ON sub.id = i.unit_id \
         WHERE i.deleted_at IS NULL ORDER BY i.id";
    const DIRECT_SQL: &str = "SELECT DISTINCT i.id FROM x_org_identity i \
         JOIN x_org_unit u ON u.id = i.unit_id AND u.deleted_at IS NULL \
         WHERE i.deleted_at IS NULL AND (u.id = ANY($1) OR u.name = ANY($1)) ORDER BY i.id";
    let flags = string_list(&body, "unitList");
    capped(&flags)?;
    let sql = if nested { NESTED_SQL } else { DIRECT_SQL };
    named_list_response(&pool, "identityList", sql, &flags).await
}

/// POST /jaxrs/identity/list/unit/sub/direct: identities directly under units.
pub async fn identity_list_unit_sub_direct(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    identity_list_with_units(pool, body, false).await
}

/// POST /jaxrs/identity/list/unit/sub/nested: identities under nested units (recursive CTE).
pub async fn identity_list_unit_sub_nested(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    identity_list_with_units(pool, body, true).await
}
