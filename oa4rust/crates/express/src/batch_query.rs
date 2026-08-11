use axum::{extract::Extension, Json};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use shared::error::AppError;
use shared::response::ActionResult;

const ID_COUNT_LIMIT: usize = 100;

// ── Request DTOs ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PersonListRequest {
    #[serde(default)]
    pub ids: Option<Vec<String>>,
    #[serde(default)]
    pub identities: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UnitListRequest {
    #[serde(default)]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct IdentityListRequest {
    #[serde(default)]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct GroupListRequest {
    #[serde(default)]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RoleListRequest {
    #[serde(default)]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PersonWithUnitRequest {
    #[serde(default)]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PersonWithIdentityRequest {
    #[serde(default)]
    pub ids: Option<Vec<String>>,
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn validate_id_list(ids: &[String]) -> Result<(), AppError> {
    if ids.is_empty() {
        return Err(AppError::BadRequest("ids list is empty".to_string()));
    }
    if ids.len() > ID_COUNT_LIMIT {
        return Err(AppError::BadRequest(format!(
            "ids list exceeds limit of {} items",
            ID_COUNT_LIMIT
        )));
    }
    Ok(())
}

fn person_row_to_value(
    row: &deadpool_postgres::tokio_postgres::Row,
    include_pii: bool,
) -> Value {
    let mut map = serde_json::Map::new();
    map.insert("id".to_string(), Value::String(row.get("id")));
    map.insert("unique".to_string(), Value::String(row.get("unique_id")));
    map.insert("name".to_string(), Value::String(row.get("name")));
    if include_pii {
        map.insert(
            "mobile".to_string(),
            row.get::<_, Option<String>>("mobile")
                .map(Value::String)
                .unwrap_or(Value::Null),
        );
        map.insert(
            "email".to_string(),
            row.get::<_, Option<String>>("email")
                .map(Value::String)
                .unwrap_or(Value::Null),
        );
    }
    map.insert(
        "icon".to_string(),
        row.get::<_, Option<String>>("icon")
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    map.insert(
        "job".to_string(),
        row.get::<_, Option<String>>("job")
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    map.insert(
        "department".to_string(),
        row.get::<_, Option<String>>("department")
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    map.insert(
        "unit".to_string(),
        row.get::<_, Option<String>>("unit")
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    map.insert(
        "position".to_string(),
        row.get::<_, Option<String>>("position")
            .map(Value::String)
            .unwrap_or(Value::Null),
    );
    Value::Object(map)
}

// ── Handlers ─────────────────────────────────────────────────────────────────

/// POST /jaxrs/express/person/list
///
/// Accepts {"ids":[...]} or {"identities":[...]}, returns full Person list.
/// No authentication required. ID count capped at 100. PII fields (mobile/email)
/// excluded by default.
pub async fn express_person_list(
    pool: Extension<Pool>,
    Json(payload): Json<PersonListRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = payload.ids.unwrap_or_default();
    let identities = payload.identities.unwrap_or_default();

    if ids.is_empty() && identities.is_empty() {
        return Ok(Json(ActionResult::error("ids or identities required")));
    }
    if ids.len() + identities.len() > ID_COUNT_LIMIT {
        return Ok(Json(ActionResult::error(
            format!("total ids + identities exceeds limit of {}", ID_COUNT_LIMIT),
        )));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let mut persons: HashMap<String, Value> = HashMap::new();

    // Query by ids
    if !ids.is_empty() {
        let sql = "SELECT id, unique_id, name, job, department, position, icon FROM auth_person WHERE deleted_at IS NULL AND id = $1";
        for id in &ids {
            let rows = client
                .query(sql, &[id])
                .await
                .map_err(|_| AppError::Internal)?;
            for row in &rows {
                let pid = row.get::<_, String>("id");
                persons.entry(pid).or_insert_with(|| person_row_to_value(row, false));
            }
        }
    }

    // Query by identities (look up persons via auth_person_identity)
    if !identities.is_empty() {
        let identity_ids_sql = "SELECT id, name FROM auth_identity WHERE deleted_at IS NULL AND id = $1";
        let mut identity_ids: Vec<String> = Vec::new();
        for id in &identities {
            let rows = client
                .query(identity_ids_sql, &[id])
                .await
                .map_err(|_| AppError::Internal)?;
            if !rows.is_empty() {
                identity_ids.push(id.clone());
            }
        }
        if !identity_ids.is_empty() {
            let person_ids_sql =
                "SELECT pi.person_id FROM auth_person_identity pi JOIN auth_identity i ON i.id = pi.identity_id WHERE i.deleted_at IS NULL AND pi.identity_id = $1";
            let mut person_id_set: Vec<String> = Vec::new();
            for identity_id in &identity_ids {
                let rows = client
                    .query(person_ids_sql, &[identity_id])
                    .await
                    .map_err(|_| AppError::Internal)?;
                for row in &rows {
                    person_id_set.push(row.get::<_, String>("person_id"));
                }
            }
            if !person_id_set.is_empty() {
                let person_sql = "SELECT id, unique_id, name, job, department, position, icon FROM auth_person WHERE deleted_at IS NULL AND id = $1";
                for pid in &person_id_set {
                    let rows = client
                        .query(person_sql, &[pid])
                        .await
                        .map_err(|_| AppError::Internal)?;
                    for row in &rows {
                        let pid = row.get::<_, String>("id");
                        persons.entry(pid).or_insert_with(|| person_row_to_value(row, false));
                    }
                }
            }
        }
    }

    let mut data: Vec<Value> = persons.into_values().collect();
    data.sort_by(|a, b| a["id"].as_str().unwrap_or("").cmp(b["id"].as_str().unwrap_or("")));

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/express/unit/list
///
/// Accepts unit ID list, returns full Unit list.
/// No authentication required. ID count capped at 100.
pub async fn express_unit_list(
    pool: Extension<Pool>,
    Json(payload): Json<UnitListRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = payload.ids.unwrap_or_default();
    validate_id_list(&ids)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    let sql = "SELECT id, name, parent_id, level FROM auth_unit WHERE deleted_at IS NULL AND id = $1";
    for id in &ids {
        let rows = client.query(sql, &[id]).await.map_err(|_| AppError::Internal)?;
        for row in &rows {
            data.push(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "parentId".to_string(),
                    row.get::<_, Option<String>>("parent_id")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "level".to_string(),
                    Value::Number(serde_json::Number::from(row.get::<_, i32>("level"))),
                ),
            ])));
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/express/identity/list
///
/// Accepts identity ID list, returns full Identity list.
/// No authentication required. ID count capped at 100.
pub async fn express_identity_list(
    pool: Extension<Pool>,
    Json(payload): Json<IdentityListRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = payload.ids.unwrap_or_default();
    validate_id_list(&ids)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    let sql = "SELECT id, name FROM auth_identity WHERE deleted_at IS NULL AND id = $1";
    for id in &ids {
        let rows = client.query(sql, &[id]).await.map_err(|_| AppError::Internal)?;
        for row in &rows {
            data.push(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
            ])));
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/express/group/list
///
/// Accepts group ID list, returns full Group list.
/// No authentication required. ID count capped at 100.
pub async fn express_group_list(
    pool: Extension<Pool>,
    Json(payload): Json<GroupListRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = payload.ids.unwrap_or_default();
    validate_id_list(&ids)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    let sql = "SELECT id, name FROM auth_group WHERE deleted_at IS NULL AND id = $1";
    for id in &ids {
        let rows = client.query(sql, &[id]).await.map_err(|_| AppError::Internal)?;
        for row in &rows {
            data.push(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
            ])));
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/express/role/list
///
/// Accepts role ID list, returns full Role list.
/// No authentication required. ID count capped at 100.
pub async fn express_role_list(
    pool: Extension<Pool>,
    Json(payload): Json<RoleListRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = payload.ids.unwrap_or_default();
    validate_id_list(&ids)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    let sql = "SELECT id, name, description FROM auth_role WHERE deleted_at IS NULL AND id = $1";
    for id in &ids {
        let rows = client.query(sql, &[id]).await.map_err(|_| AppError::Internal)?;
        for row in &rows {
            data.push(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
            ])));
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/express/person/with/unit
///
/// Accepts person ID list, returns each person with their organization (unit) info.
/// No authentication required. ID count capped at 100.
pub async fn express_person_with_unit(
    pool: Extension<Pool>,
    Json(payload): Json<PersonWithUnitRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = payload.ids.unwrap_or_default();
    validate_id_list(&ids)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    let sql = "SELECT p.id, p.unique_id, p.name, p.job, p.department, p.position, p.unit AS unit_id, u.name AS unit_name FROM auth_person p LEFT JOIN auth_unit u ON p.unit = u.id WHERE p.deleted_at IS NULL AND p.id = $1";
    for id in &ids {
        let rows = client.query(sql, &[id]).await.map_err(|_| AppError::Internal)?;
        for row in &rows {
            data.push(Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unique".to_string(), Value::String(row.get("unique_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "job".to_string(),
                    row.get::<_, Option<String>>("job")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "department".to_string(),
                    row.get::<_, Option<String>>("department")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                (
                    "position".to_string(),
                    row.get::<_, Option<String>>("position")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("unit".to_string(), Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), row.get::<_, Option<String>>("unit_id").map(Value::String).unwrap_or(Value::Null)),
                    ("name".to_string(), row.get::<_, Option<String>>("unit_name").map(Value::String).unwrap_or(Value::Null)),
                ]))),
            ])));
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// POST /jaxrs/express/person/with/identity
///
/// Accepts person ID list, returns each person with their identities.
/// No authentication required. ID count capped at 100.
pub async fn express_person_with_identity(
    pool: Extension<Pool>,
    Json(payload): Json<PersonWithIdentityRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ids = payload.ids.unwrap_or_default();
    validate_id_list(&ids)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let mut data: Vec<Value> = Vec::new();
    let sql = "SELECT p.id, p.unique_id, p.name, pi.identity_id, i.name AS identity_name FROM auth_person p JOIN auth_person_identity pi ON pi.person_id = p.id JOIN auth_identity i ON i.id = pi.identity_id WHERE p.deleted_at IS NULL AND p.id = $1 AND i.deleted_at IS NULL";
    for id in &ids {
        let rows = client.query(sql, &[id]).await.map_err(|_| AppError::Internal)?;
        let identities: Vec<Value> = rows
            .iter()
            .map(|row| {
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("identity_id"))),
                    ("name".to_string(), Value::String(row.get("identity_name"))),
                ]))
            })
            .collect();
        data.push(Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("unique".to_string(), if rows.is_empty() { Value::Null } else { Value::String(rows[0].get("unique_id")) }),
            ("name".to_string(), if rows.is_empty() { Value::Null } else { Value::String(rows[0].get("name")) }),
            ("identities".to_string(), Value::Array(identities)),
        ])));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            ),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}
