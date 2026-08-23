//! plan002 U2 缺口闭合：surface 侧 statement/stat/search/morelikethis/table/view 补齐
//!
//! statement 执行与 designer 同口径：sqlparser 校验（仅单条 SELECT）→
//! `:param` 参数化 → 分页包裹执行；stat execute 按 config.sql 真实执行聚合。

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
    ]))
}

/// 简单 SQL 格式化：主要关键字前换行，供 statement format 端点输出。
pub fn format_sql(sql: &str) -> String {
    let keywords = [
        "FROM ", "WHERE ", "GROUP BY ", "ORDER BY ", "HAVING ", "LEFT JOIN ", "RIGHT JOIN ",
        "INNER JOIN ", "OUTER JOIN ", "JOIN ",
    ];
    let mut formatted = sql.trim().to_string();
    for kw in keywords {
        let lower = formatted.to_lowercase();
        let needle = kw.to_lowercase();
        let mut result = String::new();
        let mut rest = lower.clone();
        let mut src_offset = 0;
        while let Some(pos) = rest.find(&needle) {
            let absolute = src_offset + pos;
            if absolute > 0 {
                result.push_str(&formatted[src_offset..absolute]);
                result.push('\n');
                src_offset = absolute;
            }
            rest = lower[absolute + needle.len()..].to_string();
            src_offset += needle.len();
            result.push_str(&formatted[src_offset - needle.len()..src_offset]);
        }
        if !result.is_empty() {
            result.push_str(&formatted[src_offset..]);
            formatted = result;
        }
    }
    formatted
}

// ── statement 族 ─────────────────────────────────────────────────────────────

pub async fn statement_get_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, alias, query_flag, entity_class, type, data FROM x_query_statement WHERE id = $1 LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(statement_row_json(&row)))),
        None => Ok(Json(ActionResult::error("statement not found"))),
    }
}

pub async fn statement_get_format(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, type, data FROM x_query_statement WHERE id = $1 LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let raw: String = row.get::<_, Option<String>>("data").unwrap_or_default();
            Ok(Json(ActionResult::success(Value::Object(
                serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    (
                        "name".to_string(),
                        Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                    ),
                    (
                        "type".to_string(),
                        Value::String(row.get::<_, Option<String>>("type").unwrap_or_default()),
                    ),
                    ("format".to_string(), Value::String(format_sql(&raw))),
                ]),
            ))))
        }
        None => Ok(Json(ActionResult::error("statement not found"))),
    }
}

pub async fn statement_list_with_query(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, alias, query_flag, entity_class, type, data FROM x_query_statement WHERE query_flag = $1 ORDER BY create_time DESC",
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

pub async fn statement_execute(
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

// ── stat 族 ──────────────────────────────────────────────────────────────────

fn stat_row_json(row: &deadpool_postgres::tokio_postgres::Row) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        (
            "name".to_string(),
            Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
        ),
        (
            "queryFlag".to_string(),
            Value::String(row.get::<_, Option<String>>("query_flag").unwrap_or_default()),
        ),
        (
            "statType".to_string(),
            Value::String(row.get::<_, Option<String>>("stat_type").unwrap_or_default()),
        ),
        (
            "config".to_string(),
            Value::String(row.get::<_, Option<String>>("config").unwrap_or_default()),
        ),
        (
            "creator".to_string(),
            Value::String(row.get::<_, Option<String>>("creator").unwrap_or_default()),
        ),
    ]))
}

/// stat execute 真实落地：config.sql 存在则校验后真实执行返回聚合行；
/// 否则返回 config 元数据与 executed 标记（无计算定义可执行）。
async fn execute_stat_by_id(client: &deadpool_postgres::Client, id: &str) -> Result<Value, AppError> {
    let row = client
        .query_opt(
            "SELECT id, name, stat_type, config FROM x_query_stat WHERE id = $1 LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row = match row {
        Some(r) => r,
        None => return Err(AppError::NotFound),
    };

    let config_raw: String = row.get::<_, Option<String>>("config").unwrap_or_default();
    let config: Value = serde_json::from_str(&config_raw).unwrap_or(Value::Null);
    let sql_opt = config.get("sql").and_then(|v| v.as_str()).map(|s| s.to_string());

    let mut payload = serde_json::Map::new();
    payload.insert("id".to_string(), Value::String(row.get("id")));
    payload.insert(
        "name".to_string(),
        Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
    );

    match sql_opt {
        Some(sql) if !sql.trim().is_empty() => {
            validate_single_select(&sql).map_err(|e| {
                tracing::warn!("stat '{}' execute rejected: {}", id, e);
                AppError::BadRequest(e)
            })?;
            let trimmed_sql = sql.trim().trim_end_matches(';').to_string();
            let rows = client
                .query(&trimmed_sql, &[])
                .await
                .map_err(|_| AppError::Internal)?;
            let data: Vec<Value> = rows.iter().map(row_to_json).collect();
            payload.insert("calculated".to_string(), Value::Bool(true));
            payload.insert(
                "count".to_string(),
                Value::Number(serde_json::Number::from(data.len() as i64)),
            );
            payload.insert("data".to_string(), Value::Array(data));
        }
        _ => {
            tracing::warn!("stat '{}' has no executable config.sql; returning metadata only", id);
            payload.insert("calculated".to_string(), Value::Bool(false));
            payload.insert("config".to_string(), config);
        }
    }

    Ok(Value::Object(payload))
}

pub async fn stat_get_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, query_flag, stat_type, config, creator FROM x_query_stat WHERE id = $1 LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(stat_row_json(&row)))),
        None => Ok(Json(ActionResult::error("stat not found"))),
    }
}

pub async fn stat_get_with_query(
    pool: Extension<Pool>,
    Path((flag, query_flag)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, name, query_flag, stat_type, config, creator FROM x_query_stat WHERE (name = $1 OR id = $1) AND query_flag = $2 LIMIT 1",
            &[&flag, &query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(stat_row_json(&row)))),
        None => Ok(Json(ActionResult::error("stat not found"))),
    }
}

pub async fn stat_list_with_query(
    pool: Extension<Pool>,
    Path(query_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, query_flag, stat_type, config, creator FROM x_query_stat WHERE query_flag = $1 ORDER BY create_time DESC",
            &[&query_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(stat_row_json).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub async fn stat_execute(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let result = execute_stat_by_id(&client, &id).await?;
    Ok(Json(ActionResult::success(result)))
}

// ── search / morelikethis ────────────────────────────────────────────────────

/// 全局搜索：按关键词匹配查询设计与视图名称（真实 ILIKE 查询）。
pub async fn search_post(
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

    let designs = client
        .query(
            "SELECT id, name, category FROM x_query_design WHERE name ILIKE $1 AND deleted_at IS NULL ORDER BY update_time DESC LIMIT 20",
            &[&pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let views = client
        .query(
            "SELECT id, name, view_flag, query_flag FROM x_query_view WHERE name ILIKE $1 AND deleted_at IS NULL ORDER BY create_time DESC LIMIT 20",
            &[&pattern],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let design_list: Vec<Value> = designs
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
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

    let view_list: Vec<Value> = views
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                (
                    "name".to_string(),
                    Value::String(row.get::<_, Option<String>>("name").unwrap_or_default()),
                ),
                (
                    "viewFlag".to_string(),
                    Value::String(row.get::<_, Option<String>>("view_flag").unwrap_or_default()),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("key".to_string(), Value::String(key.to_string())),
            ("designList".to_string(), Value::Array(design_list)),
            ("viewList".to_string(), Value::Array(view_list)),
        ]),
    ))))
}

/// 相似查找：在动态表数据中按关键词检索相似条目（真实 ILIKE 查询）。
pub async fn morelikethis_post(
    pool: Extension<Pool>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let key = body
        .get("key")
        .or_else(|| body.get("term"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if key.trim().is_empty() {
        return Ok(Json(ActionResult::error("key is required")));
    }
    let pattern = format!("%{}%", key.trim());
    let table_flag = body
        .get("tableFlag")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    let rows = if table_flag.is_empty() {
        client
            .query(
                "SELECT id, table_flag, data FROM x_query_table_data WHERE data ILIKE $1 ORDER BY create_time DESC LIMIT 20",
                &[&pattern],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, table_flag, data FROM x_query_table_data WHERE table_flag = $1 AND data ILIKE $2 ORDER BY create_time DESC LIMIT 20",
                &[&table_flag, &pattern],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    let data: Vec<Value> = rows.iter().map(row_to_json).collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("list".to_string(), Value::Array(data)),
        ]),
    ))))
}

// ── table 行级操作补齐 ───────────────────────────────────────────────────

pub async fn table_row_delete(
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

pub async fn table_row_insert(
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

pub async fn table_row_insert_one(
    pool: Extension<Pool>,
    Path(table_flag): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let data_str = serde_json::to_string(&body).map_err(|_| AppError::Internal)?;

    client
        .execute(
            "INSERT INTO x_query_table_data (id, table_flag, data, create_time) VALUES ($1, $2, $3, NOW())",
            &[&id, &table_flag, &data_str],
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

// ── importmodel 记录管理补齐 ─────────────────────────────────────────────

/// 删除导入记录（Java ActionDeleteRecord）
pub async fn importmodel_record_delete(
    pool: Extension<Pool>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "DELETE FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("record not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(record_id)),
            ("deleted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// 重新执行导入失败记录：重置记录状态并新建一条执行记录（真实写库）。
pub async fn importmodel_reexecute_record(
    pool: Extension<Pool>,
    Path(record_id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, model_flag, import_model_id FROM x_query_import_model_record WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ActionResult::error("record not found"))),
    };

    let model_flag: String = row.get::<_, Option<String>>("model_flag").unwrap_or_default();
    let import_model_id: String = row.get::<_, Option<String>>("import_model_id").unwrap_or_default();

    let new_record_id = uuid::Uuid::new_v4().to_string();
    client
        .execute(
            "INSERT INTO x_query_import_model_record (id, model_flag, import_model_id, status, create_time) \
             VALUES ($1, $2, $3, 'running', NOW())",
            &[&new_record_id, &model_flag, &import_model_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    client
        .execute(
            "UPDATE x_query_import_model_record SET status = 'running', update_time = NOW() WHERE id = $1",
            &[&record_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("recordId".to_string(), Value::String(record_id)),
            ("newRecordId".to_string(), Value::String(new_record_id)),
            ("reexecuted".to_string(), Value::Bool(true)),
        ]),
    ))))
}

/// bundle/v2：读取视图 bundle_data_v2 JSON 数组并按 body.page/size 分页切片。
pub async fn view_bundle_v2_post(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, view_flag, bundle_data_v2 FROM x_query_view WHERE id = $1 LIMIT 1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row = match row {
        Some(r) => r,
        None => return Ok(Json(ActionResult::error("view not found"))),
    };

    let bundle_raw: String = row
        .get::<_, Option<String>>("bundle_data_v2")
        .unwrap_or_default();
    let items: Vec<Value> = serde_json::from_str(&bundle_raw).unwrap_or_default();

    let page = body.get("page").and_then(|v| v.as_i64()).unwrap_or(1).max(1);
    let size = body
        .get("size")
        .and_then(|v| v.as_i64())
        .unwrap_or(20)
        .clamp(1, 500);
    let offset = (page - 1) * size;
    let total = items.len() as i64;

    let slice: Vec<Value> = items
        .into_iter()
        .skip(offset as usize)
        .take(size as usize)
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(row.get("id"))),
            (
                "viewFlag".to_string(),
                Value::String(row.get::<_, Option<String>>("view_flag").unwrap_or_default()),
            ),
            ("total".to_string(), Value::Number(serde_json::Number::from(total))),
            ("page".to_string(), Value::Number(serde_json::Number::from(page))),
            ("size".to_string(), Value::Number(serde_json::Number::from(size))),
            ("list".to_string(), Value::Array(slice)),
        ]),
    ))))
}
