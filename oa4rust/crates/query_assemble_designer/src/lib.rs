#![allow(dead_code, non_snake_case)]
use axum::{
    extract::{Extension, Path},
    routing::delete,
    routing::get,
    routing::post,
    routing::put,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::row_to_json, response::ActionResult};

pub mod routes;
pub mod u2_closures;

use u2_closures::{ensure_limit, validate_single_select};

const DESIGN_TABLE_PREFIX: &str = "x_query_data_";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TableColumnDefinition {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    #[serde(default = "default_nullable")]
    pub nullable: bool,
}

fn default_nullable() -> bool {
    true
}

fn validate_identifier(value: &str) -> Result<&str, AppError> {
    let valid = !value.is_empty()
        && value.len() <= 63
        && value.bytes().enumerate().all(|(index, byte)| {
            byte == b'_' || byte.is_ascii_lowercase() || (index > 0 && byte.is_ascii_digit())
        });
    if valid {
        Ok(value)
    } else {
        Err(AppError::BadRequest(format!(
            "invalid identifier '{}': use lowercase letters, digits, and underscores",
            value
        )))
    }
}

fn postgres_column_type(value: &str) -> Result<&'static str, AppError> {
    match value {
        "text" => Ok("TEXT"),
        "integer" => Ok("BIGINT"),
        "decimal" => Ok("DOUBLE PRECISION"),
        "boolean" => Ok("BOOLEAN"),
        "date" => Ok("DATE"),
        "datetime" => Ok("TIMESTAMPTZ"),
        "json" => Ok("JSONB"),
        _ => Err(AppError::BadRequest(format!(
            "unsupported column type '{}'",
            value
        ))),
    }
}

pub fn parse_table_columns(body: &Value) -> Result<Vec<TableColumnDefinition>, AppError> {
    let columns_value = body
        .get("columns")
        .ok_or_else(|| AppError::BadRequest("columns is required".to_string()))?;
    let columns: Vec<TableColumnDefinition> = serde_json::from_value(columns_value.clone())
        .map_err(|_| AppError::BadRequest("columns must be a typed array".to_string()))?;
    if columns.is_empty() {
        return Err(AppError::BadRequest(
            "at least one column is required".to_string(),
        ));
    }
    if columns.len() > 100 {
        return Err(AppError::BadRequest(
            "at most 100 columns are allowed".to_string(),
        ));
    }
    let mut names = std::collections::HashSet::new();
    for column in &columns {
        validate_identifier(&column.name)?;
        postgres_column_type(&column.data_type)?;
        if column.name == "id" {
            return Err(AppError::BadRequest(
                "column name 'id' is reserved".to_string(),
            ));
        }
        if !names.insert(column.name.as_str()) {
            return Err(AppError::BadRequest(format!(
                "duplicate column '{}'",
                column.name
            )));
        }
    }
    Ok(columns)
}

pub fn physical_table_name(table_flag: &str) -> Result<String, AppError> {
    validate_identifier(table_flag)?;
    Ok(format!("{}{}", DESIGN_TABLE_PREFIX, table_flag))
}

pub fn create_table_ddl(
    table_flag: &str,
    columns: &[TableColumnDefinition],
) -> Result<String, AppError> {
    let table_name = physical_table_name(table_flag)?;
    let definitions = columns
        .iter()
        .map(|column| {
            Ok(format!(
                "\"{}\" {}{}",
                column.name,
                postgres_column_type(&column.data_type)?,
                if column.nullable { "" } else { " NOT NULL" }
            ))
        })
        .collect::<Result<Vec<_>, AppError>>()?;
    Ok(format!(
        "CREATE TABLE \"{}\" (\"id\" UUID PRIMARY KEY DEFAULT gen_random_uuid(), {})",
        table_name,
        definitions.join(", ")
    ))
}

pub fn add_column_ddl(
    table_flag: &str,
    column: &TableColumnDefinition,
) -> Result<String, AppError> {
    let table_name = physical_table_name(table_flag)?;
    validate_identifier(&column.name)?;
    Ok(format!(
        "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {}{}",
        table_name,
        column.name,
        postgres_column_type(&column.data_type)?,
        if column.nullable { "" } else { " NOT NULL" }
    ))
}

fn table_columns_json(columns: &[TableColumnDefinition]) -> Result<String, AppError> {
    serde_json::to_string(columns).map_err(|_| AppError::Internal)
}

#[allow(non_snake_case)]
pub async fn create_table_definition(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let name = body
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    if name.is_empty() {
        return Err(AppError::BadRequest("name is required".to_string()));
    }
    let columns = parse_table_columns(&body)?;
    let columns_json = table_columns_json(&columns)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let duplicate = client
        .query_one(
            "SELECT COUNT(*) AS cnt FROM x_query_table WHERE LOWER(TRIM(COALESCE(name,''))) = $1 AND deleted_at IS NULL",
            &[&name.to_lowercase()],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if duplicate.get::<_, i64>("cnt") > 0 {
        return Ok(Json(ActionResult::error("table name already exists")));
    }
    let id = uuid::Uuid::new_v4().to_string();
    let table_flag = format!("t_{}", uuid::Uuid::new_v4().simple());
    let query_flag = body
        .get("queryFlag")
        .and_then(Value::as_str)
        .unwrap_or_default();
    client
        .execute(
            "INSERT INTO x_query_table (id, name, table_flag, query_flag, status, columns, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, 'draft', $5, 'system', NOW(), NOW())",
            &[&id, &name, &table_flag, &query_flag, &columns_json],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "id": id,
        "tableFlag": table_flag,
        "name": name,
        "columns": columns,
        "status": "draft"
    }))))
}

#[allow(non_snake_case)]
pub async fn update_table_definition(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let columns = parse_table_columns(&body)?;
    let columns_json = table_columns_json(&columns)?;
    let name = body.get("name").and_then(Value::as_str).unwrap_or_default();
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = client
        .execute(
            "UPDATE x_query_table SET name = COALESCE(NULLIF($1,''), name), columns = $2, status = 'draft', update_time = NOW() \
             WHERE table_flag = $3 AND deleted_at IS NULL",
            &[&name, &columns_json, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if result == 0 {
        return Ok(Json(ActionResult::error("table not found")));
    }
    Ok(Json(ActionResult::success(serde_json::json!({
        "tableFlag": flag,
        "columns": columns,
        "status": "draft"
    }))))
}

#[allow(non_snake_case)]
pub async fn execute_table_definition(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT columns FROM x_query_table WHERE table_flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let Some(row) = row else {
        return Ok(Json(ActionResult::error("table not found")));
    };
    let columns_raw: String = row.get("columns");
    let columns: Vec<TableColumnDefinition> = serde_json::from_str(&columns_raw)
        .map_err(|_| AppError::BadRequest("stored columns are invalid".to_string()))?;
    if columns.is_empty() {
        return Err(AppError::BadRequest(
            "at least one column is required".to_string(),
        ));
    }
    let table_name = physical_table_name(&flag)?;
    let exists = client
        .query_one(
            "SELECT to_regclass($1)::text IS NOT NULL AS exists",
            &[&table_name],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get::<_, bool>("exists");
    let mut applied = Vec::new();
    if exists {
        let rows = client
            .query(
                "SELECT column_name, data_type, is_nullable FROM information_schema.columns \
                 WHERE table_schema = current_schema() AND table_name = $1 AND column_name <> 'id'",
                &[&table_name],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        let stored_by_name = columns
            .iter()
            .map(|column| (column.name.as_str(), column))
            .collect::<std::collections::HashMap<_, _>>();
        for row in rows {
            let name: String = row.get("column_name");
            let Some(expected) = stored_by_name.get(name.as_str()) else {
                return Err(AppError::BadRequest(
                    "removing columns from a built table is not supported".to_string(),
                ));
            };
            let actual_type: String = row.get("data_type");
            let expected_type = postgres_column_type(&expected.data_type)?.to_lowercase();
            let type_matches = actual_type == expected_type
                || (expected_type == "bigint" && actual_type == "bigint")
                || (expected_type == "double precision" && actual_type == "double precision")
                || (expected_type == "timestamp with time zone"
                    && actual_type == "timestamp with time zone");
            let actual_nullable = row.get::<_, String>("is_nullable") == "YES";
            if !type_matches || actual_nullable != expected.nullable {
                return Err(AppError::BadRequest(format!(
                    "changing existing column '{}' type or nullability is not supported",
                    name
                )));
            }
        }
        let existing_names = client
            .query(
                "SELECT column_name FROM information_schema.columns WHERE table_schema = current_schema() AND table_name = $1",
                &[&table_name],
            )
            .await
            .map_err(|_| AppError::Internal)?
            .into_iter()
            .map(|row| row.get::<_, String>("column_name"))
            .collect::<std::collections::HashSet<_>>();
        for column in &columns {
            if !existing_names.contains(&column.name) {
                let ddl = add_column_ddl(&flag, column)?;
                client
                    .batch_execute(&ddl)
                    .await
                    .map_err(|_| AppError::Internal)?;
                applied.push(column.name.clone());
            }
        }
    } else {
        let ddl = create_table_ddl(&flag, &columns)?;
        client
            .batch_execute(&ddl)
            .await
            .map_err(|_| AppError::Internal)?;
        applied.extend(columns.iter().map(|column| column.name.clone()));
    }
    client
        .execute(
            "UPDATE x_query_table SET status = 'build', update_time = NOW() WHERE table_flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "tableFlag": flag,
        "physicalTable": table_name,
        "created": !exists,
        "appliedColumns": applied,
        "status": "build"
    }))))
}

#[derive(Debug, Deserialize)]
pub struct CreateDesignerRequest {
    pub name: Option<String>,
    pub query: Option<String>,
    pub category: Option<String>,
}

#[allow(non_snake_case)]
pub async fn get_designer(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, category, query_definition, creator, create_time, update_time \
             FROM x_query_design WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
                (
                    "query".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_definition")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "updateTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("update_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("query design not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn create_designer(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<CreateDesignerRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let query_definition = req.query.unwrap_or_default();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_design (id, name, category, query_definition, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
            &[&id, &name, &category, &query_definition, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(name)),
        ("category".to_string(), Value::String(category)),
        ("query".to_string(), Value::String(query_definition)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[allow(non_snake_case)]
pub async fn list_designers(
    pool: Extension<Pool>,
    axum::extract::Path(category): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, create_time, update_time FROM x_query_design \
             WHERE category = $1 AND deleted_at IS NULL ORDER BY update_time DESC",
            &[&category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "updateTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("update_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn save_designer(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    axum::extract::Json(req): Json<CreateDesignerRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = req.name.unwrap_or_default();
    let category = req.category.unwrap_or_default();
    let query_definition = req.query.unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_design SET name = $1, category = $2, query_definition = $3, update_time = NOW() \
             WHERE id = $4 AND deleted_at IS NULL",
            &[&name, &category, &query_definition, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query design not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            (
                "saved".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
            ("name".to_string(), Value::String(name)),
            ("query".to_string(), Value::String(query_definition)),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn delete_designer(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_design SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error(
            "query design not found or already deleted",
        )));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            (
                "deleted".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

// ── 裸路径变体（桌面 QueryQueryApp 以配置串引用 save/delete，list 为无参全量）──
#[allow(non_snake_case)]
pub async fn list_designers_all(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, category, create_time, update_time FROM x_query_design \
             WHERE deleted_at IS NULL ORDER BY update_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
            ]))
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
pub async fn save_designer_bare(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = req
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let name = req
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let category = req
        .get("category")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let query = req
        .get("query")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let updated = client
        .execute(
            "UPDATE x_query_design SET name = $1, category = $2, query_definition = $3, update_time = NOW() \
             WHERE id = $4 AND deleted_at IS NULL",
            &[&name, &category, &query, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let saved = if updated > 0 {
        updated
    } else {
        // 不存在则新建（upsert 语义），使裸 save 可创建。
        let new_id = if id.is_empty() {
            uuid::Uuid::new_v4().to_string()
        } else {
            id.clone()
        };
        let r = client
            .execute(
                "INSERT INTO x_query_design (id, name, category, query_definition, create_time, update_time) \
                 VALUES ($1, $2, $3, $4, NOW(), NOW())",
                &[&new_id, &name, &category, &query],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        r
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            (
                "saved".to_string(),
                Value::Number(serde_json::Number::from(saved)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn delete_designer_bare(
    pool: Extension<Pool>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = req
        .get("id")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let result = client
        .execute(
            "UPDATE x_query_design SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error(
            "query design not found or already deleted",
        )));
    }
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub fn query_assemble_designer_router(pool: Option<Pool>) -> Router {
    use u2_closures as u2;
    let router = Router::new()
        .route("/api/query/assemble/designer/get/{id}", get(get_designer))
        .route("/api/query/assemble/designer/create", post(create_designer))
        .route("/api/query/assemble/designer/list/{category}", get(list_designers))
        .route("/api/query/assemble/designer/save/{id}", post(save_designer))
        .route("/api/query/assemble/designer/delete/{id}", post(delete_designer))
        // 裸路径变体（桌面配置串引用；list 无参全量，save/delete id 走 body）
        .route("/api/query/assemble/designer/list", get(list_designers_all))
        .route("/api/query/assemble/designer/save", put(save_designer_bare))
        .route("/api/query/assemble/designer/save", post(save_designer_bare))
        .route("/api/query/assemble/designer/delete", delete(delete_designer_bare))
        .route("/api/query/assemble/designer/delete", post(delete_designer_bare))
        .route("/api/query/assemble/designer/{id}/{count}", get(crate::id_count))
        .route("/api/query/assemble/designer/importmodel/{id}", post(crate::importmodel_id))
        .route("/api/query/assemble/designer/importmodel/permission/{id}", post(crate::importmodel_id_permission))
        .route("/api/query/assemble/designer/importmodel/list/{query}/{flag}", post(crate::importmodel_list_query_flag))
        .route("/api/query/assemble/designer/neural/generate/model/{modelFlag}", get(crate::neural_generate_model_modelFlag))
        .route("/api/query/assemble/designer/neural/learn/model/{modelFlag}", get(crate::neural_learn_model_modelFlag))
        .route("/api/query/assemble/designer/neural/model/{modelFlag}", get(crate::neural_model_modelFlag))
        .route("/api/query/assemble/designer/neural/model/reset/{modelFlag}/{status}", post(crate::neural_model_modelFlag_reset_status))
        .route("/api/query/assemble/designer/neural/stop/generating/model/{modelFlag}", get(crate::neural_stop_generating_model_modelFlag))
        .route("/api/query/assemble/designer/neural/stop/learn/model/{modelFlag}", get(crate::neural_stop_learn_model_modelFlag))
        .route("/api/query/assemble/designer/output/select/file/{flag}", get(crate::output_flag_select_file))
        .route("/api/query/assemble/designer/output/select/{queryFlag}", get(crate::output_queryFlag_select))
        .route("/api/query/assemble/designer/entity/entity/properties/{query}/{category}/{entityCategory}", get(crate::query_entity_entity_category_entityCategory_properties))
        .route("/api/query/assemble/designer/icon/{query}/{flag}", get(crate::query_flag_icon))
        .route("/api/query/assemble/designer/permission/{query}/{id}", get(crate::query_id_permission))
        .route("/api/query/assemble/designer/list/querycategory/{query}/{queryCategory}", get(crate::query_list_querycategory_queryCategory))
        .route("/api/query/assemble/designer/list/summary/querycategory/{query}/{queryCategory}", get(crate::query_list_summary_querycategory_queryCategory))
        .route("/api/query/assemble/designer/stat/{id}", get(crate::stat_id))
        .route("/api/query/assemble/designer/stat/permission/{id}", get(crate::stat_id_permission))
        .route("/api/query/assemble/designer/stat/simulate/{id}", get(crate::stat_id_simulate))
        .route("/api/query/assemble/designer/stat/list/{id}/{next}/{count}", get(crate::stat_list_id_next_count))
        .route("/api/query/assemble/designer/stat/list/{query}/{flag}", get(crate::stat_list_query_flag))
        .route("/api/query/assemble/designer/table/export/{tableFlag}/{count}/{count}", get(crate::table_export_tableFlag_count_count))
        .route("/api/query/assemble/designer/table/{flag}", get(crate::table_flag))
        .route("/api/query/assemble/designer/table/execute/{flag}", post(crate::table_flag_execute))
        .route("/api/query/assemble/designer/table/build/{flag}/{status}", get(crate::table_flag_status_build))
        .route("/api/query/assemble/designer/table/draft/{flag}/{status}", get(crate::table_flag_status_draft))
        .route("/api/query/assemble/designer/table/permission/{id}", get(crate::table_id_permission))
        .route("/api/query/assemble/designer/table/list/{query}/{flag}", get(crate::table_list_query_flag))
        .route("/api/query/assemble/designer/table/list/row/{tableFlag}/{id}/{next}/{count}", get(crate::table_list_tableFlag_row_id_next_count))
        .route("/api/query/assemble/designer/table/list/row/select/where/where/{tableFlag}", get(crate::table_list_tableFlag_row_select_where_where))
        .route("/api/query/assemble/designer/table/build/dispatch/{query}", get(crate::table_query_build_dispatch))
        .route("/api/query/assemble/designer/table/row/{tableFlag}", get(crate::table_tableFlag_row))
        .route("/api/query/assemble/designer/table/row/where/where/{tableFlag}/{count}", get(crate::table_tableFlag_row_count_where_where))
        .route("/api/query/assemble/designer/table/row/delete/all/{tableFlag}", post(crate::table_tableFlag_row_delete_all))
        .route("/api/query/assemble/designer/table/row/{tableFlag}/{id}", get(crate::table_tableFlag_row_id))
        .route("/api/query/assemble/designer/table/row/save/{tableFlag}", post(crate::table_tableFlag_row_save))
        .route("/api/query/assemble/designer/bundle/{view}/{id}", get(crate::view_id_bundle))
        .route("/api/query/assemble/designer/simulate/{view}/{id}", get(crate::view_id_simulate))
        .route("/api/query/assemble/designer/list/{view}/{id}/{next}/{count}", get(crate::view_list_id_next_count))
        .route("/api/query/assemble/designer/list/{view}/{query}/{flag}", get(crate::view_list_query_flag))
        .route("/api/query/assemble/designer/delete/{id}", delete(delete_designer))
        .route("/api/query/assemble/designer/save/{id}", put(save_designer))
        .route("/api/query/assemble/designer/table/row/delete/all/{tableFlag}", delete(table_tableFlag_row_delete_all))
        .route("/api/query/assemble/designer/table/row/save/{tableFlag}", put(table_tableFlag_row_save))
        // ── plan002 U2：已实现未注册 handler 补挂 ──
        .route("/api/query/assemble/designer/search", post(designer_search))
        .route("/api/query/assemble/designer/input/compare", put(input_compare))
        .route("/api/query/assemble/designer/input/cover", put(input_cover))
        .route("/api/query/assemble/designer/input/create", put(input_create))
        .route("/api/query/assemble/designer/input/prepare/cover", put(input_prepare_cover))
        .route("/api/query/assemble/designer/input/prepare/create", put(input_prepare_create))
        .route("/api/query/assemble/designer/neural/list/model", get(neural_list_model))
        .route("/api/query/assemble/designer/neural/model", post(neural_model))
        .route("/api/query/assemble/designer/output/list", get(output_list))
        .route("/api/query/assemble/designer/query/{flag}", get(query_flag))
        .route("/api/query/assemble/designer/list/all", get(query_list_all))
        .route("/api/query/assemble/designer/list/summary", get(query_list_summary))
        .route("/api/query/assemble/designer/querycategory/list", get(query_querycategory_list))
        .route("/api/query/assemble/designer/stat/list/{id}/prev/{count}", get(stat_list_id_prev_count))
        .route("/api/query/assemble/designer/table/list/manage", get(table_list_manage))
        .route("/api/query/assemble/designer/table/reload/dynamic", get(table_reload_dynamic))
        .route("/api/query/assemble/designer/table/list/row/{tableFlag}/{id}/prev/{count}", get(table_list_tableFlag_row_id_prev_count))
        .route("/api/query/assemble/designer/view/{id}", get(view_id))
        .route("/api/query/assemble/designer/view/permission/{id}", get(view_id_permission))
        .route("/api/query/assemble/designer/view/list/{id}/prev/{count}", get(view_list_id_prev_count))
        // ── plan002 U2：statement 全族（CRUD + 执行）──
        .route("/api/query/assemble/designer/statement", post(u2::statement_create))
        .route("/api/query/assemble/designer/statement/{flag}", get(u2::statement_get_flag).put(u2::statement_edit).delete(u2::statement_delete))
        .route("/api/query/assemble/designer/statement/list/manage", get(u2::statement_manage_list))
        .route("/api/query/assemble/designer/statement/list/query/{queryFlag}", post(u2::statement_list_with_query))
        .route("/api/query/assemble/designer/statement/permission/{id}", post(u2::statement_permission))
        .route("/api/query/assemble/designer/statement/execute/{flag}/page/{page}/size/{size}", post(u2::statement_execute_v2))
        .route("/api/query/assemble/designer/statement/execute/{flag}/mode/{mode}/page/{page}/size/{size}", post(u2::statement_execute_mode_v2))
        // ── plan002 U2：importmodel / neural / stat / table / view CRUD 缺口 ──
        .route("/api/query/assemble/designer/importmodel", post(u2::importmodel_create))
        .route("/api/query/assemble/designer/importmodel/edit/{id}", put(u2::importmodel_edit))
        .route("/api/query/assemble/designer/importmodel/delete/{id}", delete(u2::importmodel_delete))
        .route("/api/query/assemble/designer/neural/delete/model/{modelFlag}", delete(u2::neural_delete_model_modelFlag))
        .route("/api/query/assemble/designer/neural/update/model/{modelFlag}", put(u2::neural_update_model_modelFlag))
        .route("/api/query/assemble/designer/stat", post(u2::stat_create))
        .route("/api/query/assemble/designer/stat/edit/{id}", put(u2::stat_edit))
        .route("/api/query/assemble/designer/stat/delete/{id}", delete(u2::stat_delete))
        .route("/api/query/assemble/designer/table", post(create_table_definition))
        .route("/api/query/assemble/designer/table/edit/{flag}", put(update_table_definition))
        .route("/api/query/assemble/designer/table/delete/{flag}", delete(u2::table_delete))
        .route("/api/query/assemble/designer/table/row/insert/{tableFlag}", post(u2::table_tableFlag_row_insert))
        .route("/api/query/assemble/designer/table/row/update/{tableFlag}/{id}", put(u2::table_tableFlag_row_update))
        .route("/api/query/assemble/designer/table/row/delete/{tableFlag}/{id}", delete(u2::table_tableFlag_row_delete))
        .route("/api/query/assemble/designer/table/build/query/{query}", get(table_query_query_build))
        .route("/api/query/assemble/designer/view", post(u2::view_create))
        .route("/api/query/assemble/designer/view/edit/{id}", put(u2::view_edit))
        .route("/api/query/assemble/designer/view/delete/{id}", delete(u2::view_delete))
        .route("/api/query/assemble/designer/icon/set/{flag}", put(u2::query_set_icon))
        // ── plan002 U2 v9：o2server 精确路径/动词闭合（权威清单 docs/audits/o2server-endpoint-inventory.json）──
        .route("/api/query/assemble/designer/designer/search", post(u2::designer_search_v2))
        .route("/api/query/assemble/designer/id/{count}", get(u2::id_generate))
        .route("/api/query/assemble/designer/importmodel/list/query/{flag}", get(crate::importmodel_list_query_flag))
        .route("/api/query/assemble/designer/importmodel/{id}", get(u2::importmodel_get_flag).put(u2::importmodel_edit_flag).delete(u2::importmodel_delete_flag))
        .route("/api/query/assemble/designer/importmodel/{id}/permission", post(u2::importmodel_permission_set))
        .route("/api/query/assemble/designer/neural/model/{modelFlag}", put(u2::neural_update_model_modelFlag).delete(u2::neural_delete_model_modelFlag))
        .route("/api/query/assemble/designer/neural/model/{modelFlag}/reset/status", get(crate::neural_model_modelFlag_reset_status))
        .route("/api/query/assemble/designer/output/{flag}/select", put(u2::output_select_put))
        .route("/api/query/assemble/designer/output/{flag}/select/file", get(crate::output_flag_select_file))
        .route("/api/query/assemble/designer/query", post(u2::query_create_v2))
        .route("/api/query/assemble/designer/execute", post(u2::designer_execute))
        .route("/api/query/assemble/designer/stat/do", post(u2::stat_do))
        .route("/api/query/assemble/designer/query/entity/{entity}/category/{entityCategory}/properties", get(crate::query_entity_entity_category_entityCategory_properties))
        .route("/api/query/assemble/designer/query/list/all", get(crate::query_list_all))
        .route("/api/query/assemble/designer/query/list/querycategory/{queryCategory}", get(crate::query_list_querycategory_queryCategory))
        .route("/api/query/assemble/designer/query/list/summary", get(crate::query_list_summary))
        .route("/api/query/assemble/designer/query/list/summary/querycategory/{queryCategory}", get(crate::query_list_summary_querycategory_queryCategory))
        .route("/api/query/assemble/designer/query/querycategory/list", get(crate::query_querycategory_list))
        .route("/api/query/assemble/designer/query/{flag}", put(u2::query_edit_flag).delete(u2::query_delete_flag))
        .route("/api/query/assemble/designer/query/{flag}/icon", put(u2::query_icon_set))
        .route("/api/query/assemble/designer/query/{flag}/permission", post(u2::query_permission_set))
        .route("/api/query/assemble/designer/stat/list/query/{queryFlag}", get(crate::stat_list_query_flag))
        .route("/api/query/assemble/designer/stat/list/{id}/next/{count}", get(crate::stat_list_id_next_count))
        .route("/api/query/assemble/designer/stat/{id}", put(u2::stat_edit).delete(u2::stat_delete))
        .route("/api/query/assemble/designer/stat/{id}/permission", post(u2::stat_permission_set))
        .route("/api/query/assemble/designer/stat/{id}/simulate", put(u2::stat_simulate_put))
        .route("/api/query/assemble/designer/statement/{flag}/execute/mode/{mode}/page/{page}/size/{size}", post(u2::statement_execute_mode_v2))
        .route("/api/query/assemble/designer/statement/{flag}/execute/page/{page}/size/{size}", post(u2::statement_execute_v2))
        .route("/api/query/assemble/designer/statement/{flag}/permission", post(u2::statement_permission))
        .route("/api/query/assemble/designer/table/export/{tableFlag}/count/{count}", get(crate::table_export_tableFlag_count_count))
        .route("/api/query/assemble/designer/table/list/query/{flag}", get(crate::table_list_query_flag))
        .route("/api/query/assemble/designer/table/list/{flag}/row/select/where/{where}", get(crate::table_list_tableFlag_row_select_where_where))
        .route("/api/query/assemble/designer/table/list/{flag}/row/{id}/next/{count}", get(crate::table_list_tableFlag_row_id_next_count))
        .route("/api/query/assemble/designer/table/list/{flag}/row/{id}/prev/{count}", get(crate::table_list_tableFlag_row_id_prev_count))
        .route("/api/query/assemble/designer/table/query/{query}/build", get(crate::table_query_build_dispatch))
        .route("/api/query/assemble/designer/table/{flag}", put(update_table_definition).delete(u2::table_delete))
        .route("/api/query/assemble/designer/table/{flag}/build/dispatch", get(u2::table_build_dispatch_flag))
        .route("/api/query/assemble/designer/table/{flag}/execute", post(execute_table_definition))
        .route("/api/query/assemble/designer/table/{flag}/permission", post(u2::table_permission_set))
        .route("/api/query/assemble/designer/table/{flag}/row", post(u2::table_tableFlag_row_insert))
        .route("/api/query/assemble/designer/table/{flag}/row/count/where/{where}", get(crate::table_tableFlag_row_count_where_where))
        .route("/api/query/assemble/designer/table/{flag}/row/delete/all", delete(crate::table_tableFlag_row_delete_all))
        .route("/api/query/assemble/designer/table/{flag}/row/save", post(crate::table_tableFlag_row_save))
        .route("/api/query/assemble/designer/table/{flag}/row/{id}", get(crate::table_tableFlag_row_id).put(u2::table_tableFlag_row_update).delete(u2::table_tableFlag_row_delete))
        .route("/api/query/assemble/designer/table/{flag}/status/build", get(crate::table_flag_status_build))
        .route("/api/query/assemble/designer/table/{flag}/status/draft", get(crate::table_flag_status_draft))
        .route("/api/query/assemble/designer/view/list/query/{queryFlag}", get(crate::view_list_query_flag))
        .route("/api/query/assemble/designer/view/list/{id}/next/{count}", get(crate::view_list_id_next_count))
        .route("/api/query/assemble/designer/view/{id}", put(u2::view_edit).delete(u2::view_delete))
        .route("/api/query/assemble/designer/view/{id}/bundle", put(u2::view_bundle_put))
        .route("/api/query/assemble/designer/view/{id}/permission", post(u2::view_permission_set))
        .route("/api/query/assemble/designer/view/{id}/simulate", put(u2::view_simulate_put))
        // ── importer / stat 斜杠路径家族（设计器桌面视图，shared::crud 通用参数化写）──
        .route("/api/query/assemble/designer/importer/list", get(importer_list))
        .route("/api/query/assemble/designer/importer/create", post(importer_create))
        .route("/api/query/assemble/designer/importer/save/{id}", put(importer_save))
        .route("/api/query/assemble/designer/importer/save/{id}", post(importer_save))
        .route("/api/query/assemble/designer/importer/delete/{id}", delete(importer_delete))
        .route("/api/query/assemble/designer/importer/delete/{id}", post(importer_delete))
        .route("/api/query/assemble/designer/stat/list", get(stat_list))
        .route("/api/query/assemble/designer/stat/create", post(stat_create))
        .route("/api/query/assemble/designer/stat/save/{id}", put(stat_save))
        .route("/api/query/assemble/designer/stat/save/{id}", post(stat_save))
        // DELETE /stat/delete/{id} 已由 U2 的 u2::stat_delete 占用（同表软删），此处仅补 POST
        .route("/api/query/assemble/designer/stat/delete/{id}", post(stat_delete));

    if let Some(pool) = pool {
        router.layer(Extension(pool))
    } else {
        router
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_assemble_designer_router(Some(pool))
}

#[allow(non_snake_case)]
pub async fn designer_search(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, create_time, update_time FROM x_query_design WHERE deleted_at IS NULL ORDER BY update_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "updateTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("update_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn id_count(
    pool: Extension<Pool>,
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM x_query_design WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let total: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "total".to_string(),
                Value::Number(serde_json::Number::from(total)),
            ),
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(count)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn importmodel_list_query_flag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "modelFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("model_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn importmodel_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "modelFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("model_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "content".to_string(),
                    Value::String(row.get::<_, Option<String>>("content").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn importmodel_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_import_model WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "permission".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("permission")
                            .unwrap_or_default(),
                    ),
                ),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn input_compare(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();

    let row = client
        .query_opt(
            "SELECT id, content FROM x_query_input WHERE id = $1",
            &[&input_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let old_content: Option<String> = row.get("content");
            let new_content = body
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let old_str = old_content.unwrap_or_default();
            let compared = !old_str.is_empty() && old_str == new_content;
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(input_id.to_string())),
                    ("oldContent".to_string(), Value::String(old_str)),
                    (
                        "newContent".to_string(),
                        Value::String(new_content.to_string()),
                    ),
                    ("compared".to_string(), Value::Bool(compared)),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("input not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn input_cover(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let content_str = body
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_input SET content = $1, update_time = NOW() WHERE id = $2",
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
            (
                "covered".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn input_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let content = body
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let creator = "system";

    let result = client
        .execute(
            "INSERT INTO x_query_input (id, content, creator, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            (
                "saved".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn input_prepare_cover(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let input_id = body.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let row = client
        .query_opt(
            "SELECT id, content FROM x_query_input WHERE id = $1",
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
                    (
                        "content".to_string(),
                        content
                            .map(Value::String)
                            .unwrap_or(Value::String("".to_string())),
                    ),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("input not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn input_prepare_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let content = body
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let creator = "system";

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "INSERT INTO x_query_input (id, content, creator, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            (
                "saved".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn neural_generate_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'generating', update_time = NOW() WHERE flag = $1",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            (
                "generating".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn neural_learn_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'learning', update_time = NOW() WHERE flag = $1",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            (
                "learning".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn neural_list_model(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, status, creator, create_time FROM x_query_neural_model WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "flag".to_string(),
                    Value::String(row.get::<_, Option<String>>("flag").unwrap_or_default()),
                ),
                (
                    "status".to_string(),
                    Value::String(row.get::<_, Option<String>>("status").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn neural_model(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let flag = body
        .get("flag")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let creator = "system";

    let id = uuid::Uuid::new_v4().to_string();
    let result = client
        .execute(
            "INSERT INTO x_query_neural_model (id, name, flag, status, creator, create_time) VALUES ($1, $2, $3, 'idle', $4, NOW())",
            &[&id, &name, &flag, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name.to_string())),
            ("flag".to_string(), Value::String(flag.to_string())),
            (
                "created".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn neural_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, status, creator, create_time FROM x_query_neural_model WHERE flag = $1 AND deleted_at IS NULL",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "flag".to_string(),
                    Value::String(row.get::<_, Option<String>>("flag").unwrap_or_default()),
                ),
                (
                    "status".to_string(),
                    Value::String(row.get::<_, Option<String>>("status").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("neural model not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn neural_model_modelFlag_reset_status(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'idle', update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE flag = $1",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            (
                "reset".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn neural_stop_generating_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'idle', update_time = NOW() WHERE flag = $1 AND status = 'generating'",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error(
            "model not found or not generating",
        )));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            (
                "stopped".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn neural_stop_learn_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'idle', update_time = NOW() WHERE flag = $1 AND status = 'learning'",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found or not learning")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            (
                "stopped".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn output_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_query_output WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "flag".to_string(),
                    Value::String(row.get::<_, Option<String>>("flag").unwrap_or_default()),
                ),
                (
                    "appName".to_string(),
                    Value::String(row.get::<_, Option<String>>("app_name").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn output_flag_select_file(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, flag, select_file FROM x_query_output WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "flag".to_string(),
                    Value::String(row.get::<_, Option<String>>("flag").unwrap_or_default()),
                ),
                (
                    "selectFile".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("select_file")
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("output not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn output_queryFlag_select(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, app_name, creator, create_time FROM x_query_output WHERE query_flag = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "flag".to_string(),
                    Value::String(row.get::<_, Option<String>>("flag").unwrap_or_default()),
                ),
                (
                    "appName".to_string(),
                    Value::String(row.get::<_, Option<String>>("app_name").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn query_entity_entity_category_entityCategory_properties(
    pool: Extension<Pool>,
    Path((entity, entity_category)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT field_name, field_label, field_type FROM x_query_entity_property WHERE entity = $1 AND category = $2 ORDER BY sort_order",
            &[&entity, &entity_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "fieldName".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("field_name")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "fieldLabel".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("field_label")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "fieldType".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("field_type")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("properties".to_string(), Value::Array(data))]),
    ))))
}

#[allow(non_snake_case)]
pub async fn query_list_all(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn query_list_querycategory_queryCategory(
    pool: Extension<Pool>,
    Path(query_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design WHERE category = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&query_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn query_list_summary(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design WHERE deleted_at IS NULL ORDER BY create_time DESC LIMIT 50",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
            ]))
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
pub async fn query_list_summary_querycategory_queryCategory(
    pool: Extension<Pool>,
    Path(query_category): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, category FROM x_query_design WHERE category = $1 AND deleted_at IS NULL ORDER BY name",
            &[&query_category],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
            ]))
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
pub async fn query_querycategory_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT DISTINCT category FROM x_query_design WHERE deleted_at IS NULL AND category IS NOT NULL ORDER BY category",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([(
                "category".to_string(),
                Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
            )]))
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
pub async fn query_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "category".to_string(),
                    Value::String(row.get::<_, Option<String>>("category").unwrap_or_default()),
                ),
                (
                    "query".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_definition")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("query not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn query_flag_icon(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, icon FROM x_query_design WHERE flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "icon".to_string(),
                    Value::String(row.get::<_, Option<String>>("icon").unwrap_or_default()),
                ),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("query not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn query_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_design WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "permission".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("permission")
                            .unwrap_or_default(),
                    ),
                ),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("query not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn stat_list_query_flag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, query_flag, stat_type, creator, create_time FROM x_query_stat WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "statType".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("stat_type")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn stat_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, query_flag, stat_type, creator, create_time FROM x_query_stat WHERE id > $1 ORDER BY id ASC LIMIT $2",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "statType".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("stat_type")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn stat_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, query_flag, stat_type, creator, create_time FROM x_query_stat WHERE id < $1 ORDER BY id DESC LIMIT $2",
            &[&id, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "statType".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("stat_type")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn stat_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, query_flag, stat_type, config, creator, create_time FROM x_query_stat WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "statType".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("stat_type")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "config".to_string(),
                    Value::String(row.get::<_, Option<String>>("config").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("stat not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn stat_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_stat WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "permission".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("permission")
                            .unwrap_or_default(),
                    ),
                ),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("stat not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn stat_id_simulate(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, config FROM x_query_stat WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "config".to_string(),
                    Value::String(row.get::<_, Option<String>>("config").unwrap_or_default()),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("stat not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn table_export_tableFlag_count_count(
    pool: Extension<Pool>,
    Path((table_flag, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 LIMIT $2",
            &[&table_flag, &count],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "data".to_string(),
                    Value::String(row.get::<_, Option<String>>("data").unwrap_or_default()),
                ),
            ]))
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
pub async fn table_list_manage(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, table_flag, query_flag, columns, status, creator, create_time, update_time FROM x_query_table WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "columns".to_string(),
                    serde_json::from_str::<Value>(
                        &row.get::<_, Option<String>>("columns").unwrap_or_default(),
                    )
                    .unwrap_or(Value::Array(Vec::new())),
                ),
                (
                    "status".to_string(),
                    Value::String(row.get::<_, Option<String>>("status").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "updateTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("update_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn table_list_query_flag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, table_flag, query_flag, creator, create_time FROM x_query_table WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
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
pub async fn table_list_tableFlag_row_select_where_where(
    pool: Extension<Pool>,
    Path((table_flag, _where)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            &"SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND data ILIKE $2 ORDER BY id DESC".to_string(),
            &[&table_flag, &format!("%{}%", _where)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "data".to_string(),
                    Value::String(row.get::<_, Option<String>>("data").unwrap_or_default()),
                ),
            ]))
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
pub async fn table_list_tableFlag_row_id_next_count(
    pool: Extension<Pool>,
    Path((table_flag, id, count)): Path<(String, String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "data".to_string(),
                    Value::String(row.get::<_, Option<String>>("data").unwrap_or_default()),
                ),
            ]))
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
pub async fn table_list_tableFlag_row_id_prev_count(
    pool: Extension<Pool>,
    Path((table_flag, id, count)): Path<(String, String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "data".to_string(),
                    Value::String(row.get::<_, Option<String>>("data").unwrap_or_default()),
                ),
            ]))
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
pub async fn table_query_query_build(
    pool: Extension<Pool>,
    Path(query): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'build', update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE query_flag = $1 AND deleted_at IS NULL",
            &[&query],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("queryFlag".to_string(), Value::String(query)),
            (
                "built".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn table_reload_dynamic(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET reloaded = true, update_time = NOW() WHERE reloaded = false",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            (
                "reloaded".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
            (
                "value".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn table_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, table_flag, query_flag, columns, status, creator, create_time, update_time FROM x_query_table WHERE table_flag = $1 AND deleted_at IS NULL LIMIT 1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "columns".to_string(),
                    serde_json::from_str::<Value>(
                        &row.get::<_, Option<String>>("columns").unwrap_or_default(),
                    )
                    .unwrap_or(Value::Array(Vec::new())),
                ),
                (
                    "status".to_string(),
                    Value::String(row.get::<_, Option<String>>("status").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "updateTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("update_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("table not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn table_flag_execute(
    pool: Extension<Pool>,
    Path(_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let sql = body.get("sql").and_then(|v| v.as_str()).unwrap_or_default();

    validate_single_select(sql).map_err(AppError::BadRequest)?;

    let limited_sql = ensure_limit(sql, 500);
    let rows = client
        .query(&limited_sql, &[])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(row_to_json).collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

#[allow(non_snake_case)]
pub async fn table_flag_status_build(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'build', update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE table_flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("table not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(flag)),
            ("status".to_string(), Value::String("build".to_string())),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn table_flag_status_draft(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'draft', update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE table_flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("table not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(flag)),
            ("status".to_string(), Value::String("draft".to_string())),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn table_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_table WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "permission".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("permission")
                            .unwrap_or_default(),
                    ),
                ),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("table not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn table_query_build_dispatch(
    pool: Extension<Pool>,
    Path(query): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'build', update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE query_flag = $1 AND deleted_at IS NULL",
            &[&query],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("queryFlag".to_string(), Value::String(query)),
            (
                "built".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn table_tableFlag_row(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "data".to_string(),
                    Value::String(row.get::<_, Option<String>>("data").unwrap_or_default()),
                ),
            ]))
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
pub async fn table_tableFlag_row_count_where_where(
    pool: Extension<Pool>,
    Path((table_flag, _where)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            &"SELECT COUNT(*) as cnt FROM x_query_table_data WHERE table_flag = $1 AND data ILIKE $2".to_string(),
            &[&table_flag, &format!("%{}%", _where)],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let count: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(table_flag)),
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(count)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn table_tableFlag_row_delete_all(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
            (
                "deleted".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
            (
                "count".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn table_tableFlag_row_save(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "INSERT INTO x_query_table_data (id, table_flag, data, create_time) VALUES ($1, $2, $3, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
            &[&id, &table_flag, &data_str],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("tableFlag".to_string(), Value::String(table_flag)),
            (
                "saved".to_string(),
                Value::Number(serde_json::Number::from(result as i64)),
            ),
        ]),
    ))))
}

#[allow(non_snake_case)]
pub async fn table_tableFlag_row_id(
    pool: Extension<Pool>,
    Path((table_flag, id)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND id = $2",
            &[&table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "tableFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("table_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "data".to_string(),
                    Value::String(row.get::<_, Option<String>>("data").unwrap_or_default()),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("row not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn view_list_query_flag(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, view_flag, query_flag, creator, to_char(create_time,'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_query_view WHERE query_flag = $1 ORDER BY create_time DESC",
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
                (
                    "viewFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("view_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get("create_time")),
                ),
            ]))
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
pub async fn view_list_id_next_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, view_flag, creator, to_char(create_time,'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_query_view WHERE id > $1 ORDER BY id ASC LIMIT $2",
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
                (
                    "viewFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("view_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get("create_time")),
                ),
            ]))
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
pub async fn view_list_id_prev_count(
    pool: Extension<Pool>,
    Path((id, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, view_flag, creator, to_char(create_time,'YYYY-MM-DD HH24:MI:SS') AS create_time FROM x_query_view WHERE id < $1 ORDER BY id DESC LIMIT $2",
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
                (
                    "viewFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("view_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get("create_time")),
                ),
            ]))
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
pub async fn view_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
                (
                    "viewFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("view_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "content".to_string(),
                    Value::String(row.get::<_, Option<String>>("content").unwrap_or_default()),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(row.get("create_time")),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn view_id_bundle(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, view_flag, bundle_data FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                (
                    "viewFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("view_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "bundle".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("bundle_data")
                            .unwrap_or_default(),
                    ),
                ),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn view_id_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, permission FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                (
                    "permission".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("permission")
                            .unwrap_or_default(),
                    ),
                ),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

#[allow(non_snake_case)]
pub async fn view_id_simulate(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, content FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let result = Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                (
                    "content".to_string(),
                    Value::String(row.get::<_, Option<String>>("content").unwrap_or_default()),
                ),
            ]));
            Ok(Json(ActionResult::success(result)))
        }
        None => Ok(Json(ActionResult::error("view not found"))),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// importer / stat 斜杠路径家族（设计器桌面视图 + shared::crud 通用参数化写）
// ─────────────────────────────────────────────────────────────────────────────

// ── importer/list（查询导入设计器，查 x_query_import_model）──
#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn importer_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, model_flag, query_flag, creator, create_time FROM x_query_import_model WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "modelFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("model_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

// ── importer 家族 CRUD（x_query_import_model，通用参数化写）──
fn importer_spec() -> shared::crud::CrudSpec {
    shared::crud::CrudSpec {
        table: "x_query_import_model",
        columns: &[
            ("name", "name"),
            ("modelFlag", "model_flag"),
            ("queryFlag", "query_flag"),
            ("content", "content"),
            ("creator", "creator"),
        ],
        soft_delete: true,
    }
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn importer_create(
    pool: Extension<Pool>,
    body: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = shared::crud_create(&pool, &importer_spec(), &body.0).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn importer_save(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    body: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let saved = shared::crud_save(&pool, &importer_spec(), &id, &body.0).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(saved)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn importer_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let deleted = shared::crud_delete(&pool, &importer_spec(), &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(deleted)),
        ]),
    ))))
}

// ── stat/list（查询统计设计器，查 x_query_stat）──
#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn stat_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, query_flag, stat_type, creator, create_time FROM x_query_stat WHERE deleted_at IS NULL ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                (
                    "id".to_string(),
                    Value::String(row.get::<_, Option<String>>("id").unwrap_or_default()),
                ),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "queryFlag".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("query_flag")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "statType".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("stat_type")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "creator".to_string(),
                    Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
                ),
                (
                    "createTime".to_string(),
                    Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
        Value::Array(data),
        count,
        0,
    )))
}

// ── stat 家族 CRUD（x_query_stat，通用参数化写）──
fn stat_spec() -> shared::crud::CrudSpec {
    shared::crud::CrudSpec {
        table: "x_query_stat",
        columns: &[
            ("name", "name"),
            ("queryFlag", "query_flag"),
            ("statType", "stat_type"),
            ("creator", "creator"),
        ],
        soft_delete: true,
    }
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn stat_create(
    pool: Extension<Pool>,
    body: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let id = shared::crud_create(&pool, &stat_spec(), &body.0).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("created".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn stat_save(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    body: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let saved = shared::crud_save(&pool, &stat_spec(), &id, &body.0).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("saved".to_string(), Value::Bool(saved)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn stat_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let deleted = shared::crud_delete(&pool, &stat_spec(), &id).await?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(deleted)),
        ]),
    ))))
}
