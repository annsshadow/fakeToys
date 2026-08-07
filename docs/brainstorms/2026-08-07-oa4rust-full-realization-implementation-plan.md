# OA4Rust 全波次真实化实施文档

**生成日期：** 2026-08-07
**状态：** 实施前置规划
**依据：** `docs/brainstorms/2026-08-05-oa4rust-comprehensive-advancement-requirements.md`

---

## 总览

| 波次 | 目标 | 包含 crate 数 | 预计工作量 | 依赖 |
|------|------|--------------|-----------|------|
| Wave 1 (U4+U3 收尾) | 基础设施加固、会话持久化、6 个完整 crate 校验 | 6 完整 + 共享层 | 中 | U1, U2 |
| Wave 2 (U5) | 补全 attendance/calendar/file/general_assemble_control 及其衍生 crate | 11 | 高 | Wave 1 |
| Wave 3 (U6) | 核心工作流：meeting/portal/process/query/cms | 31 | 极高 | Wave 2 |
| Wave 4 (U7) | 边缘模块：AI/BBS/组件/热点/推送/思维导图/快递/控制台/表达式/关联关系 | 33 | 高 | Wave 3 |

---

## Wave 1：基础设施收尾 + 完整 crate 加固

### 目标

将已完成真实化的 6 个 crate（auth, control, personal, personal_extend, message, program_init）加固到生产级，同时完成 U1-U3 的基础设施收尾工作。

### 包含 crate

| crate | 当前状态 | 目标状态 |
|-------|---------|---------|
| shared | 已完成（基础设施） | 保持不变，作为基础设施底座 |
| auth | 已完成（真实化） | 会话持久化 + 安全加固 |
| control | 已完成（真实化） | 边界情况完善 |
| personal | 已完成（真实化） | 边界情况完善 |
| personal_extend | 已完成（真实化） | 边界情况完善 |
| message | 已完成（真实化） | 保持不变，作为 Wave 1 完成标志 |
| program_init | 已完成（真实化） | 保持不变 |

### 实施任务

#### T1.1 会话持久化改造

**文件：** `oa4rust/crates/auth/src/session.rs`, `oa4rust/crates/auth/src/session_store.rs`（新建）

**现状：** `SessionManager` 为纯内存 HashMap，重启即失效，多实例部署必现问题。

**目标：** 将 `SessionManager` 迁移到 PostgreSQL `auth_session` 表持久化，保留内存缓存层做热读。

**验收标准：**
- 登录后重启服务，session 仍有效
- 多实例部署时 session 互认
- 登出时立即失效（DB 标记 revoked）
- 性能：热读命中率 > 95%，P99 < 5ms

**端点契约：** 无新增端点，改造现有 `SessionManager` 内部实现。

#### T1.2 RateLimiter 内存泄漏修复

**文件：** `oa4rust/crates/shared/src/rate_limit.rs`

**现状：** 滑动窗口无定期清理，长期运行内存线性增长。

**目标：** 为滑动窗口添加定期清理机制（TTL 过期条目自动移除）。

**验收标准：**
- 运行 24 小时后内存增长 < 10MB
- 清理任务不影响限流准确性

#### T1.3 auth 安全加固验证

**文件：** `oa4rust/crates/auth/src/` 各 handlers

**现状：** 已知安全加固大部分已实现，需验证生效。

**验收标准：**
- OAuth `state` 参数验证通过测试
- 验证码一次性使用验证通过测试
- 密码哈希 rehash 机制验证通过测试
- 速率限制 10 次/分钟/IP 验证通过测试

#### T1.4 密码规则校验

**文件：** `oa4rust/crates/personal/src/password.rs`, `oa4rust/crates/personal_extend/src/password.rs`

**现状：** 密码修改/重置端点已实现，缺少复杂度校验。

**目标：** 添加密码规则校验（长度 6-64、复杂度）。

**验收标准：**
- 长度 < 6 返回 400
- 长度 > 64 返回 400
- 前端契约字段不变

#### T1.5 头像上传加固

**文件：** `oa4rust/crates/personal_extend/src/avatar.rs`

**现状：** 头像上传已实现，缺少大小限制和格式验证。

**目标：** 添加头像大小限制（5MB）、MIME 白名单验证。

**验收标准：**
- 上传 > 5MB 返回 400
- 非图片 MIME 返回 400
- 前端契约字段不变

#### T1.6 边界情况完善

**文件：** `oa4rust/crates/control/src/` 各模块

**现状：** CRUD 已完整，边界情况待完善。

**目标：** 确保 `deleted_at IS NULL` 过滤在所有查询中生效；非法 flag 参数校验；空结果分页。

**验收标准：**
- 软删除记录不可见
- 不存在的 flag 返回空（非 500）
- 游标分页边界正确

### Wave 1 完成标志

- [x] `cargo test` auth 模块全部通过
- [x] 会话持久化验证通过（重启后 session 有效）
- [x] RateLimiter 内存泄漏修复验证通过
- [x] 密码规则校验和头像上传加固验证通过
- [x] 边界情况测试覆盖
- [x] `docs/brainstorms/oa4rust-migration-status.md` 更新

---

## Wave 2：attendance/calendar/file/general_assemble_control 补全

### 目标

补全 Wave 2 相关 crate 的真实业务逻辑，这些 crate 已有部分 PostgreSQL 查询，需补充完整 CRUD 和业务编排。

### 包含 crate

| crate | 当前状态 | 目标状态 | 预计端点数 |
|-------|---------|---------|-----------|
| attendance | 部分真实化 | 部分真实化 → 完整真实化 | ~10 |
| attendance_assemble_control | 部分真实化 | 部分真实化 → 完整真实化 | ~97 |
| attendance_core_entity | 部分真实化 | 部分真实化 → 完整真实化 | ~15 |
| calendar | 部分真实化 | 部分真实化 → 完整真实化 | ~10 |
| calendar_assemble_control | 部分真实化 | 部分真实化 → 完整真实化 | ~5 |
| calendar_core_entity | 部分真实化 | 部分真实化 → 完整真实化 | ~8 |
| file | 部分真实化 | 部分真实化 → 完整真实化 | ~8 |
| file_assemble_control | 部分真实化 | 部分真实化 → 完整真实化 | ~6 |
| file_core_entity | 部分真实化 | 部分真实化 → 完整真实化 | ~12 |
| general_assemble_control | 部分真实化 | 部分真实化 → 完整真实化 | ~71 |
| general_core_entity | 部分真实化 | 部分真实化 → 完整真实化 | ~20 |

### 实施任务

#### T2.1 attendance 补全

**文件：** `oa4rust/crates/attendance/src/lib.rs`

**现状：** list_admins/list_employee_configs/list_statistical_cycles 已真实；打卡记录、排班规则、申诉流程仍为桩。

**目标端点：**
| 路由路径 | HTTP 方法 | Handler | 实现状态 |
|----------|-----------|---------|---------|
| /jaxrs/attendance/record/list | GET | list_check_in_records | 待实现 |
| /jaxrs/attendance/rule/list | GET | list_schedule_rules | 待实现 |
| /jaxrs/attendance/appeal/list | GET | list_appeal_records | 待实现 |
| /jaxrs/attendance/appeal/submit | POST | submit_appeal | 待实现 |
| /jaxrs/attendance/appeal/audit | POST | audit_appeal | 待实现 |
| /jaxrs/attendance/appeal/archive/{id} | POST | archive_appeal | 待实现 |

**验收标准：**
- 打卡记录查询返回真实数据
- 申诉流程完整可用
- 排班规则 CRUD 正常

#### T2.2 attendance_assemble_control 补全

**文件：** `oa4rust/crates/attendance_assemble_control/src/lib.rs`

**现状：** 97 个路由中约 10 个已真实（rule/list/toggle、admin list 等）。

**目标：** 补全剩余 ~87 个路由，包括：
- 考勤规则 CRUD
- 排班管理
- 申诉审批流程
- 统计报表

**验收标准：**
- 所有路由返回真实数据
- 业务逻辑与 Java 等效

#### T2.3 attendance_core_entity 补全

**文件：** `oa4rust/crates/attendance_core_entity/src/lib.rs`

**现状：** record_list、rule_list、appeal_list 已真实。

**目标：** 补全打卡记录、排班规则、申诉记录的 CRUD。

#### T2.4 calendar 补全

**文件：** `oa4rust/crates/calendar/src/lib.rs`

**现状：** calendar_list_public/my/get 已真实；create/update/remove/event 相关仍为桩。

**目标端点：**
| 路由路径 | HTTP 方法 | Handler | 实现状态 |
|----------|-----------|---------|---------|
| /jaxrs/calendar/calendar/create | POST | calendar_create | 待实现 |
| /jaxrs/calendar/calendar/update | POST | calendar_update | 待实现 |
| /jaxrs/calendar/calendar/remove | POST | calendar_remove | 待实现 |
| /jaxrs/calendar/event/create | POST | event_create | 待实现 |
| /jaxrs/calendar/event/update | POST | event_update | 待实现 |
| /jaxrs/calendar/event/remove | POST | event_remove | 待实现 |
| /jaxrs/calendar/event/list/{calendarId} | GET | event_list | 待实现 |

**验收标准：**
- 日历 CRUD 正常
- 事件 CRUD 正常
- 共享权限正确

#### T2.5 calendar_assemble_control / calendar_core_entity 补全

**目标：** 补全 config/calendars 查询和 calendar CRUD。

#### T2.6 file 补全

**文件：** `oa4rust/crates/file/src/lib.rs`

**现状：** folder_list_top/list_with_folder/complex_top 已真实；上传/下载/权限仍为桩。

**目标端点：**
| 路由路径 | HTTP 方法 | Handler | 实现状态 |
|----------|-----------|---------|---------|
| /jaxrs/file/upload | POST | file_upload | 待实现 |
| /jaxrs/file/download/{id} | GET | file_download | 待实现 |
| /jaxrs/file/folder/create | POST | folder_create | 待实现 |
| /jaxrs/file/folder/update/{id} | POST | folder_update | 待实现 |
| /jaxrs/file/folder/delete/{id} | POST | folder_delete | 待实现 |
| /jaxrs/file/permission/set | POST | set_permission | 待实现 |

**验收标准：**
- 文件上传（multipart、MIME 白名单、5MB 限制）
- 文件下载正常
- 文件夹 CRUD 正常

#### T2.7 file_assemble_control / file_core_entity 补全

**目标：** 补全 config/storage/categories 查询和 folder/file CRUD。

#### T2.8 general_assemble_control 补全

**文件：** `oa4rust/crates/general_assemble_control/src/lib.rs`

**现状：** 1970 行，status/permissions/attendscope/area/qrcode/securityclearance 等已真实；71 个路由中约 30+ 已真实。

**目标：** 补全剩余 ~40+ 路由，包括：
- 区域管理（area CRUD 已完成，补全其余）
- 二维码管理
- 安全 clearance
- 发票管理
- 通用配置

**验收标准：**
- 所有路由返回真实数据
- 复杂查询（多表 JOIN）正确

#### T2.9 general_core_entity 补全

**现状：** dict/file/invoice CRUD 已实现。

**目标：** 补全通用配置、序列号、权限等端点。

### Wave 2 完成标志

- [ ] `cargo test` Wave 2 所有 crate 通过
- [ ] 文件上传/下载在测试环境手动验证
- [ ] 日历/考勤核心流程端到端可走通
- [ ] `docs/brainstorms/oa4rust-migration-status.md` 更新
- [ ] `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新

---

## Wave 3：核心工作流（meeting/portal/process/query/cms）

### 目标

完成流程引擎、会议、门户、查询报表、CMS 模块的真实化，覆盖核心用户工作流。

### 包含 crate

| crate | 当前状态 | 预计端点数 |
|-------|---------|-----------|
| meeting | 部分真实化 | ~9 |
| meeting_assemble_control | 部分真实化 | ~92 |
| meeting_core_entity | 部分真实化 | ~10 |
| message_assemble_communicate | 部分真实化 | ~118 |
| message_core_entity | 部分真实化 | ~5 |
| portal | 部分真实化 | ~10 |
| portal_assemble_designer | 部分真实化 | ~20 |
| portal_assemble_surface | 部分真实化 | ~15 |
| portal_core_entity | 部分真实化 | ~8 |
| process_designer | 部分真实化 | ~10 |
| process_express | 部分真实化 | ~5 |
| process_bam | 已接入（桩代码） | ~30 |
| process_surface | 部分真实化 | ~5 |
| processplatform_assemble_bam | 部分真实化 | ~20 |
| processplatform_assemble_designer | 部分真实化 | ~20 |
| processplatform_assemble_surface | 部分真实化 | ~20 |
| processplatform_core_entity | 部分真实化 | ~10 |
| processplatform_core_express | 部分真实化 | ~10 |
| processplatform_service_processing | 部分真实化 | ~30 |
| query_assemble_designer | 已接入（桩代码） | ~10 |
| query_assemble_surface | 已接入（桩代码） | ~10 |
| query_core_entity | 部分真实化 | ~10 |
| query_core_express | 已接入（桩代码） | ~5 |
| query_express | 已接入（桩代码） | ~5 |
| query_service | 已接入（桩代码） | ~5 |
| query_service_processing | 已接入（桩代码） | ~5 |
| cms_assemble_control | 部分真实化 | ~10 |
| cms_control | 已接入（桩代码） | ~20 |
| cms_core_entity | 部分真实化 | ~10 |
| cms_core_express | 已接入（桩代码） | ~10 |
| cms_express | 已接入（桩代码） | ~10 |

### 实施任务

#### T3.1 meeting 补全

**文件：** `oa4rust/crates/meeting/src/lib.rs`

**现状：** 6/9 端点已真实；room_list/building_list/openmeeting_list_room 仍为 mock。

**目标：** 补全 room_list/building_list/openmeeting_list_room 为真实 DB 查询。

**验收标准：**
- 会议室列表返回真实数据
- 建筑列表返回真实数据
- 开放式讨论区列表返回真实数据

#### T3.2 meeting_assemble_control / meeting_core_entity 补全

**目标：** 补全会议 CRUD、参与人管理、日程关联。

#### T3.3 message_assemble_communicate / message_core_entity 补全

**现状：** 118 个路由中仅少量已实现。

**目标：** 补全消息发送/接收/已读/未读/删除等端点。

#### T3.4 portal 补全

**目标：** 补全门户页面 CRUD、部件管理、脚本管理、字典管理。

#### T3.5 portal_assemble_designer / portal_assemble_surface / portal_core_entity 补全

**目标：** 补全页面设计器、surface 发布/预览、page CRUD。

#### T3.6 process_designer 补全

**现状：** application_list_summary、designer_get_route 已真实。

**目标：** 补全流程设计器 CRUD、表单定义、路由配置。

#### T3.7 process_express 补全

**现状：** task_count/read_count/application_list 已真实。

**目标：** 补全任务列表、工作项操作、流程状态查询。

#### T3.8 process_surface 补全

**现状：** list_ids/get/record 已真实。

**目标：** 补全流程实例查询、工作流状态。

#### T3.9 processplatform_assemble_bam 补全

**现状：** bam config/list 等已实现；20+ 统计路由为桩。

**目标：** 补全 BAM 监控统计路由。

#### T3.10 processplatform_assemble_designer 补全

**现状：** flow CRUD 已实现；preview 等仍为桩。

**目标：** 补全流程应用预览、发布、删除。

#### T3.11 processplatform_assemble_surface 补全

**目标：** 补全 surface preview/publish 等端点。

#### T3.12 processplatform_core_entity 补全

**目标：** 补全 work/task/ticket CRUD、workcompleted 列表。

#### T3.13 processplatform_core_express 补全

**目标：** 补全 work/task 操作（terminate/retract/processing 等）。

#### T3.14 processplatform_service_processing 补全

**目标：** 补全 process CRUD、instance 管理、cancel 等。

#### T3.15 query_assemble_designer / query_assemble_surface 补全

**目标：** 从桩代码实现查询设计器和 surface 的 CRUD。

#### T3.16 query_core_entity 补全

**目标：** 补全 item/view/import 查询的 CRUD。

#### T3.17 query_core_express / query_express / query_service / query_service_processing 补全

**目标：** 从桩代码实现查询执行、结果导出等功能。

#### T3.18 cms 系列补全

**目标：** 补全 CMS 栏目/文章/字典/索引的 CRUD 和发布流程。

### Wave 3 完成标志

- [ ] `cargo test` Wave 3 所有 crate 通过
- [ ] 核心业务流程（发起流程 → 审批 → 通知）端到端可走通
- [ ] 会议预约/查询/取消正常
- [ ] 门户页面发布/预览正常
- [ ] CMS 文章发布/撤回正常
- [ ] `docs/brainstorms/oa4rust-migration-status.md` 更新

---

## Wave 4：边缘模块（AI/BBS/组件/热点/推送/思维导图/快递/控制台/表达式/关联关系）

### 目标

完成剩余基础设施模块的真实化，允许简化或保留接口 stub 但返回真实数据。

### 包含 crate

| crate | 当前状态 | 预计端点数 |
|-------|---------|-----------|
| ai | 部分真实化 | ~10 |
| ai_assemble_control | 已接入（桩代码） | ~30 |
| ai_core_entity | 已接入（桩代码） | ~10 |
| bbs | 部分真实化 | ~10 |
| bbs_assemble_control | 部分真实化 | ~15 |
| bbs_core_entity | 部分真实化 | ~15 |
| component | 部分真实化 | ~5 |
| component_assemble_control | 部分真实化 | ~10 |
| component_core_entity | 已接入（桩代码） | ~5 |
| console | 已接入（桩代码） | ~8 |
| correlation | 部分真实化 | ~5 |
| correlation_core_entity | 部分真实化 | ~5 |
| correlation_core_express | 部分真实化 | ~5 |
| correlation_service_processing | 部分真实化 | ~15 |
| express | 部分真实化 | ~3 |
| general | 部分真实化 | ~5 |
| general_core_entity | 部分真实化 | ~20 |
| hotpic | 部分真实化 | ~5 |
| hotpic_assemble_control | 部分真实化 | ~10 |
| hotpic_core_entity | 部分真实化 | ~5 |
| jpush | 部分真实化 | ~8 |
| jpush_assemble_control | 部分真实化 | ~10 |
| jpush_core_entity | 部分真实化 | ~8 |
| mind | 部分真实化 | ~12 |
| mind_assemble_control | 部分真实化 | ~8 |
| mind_core_entity | 部分真实化 | ~12 |
| organization_assemble_control | 部分真实化 | ~20 |
| organization_assemble_express | 部分真实化 | ~5 |
| organization_core_entity | 部分真实化 | ~8 |
| organization_core_express | 部分真实化 | ~5 |
| program_center | 部分真实化 | ~8 |
| program_center_core_entity | 部分真实化 | ~8 |
| base | 部分真实化 | ~3 |

### 实施任务

#### T4.1 AI 模块（ai, ai_assemble_control, ai_core_entity）

**文件：** `oa4rust/crates/ai/src/lib.rs`, `oa4rust/crates/ai_assemble_control/src/lib.rs`, `oa4rust/crates/ai_core_entity/src/lib.rs`

**目标：**
- 模型管理 CRUD
- 推理调用（允许对接外部服务或返回模拟结果，但接口契约对齐）
- 应用管理 CRUD

**验收标准：**
- 模型列表返回真实数据
- 推理接口返回符合契约的响应
- 应用管理 CRUD 正常

#### T4.2 BBS 模块（bbs, bbs_assemble_control, bbs_core_entity）

**目标：**
- 论坛分类 CRUD
- 文章 CRUD + 搜索
- 版主管理
- 回复 CRUD

**验收标准：**
- 论坛列表/分类正常
- 文章发布/查询/搜索正常
- 回复功能正常

#### T4.3 组件管理（component, component_assemble_control, component_core_entity）

**目标：**
- 应用中心 CRUD
- 部署状态查询
- 分类管理

#### T4.4 控制台（console）

**目标：**
- 系统状态查询
- 日志查询（只读）
- 缓存清理
- 命令行执行（允许简化）

#### T4.5 关联关系（correlation 系列）

**目标：**
- 数据关联 CRUD
- 引用关系查询
- CMS/流程关联

#### T4.6 快递查询（express）

**目标：**
- 物流追踪（允许对接第三方 API 或返回模拟结果）
- 快递公司列表

#### T4.7 通用服务（general, general_core_entity）

**目标：**
- 区域管理
- 安全 clearance
- 工作日判断
- 发票管理
- 字典管理

#### T4.8 热点图片（hotpic 系列）

**目标：**
- 轮播图 CRUD
- 推荐列表
- 配置管理

#### T4.9 推送服务（jpush 系列）

**目标：**
- 设备管理 CRUD
- 消息推送（允许对接第三方推送服务或记录推送日志）
- 模板管理

#### T4.10 思维导图（mind 系列）

**目标：**
- 思维导图 CRUD
- 文件夹管理
- 版本管理

#### T4.11 组织模块（organization_*）

**目标：**
- 组织架构查询
- 人员导出/统计
- 配置管理

#### T4.12 程序中心（program_center, program_center_core_entity）

**目标：**
- 应用管理 CRUD
- 配置管理
- 脚本管理

#### T4.13 基础模块（base）

**目标：**
- echo 端点保持不变
- cache_detail 已真实
- openapi_info 已真实

### Wave 4 完成标志

- [ ] `cargo test` Wave 4 所有 crate 通过
- [ ] 核心 CRUD 接口返回真实数据
- [ ] `docs/brainstorms/oa4rust-migration-status.md` 更新
- [ ] `docs/brainstorms/oa4rust-endpoint-inventory.md` 更新
- [ ] 无 `ActionResult::success(Value::Null)` 残留

---

## 跨波次通用要求

### 测试要求

每个 crate 必须包含：
1. **单元测试**：覆盖核心业务逻辑（至少 2 个 happy path + 2 个 error path）
2. **集成测试**：带真实 DB 的端到端测试
3. **契约测试**：响应格式与 Java 后端等效（字段名、类型、非空约束）

### 代码规范

1. **snake_case**：所有函数名使用 snake_case
2. **错误处理**：使用 `AppError` 统一错误处理
3. **响应包装**：所有响应使用 `ActionResult<T>` 包装
4. **分页约定**：所有 list 端点返回 `count`、`size`、`position` 字段

### 文档更新要求

每次完成一个 crate：
1. 更新 `docs/brainstorms/oa4rust-migration-status.md`
2. 更新 `docs/brainstorms/oa4rust-endpoint-inventory.md`
3. 在 git commit message 中标注 `[oa4rust] <crate> realization complete`

### 依赖关系

```
U1 (Axum 0.8 + CORS + security headers)
  └── U2 (Route conflict resolution)
        └── U3 (Auth security hardening + session persistence)
              ├── Wave 1 (6 complete crates hardening)
              ├── Wave 2 (attendance/calendar/file/general)
              ├── Wave 3 (meeting/portal/process/query/cms)
              └── Wave 4 (AI/BBS/component/etc.)
```

---

## 附录：端点清单快速参考

### Wave 1 端点清单（6 个 crate）

#### auth（已完成）
- POST /jaxrs/authentication/login
- DELETE /jaxrs/authentication/logout
- GET /jaxrs/authentication/who
- GET /jaxrs/authentication/captcha
- POST /jaxrs/authentication/oauth/{provider}
- POST /jaxrs/authentication/refresh
- POST /jaxrs/authentication/code
- POST /jaxrs/secret/check
- POST /jaxrs/secret/set
- POST /jaxrs/secret/set/cancel

#### control（已完成）
- POST /jaxrs/person
- GET|PUT|DELETE /jaxrs/person/{flag}
- GET /jaxrs/person/list/{flag}/next/{count}
- GET /jaxrs/person/list/{flag}/prev/{count}
- POST /jaxrs/group
- GET|PUT|DELETE /jaxrs/group/{flag}
- GET /jaxrs/group/list/{flag}/next/{count}
- GET /jaxrs/group/list/{flag}/prev/{count}
- POST /jaxrs/role
- GET|PUT|DELETE /jaxrs/role/{flag}
- GET /jaxrs/role/list/{flag}/next/{count}
- GET /jaxrs/role/list/{flag}/prev/{count}
- POST /jaxrs/unit
- GET /jaxrs/unit/list
- GET|PUT|DELETE /jaxrs/unit/{flag}
- GET /jaxrs/unit/list/{flag}/next/{count}
- GET /jaxrs/unit/list/{flag}/prev/{count}

#### personal（已完成）
- GET /jaxrs/person
- PUT /jaxrs/person
- POST /jaxrs/person/mockputtopost
- PUT /jaxrs/password
- POST /jaxrs/password/mockputtopost
- POST /jaxrs/reset/code
- POST /jaxrs/reset/check
- POST /jaxrs/reset/set

#### personal_extend（已完成）
- GET /jaxrs/personal/info
- PUT /jaxrs/personal/update
- GET /jaxrs/personal/detail/{id}
- POST /jaxrs/password/change
- POST /jaxrs/password/reset
- POST /jaxrs/password/verify
- POST /jaxrs/personal/avatar/upload
- GET /jaxrs/personal/avatar/{id}

#### message（已完成）
- GET /jaxrs/message/consume/list/{consume}/count/{count}
- GET /jaxrs/message/consume/{id}/type/{type}
- POST /jaxrs/message/custom/create
- POST /jaxrs/message/mark_read/{id}
- GET /jaxrs/message/unread/count/{consume}

#### program_init（已完成）
- GET /jaxrs/secret/check
- POST /jaxrs/secret/set
- GET /jaxrs/secret/set/cancel

---

## 版本历史

| 日期 | 版本 | 说明 |
|------|------|------|
| 2026-08-07 | v1.0 | 初始版本，覆盖 Wave 1-4 完整实施计划 |
