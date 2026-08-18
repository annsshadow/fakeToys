---
date: 2026-08-10
topic: oa4rust-comprehensive-gap-audit
---

# OA4Rust 全面差距审计与补全方向

## Summary

识别 oa4rust 与 Java OA 系统在认证安全、MCP/文档、测试覆盖、业务功能四个维度的差距，确定优先级最高的补全方向，使 oa4rust 达到可替代 Java OA 的生产就绪状态。

---

## Problem Frame

oa4rust 项目在 2026-08-10 已完成全部 83 个 crate 的真实化和 SeaORM 迁移，2458 个真实 handler 覆盖大部分核心业务模块。但从"完全代替 oa"的标准审视，存在四个结构性缺口：

**认证安全模块缺失严重。** Java `x_organization_assemble_authentication` 包含双因素登录、SSO 单点登录、安全注销、Token 校验、用户切换等 20+ 个端点，Rust `auth` crate 仅实现了基础登录/登出/短信验证码/OAuth（企微、钉钉）。微信小程序登录、WeLink、政务钉钉、SSO 等场景完全缺失。

**MCP 工具桥接与 OpenAPI 文档覆盖极低。** MCP 工具桥接当前注册约 100 个工具，覆盖 7624 个总端点的约 1.3%。OpenAPI 规范仅有 14 个占位路径，无法支撑外部系统集成或 AI Agent 发现。

**行为对比测试覆盖率不足 1%。** 当前约 79/7624 个端点有 Rust vs Java 行为对比测试，无法为"代替"提供质量信心。Java 服务不可用时的降级策略（SKIP + mock）已设计，但端点清单本身不完整。

**部分业务模块存在端点级缺口。** 用户注册、电子签名、个人头像上传等 `x_organization_assemble_personal` 模块功能在 Rust `personal` crate 中未完整实现；部分 assemble_control 模块的复杂查询端点（如批量操作、报表聚合）仍有缺失。

这些缺口不是"有没有 crate"的问题——83 个 crate 已全部存在——而是"端点对齐度"的问题：Java 侧有大量端点在 Rust 侧没有对应实现，或实现不完整。

---

## Actors

- **A1（开发者，单人）：** 负责差距审计后的补全实现
- **A2（前端 o2web）：** 依赖 OA API 契约，任何新端点必须保持 `ActionResult<T>` 9 字段结构兼容
- **A3（AI Agent / MCP 客户端）：** 通过 MCP 工具桥接调用 oa4rust 功能，需要完整的工具清单
- **A4（下游规划 Agent）：** 读取本需求文档后执行 ce-plan 进行详细规划

---

## Key Flows

- **F1. 认证安全补全流程**
  - **Trigger：** 识别认证模块缺口后启动补全
  - **Actors：** A1
  - **Steps：**
    1. 对照 Java `x_organization_assemble_authentication` 的端点清单，识别 Rust 侧缺失的端点
    2. 为高优先级缺口（双因素登录、SSO、安全注销）实现 Rust handler
    3. 保持与现有 `auth` crate 的 SessionManager 和中间件兼容
    4. 运行行为对比测试验证新端点
  - **Outcome：** 认证安全模块端点对齐度从 ~40% 提升至 ~80%
  - **Covered by：** R1-R5

- **F2. MCP 工具桥接扩展流**
  - **Trigger：** MCP 客户端需要访问更多 oa4rust 功能
  - **Actors：** A1, A3
  - **Steps：**
    1. 基于 `docs/brainstorms/oa4rust-endpoint-inventory.md` 生成完整端点清单
    2. 将缺失端点注册到 `mcp_server` 的 `ROUTE_DEFS` 静态数组
    3. 为每个新工具补充描述、路径参数和 body 参数元数据
  - **Outcome：** MCP 工具覆盖率从 ~1.3% 提升至 ~60%+
  - **Covered by：** R6-R8

- **F3. 行为对比测试扩展流**
  - **Trigger：** 新增或修改端点后需要验证行为等效性
  - **Actors：** A1, A2
  - **Steps：**
    1. 基于 `tests/behavior_compare_endpoints.rs` 的自动生成机制，扩展端点清单至全量 7624 个
    2. 为新增端点配置 allowlist.yaml 中的字段命名差异规则
    3. 验证 Java 不可用时的降级策略（SKIP + mock 模式）正常工作
  - **Outcome：** 行为对比测试覆盖率达到 100%，Java 不可用时全部标记为 SKIP
  - **Covered by：** R9-R11

---

## Requirements

**认证安全模块补全**
- R1. 实现双因素登录端点（对应 Java `ActionTwoFactoryLogin`），支持 TOTP 或短信验证码的第二因子验证
- R2. 实现 SSO 单点登录端点（对应 Java `sso/SsoAction`），支持 GET/POST 两种方式的 token 登录和 3DES 加密辅助
- R3. 实现安全注销端点（对应 Java `ActionSafeLogout`），使当前用户所有 session 全部过期
- R4. 实现 Token 校验端点（对应 Java `ActionCheckToken`），允许外部系统验证 OA token 有效性
- R5. 实现用户切换端点（对应 Java `ActionSwitchUser`），需要系统管理员权限，支持管理员临时切换为其他用户身份

**MCP 工具桥接扩展**
- R6. 基于端点清单自动生成机制，将 `mcp_server` 的 `ROUTE_DEFS` 从 ~100 个扩展至覆盖全部 7624 个端点
- R7. 每个 MCP 工具必须包含：工具名称（`jaxrs_{crate}_{action}` 命名约定）、HTTP 方法、路径、描述、路径参数列表、body 参数列表
- R8. 区分需认证的端点和公开端点，在工具元数据中标注 `requires_auth` 字段

**OpenAPI 文档完善**
- R9. 将 `openapi` crate 的占位路径从 14 个扩展至覆盖全部已实现端点（~2458 个）
- R10. 每个 OpenAPI path item 必须包含：tag（模块名）、summary（端点描述）、parameters（路径参数和 query 参数）、responses（成功和错误响应结构）
- R11. 保持 `#[derive(OpenApi)]` 宏的编译通过，确保 `/openapi.json` 端点可正常生成规范

**行为对比测试全覆盖**
- R12. 将 `tests/behavior_compare_endpoints.rs` 的端点清单扩展至覆盖全部 7624 个端点（含新增认证安全端点）
- R13. 每个端点必须包含：`crate_name`、`method`、`rust_path`、`java_war`、`java_action`、`body`（可选）、`requires_auth` 字段
- R14. 验证 Java 服务不可用时，全部端点标记为 SKIP 而非 FAIL，测试套件整体通过

**业务功能补全**
- R15. 补全用户注册功能（对应 Java `x_organization_assemble_personal/regist/`），包含验证码发送、用户名/手机/邮箱唯一性校验、密码设置
- R16. 补全电子签名管理功能（对应 Java `x_organization_assemble_personal/signature/`），包含签名上传、列表查询、删除
- R17. 补全个人头像上传功能（对应 Java `x_organization_assemble_personal/icon/`），支持图片格式校验和大小限制
- R18. 补全微信小程序登录（对应 Java `mpweixin/`）和 WeLink 登录（对应 Java `welink/`）端点

**todo crate 清理**
- R19. 调查 `calendar`、`process_express`、`process_surface`、`mcp_server`、`openapi`、`shared` 的 todo 标记原因，确认为扫描口径问题则更新清单，确认为真实缺失则补全
- R20. 确保 `cargo test --workspace --lib` 在所有补全后全部通过

---

## Acceptance Examples

- AE1. **Covers R1, R2, R3, R4, R5.** 向 `/jaxrs/authentication/two/factory/login` 发送有效双因素登录请求，返回成功会话 token；向 `/jaxrs/authentication/sso/client/{client}/token/{token}` 发送有效 SSO token，返回成功会话；向 `/jaxrs/authentication/safe/logout` 发送安全注销请求后，该用户所有 session 均失效。
- AE2. **Covers R6, R7, R8.** 启动 oa4rust 并调用 MCP `tools/list`，返回工具数量从 ~100 扩展至覆盖全部已实现端点；每个工具元数据包含 `name`、`description`、`inputSchema`（含 `pathParams` 和 `bodyParams`）、`requiresAuth` 字段。
- AE3. **Covers R9, R10, R11.** 访问 `/openapi.json` 端点，返回的 OpenAPI 规范包含 ~2458 个 path item，每个 path item 有 tag、summary、parameters 和 responses；`cargo build` 无编译错误。
- AE4. **Covers R12, R13, R14.** 运行 `cargo test --test behavior_compare`，7624 个端点全部出现在测试清单中；Java 服务不可用时全部标记为 SKIP，测试套件通过。
- AE5. **Covers R15, R16, R17.** 向注册端点发送有效请求体创建新用户，返回成功；向签名端点上传签名图片，返回签名 ID；向头像上传端点上传图片，返回头像 URL。
- AE6. **Covers R19, R20.** 运行 `cargo test --workspace --lib` 全部通过；`docs/brainstorms/oa4rust-endpoint-inventory.md` 中不再有任何 todo 标记的 crate（除明确规划后续实现的新功能外）。

---

## Success Criteria

- **业务结果：** 认证安全模块端点对齐度达到 80%+（核心缺口全部补全），MCP 工具覆盖率达到 60%+，行为对比测试覆盖 100% 端点
- **质量结果：** `cargo test --workspace --lib` 全部通过，`/openapi.json` 可正常生成完整规范
- **可维护性结果：** 新增端点实现时遵循统一的模板，MCP 工具和 OpenAPI 路径可通过脚本自动生成，新开发者可参考 `docs/brainstorms/oa4rust-endpoint-inventory.md` 快速定位缺口

---

## Scope Boundaries

- **包含：** 认证安全模块补全（双因素登录、SSO、安全注销、Token 校验、用户切换）；MCP 工具桥接扩展；OpenAPI 文档完善；行为对比测试全覆盖；用户注册、电子签名、头像上传、微信小程序/WeLink 登录补全；todo crate 标记清理
- **排除在外：** 前端 o2web 代码修改；Java 后端代码修改；文件存储层迁移；定时任务/批处理框架的完整 Rust 重写；数据库连接池性能优化；微服务拆分

### Deferred for later

- 政务钉钉登录（zhengwudingding）—— 特定客户场景，非通用需求
- 批量操作端点（如批量删除、批量导入）—— 低频使用场景
- 流程平台深度功能（processplatform 的复杂编排端点）—— 需要单独评估
- SQLx 完全移除（ORM 为默认路径，复杂查询可保留 SQLx 并存）

### Outside this product's identity

- 前端 o2web 的重写或现代化改造（这是 OA 前端，不是 oa4rust 的职责）
- 独立的 OAuth 提供商 SDK 发布（MCP 工具桥接为 oa4rust 内部功能，不对外发布）
- Java 服务的永久下线（oa4rust 是替代方案，但下线决策属于运维范畴）

---

## Key Decisions

- **双因素登录使用短信验证码：** 沿用现有 `auth` crate 的 `CodeStore` 短信验证码流程，第二因子验证与首次登录验证码流程复用，无需引入 TOTP 依赖。
- **SSO token 兼容 Java 3DES 格式：** Rust 侧实现 3DES 加解密逻辑，token 格式为 `加密(credential#timestamp)`，与 Java 端互通。
- **用户切换复用现有角色体系：** 不新增 `is_admin` 字段，通过 `auth_role` 角色体系判断管理员权限，已有 `auth_person` ↔ `auth_person_role` 关联可支持。
- **MCP 和 OpenAPI 通过自动生成机制扩展：** 手动维护 7624 个端点的工具定义不现实，需要基于端点清单自动生成
- **行为对比测试全覆盖作为质量门禁：** 100% 覆盖率确保新增端点不会引入行为回归，Java 不可用时的 SKIP 降级策略已验证可行
- **todo crate 标记先调查后处理：** 部分 todo 可能是扫描口径问题（如 `calendar` 有完整实现但被标记 todo），需要先确认再决定补全方向

---

## Dependencies / Assumptions

- Java `x_organization_assemble_authentication` 和 `x_organization_assemble_personal` 的端点清单可作为 Rust 实现的参考契约
- 前端 o2web 对 `ActionResult<T>` 的 9 字段结构有隐式依赖，新增端点必须保持兼容
- 行为对比测试的 Java 服务可能不可用，框架需支持降级为 SKIP 模式
- `mcp_server` 的 `ROUTE_DEFS` 静态数组可通过脚本自动生成，避免手动维护 7624 条注册

---

## Outstanding Questions

### Resolve Before Planning

（所有阻塞问题已解决）

- ~~[Affects R1] 双因素登录的具体实现方式~~ → **已决策：** 使用短信验证码，复用现有 CodeStore 流程
- ~~[Affects R2] SSO token 的加密格式~~ → **已决策：** 兼容 Java 3DES 格式，Rust 侧实现加解密
- ~~[Affects R5] 用户切换的权限模型~~ → **已决策：** 复用现有 auth_role 角色体系，不新增字段

### Deferred to Planning

- [Affects R6][Needs research] MCP 工具桥接的自动生成脚本：是否可以在 `scripts/` 下编写一个基于端点清单的生成器
- [Affects R9][Needs research] OpenAPI 路径生成的复杂度：2458 个端点的 `utoipa` derive 宏是否能正常编译（可能存在宏展开超时）
- [Affects R15][Technical] 用户注册的用户名/手机/邮箱唯一性校验是否需要在数据库层添加唯一约束
- [Affects R19][Needs research] todo crate 标记的真实原因：需要运行 `scripts/gen_inventory.py` 重新扫描后确认
