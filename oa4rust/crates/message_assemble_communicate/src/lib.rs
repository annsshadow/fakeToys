use axum::{
    extract::Extension,
    Json, Router,
    routing::get, routing::post, routing::delete,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use uuid::Uuid;

pub mod routes;

pub async fn send_message(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let conversation_id = req.get("conversationId").and_then(|v| v.as_str()).unwrap_or_default();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system");
    let msg_type = req.get("type").and_then(|v| v.as_str()).unwrap_or("text");
    let id = Uuid::new_v4().to_string();

    client
        .execute("INSERT INTO x_message (id, conversation_id, content, sender, type, create_time) VALUES ($1, $2, $3, $4, $5, NOW())", &[&id, &conversation_id, &content, &sender, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message_conversation SET last_message_time = NOW() WHERE id = $1", &[&conversation_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("conversationId".to_string(), Value::String(conversation_id.to_string())),
            ("content".to_string(), Value::String(content.to_string())),
            ("sender".to_string(), Value::String(sender.to_string())),
            ("type".to_string(), Value::String(msg_type.to_string())),
            ("sent".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn receive_list(
    pool: Extension<Pool>,
    axum::extract::Path(consume): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consume = $1 AND consumed = false ORDER BY create_time ASC", &[&consume])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn mark_read(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message_consume SET consumed = true WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("marked_read".to_string(), Value::Bool(true))]),
    ))))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    routes::router(pool)
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;




pub async fn consume_list_consume_count_count(
    pool: Extension<Pool>,
    axum::extract::Path((consume, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let limit = count.max(1) as i64;
    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consume = $1 ORDER BY create_time DESC LIMIT $2", &[&consume, &limit])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn consume_list_consume_currentperson_count_count(
    pool: Extension<Pool>,
    axum::extract::Path((consume, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let limit = count.max(1) as i64;
    let rows = client
        .query("SELECT id, consume, content, sender, read_status, create_time FROM x_message_consume WHERE consume = $1 AND sender = consume ORDER BY create_time DESC LIMIT $2", &[&consume, &limit])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("readStatus".to_string(), Value::String(row.get("read_status"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn consume_list_consume_person_person_count_count(
    pool: Extension<Pool>,
    axum::extract::Path((consume, person, count)): axum::extract::Path<(String, String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let limit = count.max(1) as i64;
    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consume = $1 AND sender = $2 ORDER BY create_time DESC LIMIT $3", &[&consume, &person, &limit])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn consume_type_type(
    pool: Extension<Pool>,
    axum::extract::Path(msg_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, type, sender, create_time FROM x_message_consume WHERE type = $1 ORDER BY create_time DESC", &[&msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn consume_type_type_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(msg_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = Uuid::new_v4().to_string();
    client
        .execute("INSERT INTO x_message_consume (id, consume, content, type, create_time) VALUES ($1, $2, '', $3, NOW())", &[&id, &msg_type, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn consume_id_type_type(
    pool: Extension<Pool>,
    axum::extract::Path((id, msg_type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_message_consume SET type = $1 WHERE id = $2", &[&msg_type, &id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("consume not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_conversation(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let conversation_type = req.get("type").and_then(|v| v.as_str()).unwrap_or("single");
    let id = Uuid::new_v4().to_string();

    client
        .execute("INSERT INTO x_message_conversation (id, name, type, create_time) VALUES ($1, $2, $3, NOW())", &[&id, &name, &conversation_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name.to_string())),
            ("type".to_string(), Value::String(conversation_type.to_string())),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn im_conversation_business_businessId(
    pool: Extension<Pool>,
    axum::extract::Path(business_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, name, type, business_id, create_time FROM x_message_conversation WHERE business_id = $1 LIMIT 1", &[&business_id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("businessId".to_string(), Value::String(row.get("business_id"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("conversation not found"))),
    }
}

pub async fn im_conversation_list_my(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, type, last_message, create_time FROM x_message_conversation ORDER BY update_time DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("lastMessage".to_string(), Value::String(row.get("last_message"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn im_conversation_list_with_person(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, name, type, create_time FROM x_message_conversation WHERE type = 'single' ORDER BY create_time DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn im_conversation_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = req.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let title = req.get("title").and_then(|v| v.as_str()).unwrap_or_default();
    let note = req.get("note").and_then(|v| v.as_str()).unwrap_or_default();

    client
        .execute("UPDATE x_message_conversation SET title = COALESCE($2, title), note = COALESCE($3, note), update_time = NOW() WHERE id = $1", &[&id, &title, &note])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("updated".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_conversation_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, name, type, last_message, create_time FROM x_message_conversation WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("lastMessage".to_string(), Value::String(row.get("last_message"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("conversation not found"))),
    }
}

pub async fn im_conversation_id_group(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, conversation_id, person_id, role, join_time FROM x_message_conversation_member WHERE conversation_id = $1 ORDER BY join_time", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("conversationId".to_string(), Value::String(row.get("conversation_id"))),
            ("personId".to_string(), Value::String(row.get("person_id"))),
            ("role".to_string(), Value::String(row.get("role"))),
            ("joinTime".to_string(), Value::String(row.get("join_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn im_conversation_id_group_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_conversation_id_group_quit_self(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1 AND person_id = $2", &[&id, &""])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("quit".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_conversation_id_icon(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT icon_url, icon_name, create_time FROM x_message_conversation_icon WHERE conversation_id = $1 ORDER BY create_time DESC LIMIT 1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("conversationId".to_string(), Value::String(id)),
                ("iconUrl".to_string(), Value::String(row.get("icon_url"))),
                ("iconName".to_string(), Value::String(row.get("icon_name"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("icon not found"))),
    }
}

pub async fn im_conversation_id_read(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message_conversation SET read_status = 'read', read_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("read".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_conversation_id_read_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message_conversation SET read_status = 'read', read_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("read".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_conversation_id_single(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, name, type, create_time FROM x_message_conversation WHERE id = $1 AND type = 'single'", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("type".to_string(), Value::String(row.get("type"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("single conversation not found"))),
    }
}

pub async fn im_conversation_id_single_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_message_conversation WHERE id = $1 AND type = 'single'", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_conversation_id_top_cancel(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message_conversation SET top = false WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topCancelled".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_conversation_id_top_cancel_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message_conversation SET top = false WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topCancelled".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_conversation_id_top_set(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message_conversation SET top = true, top_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topSet".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_conversation_id_top_set_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message_conversation SET top = true, top_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topSet".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_manager_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, config_key, config_value, create_time FROM x_message_config ORDER BY create_time DESC LIMIT 1", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("configKey".to_string(), Value::String(row.get("config_key"))),
                ("configValue".to_string(), Value::String(row.get("config_value"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("config not found"))),
    }
}

pub async fn im_msg(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let conversation_id = req.get("conversationId").and_then(|v| v.as_str()).unwrap_or_default();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system");
    let msg_type = req.get("type").and_then(|v| v.as_str()).unwrap_or("text");
    let id = Uuid::new_v4().to_string();

    client
        .execute("INSERT INTO x_message (id, conversation_id, content, sender, type, create_time) VALUES ($1, $2, $3, $4, $5, NOW())", &[&id, &conversation_id, &content, &sender, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("conversationId".to_string(), Value::String(conversation_id.to_string())),
            ("content".to_string(), Value::String(content.to_string())),
            ("sender".to_string(), Value::String(sender.to_string())),
            ("type".to_string(), Value::String(msg_type.to_string())),
            ("sent".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn im_msg_clear(
    pool: Extension<Pool>,
    axum::extract::Path(conversation_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message SET cleared = true WHERE conversation_id = $1", &[&conversation_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("cleared".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_msg_collection(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let message_id = req.get("messageId").and_then(|v| v.as_str()).unwrap_or_default();
    client
        .execute("INSERT INTO x_message_collection (id, message_id, create_time) VALUES ($1, $2, NOW())", &[&Uuid::new_v4().to_string(), &message_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("collected".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_msg_collection_list_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);
    let rows = client
        .query("SELECT id, message_id, create_time FROM x_message_collection ORDER BY create_time DESC LIMIT $1 OFFSET $2", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("messageId".to_string(), Value::String(row.get("message_id"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn im_msg_collection_remove(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let message_id = req.get("messageId").and_then(|v| v.as_str()).unwrap_or_default();
    let result = client
        .execute("DELETE FROM x_message_collection WHERE message_id = $1", &[&message_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("removed".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_msg_download_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, file_url, file_name, file_size, create_time FROM x_message_file WHERE message_id = $1 LIMIT 1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("fileUrl".to_string(), Value::String(row.get("file_url"))),
                ("fileName".to_string(), Value::String(row.get("file_name"))),
                ("fileSize".to_string(), Value::String(row.get("file_size"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn im_msg_download_id_image_width_width_height_height(
    pool: Extension<Pool>,
    axum::extract::Path((id, width, height)): axum::extract::Path<(String, i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, file_url, file_name, create_time FROM x_message_file WHERE message_id = $1 LIMIT 1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let file_url: String = row.get("file_url");
            let resized_url = format!("{}?w={}&h={}", file_url, width, height);
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("fileUrl".to_string(), Value::String(resized_url)),
                ("fileName".to_string(), Value::String(row.get("file_name"))),
                ("width".to_string(), Value::Number(serde_json::Number::from(width))),
                ("height".to_string(), Value::Number(serde_json::Number::from(height))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn im_msg_list_object(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, conversation_id, content, sender, type, create_time FROM x_message WHERE type != 'text' ORDER BY create_time DESC LIMIT 50", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("conversationId".to_string(), Value::String(row.get("conversation_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn im_msg_list_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);
    let rows = client
        .query("SELECT id, conversation_id, content, sender, type, create_time FROM x_message ORDER BY create_time DESC LIMIT $1 OFFSET $2", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("conversationId".to_string(), Value::String(row.get("conversation_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("page".to_string(), Value::Number(serde_json::Number::from(page))),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn im_msg_revoke_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    client
        .execute("UPDATE x_message SET revoked = true, revoke_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("revoked".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn im_msg_upload_conversationId_type_type(
    pool: Extension<Pool>,
    axum::extract::Path((conversation_id, msg_type)): axum::extract::Path<(String, String)>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let file_url = req.get("fileUrl").and_then(|v| v.as_str()).unwrap_or_default();
    let file_name = req.get("fileName").and_then(|v| v.as_str()).unwrap_or_default();
    let file_size = req.get("fileSize").and_then(|v| v.as_str()).unwrap_or("0");
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system");
    let id = Uuid::new_v4().to_string();

    client
        .execute("INSERT INTO x_message_file (id, message_id, conversation_id, file_url, file_name, file_size, type, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())", &[&id, &id, &conversation_id, &file_url, &file_name, &file_size, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("conversationId".to_string(), Value::String(conversation_id)),
            ("type".to_string(), Value::String(msg_type)),
            ("fileUrl".to_string(), Value::String(file_url.to_string())),
            ("uploaded".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn instant_currentperson_consumed(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, consume_time FROM x_message_consume WHERE consumed = true ORDER BY consume_time DESC LIMIT 50", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("consumeTime".to_string(), Value::String(row.get("consume_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_currentperson_consumed_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, consume_time FROM x_message_consume WHERE consumed = true ORDER BY consume_time DESC", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("consumeTime".to_string(), Value::String(row.get("consume_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_currentperson_consumed_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id_list = req.get("idList").and_then(|v| v.as_array()).map(|arr| {
        arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<String>>()
    }).unwrap_or_default();

    if !id_list.is_empty() {
        client
            .execute("UPDATE x_message_instant SET consumed = true WHERE id = ANY($1)", &[&id_list])
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

pub async fn instant_list_currentperson_consumed_count_count_asc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, consume_time FROM x_message_consume WHERE consumed = true ORDER BY consume_time ASC LIMIT $1", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("consumeTime".to_string(), Value::String(row.get("consume_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_list_currentperson_consumed_count_count_desc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, consume_time FROM x_message_consume WHERE consumed = true ORDER BY consume_time DESC LIMIT $1", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("consumeTime".to_string(), Value::String(row.get("consume_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_list_currentperson_count_count_asc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume ORDER BY create_time ASC LIMIT $1", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_list_currentperson_count_count_desc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume ORDER BY create_time DESC LIMIT $1", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_list_currentperson_noim_count_count_desc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE type != 'im' ORDER BY create_time DESC LIMIT $1", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_list_currentperson_not_consumed_count_count_asc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consumed = false ORDER BY create_time ASC LIMIT $1", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_list_currentperson_not_consumed_count_count_desc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consumed = false ORDER BY create_time DESC LIMIT $1", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE id > $1 ORDER BY create_time ASC LIMIT $2", &[&id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn instant_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE id < $1 ORDER BY create_time DESC LIMIT $2", &[&id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn mass_enable_type(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let msg_type = req.get("type").and_then(|v| v.as_str()).unwrap_or_default();
    let enabled = req.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true);

    client
        .execute("UPDATE x_message_mass SET enabled = $1 WHERE type = $2", &[&enabled, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("type".to_string(), Value::String(msg_type.to_string())),
            ("enabled".to_string(), Value::Bool(enabled)),
        ]),
    ))))
}

pub async fn mass_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, mass_id, content, sender, create_time FROM x_message WHERE mass_id = $1 AND id > $2 ORDER BY create_time ASC LIMIT $3", &[&id, &id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("massId".to_string(), Value::String(row.get("mass_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn mass_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, mass_id, content, sender, create_time FROM x_message WHERE mass_id = $1 AND id < $2 ORDER BY create_time DESC LIMIT $3", &[&id, &id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("massId".to_string(), Value::String(row.get("mass_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn mass_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt("SELECT id, title, content, sender, create_time FROM x_message_mass WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("title".to_string(), Value::String(row.get("title"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("sender".to_string(), Value::String(row.get("sender"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("mass message not found"))),
    }
}

pub async fn mass_id_mockdeletetoget(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_message_mass WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn message_custom_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let conversation_id = req.get("conversationId").and_then(|v| v.as_str()).unwrap_or_default();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system");
    let id = Uuid::new_v4().to_string();

    client
        .execute("INSERT INTO x_message (id, conversation_id, content, sender, type, create_time) VALUES ($1, $2, $3, $4, 'custom', NOW())", &[&id, &conversation_id, &content, &sender])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("conversationId".to_string(), Value::String(conversation_id.to_string())),
            ("content".to_string(), Value::String(content.to_string())),
            ("type".to_string(), Value::String("custom".to_string())),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn message_list_paging_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);
    let rows = client
        .query("SELECT id, conversation_id, content, sender, type, create_time FROM x_message ORDER BY create_time DESC LIMIT $1 OFFSET $2", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("conversationId".to_string(), Value::String(row.get("conversation_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get("sender"))),
            ("type".to_string(), Value::String(row.get("type"))),
            ("createTime".to_string(), Value::String(row.get("create_time"))),
        ]))
    }).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("page".to_string(), Value::Number(serde_json::Number::from(page))),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}
