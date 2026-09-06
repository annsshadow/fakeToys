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
#[cfg(test)]
mod tests_u2;

pub async fn send_message(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let conversation_id = req.get("\"conversationId\"").and_then(|v| v.as_str()).unwrap_or_default();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system");
    let msg_type = req.get("type").and_then(|v| v.as_str()).unwrap_or("text");
    let id = Uuid::new_v4().to_string();

    let result = client
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
            ("\"conversationId\"".to_string(), Value::String(conversation_id.to_string())),
            ("content".to_string(), Value::String(content.to_string())),
            ("sender".to_string(), Value::String(sender.to_string())),
            ("type".to_string(), Value::String(msg_type.to_string())),
            ("sent".to_string(), Value::Bool(result > 0)),
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
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn mark_read(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_message_consume SET consumed = true WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("marked_read".to_string(), Value::Bool(result > 0))]),
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
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consume = $1 ORDER BY create_time DESC LIMIT $2::int", &[&consume, &limit])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn consume_list_consume_currentperson_count_count(
    pool: Extension<Pool>,
    axum::extract::Path((consume, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let limit = count.max(1) as i64;
    let rows = client
        .query("SELECT id, consume, content, sender, read_status, create_time FROM x_message_consume WHERE consume = $1 AND sender = consume ORDER BY create_time DESC LIMIT $2::int", &[&consume, &limit])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("readStatus".to_string(), Value::String(row.get("read_status"))),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn consume_list_consume_person_person_count_count(
    pool: Extension<Pool>,
    axum::extract::Path((consume, person, count)): axum::extract::Path<(String, String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let limit = count.max(1) as i64;
    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consume = $1 AND sender = $2 ORDER BY create_time DESC LIMIT $3::int", &[&consume, &person, &limit])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn consume_type_type_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(msg_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = Uuid::new_v4().to_string();
    let result = client
        .execute("INSERT INTO x_message_consume (id, consume, content, type, create_time) VALUES ($1, $2, '', $3, NOW())", &[&id, &msg_type, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(result > 0))]),
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
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(result > 0))]),
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

    let result = client
        .execute("INSERT INTO x_message_conversation (id, name, type, create_time) VALUES ($1, $2, $3, NOW())", &[&id, &name, &conversation_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name.to_string())),
            ("type".to_string(), Value::String(conversation_type.to_string())),
            ("created".to_string(), Value::Bool(result > 0)),
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
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
                ("businessId".to_string(), Value::String(row.get("business_id"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
            ("lastMessage".to_string(), Value::String(row.get::<_, Option<String>>("last_message").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
            ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn im_conversation_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = req.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let title = req.get("title").and_then(|v| v.as_str()).unwrap_or_default();
    let note = req.get("note").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute("UPDATE x_message_conversation SET title = COALESCE($2, title), note = COALESCE($3, note), update_time = NOW() WHERE id = $1", &[&id, &title, &note])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("updated".to_string(), Value::Bool(result > 0))]),
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
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
                ("lastMessage".to_string(), Value::String(row.get::<_, Option<String>>("last_message").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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
            ("\"conversationId\"".to_string(), Value::String(row.get("conversation_id"))),
            ("personId".to_string(), Value::String(row.get("person_id"))),
            ("role".to_string(), Value::String(row.get("role"))),
            ("joinTime".to_string(), Value::String(row.get("join_time"))),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn im_manager_config_post(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let config_key = req.get("configKey").and_then(|v| v.as_str()).unwrap_or_default();
    let config_value = req.get("configValue").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute("UPDATE x_message_config SET config_key = $1, config_value = $2, update_time = NOW() WHERE id = (SELECT id FROM x_message_config ORDER BY create_time DESC LIMIT 1)", &[&config_key, &config_value])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        let id = Uuid::new_v4().to_string();
        client
            .execute("INSERT INTO x_message_config (id, config_key, config_value, create_time) VALUES ($1, $2, $3, NOW())", &[&id, &config_key, &config_value])
            .await
            .map_err(|_| AppError::Internal)?;
    }

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
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("config not found"))),
    }
}

pub async fn im_conversation_update(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.get("name").and_then(|v| v.as_str());
    let conversation_type = req.get("type").and_then(|v| v.as_str());

    let result = client
        .execute("UPDATE x_message_conversation SET name = COALESCE($2, name), type = COALESCE($3, type), update_time = NOW() WHERE id = $1", &[&id, &name, &conversation_type])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("conversation not found")));
    }

    let row = client
        .query_opt("SELECT id, name, type, create_time FROM x_message_conversation WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("conversation not found"))),
    }
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
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // Java 仅允许群聊退出；IDOR：person 取自会话，只能退自己所在的群
    let conv_type = conversation_type(&client, &id).await?;
    if conv_type.as_deref() != Some("group") {
        return Ok(Json(ActionResult::error("conversation not found or not a group")));
    }
    if !is_conversation_member(&client, &id, &session.person_unique).await? {
        return Ok(Json(ActionResult::error("not a conversation member")));
    }

    let result = client
        .execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1 AND person_id = $2", &[&id, &session.person_unique])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("quit".to_string(), Value::Bool(result > 0))]),
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
                ("\"conversationId\"".to_string(), Value::String(id)),
                ("iconUrl".to_string(), Value::String(row.get("icon_url"))),
                ("iconName".to_string(), Value::String(row.get("icon_name"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("icon not found"))),
    }
}

pub async fn im_conversation_id_read(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // Java ActionConversationRead 校验会话成员；IDOR：仅成员可标记已读
    if !is_conversation_member(&client, &id, &session.person_unique).await? {
        return Ok(Json(ActionResult::error("not a conversation member")));
    }

    let result = client
        .execute("UPDATE x_message_conversation SET read_status = 'read', read_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("read".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_conversation_id_read_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_message_conversation SET read_status = 'read', read_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("read".to_string(), Value::Bool(result > 0))]),
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
                ("name".to_string(), Value::String(row.get::<_, Option<String>>("name").unwrap_or_default())),
                ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("single conversation not found"))),
    }
}

pub async fn im_conversation_id_single_mockdeletetoget(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // IDOR：仅会话成员可删除该单聊
    if !is_conversation_member(&client, &id, &session.person_unique).await? {
        return Ok(Json(ActionResult::error("not a conversation member")));
    }

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

    let result = client
        .execute("UPDATE x_message_conversation SET top = false WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topCancelled".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_conversation_id_top_cancel_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_message_conversation SET top = false WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topCancelled".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_conversation_id_top_set(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_message_conversation SET top = true, top_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topSet".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_conversation_id_top_set_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_message_conversation SET top = true, top_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topSet".to_string(), Value::Bool(result > 0))]),
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
                ("\"configKey\"".to_string(), Value::String(row.get("config_key"))),
                ("\"configValue\"".to_string(), Value::String(row.get("config_value"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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

    let conversation_id = req.get("\"conversationId\"").and_then(|v| v.as_str()).unwrap_or_default();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system");
    let msg_type = req.get("type").and_then(|v| v.as_str()).unwrap_or("text");
    let id = Uuid::new_v4().to_string();

    let result = client
        .execute("INSERT INTO x_message (id, conversation_id, content, sender, type, create_time) VALUES ($1, $2, $3, $4, $5, NOW())", &[&id, &conversation_id, &content, &sender, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("\"conversationId\"".to_string(), Value::String(conversation_id.to_string())),
            ("content".to_string(), Value::String(content.to_string())),
            ("sender".to_string(), Value::String(sender.to_string())),
            ("type".to_string(), Value::String(msg_type.to_string())),
            ("sent".to_string(), Value::Bool(result > 0)),
        ]),
    ))))
}

pub async fn im_msg_clear(
    pool: Extension<Pool>,
    axum::extract::Path(conversation_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_message SET cleared = true WHERE conversation_id = $1", &[&conversation_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("cleared".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_msg_collection(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let message_id = req.get("\"messageId\"").and_then(|v| v.as_str()).unwrap_or_default();
    let result = client
        .execute("INSERT INTO x_message_collection (id, message_id, create_time) VALUES ($1, $2, NOW())", &[&Uuid::new_v4().to_string(), &message_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("collected".to_string(), Value::Bool(result > 0))]),
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
        .query("SELECT id, message_id, create_time FROM x_message_collection ORDER BY create_time DESC LIMIT $1::int OFFSET $2::int", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("\"messageId\"".to_string(), Value::String(row.get("message_id"))),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn im_msg_collection_remove(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let message_id = req.get("\"messageId\"").and_then(|v| v.as_str()).unwrap_or_default();
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
                ("\"fileUrl\"".to_string(), Value::String(row.get("file_url"))),
                ("\"fileName\"".to_string(), Value::String(row.get("file_name"))),
                ("\"fileSize\"".to_string(), Value::String(row.get("file_size"))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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
                ("\"fileUrl\"".to_string(), Value::String(resized_url)),
                ("\"fileName\"".to_string(), Value::String(row.get("file_name"))),
                ("width".to_string(), Value::Number(serde_json::Number::from(width))),
                ("height".to_string(), Value::Number(serde_json::Number::from(height))),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
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
            ("\"conversationId\"".to_string(), Value::String(row.get("conversation_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn im_msg_list_page_size_size(
    pool: Extension<Pool>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let offset = ((page.max(1) - 1) * size).max(0);
    let limit = size.max(1);
    let rows = client
        .query("SELECT id, conversation_id, content, sender, type, create_time FROM x_message ORDER BY create_time DESC LIMIT $1::int OFFSET $2::int", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("\"conversationId\"".to_string(), Value::String(row.get("conversation_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}

pub async fn im_msg_revoke_id(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("UPDATE x_message SET revoked = true, revoke_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("revoked".to_string(), Value::Bool(result > 0))]),
    ))))
}

pub async fn im_msg_upload_conversationId_type_type(
    pool: Extension<Pool>,
    axum::extract::Path((conversation_id, msg_type)): axum::extract::Path<(String, String)>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let file_url = req.get("\"fileUrl\"").and_then(|v| v.as_str()).unwrap_or_default();
    let file_name = req.get("\"fileName\"").and_then(|v| v.as_str()).unwrap_or_default();
    let file_size = req.get("\"fileSize\"").and_then(|v| v.as_str()).unwrap_or("0");
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system");
    let id = Uuid::new_v4().to_string();

    let result = client
        .execute("INSERT INTO x_message_file (id, message_id, conversation_id, file_url, file_name, file_size, type, create_time) VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())", &[&id, &id, &conversation_id, &file_url, &file_name, &file_size, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("\"conversationId\"".to_string(), Value::String(conversation_id)),
            ("type".to_string(), Value::String(msg_type)),
            ("\"fileUrl\"".to_string(), Value::String(file_url.to_string())),
            ("uploaded".to_string(), Value::Bool(result > 0)),
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
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("consumeTime".to_string(), Value::String(row.get::<_, Option<String>>("consume_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("consumeTime".to_string(), Value::String(row.get::<_, Option<String>>("consume_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_currentperson_consumed_mockputtopost(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id_list = req.get("\"idList\"").and_then(|v| v.as_array()).map(|arr| {
        arr.iter().filter_map(|v| v.as_str().map(String::from)).collect::<Vec<String>>()
    }).unwrap_or_default();

    let success = id_list.is_empty();
    let result = if !id_list.is_empty() {
        Some(client
            .execute("UPDATE x_message_instant SET consumed = true WHERE id = ANY($1)", &[&id_list])
            .await
            .map_err(|_| AppError::Internal)?)
    } else {
        None
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(success || result.unwrap_or(0) > 0))]),
    ))))
}

pub async fn instant_list_currentperson_consumed_count_count_asc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, consume_time FROM x_message_consume WHERE consumed = true ORDER BY consume_time ASC LIMIT $1::int", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("consumeTime".to_string(), Value::String(row.get("consume_time"))),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_list_currentperson_consumed_count_count_desc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, consume_time FROM x_message_consume WHERE consumed = true ORDER BY consume_time DESC LIMIT $1::int", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("consumeTime".to_string(), Value::String(row.get("consume_time"))),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_list_currentperson_count_count_asc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume ORDER BY create_time ASC LIMIT $1::int", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_list_currentperson_count_count_desc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume ORDER BY create_time DESC LIMIT $1::int", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_list_currentperson_noim_count_count_desc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE type != 'im' ORDER BY create_time DESC LIMIT $1::int", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_list_currentperson_not_consumed_count_count_asc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consumed = false ORDER BY create_time ASC LIMIT $1::int", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_list_currentperson_not_consumed_count_count_desc(
    pool: Extension<Pool>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE consumed = false ORDER BY create_time DESC LIMIT $1::int", &[&count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_list_id_next_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE id > $1 ORDER BY create_time ASC LIMIT $2::int", &[&id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn instant_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, consume, content, sender, create_time FROM x_message_consume WHERE id < $1 ORDER BY create_time DESC LIMIT $2::int", &[&id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("consume".to_string(), Value::String(row.get("consume"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
        .query("SELECT id, mass_id, content, sender, create_time FROM x_message WHERE mass_id = $1 AND id > $2 ORDER BY create_time ASC LIMIT $3::int", &[&id, &id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("massId".to_string(), Value::String(row.get("mass_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

pub async fn mass_list_id_prev_count(
    pool: Extension<Pool>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query("SELECT id, mass_id, content, sender, create_time FROM x_message WHERE mass_id = $1 AND id < $2 ORDER BY create_time DESC LIMIT $3::int", &[&id, &id, &count])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("massId".to_string(), Value::String(row.get("mass_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
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
                ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
                ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("mass message not found"))),
    }
}

pub async fn message_custom_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let conversation_id = req.get("\"conversationId\"").and_then(|v| v.as_str()).unwrap_or_default();
    let content = req.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system");
    let id = Uuid::new_v4().to_string();

    let result = client
        .execute("INSERT INTO x_message (id, conversation_id, content, sender, type, create_time) VALUES ($1, $2, $3, $4, 'custom', NOW())", &[&id, &conversation_id, &content, &sender])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("\"conversationId\"".to_string(), Value::String(conversation_id.to_string())),
            ("content".to_string(), Value::String(content.to_string())),
            ("type".to_string(), Value::String("custom".to_string())),
            ("created".to_string(), Value::Bool(result > 0)),
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
        .query("SELECT id, conversation_id, content, sender, type, create_time FROM x_message ORDER BY create_time DESC LIMIT $1::int OFFSET $2::int", &[&limit, &offset])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(|row| {
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            ("\"conversationId\"".to_string(), Value::String(row.get("conversation_id"))),
            ("content".to_string(), Value::String(row.get("content"))),
            ("sender".to_string(), Value::String(row.get::<_, Option<String>>("sender").unwrap_or_default())),
            ("type".to_string(), Value::String(row.get::<_, Option<String>>("type").unwrap_or_default())),
            ("createTime".to_string(), Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
        ]))
    }).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, size)))
}

// ══════════════════════════════════════════════════════════════════
// plan002 U2 — Java 对齐缺口端点（connector / ws / mass 家族 + 动词补齐）
//
// 表：x_message_ws_session / x_message_conversation_ext（migration 063 幂等
// 补建），其余沿用既有表。写操作按 IDOR 门禁：
//   - 管理资源（mass 群发创建/删除）一律 require_admin（Java 要求
//     Manager/MessageManager 角色），is_admin 对不可用 DB fail-closed；
//   - 会话内个人操作（退群/已读/单聊删除）person_unique 取自会话，
//     操作前校验成员身份，禁止代他人操作。
// ══════════════════════════════════════════════════════════════════

async fn require_admin(
    pool: &Pool,
    session: &shared::session::Session,
) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

async fn is_conversation_member(
    client: &deadpool_postgres::Client,
    conversation_id: &str,
    person_unique: &str,
) -> Result<bool, AppError> {
    let member = client
        .query_opt(
            "SELECT 1 FROM x_message_conversation_member WHERE conversation_id = $1 AND person_id = $2",
            &[&conversation_id, &person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(member.is_some())
}

async fn conversation_type(
    client: &deadpool_postgres::Client,
    conversation_id: &str,
) -> Result<Option<String>, AppError> {
    let row = client
        .query_opt("SELECT type FROM x_message_conversation WHERE id = $1", &[&conversation_id])
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.map(|r| r.get::<_, Option<String>>("type").unwrap_or_default()))
}

/// POST /connector — Java ActionCreate：先落 Instant(consumed=false)，
/// 再为每个启用的 consumer 展开一条 Message 落库。
pub async fn connector_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let msg_type = req.get("type").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let person = req.get("person").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let title = req.get("title").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let body = match req.get("body") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let instant_id = Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_message_instant (id, body, type, person, title, consumed, create_time) VALUES ($1, $2, $3, $4, $5, false, NOW())",
            &[&instant_id, &body, &msg_type, &person, &title],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    // 启用的 consumer 渠道来自消息配置；无配置时仅保留 Instant 落库。
    let consumers = client
        .query(
            "SELECT DISTINCT consume FROM x_message_config WHERE enabled = true AND consume IS NOT NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut dispatched: u64 = 0;
    for row in consumers {
        let consumer: String = row.get::<_, Option<String>>("consume").unwrap_or_default();
        if consumer.is_empty() {
            continue;
        }
        let message_id = Uuid::new_v4().to_string();
        dispatched += client
            .execute(
                "INSERT INTO x_message (id, content, sender, type, create_time) VALUES ($1, $2, $3, $4, NOW())",
                &[&message_id, &body, &person, &consumer],
            )
            .await
            .map_err(|_| AppError::Internal)?;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("value".to_string(), Value::Bool(true)),
            ("instantId".to_string(), Value::String(instant_id)),
            ("messages".to_string(), Value::Number(serde_json::Number::from(dispatched))),
        ]),
    ))))
}

/// POST /ws — Java ActionCreate：仅向当前打开的 ws 连接投递；
/// 有在线连接时落 ws 消费记录，返回 value=true，否则如实返回 false。
pub async fn ws_create(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let person = req.get("person").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let sender = req.get("sender").and_then(|v| v.as_str()).unwrap_or("system").to_string();
    let body = match req.get("body") {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let open = client
        .query_opt(
            "SELECT 1 FROM x_message_ws_session WHERE person = $1 AND disconnected_at IS NULL LIMIT 1",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut delivered = false;
    if open.is_some() {
        let id = Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_message_consume (id, consume, content, sender, consumed, create_time) VALUES ($1, 'ws', $2, $3, false, NOW())",
                &[&id, &body, &sender],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        delivered = true;
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("value".to_string(), Value::Bool(delivered))]),
    ))))
}

/// GET /ws/count/person — 当前在线（未断开）ws 连接的去重人数。
pub async fn ws_count_person(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count: i64 = client
        .query_one(
            "SELECT COUNT(DISTINCT person) AS cnt FROM x_message_ws_session WHERE disconnected_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

/// GET /ws/list/person/current/node — 本节点在线人员列表。
pub async fn ws_list_person_current_node(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT DISTINCT person FROM x_message_ws_session WHERE disconnected_at IS NULL ORDER BY person",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("person".to_string(), Value::String(row.get("person"))),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// GET /ws/list/person — 按节点分组的在线人员列表。
pub async fn ws_list_person(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT node, person FROM x_message_ws_session WHERE disconnected_at IS NULL ORDER BY node, person",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut groups: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
    for row in &rows {
        let node: String = row.get::<_, Option<String>>("node").unwrap_or_else(|| "local".to_string());
        let person: String = row.get("person");
        groups.entry(node).or_default().push(person);
    }

    let data: Vec<Value> = groups
        .into_iter()
        .map(|(node, people)| {
            let list: Vec<Value> = people.into_iter().map(Value::String).collect();
            Value::Object(serde_json::Map::from_iter([
                ("node".to_string(), Value::String(node)),
                ("personList".to_string(), Value::Array(list)),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
}

/// 群发目标人群：personList/identityList/groupList/unitList 合并去重。
fn mass_target_list(req: &Value) -> Vec<String> {
    let mut targets: Vec<String> = Vec::new();
    for key in ["personList", "identityList", "groupList", "unitList"] {
        if let Some(arr) = req.get(key).and_then(|v| v.as_array()) {
            for v in arr {
                if let Some(s) = v.as_str() {
                    if !s.is_empty() && !targets.iter().any(|t| t == s) {
                        targets.push(s.to_string());
                    }
                }
            }
        }
    }
    targets
}

/// POST /mass — Java ActionCreate：需 Manager/MessageManager 角色，
/// 目标人群与 body 必填，落 Mass 记录（creator_person 取自会话）。
pub async fn mass_create(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // IDOR：权限门禁先于任何资源获取/写操作，fail-closed
    require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let targets = mass_target_list(&req);
    if targets.is_empty() {
        return Ok(Json(ActionResult::error("empty target")));
    }
    let body = req.get("body").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    if body.is_empty() {
        return Ok(Json(ActionResult::error("empty body")));
    }

    let id = Uuid::new_v4().to_string();
    let msg_type = req.get("type").and_then(|v| v.as_str()).unwrap_or("dingding").to_string();
    let title = req.get("title").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let send_person_list = targets.join(",");

    client
        .execute(
            "INSERT INTO x_message_mass (id, title, content, body, type, send_person_list, creator_person, enabled, create_time) \
             VALUES ($1, $2, $3, $3, $4, $5, $6, true, NOW())",
            &[&id, &title, &body, &msg_type, &send_person_list, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("type".to_string(), Value::String(msg_type)),
            ("targetCount".to_string(), Value::Number(serde_json::Number::from(targets.len() as i64))),
        ]),
    ))))
}

/// GET /mass/enable/type — 已启用群发渠道列表。
pub async fn mass_enable_type_get(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT DISTINCT type FROM x_message_mass WHERE enabled = true AND type IS NOT NULL ORDER BY type",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .filter_map(|row| row.get::<_, Option<String>>("type").map(Value::String))
        .collect();

    let count = data.len() as i64; Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

/// DELETE /mass/{id} 与 GET /mass/{id}/mockdeletetoget 共用：
/// Java ActionDelete 需 Manager/MessageManager 角色，删除前校验存在性。
pub async fn mass_id_mockdeletetoget(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // IDOR：权限门禁先于任何资源获取/删除操作，fail-closed
    require_admin(&pool, &session).await?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let exists = client
        .query_opt("SELECT id FROM x_message_mass WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;
    if exists.is_none() {
        return Ok(Json(ActionResult::error("mass message not found")));
    }

    client
        .execute("DELETE FROM x_message_mass WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// DELETE /im/conversation/{id}/single（及 GET mockdeletetoget）—
/// Java ActionDeleteSingleConversationVirtual：单聊虚拟删除（per-person ext 置位）。
pub async fn im_conversation_id_single_delete_virtual(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let conv_type = conversation_type(&client, &id).await?;
    match conv_type.as_deref() {
        None => return Ok(Json(ActionResult::error("conversation not found"))),
        Some(t) if t != "single" => {
            return Ok(Json(ActionResult::error("only single conversation can be deleted")))
        }
        _ => {}
    }

    // IDOR：只能虚拟删除自己所在的会话
    if !is_conversation_member(&client, &id, &session.person_unique).await? {
        return Ok(Json(ActionResult::error("not a conversation member")));
    }

    let ext = client
        .query_opt(
            "SELECT id FROM x_message_conversation_ext WHERE conversation_id = $1 AND person = $2",
            &[&id, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match ext {
        Some(row) => {
            let ext_id: String = row.get("id");
            client
                .execute(
                    "UPDATE x_message_conversation_ext SET is_deleted = true, last_delete_time = NOW(), last_read_time = NOW(), update_time = NOW() WHERE id = $1",
                    &[&ext_id],
                )
                .await
                .map_err(|_| AppError::Internal)?;
        }
        None => {
            let ext_id = Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_message_conversation_ext (id, conversation_id, person, is_deleted, last_delete_time, last_read_time, create_time, update_time) \
                     VALUES ($1, $2, $3, true, NOW(), NOW(), NOW(), NOW())",
                    &[&ext_id, &id, &session.person_unique],
                )
                .await
                .map_err(|_| AppError::Internal)?;
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("value".to_string(), Value::Bool(true))]),
    ))))
}

/// PUT /instant/currentperson/consumed — Java PUT：将当前人员的 instant 标记已消费。
pub async fn instant_currentperson_consumed_put(
    pool: Extension<Pool>,
    session: Extension<shared::session::Session>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let marked = client
        .execute(
            "UPDATE x_message_instant SET consumed = true WHERE person = $1 AND consumed = false",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("value".to_string(), Value::Bool(true)),
            ("marked".to_string(), Value::Number(serde_json::Number::from(marked as i64))),
        ]),
    ))))
}
