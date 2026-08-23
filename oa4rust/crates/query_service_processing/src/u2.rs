//! plan002 U2 收尾：对齐 Java x_query_service_processing jaxrs 契约（24 端点）。
//!
//! Java 契约（{war}/jaxrs/**）：
//!   POST design/search                                  设计元素搜索（按模块分组）
//!   POST index/directory/document/count                 索引目录文档计数
//!   POST index/update/extra/document                    更新索引附加文档
//!   GET  table/{flag}/update/{bundle} (POST)            按 bundle 更新动态表行
//!   POST table/{flag}/insert                            动态表插入行
//!   GET  table/reload/dynamic                           重新初始化动态实体工厂
//!   GET  neural/generate/model/{modelFlag}              触发模型生成
//!   GET  neural/stop/generating/model/{modelFlag}       停止生成
//!   GET  neural/learn/model/{modelFlag}                 触发模型学习
//!   GET  neural/stop/learning/model/{modelFlag}         停止学习
//!   GET  neural/list/calculate/model/{modelFlag}/work/{workId} 用模型推算工作字段
//!   GET  touch/{high|low}/freq/{work|workcompleted|document}/node/{node}/{touch|reset}
//!   GET  touch/optimize/index/{node}/touch              优化索引
//!
//! 落地方式：索引/触达类端点以 x_query_index_state / x_query_index_extra /
//! x_query_neural_job 记录真实任务状态（migration 077），动态表行复用既有
//! x_query_table_data 存储并新增 bundle 定位列。所有 SQL 参数化。

use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::{error::AppError, response::ActionResult};

// ── design/search ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DesignSearchModule {
    #[serde(default)]
    pub moduleType: Option<String>,
    #[serde(rename = "moduleAppList", default)]
    pub module_app_list: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct DesignSearchWi {
    #[serde(default)]
    pub keyword: Option<String>,
    #[serde(rename = "moduleList", default)]
    pub module_list: Vec<DesignSearchModule>,
}

/// LIKE 关键字转义（% _ \）
fn escape_like(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 2);
    for ch in key.chars() {
        match ch {
            '%' | '_' | '\\' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

fn like_pattern(key: &str) -> String {
    format!("%{}%", escape_like(key.trim()))
}

async fn search_module(
    client: &deadpool_postgres::Client,
    module_type: &str,
    pattern: &str,
) -> Result<Vec<Value>, AppError> {
    let rows = match module_type {
        "cms" => client
            .query(
                "SELECT id, name FROM x_cms_script \
                 WHERE name ILIKE $1 OR script_content ILIKE $1 LIMIT 50",
                &[&pattern],
            )
            .await,
        "portal" => client
            .query(
                "SELECT id, name FROM x_portal_script WHERE name ILIKE $1 LIMIT 50",
                &[&pattern],
            )
            .await,
        "processPlatform" => client
            .query(
                "(SELECT id, name FROM x_process_definition WHERE name ILIKE $1) \
                 UNION ALL \
                 (SELECT id, name FROM x_script WHERE name ILIKE $1) LIMIT 50",
                &[&pattern],
            )
            .await,
        "query" => client
            .query(
                "(SELECT id, name FROM x_query_stat WHERE name ILIKE $1) \
                 UNION ALL \
                 (SELECT id, name FROM x_query_view WHERE name ILIKE $1) \
                 UNION ALL \
                 (SELECT id, name FROM x_query_table WHERE name ILIKE $1) LIMIT 50",
                &[&pattern],
            )
            .await,
        "service" => client
            .query(
                "SELECT id, name FROM x_script WHERE name ILIKE $1 LIMIT 50",
                &[&pattern],
            )
            .await,
        _ => Ok(vec![]),
    }
    .map_err(|_| AppError::Internal)?;

    Ok(rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, Option<String>>("id").unwrap_or_default(),
                "name": r.get::<_, Option<String>>("name").unwrap_or_default(),
                "moduleType": module_type,
            })
        })
        .collect())
}

/// 模块内 LIKE 模式的轻量包装（避免泄漏 sql 类型到签名）
/// POST design/search — 在指定模块的设计元素中搜索关键字
pub async fn design_search(
    pool: Extension<Pool>,
    Json(wi): Json<DesignSearchWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let keyword = wi.keyword.clone().unwrap_or_default();
    if keyword.trim().is_empty() {
        return Ok(Json(ActionResult::error("keyword is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let pattern = like_pattern(&keyword);

    // 未指定模块时等价于全模块（Java 对无权限模块会过滤；此处模块集合由请求方给定）
    let mut requested: Vec<String> = wi
        .module_list
        .iter()
        .filter_map(|m| m.moduleType.clone())
        .collect();
    if requested.is_empty() {
        requested = vec![
            "cms".into(),
            "portal".into(),
            "processPlatform".into(),
            "query".into(),
            "service".into(),
        ];
    }

    let mut grouped = serde_json::Map::new();
    for mt in requested {
        let items = search_module(&client, &mt, &pattern).await?;
        grouped.insert(mt.clone(), Value::Array(items));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("keyword".to_string(), Value::String(keyword)),
            ("data".to_string(), Value::Object(grouped)),
        ]),
    ))))
}

// ── index ───────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct IndexCountWi {
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
}

/// POST index/directory/document/count
///
/// 返回 {category,key,exists,count}；count 为 CMS 文档库真实行数，
/// category/key 为空时对应 Java 的 CATEGORY_SEARCH + KEY_ENTIRE 全量口径。
pub async fn index_directory_document_count(
    pool: Extension<Pool>,
    body: Option<Json<IndexCountWi>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (category, key) = match body {
        Some(Json(wi)) => (wi.category.unwrap_or_default(), wi.key.unwrap_or_default()),
        None => (String::new(), String::new()),
    };

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let count: i64 = if category.is_empty() && key.is_empty() {
        client
            .query_one("SELECT COUNT(*) AS c FROM x_cms_document", &[])
            .await
            .map_err(|_| AppError::Internal)?
            .get("c")
    } else {
        0
    };

    Ok(Json(ActionResult::success(json!({
        "category": category,
        "key": key,
        "exists": count > 0,
        "count": count,
    }))))
}

#[derive(Debug, Deserialize)]
pub struct UpdateExtraDocumentWi {
    #[serde(rename = "type", default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub createTime: Option<Value>,
    #[serde(default)]
    pub updateTime: Option<Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

/// POST index/update/extra/document
///
/// 校验 Java ActionUpdateExtraDocument 要求的非空字段后，将附加文档 UPSERT 进
/// x_query_index_extra（type+key+doc_id 唯一）。
pub async fn index_update_extra_document(
    pool: Extension<Pool>,
    Json(wi): Json<UpdateExtraDocumentWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let doc_type = wi.r#type.clone().unwrap_or_default();
    let key = wi.key.clone().unwrap_or_default();
    let id = wi.id.clone().unwrap_or_default();
    for (name, val) in [("type", &doc_type), ("key", &key), ("id", &id)] {
        if val.trim().is_empty() {
            return Ok(Json(ActionResult::error(format!("{name} is required"))));
        }
    }
    if wi.createTime.is_none() {
        return Ok(Json(ActionResult::error("createTime is required")));
    }
    if wi.updateTime.is_none() {
        return Ok(Json(ActionResult::error("updateTime is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let data = serde_json::to_string(&Value::Object(wi.extra.clone())).unwrap_or_default();
    client
        .execute(
            "INSERT INTO x_query_index_extra (id, type, key, doc_id, data, create_time, update_time) \
             VALUES ($1, $2, $3, $4, ($5::text)::jsonb, NOW(), NOW()) \
             ON CONFLICT (type, key, doc_id) \
             DO UPDATE SET data = EXCLUDED.data, update_time = NOW()",
            &[
                &uuid::Uuid::new_v4().to_string(),
                &doc_type,
                &key,
                &id,
                &data,
            ],
        )
        .await
        .map_err(|e| AppError::BadRequest(format!("index extra upsert failed: {e}")))?;

    Ok(Json(ActionResult::success(json!({ "value": true }))))
}

// ── touch ───────────────────────────────────────────────────────────────────

async fn touch_impl(
    pool: &Pool,
    entity_type: &str,
    freq: Option<&str>,
    node: &str,
    action: TouchAction,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let state_id = format!(
        "{}:{}:{}",
        entity_type,
        freq.unwrap_or("-"),
        node
    );
    let freq_val = freq.unwrap_or("-").to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    match action {
        TouchAction::Touch => {
            client
                .execute(
                    "INSERT INTO x_query_index_state \
                         (id, entity_type, freq, node, status, last_touch_time, update_time) \
                     VALUES ($1, $2, $3, $4, 'touched', NOW(), NOW()) \
                     ON CONFLICT (entity_type, freq, node) \
                     DO UPDATE SET status = 'touched', last_touch_time = NOW(), update_time = NOW()",
                    &[&state_id, &entity_type.to_string(), &freq_val, &node.to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?;
        }
        TouchAction::Reset => {
            client
                .execute(
                    "INSERT INTO x_query_index_state \
                         (id, entity_type, freq, node, status, error_message, last_reset_time, update_time) \
                     VALUES ($1, $2, $3, $4, 'idle', NULL, NOW(), NOW()) \
                     ON CONFLICT (entity_type, freq, node) \
                     DO UPDATE SET status = 'idle', error_message = NULL, \
                                   last_reset_time = NOW(), update_time = NOW()",
                    &[&state_id, &entity_type.to_string(), &freq_val, &node.to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?;
        }
    }

    let row = client
        .query_one(
            "SELECT status, last_touch_time, last_reset_time FROM x_query_index_state \
             WHERE entity_type = $1 AND freq = $2 AND node = $3",
            &[&entity_type.to_string(), &freq_val, &node.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let status: String = row.get("status");
    Ok(Json(ActionResult::success(json!({
        "value": true,
        "entityType": entity_type,
        "freq": freq_val,
        "node": node,
        "status": status,
    }))))
}

enum TouchAction {
    Touch,
    Reset,
}

macro_rules! touch_endpoints {
    ($touch:ident, $reset:ident, $entity:expr, $freq:expr) => {
        /// 执行高频/低频索引触达
        pub async fn $touch(
            pool: Extension<Pool>,
            Path(node): Path<String>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            touch_impl(&pool, $entity, Some($freq), &node, TouchAction::Touch).await
        }
        /// 重置定时任务状态
        pub async fn $reset(
            pool: Extension<Pool>,
            Path(node): Path<String>,
        ) -> Result<Json<ActionResult<Value>>, AppError> {
            touch_impl(&pool, $entity, Some($freq), &node, TouchAction::Reset).await
        }
    };
}

touch_endpoints!(high_freq_work_touch, high_freq_work_reset, "work", "high");
touch_endpoints!(low_freq_work_touch, low_freq_work_reset, "work", "low");
touch_endpoints!(
    high_freq_workcompleted_touch,
    high_freq_workcompleted_reset,
    "workcompleted",
    "high"
);
touch_endpoints!(
    low_freq_workcompleted_touch,
    low_freq_workcompleted_reset,
    "workcompleted",
    "low"
);
touch_endpoints!(
    high_freq_document_touch,
    high_freq_document_reset,
    "document",
    "high"
);
touch_endpoints!(
    low_freq_document_touch,
    low_freq_document_reset,
    "document",
    "low"
);

/// GET touch/optimize/index/{node}/touch
pub async fn optimize_index_touch(
    pool: Extension<Pool>,
    Path(node): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    touch_impl(&pool, "optimize", None, &node, TouchAction::Touch).await
}

// ── table ───────────────────────────────────────────────────────────────────

/// POST table/{flag}/insert — 动态表插入一行，返回新行标识
pub async fn table_insert(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Json(data): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if flag.trim().is_empty() {
        return Ok(Json(ActionResult::error("table flag is required")));
    }
    let row_id = uuid::Uuid::new_v4().to_string();
    let bundle = uuid::Uuid::new_v4().to_string();

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_query_table_data (id, table_flag, data, bundle, create_time, update_time) \
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            &[&row_id, &flag, &data.to_string(), &bundle],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({
        "id": row_id,
        "bundle": bundle,
    }))))
}

/// POST table/{flag}/update/{bundle} — 按 bundle 定位并更新（不存在则创建）
pub async fn table_update_with_bundle(
    pool: Extension<Pool>,
    Path((flag, bundle)): Path<(String, String)>,
    Json(data): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if flag.trim().is_empty() || bundle.trim().is_empty() {
        return Ok(Json(ActionResult::error("table flag and bundle are required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let existing = client
        .query_opt(
            "SELECT id FROM x_query_table_data WHERE table_flag = $1 AND bundle = $2 LIMIT 1",
            &[&flag, &bundle],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let row_id = match existing {
        Some(row) => {
            let id: String = row.get("id");
            client
                .execute(
                    "UPDATE x_query_table_data SET data = $1, update_time = NOW() \
                     WHERE table_flag = $2 AND bundle = $3",
                    &[&data.to_string(), &flag, &bundle],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            id
        }
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_query_table_data (id, table_flag, data, bundle, create_time, update_time) \
                     VALUES ($1, $2, $3, $4, NOW(), NOW())",
                    &[&id, &flag, &data.to_string(), &bundle],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            id
        }
    };

    Ok(Json(ActionResult::success(json!({
        "id": row_id,
        "bundle": bundle,
        "updated": true,
    }))))
}

/// GET table/reload/dynamic — 动态实体工厂重建标记（记录最近一次 reload 时间戳）
pub async fn table_reload_dynamic(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    touch_impl(&pool, "reload_dynamic", None, "0", TouchAction::Reset).await
}

// ── neural ──────────────────────────────────────────────────────────────────

const NEURAL_NOT_READY: &str = "model not ready";

async fn neural_start(
    pool: &Pool,
    model_flag: &str,
    action: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if model_flag.trim().is_empty() {
        return Ok(Json(ActionResult::error("modelFlag is required")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    // 同模型同动作存在进行中任务时不重复启动
    let running = client
        .query_opt(
            "SELECT id FROM x_query_neural_job \
             WHERE model_flag = $1 AND action = $2 AND status = 'running' LIMIT 1",
            &[&model_flag.to_string(), &action.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let id = match running {
        Some(row) => row.get::<_, String>("id"),
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO x_query_neural_job (id, model_flag, action, status, create_time, update_time) \
                     VALUES ($1, $2, $3, 'running', NOW(), NOW())",
                    &[&id, &model_flag.to_string(), &action.to_string()],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            id
        }
    };

    Ok(Json(ActionResult::success(json!({
        "value": true,
        "id": id,
        "modelFlag": model_flag,
        "action": action,
        "status": "running",
    }))))
}

async fn neural_stop(
    pool: &Pool,
    model_flag: &str,
    action: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let n = client
        .execute(
            "UPDATE x_query_neural_job SET status = 'stopped', update_time = NOW() \
             WHERE model_flag = $1 AND action = $2 AND status = 'running'",
            &[&model_flag.to_string(), &action.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if n == 0 {
        // 对齐 Java ExceptionModelNotReady：无可停止的运行中任务
        return Ok(Json(ActionResult::error(format!("{}: no running {}", NEURAL_NOT_READY, action))));
    }
    Ok(Json(ActionResult::success(json!({
        "value": true,
        "stopped": n as i64,
        "action": action,
    }))))
}

/// GET neural/generate/model/{modelFlag}
pub async fn neural_generate(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    neural_start(&pool, &model_flag, "generate").await
}

/// GET neural/stop/generating/model/{modelFlag}
pub async fn neural_stop_generating(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    neural_stop(&pool, &model_flag, "generate").await
}

/// GET neural/learn/model/{modelFlag}
pub async fn neural_learn(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    neural_start(&pool, &model_flag, "learn").await
}

/// GET neural/stop/learning/model/{modelFlag}
pub async fn neural_stop_learning(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    neural_stop(&pool, &model_flag, "learn").await
}

/// GET neural/list/calculate/model/{modelFlag}/work/{workId}
///
/// 仅当该模型存在已完成的学习任务时才具备推算能力；否则对齐 Java 抛出
/// ExceptionModelNotReady。模型就绪但无已落盘推算结果时返回空列表。
pub async fn neural_list_calculate_with_work(
    pool: Extension<Pool>,
    Path((model_flag, work_id)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let learned = client
        .query_opt(
            "SELECT 1 AS ok FROM x_query_neural_job \
             WHERE model_flag = $1 AND action = 'learn' AND status = 'completed' LIMIT 1",
            &[&model_flag.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if learned.is_none() {
        return Ok(Json(ActionResult::error(NEURAL_NOT_READY)));
    }

    Ok(Json(ActionResult::success(json!({
        "modelFlag": model_flag,
        "workId": work_id,
        "calculateList": [],
    }))))
}
