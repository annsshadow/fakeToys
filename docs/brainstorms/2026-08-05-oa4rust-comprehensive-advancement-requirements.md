---
date: 2026-08-05
topic: oa4rust-comprehensive-advancement
---

# oa4rust 全面推进：需求文档

## Summary

全面推进 oa4rust 后端 Rust 迁移：先将全部 80 个 crate 接入 `main.rs` 建立完整路由框架（桩代码端点标记 TODO），再并行填充核心业务端点的真实实现并完善认证模块的非占位功能。

---

## Problem Frame

O2OA 后端当前 100% 基于 Java（Maven 55+ 模块），长期技术栈锁定在 Java 生态。oa4rust 项目已将全部 55 个 Java 模块映射为 80 个 Rust crate，编译通过且测试通过，但只有 4 个 crate 接入运行时入口（`main.rs`）。其余 76 个 crate 已实现业务逻辑并经过测试，但未接入运行时，前端无法调用、无法验证行为一致性、无法追踪迁移进度。认证模块存在占位实现（验证码返回占位图、OAuth 返回示例 URL），CMS 控制模块仅返回空列表。团队无法在 Rust 后端上推进任何实际工作，迁移停滞的代价是持续维护 Java 运行时和技术栈锁定。

---

## Requirements

**路由框架接入**
- R1. 所有 80 个 workspace crate 分两阶段接入 `main.rs`：第一阶段接入已有真实实现的 crate（control、personal_extend 等），第二阶段接入其余桩代码 crate
- R2. 桩代码端点（返回占位数据或空列表）必须在代码中标记 `TODO: [module] - real implementation needed`，所有 TODO 标记在 sprint 规划中review并分配优先级
- R3. 已存在真实实现的 crate（control、personal_extend 等）优先接入，桩代码端点不得阻塞已实现端点的暴露

**核心业务端点真实化**
- R4. 组织控制模块（人员、单位、角色、用户组）的 CRUD 端点返回真实数据库数据
- R5. 个人信息模块（个人信息查询/更新、密码管理、头像管理）的端点返回真实数据
- R6. 系统初始化模块（密码设置、初始化检查）的端点返回真实数据

**认证模块完善**
- R7. 验证码端点返回真正的验证码图片（本地生成，集成 captcha 库）
- R8. OAuth 端点实现第三方登录对接（微信、钉钉）
- R9. 认证流程完整可用（登录 → 会话 → 登出 → 刷新令牌）

**安全需求**
- R12. 所有端点强制认证（登录用户方可访问），健康检查端点除外
- R13. 所有输入端点进行参数验证（类型、长度、格式），拒绝无效输入
- R14. 认证接口速率限制（10次/分钟/IP），普通接口速率限制（100次/分钟/IP）
- R15. 所有响应强制 HTTPS（TLS 1.2+），生产环境返回安全的响应头（HSTS, X-Content-Type-Options）

**迁移策略**
- R10. 沿用 Strangler Fig 渐进式迁移策略：Rust 与 Java 并行运行，通过 nginx 反向代理按 URL 前缀路由，逐步切换流量
- R11. 迁移进度通过 `docs/brainstorms/oa4rust-migration-status.md` 模块跟踪清单持续反映，每个模块标记为待迁移 / 迁移中 / 已完成

---

## Acceptance Examples

- AE1. **Covers R4, R5, R6.** Given a seeded database with test data, when a CRUD endpoint is called, it returns data matching the Java backend's response schema (field names, types, non-null constraints) rather than empty or placeholder values.
- AE2. **Covers R7.** Given a captcha request, when the endpoint is called, it returns a generated captcha image (not a base64 placeholder or empty response).
- AE3. **Covers R8.** Given an OAuth request for a supported provider (WeChat or DingTalk), when the endpoint is called, it returns a valid authorization URL for the provider (not an example URL).

---

## Success Criteria

- 所有 80 个 crate 的路由在 `main.rs` 中可见，前端可通过对应前缀调用任意已接入的 API
- 桩代码端点明确标记 TODO，核心业务端点返回真实数据而非占位响应
- 认证模块完全可用，验证码和 OAuth 不再返回占位数据
- 迁移进度可追踪，团队能明确看到哪些模块已框架接入、哪些已有真实实现、哪些仍需填充
- Rust 服务通过反向代理与 Java 服务共存，前端无感知切换

---

## Scope Boundaries

- 不修改前端 `o2web` 的任何代码，仅通过 URL 前缀路由适配后端切换
- 不在改写期间实现 Java ↔ Rust 的实时数据同步，仅依赖一次性迁移窗口
- 不拆分为微服务，Rust 侧始终以单一进程单体服务运行
- 不包含 Rust 性能压测或与 Java 的基准对比
- 不迁移 `o2web` 前端，该部分保持现状
- 不处理 Java 服务的下线和回滚脚本（后续阶段）
- 不进行数据库 schema 变更或迁移脚本编写（沿用现有计划中的 schema）

---

## Key Decisions

- **三轨并行推进**：框架接入（所有 crate 接入 main.rs）、核心业务真实化（填充 CRUD 端点）、认证完善（替换占位实现）三条轨道同时推进，最大化并行度
- **桩代码标记 TODO 而非删除**：保留桩代码作为后续实现的明确占位，避免遗漏端点
- **已实现模块优先接入**：control、personal_extend 等已有真实实现的 crate 优先接入 main.rs，让团队尽快看到可用 API
- **Rust 为唯一技术选项**：无替代方案（Go 等），迁移必须使用 Rust
- **沿用 Strangler Fig 迁移策略**：双轨运行、按模块切换、灰度验证，已在前期计划中验证

---

## Dependencies / Assumptions

- Java 服务在 Rust 改写期间持续稳定运行，是 Rust 模块功能验证前的可用参照物
- 76 个未集成 crate 的路由与 `main.rs` 中已有 4 个 crate 的路由无冲突
- auth 模块的验证码和 OAuth 集成有可行的第三方服务或自建方案
- 迁移窗口期的数据量在可接受范围内
- 反向代理（nginx）已存在或可部署，作为前缀路由层
- `control` 和 `personal_extend` crate 的真实实现与 Java 端行为一致（需前端联调验证）
- 所有新接入端点具备认证、输入验证和速率限制保护

---

## Outstanding Questions

### Resolve Before Planning

- [Affects R1][User decision] 76 个未集成 crate 是否一次性全部接入 `main.rs`，还是按批次分批接入？→ 已改为分阶段接入：第一阶段接入已有真实实现的 crate，第二阶段接入其余桩代码 crate
- [Affects R7][User decision] 验证码图片生成采用第三方服务集成还是本地实现？→ 已指定本地生成（集成 captcha 库）
- [Affects R8][User decision] OAuth 第三方登录支持哪些平台？→ 已指定微信和钉钉

### Deferred to Planning

- [Affects R4][Needs research] 组织控制模块（control）的 20 个 CRUD 端点中，哪些在 Java 侧有复杂业务逻辑需要在 Rust 侧还原？
- [Affects R10][Technical] 双轨运行期间 Rust 与 Java 模块间的跨服务调用如何处理？
- [Affects R11][Needs research] 模块跟踪清单的格式和更新频率如何定义？