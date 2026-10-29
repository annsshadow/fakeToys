# B2 · invoke token 零验签 —— 存储与校验方案草案（2026-10-30）

> **性质**：前置设计草案，供产品拍板后落地。本文件只含方案与 SQL 草案，
> **不包含**任何已生效的迁移或 handler 改动（避免未拍板先改行为）。
> 对应 `docs/plans/2026-10-29-remaining-work-plan.md` 的 **B2** 行。

---

## 一、现状

**问题**：`POST /api/program_center/invoke/{flag}/client/{client}/token/{token}/execute`
（`crates/program_center/src/lib.rs:11270 u3_invoke_execute_with_token`）的 token 只判非空：

```rust
if enable_token && !enable_anonymous && token.map(str::trim).unwrap_or_default().is_empty() {
    return Ok(Json(ActionResult::error("token required")));
}
```

只要传一个非空字符串即可通过（**零验签**），任何知道路由的人都可以用任意 token 触发执行。

**相关现状**：
- 路由已有 `require_session` 门（无会话 401），所以不是匿名面；但 token 作为「额外调用方
  凭证」形同虚设。
- `x_program_invoke` 表（`migrations/074_u2_small_modules.sql`）有 `enable_token` /
  `enable_anonymous` 两个开关列，**没有 token 存储列**。
- 管理端点：`u2_invoke_create`（POST /invoke）、`u2_invoke_update`（PUT /invoke/{flag}）
  均 admin 门禁，可扩展 token 管理。
- `u2_invoke_get`（GET /invoke/{flag}）当前不返回 token 相关列（保持不回显 hash）。

## 二、方案 A（推荐）：静态 token，admin 配置，hash 存储

适合「固定调用方（服务端到服务端）」场景。一次配置，长期有效，换 token 即轮换。

### A1. 迁移 SQL（草案，未生效）

```sql
-- migrations/1xx_add_program_invoke_token_hash.sql
-- B2：x_program_invoke 增加 token 哈希列。只存哈希，不存明文。
ALTER TABLE "x_program_invoke"
    ADD COLUMN IF NOT EXISTS "token_hash" TEXT;

-- 需要按 token 查找时（如按 client 维度）可加索引；单表量小，可暂不加。
```

设计要点：
- 列名 `token_hash`：存 `HMAC-SHA256(token, 固定盐)` 或 `bcrypt(token)`。推荐 **HMAC-SHA256
  + 每行随机盐**（`salt` 列可并入 hash 字符串，格式 `v1:<salt>:<hex>`），避免 bcrypt 的
  60-100ms CPU 开销被滥用为放大攻击（执行端点本身不限流时）。
- 回滚：`ALTER TABLE "x_program_invoke" DROP COLUMN IF EXISTS "token_hash";`

### A2. 管理端改动（落地方案）

- `U2InvokeRequest` 增加可选 `token: Option<String>` 字段。
- **创建**（`u2_invoke_create`）：`enable_token = true` 时：
  - 若请求带 `token` → 校验长度（建议 ≥ 16 字符）后存 `token_hash`；
  - 若不带 → 服务端生成随机 token（如 32 字节 hex），**只返回一次**，落库只存 hash。
- **更新**（`u2_invoke_update`）：
  - 若请求带 `token` → 覆盖 `token_hash`（**轮换**，旧 token 立即失效）；
  - 若不带 → 保持原 `token_hash` 不变（不覆盖已配置的 token）。
- **查询**（`u2_invoke_get`）：永远不回显 `token_hash`，只回显 `enable_token` 开关。
- 取消 token：`enable_token = false` 时执行端点不再要求 token（现有语义保留）。

### A3. 校验逻辑（落地方案）

`u3_invoke_execute_inner` 中，把「只判非空」改成「校验 hash」：

```rust
if enable_token && !enable_anonymous {
    let t = token.map(str::trim).unwrap_or_default();
    if t.is_empty() {
        return Ok(Json(ActionResult::error("token required")));
    }
    let stored: Option<String> = row.get("token_hash");
    match stored.as_deref() {
        Some(h) if verify_invoke_token(t, h).await? => { /* 通过 */ }
        _ => return Ok(Json(ActionResult::error("invalid token"))),
    }
}
```

- `verify_invoke_token` 用**常量时间比较**（`subtle` / `ring::constant_time` 或自制 XOR 比较），
  防时序侧信道。
- token 校验失败与「token required」返回不同错误，便于对接方排错；不区分「invoke 不存在」
  与「token 错误」时可按产品口径合并为统一错误。

### A4. 安全要点

1. 存储只留哈希，DB 泄露不直接泄露可用的 token。
2. 轮换策略：管理员重新 PUT 即轮换，旧 token 立即失效；无自动过期（静态 token 语义），
   如需过期可后续加 `token_expires_at` 列（本草案不引入）。
3. 执行端点已有 `require_session`，token 是第二层凭证；若产品希望「纯 token 免会话」调用，
   需单独拍板（见 §四）。
4. `client` 路径参数当前未被使用。若需按 client 隔离 token，可把 `token_hash` 改为
   `client_token_hash JSONB`（client → hash 映射）；本草案按「单 token 对单 invoke」处理。

## 三、方案 B（备选）：动态签发（OAuth client credentials 风格）

- 增加 `POST /api/program_center/invoke/token` 签发端点（admin 门禁或按 client secret 签发）。
- token 短时有效（如 30 分钟），带过期时间，需要刷新机制。
- 适合「不可信客户端、频繁轮换」场景；实现量与运维成本显著高于方案 A。
- 若产品对接方需要标准协议，可考虑此方案；否则推荐 A。

## 四、待拍板清单（产品/对接方）

| # | 问题 | 默认建议 |
|---|---|---|
| 1 | token 从哪来？ | **方案 A**：admin 在创建/编辑 invoke 时配置或由服务端生成 |
| 2 | 纯 token 能否免会话调用？ | **否**（保留 `require_session`），token 作为第二层凭证 |
| 3 | token 是否按 client 隔离？ | 否（单 token 对单 invoke），`client` 参数暂作日志用途 |
| 4 | 轮换策略 | 管理员 PUT 覆盖即轮换，旧 token 立即失效 |
| 5 | 是否需要过期时间 | 暂不需要（静态 token）；后续可加 `token_expires_at` |

## 五、测试计划（拍板后落地）

- 单元：`verify_invoke_token` 正确/错误/空 token、常量时间比较。
- 集成（DB 臂）：`enable_token=true` 且配置 token 后，正确 token 放行、错误 token 拒绝且
  **不更新 last_start_time**；`enable_token=false` 时无 token 也放行（回归现状语义）。
- 管理端：create 带 token 落 hash、update 覆盖轮换、get 不回显 hash。
- 迁移：`token_hash` 列可空，老数据（无 token）在 `enable_token=true` 时返回「token required」
  而非 500（fail-closed）。

## 六、关联项

- B4（bbs create_reply 功能桩）：**已确认完成**（见计划 B4 行销账）——`create_reply` 已真实
  `INSERT INTO x_bbs_reply` 且带 `require_session` 门，`x_bbs_reply` 表迁移齐全（migrations
  021/033/037/067）。
- C1（shared 覆盖率 94%）：本机 `cargo llvm-cov` 在 GNU 工具链下编译失败（E0463，
  环境性），覆盖率以计划文档记录的 93.28% 为基线；判据保持条件项。