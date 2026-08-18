---
date: 2026-08-12
topic: oa4rust-final-gap-closure
---

# OA4Rust 最终缺口补全 — 编译错误修复、Stub 清理、响应一致性

## Summary

修复阻碍 `cargo test` 通过的 portal 测试编译错误，补全 WeChat 模板消息发送和注册短信发送两个 stub，统一 processplatform_service_processing 和 cms_assemble_control 中"记录不存在"的响应格式，使 oa4rust 在所有功能和业务逻辑上完全代替 o2server。

---

## Problem Frame

oa4rust 已完成 86 个 crate 的真实化和 7,600+ 条路由注册，核心业务逻辑与 Java o2server 高度对齐。但深度扫描揭示四个残余缺口：`portal/tests.rs` 引用了已重命名的函数导致测试套件编译失败；`auth/mpweixin.rs` 中的微信小程序模板消息发送仍是 stub；`personal/regist.rs` 中的注册短信发送仅有存库逻辑而未实际发送；processplatform_service_processing 和 cms_assemble_control 在记录不存在时返回 `Value::Null` 而非结构化的错误响应，与 Java 端行为不一致。这些缺口虽不影响主干编译（`cargo check` 通过），但阻塞测试验证并导致部分端点在找不到记录时返回结构不统一。

---

## Actors

- A1. **开发者**：修复测试编译错误、补全 stub、统一响应格式
- A2. **前端 o2web**：依赖一致的错误响应结构，`Value::Null` 可能导致前端渲染异常
- A3. **CI 流水线**：`cargo test --workspace` 必须全部通过
- A4. **微信小程序用户**：需要模板消息发送功能用于业务通知

---

## Key Flows

- **F1. 测试修复流**
  - **Trigger：** 运行 `cargo test --workspace` 时 portal 测试编译失败
  - **Actors：** A1, A3
  - **Steps：** 1. 确认 `portal_id` 函数签名 2. 修正 `tests.rs` 中的函数引用 3. 验证测试通过
  - **Outcome：** `cargo test --workspace --lib` 通过，无编译错误
  - **Covered by:** R1

- **F2. 小程序模板消息补全流程**
  - **Trigger：** 管理员调用模板消息测试端点
  - **Actors：** A1, A4
  - **Steps：** 1. 接收目标用户 ID 和模板 ID 2. 调用微信 API 发送模板消息 3. 记录发送日志
  - **Outcome：** 模板消息真实发送到微信服务器，返回发送结果
  - **Covered by:** R2

- **F3. 注册短信发送补全流程**
  - **Trigger：** 用户调用注册接口发送验证码
  - **Actors：** A1, A4
  - **Steps：** 1. 生成随机验证码 2. 调用短信服务商 API 发送 3. 存储验证码到数据库
  - **Outcome：** 用户收到真实短信验证码，注册流程可正常进行
  - **Covered by:** R3

- **F4. 响应格式统一流**
  - **Trigger：** 客户端调用 processplatform 或 cms_assemble_control 的 GET 端点，记录不存在
  - **Actors：** A1, A2
  - **Steps：** 1. 识别 4 处 `None => Value::Null` 返回 2. 改为返回结构化空对象或错误响应 3. 与 Java 端行为对齐
  - **Outcome：** 所有端点在记录不存在时返回一致的响应格式
  - **Covered by:** R4-R7

---

## Requirements

**测试编译修复**
- R1. 修复 `crates/portal/src/tests.rs:29` 中的函数引用错误：将 `portal_get` 改为 `portal_id`，确保 `cargo test --workspace --lib` 通过

**小程序模板消息补全**
- R2. 补全 `crates/auth/src/mpweixin.rs:213` 处的模板消息发送 stub：实现调用微信模板消息 API 的逻辑，支持 `TmplMsgId` 和 `content` 参数，发送失败时返回明确错误信息

**注册短信发送补全**
- R3. 补全 `crates/personal/src/regist.rs:202` 处的短信发送 TODO：实现调用短信服务商 API 的逻辑（复用现有 `code_store` 机制），支持多供应商配置（环境变量控制）

**响应格式统一**
- R4. 修复 `crates/processplatform_service_processing/src/lib.rs:416`（`work_id_manual_append_identity`）：记录不存在时返回 `{error: "record not found"}` 而非 `Value::Null`
- R5. 修复 `crates/processplatform_service_processing/src/lib.rs:792`（`taskcompleted_next_task_identity`）：同上
- R6. 修复 `crates/processplatform_service_processing/src/lib.rs:887`（`snap_work_workId_type_snap`）：同上
- R7. 修复 `crates/cms_assemble_control/src/lib.rs:222`（`get_by_id` helper）：返回 `ActionResult::success(Value::Null)` 或结构化空对象，与其他端点保持一致

---

## Acceptance Examples

- AE1. **Covers R1.** Given 运行 `cargo test --workspace --lib`，when 测试编译阶段，then portal 测试模块编译成功，全部测试用例通过
- AE2. **Covers R2.** Given 微信小程序已配置 `MPWEXIN_APP_ID` 和 `MPWEXIN_APP_SECRET`，when 管理员调用模板消息测试端点，then 微信服务器收到模板消息，返回发送成功响应
- AE3. **Covers R3.** Given 短信服务商已配置环境变量，when 用户调用注册验证码端点，then 用户收到真实短信验证码，注册流程可正常进行
- AE4. **Covers R4-R7.** Given 调用 processplatform 或 cms_assemble_control 的 GET 端点且记录不存在，when 请求返回时，then 响应包含结构化错误信息而非 `null`，前端可正确识别和处理

---

## Success Criteria

- `cargo check --workspace` 通过，无新增编译错误或警告
- `cargo test --workspace --lib` 全部通过，无编译错误
- `auth/mpweixin.rs` 模板消息端点返回真实发送结果而非 stub 消息
- `personal/regist.rs` 注册验证码端点发送真实短信
- processplatform_service_processing 和 cms_assemble_control 的 GET 端点在记录不存在时返回结构化响应
- 无新增 `stub`、`todo!`、`unimplemented!` 标记

---

## Scope Boundaries

- 仅修复上述 4 类缺口，不新增 Java 端不存在的新功能
- 短信发送仅实现基础 API 调用，不实现复杂模板管理和发送状态回调
- 模板消息仅实现微信官方 API 调用，不实现消息队列和异步发送
- 响应格式统一仅处理"记录不存在"场景，不修改其他业务的错误处理逻辑
- 前端 o2web 的代码修改不在范围内
- Java o2server 的代码修改不在范围内

### Deferred for later

- 短信发送的模板管理和发送状态回调
- 模板消息的异步发送队列
- 更复杂的错误码体系和国际化错误消息
- 其他模块的类似响应格式问题（如有）

### Outside this product's identity

- 前端 o2web 的重写或现代化改造
- 独立的短信服务商 SDK 发布
- Java 服务的永久下线决策

---

## Key Decisions

- **测试修复优先**：编译错误是阻塞性问题，必须首先修复以确保 CI 通过
- **短信发送复用现有 code_store 机制**：与现有认证模块的短信验证码流程保持一致，不引入新的依赖
- **模板消息直接调用微信 API**：不引入消息队列，保持实现简单，后续可根据需要扩展
- **响应格式统一使用结构化错误对象**：与 Java 端行为一致，前端可正确识别和处理

---

## Dependencies / Assumptions

- 微信小程序 API 密钥已通过环境变量配置（`MPWEXIN_APP_ID`、`MPWEXIN_APP_SECRET`）
- 短信服务商 API 密钥已通过环境变量配置
- 现有 `code_store` 机制支持短信发送扩展
- 前端 o2web 对错误响应的结构有隐式依赖，新增错误格式需保持兼容

---

## Outstanding Questions

### Resolve Before Planning

（无阻塞问题）

### Deferred to Planning

- [Affects R2][Needs research] 微信模板消息 API 的具体参数和响应格式——需查阅微信官方文档
- [Affects R3][Needs research] 短信服务商 API 选择——需评估现有配置和环境变量
- [Affects R4-R7][Technical] 响应格式的具体结构——需对照 Java 端行为确认
