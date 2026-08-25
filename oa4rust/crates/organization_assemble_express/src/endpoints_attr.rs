//! plan002 U2 收尾 (part 8): personattribute / unitattribute 各 5 个端点 +
//! empower / empowerlog / distinguishedname 各 1 个端点。
//!
//! 属性写入（set/append）在事务内执行真实 INSERT/DELETE，值经归一化查重
//! （trim、去空、保序去重；append 额外跳过已存在值）。

use axum::{
    extract::Extension,
    Json,
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::endpoints::{capped, count_data, normalize_flags, ok_json, string_field, string_list};

// ── personattribute ───────────────────────────────────────────────────────────

/// POST /jaxrs/personattribute/list/name/person (Java ActionListNameWithPerson，
/// Wi{personList})：人员的属性名集合。
pub async fn personattr_list_name_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT a.attribute_key FROM x_org_person_attribute a \
         JOIN x_org_person p ON p.id = a.person_id AND p.deleted_at IS NULL \
         WHERE a.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) \
         ORDER BY a.attribute_key";
    named_keys(pool, "nameList", SQL, body, "personList").await
}

/// POST /jaxrs/personattribute/list/attribute/person/name
/// (Java ActionListAttributeWithPersonWithName，Wi{person, name})：属性值列表。
pub async fn personattr_list_attribute_person_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let (Some(person), Some(name)) =
        (string_field(&body, "person"), string_field(&body, "name"))
    else {
        return Ok(AxumJson(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([("attributeList".to_string(), Value::Array(vec![]))]),
        ))));
    };
    const SQL: &str = "SELECT a.attribute_value FROM x_org_person_attribute a \
         JOIN x_org_person p ON p.id = a.person_id AND p.deleted_at IS NULL \
         WHERE a.deleted_at IS NULL AND (p.id = $1 OR p.name = $1) \
         AND a.attribute_key = $2 ORDER BY a.id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(SQL, &[&person, &name])
        .await
        .map_err(|_| AppError::Internal)?;
    let values: Vec<String> = rows.iter().map(|r| r.get::<_, Option<String>>(0).unwrap_or_default()).collect();
    Ok(AxumJson(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([(
            "attributeList".to_string(),
            Value::Array(values.into_iter().map(Value::String).collect()),
        )]),
    ))))
}

/// POST /jaxrs/personattribute/list/person/object (Java ActionListWithPersonObject)：
/// 人员属性对象（按 person+key 分组，valueList 聚合）。
pub async fn personattr_list_person_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "personList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    const SQL: &str = "SELECT a.id, a.person_id, a.attribute_key, a.attribute_value \
         FROM x_org_person_attribute a \
         WHERE a.deleted_at IS NULL AND (a.person_id = ANY($1) OR a.person_id IN (\
             SELECT id FROM x_org_person WHERE deleted_at IS NULL AND name = ANY($1))) \
         ORDER BY a.person_id, a.attribute_key, a.id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    let mut cur: Option<(String, String), > = None;
    for row in &rows {
        let pid: String = row.get("person_id");
        let key: String = row.get("attribute_key");
        let value: Option<String> = row.get("attribute_value");
        if cur.as_ref() != Some(&(pid.clone(), key.clone())) {
            cur = Some((pid.clone(), key.clone()));
            data.push(Value::Object(serde_json::Map::from_iter([
                ("person".to_string(), Value::String(pid)),
                ("name".to_string(), Value::String(key)),
                ("attributeList".to_string(), Value::Array(vec![])),
            ])));
        }
        if let Some(Value::Object(m)) = data.last_mut() {
            if let Some(Value::Array(list)) = m.get_mut("attributeList") {
                list.push(Value::String(value.unwrap_or_default()));
            }
        }
    }
    ok_json(count_data(data.len(), data))
}

async fn attr_write_values(
    pool: &Pool,
    body: &Value,
    table: &str,
    owner_col: &str,
    append: bool,
) -> Result<(), AppError> {
    let Some(owner) = string_field(body, "person").or_else(|| string_field(body, "unit")) else {
        return Err(AppError::BadRequest("person/unit is required".into()));
    };
    let Some(name) = string_field(body, "name") else {
        return Err(AppError::BadRequest("name is required".into()));
    };
    let values = normalize_flags(string_list(body, "attributeList"));
    let mut client = pool.get().await.map_err(|_| AppError::Internal)?;
    let tx = client.transaction().await.map_err(|_| AppError::Internal)?;

    // 归一化查重：append 仅补缺失值；set 全量替换
    let existing: Vec<String> = if append {
        let real_sql = format!(
            "SELECT COALESCE(attribute_value, '') FROM {table} \
             WHERE deleted_at IS NULL AND {owner_col} IN (\
                 SELECT id FROM x_org_{owner_tbl} WHERE deleted_at IS NULL \
                 AND (id = $2 OR name = $2)) AND attribute_key = $1",
            owner_tbl = if owner_col == "person_id" { "person" } else { "unit" },
        );
        tx.query(&real_sql, &[&name, &owner])
            .await
            .map_err(|_| AppError::Internal)?
            .iter()
            .map(|r| r.get::<_, String>(0))
            .collect()
    } else {
        vec![]
    };

    let insert_sql = format!(
        "INSERT INTO {table} (id, {owner_col}, attribute_key, attribute_value) \
         VALUES ($1, (SELECT id FROM x_org_{owner_tbl} WHERE deleted_at IS NULL \
             AND (id = $2 OR name = $2) ORDER BY id LIMIT 1), $3, $4)",
        owner_tbl = if owner_col == "person_id" { "person" } else { "unit" },
    );
    if !append {
        let delete_sql = format!(
            "UPDATE {table} SET deleted_at = CURRENT_TIMESTAMP \
             WHERE deleted_at IS NULL AND {owner_col} IN (\
                 SELECT id FROM x_org_{owner_tbl} WHERE deleted_at IS NULL \
                 AND (id = $2 OR name = $2)) AND attribute_key = $1",
            owner_tbl = if owner_col == "person_id" { "person" } else { "unit" },
        );
        tx.execute(&delete_sql, &[&name, &owner])
            .await
            .map_err(|_| AppError::Internal)?;
    }
    for v in &values {
        if append && existing.iter().any(|e| e.eq_ignore_ascii_case(v)) {
            continue;
        }
        let id = uuid::Uuid::new_v4().to_string();
        tx.execute(&insert_sql, &[&id, &owner, &name, v])
            .await
            .map_err(|_| AppError::Internal)?;
    }
    tx.commit().await.map_err(|_| AppError::Internal)?;
    Ok(())
}

fn wrap_true() -> Value {
    Value::Object(serde_json::Map::from_iter([("value".to_string(), Value::Bool(true))]))
}

/// POST /jaxrs/personattribute/set/person/name (Java ActionSetWithPersonWithName，
/// Wi{person, name, attributeList})：全量替换属性值。
pub async fn personattr_set_person_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    attr_write_values(&pool, &body, "x_org_person_attribute", "person_id", false).await?;
    Ok(AxumJson(ActionResult::success(wrap_true())))
}

/// POST /jaxrs/personattribute/append/person/name (Java ActionAppendWithPersonWithName)：
/// 追加缺失值（归一化查重后）。
pub async fn personattr_append_person_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    attr_write_values(&pool, &body, "x_org_person_attribute", "person_id", true).await?;
    Ok(AxumJson(ActionResult::success(wrap_true())))
}

async fn named_keys(
    pool: Extension<Pool>,
    key: &'static str,
    sql: &str,
    body: Value,
    list_field: &str,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, list_field));
    capped(&flags)?;
    if flags.is_empty() {
        return Ok(AxumJson(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([(key.to_string(), Value::Array(vec![]))]),
        ))));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
    Ok(AxumJson(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([(
            key.to_string(),
            Value::Array(list.into_iter().map(Value::String).collect()),
        )]),
    ))))
}

// ── unitattribute（镜像 personattribute，owner 为 unit） ──────────────────────

/// POST /jaxrs/unitattribute/list/name/unit (Wi{unitList})：组织的属性名集合。
pub async fn unitattr_list_name_unit(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT a.attribute_key FROM x_org_unit_attribute a \
         JOIN x_org_unit u ON u.id = a.unit_id AND u.deleted_at IS NULL \
         WHERE a.deleted_at IS NULL AND (u.id = ANY($1) OR u.name = ANY($1)) \
         ORDER BY a.attribute_key";
    named_keys(pool, "nameList", SQL, body, "unitList").await
}

/// POST /jaxrs/unitattribute/list/attribute/unit/name (Wi{unit, name})。
pub async fn unitattr_list_attribute_unit_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let (Some(unit), Some(name)) = (string_field(&body, "unit"), string_field(&body, "name"))
    else {
        return Ok(AxumJson(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([("attributeList".to_string(), Value::Array(vec![]))]),
        ))));
    };
    const SQL: &str = "SELECT a.attribute_value FROM x_org_unit_attribute a \
         JOIN x_org_unit u ON u.id = a.unit_id AND u.deleted_at IS NULL \
         WHERE a.deleted_at IS NULL AND (u.id = $1 OR u.name = $1) \
         AND a.attribute_key = $2 ORDER BY a.id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(SQL, &[&unit, &name])
        .await
        .map_err(|_| AppError::Internal)?;
    let values: Vec<String> =
        rows.iter().map(|r| r.get::<_, Option<String>>(0).unwrap_or_default()).collect();
    Ok(AxumJson(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([(
            "attributeList".to_string(),
            Value::Array(values.into_iter().map(Value::String).collect()),
        )]),
    ))))
}

/// POST /jaxrs/unitattribute/list/unit/object (Java ActionListWithUnitObject)。
pub async fn unitattr_list_unit_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "unitList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    const SQL: &str = "SELECT a.id, a.unit_id, a.attribute_key, a.attribute_value \
         FROM x_org_unit_attribute a \
         WHERE a.deleted_at IS NULL AND (a.unit_id = ANY($1) OR a.unit_id IN (\
             SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND name = ANY($1))) \
         ORDER BY a.unit_id, a.attribute_key, a.id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    let mut cur: Option<(String, String)> = None;
    for row in &rows {
        let uid: String = row.get("unit_id");
        let key: String = row.get("attribute_key");
        let value: Option<String> = row.get("attribute_value");
        if cur.as_ref() != Some(&(uid.clone(), key.clone())) {
            cur = Some((uid.clone(), key.clone()));
            data.push(Value::Object(serde_json::Map::from_iter([
                ("unit".to_string(), Value::String(uid)),
                ("name".to_string(), Value::String(key)),
                ("attributeList".to_string(), Value::Array(vec![])),
            ])));
        }
        if let Some(Value::Object(m)) = data.last_mut() {
            if let Some(Value::Array(list)) = m.get_mut("attributeList") {
                list.push(Value::String(value.unwrap_or_default()));
            }
        }
    }
    ok_json(count_data(data.len(), data))
}

/// POST /jaxrs/unitattribute/set/unit/name：全量替换组织属性值。
pub async fn unitattr_set_unit_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    attr_write_values(&pool, &body, "x_org_unit_attribute", "unit_id", false).await?;
    Ok(AxumJson(ActionResult::success(wrap_true())))
}

/// POST /jaxrs/unitattribute/append/unit/name：追加缺失的组织属性值。
pub async fn unitattr_append_unit_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    attr_write_values(&pool, &body, "x_org_unit_attribute", "unit_id", true).await?;
    Ok(AxumJson(ActionResult::success(wrap_true())))
}

// ── empower / empowerlog / distinguishedname ─────────────────────────────────

/// POST /jaxrs/empower/list/identity/object (Java ActionListWithIdentityObject，
/// Wi{identityList})：身份维度的授权对象。x_empower 的 from_identity/to_identity
/// 由迁移 071 提供；application/process/work 维度在当前表结构中不存在，
/// 不做过滤（如实返回身份命中的授权行）。
pub async fn empower_list_identity_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "identityList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    const SQL: &str = "SELECT id, from_person, to_person, from_identity, to_identity, role_id, enabled \
         FROM x_empower WHERE deleted_at IS NULL \
         AND (from_identity = ANY($1) OR to_identity = ANY($1)) ORDER BY id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    for row in &rows {
        let mut obj = serde_json::Map::new();
        obj.insert("id".to_string(), Value::String(row.get("id")));
        for col in ["from_person", "to_person", "from_identity", "to_identity", "role_id"] {
            let v: Option<String> = row.get(col);
            obj.insert(col.to_string(), v.map(Value::String).unwrap_or(Value::Null));
        }
        obj.insert("enabled".to_string(), Value::Bool(row.get::<_, Option<bool>>("enabled").unwrap_or(false)));
        data.push(Value::Object(obj));
    }
    ok_json(count_data(data.len(), data))
}

/// POST /jaxrs/empowerlog (Java EmpowerLogAction#create，Wi extends EmpowerLog)：
/// 校验必填字段后落库 x_org_empower_log。
pub async fn empowerlog_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let required = [
        ("application", string_field(&body, "application")),
        ("process", string_field(&body, "process")),
        ("fromIdentity", string_field(&body, "fromIdentity")),
        ("toIdentity", string_field(&body, "toIdentity")),
    ];
    for (field, value) in &required {
        if value.is_none() {
            return Err(AppError::BadRequest(format!("{field} is empty")));
        }
    }
    let work = string_field(&body, "work");
    const SQL: &str = "INSERT INTO x_org_empower_log \
         (id, application, process, work, from_identity, to_identity) \
         VALUES ($1, $2, $3, $4, $5, $6)";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            SQL,
            &[
                &uuid::Uuid::new_v4().to_string(),
                &required[0].1,
                &required[1].1,
                &work,
                &required[2].1,
                &required[3].1,
            ],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(AxumJson(ActionResult::success(wrap_true())))
}

/// POST /jaxrs/distinguishedname/list (Java distinguishedname ActionList，
/// Wi/Wo 均为 {distinguishedNameList})：过滤出在任一组织实体中真实存在的 DN。
pub async fn distinguishedname_list(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "distinguishedNameList"));
    capped(&flags)?;
    if flags.is_empty() {
        return Ok(AxumJson(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([(
                "distinguishedNameList".to_string(),
                Value::Array(vec![]),
            )]),
        ))));
    }
    // 占位符均为静态结构，无用户输入拼接；$1 为归一化后的 flag 数组
    let pick = "(id = ANY($1) OR name = ANY($1))";
    let sql = format!(
        "SELECT id FROM x_org_person WHERE deleted_at IS NULL AND {pick} \
         UNION SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND {pick} \
         UNION SELECT id FROM x_org_identity WHERE deleted_at IS NULL AND {pick} \
         UNION SELECT id FROM x_org_group WHERE deleted_at IS NULL AND {pick} \
         UNION SELECT id FROM x_org_role WHERE deleted_at IS NULL AND {pick} \
         UNION SELECT id FROM x_org_duty WHERE deleted_at IS NULL AND {pick}"
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    use std::collections::HashSet;
    let found: HashSet<String> = rows.iter().map(|r| r.get::<_, String>(0)).collect();
    let valid: Vec<String> = flags.into_iter().filter(|f| found.contains(f)).collect();
    Ok(AxumJson(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([(
            "distinguishedNameList".to_string(),
            Value::Array(valid.into_iter().map(Value::String).collect()),
        )]),
    ))))
}
