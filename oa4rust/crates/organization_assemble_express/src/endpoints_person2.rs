//! plan002 U2 收尾 (part 3): person 剩余 27 个端点。
//!
//! 对齐 Java x_organization_assemble_express PersonAction 其余方法：
//! login/after、login/recent、pair/identity、detail、各 object 变体、
//! person sub/sup direct/nested、unit sub/nested(+like)、personattribute 关联查询。
//! 约定沿用 endpoints.rs 模块注释；GET /jaxrs/person/{flag} 由 control crate
//! 占用（同 method+path 冲突），本模块不注册。
//!
//! 契约说明：
//! - lastLoginTime 语义由 x_org_login_record(login_time) 提供，date 参数以
//!   $N::timestamp 显式转换后参与比较，非法格式返回 400；
//! - 所有批量入口 capped(≤100) + normalize_flags 归一化查重。

use axum::{
    extract::{Extension, Json, Path},
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::endpoints::{
    bool_field, capped, count_data, named_list, normalize_flags, ok_json, resolve_person_ids,
    row_to_map, string_field, string_list, PICK_ANY,
};

// ── 登录相关（x_org_login_record 提供 lastLoginTime 语义） ───────────────────

const LOGIN_AFTER_IDS_SQL: &str = "SELECT DISTINCT p.id FROM x_org_person p \
     JOIN x_org_login_record lr ON lr.person_id = p.id \
     WHERE p.deleted_at IS NULL AND lr.login_time > $1::timestamp ORDER BY p.id";
const LOGIN_AFTER_OBJ_SQL: &str = "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
     JOIN x_org_login_record lr ON lr.person_id = p.id \
     WHERE p.deleted_at IS NULL AND lr.login_time > $1::timestamp ORDER BY p.id";

async fn login_after(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let Some(date) = string_field(&body, "date") else {
        // Java：date 为 null 时返回空 Wo.personList
        return ok_json(named_list("personList", &[]));
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = if objects { LOGIN_AFTER_OBJ_SQL } else { LOGIN_AFTER_IDS_SQL };
    let rows = match client.query(sql, &[&date]).await {
        Ok(rows) => rows,
        // 非法日期文本在 PG 侧触发 invalid_text_representation
        Err(e) if e.code() == Some(&deadpool_postgres::tokio_postgres::error::SqlState::INVALID_DATETIME_FORMAT) => {
            return Err(AppError::BadRequest(format!("invalid date: {date}")));
        }
        Err(_) => return Err(AppError::Internal),
    };
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_json(count_data(data.len(), data))
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::success(named_list(
            "personList",
            &list,
        ))))
    }
}

/// POST /jaxrs/person/list/login/after (Java ActionListLoginAfter)。
pub async fn person_list_login_after(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    login_after(pool, body, false).await
}

/// POST /jaxrs/person/list/login/after/object。
pub async fn person_list_login_after_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    login_after(pool, body, true).await
}

const LOGIN_RECENT_IDS_SQL: &str = "SELECT p.id, MAX(lr.login_time) AS last_login \
     FROM x_org_person p JOIN x_org_login_record lr ON lr.person_id = p.id \
     WHERE p.deleted_at IS NULL GROUP BY p.id ORDER BY last_login DESC";
const LOGIN_RECENT_OBJ_SQL: &str = "SELECT p.id, p.name, p.unit_id, MAX(lr.login_time) AS last_login \
     FROM x_org_person p JOIN x_org_login_record lr ON lr.person_id = p.id \
     WHERE p.deleted_at IS NULL GROUP BY p.id, p.name, p.unit_id ORDER BY last_login DESC";

async fn login_recent(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let count = body.get("count").and_then(|v| v.as_i64()).unwrap_or(0);
    let limit: Option<i64> = if count > 0 { Some(count.min(10_000)) } else { None };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let base = if objects { LOGIN_RECENT_OBJ_SQL } else { LOGIN_RECENT_IDS_SQL };
    // row_to_map 不识别 last_login 列时忽略；id 列恒在首位
    let sql = match limit {
        Some(_) => format!("{base} LIMIT $1"),
        None => base.to_string(),
    };
    let rows = if limit.is_some() {
        client
            .query(&sql, &[&limit])
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client.query(&sql, &[]).await.map_err(|_| AppError::Internal)?
    };
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_json(count_data(data.len(), data))
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::success(named_list(
            "personList",
            &list,
        ))))
    }
}

/// POST /jaxrs/person/list/login/recent (Java ActionListLoginRecent)。
pub async fn person_list_login_recent(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    login_recent(pool, body, false).await
}

/// POST /jaxrs/person/list/login/recent/object。
pub async fn person_list_login_recent_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    login_recent(pool, body, true).await
}

// ── pair / detail ─────────────────────────────────────────────────────────────

/// POST /jaxrs/person/list/pair/identity (Java ActionListPairIdentity)：
/// 输入顺序保持的 identity→person 配对，未命中 person 为 null。
pub async fn person_list_pair_identity(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "identityList"));
    capped(&flags)?;
    if flags.is_empty() {
        return Ok(AxumJson(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([(
                "identityPersonPairList".to_string(),
                Value::Array(vec![]),
            )]),
        ))));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            &format!(
                "SELECT id, name, person_id FROM x_org_identity \
                 WHERE deleted_at IS NULL AND {}",
                PICK_ANY
            ),
            &[&flags],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let pairs: Vec<Value> = flags
        .iter()
        .map(|flag| {
            let hit = rows.iter().find(|r| {
                r.get::<_, String>("id") == *flag || r.get::<_, String>("name") == *flag
            });
            let person = hit.and_then(|r| r.get::<_, Option<String>>("person_id"));
            let mut m = serde_json::Map::new();
            m.insert("identity".to_string(), Value::String(flag.clone()));
            m.insert(
                "person".to_string(),
                person.map(Value::String).unwrap_or(Value::Null),
            );
            Value::Object(m)
        })
        .collect();
    Ok(AxumJson(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([(
            "identityPersonPairList".to_string(),
            Value::Array(pairs),
        )]),
    ))))
}

/// POST /jaxrs/person/detail/{flag} (Java ActionDetail)：人员 + 身份/组织(含递归上级)/
/// 职务/群组(含递归上级)/角色/个人属性。fetch* 开关默认 true，显式 false 时清空对应列表。
pub async fn person_detail_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    body: Option<Json<Value>>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let fetch = |k: &str| {
        body.as_ref()
            .map(|Json(b)| bool_field(b, k, true))
            .unwrap_or(true)
    };
    let f_unit = fetch("fetchUnit");
    let f_duty = fetch("fetchUnitDuty");
    let f_group = fetch("fetchGroup");
    let f_role = fetch("fetchRole");
    let f_attr = fetch("fetchPersonAttribute");
    let f_id = fetch("fetchIdentity");

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let persons = resolve_person_ids(&client, std::slice::from_ref(&flag)).await?;
    let Some(pid) = persons.first().cloned() else {
        return ok_json(Value::Null);
    };

    let empty: Vec<String> = vec![];
    let need_identity_chain = f_id || f_unit || f_duty || f_group || f_role;
    let identity_ids: Vec<String> = if need_identity_chain {
        client
            .query(
                "SELECT id FROM x_org_identity WHERE deleted_at IS NULL AND person_id = $1 ORDER BY id",
                &[&pid],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .iter()
            .map(|r| r.get(0))
            .collect()
    } else {
        empty.clone()
    };

    let unit_ids: Vec<String> = if f_unit && !identity_ids.is_empty() {
        client
            .query(
                "WITH RECURSIVE sup AS (\
                     SELECT id, parent_id FROM x_org_unit \
                     WHERE deleted_at IS NULL AND id IN (\
                         SELECT DISTINCT unit_id FROM x_org_identity \
                         WHERE deleted_at IS NULL AND person_id = $1 AND unit_id IS NOT NULL)\
                     UNION \
                     SELECT u.id, u.parent_id FROM x_org_unit u JOIN sup s ON s.parent_id = u.id \
                     WHERE u.deleted_at IS NULL) \
                 SELECT id FROM sup ORDER BY id",
                &[&pid],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .iter()
            .map(|r| r.get(0))
            .collect()
    } else {
        empty.clone()
    };

    let duty_ids: Vec<String> = if f_duty && !identity_ids.is_empty() {
        client
            .query(
                "SELECT id FROM x_org_duty WHERE deleted_at IS NULL AND identity_id = ANY($1) ORDER BY id",
                &[&identity_ids],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .iter()
            .map(|r| r.get(0))
            .collect()
    } else {
        empty.clone()
    };

    let group_ids: Vec<String> = if f_group {
        client
            .query(
                "WITH RECURSIVE hit AS (\
                     SELECT g.id, g.parent_id FROM x_org_group g \
                     JOIN x_org_group_member m ON m.group_id = g.id \
                     WHERE g.deleted_at IS NULL AND m.person_id = $1\
                     UNION \
                     SELECT pg.id, pg.parent_id FROM x_org_group pg \
                     JOIN hit h ON h.parent_id = pg.id WHERE pg.deleted_at IS NULL) \
                 SELECT id FROM hit ORDER BY id",
                &[&pid],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .iter()
            .map(|r| r.get(0))
            .collect()
    } else {
        empty.clone()
    };

    let role_ids: Vec<String> = if f_role && !group_ids.is_empty() {
        client
            .query(
                "SELECT DISTINCT r.id FROM x_org_role r \
                 JOIN x_org_group_role gr ON gr.role_id = r.id \
                 WHERE r.deleted_at IS NULL AND gr.group_id = ANY($1) ORDER BY r.id",
                &[&group_ids],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .iter()
            .map(|r| r.get(0))
            .collect()
    } else {
        empty.clone()
    };

    let attr_ids: Vec<String> = if f_attr {
        client
            .query(
                "SELECT id FROM x_org_person_attribute WHERE deleted_at IS NULL AND person_id = $1 ORDER BY id",
                &[&pid],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .iter()
            .map(|r| r.get(0))
            .collect()
    } else {
        empty.clone()
    };

    let person_rows = client
        .query(
            "SELECT id, name, unit_id FROM x_org_person WHERE deleted_at IS NULL AND id = $1",
            &[&pid],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if person_rows.is_empty() {
        return ok_json(Value::Null);
    }
    let mut data = row_to_map(&person_rows[0]);
    let arr = |mut v: Vec<String>| {
        v.sort();
        v.dedup();
        Value::Array(v.into_iter().map(Value::String).collect())
    };
    if let Value::Object(ref mut m) = data {
        m.insert("distinguishedName".to_string(), Value::String(pid));
        m.insert("identityList".to_string(), if f_id { arr(identity_ids) } else { Value::Array(vec![]) });
        m.insert("unitList".to_string(), if f_unit { arr(unit_ids) } else { Value::Array(vec![]) });
        m.insert("unitDutyList".to_string(), if f_duty { arr(duty_ids) } else { Value::Array(vec![]) });
        m.insert("groupList".to_string(), if f_group { arr(group_ids) } else { Value::Array(vec![]) });
        m.insert("roleList".to_string(), if f_role { arr(role_ids) } else { Value::Array(vec![]) });
        m.insert(
            "personAttributeList".to_string(),
            if f_attr { arr(attr_ids) } else { Value::Array(vec![]) },
        );
    }
    ok_json(data)
}

// ── group / identity / role 的 object 变体 ────────────────────────────────────

/// POST /jaxrs/person/list/group/object (Java ActionListWithGroupObject)。
pub async fn person_list_group_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
         JOIN x_org_group_member m ON m.person_id = p.id \
         JOIN x_org_group g ON g.id = m.group_id AND g.deleted_at IS NULL \
         WHERE p.deleted_at IS NULL AND (g.id = ANY($1) OR g.name = ANY($1)) ORDER BY p.id";
    person_flag_objects(pool, body, "groupList", SQL).await
}

/// POST /jaxrs/person/list/identity/object (Java ActionListWithIdentityObject)。
pub async fn person_list_identity_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
         JOIN x_org_identity i ON i.person_id = p.id AND i.deleted_at IS NULL \
         WHERE p.deleted_at IS NULL AND (i.id = ANY($1) OR i.name = ANY($1)) ORDER BY p.id";
    person_flag_objects(pool, body, "identityList", SQL).await
}

/// POST /jaxrs/person/list/role/object (Java ActionListWithRoleObject)。
pub async fn person_list_role_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
         JOIN x_org_group_member m ON m.person_id = p.id \
         JOIN x_org_group_role gr ON gr.group_id = m.group_id \
         JOIN x_org_role r ON r.id = gr.role_id AND r.deleted_at IS NULL \
         WHERE p.deleted_at IS NULL AND (r.id = ANY($1) OR r.name = ANY($1)) ORDER BY p.id";
    person_flag_objects(pool, body, "roleList", SQL).await
}

async fn person_flag_objects(
    pool: Extension<Pool>,
    body: Value,
    key: &str,
    sql: &str,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, key));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(sql, &[&flags])
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_json(count_data(data.len(), data))
}

// ── personattribute 关联查询 ──────────────────────────────────────────────────

const ATTR_PERSON_BASE: &str = "FROM x_org_person p \
     JOIN x_org_person_attribute a ON a.person_id = p.id AND a.deleted_at IS NULL \
     WHERE p.deleted_at IS NULL AND a.attribute_key = $1 \
       AND ($2::text IS NULL OR a.attribute_value = $2)";

/// POST /jaxrs/person/list/personattribute (Java ActionListWithPersonAttribute，
/// Wi{name, attribute})：拥有指定属性名(与可选属性值)的人员。
pub async fn person_list_personattribute(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    attr_persons(pool, body, false).await
}

/// POST /jaxrs/person/list/personattribute/object。
pub async fn person_list_personattribute_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    attr_persons(pool, body, true).await
}

async fn attr_persons(
    pool: Extension<Pool>,
    body: Value,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let Some(name) = string_field(&body, "name") else {
        // Java：name 为空直接返回空结果
        return ok_json(named_list("personList", &[]));
    };
    let attribute = string_field(&body, "attribute");
    let select = if objects {
        "SELECT DISTINCT p.id, p.name, p.unit_id"
    } else {
        "SELECT DISTINCT p.id"
    };
    let sql = format!("{select} {ATTR_PERSON_BASE} ORDER BY p.id");
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(&sql, &[&name, &attribute])
        .await
        .map_err(|_| AppError::Internal)?;
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_json(count_data(data.len(), data))
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::success(named_list(
            "personList",
            &list,
        ))))
    }
}

// ── 个人的下级/上级（经由身份所在组织的树形关系） ─────────────────────────────

/// 种子：给定人员的身份所在组织。
const SEED_UNITS_OF_PERSONS: &str = "SELECT DISTINCT i.unit_id FROM x_org_identity i \
     JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
     WHERE i.deleted_at IS NULL AND i.unit_id IS NOT NULL AND (p.id = ANY($1) OR p.name = ANY($1))";

const PERSON_SUB_DIRECT_TPL: &str = "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
     JOIN x_org_unit u ON u.id = p.unit_id AND u.deleted_at IS NULL \
     WHERE p.deleted_at IS NULL AND u.parent_id IN ({SEED}) ORDER BY p.id";

const PERSON_SUB_NESTED_TPL: &str = "WITH RECURSIVE seeds AS ({SEED}), \
     sub AS (SELECT u.id FROM x_org_unit u JOIN seeds s ON u.parent_id = s.id WHERE u.deleted_at IS NULL \
             UNION \
             SELECT u2.id FROM x_org_unit u2 JOIN sub ON u2.parent_id = sub.id WHERE u2.deleted_at IS NULL) \
     SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
     JOIN sub ON sub.id = p.unit_id WHERE p.deleted_at IS NULL ORDER BY p.id";

const PERSON_SUP_DIRECT_TPL: &str = "WITH seeds AS ({SEED}), \
     pars AS (SELECT par.parent_id AS pid FROM x_org_unit par \
              JOIN seeds s ON s.unit_id = par.id AND par.parent_id IS NOT NULL \
              WHERE par.deleted_at IS NULL) \
     SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
     JOIN x_org_unit u ON u.id = p.unit_id AND u.deleted_at IS NULL \
     WHERE p.deleted_at IS NULL AND u.id IN (SELECT pid FROM pars) ORDER BY p.id";

const PERSON_SUP_NESTED_TPL: &str = "WITH RECURSIVE seeds AS ({SEED}), \
     sup AS (SELECT u.id, u.parent_id FROM x_org_unit u JOIN seeds s ON u.parent_id = s.unit_id \
             WHERE u.deleted_at IS NULL \
             UNION \
             SELECT pu.id, pu.parent_id FROM x_org_unit pu JOIN sup ON sup.parent_id = pu.id \
             WHERE pu.deleted_at IS NULL) \
     SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
     JOIN sup ON sup.id = p.unit_id WHERE p.deleted_at IS NULL ORDER BY p.id";

macro_rules! person_tree_endpoint {
    ($fn_name:ident, $tpl:expr, $objects:expr) => {
        pub async fn $fn_name(
            pool: Extension<Pool>,
            Json(body): Json<Value>,
        ) -> Result<AxumJson<ActionResult<Value>>, AppError> {
            person_tree_scope(pool, body, $tpl, $objects).await
        }
    };
}

async fn person_tree_scope(
    pool: Extension<Pool>,
    body: Value,
    template: &str,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "personList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let sql = template.replace("{SEED}", SEED_UNITS_OF_PERSONS);
    let select_sql = if objects {
        sql
    } else {
        sql.replacen(
            "SELECT DISTINCT p.id, p.name, p.unit_id",
            "SELECT DISTINCT p.id",
            1,
        )
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(&select_sql, &[&flags])
        .await
        .map_err(|_| AppError::Internal)?;
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_json(count_data(data.len(), data))
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::success(named_list(
            "personList",
            &list,
        ))))
    }
}

person_tree_endpoint!(person_list_person_sub_direct, PERSON_SUB_DIRECT_TPL, false);
person_tree_endpoint!(person_list_person_sub_direct_object, PERSON_SUB_DIRECT_TPL, true);
person_tree_endpoint!(person_list_person_sub_nested, PERSON_SUB_NESTED_TPL, false);
person_tree_endpoint!(person_list_person_sub_nested_object, PERSON_SUB_NESTED_TPL, true);
person_tree_endpoint!(person_list_person_sup_direct, PERSON_SUP_DIRECT_TPL, false);
person_tree_endpoint!(person_list_person_sup_direct_object, PERSON_SUP_DIRECT_TPL, true);
person_tree_endpoint!(person_list_person_sup_nested, PERSON_SUP_NESTED_TPL, false);
person_tree_endpoint!(person_list_person_sup_nested_object, PERSON_SUP_NESTED_TPL, true);

// ── 组织成员（direct / nested / like × id/object） ────────────────────────────

const UNIT_SEEDS_SQL: &str =
    "SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))";

const UNIT_SUB_DIRECT_PERSONS_TPL: &str = "SELECT DISTINCT p.id, p.name, p.unit_id \
     FROM x_org_person p JOIN x_org_unit u ON u.id = p.unit_id AND u.deleted_at IS NULL \
     WHERE p.deleted_at IS NULL AND u.parent_id IN ({SEED}){FILTER} ORDER BY p.id";

const UNIT_SUB_NESTED_PERSONS_TPL: &str = "WITH RECURSIVE sub AS ({RECURSIVE_SEEDS}) \
     SELECT DISTINCT p.id, p.name, p.unit_id \
     FROM x_org_person p JOIN x_org_unit u ON u.id = p.unit_id AND u.deleted_at IS NULL \
     WHERE p.deleted_at IS NULL AND u.id IN (SELECT id FROM sub){FILTER} ORDER BY p.id";

const UNIT_RECURSIVE_SEEDS_SQL: &str = "\
     SELECT id FROM x_org_unit WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1)) \
     UNION \
     SELECT u.id FROM x_org_unit u JOIN sub s ON u.parent_id = s.id WHERE u.deleted_at IS NULL";

macro_rules! person_unit_endpoint {
    ($fn_name:ident, $tpl:expr, $like:expr, $objects:expr) => {
        pub async fn $fn_name(
            pool: Extension<Pool>,
            Json(body): Json<Value>,
        ) -> Result<AxumJson<ActionResult<Value>>, AppError> {
            person_of_units(pool, body, $tpl, $like, $objects).await
        }
    };
}

async fn person_of_units(
    pool: Extension<Pool>,
    body: Value,
    template: &str,
    like: bool,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "unitList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let key = if like { string_field(&body, "key") } else { None };
    let filter = if like {
        " AND ($2::text IS NULL OR p.name ILIKE '%' || $2 || '%')"
    } else {
        ""
    };
    let sql = template
        .replace("{RECURSIVE_SEEDS}", UNIT_RECURSIVE_SEEDS_SQL)
        .replace("{SEED}", UNIT_SEEDS_SQL)
        .replace("{FILTER}", filter);
    let final_sql = if objects {
        sql
    } else {
        sql.replacen(
            "SELECT DISTINCT p.id, p.name, p.unit_id",
            "SELECT DISTINCT p.id",
            1,
        )
    };
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = if like {
        client
            .query(&final_sql, &[&flags, &key])
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(&final_sql, &[&flags])
            .await
            .map_err(|_| AppError::Internal)?
    };
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_json(count_data(data.len(), data))
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::success(named_list(
            "personList",
            &list,
        ))))
    }
}

person_unit_endpoint!(person_list_unit_sub_direct, UNIT_SUB_DIRECT_PERSONS_TPL, false, false);
person_unit_endpoint!(
    person_list_unit_sub_direct_object,
    UNIT_SUB_DIRECT_PERSONS_TPL,
    false,
    true
);
person_unit_endpoint!(person_list_unit_sub_nested, UNIT_SUB_NESTED_PERSONS_TPL, false, false);
person_unit_endpoint!(
    person_list_unit_sub_nested_object,
    UNIT_SUB_NESTED_PERSONS_TPL,
    false,
    true
);
person_unit_endpoint!(person_list_unit_sub_direct_like, UNIT_SUB_DIRECT_PERSONS_TPL, true, false);
person_unit_endpoint!(
    person_list_unit_sub_direct_like_object,
    UNIT_SUB_DIRECT_PERSONS_TPL,
    true,
    true
);
person_unit_endpoint!(person_list_unit_sub_nested_like, UNIT_SUB_NESTED_PERSONS_TPL, true, false);
person_unit_endpoint!(
    person_list_unit_sub_nested_like_object,
    UNIT_SUB_NESTED_PERSONS_TPL,
    true,
    true
);
