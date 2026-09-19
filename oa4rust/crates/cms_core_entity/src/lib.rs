use axum::{
    extract::{Extension, Json, Path},
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde_json::Value;
use shared::{
    error::AppError,
    response::{option_to_json, ActionResult},
};

pub mod entities;
pub mod routes;

use entities::{cms_article, cms_category};

#[allow(non_snake_case)]
pub async fn category_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = cms_category::Entity::find()
        .filter(cms_category::Column::DeletedAt.is_null())
        .order_by_asc(cms_category::Column::SortOrder)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert("name".to_string(), Value::String(m.name.clone()));
            if let Some(val) = option_to_json(m.parent_id.clone().map(Value::String)) {
                map.insert("parentId".to_string(), val);
            }
            map.insert(
                "sortOrder".to_string(),
                Value::Number(serde_json::Number::from(m.sort_order)),
            );
            map.insert("status".to_string(), Value::String(m.status.clone()));
            map.insert(
                "createTime".to_string(),
                Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
            );
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[allow(non_snake_case)]
pub async fn category_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = cms_category::Entity::find_by_id(&id)
        .filter(cms_category::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert("name".to_string(), Value::String(m.name.clone()));
            if let Some(val) = option_to_json(m.parent_id.clone().map(Value::String)) {
                map.insert("parentId".to_string(), val);
            }
            map.insert(
                "sortOrder".to_string(),
                Value::Number(serde_json::Number::from(m.sort_order)),
            );
            map.insert("status".to_string(), Value::String(m.status.clone()));
            map.insert(
                "createTime".to_string(),
                Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
            );
            let result = Value::Object(map);
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("category not found"))),
    }
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn category_create(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let parent_id = payload
        .get("\"parentId\"")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let sort_order = payload
        .get("sortOrder")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("active")
        .to_string();
    let _create_time = chrono::Utc::now();

    let active_model = cms_category::ActiveModel {
        id: Set(id.clone()),
        name: Set(name.clone()),
        parent_id: Set(parent_id),
        sort_order: Set(sort_order),
        status: Set(status.clone()),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
        deleted_at: Set(None),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name)),
            (
                "sortOrder".to_string(),
                Value::Number(serde_json::Number::from(sort_order)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn article_list(
    db: Extension<DatabaseConnection>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let models = cms_article::Entity::find()
        .filter(cms_article::Column::DeletedAt.is_null())
        .order_by_desc(cms_article::Column::CreateTime)
        .limit(20)
        .all(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = models
        .iter()
        .map(|m| {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert(
                "categoryId".to_string(),
                Value::String(m.category_id.clone()),
            );
            map.insert("title".to_string(), Value::String(m.title.clone()));
            if let Some(val) = option_to_json(m.content.clone().map(Value::String)) {
                map.insert("content".to_string(), val);
            }
            map.insert("authorId".to_string(), Value::String(m.author_id.clone()));
            map.insert("status".to_string(), Value::String(m.status.clone()));
            if let Some(val) =
                option_to_json(m.publish_time.map(|dt| Value::String(dt.to_string())))
            {
                map.insert("publishTime".to_string(), val);
            }
            map.insert(
                "createTime".to_string(),
                Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
            );
            Value::Object(map)
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[allow(non_snake_case)]
pub async fn article_get(
    db: Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let model = cms_article::Entity::find_by_id(&id)
        .filter(cms_article::Column::DeletedAt.is_null())
        .one(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    match model {
        Some(m) => {
            let mut map = serde_json::Map::new();
            map.insert("id".to_string(), Value::String(m.id.clone()));
            map.insert(
                "categoryId".to_string(),
                Value::String(m.category_id.clone()),
            );
            map.insert("title".to_string(), Value::String(m.title.clone()));
            if let Some(val) = option_to_json(m.content.clone().map(Value::String)) {
                map.insert("content".to_string(), val);
            }
            map.insert("authorId".to_string(), Value::String(m.author_id.clone()));
            map.insert("status".to_string(), Value::String(m.status.clone()));
            if let Some(val) =
                option_to_json(m.publish_time.map(|dt| Value::String(dt.to_string())))
            {
                map.insert("publishTime".to_string(), val);
            }
            map.insert(
                "createTime".to_string(),
                Value::String(m.create_time.map(|dt| dt.to_string()).unwrap_or_default()),
            );
            let result = Value::Object(map);
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("article not found"))),
    }
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn article_create(
    db: Extension<DatabaseConnection>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let category_id = payload
        .get("categoryId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let title = payload
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let author_id = payload
        .get("authorId")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let status = payload
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("draft")
        .to_string();

    let active_model = cms_article::ActiveModel {
        id: Set(id.clone()),
        category_id: Set(category_id.clone()),
        title: Set(title.clone()),
        content: Set(content),
        author_id: Set(author_id),
        status: Set(status),
        publish_time: Set(None),
        create_time: Set(Some(chrono::Utc::now().naive_utc())),
        deleted_at: Set(None),
    };

    active_model
        .insert(&db.0)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("title".to_string(), Value::String(title)),
            ("categoryId".to_string(), Value::String(category_id)),
        ]),
    ))))
}

// ── cms/core/entity/* 斜杠路径家族（前端 o2server 斜杠口径补齐，查真实表 093 迁移）──
/// 每表显式列映射：键名与 create/save payload 的 camelCase 口径一致；
/// 时间戳走 ::text（tokio-postgres 在本栈里 NaiveDateTime 不实现 FromSql）。
async fn list_cms_entity_table(
    pool: &deadpool_postgres::Pool,
    table: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let sql = match table {
        "x_cms_column" => {
            "SELECT id, name, parent_id AS parentId, sort_order AS sortOrder, \
             description, status, create_time::text AS createTime, update_time::text AS updateTime \
             FROM x_cms_column WHERE deleted_at IS NULL ORDER BY sort_order, id"
        }
        "x_cms_index" => {
            "SELECT id, name, target, sort_order AS sortOrder, \
             description, status, create_time::text AS createTime, update_time::text AS updateTime \
             FROM x_cms_index WHERE deleted_at IS NULL ORDER BY sort_order, id"
        }
        "x_cms_module" => {
            "SELECT id, name, app_id AS appId, module_type AS moduleType, \
             description, status, create_time::text AS createTime, update_time::text AS updateTime \
             FROM x_cms_module WHERE deleted_at IS NULL ORDER BY id"
        }
        "x_cms_note" => {
            "SELECT id, title, content, person_id AS personId, \
             status, create_time::text AS createTime, update_time::text AS updateTime \
             FROM x_cms_note WHERE deleted_at IS NULL ORDER BY create_time DESC, id"
        }
        _ => {
            "SELECT id, column_id AS columnId, person_id AS personId, role, \
             create_time::text AS createTime, update_time::text AS updateTime \
             FROM x_cms_column_manager WHERE deleted_at IS NULL ORDER BY id"
        }
    };
    let rows = client
        .query(sql, &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(shared::response::row_to_json).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_column_list(
    pool: Extension<deadpool_postgres::Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_cms_entity_table(&pool, "x_cms_column").await
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_column_manager_list(
    pool: Extension<deadpool_postgres::Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_cms_entity_table(&pool, "x_cms_column_manager").await
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_index_list(
    pool: Extension<deadpool_postgres::Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_cms_entity_table(&pool, "x_cms_index").await
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_module_list(
    pool: Extension<deadpool_postgres::Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_cms_entity_table(&pool, "x_cms_module").await
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_note_list(
    pool: Extension<deadpool_postgres::Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_cms_entity_table(&pool, "x_cms_note").await
}

// ── cms/core/entity/* 的 create/save/delete（参数化写操作，表 093）──
async fn cms_entity_soft_delete(
    pool: &deadpool_postgres::Pool,
    table: &str,
    id: &str,
) -> Result<bool, AppError> {
    use deadpool_postgres::tokio_postgres::types::ToSql;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let owned = id.to_string();
    let refs: Vec<&(dyn ToSql + Sync)> = vec![&owned as &(dyn ToSql + Sync)];
    let n = client
        .execute(
            &format!("UPDATE {table} SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"),
            &refs,
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(n > 0)
}

/// 参数化 create：table 是内部常量（非用户输入），命名列按表取；值全部走 $ 占位防注入。
async fn cms_entity_create(
    pool: &deadpool_postgres::Pool,
    table: &str,
    payload: &Value,
) -> Result<String, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    use deadpool_postgres::tokio_postgres::types::ToSql;
    let id = uuid::Uuid::new_v4().to_string();
    let s = |k: &str| {
        payload
            .get(k)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    let (sql, values): (String, Vec<String>) = match table {
        "x_cms_column" => (
            "INSERT INTO x_cms_column (id, name, parent_id, description, status, create_time, update_time) \
             VALUES ($1,$2,$3,$4,'active',NOW(),NOW())".to_string(),
            vec![id.clone(), s("name"), s("parentId"), s("description")],
        ),
        "x_cms_index" => (
            "INSERT INTO x_cms_index (id, name, target, description, create_time, update_time) \
             VALUES ($1,$2,$3,$4,NOW(),NOW())".to_string(),
            vec![id.clone(), s("name"), s("target"), s("description")],
        ),
        "x_cms_module" => (
            "INSERT INTO x_cms_module (id, name, app_id, module_type, description, status, create_time, update_time) \
             VALUES ($1,$2,$3,$4,$5,'active',NOW(),NOW())".to_string(),
            vec![id.clone(), s("name"), s("appId"), s("moduleType"), s("description")],
        ),
        "x_cms_note" => (
            "INSERT INTO x_cms_note (id, title, content, person_id, status, create_time, update_time) \
             VALUES ($1,$2,$3,$4,'active',NOW(),NOW())".to_string(),
            vec![id.clone(), s("title"), s("content"), s("personId")],
        ),
        _ => (
            "INSERT INTO x_cms_column_manager (id, column_id, person_id, role, create_time, update_time) \
             VALUES ($1,$2,$3,'manager',NOW(),NOW())".to_string(),
            vec![id.clone(), s("columnId"), s("personId")],
        ),
    };
    let params: Vec<&(dyn ToSql + Sync)> =
        values.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
    client
        .execute(&sql, &params)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(id)
}

/// 参数化 save：更新该表命名列（写入传入字段），值走 $ 占位防注入。
async fn cms_entity_save(
    pool: &deadpool_postgres::Pool,
    table: &str,
    id: &str,
    payload: &Value,
) -> Result<bool, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    use deadpool_postgres::tokio_postgres::types::ToSql;
    let s = |k: &str| {
        payload
            .get(k)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    let (sql, values): (String, Vec<String>) = match table {
        "x_cms_column" => (
            "UPDATE x_cms_column SET name=$1, description=$2, update_time=NOW() WHERE id=$3 AND deleted_at IS NULL".to_string(),
            vec![s("name"), s("description"), id.to_string()],
        ),
        "x_cms_index" => (
            "UPDATE x_cms_index SET name=$1, target=$2, description=$3, update_time=NOW() WHERE id=$4 AND deleted_at IS NULL".to_string(),
            vec![s("name"), s("target"), s("description"), id.to_string()],
        ),
        "x_cms_module" => (
            "UPDATE x_cms_module SET name=$1, app_id=$2, module_type=$3, description=$4, update_time=NOW() WHERE id=$5 AND deleted_at IS NULL".to_string(),
            vec![s("name"), s("appId"), s("moduleType"), s("description"), id.to_string()],
        ),
        "x_cms_note" => (
            "UPDATE x_cms_note SET title=$1, content=$2, person_id=$3, update_time=NOW() WHERE id=$4 AND deleted_at IS NULL".to_string(),
            vec![s("title"), s("content"), s("personId"), id.to_string()],
        ),
        _ => (
            "UPDATE x_cms_column_manager SET column_id=$1, person_id=$2, role=$3, update_time=NOW() WHERE id=$4 AND deleted_at IS NULL".to_string(),
            vec![s("columnId"), s("personId"), s("role"), id.to_string()],
        ),
    };
    let params: Vec<&(dyn ToSql + Sync)> =
        values.iter().map(|v| v as &(dyn ToSql + Sync)).collect();
    let n = client
        .execute(&sql, &params)
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(n > 0)
}

// ── 5 家族 × (create/save/delete) 薄 handler ──
#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_column_create(
    pool: Extension<deadpool_postgres::Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = cms_entity_create(&pool, "x_cms_column", &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_column_save(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_save(&pool, "x_cms_column", &id, &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_column_delete(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_soft_delete(&pool, "x_cms_column", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_column_manager_create(
    pool: Extension<deadpool_postgres::Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = cms_entity_create(&pool, "x_cms_column_manager", &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_column_manager_save(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_save(&pool, "x_cms_column_manager", &id, &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_column_manager_delete(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_soft_delete(&pool, "x_cms_column_manager", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_index_create(
    pool: Extension<deadpool_postgres::Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = cms_entity_create(&pool, "x_cms_index", &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_index_save(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_save(&pool, "x_cms_index", &id, &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_index_delete(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_soft_delete(&pool, "x_cms_index", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_module_create(
    pool: Extension<deadpool_postgres::Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = cms_entity_create(&pool, "x_cms_module", &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_module_save(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_save(&pool, "x_cms_module", &id, &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_module_delete(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_soft_delete(&pool, "x_cms_module", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_note_create(
    pool: Extension<deadpool_postgres::Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = cms_entity_create(&pool, "x_cms_note", &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_note_save(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_save(&pool, "x_cms_note", &id, &payload).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn cms_entity_note_delete(
    pool: Extension<deadpool_postgres::Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let ok = cms_entity_soft_delete(&pool, "x_cms_note", &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(ok)),
        ]),
    ))))
}

pub fn cms_core_entity_router(pool: deadpool_postgres::Pool) -> Router {
    Router::new()
        .route("/api/cms/category/list", get(category_list))
        .route("/api/cms/category/{id}", get(category_get))
        .route("/api/cms/category/create", post(category_create))
        .route("/api/cms/article/list", get(article_list))
        .route("/api/cms/article/{id}", get(article_get))
        .route("/api/cms/article/create", post(article_create))
        // ── cms/core/entity/* 斜杠路径家族（补齐 KNOWN_BACKEND_GAPS）──
        .route(
            "/api/cms/core/entity/column/list",
            get(cms_entity_column_list),
        )
        .route(
            "/api/cms/core/entity/column_manager/list",
            get(cms_entity_column_manager_list),
        )
        .route(
            "/api/cms/core/entity/index/list",
            get(cms_entity_index_list),
        )
        .route(
            "/api/cms/core/entity/module/list",
            get(cms_entity_module_list),
        )
        .route("/api/cms/core/entity/note/list", get(cms_entity_note_list))
        // ── 5 家族的 create/save/delete（真实写操作，支撑桌面 CRUD 视图）──
        .route(
            "/api/cms/core/entity/column/create",
            post(cms_entity_column_create),
        )
        .route(
            "/api/cms/core/entity/column/save/{id}",
            put(cms_entity_column_save),
        )
        .route(
            "/api/cms/core/entity/column/save/{id}",
            post(cms_entity_column_save),
        )
        .route(
            "/api/cms/core/entity/column/delete/{id}",
            delete(cms_entity_column_delete),
        )
        .route(
            "/api/cms/core/entity/column/delete/{id}",
            post(cms_entity_column_delete),
        )
        .route(
            "/api/cms/core/entity/column_manager/create",
            post(cms_entity_column_manager_create),
        )
        .route(
            "/api/cms/core/entity/column_manager/save/{id}",
            put(cms_entity_column_manager_save),
        )
        .route(
            "/api/cms/core/entity/column_manager/save/{id}",
            post(cms_entity_column_manager_save),
        )
        .route(
            "/api/cms/core/entity/column_manager/delete/{id}",
            delete(cms_entity_column_manager_delete),
        )
        .route(
            "/api/cms/core/entity/column_manager/delete/{id}",
            post(cms_entity_column_manager_delete),
        )
        .route(
            "/api/cms/core/entity/index/create",
            post(cms_entity_index_create),
        )
        .route(
            "/api/cms/core/entity/index/save/{id}",
            put(cms_entity_index_save),
        )
        .route(
            "/api/cms/core/entity/index/save/{id}",
            post(cms_entity_index_save),
        )
        .route(
            "/api/cms/core/entity/index/delete/{id}",
            delete(cms_entity_index_delete),
        )
        .route(
            "/api/cms/core/entity/index/delete/{id}",
            post(cms_entity_index_delete),
        )
        .route(
            "/api/cms/core/entity/module/create",
            post(cms_entity_module_create),
        )
        .route(
            "/api/cms/core/entity/module/save/{id}",
            put(cms_entity_module_save),
        )
        .route(
            "/api/cms/core/entity/module/save/{id}",
            post(cms_entity_module_save),
        )
        .route(
            "/api/cms/core/entity/module/delete/{id}",
            delete(cms_entity_module_delete),
        )
        .route(
            "/api/cms/core/entity/module/delete/{id}",
            post(cms_entity_module_delete),
        )
        .route(
            "/api/cms/core/entity/note/create",
            post(cms_entity_note_create),
        )
        .route(
            "/api/cms/core/entity/note/save/{id}",
            put(cms_entity_note_save),
        )
        .route(
            "/api/cms/core/entity/note/save/{id}",
            post(cms_entity_note_save),
        )
        .route(
            "/api/cms/core/entity/note/delete/{id}",
            delete(cms_entity_note_delete),
        )
        .route(
            "/api/cms/core/entity/note/delete/{id}",
            post(cms_entity_note_delete),
        )
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_core_entity_router(pool)
}
