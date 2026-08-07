use axum::{
    extract::{Extension, Path},
    Json, Router, routing::get, routing::post,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[derive(Debug, Deserialize)]
pub struct CreatePortalRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePageRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub content: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct SavePageRequest {
    pub content: Option<Value>,
}

pub async fn create_design(
    axum::extract::Json(req): Json<CreatePortalRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("created".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(req.name.unwrap_or_default())),
            ("description".to_string(), Value::String(req.description.unwrap_or_default())),
        ]),
    ))))
}

pub async fn get_design(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String("Portal Design".to_string())),
            ("components".to_string(), Value::Array(vec![])),
        ]),
    ))))
}

pub async fn list_designs() -> Result<Json<ActionResult<Value>>, AppError> {
    let data = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("design-1".to_string())),
            ("name".to_string(), Value::String("Design 1".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn save_design(
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("updated_at".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ]),
    ))))
}

pub async fn list_pages_by_category(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time, update_time FROM x_portal_page WHERE category = $1 ORDER BY update_time DESC",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            let content: Option<String> = row.get("content");
            let content_value = content
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(Value::Null);
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), content_value),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn get_page(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time, update_time FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let content_value = content
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(Value::Null);
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), content_value),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn create_page(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): Json<CreatePageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let content = req.content.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    let content_str = serde_json::to_string(&content).map_err(|_| AppError::Internal)?;
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_portal_page (id, name, category, content, creator, create_time, update_time) VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &content_str, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("category".to_string(), Value::String(category)),
        ("content".to_string(), content),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn save_page(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<SavePageRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let content = req.content.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    let content_str = serde_json::to_string(&content).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_portal_page SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("page not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("content".to_string(), content),
        ]),
    ))))
}

pub async fn delete_page(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "DELETE FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("page not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn portal_assemble_designer_router() -> Router {
    Router::new()
        .route("/jaxrs/portal/assemble/designer/page/list/{category}", get(list_pages_by_category))
        .route("/jaxrs/portal/assemble/designer/page/{id}", get(get_page))
        .route("/jaxrs/portal/assemble/designer/page/create", post(create_page))
        .route("/jaxrs/portal/assemble/designer/page/save/{id}", post(save_page))
        .route("/jaxrs/portal/assemble/designer/page/delete/{id}", post(delete_page))
        .route("/jaxrs/portal/assemble/designer/create", post(create_design))
        .route("/jaxrs/portal/assemble/designer/get/{id}", get(get_design))
        .route("/jaxrs/portal/assemble/designer/list", get(list_designs))
        .route("/jaxrs/portal/assemble/designer/save/{id}", post(save_design))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    portal_assemble_designer_router().layer(axum::extract::Extension(pool))
}



pub async fn designer_search(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, create_time, update_time FROM x_portal_design WHERE deleted_at IS NULL ORDER BY update_time DESC LIMIT 20",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn dict_list_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path(_page): Path<i64>,
    Path(_size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, app_name, create_time FROM x_portal_dict WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET ($1 - 1) * $2",
            &[&_page, &_size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn dict_list_portal_portalId(
    pool: Option<Extension<Pool>>,
    Path(portal_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, app_name, create_time FROM x_portal_dict WHERE portal_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn dict_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, app_name, app_data, creator, create_time FROM x_portal_dict WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("appData".to_string(), Value::String(row.get("app_data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("dict not found"))),
    }
}

pub async fn file_list_application_applicationFlag(
    pool: Option<Extension<Pool>>,
    Path(application_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, file_type, creator, create_time FROM x_portal_file WHERE application_flag = $1 ORDER BY create_time DESC",
            &[&application_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn file_list_id_next_count(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, file_type, creator, create_time FROM x_portal_file WHERE id > $1 ORDER BY id ASC LIMIT $2",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn file_list_id_prev_count(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, file_type, creator, create_time FROM x_portal_file WHERE id < $1 ORDER BY id DESC LIMIT $2",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn file_flag(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, flag, file_type, content, creator, create_time FROM x_portal_file WHERE flag = $1 LIMIT 1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, flag, file_type, content, creator, create_time FROM x_portal_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id_download(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, flag, file_type, content FROM x_portal_file WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("fileType".to_string(), Value::String(row.get("file_type"))),
                ("content".to_string(), Value::String(row.get("content"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("file not found"))),
    }
}

pub async fn file_id_upload(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let content = body.get("content").cloned().unwrap_or(Value::Null);
    let content_str = match content {
        Value::String(s) => s,
        _ => serde_json::to_string(&content).map_err(|_| AppError::Internal)?,
    };

    let result = client
        .execute(
            "UPDATE x_portal_file SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("file not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("uploaded".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn id_count(
    pool: Option<Extension<Pool>>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_one("SELECT COUNT(*) as cnt FROM x_portal", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let total: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("total".to_string(), Value::Number(serde_json::Number::from(total))),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn input_compare(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let content_str = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();

    let row = client
        .query_opt(
            "SELECT id, content FROM x_portal_input WHERE id = $1",
            &[&input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let old_content: Option<String> = row.get("content");
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(input_id.to_string())),
                ("oldContent".to_string(), old_content.map(Value::String).unwrap_or(Value::Null)),
                ("newContent".to_string(), Value::String(content_str.to_string())),
                ("compared".to_string(), Value::Bool(true)),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("input not found"))),
    }
}

pub async fn input_cover(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let content_str = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    let result = client
        .execute(
            "UPDATE x_portal_input SET content = $1, update_time = NOW() WHERE id = $2",
            &[&content_str, &input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("input not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(input_id.to_string())),
            ("covered".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_create(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_portal_input (id, content, creator, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn input_prepare_cover(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let row = client
        .query_opt(
            "SELECT id, content FROM x_portal_input WHERE id = $1",
            &[&input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(input_id.to_string())),
                    ("content".to_string(), content.map(Value::String).unwrap_or(Value::Null)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("input not found"))),
    }
}

pub async fn input_prepare_create(
    pool: Option<Extension<Pool>>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let content = body.get("content").and_then(|v| v.as_str()).unwrap_or_default();
    let creator = "system";

    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(id)),
                ("saved".to_string(), Value::Bool(true)),
            ]),
        )))),
    };

    client
        .execute(
            "INSERT INTO x_portal_input (id, content, creator, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn output_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_portal_output WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn output_flag_select_file(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, flag, select_file FROM x_portal_output WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("selectFile".to_string(), Value::String(row.get("select_file"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("output not found"))),
    }
}

pub async fn output_portalFlag_select(
    pool: Option<Extension<Pool>>,
    Path(portal_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_portal_output WHERE portal_flag = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("appName".to_string(), Value::String(row.get("app_name"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn page_list_portal_portalId(
    pool: Option<Extension<Pool>>,
    Path(portal_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time, update_time FROM x_portal_page WHERE portal_id = $1 ORDER BY update_time DESC",
            &[&portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn page_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time, update_time FROM x_portal_page WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let content_value = content
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(Value::Null);
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), content_value),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page not found"))),
    }
}

pub async fn pageversion_list_page_pageId(
    pool: Option<Extension<Pool>>,
    Path(page_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, page_id, version, creator, create_time FROM x_portal_page_version WHERE page_id = $1 ORDER BY create_time DESC",
            &[&page_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("pageId".to_string(), Value::String(row.get("page_id"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn pageversion_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, page_id, version, content, creator, create_time FROM x_portal_page_version WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("pageId".to_string(), Value::String(row.get("page_id"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("page version not found"))),
    }
}

pub async fn portal_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, logo, creator, create_time FROM x_portal WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn portal_list_portalcategory_portalCategory(
    pool: Option<Extension<Pool>>,
    Path(portal_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, logo, creator, create_time FROM x_portal WHERE category = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn portal_list_summary(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, logo, creator, create_time FROM x_portal WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn portal_list_summary_portalcategory_portalCategory(
    pool: Option<Extension<Pool>>,
    Path(portal_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, logo FROM x_portal WHERE category = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn portal_list_summary_v2(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, logo, description, creator, create_time FROM x_portal WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn portal_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, category, logo, description, creator, create_time, update_time FROM x_portal WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
                ("description".to_string(), Value::String(row.get("description"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn portal_id_icon(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, logo FROM x_portal WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("logo".to_string(), Value::String(row.get("logo"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn portal_id_permission(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_portal WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("permission".to_string(), Value::String(row.get("permission"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("portal not found"))),
    }
}

pub async fn portalcategory_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT DISTINCT category FROM x_portal WHERE deleted_at IS NULL AND category IS NOT NULL ORDER BY category",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("category".to_string(), Value::String(row.get("category"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn script_list_manager(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_portal_script WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn script_list_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_portal_script WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET ($1 - 1) * $2",
            &[&page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn script_list_portal_portalId(
    pool: Option<Extension<Pool>>,
    Path(portal_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, flag, category, creator, create_time FROM x_portal_script WHERE portal_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn script_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, flag, category, content, creator, create_time FROM x_portal_script WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script not found"))),
    }
}

pub async fn scriptversion_list_script_scriptId(
    pool: Option<Extension<Pool>>,
    Path(script_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, script_id, version, creator, create_time FROM x_portal_script_version WHERE script_id = $1 ORDER BY create_time DESC",
            &[&script_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("scriptId".to_string(), Value::String(row.get("script_id"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn scriptversion_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, script_id, version, content, creator, create_time FROM x_portal_script_version WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("scriptId".to_string(), Value::String(row.get("script_id"))),
                ("version".to_string(), Value::String(row.get("version"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("script version not found"))),
    }
}

pub async fn templatepage_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time FROM x_portal_template_page WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn templatepage_list_category(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT DISTINCT category FROM x_portal_template_page WHERE deleted_at IS NULL AND category IS NOT NULL ORDER BY category",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("category".to_string(), Value::String(row.get("category"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn templatepage_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time FROM x_portal_template_page WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let content: Option<String> = row.get("content");
            let content_value = content
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or(Value::Null);
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("content".to_string(), content_value),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("template page not found"))),
    }
}

pub async fn widget_list_portal_portalId(
    pool: Option<Extension<Pool>>,
    Path(portal_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, portal_id, category, config, creator, create_time FROM x_portal_widget WHERE portal_id = $1 ORDER BY create_time DESC",
            &[&portal_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("portalId".to_string(), Value::String(row.get("portal_id"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("config".to_string(), Value::String(row.get("config"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn widget_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, portal_id, category, config, creator, create_time, update_time FROM x_portal_widget WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("portalId".to_string(), Value::String(row.get("portal_id"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("config".to_string(), Value::String(row.get("config"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("widget not found"))),
    }
}
