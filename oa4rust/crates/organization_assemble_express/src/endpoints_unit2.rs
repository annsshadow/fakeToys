//! plan002 U2 收尾 (part 4): unit 剩余 29 个端点。
//!
//! 对齐 Java UnitAction 其余方法。新结构 x_org_unit."type" 由迁移 071 提供。
//! 约定沿用 endpoints.rs 模块注释（无认证、批量≤100、PII 门控不涉及本组）。

use axum::{
    extract::{Extension, Json, Path},
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::endpoints::{
    bool_field, capped, count_data, int_list, named_list, normalize_flags, ok_json, row_to_map,
    string_field, string_list, wrap_bool,
};

const UNIT_COLS: &str = "id, name, parent_id, level";
const UNIT_COLS_TYPE: &str = "id, name, parent_id, level, \"type\"";

fn finish_rows(
    rows: Vec<deadpool_postgres::tokio_postgres::Row>,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_json(count_data(data.len(), data))
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::success(named_list(
            "unitList",
            &list,
        ))))
    }
}

/// 身份所在组织的递归上级链（含自身），种子为身份直接所属组织（pick 单个）。
const SUP_CHAIN_OF_IDENTITY_UNIT: &str = "\
     WITH RECURSIVE base AS (\
         SELECT i.unit_id AS id FROM x_org_identity i \
         WHERE i.deleted_at IS NULL AND (i.id = $1 OR i.name = $1) \
           AND i.unit_id IS NOT NULL ORDER BY i.id LIMIT 1),\
     sup AS (\
         SELECT u.id, u.parent_id FROM x_org_unit u \
         JOIN base b ON u.id = b.id WHERE u.deleted_at IS NULL\
         UNION\
         SELECT pu.id, pu.parent_id FROM x_org_unit pu \
         JOIN sup s ON s.parent_id = pu.id WHERE pu.deleted_at IS NULL)";

async fn unit_pick_on_identity_chain(
    pool: Extension<Pool>,
    body: Value,
    by_level: bool,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let Some(identity) = string_field(&body, "identity") else {
        return ok_json(Value::Null);
    };
    let second: Option<(String, bool)> = if by_level {
        body.get("level")
            .and_then(|v| v.as_i64())
            .filter(|l| *l > 0)
            .map(|l| (l.to_string(), false))
    } else {
        string_field(&body, "type").map(|t| (t, true))
    };
    let Some((second, is_text)) = second else {
        return ok_json(Value::Null);
    };
    // level 为整数比较、type 为文本比较；用两个占位类型分支构造 SQL
    let predicate = if is_text {
        "\"type\" = $2".to_string()
    } else {
        "level = $2::int".to_string()
    };
    let sql = format!(
        "{SUP_CHAIN_OF_IDENTITY_UNIT} \
         SELECT {cols} FROM x_org_unit WHERE deleted_at IS NULL AND {predicate} \
         AND id IN (SELECT id FROM sup) ORDER BY id LIMIT 1",
        cols = if objects { UNIT_COLS_TYPE } else { "id" },
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = if is_text {
        client
            .query(&sql, &[&identity, &second])
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        let lvl: Option<i32> = second.parse().ok();
        match lvl {
            Some(lvl) => client
                .query(&sql, &[&identity, &lvl])
                .await
                .map_err(|_| AppError::Internal)?,
            None => return ok_json(Value::Null),
        }
    };
    match rows.first() {
        None => ok_json(Value::Null),
        Some(row) => {
            if objects {
                ok_json(row_to_map(row))
            } else {
                let id: String = row.get("id");
                Ok(AxumJson(ActionResult::success(Value::Object(
                    serde_json::Map::from_iter([("unit".to_string(), Value::String(id))]),
                ))))
            }
        }
    }
}

/// POST /jaxrs/unit/identity/level (Java ActionGetWithIdentityWithLevel)：{unit|null}。
pub async fn unit_get_with_identity_with_level(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_pick_on_identity_chain(pool, body, true, false).await
}

/// POST /jaxrs/unit/identity/level/object。
pub async fn unit_get_with_identity_with_level_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_pick_on_identity_chain(pool, body, true, true).await
}

/// POST /jaxrs/unit/identity/type (Java ActionGetWithIdentityWithType)：{unit|null}。
pub async fn unit_get_with_identity_with_type(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_pick_on_identity_chain(pool, body, false, false).await
}

/// POST /jaxrs/unit/identity/type/object：命中的组织对象或 null。
pub async fn unit_get_with_identity_with_type_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_pick_on_identity_chain(pool, body, false, true).await
}

// ── unit/list/identity(+object)、identity/sup/nested(+object) ─────────────────

const UNITS_OF_IDENTITIES_OBJ: &str = "SELECT DISTINCT u.id, u.name, u.parent_id, u.level \
     FROM x_org_unit u JOIN x_org_identity i ON i.unit_id = u.id AND i.deleted_at IS NULL \
     WHERE u.deleted_at IS NULL AND (i.id = ANY($1) OR i.name = ANY($1)) ORDER BY u.level, u.id";

async fn units_of_identities(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "identityList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = if objects {
        UNITS_OF_IDENTITIES_OBJ.to_string()
    } else {
        "SELECT DISTINCT u.id FROM x_org_unit u \
         JOIN x_org_identity i ON i.unit_id = u.id AND i.deleted_at IS NULL \
         WHERE u.deleted_at IS NULL AND (i.id = ANY($1) OR i.name = ANY($1)) ORDER BY u.level, u.id"
            .to_string()
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    finish_rows(rows, objects)
}

/// POST /jaxrs/unit/list/identity (Java ActionListWithIdentity)：身份所在组织。
pub async fn unit_list_identity(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    units_of_identities(pool, body, false).await
}

/// POST /jaxrs/unit/list/identity/object。
pub async fn unit_list_identity_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    units_of_identities(pool, body, true).await
}

/// 递归逐级上级（不含种子自身）的通用实现。seeds_sql 必须产出 (id, parent_id)。
async fn sup_nested_of_units(
    pool: Extension<Pool>,
    flags_key: &str,
    seeds_sql: &str,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, flags_key));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = format!(
        "WITH RECURSIVE seeds AS ({seeds}), \
         sup AS (\
             SELECT u.id, u.parent_id FROM x_org_unit u JOIN seeds s ON u.parent_id = s.id \
             WHERE u.deleted_at IS NULL\
             UNION\
             SELECT pu.id, pu.parent_id FROM x_org_unit pu JOIN sup ON sup.parent_id = pu.id \
             WHERE pu.deleted_at IS NULL) \
         SELECT {cols} FROM x_org_unit WHERE deleted_at IS NULL AND id IN (SELECT id FROM sup) \
         ORDER BY level, id",
        seeds = seeds_sql,
        cols = if objects { UNIT_COLS } else { "id" }
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    finish_rows(rows, objects)
}

const SEED_UNITS_OF_IDENTITIES: &str = "SELECT DISTINCT u.id, u.parent_id FROM x_org_unit u \
     JOIN x_org_identity i ON i.unit_id = u.id AND i.deleted_at IS NULL \
     WHERE u.deleted_at IS NULL AND (i.id = ANY($1) OR i.name = ANY($1))";

/// POST /jaxrs/unit/list/identity/sup/nested (Java ActionListWithIdentitySupNested)。
pub async fn unit_list_identity_sup_nested(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    sup_nested_of_units(pool, "identityList", SEED_UNITS_OF_IDENTITIES, body, false).await
}

/// POST /jaxrs/unit/list/identity/sup/nested/object。
pub async fn unit_list_identity_sup_nested_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    sup_nested_of_units(pool, "identityList", SEED_UNITS_OF_IDENTITIES, body, true).await
}

// ── unit/list/level(+object)、list/level/name/object ──────────────────────────

async fn level_query(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let levels = int_list(&body, "levelList")?;
    if levels.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = format!(
        "SELECT {} FROM x_org_unit WHERE deleted_at IS NULL AND level = ANY($1) ORDER BY level, id",
        if objects { UNIT_COLS } else { "id" }
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, &[&levels]).await.map_err(|_| AppError::Internal)?;
    finish_rows(rows, objects)
}

/// POST /jaxrs/unit/list/level (Java ActionListWithLevel，Wi{levelList})。
pub async fn unit_list_level(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    level_query(pool, body, false).await
}

/// POST /jaxrs/unit/list/level/object。
pub async fn unit_list_level_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    level_query(pool, body, true).await
}

/// POST /jaxrs/unit/list/level/name/object (Java ActionListWithLevelNameObject，
/// Wi{unitList})：组织对象 + levelName 根路径 + 直接下级组织/身份/职务计数。
pub async fn unit_list_level_name_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "WITH RECURSIVE chain AS (\
         SELECT r.id, r.name::text AS path FROM x_org_unit r \
         WHERE r.deleted_at IS NULL AND NOT EXISTS (\
             SELECT 1 FROM x_org_unit pp WHERE pp.id = r.parent_id AND pp.deleted_at IS NULL)\
         UNION ALL \
         SELECT c.id, ch.path || '/' || c.name FROM x_org_unit c \
         JOIN chain ch ON c.parent_id = ch.id WHERE c.deleted_at IS NULL),\
         seeds AS (SELECT id FROM x_org_unit WHERE deleted_at IS NULL \
             AND (id = ANY($1) OR name = ANY($1)))\
         SELECT u.id, u.name, u.parent_id, u.level, ch.path AS level_name, \
           (SELECT COUNT(*) FROM x_org_unit su WHERE su.parent_id = u.id AND su.deleted_at IS NULL) AS sub_direct_unit_count, \
           (SELECT COUNT(*) FROM x_org_identity si WHERE si.unit_id = u.id AND si.deleted_at IS NULL) AS sub_direct_identity_count, \
           (SELECT COUNT(*) FROM x_org_duty sd WHERE sd.unit_id = u.id AND sd.deleted_at IS NULL) AS sub_direct_duty_count \
         FROM x_org_unit u JOIN seeds s ON s.id = u.id JOIN chain ch ON ch.id = u.id ORDER BY u.id";

    let flags = normalize_flags(string_list(&body, "unitList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    for row in &rows {
        let mut obj = row_to_map(row);
        let level_name: Option<String> = row.get("level_name");
        let sub_units: i64 = row.get("sub_direct_unit_count");
        let sub_ids: i64 = row.get("sub_direct_identity_count");
        let sub_duties: i64 = row.get("sub_direct_duty_count");
        if let Value::Object(ref mut m) = obj {
            if let Some(p) = level_name {
                m.insert("matchKey".to_string(), Value::String(p.clone()));
                m.insert("levelName".to_string(), Value::String(p));
            }
            m.insert("subDirectUnitCount".to_string(), Value::Number(sub_units.into()));
            m.insert("subDirectIdentityCount".to_string(), Value::Number(sub_ids.into()));
            m.insert("subDirectDutyCount".to_string(), Value::Number(sub_duties.into()));
        }
        data.push(obj);
    }
    ok_json(count_data(data.len(), data))
}

// ── unit/list/person(+object)、person/sup/nested(+object) ─────────────────────

const UNITS_OF_PERSONS_OBJ: &str = "SELECT DISTINCT u.id, u.name, u.parent_id, u.level \
     FROM x_org_unit u JOIN x_org_person p ON p.unit_id = u.id AND p.deleted_at IS NULL \
     WHERE u.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY u.level, u.id";

async fn units_of_persons(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "personList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = if objects {
        UNITS_OF_PERSONS_OBJ.to_string()
    } else {
        "SELECT DISTINCT u.id FROM x_org_unit u \
         JOIN x_org_person p ON p.unit_id = u.id AND p.deleted_at IS NULL \
         WHERE u.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY u.level, u.id"
            .to_string()
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    finish_rows(rows, objects)
}

/// POST /jaxrs/unit/list/person (Java ActionListWithPerson)：人员所在组织。
pub async fn unit_list_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    units_of_persons(pool, body, false).await
}

/// POST /jaxrs/unit/list/person/object。
pub async fn unit_list_person_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    units_of_persons(pool, body, true).await
}

const SEED_UNITS_OF_PERSONS_SUP: &str = "SELECT DISTINCT par.id, par.parent_id FROM x_org_unit par \
     WHERE par.deleted_at IS NULL AND par.id IN (\
         SELECT p.unit_id FROM x_org_person p WHERE p.deleted_at IS NULL \
         AND p.unit_id IS NOT NULL AND (p.id = ANY($1) OR p.name = ANY($1)))";

/// POST /jaxrs/unit/list/person/sup/nested (Java ActionListWithPersonSupNested)。
pub async fn unit_list_person_sup_nested(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    sup_nested_of_units(pool, "personList", SEED_UNITS_OF_PERSONS_SUP, body, false).await
}

/// POST /jaxrs/unit/list/person/sup/nested/object。
pub async fn unit_list_person_sup_nested_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    sup_nested_of_units(pool, "personList", SEED_UNITS_OF_PERSONS_SUP, body, true).await
}

// ── unitattribute / unitduty 关联查询 ─────────────────────────────────────────

async fn attr_units(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let Some(name) = string_field(&body, "name") else {
        return ok_json(named_list("unitList", &[]));
    };
    let attribute = string_field(&body, "attribute");
    let sql = format!(
        "SELECT {} FROM x_org_unit u \
         JOIN x_org_unit_attribute a ON a.unit_id = u.id AND a.deleted_at IS NULL \
         WHERE u.deleted_at IS NULL AND a.attribute_key = $1 \
           AND ($2::text IS NULL OR a.attribute_value = $2) ORDER BY u.id",
        if objects { UNIT_COLS } else { "id" }
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(&sql, &[&name, &attribute])
        .await
        .map_err(|_| AppError::Internal)?;
    finish_rows(rows, objects)
}

/// POST /jaxrs/unit/list/unitattribute (Java ActionListWithUnitAttribute，Wi{name, attribute})。
pub async fn unit_list_unitattribute(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    attr_units(pool, body, false).await
}

/// POST /jaxrs/unit/list/unitattribute/object。
pub async fn unit_list_unitattribute_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    attr_units(pool, body, true).await
}

async fn duty_units(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let Some(name) = string_field(&body, "name") else {
        return ok_json(named_list("unitList", &[]));
    };
    let identity = string_field(&body, "identity");
    let sql = format!(
        "SELECT {} FROM x_org_unit u \
         JOIN x_org_duty d ON d.unit_id = u.id AND d.deleted_at IS NULL \
         WHERE u.deleted_at IS NULL AND d.name = $1 \
           AND ($2::text IS NULL OR d.identity_id = $2) ORDER BY u.id",
        if objects { UNIT_COLS } else { "id" }
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(&sql, &[&name, &identity])
        .await
        .map_err(|_| AppError::Internal)?;
    finish_rows(rows, objects)
}

/// POST /jaxrs/unit/list/unitduty (Java ActionListWithUnitDuty，Wi{name, identity})：
/// 拥有指定职务名称（可叠加任职身份）的组织。
pub async fn unit_list_unitduty(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    duty_units(pool, body, false).await
}

/// POST /jaxrs/unit/list/unitduty/object。
pub async fn unit_list_unitduty_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    duty_units(pool, body, true).await
}

// ── check/unit/has/*、list/types(+object)、GET type/{type}/object、tree ────────

/// POST /jaxrs/unit/check/unit/has/identity (Java ActionHasIdentity，Wi{unit, identity, recursive})。
pub async fn unit_check_unit_has_identity(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let (Some(unit), Some(identity)) =
        (string_field(&body, "unit"), string_field(&body, "identity"))
    else {
        return Ok(AxumJson(ActionResult::success(wrap_bool(false))));
    };
    let recursive = bool_field(&body, "recursive", true);
    let sql = if recursive {
        "WITH RECURSIVE sub AS (\
             SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = $1 OR name = $1)\
             UNION \
             SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) \
         SELECT EXISTS(SELECT 1 FROM x_org_identity i WHERE i.deleted_at IS NULL \
             AND (i.id = $2 OR i.name = $2) AND i.unit_id IN (SELECT id FROM sub))"
    } else {
        "SELECT EXISTS(SELECT 1 FROM x_org_identity i \
         JOIN x_org_unit u ON u.id = i.unit_id AND u.deleted_at IS NULL \
         WHERE i.deleted_at IS NULL AND (i.id = $2 OR i.name = $2) \
         AND (u.id = $1 OR u.name = $1))"
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(sql, &[&unit, &identity])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(AxumJson(ActionResult::success(wrap_bool(row.get::<_, bool>(0)))))
}

/// POST /jaxrs/unit/check/unit/has/unit (Java ActionHasUnit，Wi{unit, subUnit, recursive})：
/// 校验 subUnit 是否属于 unit 的下级（recursive 时含任意层级）。
pub async fn unit_check_unit_has_unit(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let (Some(unit), Some(sub_unit)) = (string_field(&body, "unit"), string_field(&body, "subUnit"))
    else {
        return Ok(AxumJson(ActionResult::success(wrap_bool(false))));
    };
    let recursive = bool_field(&body, "recursive", true);
    let sql = if recursive {
        "WITH RECURSIVE sub AS (\
             SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = $1 OR name = $1)\
             UNION \
             SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) \
         SELECT EXISTS(SELECT 1 FROM x_org_unit s2 WHERE s2.deleted_at IS NULL \
             AND (s2.id = $2 OR s2.name = $2) AND s2.id IN (SELECT id FROM sub))"
    } else {
        "SELECT EXISTS(SELECT 1 FROM x_org_unit s2 \
         JOIN x_org_unit p ON p.id = s2.parent_id AND p.deleted_at IS NULL \
         WHERE s2.deleted_at IS NULL AND (s2.id = $2 OR s2.name = $2) \
         AND (p.id = $1 OR p.name = $1))"
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(sql, &[&unit, &sub_unit])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(AxumJson(ActionResult::success(wrap_bool(row.get::<_, bool>(0)))))
}

async fn types_query(
    types: Vec<String>,
    objects: bool,
    pool: Extension<Pool>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    capped(&types)?;
    if types.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = format!(
        "SELECT {} FROM x_org_unit WHERE deleted_at IS NULL AND \"type\" = ANY($1) ORDER BY level, id",
        if objects { UNIT_COLS_TYPE } else { "id" }
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, &[&types]).await.map_err(|_| AppError::Internal)?;
    finish_rows(rows, objects)
}

/// POST /jaxrs/unit/list/types (Java ActionListWithTypes，Wi{typeList})。
pub async fn unit_list_types(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    types_query(normalize_flags(string_list(&body, "typeList")), false, pool).await
}

/// POST /jaxrs/unit/list/types/object。
pub async fn unit_list_types_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    types_query(normalize_flags(string_list(&body, "typeList")), true, pool).await
}

/// GET /jaxrs/unit/list/type/{type}/object (Java ActionListWithTypeObject)。
pub async fn unit_list_type_type_object(
    pool: Extension<Pool>,
    Path(unit_type): Path<String>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    types_query(vec![unit_type], true, pool).await
}

/// POST /jaxrs/unit/list/unit/tree (Java ActionListWithUnitTree，Wi{unitList})：
/// 以种子组织为根的真实递归子树，内存装配嵌套 children 结构。
pub async fn unit_list_unit_tree(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "WITH RECURSIVE seeds AS (\
         SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))),\
         tree AS (\
             SELECT u.id, u.name, u.parent_id, u.level FROM x_org_unit u JOIN seeds ON u.id = seeds.id\
             UNION \
             SELECT c.id, c.name, c.parent_id, c.level FROM x_org_unit c \
             JOIN tree t ON c.parent_id = t.id WHERE c.deleted_at IS NULL)\
         SELECT id, name, parent_id, level FROM tree ORDER BY level, id";

    let flags = normalize_flags(string_list(&body, "unitList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;

    use std::collections::{HashMap, HashSet};
    let mut base: HashMap<String, Value> = HashMap::new();
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut root_ids: Vec<String> = Vec::new();
    for row in &rows {
        let id: String = row.get("id");
        let mut obj = row_to_map(row);
        if let Value::Object(ref mut m) = obj {
            m.insert("children".to_string(), Value::Array(vec![]));
        }
        base.insert(id, obj);
    }
    for row in &rows {
        let id: String = row.get("id");
        let parent: Option<String> = row.get("parent_id");
        match parent.as_deref() {
            Some(pid) if pid != id.as_str() && base.contains_key(pid) => {
                children_map.entry(pid.to_string()).or_default().push(id);
            }
            _ => root_ids.push(id),
        }
    }

    fn assemble(
        id: &str,
        base: &HashMap<String, Value>,
        children_map: &HashMap<String, Vec<String>>,
        seen: &mut HashSet<String>,
    ) -> Value {
        seen.insert(id.to_string());
        let mut node = base.get(id).cloned().unwrap_or(Value::Null);
        if let Some(kids) = children_map.get(id) {
            let mut built: Vec<Value> = Vec::new();
            for kid in kids {
                if !seen.contains(kid.as_str()) {
                    built.push(assemble(kid, base, children_map, seen));
                }
            }
            if let Value::Object(ref mut m) = node {
                m.insert("children".to_string(), Value::Array(built));
            }
        }
        node
    }

    let mut seen: HashSet<String> = HashSet::new();
    let mut roots: Vec<Value> = Vec::new();
    for id in &root_ids {
        if !seen.contains(id.as_str()) {
            roots.push(assemble(id, &base, &children_map, &mut seen));
        }
    }
    ok_json(count_data(roots.len(), roots))
}