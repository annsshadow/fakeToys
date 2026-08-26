//! plan002 U2 收尾 (part 7): role 剩余 2 个 + unitduty 剩余 5 个端点。
//!
//! 对齐 Java RoleAction / UnitDutyAction 其余方法。约定见 endpoints.rs。

use axum::{
    extract::{Extension, Json},
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::endpoints::{capped, named_list, normalize_flags, ok_java_list, ok_json, row_to_map, string_field, string_list, PICK_ANY};

/// POST /jaxrs/role/list/object (Java ActionListObject)：批量角色对象。
pub async fn role_list_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "roleList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    const SQL: &str = "SELECT id, name, description FROM x_org_role \
         WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) ORDER BY id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/role/list/person/object (Java ActionListWithPersonObject)。
pub async fn role_list_person_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "personList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    const SQL: &str = "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
         JOIN x_org_group_member m ON m.person_id = p.id \
         JOIN x_org_group_role gr ON gr.group_id = m.group_id \
         JOIN x_org_role r ON r.id = gr.role_id AND r.deleted_at IS NULL \
         WHERE p.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY p.id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_java_list(data.len(), data)
}

// ── unitduty ──────────────────────────────────────────────────────────────────

fn finish_duty_rows(
    rows: Vec<deadpool_postgres::tokio_postgres::Row>,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_java_list(data.len(), data)
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::java_success(
            named_list("identityList", &list),
            list.len() as i64,
            0,
        )))
    }
}

/// Wi{name?, nameList?} ∪ Wi{unit?, unitList?} 的合并解析。
pub(crate) fn merged_flags(body: &Value, single_key: &str, list_key: &str) -> Vec<String> {
    let mut all = string_list(body, list_key);
    if let Some(one) = string_field(body, single_key) {
        all.push(one);
    }
    normalize_flags(all)
}

async fn duty_identities_by_unit_name(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let names = merged_flags(&body, "name", "nameList");
    let units = merged_flags(&body, "unit", "unitList");
    capped(&names)?;
    capped(&units)?;
    if names.is_empty() || units.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let recursive = body
        .get("recursiveUnit")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    // recursiveUnit=true 时组织范围取嵌套子树，否则仅直接指定组织
    let scope_sql = if recursive {
        "WITH RECURSIVE sub AS (\
         SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND PICK_UNITS\
         UNION \
         SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL)\
         SELECT d.id, d.name, d.unit_id, d.identity_id FROM x_org_duty d \
         WHERE d.deleted_at IS NULL AND d.name = ANY($2) \
         AND d.unit_id IN (SELECT id FROM sub) ORDER BY d.id"
            .replace("PICK_UNITS", PICK_ANY)
    } else {
        "SELECT d.id, d.name, d.unit_id, d.identity_id FROM x_org_duty d \
         JOIN x_org_unit u ON u.id = d.unit_id AND u.deleted_at IS NULL \
         WHERE d.deleted_at IS NULL AND d.name = ANY($2) \
         AND (u.id = ANY($1) OR u.name = ANY($1)) ORDER BY d.id"
            .to_string()
    };
    let final_sql = if objects {
        scope_sql
    } else {
        scope_sql.replacen(
            "SELECT d.id, d.name, d.unit_id, d.identity_id",
            "SELECT DISTINCT d.identity_id",
            1,
        )
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(&final_sql, &[&units, &names])
        .await
        .map_err(|_| AppError::Internal)?;
    finish_duty_rows(rows, objects)
}

/// POST /jaxrs/unitduty/list/identity/unit/name
/// (Java ActionListIdentityWithUnitWithName)：按职务名称+组织范围取身份。
pub async fn unitduty_list_identity_unit_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    duty_identities_by_unit_name(pool, body, false).await
}

/// POST /jaxrs/unitduty/list/identity/unit/name/object。
pub async fn unitduty_list_identity_unit_name_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    duty_identities_by_unit_name(pool, body, true).await
}

/// POST /jaxrs/unitduty/list/name/identity (Java ActionListNameWithIdentity，
/// Wi{identityList})：身份持有的职务名称集合。
pub async fn unitduty_list_name_identity(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "identityList"));
    capped(&flags)?;
    const SQL: &str = "SELECT DISTINCT d.name FROM x_org_duty d \
         WHERE d.deleted_at IS NULL AND d.identity_id IN (\
             SELECT id FROM x_org_identity WHERE deleted_at IS NULL \
             AND (id = ANY($1) OR name = ANY($1))) ORDER BY d.name";
    named_list_duty(pool, SQL, flags).await
}

async fn named_list_duty(
    pool: Extension<Pool>,
    sql: &str,
    flags: Vec<String>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    if flags.is_empty() {
        return Ok(AxumJson(ActionResult::java_success(
            named_list("nameList", &[]),
            0,
            0,
        )));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
    Ok(AxumJson(ActionResult::java_success(
        named_list("nameList", &list),
        list.len() as i64,
        0,
    )))
}

/// POST /jaxrs/unitduty/list/unit/object (Java ActionListWithUnitObject，
/// Wi{unitList})：组织的职务对象（直接所属）。
pub async fn unitduty_list_unit_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "unitList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    const SQL: &str = "SELECT d.id, d.name, d.unit_id, d.identity_id FROM x_org_duty d \
         JOIN x_org_unit u ON u.id = d.unit_id AND u.deleted_at IS NULL \
         WHERE d.deleted_at IS NULL AND (u.id = ANY($1) OR u.name = ANY($1)) ORDER BY d.id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/unitduty/find/by/unit/name (Java ActionGetWithUnitWithName，
/// Wi{name, unit})：按组织+职务名精确取单个职务对象或 null。
pub async fn unitduty_find_by_unit_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let Some(name) = string_field(&body, "name") else {
        return Err(AppError::BadRequest("name is required".into()));
    };
    let Some(unit) = string_field(&body, "unit") else {
        return Err(AppError::BadRequest("unit is required".into()));
    };
    const SQL: &str = "SELECT d.id, d.name, d.unit_id, d.identity_id FROM x_org_duty d \
         WHERE d.deleted_at IS NULL AND d.name = $1 AND d.unit_id IN (\
             SELECT id FROM x_org_unit WHERE deleted_at IS NULL \
             AND (id = $2 OR name = $2) ORDER BY id LIMIT 1) ORDER BY d.id LIMIT 1";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(SQL, &[&name, &unit])
        .await
        .map_err(|_| AppError::Internal)?;
    match rows.first() {
        None => ok_json(Value::Null),
        Some(row) => ok_json(row_to_map(row)),
    }
}
