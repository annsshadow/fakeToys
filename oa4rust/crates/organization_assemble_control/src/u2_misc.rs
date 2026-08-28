use super::u2_helpers::*;
use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::error::AppError;

pub async fn unit_attribute_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let attr_key = normalize_key(opt(&body, &["attributeKey", "key", "name"]).unwrap_or_default());
    if attr_key.is_empty() {
        return Err(AppError::BadRequest("attributeKey is required".to_string()));
    }
    let unit_flag = opt(&body, &["unitId", "unit"]).unwrap_or_default().to_string();
    let unit_id = resolve_generic_id(&client, UNIT_TABLE, &unit_flag)
        .await?
        .ok_or_else(|| AppError::BadRequest("unit not found".to_string()))?;
    if normalized_name_dup(&client, "x_org_unit_attribute", "unit_id", &unit_id, &attr_key).await? {
        return err("unitattribute already exists");
    }
    let attr_value = opt(&body, &["attributeValue", "value"]).unwrap_or_default().to_string();
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_unit_attribute (id, unit_id, attribute_key, attribute_value, creator) VALUES ($1, $2, $3, $4, $5)",
            &[&id, &unit_id, &attr_key, &attr_value, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("id".to_string(), Value::String(id))].into_iter().collect(),
    ))
}

pub async fn unit_attribute_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(aid) = resolve_generic_id(&client, "x_org_unit_attribute", &flag).await? else {
        return err("unitattribute not found");
    };
    let attr_value = opt(&body, &["attributeValue", "value"]).unwrap_or_default().to_string();
    let updated = client
        .execute(
            "UPDATE x_org_unit_attribute SET attribute_value = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&aid, &attr_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("unitattribute not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(aid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn unit_attribute_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    unit_attribute_edit(pool, session, Path(flag), Json(body)).await
}

pub async fn unit_attribute_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, "x_org_unit_attribute", &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))].into_iter().collect(),
        )),
        None => err("unitattribute not found"),
    }
}


pub async fn person_attribute_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let attr_key = normalize_key(opt(&body, &["attributeKey", "key", "name"]).unwrap_or_default());
    if attr_key.is_empty() {
        return Err(AppError::BadRequest("attributeKey is required".to_string()));
    }
    let person_flag = opt(&body, &["personId", "person"]).unwrap_or_default().to_string();
    let person_id = super::u2_person::resolve_person_id(&client, &person_flag)
        .await?
        .ok_or_else(|| AppError::BadRequest("person not found".to_string()))?;
    if normalized_name_dup(&client, "x_org_person_attribute", "person_id", &person_id, &attr_key).await? {
        return err("personattribute already exists");
    }
    let attr_value = opt(&body, &["attributeValue", "value"]).unwrap_or_default().to_string();
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_person_attribute (id, person_id, attribute_key, attribute_value, creator) VALUES ($1, $2, $3, $4, $5)",
            &[&id, &person_id, &attr_key, &attr_value, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("id".to_string(), Value::String(id))].into_iter().collect(),
    ))
}

pub async fn person_attribute_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(aid) = resolve_generic_id(&client, "x_org_person_attribute", &flag).await? else {
        return err("personattribute not found");
    };
    let attr_value = opt(&body, &["attributeValue", "value"]).unwrap_or_default().to_string();
    let updated = client
        .execute(
            "UPDATE x_org_person_attribute SET attribute_value = $2 WHERE id = $1 AND deleted_at IS NULL",
            &[&aid, &attr_value],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("personattribute not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(aid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn person_attribute_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    person_attribute_edit(pool, session, Path(flag), Json(body)).await
}

pub async fn person_attribute_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, "x_org_person_attribute", &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))].into_iter().collect(),
        )),
        None => err("personattribute not found"),
    }
}


pub async fn permission_setting_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    let unit_flag = opt(&body, &["unitId", "unit"]).unwrap_or_default().to_string();
    let unit_id = if unit_flag.is_empty() {
        String::new()
    } else {
        resolve_generic_id(&client, UNIT_TABLE, &unit_flag)
            .await?
            .ok_or_else(|| AppError::BadRequest("unit not found".to_string()))?
    };
    if normalized_name_dup(&client, PERM_TABLE, "unit_id", &unit_id, &name).await? {
        return err("permission setting already exists");
    }
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_permission_setting (id, name, unit_id, creator) VALUES ($1, $2, NULLIF($3,''), $4)",
            &[&id, &name, &unit_id, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![("id".to_string(), Value::String(id))].into_iter().collect(),
    ))
}

pub async fn permission_setting_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(pid) = resolve_generic_id(&client, PERM_TABLE, &flag).await? else {
        return err("permission setting not found");
    };
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let updated = client
        .execute(
            "UPDATE x_org_permission_setting SET name = CASE WHEN $2 = '' THEN name ELSE $2 END WHERE id = $1 AND deleted_at IS NULL",
            &[&pid, &name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("permission setting not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(pid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn permission_setting_mock_put_to_post(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    permission_setting_edit(pool, session, Path(flag), Json(body)).await
}

pub async fn permission_setting_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, PERM_TABLE, &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))].into_iter().collect(),
        )),
        None => err("permission setting not found"),
    }
}

pub async fn person_card_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    if normalized_name_dup(&client, CARD_TABLE, "creator", "", &name).await? {
        return err("person card already exists");
    }
    let group_type = opt(&body, &["groupType", "type"]).unwrap_or_default().to_string();
    let distinguished_name = opt(&body, &["distinguishedName"]).unwrap_or_default().to_string();
    let mobile = opt(&body, &["mobile"]).unwrap_or_default().to_string();
    let office_phone = opt(&body, &["officePhone"]).unwrap_or_default().to_string();
    let address = opt(&body, &["address"]).unwrap_or_default().to_string();
    let description = opt(&body, &["description"]).unwrap_or_default().to_string();
    let creator = session.person_unique.clone();
    let id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_personcard (id, name, group_type, distinguished_name, mobile, office_phone, address, description, creator)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
            &[&id, &name, &group_type, &distinguished_name, &mobile, &office_phone, &address, &description, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn person_card_edit(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let Some(cid) = resolve_generic_id(&client, CARD_TABLE, &flag).await? else {
        return err("person card not found");
    };
    let name = normalize_key(opt(&body, &["name"]).unwrap_or_default());
    let mobile = opt(&body, &["mobile"]).unwrap_or_default().to_string();
    let office_phone = opt(&body, &["officePhone"]).unwrap_or_default().to_string();
    let address = opt(&body, &["address"]).unwrap_or_default().to_string();
    let description = opt(&body, &["description"]).unwrap_or_default().to_string();
    let updated = client
        .execute(
            "UPDATE x_org_personcard SET
                name = CASE WHEN $2 = '' THEN name ELSE $2 END,
                mobile = CASE WHEN $3 = '' THEN mobile ELSE $3 END,
                office_phone = CASE WHEN $4 = '' THEN office_phone ELSE $4 END,
                address = CASE WHEN $5 = '' THEN address ELSE $5 END,
                description = CASE WHEN $6 = '' THEN description ELSE $6 END
             WHERE id = $1 AND deleted_at IS NULL",
            &[&cid, &name, &mobile, &office_phone, &address, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if updated == 0 {
        return err("person card not updated");
    }
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(cid)),
            ("value".to_string(), Value::Bool(true)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn person_card_delete(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Path(flag): Path<String>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    match soft_delete_generic(&client, CARD_TABLE, &flag).await? {
        Some(id) => ok(Value::Object(
            vec![("id".to_string(), Value::String(id))].into_iter().collect(),
        )),
        None => err("person card not found"),
    }
}

async fn card_page(pool: &Pool, page: i64, size: i64, body: &Value, with_group: bool) -> HandlerResult {
    let client = client_of(pool).await?;
    let page = page.max(1);
    let size = size.clamp(1, MAX_PAGE_SIZE);
    let offset = ((page - 1) * size).to_string();
    let size_str = size.to_string();
    let key = normalize_key(opt(body, &["key"]).unwrap_or_default());
    let group_type = normalize_key(opt(body, &["groupType", "type"]).unwrap_or_default());
    let cond = if with_group {
        "deleted_at IS NULL
                AND ($1 = '' OR name ILIKE '%'||$1||'%' OR mobile ILIKE '%'||$1||'%'
                     OR office_phone ILIKE '%'||$1||'%' OR group_type ILIKE '%'||$1||'%')
                AND ($2 = '' OR group_type = $2)"
    } else {
        "deleted_at IS NULL
                AND ($1 = '' OR name ILIKE '%'||$1||'%' OR mobile ILIKE '%'||$1||'%'
                     OR office_phone ILIKE '%'||$1||'%' OR description ILIKE '%'||$1||'%')"
    };
    let total_sql = format!("SELECT COUNT(*)::bigint AS cnt FROM x_org_personcard WHERE {cond}");
    let total: i64 = client
        .query_one(&total_sql, &[&key, &group_type])
        .await
        .map_err(|_| AppError::Internal)?
        .get("cnt");
    let data_sql = format!(
        "SELECT id, name, group_type, distinguished_name, mobile, office_phone, address, description, creator, create_time::text
           FROM x_org_personcard WHERE {cond} ORDER BY create_time::text DESC LIMIT $3::int OFFSET $4::int"
    );
    let rows = client
        .query(&data_sql, &[&key, &group_type, &size_str, &offset])
        .await
        .map_err(|_| AppError::Internal)?;
    let mut data = Vec::new();
    for row in &rows {
        let mut item = serde_json::Map::new();
        for (key_name, col) in [
            ("id", "id"),
            ("name", "name"),
            ("groupType", "group_type"),
            ("distinguishedName", "distinguished_name"),
            ("mobile", "mobile"),
            ("officePhone", "office_phone"),
            ("address", "address"),
            ("description", "description"),
            ("creator", "creator"),
            ("createTime", "create_time"),
        ] {
            let v: Option<String> = row.try_get(col).ok().flatten();
            if let Some(v) = v {
                item.insert(key_name.to_string(), Value::String(v));
            }
        }
        data.push(Value::Object(item));
    }
    ok(Value::Object(
        vec![
            ("count".to_string(), Value::Number(total.into())),
            ("page".to_string(), Value::Number(page.into())),
            ("size".to_string(), Value::Number(size.into())),
            ("data".to_string(), Value::Array(data)),
        ]
        .into_iter()
        .collect(),
    ))
}

pub async fn person_card_listpaging(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    Json(body): Json<Value>,
) -> HandlerResult {
    card_page(&pool, page, size, &body, false).await
}

pub async fn person_card_listpaging_mock_put_to_post(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    Json(body): Json<Value>,
) -> HandlerResult {
    card_page(&pool, page, size, &body, false).await
}

pub async fn person_card_listpaging_with_group(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    Json(body): Json<Value>,
) -> HandlerResult {
    card_page(&pool, page, size, &body, true).await
}

pub async fn person_card_listpaging_with_group_mock_put_to_post(
    pool: Extension<Pool>,
    Path((page, size)): Path<(i64, i64)>,
    Json(body): Json<Value>,
) -> HandlerResult {
    card_page(&pool, page, size, &body, true).await
}

pub async fn input_person_import(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    Json(body): Json<Value>,
) -> HandlerResult {
    require_admin(&pool, &session).await?;
    let client = client_of(&pool).await?;
    let person_list = body
        .get("personList")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    check_batch_len(person_list.len())?;
    let mut inserted: i64 = 0;
    let mut skipped: i64 = 0;
    for item in &person_list {
        let Some(raw_name) = item.get("name").and_then(|v| v.as_str()) else {
            continue;
        };
        let name = normalize_key(raw_name);
        if name.is_empty() {
            continue;
        }
        let dup = client
            .query_opt(
                "SELECT id FROM x_org_person WHERE LOWER(TRIM(name)) = LOWER($1) AND deleted_at IS NULL",
                &[&name],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if dup.is_some() {
            skipped += 1;
            continue;
        }
        let mobile = item.get("mobile").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let email = item.get("email").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let creator = session.person_unique.clone();
        let id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_org_person (id, name, mobile, email, creator) VALUES ($1, $2, NULLIF($3,''), NULLIF($4,''), $5)",
                &[&id, &name, &mobile, &email, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        inserted += 1;
    }
    let message = format!("inserted={inserted},skipped={skipped}");
    let batch_id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_org_import_result (id, status, message) VALUES ($1, 'done', $2)",
            &[&batch_id, &message],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    ok(Value::Object(
        vec![
            ("id".to_string(), Value::String(batch_id)),
            ("inserted".to_string(), Value::Number(inserted.into())),
            ("skipped".to_string(), Value::Number(skipped.into())),
        ]
        .into_iter()
        .collect(),
    ))
}
