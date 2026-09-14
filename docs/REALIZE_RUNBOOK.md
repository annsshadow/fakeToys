# OA4Rust 桩代码真实化执行手册（Runbook）

本手册供逐 crate 消除桩代码的执行者（人工或子代理）使用。目标：让每个 crate 的
`router(pool)` 暴露**真实**的 PostgreSQL 业务逻辑 handler，清除所有 `stub_` / `TODO`
/`Value::Null` 桩标记，并补至少 1 个可编译通过的测试。

---

## 0. 全局约束（违反会导致全仓编译失败，务必遵守）

- **禁止修改**：`oa4rust/Cargo.toml`、`oa4rust/src/main.rs`、`oa4rust/crates/shared/**`、
  其他 crate 的代码。只修改你负责的那几个 crate 目录。
- **保留签名**：每个 crate 必须导出 `pub fn router(pool: Pool) -> axum::Router`
  （与 `main.rs` 调用 `xxx::router(pool.clone())` 一致）。不要把 `pool` 改名或改类型，
  不要改为 `async fn`。`router` 内部可以 `.layer(Extension(pool))`。
- **编译门禁**：完成后必须 `cargo build -p <你的crate名>` 通过（warning 可接受，error 必须清零）。
- **不要引入新依赖**：只用 crate 已有的依赖（`axum` / `deadpool_postgres` / `serde_json` /
  `serde` / `shared`）。
- **DB 不可用**：本环境无 PostgreSQL，集成测试无法连库。测试必须**无库也能编译并通过**
  （纯单元/结构测试，或对 DB 测试用 `env::var("DATABASE_URL")` 守卫后提前返回）。

---

## 1. 现有代码模式（务必先读再改）

### 1.1 Handler 签名（所有真实 handler 统一）
```rust
use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

pub async fn get_x(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client.query_opt("SELECT ... FROM x_table WHERE id = $1", &[&id]).await
        .map_err(|_| AppError::Internal)?;
    match row {
        Some(row) => Ok(Json(ActionResult::success(/* 真实 JSON */))),
        None => Ok(Json(ActionResult::error("not found"))),
    }
}
```

### 1.2 通用读 handler（不知道具体列名时复用，避免 Value::Null 桩）
```rust
use shared::response::row_to_json;
pub async fn list_x(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client.query("SELECT * FROM x_table ORDER BY id LIMIT 200", &[])
        .await.map_err(|_| AppError::Internal)?;
    let data: Vec<Value> = rows.iter().map(|r| row_to_json(r)).collect();
    Ok(Json(ActionResult::success(Value::Array(data))))
}
```
`row_to_json` 已在 `shared::response` 提供，自动把每行转为 JSON（支持 bool/整数/浮点/文本）。

### 1.3 通用写 handler（INSERT/UPDATE 模板）
```rust
pub async fn save_x(
    pool: Extension<Pool>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = payload.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let existing = client.query_opt("SELECT id FROM x_table WHERE id = $1", &[&id]).await
        .map_err(|_| AppError::Internal)?;
    if existing.is_some() {
        client.execute("UPDATE x_table SET name = $1 WHERE id = $2", &[&name, &id]).await
            .map_err(|_| AppError::Internal)?;
    } else {
        let new_id = if id.is_empty() { uuid::Uuid::new_v4().to_string() } else { id };
        client.execute("INSERT INTO x_table (id, name) VALUES ($1, $2)", &[&new_id, &name]).await
            .map_err(|_| AppError::Internal)?;
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::json!({"id": id, "saved": true})))))
}
```

---

## 2. 六步执行流程（每个 crate）

### Step 1 — 通读 crate
读 `crates/<name>/src/lib.rs` 及其 `mod` 子模块（如有 `routes.rs`、`person.rs` 等）。
确认：现有真实 handler 有哪些？`router()` 当前返回什么？

### Step 2 — 暴露真实路由（最关键）
`main.rs` 调用 `<crate>::router(pool)`。很多 crate 的 `router()` 目前只返回：
```rust
pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/<crate>/health", axum::routing::get(|| async { "TODO: ... real implementation needed" }))
}
```
把它改成暴露真实 handler。常见两种情形：
- **情形 A**：crate 已有 `<crate>_router(pool) -> Router { routes::<crate>_routes(pool) }`
  且该函数已把真实 handler 都注册好了。则把 `router` 改为：
  ```rust
  pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
      <crate>_router(pool)
  }
  ```
- **情形 B**：真实 handler 直接写在 `lib.rs`/`routes.rs` 里但未全部注册。则 `router(pool)`
  构建 `Router::new().route(...).layer(Extension(pool))`，把每个真实 handler 都 `.route(...)` 注册上
  （参考 `control` crate 的 `routes.rs` 写法）。
- **情形 C**：`router()` 直接包含真实 handler（如 `control`、`auth`），无需改动路由，只做 Step 3/4。

> 注意：若一个 handler 同时需要认证/授权，保持现状即可——`main.rs` 已统一在顶层
> 套了 `auth_middleware` / `authorize_middleware`，crate 内部无需重复加。

### Step 3 — 去除 `stub_` 前缀
对 crate 内所有 `.rs`，把函数名 `stub_<crate>_` 前缀去掉（仅去前缀，保留其余）。
例如 `stub_attendance_assemble_control_foo` → `foo`，同时 routes 里的
`crate::stub_attendance_assemble_control_foo` 引用也要同步改（前缀字符串一致，全局替换即可）。
可用脚本：
```bash
cd crates/<name>/src
grep -rl "stub_<name>_" . | xargs sed -i "s/stub_<name>_//g"
```
（Windows 下用 Python 等价替换，见下方"安全替换"。）

### Step 4 — 消除纯 `Value::Null` 桩
搜索 `ActionResult::success(Value::Null)`。若其 handler 函数体内**没有任何 DB 查询**
（纯返回 Null），按 1.2/1.3 改写为真实 handler：
- GET 列表/详情 → 用 `SELECT * FROM <推断表名>` + `row_to_json`（表名按路由推断，
  形如 `x_<module>_<entity>`，参考同 crate 已有的真实 SQL 表名）。
- POST/PUT/DELETE → 用 1.3 模板（INSERT/UPDATE/DELETE）。
若 handler 内已有真实查询只是额外 return Null，则只替换那个 Null 返回为真实结果。

### Step 5 — 删除 TODO 占位
删除所有 `"TODO: ... real implementation needed"` 字符串及对应占位路由。
（Step 2 改掉 `router()` 时通常已一并删除。）

### Step 6 — 补测试
在 crate 末尾确保有：
```rust
#[cfg(test)]
mod tests;
```
并在 `tests.rs` 中至少放 1 个**无库也能通过**的单元测试，例如：
```rust
#[cfg(test)]
mod tests {
    use shared::response::ActionResult;
    #[test]
    fn success_has_type_field() {
        let r = ActionResult::success(serde_json::json!({"ok": true}));
        assert_eq!(r.r#type.as_deref(), Some("success"));
        assert!(r.data.is_some());
    }
}
```
若该 crate 已有 `tests.rs`，在其内追加即可，不要破坏现有测试。

---

## 3. 安全替换（Windows 无 GNU sed 时用 Python）
```python
import pathlib, re
crate = "attendance_assemble_control"
prefix = f"stub_{crate}_"
base = pathlib.Path(f"crates/{crate}/src")
for f in base.rglob("*.rs"):
    t = f.read_text(encoding="utf-8")
    if prefix in t:
        f.write_text(t.replace(prefix, ""), encoding="utf-8")
        print("updated", f)
```

---

## 4. 完成判据（每个 crate）
- [ ] `cargo build -p <crate>` 通过（无 error）。
- [ ] grep `stub_` 该 crate 目录结果为 0。
- [ ] grep `real implementation needed` 该 crate 目录结果为 0。
- [ ] grep `ActionResult::success(Value::Null)` 该 crate 目录结果为 0（或仅剩带真实查询后正确返回的个别——不允许纯 Null）。
- [ ] `router(pool)` 暴露了真实 handler（不再只是 health 占位）。
- [ ] 至少有 1 个可编译通过的测试。

---

## 4.1 设计器路由对账回归（跨 crate，一次性）

`oa4rust/tests/designer_route_match.rs` 用 `axum oneshot`（懒建 `deadpool` 池，**无需 DB**）
把 process/query/portal/form 四大设计器前端真实调用点固化成断言，区分：
`404`=无路由 / `405`=方法不符（路径被宽 `{id}` 影子捕获）/ 非 404·405（=500/415）=已进 handler（
GET 类命中宽路由即"静默影子误路由"，生产带 DB 会返 200 错体，最高危）。

**命令**（须在装有 Rust 工具链的 Windows/host 侧执行；本沙箱无 cargo 无法跑）：
```bash
cd oa4rust && cargo test --test designer_route_match -- --nocapture
```
- 修复某条设计器路由后，把该 case 的 `Expect` 由当前档改为目标态（通常 `Matched`）并补真实语义；
- 详见 `docs/plans/2026-09-11-001-assess-oa4rust-web-full-replacement-gap-plan.md` §9.4 对账表与 W6。
- 判据：命令退出 0（全绿）= 当前快照与实测一致；若某 case 失败，说明路由被改动或分类需回写 §9.4。

---

## 4.2 S4 金丝雀 / pilot 观察窗口门禁

S4 不做 Java 影子比对，也不从代码仓库伪造一次 pilot 结果。它只在小范围用户实际使用
OA4Rust 新栈后，离线核算专用 Nginx access log 的请求总数与 5xx 错误预算。当前服务没有
Prometheus 请求计数器；`trace_middleware` 又只记录 5xx，无法提供分母，因此以入口 access log
作为唯一计数源。

### 前置约束

- `service_url` 必须是 pilot 用户实际访问的 OA4Rust 新栈 URL；对应虚拟主机不得混入 Java 流量。
- 单独记录 pilot 的 `/jaxrs/` API access log。不要拿混合站点、影子请求或人工拼接日志验收。
- 观察窗口使用带时区的 ISO-8601 时间，脚本按 `[window_start, window_end)` 计算。
- `5xx` 定义为 HTTP `500..599`；错误率为 `5xx / 窗口内全部已解析请求`。
- 可用 5xx 个数为 `floor(请求数 × 阈值百分比 / 100)`；另设 `min_requests` 防止空窗口通过。
- 日志有任何非空行无法解析时整次门禁失败，禁止静默丢行。

现有 `deploy/nginx.conf` 的 `main` 格式可直接解析。建议在 pilot 专用 `server` 中用条件日志仅记录
API（`map` 位于 `http` 块，`access_log` 位于 pilot `server` 块）：

```nginx
map $uri $oa4rust_pilot_api {
    default 0;
    ~^/jaxrs/ 1;
}

server {
    # 此 server 仅承载 OA4Rust pilot 新栈
    access_log /var/log/nginx/oa4rust-pilot.access.log main if=$oa4rust_pilot_api;
    # 其余 TLS、静态资源和 proxy 路由保持现场配置。
}
```

### 现场执行

先确认健康端点，再在窗口开始前轮转/截取专用日志，记录用户与部署版本。窗口结束后执行：

```bash
cd oa4rust
python3 scripts/pilot_gate.py \
  --service-url "https://pilot.example.com" \
  --window-start "2026-09-12T10:00:00Z" \
  --window-end "2026-09-12T18:00:00Z" \
  --max-5xx-rate-percent "1" \
  --min-requests "100" \
  --pilot-user "pilot-user-01" \
  --pilot-user "pilot-user-02" \
  --version "image@sha256:..." \
  --access-log /secure/path/oa4rust-pilot.access.log \
  --output target/pilot-gate/report.json \
  --checklist-output target/pilot-gate/manual-checklist.md
```

退出码语义：`0`=自动日志门禁通过，`1`=请求量/5xx/日志完整性门禁失败，`2`=输入或文件错误。
无论退出码如何，脚本在可解析输入下都写 `report.json` 和人工清单；报告包含日志 SHA-256、
pilot 用户、窗口、服务 URL、版本、状态码分布和失败原因。脚本不会请求业务接口制造流量。

也可手工触发独立 workflow `.github/workflows/oa4rust-pilot-gate.yml`。填写相同参数及一个
`https://` access log 下载地址；私有下载地址可配置仓库 secret `PILOT_ACCESS_LOG_TOKEN`。
workflow 上传 `report.json` 与 `manual-checklist.md`，但不上传可能含个人信息的原始日志。

### 放大 / 停止判据

1. 自动门禁必须为 `decision=pass`，且 artifact 与现场日志 SHA-256 对得上。
2. 打开生成的 `manual-checklist.md`，逐项抽检关键旅程、数据正确性、用户反馈与回退准备；
   生成时所有复选框故意保持未完成，必须由审核人签名并记录 UTC 时间。
3. 自动或人工任一项失败，立即停止扩大 pilot 范围，保留日志/报告/工单并处置后开启新窗口；
   不得把失败窗口与后续日志拼接成一次通过。
4. 只有真实外部窗口执行完、自动门禁通过且人工清单闭合后，现场负责人才能批准下一阶段。
   仓库 fixture 和 CI 单测只证明算法可运行，**绝不代表实际 pilot 已通过**。

本地设施回归：

```bash
cd oa4rust
python3 tests/test_pilot_gate.py
python3 -m py_compile scripts/pilot_gate.py tests/test_pilot_gate.py
```

---

## 5. 报告格式（每个 crate 完成后回报）
```
crate: <name>
- 真实 handler 数: N
- 改写 stub_: M 个
- 消除 Value::Null: K 个
- router 暴露: 是/否（说明情形 A/B/C）
- cargo build -p <name>: 通过
- 遗留 TODO: 无 / <列出>
```
