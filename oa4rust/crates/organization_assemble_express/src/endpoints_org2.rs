//! plan002 U2 收尾 (part 5): identity 剩余 9 个端点。
//!
//! 对齐 Java IdentityAction 其余方法。x_org_identity.person_id/major 由迁移
//! 066 提供；群组维度经由 x_org_group_member(person) 关联解析。

use axum::{
    extract::{Extension, Json},
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::endpoints::{capped, count_data, normalize_flags, ok_json, row_to_map, string_list};

const IDENTITY_COLS: &str = "id, name, unit_id, person_id";

fn finish_identity_rows(
    rows: Vec<deadpool_postgres::tokio_postgres::Row>,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_json(count_data(data.len(), data))
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::success(named_list_response_key(
            &list,
        ))))
    }
}

fn named_list_response_key(list: &[String]) -> Value {
    Value::Object(serde_json::Map::from_iter([(
        "identityList".to_string(),
        Value::Array(list.iter().map(|s| Value::String(s.clone())).collect()),
    )]))
}

async fn identities_of_persons_full(
    pool: &Pool,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "personList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = format!(
        "SELECT DISTINCT i.id, i.name, i.unit_id, i.person_id FROM x_org_identity i \
         JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
         WHERE i.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY i.id"
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql_final = if objects {
        sql
    } else {
        format!(
            "SELECT DISTINCT i.id FROM x_org_identity i \
             JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
             WHERE i.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY i.id"
        )
    };
    let rows = client
        .query(&sql_final, &[&flags])
        .await
        .map_err(|_| AppError::Internal)?;
    finish_identity_rows(rows, objects)
}

/// POST /jaxrs/identity/list/unit/person (Java ActionListWithPersonWithUnit，
/// Wi{personList, unitList})：指定人员 ∩ 指定组织（直接所属）的身份。
pub async fn identity_list_unit_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_person_identities(&pool, body, false).await
}

/// POST /jaxrs/identity/list/unit/person/object。
pub async fn identity_list_unit_person_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    unit_person_identities(&pool, body, true).await
}

async fn unit_person_identities(
    pool: &Pool,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let persons = normalize_flags(string_list(&body, "personList"));
    let units = normalize_flags(string_list(&body, "unitList"));
    capped(&persons)?;
    capped(&units)?;
    if persons.is_empty() || units.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = if objects {
        "SELECT DISTINCT i.id, i.name, i.unit_id, i.person_id FROM x_org_identity i \
         JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
         JOIN x_org_unit u ON u.id = i.unit_id AND u.deleted_at IS NULL \
         WHERE i.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) \
         AND (u.id = ANY($2) OR u.name = ANY($2)) ORDER BY i.id"
    } else {
        "SELECT DISTINCT i.id FROM x_org_identity i \
         JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
         JOIN x_org_unit u ON u.id = i.unit_id AND u.deleted_at IS NULL \
         WHERE i.deleted_at IS NULL AND (p.id = ANY($1) OR p.name = ANY($1)) \
         AND (u.id = ANY($2) OR u.name = ANY($2)) ORDER BY i.id"
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(sql, &[&persons, &units])
        .await
        .map_err(|_| AppError::Internal)?;
    finish_identity_rows(rows, objects)
}

async fn identities_in_groups(
    pool: &Pool,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    // 群组维度：成员(person)→身份解析，含 parent_id 递归子群组（迁移 071）
    const SQL_IDS: &str = "WITH RECURSIVE gs AS (\
         SELECT id FROM x_org_group WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))\
         UNION \
         SELECT g.id FROM x_org_group g JOIN gs ON g.parent_id = gs.id WHERE g.deleted_at IS NULL)\
         SELECT DISTINCT i.id FROM x_org_identity i \
         JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
         JOIN x_org_group_member m ON m.person_id = p.id AND m.group_id IN (SELECT id FROM gs) \
         WHERE i.deleted_at IS NULL ORDER BY i.id";
    const SQL_OBJ: &str = "WITH RECURSIVE gs AS (\
         SELECT id FROM x_org_group WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))\
         UNION \
         SELECT g.id FROM x_org_group g JOIN gs ON g.parent_id = gs.id WHERE g.deleted_at IS NULL)\
         SELECT DISTINCT i.id, i.name, i.unit_id, i.person_id FROM x_org_identity i \
         JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
         JOIN x_org_group_member m ON m.person_id = p.id AND m.group_id IN (SELECT id FROM gs) \
         WHERE i.deleted_at IS NULL ORDER BY i.id";
    let flags = normalize_flags(string_list(&body, "groupList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = if objects { SQL_OBJ } else { SQL_IDS };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    finish_identity_rows(rows, objects)
}

/// POST /jaxrs/identity/list/group (Java ActionListWithGroup)：群组(含子群组)包含的身份。
pub async fn identity_list_group(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    identities_in_groups(&pool, body, false).await
}

/// POST /jaxrs/identity/list/group/object。
pub async fn identity_list_group_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    identities_in_groups(&pool, body, true).await
}

async fn major_identities_of_persons(
    pool: &Pool,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "personList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = if objects {
        format!(
            "SELECT DISTINCT i.id, i.name, i.unit_id, i.person_id FROM x_org_identity i \
             JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
             WHERE i.deleted_at IS NULL AND i.major AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY i.id"
        )
    } else {
        format!(
            "SELECT DISTINCT i.id FROM x_org_identity i \
             JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
             WHERE i.deleted_at IS NULL AND i.major AND (p.id = ANY($1) OR p.name = ANY($1)) ORDER BY i.id"
        )
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    finish_identity_rows(rows, objects)
}

/// POST /jaxrs/identity/list/major/person (Java ActionListMajorWithPerson)：主身份。
pub async fn identity_list_major_person(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    major_identities_of_persons(&pool, body, false).await
}

/// POST /jaxrs/identity/list/major/person/object。
pub async fn identity_list_major_person_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    major_identities_of_persons(&pool, body, true).await
}

/// POST /jaxrs/identity/list/person/object (Java ActionListWithPersonObject)。
pub async fn identity_list_person_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    identities_of_persons_full(&pool, body, true).await
}

/// POST /jaxrs/identity/list/unit/sub/direct/object。
pub async fn identity_list_unit_sub_direct_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const OBJ_SQL: &str = "SELECT DISTINCT i.id, i.name, i.unit_id, i.person_id FROM x_org_identity i \
         JOIN x_org_unit u ON u.id = i.unit_id AND u.deleted_at IS NULL \
         WHERE i.deleted_at IS NULL AND (u.id = ANY($1) OR u.name = ANY($1)) ORDER BY i.id";
    let flags = normalize_flags(string_list(&body, "unitList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(OBJ_SQL, &[&flags])
        .await
        .map_err(|_| AppError::Internal)?;
    finish_identity_rows(rows, true)
}

/// POST /jaxrs/identity/list/unit/sub/nested/object。
pub async fn identity_list_unit_sub_nested_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const OBJ_SQL: &str = "WITH RECURSIVE sub AS (\
         SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))\
         UNION \
         SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL)\
         SELECT DISTINCT i.id, i.name, i.unit_id, i.person_id FROM x_org_identity i \
         JOIN sub ON sub.id = i.unit_id WHERE i.deleted_at IS NULL ORDER BY i.id";
    let flags = normalize_flags(string_list(&body, "unitList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(OBJ_SQL, &[&flags])
        .await
        .map_err(|_| AppError::Internal)?;
    finish_identity_rows(rows, true)
}
