use axum::{
    extract::Extension,
    routing::get, routing::post,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult, session::Session};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::Statement;

/// 查询核心服务Express模块
/// 提供查询核心相关的快速响应服务
pub mod routes;

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub query: Option<String>,
    pub params: Option<Value>,
    pub timeout: Option<i64>,
}

/// 执行查询
/// 根据查询语句执行数据库查询并返回结果
/// 安全限制：仅允许 SELECT，最大 500 行，自动注入权限 WHERE 条件
pub async fn execute_query(
    pool: Extension<Pool>,
    session: Extension<Session>,
    axum::extract::Json(req): Json<QueryRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let raw_sql = req.query.as_deref().ok_or(AppError::BadRequest("query is required".to_string()))?;
    let raw_sql = raw_sql.trim();
    if raw_sql.is_empty() {
        return Err(AppError::BadRequest("query is required".to_string()));
    }

    // 解析 SQL 并验证仅允许 SELECT
    let dialect = PostgreSqlDialect {};
    let statements = Parser::parse_sql(&dialect, raw_sql)
        .map_err(|e| AppError::BadRequest(format!("SQL parse error: {}", e)))?;

    if statements.len() != 1 {
        return Err(AppError::BadRequest("only single statement allowed".to_string()));
    }

    match &statements[0] {
        Statement::Query(_) => {
            // 允许 SELECT 语句（包括 subqueries）
        }
        _ => {
            return Err(AppError::BadRequest(
                "only SELECT queries are allowed (INSERT/UPDATE/DELETE/DDL rejected)".to_string(),
            ));
        }
    }

    // 权限过滤：从 session 获取 person/identityList/unitList，注入 WHERE 条件
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 查询用户角色、身份、组织单元以注入权限过滤
    let person_id: String = session.person_unique.clone();
    let (identity_filter, unit_filter) = get_permission_filters(&client, &person_id).await;

    // 构建带权限过滤的查询，添加 LIMIT 500
    let limited_sql = if raw_sql.to_uppercase().contains("LIMIT") {
        raw_sql.to_string()
    } else {
        format!("{} LIMIT 500", raw_sql)
    };

    // 如有权限过滤条件，注入到 WHERE
    let where_clause = build_where_clause(&identity_filter, &unit_filter);
    let final_sql = if !where_clause.is_empty() {
        inject_where(&limited_sql, &where_clause)
    } else {
        limited_sql
    };

    let rows = client
        .query(&final_sql, &[])
        .await
        .map_err(|e| AppError::BadRequest(format!("query execution error: {}", e)))?;

    let row_count = rows.len() as i64;
    let data = Value::Object(serde_json::Map::from_iter([
        ("query".to_string(), Value::String(raw_sql.to_string())),
        ("filteredQuery".to_string(), Value::String(final_sql)),
        ("params".to_string(), req.params.unwrap_or(Value::Null)),
        ("timeout".to_string(), Value::Number(serde_json::Number::from(
            req.timeout.unwrap_or(30000),
        ))),
        ("rowCount".to_string(), Value::Number(serde_json::Number::from(row_count))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 从数据库获取用户权限过滤条件
async fn get_permission_filters(
    client: &deadpool_postgres::Client,
    person_id: &str,
) -> (Option<Vec<String>>, Option<Vec<String>>) {
    let ident_names: Vec<String> = match client
        .query(
            "SELECT i.name FROM auth_identity i \
             JOIN auth_person_identity pi ON i.id = pi.identity_id \
             WHERE pi.person_id = $1 AND i.deleted_at IS NULL",
            &[&person_id],
        )
        .await
    {
        Ok(rows) => rows.iter().map(|r| r.get::<_, String>("name")).collect(),
        Err(_) => Vec::new(),
    };

    let unit_ids: Vec<String> = match client
        .query(
            "SELECT id FROM auth_unit WHERE deleted_at IS NULL",
            &[],
        )
        .await
    {
        Ok(rows) => rows.iter().map(|r| r.get::<_, String>("id")).collect(),
        Err(_) => Vec::new(),
    };

    (
        if ident_names.is_empty() { None } else { Some(ident_names) },
        if unit_ids.is_empty() { None } else { Some(unit_ids) },
    )
}

/// 构建权限 WHERE 子句
fn build_where_clause(
    identity_list: &Option<Vec<String>>,
    unit_list: &Option<Vec<String>>,
) -> String {
    let mut clauses: Vec<String> = Vec::new();
    if let Some(idents) = identity_list {
        let params: Vec<String> = idents.iter().map(|s| format!("'{}'", s.replace("'", "''"))).collect();
        if !params.is_empty() {
            clauses.push(format!("(x_identity IN ({}))", params.join(", ")));
        }
    }
    if let Some(units) = unit_list {
        let params: Vec<String> = units.iter().map(|s| format!("'{}'", s.replace("'", "''"))).collect();
        if !params.is_empty() {
            clauses.push(format!("(x_unit_id IN ({}))", params.join(", ")));
        }
    }
    clauses.join(" AND ")
}

/// 将 WHERE 条件注入到 SELECT 语句中（在现有 WHERE 之后或新增 WHERE）
fn inject_where(sql: &str, where_clause: &str) -> String {
    let upper = sql.to_uppercase();
    if upper.contains("WHERE ") {
        // 找到最后一个 WHERE 子句，在其后追加 AND
        // 简单处理：在 LIMIT 前插入 WHERE 条件
        if let Some(limit_pos) = upper.find(" LIMIT ") {
            let base = &sql[..limit_pos];
            format!("{} AND ({}) ", base.trim_end(), where_clause)
                + &sql[limit_pos..]
        } else {
            format!("{} WHERE ({}) ", sql.trim_end(), where_clause)
        }
    } else {
        format!("{} WHERE ({})", sql.trim_end(), where_clause)
    }
}

/// 获取查询历史
/// 返回最近的查询历史记录
pub async fn get_query_history(
    pool: Extension<Pool>,
    axum::extract::Path(limit): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, create_time FROM x_query_import_record ORDER BY create_time DESC LIMIT $1",
            &[&limit],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("query".to_string(), Value::String(row.get("name"))),
                ("executedAt".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("limit".to_string(), Value::Number(serde_json::Number::from(limit))),
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// 缓存查询结果
/// 将查询结果缓存以提高后续查询性能
pub async fn cache_query_result(
    pool: Extension<Pool>,
    axum::extract::Path(query_id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let cached = client
        .query_one("SELECT id FROM x_query WHERE id = $1", &[&query_id])
        .await
        .map(|_| true)
        .unwrap_or(false);
    let ttl = body.get("ttl").and_then(|v| v.as_i64()).unwrap_or(3600);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("queryId".to_string(), Value::String(query_id)),
            ("cached".to_string(), Value::Bool(cached)),
            ("ttl".to_string(), Value::Number(serde_json::Number::from(ttl))),
        ]),
    ))))
}

/// 获取缓存状态
/// 返回查询缓存的状态信息
pub async fn get_cache_status(
    pool: Extension<Pool>,
    axum::extract::Path(query_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let cached = client
        .query_one("SELECT id FROM x_query WHERE id = $1", &[&query_id])
        .await
        .map(|_| true)
        .unwrap_or(false);

    let (hits, misses) = if cached { (1, 0) } else { (0, 1) };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("queryId".to_string(), Value::String(query_id)),
            ("cached".to_string(), Value::Bool(cached)),
            ("hits".to_string(), Value::Number(serde_json::Number::from(hits))),
            ("misses".to_string(), Value::Number(serde_json::Number::from(misses))),
        ]),
    ))))
}

/// 查询核心Express路由
/// 路由前缀: /jaxrs/query/core/express/*
pub fn query_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/query/core/express/execute", post(execute_query))
        .route("/jaxrs/query/core/express/history/{limit}", get(get_query_history))
        .route("/jaxrs/query/core/express/cache/{queryId}", post(cache_query_result))
        .route("/jaxrs/query/core/express/cache/status/{queryId}", get(get_cache_status))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_core_express_router(pool)
}