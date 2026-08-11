---
date: 2026-08-11
topic: oa4rust-full-gap-closure
---

# OA4Rust 全面差距补全 — 业务逻辑对齐与功能补全

## Summary

修复认证核心业务逻辑差距（登录安全检查、双因素两阶段流程、Token 校验鉴权、安全注销广播）、补全个人模块缺失端点（电子签名、头像）、修复 null 桩、对齐响应结构，并新增 LDAP 集成、批量查询端点和授权管理 CRUD，使 oa4rust 在业务逻辑和端点覆盖上达到与 Java OA 100% 对齐。

---

## Problem Frame

oa4rust 已完成 83 个 crate 的真实化和 2510 个真实 handler，但 2026-08-11 的深度审计揭示：端点路由层面的"存在"不等于业务逻辑的对齐。认证模块的登录流程缺少 locked/banned/passwordExpired 安全检查，双因素登录采用单请求合并模式而非 Java 的两阶段流程，check_token 端点缺乏管理员鉴权且返回结构不符，safe_logout 缺失多实例广播，switch_user 响应字段残缺。个人模块完全缺失电子签名和头像端点。correlation 和 hotpic 模块的 delete handler 返回 `Value::Null` 而非符合 Java 契约的对象。PersonInfo 仅有 3 个字段而 Java Person 有 10+ 字段。LDAP 集成和批量查询功能完全缺失。这些差距导致前端 o2web 无法正常联调，外部系统集成时行为不一致。

---

## Actors

- A1. **开发者**：实现所有差距补全
- A2. **前端 o2web**：依赖 `ActionResult<T>` 9 字段结构和完整 Person 响应，任何字段缺失会导致渲染异常
- A3. **AI Agent / MCP 客户端**：通过 MCP 工具桥接调用 oa4rust，需要端点行为与 Java 一致
- A4. **CI 流水线**：`cargo test --workspace` 验证无回归

---

## Key Flows

- **F1. 登录认证流程（密码登录）**
  - **Trigger：** 用户提交 credential + password
  - **Actors：** A2
  - **Steps：**
    1. 校验 credential 和 password 非空
    2. 检查 locked 状态 → 锁定用户返回 locked 错误
    3. 检查 passwordExpired → 首次登录/密码过期返回 passwordExpired=true
    4. 校验密码（bcrypt/MD5/DES 兼容）
    5. 签发会话 token，返回完整 PersonInfo + tokenType + roleList + passwordExpired
  - **Outcome：** 与 Java ActionLogin 行为完全一致
  - **Covered by：** R1, R2, R3, R10

- **F2. 双因素登录流程（两阶段）**
  - **Trigger：** 用户提交 credential + password（第一因子）
  - **Actors：** A2
  - **Steps：**
    1. 第一因子验证：credential + password（同登录流程）
    2. 验证通过 → 发送短信验证码，返回 `value=true, passwordExpired=...`
    3. 客户端用 credential + codeAnswer 调 codeLogin
    4. 第二因子验证：短信验证码
    5. 验证通过 → 签发会话 token，返回完整 PersonInfo
  - **Outcome：** 两阶段流程与 Java ActionTwoFactoryLogin + ActionCodeLogin 一致
  - **Covered by：** R4, R5

- **F3. 电子签名上传流程**
  - **Trigger：** 用户上传签名图片
  - **Actors：** A2
  - **Steps：**
    1. multipart/form-data 接收图片字节
    2. Base64 编码后存入 custom 表（name=CUSTOM_SIGNATURE_NAME, person=当前用户）
    3. 返回上传成功确认
  - **Outcome：** 与 Java ActionUpload 行为一致
  - **Covered by：** R6

---

## Requirements

**认证核心业务逻辑修复**
- R1. 登录端点（POST /jaxrs/authentication）必须检查用户 locked 状态（返回 locked 错误）和 passwordExpired（首次登录或未修改过密码且 Config.firstLoginModifyPwd=true 时返回 passwordExpired=true），与 Java ActionLogin 行为一致
- R2. 登录响应结构必须扩展为与 Java AbstractWoAuthentication 一致的字段：token, tokenType, roleList, passwordExpired, identityList, 以及完整 Person 字段（id, unique, name, mobile, email, icon, job, department, unit, position）
- R3. check_token 端点（POST /jaxrs/authentication/check/token）必须增加管理员权限校验（isManager），返回 token 持有者的 distinguishedName 字符串（而非 {authenticated: true/false}）
- R4. 双因素登录拆分为两阶段：第一阶段 POST /jaxrs/authentication/two/factory/login 验证密码后发送短信验证码并返回 value=true + passwordExpired；第二阶段 POST /jaxrs/authentication/code 验证 credential + codeAnswer 并签发 token
- R5. safe_logout 端点（POST /jaxrs/authentication/safe/logout）必须写入 TokenThreshold 实体记录当前时间戳，并在多实例场景下广播更新（单实例可跳过广播）
- R6. switch_user 响应必须补全 tokenType, roleList, passwordExpired 字段，与 Java ActionSwitchUser 的 AbstractWoAuthentication 返回结构一致

**个人模块端点补全**
- R7. 补全电子签名端点：POST /jaxrs/person/signature/upload（multipart，Base64 存 PostgreSQL custom 表）、GET /jaxrs/person/signature/list（当前用户签名列表）、GET /jaxrs/person/signature/delete/{id}（软删除）；管理员可用 GET /jaxrs/person/signature/manager/list 查看所有用户签名
- R8. 补全头像端点：GET /jaxrs/person/icon/{person}（无权限也可访问，返回该用户头像信息）、POST /jaxrs/person/icon/upload（multipart，存储为 base64 到 auth_person.icon 字段）
- R9. 用户注册端点（POST /jaxrs/person/regist）必须校验验证码（复用 ResetCodeStore）并检查用户名/手机/邮箱唯一性，返回与 Java ActionCreate 一致的响应结构

**Null 桩修复**
- R10. correlation_core_entity delete 端点（DELETE /jaxrs/correlation/core/entity/delete/{id}）返回 `ActionResult::success(json!({"success": true}))` 而非 `Value::Null`
- R11. hotpic_core_entity delete 端点（DELETE /jaxrs/hotpic/core/entity/delete/{id}）同上；hotpic list 端点返回的 data 数组必须包含 base64 字段
- R12. 修复后 `cargo test -p correlation_core_entity` 和 `cargo test -p hotpic_core_entity` 全部通过

**LDAP 集成**
- R13. 新增 LDAP 认证模块：通过环境变量 `LDAP_URL` / `LDAP_BASE_DN` / `LDAP_BIND_USER` / `LDAP_BIND_PWD` / `LDAP_ENABLE` 配置，默认关闭
- R14. 登录时若 LDAP_ENABLE=true 且 LDAP 认证成功，直接签发会话；LDAP 认证失败时回退到数据库密码校验
- R15. LDAP 使用简单绑定（simple bind）方式，连接超时 3 秒，失败不阻塞主流程
- R16. Cargo.toml 新增 `ldappress` 或等效 LDAP crate 依赖

**批量查询端点**
- R17. 批量查询人员（POST /jaxrs/express/person/list）：接受 `{"ids":["id1","id2"]}` 或 `{"identities":["id1","id2"]}`，返回完整 Person 对象列表
- R18. 批量查询组织单位（POST /jaxrs/express/unit/list）：接受单位 ID 列表，返回完整 Unit 对象列表
- R19. 批量查询身份（POST /jaxrs/express/identity/list）：接受身份 ID 列表，返回完整 Identity 对象列表
- R20. 批量查询群组（POST /jaxrs/express/group/list）：接受群组 ID 列表，返回完整 Group 对象列表
- R21. 批量查询角色（POST /jaxrs/express/role/list）：接受角色 ID 列表，返回完整 Role 对象列表
- R22. 批量查询人员所在组织（POST /jaxrs/express/person/with/unit）：接受人员 ID 列表，返回每个人员所属组织信息
- R23. 批量查询人员所在身份（POST /jaxrs/express/person/with/identity）：接受人员 ID 列表，返回每个人员的所有身份
- R24. 以上批量查询端点无需认证（express 模块特性，与 Java 一致）

**授权管理（Empower）**
- R25. 补全授权管理 CRUD：POST /jaxrs/person/empower（创建授权）、GET /jaxrs/person/empower/{id}（查询授权）、PUT /jaxrs/person/empower/{id}（更新授权）、DELETE /jaxrs/person/empower/{id}（删除授权）、GET /jaxrs/person/empower/{id}/enable（启用）、GET /jaxrs/person/empower/{id}/disable（禁用）
- R26. 管理员端点：POST /jaxrs/person/empower/manager（管理员创建）、PUT /jaxrs/person/empower/manager/{id}（管理员更新）、DELETE /jaxrs/person/empower/manager/{id}（管理员删除）、POST /jaxrs/person/empower/manager/list/paging/{page}/size/{size}（管理员分页查询）
- R27. 查询当前用户授权：GET /jaxrs/person/empower/list/currentperson（我的授权）、GET /jaxrs/person/empower/list/currentperson/enable（我的生效授权）、GET /jaxrs/person/empower/list/to（我拥有的被授权）、GET /jaxrs/person/empower/list/to/enable（我生效的被授权）
- R28. 权限控制：管理员可管理他人授权，普通用户只能管理自身授权

---

## Acceptance Examples

- AE1. **Covers R1, R2, R10.** Given 用户处于 locked 状态，当 POST /jaxrs/authentication 发送正确密码时，返回 locked 错误而非正常登录；Given 用户首次登录且密码未过期，响应包含 `passwordExpired: false` 和完整 Person 字段（id, unique, name, mobile, email, icon, job, department, unit, position, token, tokenType, roleList）。
- AE2. **Covers R4, R5.** Given 双因素登录已启用，当 POST /jaxrs/authentication/two/factory/login 发送正确的 credential+password 时，返回 `{value: true, passwordExpired: false}` 并发送短信验证码；当 POST /jaxrs/authentication/code 发送正确 credential+codeAnswer 时，签发会话并返回完整 token+Person。
- AE3. **Covers R3.** Given 普通用户调用 POST /jaxrs/authentication/check/token，返回 403；Given 管理员调用，返回 token 持有者的 distinguishedName 字符串。
- AE4. **Covers R6.** Given 管理员调用 POST /jaxrs/authentication/switchuser 切换为其他用户，响应包含 token, tokenType, roleList, passwordExpired 和完整 PersonInfo。
- AE5. **Covers R7.** Given 已登录用户 POST /jaxrs/person/signature/upload 上传签名图片，签名 Base64 存入 custom 表；GET /jaxrs/person/signature/list 返回该用户所有签名。
- AE6. **Covers R8.** Given 任何用户（无需认证）GET /jaxrs/person/icon/{person}，返回该用户的头像信息；Given 已登录用户 POST /jaxrs/person/icon/upload 上传头像，auth_person.icon 字段更新。
- AE7. **Covers R11, R12.** Given DELETE /jaxrs/correlation/core/entity/delete/{id}，返回 `{success: true}` 而非 null；Given GET /jaxrs/hotpic/core/entity/list，返回的 data 数组每条包含 base64 字段。
- AE8. **Covers R13, R14.** Given LDAP_ENABLE=true 且 LDAP_URL 已配置，当登录时 LDAP 认证成功，直接签发会话；LDAP 认证失败时回退到数据库密码校验。
- AE9. **Covers R17.** Given POST /jaxrs/express/person/list 发送 `{"ids":["id1","id2"]}`，返回包含完整 Person 字段的列表。
- AE10. **Covers R25, R26, R27.** Given 管理员 POST /jaxrs/person/empower/manager 创建授权，授权写入数据库；GET /jaxrs/person/empower/list/currentperson 返回当前用户的授权列表。

---

## Success Criteria

- `cargo check --workspace` 通过，无新增编译错误
- `cargo test --workspace` 通过，无新增失败
- 登录流程：locked 检查、passwordExpired 检查、完整 Person 响应字段全部生效
- 双因素登录：两阶段流程（sendCode + codeLogin）与 Java 行为一致
- check_token：管理员鉴权生效，响应结构为 distinguishedName 字符串
- 电子签名和头像端点：上传、列表、删除、读取全部可用
- LDAP 集成：环境变量控制开关，认证失败时正确回退
- 批量查询：5 个主体（person/unit/identity/group/role）的核心批量查询端点可用
- 授权管理：CRUD 和权限控制全部生效
- `docs/brainstorms/oa4rust-endpoint-inventory.md` 中不再有任何 todo 或 doing 标记（除已确认后续实现的新功能外）

---

## Scope Boundaries

- 仅补全 oa4rust 与 Java OA 的业务逻辑差距，不修改 Java 端代码
- 前端 o2web 的修改不在范围内
- LDAP 仅用于认证，不实现 LDAP 用户自动同步
- 批量查询端点仅实现核心的 list 变体（按 ID 列表查询），不实现深度递归查询（如 unit sub-nested/sup-nested 全量递归）
- 授权管理仅实现 personal 模块下的 empower 端点
- 缓存层（Java CacheManager）暂不迁移

### Deferred for later

- 多级递归组织导航（unit sub-nested/sup-nested 全量递归）
- LDAP 用户自动同步和增量更新
- 缓存层性能优化
- 流程平台深度功能（processplatform 复杂编排端点）
- SQLx 完全移除（SeaORM 为默认路径）

### Outside this product's identity

- 前端 o2web 的重写或现代化改造
- 独立的 OAuth 提供商 SDK 发布
- Java 服务的永久下线决策

---

## Key Decisions

- **双因素登录两阶段拆分**：Java 先发码（value=true）再验码（codeLogin），Rust 当前单请求模式不符合前端契约，必须拆分为两个独立端点
- **LDAP 可选集成**：通过环境变量开关控制，默认关闭，认证失败时静默回退数据库密码，不阻塞主流程
- **批量查询无认证**：express 模块批量查询与 Java 一致，不需要认证（前端内部调用场景）
- **授权管理复用现有角色体系**：管理员判断通过 auth_role 角色体系，不新增字段
- **电子签名和头像 BLOB 存储**：与 Java 一致使用 PostgreSQL BLOB（base64 字符串），不改存文件系统

---

## Dependencies / Assumptions

- Java `AbstractWoAuthentication` 的字段结构（token, tokenType, roleList, passwordExpired, identityList, 以及 Person 全字段）是前端 o2web 的隐式依赖
- `custom` 表（x_custom）已存在，支持按 person + name 存储签名数据
- `auth_person` 表已有 icon 字段（现有代码已查询），无需 DDL 变更
- `ldappress` crate 与当前 Rust 1.75 工具链兼容
- 批量查询端点不涉及复杂的跨表关联，仅按 ID 列表查询主表

---

## Outstanding Questions

### Resolve Before Planning

（所有阻塞问题已在此 brainstorm 阶段解决，无 pending 阻塞问题）

### Deferred to Planning

- [Affects R1][Technical] passwordExpired 检查依赖 `change_password_time` 和 `password_expired_time` 字段是否存在于 auth_person 表——需实际验证 schema
- [Affects R5][Technical] TokenThreshold 实体是否已在数据库中创建——需确认 migrations
- [Affects R13][Needs research] `ldappress` vs `ldap3` vs `ldap-src` crate 选择——需验证 Rust 1.75 兼容性和 API 成熟度
- [Affects R17-R24][Needs research] 批量查询端点的实际路由路径——需对照 Java express 模块路由注册确认
- [Affects R25-R28][Needs research] empower 相关数据表（x_empower）的 schema 是否存在——需确认 migrations
