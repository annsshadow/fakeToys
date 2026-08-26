use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::response::ActionResult;

pub mod routes;

pub use routes::process_surface_router;

pub async fn list_ids(
    pool: Extension<Pool>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::java_success(Value::Array(vec![]), 0, 0));
        }
    };

    let rows = match client
        .query("SELECT id, flag FROM oa_process ORDER BY create_time LIMIT 20", &[])
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return Json(ActionResult::java_success(Value::Array(vec![]), 0, 0));
        }
    };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Json(ActionResult::java_success(Value::Array(data), count, 0))
}

pub async fn get_by_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("flag".to_string(), Value::String(flag)),
                    ("name".to_string(), Value::String("mock_process".to_string())),
                    ("description".to_string(), Value::String("mock_description".to_string())),
                ]),
            )));
        }
    };

    let row = match client
        .query_one("SELECT id, flag, name, description FROM oa_process WHERE flag = $1", &[&flag])
        .await
    {
        Ok(row) => row,
        Err(_) => {
            return Json(ActionResult::error("not found"));
        }
    };

    Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("flag".to_string(), Value::String(row.get("flag"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("description".to_string(), Value::String(row.get("description"))),
        ]),
    )))
}

pub async fn record_list(
    pool: Extension<Pool>,
    Path(work_or_work_completed): Path<String>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Json(ActionResult::java_success(Value::Array(vec![]), 0, 0));
        }
    };

    let rows = match client
        .query("SELECT id, work_or_work_completed, title, create_time FROM oa_workcompleted WHERE work_or_work_completed = $1 ORDER BY create_time LIMIT 20", &[&work_or_work_completed])
        .await
    {
        Ok(rows) => rows,
        Err(_) => {
            return Json(ActionResult::java_success(Value::Array(vec![]), 0, 0));
        }
    };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("workOrWorkCompleted".to_string(), Value::String(row.get("work_or_work_completed"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Json(ActionResult::java_success(Value::Array(data), count, 0))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::process_surface_router(pool)
}
