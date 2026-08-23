//! plan002 U2 缺口闭合：statement 全族 + importmodel/neural/stat/table/view CRUD
//!
//! statement 执行链路：按 flag/alias 加载语句 → sqlparser 校验（仅单条 SELECT）
//! → `:param` 占位符参数化 → 分页包裹执行；mode 含 "count" 时附带 COUNT 总数。
//! CRUD 创建路径统一做 trim+lowercase 归一化查重。

use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::row_to_json, response::ActionResult};
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

/// sqlparser 安全约束：仅允许单条 SELECT 语句，拒绝 DML/DDL/多语句。
pub fn validate_single_select(sql: &str) -> Result<(), String> {
    let trimmed = sql.trim();
    if trimmed.is_empty() {
        return Err("sql is required".to_string());
    }
    let statements = Parser::parse_sql(&PostgreSqlDialect {}, trimmed)
        .map_err(|e| format!("SQL parse error: {}", e))?;
    if statements.len() != 1 {
        return Err("only single statement allowed".to_string());
    }
    match &statements[0] {
        Statement::Query(_) => Ok(()),
        _ => Err(
            "only SELECT queries are allowed (INSERT/UPDATE/DELETE/DDL rejected)".to_string(),
        ),
    }
}

/// 无 LIMIT 时注入 LIMIT max_rows，防止全表拖取。
pub fn ensure_limit(sql: &str, max_rows: i64) -> String {
    let trimmed = sql.trim().trim_end_matches(';');
    if trimmed.to_uppercase().contains(" LIMIT ") {
        trimmed.to_string()
    } else {
        format!("{} LIMIT {}", trimmed, max_rows)
    }
}

/// 归一化标识符用于查重：trim + 小写。
pub fn normalize_identifier(s: &str) -> String {
    s.trim().to_lowercase()
}

/// 参数值类型化包装：数字→INT8/FLOAT8，布尔→BOOL，其余→TEXT。
#[derive(Debug)]
pub enum SqlParam {
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl SqlParam {
    fn from_value(v: &Value) -> Self {
        match v {
            Value::Null => SqlParam::Null,
            Value::Bool(b) => SqlParam::Bool(*b),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    SqlParam::Int(i)
                } else {
                    SqlParam::Float(n.as_f64().unwrap_or_default())
                }
            }
            other => SqlParam::Text(other.to_string()),
        }
    }

    fn as_to_sql(&self) -> &(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync) {
        match self {
            SqlParam::Text(s) => s,
            SqlParam::Int(i) => i,
            SqlParam::Float(f) => f,
            SqlParam::Bool(b) => b,
            SqlParam::Null => &Option::<String>::None,
        }
    }
}

/// 将 SQL 中的 `:name` 占位符替换为 $N 并按 parameters 取值参数化。
/// 单引号字符串字面量内的冒号、`::` 类型转换不参与替换。
pub fn parameterize_statement_sql(sql: &str, params: &Value) -> (String, Vec<SqlParam>) {
    let mut out = String::with_capacity(sql.len());
    let mut values: Vec<SqlParam> = Vec::new();
    let bytes = sql.as_bytes();
    let mut in_string = false;
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '\'' {
            in_string = !in_string;
            out.push(c);
            i += 1;
            continue;
        }
        if !in_string && c == ':' {
            if bytes.get(i + 1) == Some(&b':') {
                // PostgreSQL 类型转换 `::` 不参与参数化
                out.push_str("::");
                i += 2;
                continue;
            }
            let mut j = i + 1;
            let mut name = String::new();
            while j < bytes.len() {
                let ch = bytes[j] as char;
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    name.push(ch);
                    j += 1;
                } else {
                    break;
                }
            }
            if !name.is_empty() {
                values.push(SqlParam::from_value(params.get(&name).unwrap_or(&Value::Null)));
                out.push_str(&format!("${}", values.len()));
                i = j;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    (out, values)
}

fn bind_params(
    values: &[SqlParam],
) -> Vec<&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)> {
    values.iter().map(|v| v.as_to_sql()).collect()
}

fn statement_row_json(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        (
            "name".to_string(),
            Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
        ),
        (
            "alias".to_string(),
            Value::String(row.get::<_, Option<String>>("alias").unwrap_or_default()),
        ),
        (
            "queryFlag".to_string(),
            Value::String(row.get::<_, Option<String>>("query_flag").unwrap_or_default()),
        ),
        (
            "entityClassName".to_string(),
            Value::String(row.get::<_, Option<String>>("entity_class").unwrap_or_default()),
        ),
        (
            "type".to_string(),
            Value::String(row.get::<_, Option<String>>("type").unwrap_or_default()),
        ),
        (
            "data".to_string(),
            Value::String(row.get::<_, Option<String>>("data").unwrap_or_default()),
        ),
        (
            "countingData".to_string(),
            Value::String(row.get::<_, Option<String>>("counting_data").unwrap_or_default()),
        ),
        (
            "creator".to_string(),
            Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
        ),
    ]))
}

async fn statement_duplicate_exists(
    client: &deadpool_postgres::Client,
    name: &str,
    alias: &str,
) -> Result<bool, AppError> {
    let norm_name = normalize_identifier(name);
    let norm_alias = normalize_identifier(alias);
    let row = client
        .query_one(
            "SELECT COUNT(*) AS cnt FROM x_query_statement \
             WHERE ($1 <> '' AND LOWER(TRIM(COALESCE(name,''))) = $1) \
                OR ($2 <> '' AND LOWER(TRIM(COALESCE(alias,''))) = $2)",
            &[&norm_name, &norm_alias],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(row.get::<_, i64>("cnt") > 0)
}

// ── statement 族 ─────────────────────────────────────────────────────────────

pub async fn statement_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    let alias = body.get("alias").and_then(|v| v.as_str()).unwrap_or_default();
    let query_flag = body.get("queryFlag").and_then(|v| v.as_str()).unwrap_or("");
    let entity_class = body
        .get("entityClassName")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let entity_category = body
        .get("entityCategory")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let stmt_type = body.get("type").and_then(|v| v.as_str()).unwrap_or("select");
    let data = body.get("data").and_then(|v| v.as_str()).unwrap_or("");
    let counting_data = body
        .get("countingData")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if name.trim().is_empty() || data.trim().is_empty() {
        return Ok(Json(ActionResult::error("name and data are required")));
    }

    // 归一化查重：name/alias 归一化后重复即拒绝
    if statement_duplicate_exists(&client, name, alias).await? {
        tracing::warn!("statement create rejected: duplicate name/alias '{}'", name);
        return Ok(Json(ActionResult::error(
            "statement name or alias already exists",
        )));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_statement (id, name, alias, query_flag, entity_class, entity_category, type, data, counting_data, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, NOW(), NOW())",
            &[&id, &name, &alias, &query_flag, &entity_class, &entity_category, &stmt_type, &data, &counting_data, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("name".to_string(), Value::String(name.to_string())),
        ]),
    ))))
}

pub async fn statement_edit(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let alias = body.get("alias").and_then(|v| v.as_str()).unwrap_or("");
    let data = body.get("data").and_then(|v| v.as_str()).unwrap_or("");
    let counting_data = body
        .get("countingData")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if !alias.is_empty() {
        let dup_row = client
            .query_one(
                "SELECT COUNT(*) AS cnt FROM x_query_statement WHERE LOWER(TRIM(COALESCE(alias,''))) = $1 AND id <> $2",
                &[&normalize_identifier(alias), &flag],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if dup_row.get::<_, i64>("cnt") > 0 {
            return Ok(Json(ActionResult::error("statement alias already exists")));
        }
    }

    let result = client
        .execute(
            "UPDATE x_query_statement SET alias = COALESCE(NULLIF($1,''), alias), data = COALESCE(NULLIF($2,''), data), counting_data = COALESCE(NULLIF($3,''), counting_data), update_time = NOW() WHERE id = $4",
            &[&alias, &data, &counting_data, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("statement not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn statement_delete(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_query_statement WHERE id = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("statement not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(flag))]),
    ))))
}

pub async fn statement_get_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, alias, query_flag, entity_class, type, data, counting_data, creator FROM x_query_statement WHERE id = $1 OR (COALESCE(alias,'') <> '' AND alias = $1) LIMIT 1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(statement_row_json(&row)))),
        None => Ok(Json(ActionResult::error("statement not found"))),
    }
}

pub async fn statement_manage_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, alias, query_flag, entity_class, type, data, counting_data, creator FROM x_query_statement ORDER BY create_time DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(statement_row_json).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn statement_list_with_query(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, alias, query_flag, entity_class, type, data, counting_data, creator FROM x_query_statement WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(statement_row_json).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn statement_permission(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let permission = serde_json::to_string(
        body.get("permissionList")
            .unwrap_or(&Value::Array(Vec::new())),
    )
    .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_statement SET permission = $1, update_time = NOW() WHERE id = $2",
            &[&permission, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("statement not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("permissionSet".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// statement 执行核心：加载语句 → sqlparser 校验 → :param 参数化 → 分页执行。
async fn execute_statement_by_flag(
    client: &deadpool_postgres::Client,
    flag: &str,
    mode: &str,
    page: i64,
    size: i64,
    parameters: &Value,
) -> Result<Value, AppError> {
    let row = client
        .query_opt(
            "SELECT id, name, data FROM x_query_statement WHERE id = $1 OR (COALESCE(alias,'') <> '' AND alias = $1) LIMIT 1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row = match row {
        Some(r) => r,
        None => {
            tracing::warn!("statement '{}' not found for execute", flag);
            return Err(AppError::NotFound);
        }
    };

    let raw_sql: String = row.get::<_, Option<String>>("data").unwrap_or_default();

    // 无法安全执行的语句显式拒绝（400），绝不静默放行
    validate_single_select(&raw_sql).map_err(|e| {
        tracing::warn!("statement '{}' execute rejected: {}", flag, e);
        AppError::BadRequest(e)
    })?;

    let (parameterized_sql, values) = parameterize_statement_sql(&raw_sql, parameters);
    let binds = bind_params(&values);

    let mut payload = serde_json::Map::new();
    payload.insert("id".to_string(), Value::String(row.get("id")));
    payload.insert(
        "name".to_string(),
        Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
    );

    if mode.contains("count") {
        let count_sql = format!(
            "SELECT COUNT(*) AS total FROM ({}) AS stmt_count_sub",
            parameterized_sql.trim().trim_end_matches(';')
        );
        let count_row = client
            .query_one(&count_sql, &binds[..])
            .await
            .map_err(|_| AppError::Internal)?;
        let total: i64 = count_row.get("total");
        payload.insert("total".to_string(), Value::Number(serde_json::Number::from(total)));
    }

    let offset = if page > 0 { (page - 1) * size } else { 0 };
    let paged_sql = format!(
        "SELECT * FROM ({}) AS stmt_page_sub LIMIT ${} OFFSET ${}",
        parameterized_sql.trim().trim_end_matches(';'),
        values.len() + 1,
        values.len() + 2
    );

    let mut page_binds = binds.clone();
    page_binds.push(&size);
    page_binds.push(&offset);

    let rows = client
        .query(&paged_sql, &page_binds[..])
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(row_to_json).collect();
    payload.insert(
        "count".to_string(),
        Value::Number(serde_json::Number::from(data.len() as i64)),
    );
    payload.insert("data".to_string(), Value::Array(data));

    Ok(Value::Object(payload))
}

pub async fn statement_execute_v2(
    pool: Extension<Pool>,
    Path((flag, page, size)): Path<(String, i64, i64)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let parameters = body.get("parameter").cloned().unwrap_or(Value::Null);
    let result = execute_statement_by_flag(&client, &flag, "", page, size, &parameters).await?;
    Ok(Json(ActionResult::success(result)))
}

pub async fn statement_execute_mode_v2(
    pool: Extension<Pool>,
    Path((flag, mode, page, size)): Path<(String, String, i64, i64)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let parameters = body.get("parameter").cloned().unwrap_or(Value::Null);
    let result = execute_statement_by_flag(&client, &flag, &mode, page, size, &parameters).await?;
    Ok(Json(ActionResult::success(result)))
}

// ── importmodel CRUD ─────────────────────────────────────────────────────────

pub async fn importmodel_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    // 归一化查重
    let norm = normalize_identifier(name);
    let dup_row = client
        .query_one(
            "SELECT COUNT(*) AS cnt FROM x_query_import_model WHERE LOWER(TRIM(COALESCE(name,''))) = $1",
            &[&norm],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if dup_row.get::<_, i64>("cnt") > 0 {
        return Ok(Json(ActionResult::error("import model name already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let model_flag = uuid::Uuid::new_v4().to_string();
    let query_flag = body.get("queryFlag").and_then(|v| v.as_str()).unwrap_or("");
    let content = body.get("data").and_then(|v| v.as_str()).unwrap_or("");
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_import_model (id, name, model_flag, query_flag, content, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[&id, &name, &model_flag, &query_flag, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("modelFlag".to_string(), Value::String(model_flag)),
        ]),
    ))))
}

pub async fn importmodel_edit(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let content = body.get("data").and_then(|v| v.as_str()).unwrap_or("");

    let result = client
        .execute(
            "UPDATE x_query_import_model SET name = COALESCE(NULLIF($1,''), name), content = COALESCE(NULLIF($2,''), content), update_time = NOW() WHERE id = $3",
            &[&name, &content, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("import model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn importmodel_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_query_import_model WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("import model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

// ── neural delete/update ─────────────────────────────────────────────────────

pub async fn neural_delete_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_query_neural_model WHERE flag = $1",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("neural model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("modelFlag".to_string(), Value::String(model_flag))]),
    ))))
}

pub async fn neural_update_model_modelFlag(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET name = COALESCE(NULLIF($1,''), name), update_time = NOW() WHERE flag = $2",
            &[&name, &model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("neural model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("updated".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── stat create/edit/delete ──────────────────────────────────────────────

pub async fn stat_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    // 归一化查重
    let norm = normalize_identifier(name);
    let dup_row = client
        .query_one(
            "SELECT COUNT(*) AS cnt FROM x_query_stat WHERE LOWER(TRIM(COALESCE(name,''))) = $1",
            &[&norm],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if dup_row.get::<_, i64>("cnt") > 0 {
        return Ok(Json(ActionResult::error("stat name already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let query_flag = body.get("queryFlag").and_then(|v| v.as_str()).unwrap_or("");
    let stat_type = body.get("type").and_then(|v| v.as_str()).unwrap_or("select");
    let config = match body.get("config") {
        Some(v) => serde_json::to_string(v).map_err(|_| AppError::Internal)?,
        None => String::new(),
    };
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_stat (id, name, query_flag, stat_type, config, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())",
            &[&id, &name, &query_flag, &stat_type, &config, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub async fn stat_edit(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let config = match body.get("config") {
        Some(v) => serde_json::to_string(v).map_err(|_| AppError::Internal)?,
        None => String::new(),
    };

    let result = client
        .execute(
            "UPDATE x_query_stat SET name = COALESCE(NULLIF($1,''), name), config = COALESCE(NULLIF($2,''), config), update_time = NOW() WHERE id = $3",
            &[&name, &config, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("stat not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn stat_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_query_stat WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("stat not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

// ── table CRUD + 行级操作补齐 ────────────────────────────────────────────

pub async fn table_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    // 归一化查重
    let norm = normalize_identifier(name);
    let dup_row = client
        .query_one(
            "SELECT COUNT(*) AS cnt FROM x_query_table WHERE LOWER(TRIM(COALESCE(name,''))) = $1 AND deleted_at IS NULL",
            &[&norm],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if dup_row.get::<_, i64>("cnt") > 0 {
        return Ok(Json(ActionResult::error("table name already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let table_flag = uuid::Uuid::new_v4().to_string();
    let query_flag = body.get("queryFlag").and_then(|v| v.as_str()).unwrap_or("");
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_table (id, name, table_flag, query_flag, status, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, 'draft', $5, NOW(), NOW())",
            &[&id, &name, &table_flag, &query_flag, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("tableFlag".to_string(), Value::String(table_flag)),
        ]),
    ))))
}

pub async fn table_edit(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");

    let result = client
        .execute(
            "UPDATE x_query_table SET name = COALESCE(NULLIF($1,''), name), update_time = NOW() WHERE table_flag = $2",
            &[&name, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("table not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(flag)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_delete(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_query_table WHERE table_flag = $1", &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("table not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(flag)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn table_tableFlag_row_insert(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "INSERT INTO x_query_table_data (id, table_flag, data, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &table_flag, &data_str],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("inserted".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_tableFlag_row_update(
    pool: Extension<Pool>,
    Path((table_flag, id)): Path<(String, String)>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

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
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn table_tableFlag_row_delete(
    pool: Extension<Pool>,
    Path((table_flag, id)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_query_table_data WHERE table_flag = $1 AND id = $2",
            &[&table_flag, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("row not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

// ── view create/edit/delete + query icon ─────────────────────────────────────

pub async fn view_create(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    // 归一化查重
    let norm = normalize_identifier(name);
    let dup_row = client
        .query_one(
            "SELECT COUNT(*) AS cnt FROM x_query_view WHERE LOWER(TRIM(COALESCE(name,''))) = $1 AND deleted_at IS NULL",
            &[&norm],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if dup_row.get::<_, i64>("cnt") > 0 {
        return Ok(Json(ActionResult::error("view name already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let view_flag = uuid::Uuid::new_v4().to_string();
    let query_flag = body.get("queryFlag").and_then(|v| v.as_str()).unwrap_or("");
    let content = match body.get("data") {
        Some(v) => serde_json::to_string(v).map_err(|_| AppError::Internal)?,
        None => String::new(),
    };
    let creator = "system";

    client
        .execute(
            "INSERT INTO x_query_view (id, name, view_flag, query_flag, content, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &name, &view_flag, &query_flag, &content, &creator],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("viewFlag".to_string(), Value::String(view_flag)),
        ]),
    ))))
}

pub async fn view_edit(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let content = match body.get("data") {
        Some(v) => serde_json::to_string(v).map_err(|_| AppError::Internal)?,
        None => String::new(),
    };

    let result = client
        .execute(
            "UPDATE x_query_view SET name = COALESCE(NULLIF($1,''), name), content = COALESCE(NULLIF($2,''), content), update_time = NOW() WHERE id = $3",
            &[&name, &content, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

pub async fn view_delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute("DELETE FROM x_query_view WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("view not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

pub async fn query_set_icon(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let icon = body.get("icon").and_then(|v| v.as_str()).unwrap_or("");

    let result = client
        .execute(
            "UPDATE x_query_design SET icon = $1, update_time = NOW() WHERE flag = $2 AND deleted_at IS NULL",
            &[&icon, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("flag".to_string(), Value::String(flag)),
            ("iconUpdated".to_string(), Value::Bool(true)),
        ]),
    ))))
}
