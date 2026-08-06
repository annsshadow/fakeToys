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
pub struct CreateSurfaceRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub query: Option<String>,
    pub template: Option<String>,
}

pub async fn get_surface(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, category, content, creator, create_time, update_time \
             FROM x_query_surface WHERE id = $1",
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
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("query surface not found"))),
    }
}

pub async fn create_surface(
    pool: Option<Extension<Pool>>,
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let content = req.query.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_surface (id, name, category, content, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("category".to_string(), Value::String(category)),
        ("content".to_string(), Value::String(content)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

pub async fn list_surfaces(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, content, creator, create_time, update_time \
             FROM x_query_surface WHERE category = $1 ORDER BY update_time DESC",
            &[&category],
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
                ("updateTime".to_string(), Value::String(row.get("update_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn save_surface(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<CreateSurfaceRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let content = req.query.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_surface SET name = $1, category = $2, content = $3, update_time = NOW() \
             WHERE id = $4",
            &[&name, &category, &content, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query surface not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(true)),
            ("name".to_string(), Value::String(name)),
            ("category".to_string(), Value::String(category)),
            ("content".to_string(), Value::String(content)),
        ]),
    ))))
}

pub async fn delete_surface(
    pool: Option<Extension<Pool>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute("DELETE FROM x_query_surface WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query surface not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn preview_surface(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id.clone())),
            ("preview_url".to_string(), Value::String(format!("/preview/query/{}", id))),
            ("html".to_string(), Value::String("<div>Query Preview</div>".to_string())),
        ]),
    ))))
}

pub fn query_assemble_surface_router() -> Router {
    Router::new()
        .route("/jaxrs/query/assemble/surface/get/{id}", get(get_surface))
        .route("/jaxrs/query/assemble/surface/create", post(create_surface))
        .route("/jaxrs/query/assemble/surface/list/{category}", get(list_surfaces))
        .route("/jaxrs/query/assemble/surface/save/{id}", post(save_surface))
        .route("/jaxrs/query/assemble/surface/delete/{id}", post(delete_surface))
        .route("/jaxrs/query/assemble/surface/preview/{id}", get(preview_surface))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_assemble_surface_router()
        .layer(Extension(pool))
        .route("/query_assemble_surface/health", axum::routing::get(|| async { "ok" }))
}


pub async fn stub_query_assemble_surface_importmodel_execute_record_recordId(
    pool: Option<Extension<Pool>>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, model_flag, data FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                    ("executed".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn stub_query_assemble_surface_importmodel_flag_flag_query_queryFlag(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, model_flag, query_flag FROM x_query_import_model WHERE flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

pub async fn stub_query_assemble_surface_importmodel_list_query_queryFlag(
    pool: Option<Extension<Pool>>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, model_flag, query_flag, creator, create_time FROM x_query_import_model WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
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

pub async fn stub_query_assemble_surface_importmodel_list_record_item_paging_page_size_size(
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
            "SELECT id, model_flag, data, creator, create_time FROM x_query_import_model_record ORDER BY create_time DESC LIMIT $2 OFFSET ($1 - 1) * $2",
            &[&page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
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

pub async fn stub_query_assemble_surface_importmodel_list_record_paging_page_size_size(
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
            "SELECT id, model_flag, data, creator, create_time FROM x_query_import_model_record ORDER BY create_time DESC LIMIT $2 OFFSET ($1 - 1) * $2",
            &[&page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
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

pub async fn stub_query_assemble_surface_importmodel_record_recordId(
    pool: Option<Extension<Pool>>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, model_flag, data, creator, create_time FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn stub_query_assemble_surface_importmodel_record_recordId_mockdeletetoget(
    pool: Option<Extension<Pool>>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, model_flag, data FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn stub_query_assemble_surface_importmodel_record_recordId_status(
    pool: Option<Extension<Pool>>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, status FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("status".to_string(), Value::String(row.get("status"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("record not found"))),
    }
}

pub async fn stub_query_assemble_surface_importmodel_uuid(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("uuid".to_string(), Value::String(id)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_importmodel_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, model_flag, query_flag, content, creator, create_time FROM x_query_import_model WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

pub async fn stub_query_assemble_surface_importmodel_id_execute(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, model_flag FROM x_query_import_model WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let record_id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_query_import_model_record (id, model_flag, import_model_id, create_time) VALUES ($1, $2, $3, NOW())",
                    &[&record_id, &row.get::<_, String>("model_flag"), &id],
                )
                .await
                .map_err(|_| AppError::Internal)?;

            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("recordId".to_string(), Value::String(record_id)),
                    ("executed".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

pub async fn stub_query_assemble_surface_neural_list_calculate_model_modelFlag_work_workId(
    pool: Option<Extension<Pool>>,
    Path(model_flag): Path<String>,
    Path(work_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, model_flag, work_id, result, creator, create_time FROM x_query_neural_calculate WHERE model_flag = $1 AND work_id = $2 ORDER BY create_time DESC",
            &[&model_flag, &work_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                ("workId".to_string(), Value::String(row.get("work_id"))),
                ("result".to_string(), Value::String(row.get("result"))),
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

pub async fn stub_query_assemble_surface_query_list(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design WHERE deleted_at IS NULL ORDER BY create_time DESC",
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

pub async fn stub_query_assemble_surface_query_list_key_key(
    pool: Option<Extension<Pool>>,
    Path(key): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design WHERE name ILIKE $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&format!("%{}%", key)],
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
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_query_flag(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, category, query_definition, creator, create_time FROM x_query_design WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("category".to_string(), Value::String(row.get("category"))),
                ("query".to_string(), Value::String(row.get("query_definition"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("query not found"))),
    }
}

pub async fn stub_query_assemble_surface_table_list_paging_page_size_size(
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
            "SELECT id, name, table_flag, creator, create_time FROM x_query_table WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT $2 OFFSET ($1 - 1) * $2",
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
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
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

pub async fn stub_query_assemble_surface_table_list_table_tableFlag_row_paging_page_size_size(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 ORDER BY id DESC LIMIT $3 OFFSET ($2 - 1) * $3",
            &[&table_flag, &page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_list_id_next_count(
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
            "SELECT id, table_flag, data FROM x_query_table_data WHERE id > $1 ORDER BY id ASC LIMIT $2",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_list_id_prev_count(
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
            "SELECT id, table_flag, data FROM x_query_table_data WHERE id < $1 ORDER BY id DESC LIMIT $2",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_list_tableFlag_row_select(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 ORDER BY id DESC LIMIT 100",
            &[&table_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_list_tableFlag_row_select_where_where(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(_where): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            &format!("SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND data ILIKE $2 ORDER BY id DESC LIMIT 100"),
            &[&table_flag, &format!("%{}%", _where)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_list_tableFlag_row_id_next_count(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id > $2 ORDER BY id ASC LIMIT $3",
            &[&table_flag, &id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_list_tableFlag_row_id_prev_count(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id < $2 ORDER BY id DESC LIMIT $3",
            &[&table_flag, &id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_reload_dynamic(
    pool: Option<Extension<Pool>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "UPDATE x_query_table SET reloaded = true, update_time = NOW() WHERE reloaded = false",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("reloaded".to_string(), Value::Number(serde_json::Number::from(result as i64))),
            ("success".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_table_flag(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, table_flag, creator, create_time FROM x_query_table WHERE table_flag = $1 LIMIT 1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("table not found"))),
    }
}

pub async fn stub_query_assemble_surface_table_tableFlag_row(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 ORDER BY id DESC LIMIT 100",
            &[&table_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_tableFlag_row_count_where_where(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(_where): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_one(
            &format!("SELECT COUNT(*) as cnt FROM x_query_table_data WHERE table_flag = $1 AND data ILIKE $2"),
            &[&table_flag, &format!("%{}%", _where)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("count".to_string(), Value::Number(serde_json::Number::from(count))),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_table_tableFlag_row_delete_all(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let result = client
        .execute(
            "DELETE FROM x_query_table_data WHERE table_flag = $1",
            &[&table_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("deleted".to_string(), Value::Bool(true)),
            ("count".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_table_tableFlag_row_delete_all_mockdeletetoget(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 ORDER BY id DESC",
            &[&table_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                ("data".to_string(), Value::String(row.get("data"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("tableFlag".to_string(), Value::String(table_flag)),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn stub_query_assemble_surface_table_tableFlag_row_one(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 LIMIT 1",
            &[&table_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("row not found"))),
    }
}

pub async fn stub_query_assemble_surface_table_tableFlag_row_id(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id = $2",
            &[&table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("row not found"))),
    }
}

pub async fn stub_query_assemble_surface_table_tableFlag_row_id_mockdeletetoget(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id = $2",
            &[&table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("tableFlag".to_string(), Value::String(row.get("table_flag"))),
                    ("data".to_string(), Value::String(row.get("data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("row not found"))),
    }
}

pub async fn stub_query_assemble_surface_table_tableFlag_row_id_mockputtopost(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table_data SET data = $1, update_time = NOW() WHERE table_flag = $2 AND id = $3",
            &[&data_str, &table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("row not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_table_tableFlag_row_id_part_update(
    pool: Option<Extension<Pool>>,
    Path(table_flag): Path<String>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table_data SET data = jsonb_set(COALESCE(data, '{}'), $1, $2::jsonb, true), update_time = NOW() WHERE table_flag = $3 AND id = $4",
            &[&"{/part}", &data_str, &table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("row not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("tableFlag".to_string(), Value::String(table_flag)),
            ("saved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_view_excel_result_flag(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, view_flag, excel_data, creator, create_time FROM x_query_view_excel WHERE view_flag = $1 ORDER BY create_time DESC",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("excelData".to_string(), Value::String(row.get("excel_data"))),
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

pub async fn stub_query_assemble_surface_view_flag_flag_query_queryFlag(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, view_flag, query_flag, content, creator, create_time FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_flag_flag_query_queryFlag_bundle(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, view_flag, bundle_data FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                    ("bundle".to_string(), Value::String(row.get("bundle_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_flag_flag_query_queryFlag_bundle_mockputtopost(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let bundle_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_view SET bundle_data = $1, update_time = NOW() WHERE view_flag = $2 AND query_flag = $3",
            &[&bundle_str, &flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("viewFlag".to_string(), Value::String(flag)),
            ("queryFlag".to_string(), Value::String(query_flag)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_view_flag_flag_query_queryFlag_excel(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, view_flag, excel_data FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                    ("excelData".to_string(), Value::String(row.get("excel_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_flag_flag_query_queryFlag_excel_mockputtopost(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let excel_str = body.get("excelData").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_view SET excel_data = $1, update_time = NOW() WHERE view_flag = $2 AND query_flag = $3",
            &[&excel_str, &flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("viewFlag".to_string(), Value::String(flag)),
            ("queryFlag".to_string(), Value::String(query_flag)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_view_flag_flag_query_queryFlag_execute(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, content FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("viewFlag".to_string(), Value::String(flag)),
                    ("executed".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_flag_flag_query_queryFlag_execute_mockputtopost(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_view SET content = $1, update_time = NOW() WHERE view_flag = $2 AND query_flag = $3",
            &[&data_str, &flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("viewFlag".to_string(), Value::String(flag)),
            ("queryFlag".to_string(), Value::String(query_flag)),
            ("executed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_view_flag_flag_query_queryFlag_execute_v2_page_page_size_size(
    pool: Option<Extension<Pool>>,
    Path(flag): Path<String>,
    Path(query_flag): Path<String>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, view_flag, query_flag, content, creator, create_time FROM x_query_view WHERE view_flag = $1 AND query_flag = $2 ORDER BY create_time DESC LIMIT $4 OFFSET ($3 - 1) * $4",
            &[&flag, &query_flag, &page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
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

pub async fn stub_query_assemble_surface_view_list_query_queryFlag(
    pool: Option<Extension<Pool>>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, name, view_flag, query_flag, creator, create_time FROM x_query_view WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
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

pub async fn stub_query_assemble_surface_view_id(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, name, view_flag, query_flag, content, creator, create_time FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                ("queryFlag".to_string(), Value::String(row.get("query_flag"))),
                ("content".to_string(), Value::String(row.get("content"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_id_bundle(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, view_flag, bundle_data FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                    ("bundle".to_string(), Value::String(row.get("bundle_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_id_bundle_mockputtopost(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let bundle_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_view SET bundle_data = $1, update_time = NOW() WHERE id = $2",
            &[&bundle_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_view_id_bundle_v2(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, view_flag, bundle_data_v2 FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                    ("bundle".to_string(), Value::String(row.get("bundle_data_v2"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_id_excel(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, view_flag, excel_data FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("viewFlag".to_string(), Value::String(row.get("view_flag"))),
                    ("excelData".to_string(), Value::String(row.get("excel_data"))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_id_excel_mockputtopost(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let excel_str = body.get("excelData").and_then(|v| v.as_str()).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_view SET excel_data = $1, update_time = NOW() WHERE id = $2",
            &[&excel_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_view_id_execute(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let row = client
        .query_opt(
            "SELECT id, content FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(_) => {
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(id)),
                    ("executed".to_string(), Value::Bool(true)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

pub async fn stub_query_assemble_surface_view_id_execute_mockputtopost(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_view SET content = $1, update_time = NOW() WHERE id = $2",
            &[&data_str, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("executed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn stub_query_assemble_surface_view_id_execute_v2_page_page_size_size(
    pool: Option<Extension<Pool>>,
    Path(id): Path<String>,
    Path(page): Path<i64>,
    Path(size): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = match pool {
        Some(Extension(pool)) => pool.get().await.map_err(|_| AppError::Internal)?,
        None => return Ok(Json(ActionResult::success(Value::Null))),
    };

    let rows = client
        .query(
            "SELECT id, content, creator, create_time FROM x_query_view WHERE id = $1 ORDER BY create_time DESC LIMIT $3 OFFSET ($2 - 1) * $3",
            &[&id, &page, &size],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
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
