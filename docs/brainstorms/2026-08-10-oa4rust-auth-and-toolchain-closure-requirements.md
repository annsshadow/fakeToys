---
date: 2026-08-10
topic: oa4rust-auth-and-toolchain-closure
---

# OA4Rust 认证提供方补全与工具链扩展

## Summary

补全 oa4rust 与 Java OA 在认证安全模块和工具链基础设施三个维度的差距：新增 5 个第三方 OAuth 提供方（微信小程序、WeLink、政务钉钉、移动办公 SSO、企业微信点单）、SSO GET 端点，同时扩展 MCP 工具桥接、OpenAPI 文档和行为对比测试至全量覆盖，使 oa4rust 达到可替代 Java OA 的生产就绪状态。

---

## Problem Frame

2026-08-10 的差距审计文档已大部分完成（双因素登录、SSO POST、安全注销、Token 校验、用户切换、用户注册、头像上传、OpenAPI 自动生成、program_center 写操作、计划清理等均已实现），`auth` crate 已包含 12 个源文件，认证模块核心功能基本完备。

但仍有结构性缺口：

**认证提供方覆盖不足。** 现有 `auth/oauth.rs` 仅实现企业微信和钉钉两个提供方，Java 端还有微信小程序（4 端点）、WeLink（1 端点）、政务钉钉（2 端点）、移动办公 MOA SSO（1 端点）、企业微信点单登录（3 端点）等 5 个提供方完全缺失。这些提供方面向不同用户场景（公众号用户、华为 WeLink 企业用户、政府机构、移动端 APP），缺少任何一个都会导致对应用户群体无法登录。

**SSO GET 端点缺失。** Java 提供 `sso/client/{client}/token/{token}` GET 端点供浏览器重定向场景使用（与 POST 端点互补），Rust 仅有 POST 方式，前端无法从浏览器 URL 直接触发 SSO 登录。

**数据库 schema 缺少 OAuth 字段。** `auth_person` 表缺少 `mpwxopenId` 字段（微信小程序 openid），新提供方登录后无法将第三方账号与本地用户关联。

**工具链覆盖率极低。** MCP 工具桥接仅注册约 100 个工具（1.3%），OpenAPI 文档已生成 757 条路径但缺少 path_params 推断，行为对比测试仅覆盖 79/7624 端点（1%），无法为"代替"提供质量信心。

---

## Actors

- **A1（开发者，单人）：** 负责认证提供方实现、数据库迁移、工具链扩展
- **A2（前端 o2web）：** 依赖新增端点的 `ActionResult<T>` 9 字段结构，前端需感知新提供方登录入口
- **A3（AI Agent / MCP 客户端）：** 通过 MCP 工具桥接调用 oa4rust 功能，需要完整的工具清单
- **A4（下游规划 Agent）：** 读取本需求文档后执行 ce-plan 进行详细规划

---

## Key Flows

- **F1. 第三方 OAuth 登录流程**
  - **Trigger：** 用户从微信小程序/WeLink/政务钉钉/移动办公/企业微信点单入口发起登录
  - **Actors：** A1, A2
  - **Steps：**
    1. 前端携带第三方平台返回的 code 调 oa4rust 对应端点
    2. oa4rust 用 code 换取第三方平台用户标识（openid/userId/dingUserId）
    3. 按约定前缀（`mpwx_`/`welink_`/`zwding_`/`andfx_`）查询 auth_person
    4. 用户存在则创建会话返回 token；不存在则返回 unbind=true（需前端引导绑定）
  - **Outcome：** 用户成功通过第三方平台登录 oa4rust，返回完整会话 token
  - **Covered by：** R1-R7

- **F2. SSO 浏览器重定向登录流程**
  - **Trigger：** 外部系统通过浏览器 URL 重定向触发 SSO 登录
  - **Actors：** A1
  - **Steps：**
    1. 外部系统构造 GET URL：`/jaxrs/authentication/sso/client/{client}/token/{token}`
    2. oa4rust 解密 token（3DES EDE2），提取 credential
    3. 验证时效性（5 分钟内），查询用户，签发会话
  - **Outcome：** 浏览器重定向后用户自动登录，token 写入 cookie
  - **Covered by：** R8

- **F3. MCP/OpenAPI/测试全覆盖流程**
  - **Trigger：** 新增端点后需要工具链和测试覆盖
  - **Actors：** A1, A3
  - **Steps：**
    1. 运行 `scripts/gen_mcp_tools.py` 基于端点清单自动生成 `mcp_server/generated_routes.rs`
    2. 运行 `scripts/gen_openapi_paths.py` 扩展 OpenAPI 至全部已实现端点
    3. 运行 `scripts/gen_behavior_compare.py` 生成全量 7624 端点的行为对比测试清单
  - **Outcome：** MCP 工具覆盖率 100%，OpenAPI 文档完整，行为对比测试覆盖全量端点
  - **Covered by：** R9-R13

---

## Requirements

**第三方 OAuth 提供方实现**
- R1. 实现微信小程序登录模块（`auth/src/mpweixin.rs`），包含 4 个端点：`GET /jaxrs/mpweixin/login/code/{code}`（code→openid→登录或返回 unbind）、`GET /jaxrs/mpweixin/bind/code/{code}`（code→openid→绑定到当前登录用户）、`GET /jaxrs/mpweixin/bind/openid/{openid}`（直接绑定 openid）、`POST /jaxrs/mpweixin/menu/test/send/to/{person}`（管理员发送模板消息，admin only）
- R2. 实现 WeLink 登录模块（`auth/src/welink.rs`），包含 1 个端点：`GET /jaxrs/welink/code/{code}`（code→华为云 userId→登录，unique_id 前缀 `welink_`）
- R3. 实现政务钉钉登录模块（`auth/src/zhengwudingding.rs`），包含 2 个端点：`GET /jaxrs/zhengwudingding/code/{code}`（code→dingUserId→userId 两步映射→登录，unique_id 前缀 `zwding_`）、`GET /jaxrs/zhengwudingding/info`（获取政务钉钉配置状态）
- R4. 实现移动办公 SSO 模块（`auth/src/andfx.rs`），包含 1 个端点：`GET /jaxrs/andfx/moa/sso/token/{token}/enter/{enterId}`（token 解析→enterId 校验→登录，unique_id 前缀 `andfx_`）
- R5. 实现企业微信点单登录模块（`auth/src/qiyeweixin.rs`），包含 3 个端点：`GET /jaxrs/qiyeweixin/code/{code}`（code 登录）、`GET /jaxrs/qiyeweixin/update/person/detail/{code}`（登录并同步用户详细信息）、`POST /jaxrs/qiyeweixin/jssdk/sign/info`（JSSDK 签名，SHA1(jsapi_ticket+noncestr+timestamp+url)）
- R6. 所有 OAuth 提供方遵循现有 `auth/oauth.rs` 的模式：环境变量配置 AppKey/AppSecret，`unique_id` 约定前缀绑定第三方账号，未绑定用户返回 `unbind=true` 引导前端绑定
- R7. 为每个新提供方模块添加单元测试，覆盖：code 换取用户标识、用户不存在返回 unbind、用户已存在返回 token、无效 code 返回错误

**数据库 schema 补全**
- R8. 创建 migration 脚本（`migrations/013_add_oauth_fields.sql`），为 `auth_person` 表新增 `mpwxopenId VARCHAR(255)` 字段，用于存储微信小程序 openid，与 Java 实体 `Person.mpweixinOpenId_FIELDNAME` 对齐

**SSO GET 端点**
- R9. 在 `auth/src/sso.rs` 新增 `GET /jaxrs/authentication/sso/client/{client}/token/{token}` 端点，与现有 POST 端点共享 3DES 解密逻辑，支持浏览器 URL 重定向场景，解密后流程与 POST 完全一致

**MCP 工具桥接扩展**
- R10. 扩展 `scripts/gen_mcp_tools.py`，基于 `crates/*/src/lib.rs` 的路由注册自动生成 `mcp_server/src/generated_routes.rs`，覆盖全部 7624 个端点，每个工具包含：工具名称（`jaxrs_{crate}_{action}` 命名）、HTTP 方法、路径、描述、pathParams、bodyParams、requiresAuth
- R11. `mcp_server/src/lib.rs` 加载 `generated_routes.rs`，工具清单从 ~100 扩展至覆盖全部 7624 端点

**OpenAPI 文档完善**
- R12. 扩展 `scripts/gen_openapi_paths.py`，在现有 757 条路径基础上，覆盖全部 2458 个已实现 handler 的路径注解，确保 `cargo check` 通过（若 utoipa derive 宏处理过多路径导致编译超时，则分批生成）

**行为对比测试全覆盖**
- R13. 扩展 `tests/behavior_compare_endpoints.rs`，端点清单从 ~79 扩展至覆盖全部 7624 个端点（含新增 OAuth 端点），每个端点包含 `crate_name`、`method`、`rust_path`、`java_war`、`java_action`、`body`（可选）、`requires_auth` 字段；Java 不可用时全部标记 SKIP 而非 FAIL

---

## Acceptance Examples

- AE1. **Covers R1, R2, R3, R4, R5.** 向 `/jaxrs/mpweixin/login/code/{code}` 发送有效微信小程序 code，返回包含 `token` 和 `person` 的 ActionResult；未绑定的 openid 返回 `unbind=true` 和 `mpwxopenId`；向 `/jaxrs/welink/code/{code}` 发送有效 WeLink code，返回成功会话 token；向 `/jaxrs/zhengwudingding/code/{code}` 发送有效政务钉钉 code，返回成功会话 token；向 `/jaxrs/andfx/moa/sso/token/{token}/enter/{enterId}` 发送有效 token，返回成功会话；向 `/jaxrs/qiyeweixin/code/{code}` 发送有效企业微信扫码 code，返回成功会话；向 `/jaxrs/qiyeweixin/jssdk/sign/info` 发送 URL 和 nonceStr，返回有效的 SHA1 签名。
- AE2. **Covers R9.** 向 `GET /jaxrs/authentication/sso/client/{client}/token/{token}` 发送有效的 3DES 加密 token，返回成功会话 token 和用户信息；token 过期（超过 5 分钟）返回 400 错误。
- AE3. **Covers R8.** 运行 `psql` 后执行 `\d auth_person`，确认存在 `mpwxopenId` 列；运行 `cargo test --workspace --lib` 全部通过。
- AE4. **Covers R10, R11.** 启动 oa4rust 并调用 MCP `tools/list`，返回工具数量覆盖全部已实现端点（~2458+）；每个工具元数据包含 `name`、`description`、`inputSchema`（含 `pathParams` 和 `bodyParams`）、`requiresAuth` 字段。
- AE5. **Covers R12.** 访问 `/openapi.json` 端点，返回的 OpenAPI 规范包含全部已实现端点的 path item，每个 path item 有 `tag`、`summary`、`parameters`（path/body）、`responses`；`cargo check --workspace` 无编译错误。
- AE6. **Covers R13.** 运行 `cargo test --test behavior_compare`，7624 个端点全部出现在测试清单中；Java 服务不可用时全部标记为 SKIP，测试套件通过。

---

## Success Criteria

- **业务结果：** 5 个第三方 OAuth 提供方全部可用，SSO GET 端点可用，MCP 工具覆盖率 ≥ 60%，行为对比测试覆盖 100% 端点
- **质量结果：** `cargo test --workspace --lib` 全部通过，`cargo check --workspace` 无新增 error，`/openapi.json` 可正常生成完整规范
- **可维护性结果：** 新增 OAuth 提供方遵循统一模板，MCP 工具和 OpenAPI 路径可通过脚本自动生成

---

## Scope Boundaries

- **包含：** 5 个第三方 OAuth 提供方实现（mpweixin、welink、zhengwudingding、andfx、qiyeweixin）；SSO GET 端点；`auth_person.mpwxopenId` 数据库迁移；MCP 工具桥接扩展；OpenAPI 文档完善；behavior_compare 测试全量覆盖
- **排除在外：** 电子签名管理（signature，低优先级， deferred）；console 模块完整实现（全部桩代码，低频使用）；AI 模块 MCP/文件/索引 stub 补全（AI 功能本身未完成）；政务钉钉 GET 方式详细实现（仅需 login 和 info 两个端点）

### Deferred for later

- 电子签名管理（`personal/signature/`）—— 低频使用场景，不在当前迭代
- console 模块完整实现 —— 全部桩代码，低频使用
- AI 模块 MCP/文件/索引端点补全 —— AI 功能框架本身未完成
- SQLx 完全移除 —— 低优先级，ORM 为默认路径
- 批量操作端点 —— 低频使用场景

### Outside this product's identity

- 前端 o2web 的重写或现代化改造（这是 OA 前端，不是 oa4rust 的职责）
- 独立的 OAuth 提供商 SDK 发布（MCP 工具桥接为 oa4rust 内部功能）
- Java 服务的永久下线（运维范畴）

---

## Key Decisions

- **OAuth 登录返回完整用户信息：** 与 Java 端对齐，返回 name、mobile、email、roles 等完整 Person 信息 + token，而非仅 token+unique
- **SSO client→key 映射存数据库：** 新建 `sso_client` 表存储 client 名称与 3DES key 的映射，与 Java `Config.token().findSso(client)` 逻辑对齐
- **OAuth 模块拆分策略：** 每个提供方独立为 `auth/src/{provider}.rs` 子模块，与现有 `auth/src/oauth.rs`（qywx/dingding）保持一致，通过 `lib.rs` 的 `pub mod` 导出，避免在单一 oauth.rs 中无限增长
- **unique_id 前缀约定：** 微信小程序用 `mpwx_{openid}`，WeLink 用 `welink_{userId}`，政务钉钉用 `zwding_{dingUserId}`，移动办公用 `andfx_{token}`，与现有 `qywx_`/`dingding_` 一致
- **mpwxopenId 字段策略：** 不新增独立表，直接在 `auth_person` 新增 `mpwxopenId VARCHAR(255)` 列，与 Java 实体对齐，migration 脚本追加即可
- **SSO GET/POST 共享解密逻辑：** GET 和 POST 端点共用 `decrypt_sso_token()` 内部函数，差异仅在 HTTP 方法提取参数方式（path vs body）
- **MCP 和 OpenAPI 通过自动生成机制扩展：** 手动维护全量端点不现实，基于 `scripts/gen_mcp_tools.py` 和 `scripts/gen_openapi_paths.py` 自动生成，与现有 OpenAPI 757 条路径的生成模式一致
- **行为对比测试全量覆盖作为质量门禁：** 100% 覆盖率确保新增端点不会引入行为回归，Java 不可用时的 SKIP 降级策略已验证可行

---

## Dependencies / Assumptions

- Java `mpweixin`/`welink`/`zhengwudingding`/`andfx`/`qiyeweixin` 的端点清单可作为 Rust 实现的参考契约
- 前端 o2web 对 `ActionResult<T>` 的 9 字段结构有隐式依赖，新增端点必须保持兼容
- 行为对比测试的 Java 服务可能不可用，框架需支持降级为 SKIP 模式
- `auth_person` 表现有字段足以支持第三方 OAuth 登录（`unique_id` 用前缀约定绑定第三方账号）
- 微信小程序/WeLink/政务钉钉/移动办公的企业配置（AppKey/AppSecret）通过环境变量注入，不硬编码
- `sso_client` 表不存在，需新建 migration 脚本创建该表

---

## Outstanding Questions

### Resolve Before Planning

- ~~[Affects R7][Technical] 微信小程序和 WeLink 登录时，是否需要返回与 Java 一致的完整用户信息~~ → **已决策：** 返回完整用户信息（name、mobile、email、roles 等）+ token，与 Java 端对齐
- ~~[Affects R9][Technical] SSO GET 端点的 3DES key 来源~~ → **已决策：** 从数据库 `sso_client` 表读取 client→key 映射，需新建表和 migration

### Deferred to Planning

- [Affects R10][Needs research] MCP 工具生成的命名冲突处理：7624 个端点中可能有路径相似的端点，`tool_name` 需唯一，需确认生成脚本的命名去重策略
- [Affects R12][Needs research] utoipa derive 宏处理 ~2458 个路径时是否会导致编译超时——需实际验证，若超时则降级为分批生成
- [Affects R13][Needs research] behavior_compare 测试清单的自动生成脚本：是否可复用 `scripts/gen_behavior_compare.py` 或需新增
