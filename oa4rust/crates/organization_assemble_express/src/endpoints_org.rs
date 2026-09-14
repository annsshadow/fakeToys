//! plan002 U2 (part 2): unit / group / role / unitduty express endpoints.
//! Conventions: see endpoints.rs module docs.

use axum::{
    extract::{Extension, Json},
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::endpoints::{capped, named_list_response, ok_java_list, string_list, PICK_ANY};

/// POST /jaxrs/unit/list: batch unit lookup (Java UnitAction#list; GET variant is control's).
pub async fn unit_list(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_batch(pool, body).await
}

/// POST /jaxrs/unit/list/object: batch unit objects.
pub async fn unit_list_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_batch(pool, body).await
}

async fn unit_batch(
    pool: Extension<Pool>,
    body: Value,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "unitList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT id, name, parent_id, level FROM x_org_unit \
                 WHERE deleted_at IS NULL AND {} ORDER BY level, id",
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(crate::endpoints::row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// GET /jaxrs/unit/list/all: all unit ids.
pub async fn unit_list_all(
    pool: Extension<Pool>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id FROM x_org_unit WHERE deleted_at IS NULL ORDER BY level, id",
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

/// GET /jaxrs/unit/list/all/object: all unit objects.
pub async fn unit_list_all_object(
    pool: Extension<Pool>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, level FROM x_org_unit WHERE deleted_at IS NULL ORDER BY level, id",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(crate::endpoints::row_to_map).collect();
    crate::endpoints::ok_java_list(data.len(), data)
}

async fn unit_tree_scope(
    pool: Extension<Pool>,
    body: Value,
    direction: &str,
    nested: bool,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SUB_DIRECT_SQL: &str = "SELECT id FROM x_org_unit WHERE deleted_at IS NULL \
         AND parent_id IN (SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY id";
    const SUB_DIRECT_OBJ_SQL: &str = "SELECT id, name, parent_id, level FROM x_org_unit WHERE deleted_at IS NULL \
         AND parent_id IN (SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY id";
    const SUB_NESTED_SQL: &str = "WITH RECURSIVE sub AS (\
         SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
         UNION \
         SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) \
         SELECT id FROM sub WHERE id NOT IN (SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY id";
    const SUB_NESTED_OBJ_SQL: &str = "WITH RECURSIVE sub AS (\
         SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
         UNION \
         SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) \
         SELECT u.id, u.name, u.parent_id, u.level FROM x_org_unit u \
         WHERE u.deleted_at IS NULL AND u.id IN (SELECT id FROM sub) AND u.id NOT IN (\
             SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY u.id";
    const SUP_DIRECT_SQL: &str = "WITH seeds AS (\
         SELECT id, parent_id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         SELECT DISTINCT u.id FROM x_org_unit u JOIN seeds s ON u.id = s.parent_id \
         WHERE u.deleted_at IS NULL ORDER BY u.id";
    const SUP_DIRECT_OBJ_SQL: &str = "WITH seeds AS (\
         SELECT id, parent_id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         SELECT DISTINCT u.id, u.name, u.parent_id, u.level FROM x_org_unit u JOIN seeds s ON u.id = s.parent_id \
         WHERE u.deleted_at IS NULL ORDER BY u.id";
    const SUP_NESTED_SQL: &str = "WITH RECURSIVE sup AS (\
         SELECT id, parent_id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
         UNION \
         SELECT u.id, u.parent_id FROM x_org_unit u JOIN sup s ON s.parent_id = u.id WHERE u.deleted_at IS NULL) \
         SELECT id FROM sup WHERE id NOT IN (SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY id";
    const SUP_NESTED_OBJ_SQL: &str = "WITH RECURSIVE sup AS (\
         SELECT id, parent_id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
         UNION \
         SELECT u.id, u.parent_id FROM x_org_unit u JOIN sup s ON s.parent_id = u.id WHERE u.deleted_at IS NULL) \
         SELECT uu.id, uu.name, uu.parent_id, uu.level FROM x_org_unit uu \
         WHERE uu.deleted_at IS NULL AND uu.id IN (SELECT id FROM sup) AND uu.id NOT IN (\
             SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY uu.id";

    let flags = string_list(&body, "unitList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let sql = match (direction, nested, objects) {
        ("sub", false, false) => SUB_DIRECT_SQL,
        ("sub", false, true) => SUB_DIRECT_OBJ_SQL,
        ("sub", true, false) => SUB_NESTED_SQL,
        ("sub", true, true) => SUB_NESTED_OBJ_SQL,
        ("sup", false, false) => SUP_DIRECT_SQL,
        ("sup", false, true) => SUP_DIRECT_OBJ_SQL,
        ("sup", true, false) => SUP_NESTED_SQL,
        (_, _, _) => SUP_NESTED_OBJ_SQL,
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(sql, &[&flags])
        .await
        .map_err(|_| AppError::Internal)?;
    if objects {
        let data: Vec<Value> = rows.iter().map(crate::endpoints::row_to_map).collect();
        ok_java_list(data.len(), data)
    } else {
        let list: Vec<Value> = rows
            .iter()
            .map(|r| Value::String(r.get::<_, String>("id")))
            .collect();
        ok_java_list(list.len(), list)
    }
}

/// POST /jaxrs/unit/list/unit/sub/direct: direct child units.
pub async fn unit_list_unit_sub_direct(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sub", false, false).await
}

/// POST /jaxrs/unit/list/unit/sub/nested: recursive descendant units.
pub async fn unit_list_unit_sub_nested(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sub", true, false).await
}

/// POST /jaxrs/unit/list/unit/sup/direct: direct parent units.
pub async fn unit_list_unit_sup_direct(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sup", false, false).await
}

/// POST /jaxrs/unit/list/unit/sup/nested: recursive ancestor units.
pub async fn unit_list_unit_sup_nested(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sup", true, false).await
}

/// POST /jaxrs/unit/list/unit/sub/direct/object (Java ActionListWithUnitSubDirectObject)。
pub async fn unit_list_unit_sub_direct_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sub", false, true).await
}

/// POST /jaxrs/unit/list/unit/sub/nested/object。
pub async fn unit_list_unit_sub_nested_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sub", true, true).await
}

/// POST /jaxrs/unit/list/unit/sup/direct/object。
pub async fn unit_list_unit_sup_direct_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sup", false, true).await
}

/// POST /jaxrs/unit/list/unit/sup/nested/object。
pub async fn unit_list_unit_sup_nested_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sup", true, true).await
}

/// POST /jaxrs/unit/check/unit/has/person: does the person hold an identity
/// within the given unit? (Java UnitAction#checkHasPerson → ActionHasPerson)
///
/// Wi = {person, unit, recursive(default true)}; Wo = WrapBoolean →
/// `data: {value: <bool>}`. person/unit 均可按 id 或名称定位；基础判定 =
/// 该人身份组织的"自身 + 子孙"集合包含目标组织；recursive 时再查目标组织
/// 是否处于任一身份组织的祖先链（sup-nested）。空载荷或查无对象 → false。
pub async fn unit_check_unit_has_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let person = body.get("person").and_then(Value::as_str).unwrap_or("").trim();
    let unit = body.get("unit").and_then(Value::as_str).unwrap_or("").trim();
    let recursive = body.get("recursive").and_then(Value::as_bool).unwrap_or(true);

    let mut value = false;
    if !person.is_empty() && !unit.is_empty() {
        let client = pool.get().await.map_err(|_| AppError::Internal)?;
        let person_row = client
            .query_opt(
                "SELECT id FROM x_org_person WHERE deleted_at IS NULL AND (id = $1 OR name = $1)",
                &[&person, &person],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        let target_row = client
            .query_opt(
                "SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = $1 OR name = $1)",
                &[&unit, &unit],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if let (Some(prow), Some(trow)) = (person_row, target_row) {
            let person_id: String = prow.get("id");
            let target_id: String = trow.get("id");
            let seed_rows = client
                .query(
                    "SELECT DISTINCT unit_id FROM x_org_identity \
                     WHERE person_id = $1 AND deleted_at IS NULL AND unit_id IS NOT NULL",
                    &[&person_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            let seeds: Vec<String> =
                seed_rows.iter().map(|r| r.get::<_, String>("unit_id")).collect();
            if !seeds.is_empty() {
                // 基础判定：身份组织的自身 + 全部子孙组织
                let sub_rows = client
                    .query(
                        "WITH RECURSIVE sub AS (\
                         SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
                         UNION \
                         SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) \
                         SELECT id FROM sub",
                        &[&seeds],
                    )
                    .await
                    .map_err(|_| AppError::Internal)?;
                value = sub_rows.iter().any(|r| r.get::<_, String>("id") == target_id);
                // recursive 判定：目标组织位于任一身份组织的祖先链（sup-nested）
                if !value && recursive {
                    let sup_rows = client
                        .query(
                            "WITH RECURSIVE sup AS (\
                             SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
                             UNION \
                             SELECT u.id FROM x_org_unit u JOIN sup s ON u.id = s.parent_id WHERE u.deleted_at IS NULL) \
                             SELECT id FROM sup",
                            &[&seeds],
                        )
                        .await
                        .map_err(|_| AppError::Internal)?;
                    value = sup_rows.iter().any(|r| r.get::<_, String>("id") == target_id);
                }
            }
        }
    }
    let mut map = serde_json::Map::new();
    map.insert("value".to_string(), Value::Bool(value));
    Ok(AxumJson(ActionResult::success(Value::Object(map))))
}

// ── Group ─────────────────────────────────────────────────────────────────────

/// POST /jaxrs/group/list: batch group lookup.
pub async fn group_list(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "groupList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT id, name FROM x_org_group WHERE deleted_at IS NULL AND {} ORDER BY id",
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(crate::endpoints::row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/group/list/object: batch group objects with member lists.
pub async fn group_list_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "groupList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT g.id, g.name, \"type\", unit_id, \
                 (SELECT COUNT(*) FROM x_org_group_member m WHERE m.group_id = g.id) AS member_count \
                 FROM x_org_group g WHERE g.deleted_at IS NULL AND {} ORDER BY g.id",
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    for row in &rows {
        let mut obj = crate::endpoints::row_to_map(row);
        let gid: String = row.get("id");
        let member_rows = client
            .query(
                "SELECT person_id FROM x_org_group_member WHERE group_id = $1 ORDER BY person_id",
                &[&gid],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if let Value::Object(ref mut map) = obj {
            let members: Vec<Value> = member_rows
                .iter()
                .map(|m| Value::String(m.get::<_, String>("person_id")))
                .collect();
            map.insert("personList".to_string(), Value::Array(members));
        }
        data.push(obj);
    }
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/group/list/person: persons contained in the given groups.
pub async fn group_list_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "groupList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
             JOIN x_org_group_member m ON m.person_id = p.id \
             JOIN x_org_group g ON g.id = m.group_id AND g.deleted_at IS NULL \
             WHERE p.deleted_at IS NULL AND (g.id = ANY($1) OR g.name = ANY($1)) ORDER BY p.id",
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(crate::endpoints::row_to_map).collect();
    ok_java_list(data.len(), data)
}

// ── Role ──────────────────────────────────────────────────────────────────────

/// POST /jaxrs/role/list: batch role lookup.
pub async fn role_list(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "roleList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT id, name, description FROM x_org_role WHERE deleted_at IS NULL AND {} ORDER BY id",
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(crate::endpoints::row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/role/list/person: persons holding any of the given roles.
pub async fn role_list_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "roleList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
             JOIN x_org_group_member m ON m.person_id = p.id \
             JOIN x_org_group_role gr ON gr.group_id = m.group_id \
             JOIN x_org_role r ON r.id = gr.role_id AND r.deleted_at IS NULL \
             WHERE p.deleted_at IS NULL AND (r.id = ANY($1) OR r.name = ANY($1)) ORDER BY p.id",
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(crate::endpoints::row_to_map).collect();
    ok_java_list(data.len(), data)
}

// ── UnitDuty ──────────────────────────────────────────────────────────────────

/// POST /jaxrs/unitduty/list/name: batch duty lookup by duty names (Java Wi nameList).
pub async fn unitduty_list_name(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "nameList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_java_list(0, vec![]);
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT id, name, unit_id, identity_id FROM x_org_duty \
                 WHERE deleted_at IS NULL AND {} ORDER BY id",
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(crate::endpoints::row_to_map).collect();
    ok_java_list(data.len(), data)
}

/// POST /jaxrs/unitduty/list/name/unit: distinct duty names held in units.
pub async fn unitduty_list_name_unit(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT d.name FROM x_org_duty d \
         JOIN x_org_unit u ON u.id = d.unit_id AND u.deleted_at IS NULL \
         WHERE d.deleted_at IS NULL AND (u.id = ANY($1) OR u.name = ANY($1)) ORDER BY d.name";
    let flags = string_list(&body, "unitList");
    capped(&flags)?;
    named_list_response(&pool, "dutyNameList", SQL, &flags).await
}
