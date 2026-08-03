use axum::extract::{Extension, Path, Query};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::response::ActionResult;

pub async fn get(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return Json(ActionResult::error("database connection error")),
    };

    let row = match client
        .query_one(
            "SELECT id, unique_id, name, mobile, email FROM auth_person WHERE unique_id = $1",
            &[&flag],
        )
        .await
    {
        Ok(r) => r,
        Err(_) => return Json(ActionResult::error("person not found")),
    };

    let id: String = row.get("id");
    let unique: String = row.get("unique_id");
    let name: String = row.get("name");
    let mobile: Option<String> = row.get("mobile");
    let email: Option<String> = row.get("email");

    let result = ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("flag".to_string(), Value::String(flag)),
        ("id".to_string(), Value::String(id)),
        ("unique".to_string(), Value::String(unique)),
        ("name".to_string(), Value::String(name)),
        ("mobile".to_string(), mobile.map(Value::String).unwrap_or(Value::Null)),
        ("email".to_string(), email.map(Value::String).unwrap_or(Value::Null)),
    ])));
    Json(result)
}

pub async fn list(
    pool: Extension<Pool>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Json<ActionResult<Value>> {
    let client = match pool.get().await {
        Ok(c) => c,
        Err(_) => return Json(ActionResult::error("database connection error")),
    };

    let page = params.get("page").and_then(|p| p.parse::<i64>().ok()).unwrap_or(1);
    let size = params.get("size").and_then(|s| s.parse::<i64>().ok()).unwrap_or(20);
    let offset = (page - 1) * size;

    let count_result = client
        .query_one("SELECT COUNT(*) as count FROM auth_person", &[])
        .await;
    let total = count_result.map(|r| r.get::<_, i64>("count")).unwrap_or(0);

    let rows = match client
        .query(
            "SELECT id, unique_id, name, mobile, email FROM auth_person LIMIT $1 OFFSET $2",
            &[&size, &offset],
        )
        .await
    {
        Ok(rows) => rows,
        Err(_) => return Json(ActionResult::error("query failed")),
    };

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("unique".to_string(), Value::String(row.get("unique_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mobile".to_string(), row.get::<_, Option<String>>("mobile").map(Value::String).unwrap_or(Value::Null)),
                ("email".to_string(), row.get::<_, Option<String>>("email").map(Value::String).unwrap_or(Value::Null)),
            ]))
        })
        .collect();

    let result = ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(total))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("data".to_string(), Value::Array(data)),
    ])));
    Json(result)
}
