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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        Value::Array(data),
        count,
        0,
    )))
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
             VALUES ($1, $2, $3, $4, $5, $6, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
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
            "UPDATE x_query_import_model SET name = COALESCE(NULLIF($1,''), name), content = COALESCE(NULLIF($2,''), content), update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE id = $3",
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
            "UPDATE x_query_neural_model SET name = COALESCE(NULLIF($1,''), name), update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE flag = $2",
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
             VALUES ($1, $2, $3, $4, $5, $6, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
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
            "UPDATE x_query_stat SET name = COALESCE(NULLIF($1,''), name), config = COALESCE(NULLIF($2,''), config), update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE id = $3",
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
             VALUES ($1, $2, $3, $4, 'draft', $5, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
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
            "UPDATE x_query_table SET name = COALESCE(NULLIF($1,''), name), update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE table_flag = $2",
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
            "INSERT INTO x_query_table_data (id, table_flag, data, create_time) VALUES ($1, $2, $3, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'))",
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
            "UPDATE x_query_table_data SET data = $1, update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') WHERE table_flag = $2 AND id = $3",
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

// ──────────────────────────────────────────────────────────────────────────────
// plan002 U2 v9 缺口闭合（designer）
//
// 约定（对齐 processplatform_assemble_designer / docs/solutions IDOR 文档）：
//   - 写端点先查记录归属（creator_person 回退 creator），经 require_owner 校验；
//     管理员放行；归属列为空降级为管理员门禁（fail-closed）。
//   - 新建 creator 取会话（session.person_unique），绝不信任请求体。
//   - 命名类写入前归一化查重（normalize_identifier：trim + 小写）。
// ──────────────────────────────────────────────────────────────────────────────

use shared::session::Session;

async fn require_admin_gate(pool: &Pool, session: &Session) -> Result<(), AppError> {
    if shared::middleware::is_admin(pool, &session.person_unique).await {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

/// 写端点统一 IDOR 门禁：
///   Ok(false) —— 记录不存在，由调用方转 ActionResult::error；
///   Err(403) —— 非属主且非管理员；
/// 归属列为空时降级为管理员门禁（fail-closed）。
async fn guard_write(
    pool: &Pool,
    session: &Session,
    client: &deadpool_postgres::Client,
    select_owner_sql: &str,
    key: &str,
) -> Result<bool, AppError> {
    let row = client
        .query_opt(select_owner_sql, &[&key])
        .await
        .map_err(|_| AppError::Internal)?;
    match row {
        None => Ok(false),
        Some(row) => {
            let owner: String = row.get::<_, Option<String>>("owner").unwrap_or_default();
            if owner.is_empty() {
                require_admin_gate(pool, session).await?;
            } else {
                shared::middleware::require_owner(pool, session, &owner).await?;
            }
            Ok(true)
        }
    }
}

fn body_str<'a>(body: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|k| body.get(*k).and_then(|v| v.as_str()))
}

/// POST /designer/search —— 按关键词检索查询设计（真实 ILIKE，拒绝空 key）。
pub async fn designer_search_v2(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let key = body
        .get("key")
        .or_else(|| body.get("query"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if key.trim().is_empty() {
        return Ok(Json(ActionResult::error("key is required")));
    }
    let pattern = format!("%{}%", key.trim());

    let rows = client
        .query(
            "SELECT id, name, category, creator, create_time FROM x_query_design \
             WHERE (name ILIKE $1 OR category ILIKE $1) AND deleted_at IS NULL \
             ORDER BY update_time DESC NULLS LAST LIMIT 20",
            &[&pattern],
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
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

/// GET /id/{count} —— 生成唯一标识列表（0 < count < 200，对齐 Java ActionGet）。
pub async fn id_generate(
    Path(count): Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let n = count.clamp(0, 199);
    let ids: Vec<Value> = (0..n)
        .map(|_| Value::String(uuid::Uuid::new_v4().to_string()))
        .collect();
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(ids.len() as i64))),
            ("data".to_string(), Value::Array(ids)),
        ]),
    ))))
}

/// GET /importmodel/{flag} —— 按 id 或 model_flag 获取导入模型。
pub async fn importmodel_get_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, model_flag, query_flag, content, creator, create_time \
             FROM x_query_import_model WHERE id = $1 OR model_flag = $1 LIMIT 1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("modelFlag".to_string(), Value::String(row.get("model_flag"))),
                (
                    "queryFlag".to_string(),
                    Value::String(row.get::<_, Option<String>>("query_flag").unwrap_or_default()),
                ),
                (
                    "content".to_string(),
                    Value::String(row.get::<_, Option<String>>("content").unwrap_or_default()),
                ),
                ("creator".to_string(), Value::String(row.get("creator"))),
                ("createTime".to_string(), Value::String(row.get("create_time"))),
            ]),
        )))),
        None => Ok(Json(ActionResult::error("import model not found"))),
    }
}

/// PUT /importmodel/{flag} —— 更新导入模型（IDOR 门禁 + 归一化查重排除自身）。
pub async fn importmodel_edit_flag(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_import_model \
         WHERE id = $1 OR model_flag = $1 LIMIT 1",
        &flag,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("import model not found")));
    }

    let name = body_str(&body, &["name"]).unwrap_or_default();
    if !name.trim().is_empty() {
        let norm = normalize_identifier(name);
        let dup_row = client
            .query_one(
                "SELECT COUNT(*) AS cnt FROM x_query_import_model \
                 WHERE LOWER(TRIM(COALESCE(name,''))) = $1 AND id <> $2",
                &[&norm, &flag],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if dup_row.get::<_, i64>("cnt") > 0 {
            return Ok(Json(ActionResult::error("import model name already exists")));
        }
    }
    let content = body_str(&body, &["data", "content"]).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_import_model \
             SET name = COALESCE(NULLIF($1,''), name), content = COALESCE(NULLIF($2,''), content), \
                 update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE id = $3 OR model_flag = $3",
            &[&name, &content, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("import model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// DELETE /importmodel/{flag} —— 删除导入模型（IDOR 门禁）。
pub async fn importmodel_delete_flag(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_import_model \
         WHERE id = $1 OR model_flag = $1 LIMIT 1",
        &flag,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("import model not found")));
    }

    let result = client
        .execute(
            "DELETE FROM x_query_import_model WHERE id = $1 OR model_flag = $1",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("import model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(flag))]),
    ))))
}

/// POST /importmodel/{flag}/permission —— 设置权限（IDOR 门禁，权限体序列化落库）。
pub async fn importmodel_permission_set(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_import_model \
         WHERE id = $1 OR model_flag = $1 LIMIT 1",
        &flag,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("import model not found")));
    }

    let permission = serde_json::to_string(
        body.get("permissionList").unwrap_or(&Value::Array(Vec::new())),
    )
    .map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_import_model SET permission = $1, update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE id = $2 OR model_flag = $2",
            &[&permission, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("import model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag)),
            ("permissionSet".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// PUT /output/{flag}/select —— 设置输出选择（IDOR 门禁，select_file/query_flag 真实落库）。
pub async fn output_select_put(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_output \
         WHERE flag = $1 LIMIT 1",
        &flag,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("output not found")));
    }

    let select_file = body_str(&body, &["selectFile", "file"]).unwrap_or_default();
    let query_flag = body_str(&body, &["queryFlag", "query"]).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_output \
             SET select_file = COALESCE(NULLIF($1,''), select_file), \
                 query_flag = COALESCE(NULLIF($2,''), query_flag), \
                 update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE flag = $3",
            &[&select_file, &query_flag, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("output not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("flag".to_string(), Value::String(flag)),
            ("selected".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// POST /query —— 创建查询设计（归一化查重，creator 取会话）。
pub async fn query_create_v2(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let name = body_str(&body, &["name"]).unwrap_or_default();
    if name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let norm = normalize_identifier(name);
    let dup_row = client
        .query_one(
            "SELECT COUNT(*) AS cnt FROM x_query_design \
             WHERE LOWER(TRIM(COALESCE(name,''))) = $1 AND deleted_at IS NULL",
            &[&norm],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if dup_row.get::<_, i64>("cnt") > 0 {
        return Ok(Json(ActionResult::error("query name already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let flag = uuid::Uuid::new_v4().to_string();
    let category = body_str(&body, &["category"]).unwrap_or_default();
    let query_definition = body_str(&body, &["data", "query"]).unwrap_or_default();

    client
        .execute(
            "INSERT INTO x_query_design (id, flag, name, category, query_definition, creator, create_time, update_time) \
             VALUES ($1, $2, $3, $4, $5, $6, to_char(NOW(),'YYYY-MM-DD HH24:MI:SS'), NOW())",
            &[&id, &flag, &name, &category, &query_definition, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("flag".to_string(), Value::String(flag)),
        ]),
    ))))
}

/// DELETE /query/{flag} —— 删除查询设计（软删 + IDOR 门禁）。
pub async fn query_delete_flag(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_design \
         WHERE flag = $1 OR id = $1 LIMIT 1",
        &flag,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("query not found")));
    }

    let result = client
        .execute(
            "UPDATE x_query_design SET deleted_at = NOW() WHERE (flag = $1 OR id = $1) AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(flag))]),
    ))))
}

/// PUT /query/{flag} —— 更新查询设计（IDOR 门禁 + 归一化查重排除自身）。
pub async fn query_edit_flag(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_design \
         WHERE flag = $1 OR id = $1 LIMIT 1",
        &flag,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("query not found")));
    }

    let name = body_str(&body, &["name"]).unwrap_or_default();
    if !name.trim().is_empty() {
        let norm = normalize_identifier(name);
        let dup_row = client
            .query_one(
                "SELECT COUNT(*) AS cnt FROM x_query_design \
                 WHERE LOWER(TRIM(COALESCE(name,''))) = $1 AND flag <> $2 AND deleted_at IS NULL",
                &[&norm, &flag],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if dup_row.get::<_, i64>("cnt") > 0 {
            return Ok(Json(ActionResult::error("query name already exists")));
        }
    }
    let category = body_str(&body, &["category"]).unwrap_or_default();
    let query_definition = body_str(&body, &["data", "query"]).unwrap_or_default();

    let result = client
        .execute(
            "UPDATE x_query_design \
             SET name = COALESCE(NULLIF($1,''), name), \
                 category = COALESCE(NULLIF($2,''), category), \
                 query_definition = COALESCE(NULLIF($3,''), query_definition), \
                 update_time = NOW() \
             WHERE (flag = $4 OR id = $4) AND deleted_at IS NULL",
            &[&name, &category, &query_definition, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("query not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag)),
            ("updated".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// POST /query/{flag}/icon —— 设置图标（IDOR 门禁，与 PUT icon 同一落库语义）。
pub async fn query_icon_set(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_design \
         WHERE flag = $1 OR id = $1 LIMIT 1",
        &flag,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("query not found")));
    }

    let icon = body_str(&body, &["icon"]).unwrap_or("");
    let result = client
        .execute(
            "UPDATE x_query_design SET icon = $1, update_time = NOW() \
             WHERE (flag = $2 OR id = $2) AND deleted_at IS NULL",
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

/// POST /{resource}/{flag}/permission 族共用实现：权限体序列化后写 permission 列。
async fn set_permission_generic(
    pool: &Pool,
    session: &Session,
    resource: PermissionResource,
    flag: &str,
    body: &Value,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let sql = format!(
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM {} WHERE {} LIMIT 1",
        resource.table, resource.predicate
    );
    let found = guard_write(pool, session, &client, &sql, flag).await?;
    if !found {
        return Ok(Json(ActionResult::error(format!("{} not found", resource.label))));
    }

    let permission = serde_json::to_string(
        body.get("permissionList").unwrap_or(&Value::Array(Vec::new())),
    )
    .map_err(|_| AppError::Internal)?;

    let update_sql = format!(
        "UPDATE {} SET permission = $1, update_time = {} WHERE {}",
        resource.table, resource.time_expr, resource.predicate
    );
    let result = client
        .execute(update_sql.as_str(), &[&permission, &flag])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error(format!("{} not found", resource.label))));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(flag.to_string())),
            ("permissionSet".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 权限写入口的目标资源描述（表/谓词/时间表达式均为编译期常量，无注入面）。
/// update_time 列类型随表而异（TEXT/TIMESTAMP），时间表达式必须匹配列类型。
struct PermissionResource {
    table: &'static str,
    predicate: &'static str,
    label: &'static str,
    time_expr: &'static str,
}

const TS_TEXT: &str = "to_char(NOW(),'YYYY-MM-DD HH24:MI:SS')";
const TS_TIMESTAMP: &str = "NOW()";

const PERM_QUERY: PermissionResource = PermissionResource {
    table: "x_query_design",
    predicate: "(flag = $1 OR id = $1)",
    label: "query",
    time_expr: TS_TIMESTAMP,
};
const PERM_STAT: PermissionResource = PermissionResource {
    table: "x_query_stat",
    predicate: "id = $1",
    label: "stat",
    time_expr: TS_TEXT,
};
const PERM_TABLE: PermissionResource = PermissionResource {
    table: "x_query_table",
    predicate: "(table_flag = $1 OR id = $1)",
    label: "table",
    time_expr: TS_TEXT,
};
const PERM_VIEW: PermissionResource = PermissionResource {
    table: "x_query_view",
    predicate: "(id = $1 OR view_flag = $1)",
    label: "view",
    time_expr: TS_TIMESTAMP,
};

/// POST /query/{flag}/permission
pub async fn query_permission_set(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    set_permission_generic(&pool, &session, PERM_QUERY, &flag, &body).await
}

/// POST /stat/{id}/permission
pub async fn stat_permission_set(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    set_permission_generic(&pool, &session, PERM_STAT, &id, &body).await
}

/// POST /table/{flag}/permission
pub async fn table_permission_set(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    set_permission_generic(&pool, &session, PERM_TABLE, &flag, &body).await
}

/// POST /view/{id}/permission
pub async fn view_permission_set(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    set_permission_generic(&pool, &session, PERM_VIEW, &id, &body).await
}

/// GET /table/{flag}/build/dispatch —— 按 table_flag 触发构建（真实状态落库）。
pub async fn table_build_dispatch_flag(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_table SET status = 'build', reloaded = FALSE, update_time = to_char(NOW(),'YYYY-MM-DD HH24:MI:SS') \
             WHERE table_flag = $1 AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("tableFlag".to_string(), Value::String(flag)),
            ("dispatched".to_string(), Value::Number(serde_json::Number::from(result as i64))),
        ]),
    ))))
}

/// stat/view simulate 共用执行核心：config/content JSON 中提取 sql，
/// sqlparser 校验后真实执行返回聚合行；无可执行定义时如实返回 calculated=false。
async fn execute_configured_sql(
    client: &deadpool_postgres::Client,
    config_raw: &str,
) -> Result<(bool, Value), AppError> {
    let config: Value = serde_json::from_str(config_raw).unwrap_or(Value::Null);
    let sql_opt = config
        .get("sql")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    match sql_opt {
        Some(sql) if !sql.trim().is_empty() => {
            validate_single_select(&sql).map_err(AppError::BadRequest)?;
            let trimmed = sql.trim().trim_end_matches(';');
            let limited = ensure_limit(trimmed, 500);
            let rows = client
                .query(&limited, &[])
                .await
                .map_err(|_| AppError::Internal)?;
            let data: Vec<Value> = rows.iter().map(row_to_json).collect();
            let mut payload = serde_json::Map::new();
            payload.insert("calculated".to_string(), Value::Bool(true));
            payload.insert(
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            );
            payload.insert("calculateGrid".to_string(), Value::Array(data));
            Ok((true, Value::Object(payload)))
        }
        _ => {
            tracing::warn!("simulate requested but no executable config.sql; returning metadata only");
            Ok((
                false,
                serde_json::json!({ "calculated": false, "config": config }),
            ))
        }
    }
}

/// PUT /stat/{id}/simulate —— 统计模拟（IDOR 门禁 + config.sql 真实执行）。
pub async fn stat_simulate_put(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(_body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_stat WHERE id = $1",
        &id,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("stat not found")));
    }

    let row = client
        .query_opt(
            "SELECT id, name, config FROM x_query_stat WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ActionResult::error("stat not found"))),
    };

    let config_raw: String = row.get::<_, Option<String>>("config").unwrap_or_default();
    let (calculated, mut payload_map) = execute_configured_sql(&client, &config_raw).await?;

    if let Some(payload) = payload_map.as_object_mut() {
        payload.insert("id".to_string(), Value::String(row.get("id")));
        payload.insert(
            "name".to_string(),
            Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
        );
        payload.insert("calculated".to_string(), Value::Bool(calculated));
    }

    Ok(Json(ActionResult::success(payload_map)))
}

/// PUT /view/{id}/bundle —— 保存视图 bundle（IDOR 门禁）。
pub async fn view_bundle_put(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_view WHERE id = $1",
        &id,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("view not found")));
    }

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
            ("bundleSaved".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// PUT /view/{id}/simulate —— 视图模拟：content JSON 提取 sql 校验后真实执行。
pub async fn view_simulate_put(
    pool: Extension<Pool>,
    session: Extension<Session>,
    Path(id): Path<String>,
    Json(_body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let found = guard_write(
        &pool,
        &session,
        &client,
        "SELECT COALESCE(creator_person, creator, '') AS owner FROM x_query_view WHERE id = $1",
        &id,
    )
    .await?;
    if !found {
        return Ok(Json(ActionResult::error("view not found")));
    }

    let row = client
        .query_opt(
            "SELECT id, view_flag, content FROM x_query_view WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ActionResult::error("view not found")))
    };

    let content_raw: String = row.get::<_, Option<String>>("content").unwrap_or_default();
    let (calculated, mut payload_map) = execute_configured_sql(&client, &content_raw).await?;

    if let Some(payload) = payload_map.as_object_mut() {
        payload.insert("id".to_string(), Value::String(row.get("id")));
        payload.insert(
            "viewFlag".to_string(),
            Value::String(row.get::<_, Option<String>>("view_flag").unwrap_or_default()),
        );
        payload.insert("calculated".to_string(), Value::Bool(calculated));
    }

    Ok(Json(ActionResult::success(payload_map)))
}
