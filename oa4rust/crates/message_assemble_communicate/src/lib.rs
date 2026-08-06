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

#[derive(Debug, serde::Deserialize)]
pub struct SendRequest {
    pub from: Option<String>,
    pub to: Option<String>,
    pub content: Option<String>,
}

pub async fn send_message(
    axum::extract::Json(req): Json<SendRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("sent".to_string(), Value::Bool(true)),
            ("from".to_string(), Value::String(req.from.unwrap_or_default())),
            ("to".to_string(), Value::String(req.to.unwrap_or_default())),
        ]),
    ))))
}

pub async fn receive_list(
    axum::extract::Path(consume): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("msg-1".to_string())),
            ("consume".to_string(), Value::String(consume)),
            ("status".to_string(), Value::String("unread".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn mark_read(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("marked_read".to_string(), Value::Bool(true)),
    ])))))
}

pub fn message_assemble_communicate_router(pool: Option<Pool>) -> Router {
    let mut router = Router::new()
        .route("/jaxrs/message/assemble/communicate/send", post(send_message))
        .route("/jaxrs/message/assemble/communicate/receive/{consume}", get(receive_list))
        .route("/jaxrs/message/assemble/communicate/mark_read/{id}", post(mark_read))
        .route("/jaxrs/message/assemble/communicate/consume/list/{consume}/count/{count}", get(stub_message_assemble_communicate_consume_list_consume_count_count))
        .route("/jaxrs/message/assemble/communicate/consume/list/{consume}/currentperson/count/{count}", get(stub_message_assemble_communicate_consume_list_consume_currentperson_count_count))
        .route("/jaxrs/message/assemble/communicate/consume/list/{consume}/person/{person}/count/{count}", get(stub_message_assemble_communicate_consume_list_consume_person_person_count_count))
        .route("/jaxrs/message/assemble/communicate/consume/type/{type}", get(stub_message_assemble_communicate_consume_type_type))
        .route("/jaxrs/message/assemble/communicate/consume/type/{type}/mockputtopost", post(stub_message_assemble_communicate_consume_type_type_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/consume/{id}/type/{type}", post(stub_message_assemble_communicate_consume_id_type_type))
        .route("/jaxrs/message/assemble/communicate/im/conversation", post(stub_message_assemble_communicate_im_conversation))
        .route("/jaxrs/message/assemble/communicate/im/conversation/business/{businessId}", get(stub_message_assemble_communicate_im_conversation_business_businessId))
        .route("/jaxrs/message/assemble/communicate/im/conversation/list/my", get(stub_message_assemble_communicate_im_conversation_list_my))
        .route("/jaxrs/message/assemble/communicate/im/conversation/list/with/person", get(stub_message_assemble_communicate_im_conversation_list_with_person))
        .route("/jaxrs/message/assemble/communicate/im/conversation/mockputtopost", post(stub_message_assemble_communicate_im_conversation_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}", get(stub_message_assemble_communicate_im_conversation_id))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group", get(stub_message_assemble_communicate_im_conversation_id_group))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget", delete(stub_message_assemble_communicate_im_conversation_id_group_mockdeletetoget))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/group/quit/self", post(stub_message_assemble_communicate_im_conversation_id_group_quit_self))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/icon", get(stub_message_assemble_communicate_im_conversation_id_icon))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/read", post(stub_message_assemble_communicate_im_conversation_id_read))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/read/mockputtopost", post(stub_message_assemble_communicate_im_conversation_id_read_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/single", get(stub_message_assemble_communicate_im_conversation_id_single))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/single/mockdeletetoget", delete(stub_message_assemble_communicate_im_conversation_id_single_mockdeletetoget))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel", post(stub_message_assemble_communicate_im_conversation_id_top_cancel))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel/mockputtopost", post(stub_message_assemble_communicate_im_conversation_id_top_cancel_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/set", post(stub_message_assemble_communicate_im_conversation_id_top_set))
        .route("/jaxrs/message/assemble/communicate/im/conversation/{id}/top/set/mockputtopost", post(stub_message_assemble_communicate_im_conversation_id_top_set_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/im/manager/config", get(stub_message_assemble_communicate_im_manager_config))
        .route("/jaxrs/message/assemble/communicate/im/msg", post(stub_message_assemble_communicate_im_msg))
        .route("/jaxrs/message/assemble/communicate/im/msg/clear", post(stub_message_assemble_communicate_im_msg_clear))
        .route("/jaxrs/message/assemble/communicate/im/msg/collection", post(stub_message_assemble_communicate_im_msg_collection))
        .route("/jaxrs/message/assemble/communicate/im/msg/collection/list/{page}/size/{size}", get(stub_message_assemble_communicate_im_msg_collection_list_page_size_size))
        .route("/jaxrs/message/assemble/communicate/im/msg/collection/remove", post(stub_message_assemble_communicate_im_msg_collection_remove))
        .route("/jaxrs/message/assemble/communicate/im/msg/download/{id}", get(stub_message_assemble_communicate_im_msg_download_id))
        .route("/jaxrs/message/assemble/communicate/im/msg/download/{id}/image/width/{width}/height/{height}", get(stub_message_assemble_communicate_im_msg_download_id_image_width_width_height_height))
        .route("/jaxrs/message/assemble/communicate/im/msg/list/object", get(stub_message_assemble_communicate_im_msg_list_object))
        .route("/jaxrs/message/assemble/communicate/im/msg/list/{page}/size/{size}", get(stub_message_assemble_communicate_im_msg_list_page_size_size))
        .route("/jaxrs/message/assemble/communicate/im/msg/revoke/{id}", post(stub_message_assemble_communicate_im_msg_revoke_id))
        .route("/jaxrs/message/assemble/communicate/im/msg/upload/{conversationId}/type/{type}", post(stub_message_assemble_communicate_im_msg_upload_conversationId_type_type))
        .route("/jaxrs/message/assemble/communicate/instant/currentperson/consumed", get(stub_message_assemble_communicate_instant_currentperson_consumed))
        .route("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/all", get(stub_message_assemble_communicate_instant_currentperson_consumed_all))
        .route("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost", post(stub_message_assemble_communicate_instant_currentperson_consumed_mockputtopost))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/asc", get(stub_message_assemble_communicate_instant_list_currentperson_consumed_count_count_asc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/desc", get(stub_message_assemble_communicate_instant_list_currentperson_consumed_count_count_desc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/asc", get(stub_message_assemble_communicate_instant_list_currentperson_count_count_asc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/desc", get(stub_message_assemble_communicate_instant_list_currentperson_count_count_desc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/noim/count/{count}/desc", get(stub_message_assemble_communicate_instant_list_currentperson_noim_count_count_desc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/asc", get(stub_message_assemble_communicate_instant_list_currentperson_not_consumed_count_count_asc))
        .route("/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/desc", get(stub_message_assemble_communicate_instant_list_currentperson_not_consumed_count_count_desc))
        .route("/jaxrs/message/assemble/communicate/instant/list/{id}/next/{count}", get(stub_message_assemble_communicate_instant_list_id_next_count))
        .route("/jaxrs/message/assemble/communicate/instant/list/{id}/prev/{count}", get(stub_message_assemble_communicate_instant_list_id_prev_count))
        .route("/jaxrs/message/assemble/communicate/mass/enable/type", post(stub_message_assemble_communicate_mass_enable_type))
        .route("/jaxrs/message/assemble/communicate/mass/list/{id}/next/{count}", get(stub_message_assemble_communicate_mass_list_id_next_count))
        .route("/jaxrs/message/assemble/communicate/mass/list/{id}/prev/{count}", get(stub_message_assemble_communicate_mass_list_id_prev_count))
        .route("/jaxrs/message/assemble/communicate/mass/{id}", get(stub_message_assemble_communicate_mass_id))
        .route("/jaxrs/message/assemble/communicate/mass/{id}/mockdeletetoget", delete(stub_message_assemble_communicate_mass_id_mockdeletetoget))
        .route("/jaxrs/message/assemble/communicate/message/custom/create", post(stub_message_assemble_communicate_message_custom_create))
        .route("/jaxrs/message/assemble/communicate/message/list/paging/{page}/size/{size}", get(stub_message_assemble_communicate_message_list_paging_page_size_size));

    if let Some(pool) = pool {
        router = router.layer(Extension(pool));
    }

    router
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    message_assemble_communicate_router(Some(pool))
}


/// Stub handler for /jaxrs/message/assemble/communicate/consume/list/{consume}/count/{count}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_consume_list_consume_count_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((consume, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/consume/list/{consume}/currentperson/count/{count}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_consume_list_consume_currentperson_count_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((consume, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/consume/list/{consume}/person/{person}/count/{count}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_consume_list_consume_person_person_count_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((consume, person, count)): axum::extract::Path<(String, String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/consume/type/{type}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_consume_type_type(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(msg_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/consume/type/{type}/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_consume_type_type_mockputtopost(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(msg_type): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = Uuid::new_v4().to_string();
    client
        .execute("INSERT INTO x_message_consume (id, consume, content, type, create_time) VALUES ($1, $2, '', $3, NOW())", &[&id, &msg_type, &msg_type])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("saved".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/consume/{id}/type/{type}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_consume_id_type_type(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((id, msg_type)): axum::extract::Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/business/{businessId}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_business_businessId(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(business_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/list/my
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_list_my(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/list/with/person
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_list_with_person(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_mockputtopost(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/group
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_group(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_group_mockdeletetoget(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(result > 0))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/group/quit/self
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_group_quit_self(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("DELETE FROM x_message_conversation_member WHERE conversation_id = $1 AND person_id = $2", &[&id, &""])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("quit".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/icon
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_icon(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/read
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_read(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("UPDATE x_message_conversation SET read_status = 'read', read_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("read".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/read/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_read_mockputtopost(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("UPDATE x_message_conversation SET read_status = 'read', read_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("read".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/single
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_single(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/single/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_single_mockdeletetoget(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute("DELETE FROM x_message_conversation WHERE id = $1 AND type = 'single'", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(result > 0))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_top_cancel(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("UPDATE x_message_conversation SET top = false WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topCancelled".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_top_cancel_mockputtopost(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("UPDATE x_message_conversation SET top = false WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topCancelled".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/top/set
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_top_set(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("UPDATE x_message_conversation SET top = true, top_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topSet".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/conversation/{id}/top/set/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_conversation_id_top_set_mockputtopost(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("UPDATE x_message_conversation SET top = true, top_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("topSet".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/manager/config
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_manager_config(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/clear
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_clear(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(conversation_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("UPDATE x_message SET cleared = true WHERE conversation_id = $1", &[&conversation_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("cleared".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/collection
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_collection(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let message_id = req.get("messageId").and_then(|v| v.as_str()).unwrap_or_default();
    client
        .execute("INSERT INTO x_message_collection (id, message_id, create_time) VALUES ($1, $2, NOW())", &[&Uuid::new_v4().to_string(), &message_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("collected".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/collection/list/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_collection_list_page_size_size(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/collection/remove
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_collection_remove(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let message_id = req.get("messageId").and_then(|v| v.as_str()).unwrap_or_default();
    let result = client
        .execute("DELETE FROM x_message_collection WHERE message_id = $1", &[&message_id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("removed".to_string(), Value::Bool(result > 0))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/download/{id}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_download_id(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/download/{id}/image/width/{width}/height/{height}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_download_id_image_width_width_height_height(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((id, width, height)): axum::extract::Path<(String, i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/list/object
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_list_object(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/list/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_list_page_size_size(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/revoke/{id}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_revoke_id(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    client
        .execute("UPDATE x_message SET revoked = true, revoke_time = NOW() WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("revoked".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/im/msg/upload/{conversationId}/type/{type}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_im_msg_upload_conversationId_type_type(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((conversation_id, msg_type)): axum::extract::Path<(String, String)>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/currentperson/consumed
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_currentperson_consumed(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/currentperson/consumed/all
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_currentperson_consumed_all(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_currentperson_consumed_mockputtopost(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("success".to_string(), Value::Bool(true))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/asc
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_currentperson_consumed_count_count_asc(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/desc
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_currentperson_consumed_count_count_desc(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/asc
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_currentperson_count_count_asc(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/desc
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_currentperson_count_count_desc(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/currentperson/noim/count/{count}/desc
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_currentperson_noim_count_count_desc(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/asc
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_currentperson_not_consumed_count_count_asc(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/desc
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_currentperson_not_consumed_count_count_desc(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(count): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/{id}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_id_next_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/instant/list/{id}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_instant_list_id_prev_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/mass/enable/type
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_mass_enable_type(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/mass/list/{id}/next/{count}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_mass_list_id_next_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/mass/list/{id}/prev/{count}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_mass_list_id_prev_count(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((id, count)): axum::extract::Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/mass/{id}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_mass_id(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/mass/{id}/mockdeletetoget
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_mass_id_mockdeletetoget(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute("DELETE FROM x_message_mass WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(result > 0))]),
    ))))
}

/// Stub handler for /jaxrs/message/assemble/communicate/message/custom/create
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_message_custom_create(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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

/// Stub handler for /jaxrs/message/assemble/communicate/message/list/paging/{page}/size/{size}
/// TODO: Implement real business logic
pub async fn stub_message_assemble_communicate_message_list_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    axum::extract::Path((page, size)): axum::extract::Path<(i64, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

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
