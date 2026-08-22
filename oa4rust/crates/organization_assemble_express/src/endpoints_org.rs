//! plan002 U2 (part 2): unit / group / role / unitduty express endpoints.
//! Conventions: see endpoints.rs module docs.

use axum::{
    extract::{Extension, Json},
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::endpoints::{capped, count_data, named_list_response, ok_json, string_list, PICK_ANY};

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
        return ok_json(count_data(0, vec![]));
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
    ok_json(count_data(data.len(), data))
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
    ok_json(count_data(list.len(), list))
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
    ok_json(count_data(data.len(), data))
}

async fn unit_tree_scope(
    pool: Extension<Pool>,
    body: Value,
    direction: &str,
    nested: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SUB_DIRECT_SQL: &str = "SELECT id FROM x_org_unit WHERE deleted_at IS NULL \
         AND parent_id IN (SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY id";
    const SUB_NESTED_SQL: &str = "WITH RECURSIVE sub AS (\
         SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
         UNION \
         SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL) \
         SELECT id FROM sub WHERE id NOT IN (SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY id";
    const SUP_DIRECT_SQL: &str = "WITH seeds AS (\
         SELECT id, parent_id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         SELECT DISTINCT u.id FROM x_org_unit u JOIN seeds s ON u.id = s.parent_id \
         WHERE u.deleted_at IS NULL ORDER BY u.id";
    const SUP_NESTED_SQL: &str = "WITH RECURSIVE sup AS (\
         SELECT id, parent_id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
         UNION \
         SELECT u.id, u.parent_id FROM x_org_unit u JOIN sup s ON s.parent_id = u.id WHERE u.deleted_at IS NULL) \
         SELECT id FROM sup WHERE id NOT IN (SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))) \
         ORDER BY id";

    let flags = string_list(&body, "unitList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = match (direction, nested) {
        ("sub", false) => SUB_DIRECT_SQL,
        ("sub", true) => SUB_NESTED_SQL,
        ("sup", false) => SUP_DIRECT_SQL,
        _ => SUP_NESTED_SQL,
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(sql, &[&flags])
        .await
        .map_err(|_| AppError::Internal)?;
    let list: Vec<Value> = rows
        .iter()
        .map(|r| Value::String(r.get::<_, String>("id")))
        .collect();
    ok_json(count_data(list.len(), list))
}

/// POST /jaxrs/unit/list/unit/sub/direct: direct child units.
pub async fn unit_list_unit_sub_direct(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sub", false).await
}

/// POST /jaxrs/unit/list/unit/sub/nested: recursive descendant units.
pub async fn unit_list_unit_sub_nested(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sub", true).await
}

/// POST /jaxrs/unit/list/unit/sup/direct: direct parent units.
pub async fn unit_list_unit_sup_direct(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sup", false).await
}

/// POST /jaxrs/unit/list/unit/sup/nested: recursive ancestor units.
pub async fn unit_list_unit_sup_nested(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_tree_scope(pool, body, "sup", true).await
}

/// POST /jaxrs/unit/check/unit/has/person: does each unit contain persons?
pub async fn unit_check_unit_has_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "unitList");
    capped(&flags)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut map = serde_json::Map::new();
    for flag in &flags {
        map.insert(flag.clone(), Value::Bool(false));
    }
    if !flags.is_empty() {
        let rows = client
            .query(
                "SELECT DISTINCT u.id FROM x_org_unit u \
                 JOIN x_org_person p ON p.unit_id = u.id AND p.deleted_at IS NULL \
                 WHERE u.deleted_at IS NULL AND (u.id = ANY($1) OR u.name = ANY($1))",
                &[&flags],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        for row in rows {
            let id: String = row.get("id");
            map.insert(id, Value::Bool(true));
        }
    }
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
        return ok_json(count_data(0, vec![]));
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
    ok_json(count_data(data.len(), data))
}

/// POST /jaxrs/group/list/object: batch group objects with member lists.
pub async fn group_list_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "groupList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
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
    ok_json(count_data(data.len(), data))
}

/// POST /jaxrs/group/list/person: persons contained in the given groups.
pub async fn group_list_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "groupList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
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
    ok_json(count_data(data.len(), data))
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
        return ok_json(count_data(0, vec![]));
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
    ok_json(count_data(data.len(), data))
}

/// POST /jaxrs/role/list/person: persons holding any of the given roles.
pub async fn role_list_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = string_list(&body, "roleList");
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
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
    ok_json(count_data(data.len(), data))
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
        return ok_json(count_data(0, vec![]));
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
    ok_json(count_data(data.len(), data))
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
