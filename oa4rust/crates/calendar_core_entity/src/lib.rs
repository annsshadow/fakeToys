use axum::{
    extract::Extension,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CalendarItem {
    pub id: String,
    pub name: String,
    pub calendar_type: String,
    pub target: String,
    pub color: String,
    pub description: Option<String>,
    pub createor: String,
    pub is_public: bool,
    pub status: String,
}

pub async fn calendar_list_public(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM CAL_CALENDAR WHERE is_public = true AND status = 'OPEN' ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("target".to_string(), Value::String(row.get("target"))),
                ("color".to_string(), Value::String(row.get("color"))),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("createor".to_string(), Value::String(row.get("createor"))),
                ("isPublic".to_string(), Value::Bool(row.get("is_public"))),
                ("status".to_string(), Value::String(row.get("status"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn calendar_list_my(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM CAL_CALENDAR WHERE status = 'OPEN' ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut my_calendars = Vec::new();
    let mut unit_calendars = Vec::new();

    for row in rows.iter() {
        let calendar_type: String = row.get("type");
        let item: Value = Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("type".to_string(), Value::String(calendar_type.clone())),
            ("target".to_string(), Value::String(row.get("target"))),
            ("color".to_string(), Value::String(row.get("color"))),
            (
                "description".to_string(),
                row.get::<_, Option<String>>("description")
                    .map(Value::String)
                    .unwrap_or(Value::Null),
            ),
            ("createor".to_string(), Value::String(row.get("createor"))),
            ("isPublic".to_string(), Value::Bool(row.get("is_public"))),
            ("status".to_string(), Value::String(row.get("status"))),
        ]));

        if calendar_type.eq_ignore_ascii_case("UNIT") {
            unit_calendars.push(item);
        } else {
            my_calendars.push(item);
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "myCalendars".to_string(),
                Value::Array(my_calendars),
            ),
            (
                "unitCalendars".to_string(),
                Value::Array(unit_calendars),
            ),
            (
                "followCalendars".to_string(),
                Value::Array(Vec::<Value>::new()),
            ),
        ]),
    ))))
}

pub async fn calendar_get(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, type, target, color, description, createor, is_public, status \
             FROM CAL_CALENDAR WHERE id = $1 AND status = 'OPEN'",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("type".to_string(), Value::String(row.get("type"))),
        ("target".to_string(), Value::String(row.get("target"))),
        ("color".to_string(), Value::String(row.get("color"))),
        (
            "description".to_string(),
            row.get::<_, Option<String>>("description")
                .map(Value::String)
                .unwrap_or(Value::Null),
        ),
        ("createor".to_string(), Value::String(row.get("createor"))),
        ("isPublic".to_string(), Value::Bool(row.get("is_public"))),
        ("status".to_string(), Value::String(row.get("status"))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub fn calendar_core_entity_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/calendar/core/entity/calendar/list/public", get(calendar_list_public))
        .route("/jaxrs/calendar/core/entity/calendar/list/my", get(calendar_list_my))
        .route("/jaxrs/calendar/core/entity/calendar/{id}", get(calendar_get))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/calendar_core_entity/health", axum::routing::get(|| async { "TODO: calendar_core_entity - real implementation needed" }))
}