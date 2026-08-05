use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::response::ActionResult;

pub mod routes;

pub use routes::process_express_router;

pub async fn task_count(
    pool: Extension<Pool>,
    Path(credential): Path<String>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("credential".to_string(), Value::String(credential)),
                    ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                ]),
            )));
        }
    };

    let row = match client
        .query_one("SELECT COUNT(*) as count FROM auth_person WHERE unique_id = $1", &[&credential])
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("credential".to_string(), Value::String(credential)),
                    ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                ]),
            )));
        }
    };

    let count: i64 = row.get("count");

    Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("credential".to_string(), Value::String(credential)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    )))
}

pub async fn read_count(
    pool: Extension<Pool>,
    Path(credential): Path<String>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("credential".to_string(), Value::String(credential)),
                    ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                ]),
            )));
        }
    };

    let row = match client
        .query_one("SELECT COUNT(*) as count FROM auth_person WHERE unique_id = $1", &[&credential])
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("credential".to_string(), Value::String(credential)),
                    ("count".to_string(), Value::Number(serde_json::Number::from(0))),
                ]),
            )));
        }
    };

    let count: i64 = row.get("count");

    Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("credential".to_string(), Value::String(credential)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
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
        .query("SELECT id, name FROM auth_unit ORDER BY name LIMIT 10", &[])
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

#[cfg(test)]
mod tests;

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/process_express/health", axum::routing::get(|| async { "TODO: process_express - real implementation needed" }))
}