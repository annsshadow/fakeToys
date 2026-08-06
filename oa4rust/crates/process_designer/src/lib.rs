use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::response::ActionResult;

pub mod routes;

pub use routes::process_designer_router;

pub async fn application_list_summary(
    pool: Extension<Pool>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                    ("data".to_string(), Value::Array(vec![])),
                ]),
            )));
        }
    };

    let rows = match client
        .query(
            "SELECT id, name, application_category FROM application ORDER BY name",
            &[],
        )
        .await
        {
            Ok(rows) => rows,
            Err(_) => {
                return Json(ActionResult::success(Value::Object(
                    serde_json::Map::from_iter([
                        ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                        ("data".to_string(), Value::Array(vec![])),
                    ]),
                )));
            }
        };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "applicationCategory".to_string(),
                    Value::String(row.get("application_category")),
                ),
            ]))
        })
        .collect();

    Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    )))
}

pub async fn application_list(
    pool: Extension<Pool>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                    ("data".to_string(), Value::Array(vec![])),
                ]),
            )));
        }
    };

    let rows = match client
        .query(
            "SELECT id, name, application_category, description, creator FROM application ORDER BY name",
            &[],
        )
        .await
        {
            Ok(rows) => rows,
            Err(_) => {
                return Json(ActionResult::success(Value::Object(
                    serde_json::Map::from_iter([
                        ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                        ("data".to_string(), Value::Array(vec![])),
                    ]),
                )));
            }
        };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "applicationCategory".to_string(),
                    Value::String(row.get("application_category")),
                ),
                (
                    "description".to_string(),
                    row.get::<_, Option<String>>("description")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
                ("creator".to_string(), Value::String(row.get("creator"))),
            ]))
        })
        .collect();

    Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    )))
}

pub async fn designer_get_route(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([("data".to_string(), Value::Null)]),
            )));
        }
    };

    let row = match client
        .query_one("SELECT id, name, process_id, type, description FROM route WHERE id = $1", &[&id])
        .await
        {
            Ok(row) => row,
            Err(_) => {
                return Json(ActionResult::success(Value::Object(
                    serde_json::Map::from_iter([("data".to_string(), Value::Null)]),
                )));
            }
        };

    let route_data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("processId".to_string(), Value::String(row.get("process_id"))),
        ("type".to_string(), Value::String(row.get::<_, String>("type"))),
        (
            "description".to_string(),
            row.get::<_, Option<String>>("description")
                .map(Value::String)
                .unwrap_or(Value::Null),
        ),
    ]));

    Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("data".to_string(), route_data)]),
    )))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::process_designer_router(pool)
}