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
