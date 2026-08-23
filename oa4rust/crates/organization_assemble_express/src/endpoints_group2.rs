//! plan002 U2 收尾 (part 6): group 剩余 13 个端点。
//!
//! 对齐 Java GroupAction 其余方法。群组层级依赖迁移 071 提供的
//! x_org_group.parent_id（o2 Group.groupList 的关系化表达）。

use axum::{
    extract::{Extension, Json},
    Json as AxumJson,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

use crate::endpoints::{capped, count_data, normalize_flags, ok_json, row_to_map, string_field, string_list, wrap_bool, PICK_ANY};

const GROUP_COLS: &str = "g.id, g.name, g.parent_id, \"type\", g.unit_id";

fn finish_group_rows(
    rows: Vec<deadpool_postgres::tokio_postgres::Row>,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    if objects {
        let data: Vec<Value> = rows.iter().map(row_to_map).collect();
        ok_json(count_data(data.len(), data))
    } else {
        let list: Vec<String> = rows.iter().map(|r| r.get(0)).collect();
        Ok(AxumJson(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([(
                "groupList".to_string(),
                Value::Array(list.into_iter().map(Value::String).collect()),
            )]),
        ))))
    }
}

/// POST /jaxrs/group/has/role (Java ActionHasRole，Wi{group, roleList})：
/// 群组是否拥有指定角色之一（WrapBoolean）。
pub async fn group_has_role(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let Some(group) = string_field(&body, "group") else {
        return Ok(AxumJson(ActionResult::success(wrap_bool(false))));
    };
    let roles = normalize_flags(string_list(&body, "roleList"));
    capped(&roles)?;
    if roles.is_empty() {
        return Ok(AxumJson(ActionResult::success(wrap_bool(false))));
    }
    const SQL: &str = "SELECT EXISTS(SELECT 1 FROM x_org_group_role gr \
         JOIN x_org_group g ON g.id = gr.group_id AND g.deleted_at IS NULL \
         JOIN x_org_role r ON r.id = gr.role_id AND r.deleted_at IS NULL \
         WHERE (g.id = $1 OR g.name = $1) AND (r.id = ANY($2) OR r.name = ANY($2)))";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(SQL, &[&group, &roles])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(AxumJson(ActionResult::success(wrap_bool(row.get::<_, bool>(0)))))
}

/// sub/sup × direct/nested × id/object 共用实现。
async fn group_tree_scope(
    pool: Extension<Pool>,
    body: Value,
    direction: &str,
    nested: bool,
    objects: bool,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "groupList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let scope_sql = match (direction, nested) {
        ("sub", false) => {
            "SELECT c.id FROM x_org_group c WHERE c.deleted_at IS NULL AND c.parent_id IN (\
             SELECT id FROM x_org_group WHERE deleted_at IS NULL AND PICK_FLAGS)"
                .replace("PICK_FLAGS", PICK_ANY)
        }
        ("sub", true) => {
            "WITH RECURSIVE sub AS (\
             SELECT id FROM x_org_group WHERE deleted_at IS NULL AND PICK_FLAGS\
             UNION \
             SELECT g.id FROM x_org_group g JOIN sub s ON g.parent_id = s.id WHERE g.deleted_at IS NULL)\
             SELECT id FROM sub WHERE id NOT IN (\
             SELECT id FROM x_org_group WHERE deleted_at IS NULL AND PICK_FLAGS)"
                .replace("PICK_FLAGS", PICK_ANY)
        }
        ("sup", false) => {
            "SELECT p.id FROM x_org_group p \
             JOIN x_org_group s ON s.parent_id = p.id AND s.deleted_at IS NULL \
             WHERE p.deleted_at IS NULL AND PICK_FLAGS_S"
                .replace("PICK_FLAGS_S", "(s.id = ANY($1) OR s.name = ANY($1))")
        }
        _ => {
            "WITH RECURSIVE sup AS (\
             SELECT id, parent_id FROM x_org_group WHERE deleted_at IS NULL AND PICK_FLAGS\
             UNION \
             SELECT p.id, p.parent_id FROM x_org_group p JOIN sup s ON s.parent_id = p.id \
             WHERE p.deleted_at IS NULL)\
             SELECT id FROM sup WHERE id NOT IN (\
             SELECT id FROM x_org_group WHERE deleted_at IS NULL AND PICK_FLAGS)"
                .replace("PICK_FLAGS", PICK_ANY)
        }
    };
    let sql = format!(
        "SELECT {} FROM x_org_group g WHERE g.deleted_at IS NULL \
         AND g.id IN ({}) ORDER BY g.id",
        if objects { GROUP_COLS } else { "g.id" },
        scope_sql
    );
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(&sql, &[&flags]).await.map_err(|_| AppError::Internal)?;
    finish_group_rows(rows, objects)
}

macro_rules! group_tree_endpoint {
    ($fn_name:ident, $dir:expr, $nested:expr, $objects:expr) => {
        pub async fn $fn_name(
            pool: Extension<Pool>,
            Json(body): Json<Value>,
        ) -> Result<AxumJson<ActionResult<Value>>, AppError> {
            group_tree_scope(pool, body, $dir, $nested, $objects).await
        }
    };
}

/// POST /jaxrs/group/list/group/sub/direct。
group_tree_endpoint!(group_list_group_sub_direct, "sub", false, false);
/// POST /jaxrs/group/list/group/sub/direct/object。
group_tree_endpoint!(group_list_group_sub_direct_object, "sub", false, true);
/// POST /jaxrs/group/list/group/sub/nested。
group_tree_endpoint!(group_list_group_sub_nested, "sub", true, false);
/// POST /jaxrs/group/list/group/sub/nested/object。
group_tree_endpoint!(group_list_group_sub_nested_object, "sub", true, true);
/// POST /jaxrs/group/list/group/sup/direct。
group_tree_endpoint!(group_list_group_sup_direct, "sup", false, false);
/// POST /jaxrs/group/list/group/sup/direct/object。
group_tree_endpoint!(group_list_group_sup_direct_object, "sup", false, true);
/// POST /jaxrs/group/list/group/sup/nested。
group_tree_endpoint!(group_list_group_sup_nested, "sup", true, false);
/// POST /jaxrs/group/list/group/sup/nested/object。
group_tree_endpoint!(group_list_group_sup_nested_object, "sup", true, true);

/// POST /jaxrs/group/list/person/object (Java ActionListWithPersonObject)。
pub async fn group_list_person_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let flags = normalize_flags(string_list(&body, "groupList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    const SQL: &str = "SELECT DISTINCT p.id, p.name, p.unit_id FROM x_org_person p \
         JOIN x_org_group_member m ON m.person_id = p.id \
         JOIN x_org_group g ON g.id = m.group_id AND g.deleted_at IS NULL \
         WHERE p.deleted_at IS NULL AND (g.id = ANY($1) OR g.name = ANY($1)) ORDER BY p.id";
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_json(count_data(data.len(), data))
}

/// POST /jaxrs/group/list/identity (Java ActionListWithIdentity)：成员(person)→身份解析。
pub async fn group_list_identity(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT i.id FROM x_org_identity i \
         JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
         JOIN x_org_group_member m ON m.person_id = p.id \
         JOIN x_org_group g ON g.id = m.group_id AND g.deleted_at IS NULL \
         WHERE i.deleted_at IS NULL AND (g.id = ANY($1) OR g.name = ANY($1)) ORDER BY i.id";
    let flags = normalize_flags(string_list(&body, "groupList"));
    capped(&flags)?;
    named_list_response_group(pool, "identityList", SQL, flags).await
}

/// POST /jaxrs/group/list/identity/object。
pub async fn group_list_identity_object(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "SELECT DISTINCT i.id, i.name, i.unit_id, i.person_id FROM x_org_identity i \
         JOIN x_org_person p ON p.id = i.person_id AND p.deleted_at IS NULL \
         JOIN x_org_group_member m ON m.person_id = p.id \
         JOIN x_org_group g ON g.id = m.group_id AND g.deleted_at IS NULL \
         WHERE i.deleted_at IS NULL AND (g.id = ANY($1) OR g.name = ANY($1)) ORDER BY i.id";
    let flags = normalize_flags(string_list(&body, "groupList"));
    capped(&flags)?;
    if flags.is_empty() {
        return ok_json(count_data(0, vec![]));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query(SQL, &[&flags]).await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(row_to_map).collect();
    ok_json(count_data(data.len(), data))
}

async fn named_list_response_group(
    pool: Extension<Pool>,
    key: &'static str,
    sql: &str,
    flags: Vec<String>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
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

/// POST /jaxrs/group/list/group/tree (Java ActionListWithGroupTree，Wi{groupList})：
/// 以种子群组为根的嵌套树（真实递归子树 + 直接成员计数）。
pub async fn group_list_group_tree(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    const SQL: &str = "WITH RECURSIVE seeds AS (\
         SELECT id FROM x_org_group WHERE deleted_at IS NULL AND (id = ANY($1) OR name = ANY($1))),\
         tree AS (\
             SELECT g.id, g.name, g.parent_id, \"type\", g.unit_id FROM x_org_group g \
             JOIN seeds ON g.id = seeds.id\
             UNION \
             SELECT c.id, c.name, c.parent_id, c.\"type\", c.unit_id FROM x_org_group c \
             JOIN tree t ON c.parent_id = t.id WHERE c.deleted_at IS NULL)\
         SELECT id, name, parent_id, \"type\", unit_id FROM tree ORDER BY id";
    const CHILD_COUNT_SQL: &str = "SELECT parent_id, COUNT(*) FROM x_org_group \
         WHERE deleted_at IS NULL AND parent_id = ANY($1) GROUP BY parent_id";
    const PERSON_COUNT_SQL: &str = "SELECT m.group_id, COUNT(*) FROM x_org_group_member m \
         WHERE m.group_id = ANY($1) GROUP BY m.group_id";
    const IDENTITY_COUNT_SQL: &str = "SELECT m.group_id, COUNT(DISTINCT i.id) \
         FROM x_org_group_member m JOIN x_org_identity i ON i.person_id = m.person_id \
         AND i.deleted_at IS NULL WHERE m.group_id = ANY($1) GROUP BY m.group_id";

    let flags = normalize_flags(string_list(&body, "groupList"));
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
    let mut all_ids: Vec<String> = Vec::new();
    for row in &rows {
        let id: String = row.get("id");
        let mut obj = row_to_map(row);
        if let Value::Object(ref mut m) = obj {
            m.insert("subGroups".to_string(), Value::Array(vec![]));
            m.insert("subDirectGroupCount".to_string(), Value::Number(0.into()));
            m.insert("subDirectPersonCount".to_string(), Value::Number(0.into()));
            m.insert("subDirectIdentityCount".to_string(), Value::Number(0.into()));
        }
        all_ids.push(id.clone());
        base.insert(id, obj);
    }
    // 第二遍分类：parent 在集合内 → 子节点；否则为根
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

    // 计数三连查（真实聚合）
    let mut fill_counts = |rows: Vec<deadpool_postgres::tokio_postgres::Row>, key: &str| {
        for r in &rows {
            let gid: String = r.get(0);
            let cnt: i64 = r.get(1);
            if let Some(node) = base.get_mut(&gid) {
                if let Value::Object(ref mut m) = node {
                    m.insert(key.to_string(), Value::Number(cnt.into()));
                }
            }
        }
    };
    fill_counts(
        client
            .query(CHILD_COUNT_SQL, &[&all_ids])
            .await
            .map_err(|_| AppError::Internal)?,
        "subDirectGroupCount",
    );
    fill_counts(
        client
            .query(PERSON_COUNT_SQL, &[&all_ids])
            .await
            .map_err(|_| AppError::Internal)?,
        "subDirectPersonCount",
    );
    fill_counts(
        client
            .query(IDENTITY_COUNT_SQL, &[&all_ids])
            .await
            .map_err(|_| AppError::Internal)?,
        "subDirectIdentityCount",
    );

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
                m.insert("subGroups".to_string(), Value::Array(built));
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
