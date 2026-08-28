//! plan002 U2 收尾：对齐 Java x_correlation_service_processing CorrelationAction 契约。
//!
//! Java 契约（{war}/jaxrs/correlation/**）：
//!   POST   correlation/type/processplatform/job/{job}          创建关联内容（upsert）
//!   POST   correlation/type/cms/document/{document}            创建关联内容（upsert）
//!   POST   correlation/update/type/processplatform/job/{job}   按 site 整体替换
//!   POST   correlation/update/type/cms/document/{document}     按 site 整体替换
//!   POST   correlation/delete/type/processplatform/job/{job}   按 idList 删除（校验归属）
//!   POST   correlation/delete/type/cms/document/{document}     按 idList 删除（校验归属）
//!   POST   correlation/readable/type/processplatform           判断被关联来源是否可读
//!   POST   correlation/readable/type/cms                       判断被关联来源是否可读
//!   GET    correlation/list/type/processplatform/job/{job}      列示关联内容
//!   GET    correlation/list/type/processplatform/job/{job}/site/{site}
//!   GET    correlation/list/type/cms/document/{document}
//!   GET    correlation/list/type/cms/document/{document}/site/{site}
//!
//! 存储列对齐 Java Correlation 实体（migration 077 增量补充）：
//!   from_type / from_bundle / target_type / target_bundle / person / site /
//!   view / target_title / target_category / target_start_time / target_creator_person。
//! 旧版遗留列（type/target_id/person_id）仅由既有非契约端点继续使用，互不干扰。

use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::{error::AppError, response::ActionResult};

pub const TYPE_PROCESSPLATFORM: &str = "processplatform";
pub const TYPE_CMS: &str = "cms";

type Row = deadpool_postgres::tokio_postgres::Row;

// ── Wi ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TargetWi {
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub bundle: Option<String>,
    #[serde(default)]
    pub site: Option<String>,
    #[serde(default)]
    pub view: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWi {
    #[serde(default)]
    pub person: Option<String>,
    #[serde(rename = "targetList", default)]
    pub target_list: Vec<TargetWi>,
}

#[derive(Debug, Deserialize)]
pub struct SiteTargetWi {
    #[serde(default)]
    pub site: Option<String>,
    #[serde(rename = "targetList", default)]
    pub target_list: Vec<TargetWi>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWi {
    #[serde(default)]
    pub person: Option<String>,
    #[serde(rename = "siteTargetList", default)]
    pub site_target_list: Vec<SiteTargetWi>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteWi {
    #[serde(rename = "idList", default)]
    pub id_list: Vec<String>,
}

/// Java ActionReadableTypeCmsWi 字段名即为历史拼写 `doucment`
#[derive(Debug, Deserialize)]
pub struct ReadableCmsWi {
    #[serde(default)]
    pub person: Option<String>,
    #[serde(default)]
    pub doucment: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReadablePpWi {
    #[serde(default)]
    pub person: Option<String>,
    #[serde(default)]
    pub job: Option<String>,
}

fn target_entry(t: &TargetWi) -> Value {
    json!({
        "type": t.r#type.clone().unwrap_or_default(),
        "bundle": t.bundle.clone().unwrap_or_default(),
        "site": t.site.clone().unwrap_or_default(),
        "view": t.view.clone().unwrap_or_default(),
    })
}

/// 归一化（trim + 类型小写）；type/bundle 为空视为非法目标
fn normalized(t: &TargetWi) -> Option<(String, String, String, String)> {
    let ty = t.r#type.as_deref()?.trim();
    let bundle = t.bundle.as_deref()?.trim();
    if ty.is_empty() || bundle.is_empty() {
        return None;
    }
    let site = t.site.as_deref().unwrap_or("").trim().to_string();
    let view = t.view.as_deref().unwrap_or("").trim().to_string();
    Some((ty.to_lowercase(), bundle.to_string(), site, view))
}

async fn upsert_targets(
    client: &deadpool_postgres::Client,
    from_type: &str,
    from_bundle: &str,
    person: &str,
    targets: &[TargetWi],
) -> Result<(Vec<Value>, Vec<Value>), AppError> {
    let mut success = Vec::new();
    let mut failure = Vec::new();

    for t in targets {
        let Some((ty, bundle, site, view)) = normalized(t) else {
            failure.push(target_entry(t));
            continue;
        };
        // 仅接受 processplatform / cms 两类目标，其余按 Java readTarget 语义计入失败列表
        if ty != TYPE_PROCESSPLATFORM && ty != TYPE_CMS {
            failure.push(target_entry(t));
            continue;
        }

        let existing = client
            .query_opt(
                "SELECT id FROM x_correlation \
                 WHERE from_type = $1 AND from_bundle = $2 \
                   AND target_type = $3 AND target_bundle = $4 \
                   AND COALESCE(site, '') = $5 LIMIT 1",
                &[&from_type.to_string(), &from_bundle.to_string(), &ty, &bundle, &site],
            )
            .await
            .map_err(|_| AppError::Internal)?;

        match existing {
            Some(row) => {
                // 已存在：合并 view 等展示字段（保留原 target_title）
                let _ = client
                    .execute(
                        "UPDATE x_correlation SET view = $1, update_time = NOW() WHERE id = $2",
                        &[&view, &row.get::<_, String>("id")],
                    )
                    .await;
            }
            None => {
                let id = uuid::Uuid::new_v4().to_string();
                client
                    .execute(
                        "INSERT INTO x_correlation \
                           (id, from_type, from_bundle, target_type, target_bundle, person, \
                            site, view, create_time, update_time) \
                         VALUES ($1, $2, $3, $4, $5, $6, NULLIF($7, ''), NULLIF($8, ''), NOW(), NOW())",
                        &[&id, &from_type.to_string(), &from_bundle.to_string(), &ty,
                          &bundle, &person.to_string(), &site, &view],
                    )
                    .await
                    .map_err(|_| AppError::Internal)?;
            }
        }

        success.push(json!({
            "type": ty,
            "bundle": bundle,
            "site": site,
            "view": view,
        }));
    }

    Ok((success, failure))
}

// ── 创建（upsert）───────────────────────────────────────────────────────────

async fn create_impl(
    pool: &Pool,
    from_type: &str,
    from_bundle: &str,
    wi: CreateWi,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if from_bundle.trim().is_empty() {
        return Ok(Json(ActionResult::error("bundle is required")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let person = wi.person.unwrap_or_default();
    let (success, failure) =
        upsert_targets(&client, from_type, &from_bundle, &person, &wi.target_list).await?;
    Ok(Json(ActionResult::success(json!({
        "successList": success,
        "failureList": failure,
    }))))
}

/// POST correlation/type/processplatform/job/{job}
pub async fn create_pp(
    pool: Extension<Pool>,
    Path(job): Path<String>,
    Json(wi): Json<CreateWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    create_impl(&pool, TYPE_PROCESSPLATFORM, &job, wi).await
}

/// POST correlation/type/cms/document/{document}
pub async fn create_cms(
    pool: Extension<Pool>,
    Path(document): Path<String>,
    Json(wi): Json<CreateWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    create_impl(&pool, TYPE_CMS, &document, wi).await
}

// ── 更新（按 site 替换）─────────────────────────────────────────────────────

async fn update_impl(
    pool: &Pool,
    from_type: &str,
    from_bundle: &str,
    wi: UpdateWi,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if from_bundle.trim().is_empty() {
        return Ok(Json(ActionResult::error("bundle is required")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let person = wi.person.unwrap_or_default();
    let mut success_all = Vec::new();
    let mut failure_all = Vec::new();

    for site_target in wi.site_target_list {
        let Some(site) = site_target
            .site
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
        else {
            // Java：site 为空的分组直接跳过
            continue;
        };

        // 先删除该 site 下全部旧关联，再插入新集合（Java ActionUpdate* 语义）
        client
            .execute(
                "DELETE FROM x_correlation \
                 WHERE from_type = $1 AND from_bundle = $2 AND COALESCE(site, '') = $3",
                &[&from_type.to_string(), &from_bundle.to_string(), &site],
            )
            .await
            .map_err(|_| AppError::Internal)?;

        let owned: Vec<TargetWi> = site_target
            .target_list
            .into_iter()
            .map(|mut t| {
                t.site = Some(site.clone());
                t
            })
            .collect();

        let (success, failure) =
            upsert_targets(&client, from_type, from_bundle, &person, &owned).await?;
        success_all.extend(success);
        failure_all.extend(failure);
    }

    Ok(Json(ActionResult::success(json!({
        "successList": success_all,
        "failureList": failure_all,
    }))))
}

/// POST correlation/update/type/processplatform/job/{job}
pub async fn update_pp(
    pool: Extension<Pool>,
    Path(job): Path<String>,
    Json(wi): Json<UpdateWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    update_impl(&pool, TYPE_PROCESSPLATFORM, &job, wi).await
}

/// POST correlation/update/type/cms/document/{document}
pub async fn update_cms(
    pool: Extension<Pool>,
    Path(document): Path<String>,
    Json(wi): Json<UpdateWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    update_impl(&pool, TYPE_CMS, &document, wi).await
}

// ── 删除（按 idList，校验归属）──────────────────────────────────────────────

async fn delete_impl(
    pool: &Pool,
    from_type: &str,
    from_bundle: &str,
    wi: DeleteWi,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if wi.id_list.is_empty() {
        return Ok(Json(ActionResult::success(json!({ "value": true }))));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, from_type, from_bundle FROM x_correlation WHERE id = ANY($1)",
            &[&wi.id_list],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    for row in &rows {
        let row_type: String = row.get::<_, Option<String>>("from_type").unwrap_or_default();
        let row_bundle: String = row.get::<_, Option<String>>("from_bundle").unwrap_or_default();
        if !row_type.eq_ignore_ascii_case(from_type) {
            return Ok(Json(ActionResult::error(format!(
                "type not match: {} != {}",
                row_type, from_type
            ))));
        }
        if row_bundle != from_bundle {
            return Ok(Json(ActionResult::error(format!(
                "bundle not match: {} != {}",
                row_bundle, from_bundle
            ))));
        }
    }

    let n = client
        .execute(
            "DELETE FROM x_correlation WHERE id = ANY($1)",
            &[&wi.id_list],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({
        "value": true,
        "deleted": n as i64,
    }))))
}

/// POST correlation/delete/type/processplatform/job/{job}
pub async fn delete_pp(
    pool: Extension<Pool>,
    Path(job): Path<String>,
    Json(wi): Json<DeleteWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_impl(&pool, TYPE_PROCESSPLATFORM, &job, wi).await
}

/// POST correlation/delete/type/cms/document/{document}
pub async fn delete_cms(
    pool: Extension<Pool>,
    Path(document): Path<String>,
    Json(wi): Json<DeleteWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    delete_impl(&pool, TYPE_CMS, &document, wi).await
}

// ── 可读性判断 ──────────────────────────────────────────────────────────────

/// 查找指向 target 的全部关联的来源；只要存在任一来源满足：
///   - processplatform 来源：当前人在 x_review 有评审记录，或为该 work 创建者；
///   - cms 来源：文档存在且创建者即本人（或未记录创建者）；
/// 则 value=true。权限表不存在时视为无证据 → value=false。
async fn readable_impl(
    pool: &Pool,
    person: &str,
    target_type: &str,
    target_bundle: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT DISTINCT from_type, from_bundle FROM x_correlation \
             WHERE target_type = $1 AND target_bundle = $2 \
               AND from_bundle IS NOT NULL",
            &[&target_type.to_string(), &target_bundle.to_string()],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let mut pp_bundles: Vec<String> = Vec::new();
    let mut cms_bundles: Vec<String> = Vec::new();
    for row in rows {
        let ft: String = row.get::<_, Option<String>>("from_type").unwrap_or_default();
        let fb: String = row.get("from_bundle");
        match ft.to_lowercase().as_str() {
            TYPE_PROCESSPLATFORM => pp_bundles.push(fb),
            TYPE_CMS => cms_bundles.push(fb),
            _ => {}
        }
    }

    let mut readable = false;

    if !pp_bundles.is_empty() && !person.trim().is_empty() {
        // 权限证据：评审记录 或 work 创建者本人；权限基表缺失时不作为失败条件
        if let Ok(Some(row)) = client
            .query_opt(
                "SELECT EXISTS(\
                     SELECT 1 FROM x_review WHERE reviewer = $1 AND work_id = ANY($2)\
                 ) OR EXISTS(\
                     SELECT 1 FROM x_work WHERE creator = $1 AND id = ANY($2)\
                 ) AS readable",
                &[&person.to_string(), &pp_bundles],
            )
            .await
        {
            readable |= row.get::<_, bool>("readable");
        }
    }

    if !readable && !cms_bundles.is_empty() && !person.trim().is_empty() {
        // schema 子集无独立权限表：以「文档存在且创建者可确认」作为数据级代理
        if let Ok(Some(row)) = client
            .query_opt(
                "SELECT EXISTS(\
                     SELECT 1 FROM x_cms_document \
                     WHERE id = ANY($1) \
                       AND (creator_person IS NULL OR creator_person = $2)\
                 ) AS readable",
                &[&cms_bundles, &person.to_string()],
            )
            .await
        {
            readable |= row.get::<_, bool>("readable");
        }
    }

    Ok(Json(ActionResult::success(json!({ "value": readable }))))
}

/// POST correlation/readable/type/processplatform
pub async fn readable_pp(
    pool: Extension<Pool>,
    Json(wi): Json<ReadablePpWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let job = wi.job.unwrap_or_default();
    if job.trim().is_empty() {
        return Ok(Json(ActionResult::error("job is required")));
    }
    let person = wi.person.unwrap_or_default();
    readable_impl(&pool, &person, TYPE_PROCESSPLATFORM, &job).await
}

/// POST correlation/readable/type/cms
pub async fn readable_cms(
    pool: Extension<Pool>,
    Json(wi): Json<ReadableCmsWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // Java Wi 字段历史拼写即为 doucment
    let document = wi.doucment.unwrap_or_default();
    if document.trim().is_empty() {
        return Ok(Json(ActionResult::error("doucment is required")));
    }
    let person = wi.person.unwrap_or_default();
    readable_impl(&pool, &person, TYPE_CMS, &document).await
}

// ── 列示 ────────────────────────────────────────────────────────────────────

const LIST_COLUMNS: &str = "id, from_type, from_bundle, target_type, target_bundle, \
                            person, site, view, target_title, target_category, \
                            target_start_time, target_creator_person, create_time";

fn opt_str(row: &Row, col: &str) -> String {
    row.get::<_, Option<String>>(col).unwrap_or_default()
}

fn row_to_item(row: &Row) -> Value {
    json!({
        "id": opt_str(row, "id"),
        "fromType": opt_str(row, "from_type"),
        "fromBundle": opt_str(row, "from_bundle"),
        "targetType": opt_str(row, "target_type"),
        "targetBundle": opt_str(row, "target_bundle"),
        "person": opt_str(row, "person"),
        "site": opt_str(row, "site"),
        "view": opt_str(row, "view"),
        "targetTitle": opt_str(row, "target_title"),
        "targetCategory": opt_str(row, "target_category"),
        "targetStartTime": opt_str(row, "target_start_time"),
        "targetCreatorPerson": opt_str(row, "target_creator_person"),
        "createTime": opt_str(row, "create_time"),
    })
}

async fn list_impl(
    pool: &Pool,
    from_type: &str,
    from_bundle: &str,
    site: Option<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = match site.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(s) => {
            client
                .query(
                    &format!(
                        "SELECT {LIST_COLUMNS} FROM x_correlation \
                         WHERE from_type = $1 AND from_bundle = $2 AND COALESCE(site, '') = $3 \
                         ORDER BY create_time DESC NULLS LAST"
                    ),
                    &[&from_type.to_string(), &from_bundle.to_string(), &s.to_string()],
                )
                .await
        }
        None => {
            client
                .query(
                    &format!(
                        "SELECT {LIST_COLUMNS} FROM x_correlation \
                         WHERE from_type = $1 AND from_bundle = $2 \
                         ORDER BY create_time DESC NULLS LAST"
                    ),
                    &[&from_type.to_string(), &from_bundle.to_string()],
                )
                .await
        }
    }
    .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows.iter().map(row_to_item).collect();
    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(Value::Array(data), count, 0)))
}

/// GET correlation/list/type/processplatform/job/{job}
pub async fn list_pp(
    pool: Extension<Pool>,
    Path(job): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_impl(&pool, TYPE_PROCESSPLATFORM, &job, None).await
}

/// GET correlation/list/type/processplatform/job/{job}/site/{site}
pub async fn list_pp_site(
    pool: Extension<Pool>,
    Path((job, site)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_impl(&pool, TYPE_PROCESSPLATFORM, &job, Some(site)).await
}

/// GET correlation/list/type/cms/document/{document}
pub async fn list_cms(
    pool: Extension<Pool>,
    Path(document): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_impl(&pool, TYPE_CMS, &document, None).await
}

/// GET correlation/list/type/cms/document/{document}/site/{site}
pub async fn list_cms_site(
    pool: Extension<Pool>,
    Path((document, site)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    list_impl(&pool, TYPE_CMS, &document, Some(site)).await
}
