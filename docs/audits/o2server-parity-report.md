# OA4Rust vs Java o2server 接口对齐审计报告

**生成时间：** 2026-08-12
**审计范围：** oa4rust 88 个 crate, 2625 个 handler; Java o2server 55 个模块

---

## 概览

| 指标 | 数值 |
|------|------|
| Rust handler 总数 | 2625 |
| Rust 路由总数 | 1012 |
| Rust 测试总数 | 787 |
| Java 模块数 | 55 |
| Java @Path 总数 | 5858 |
| 整体测试覆盖率 | ~15% |

---

## 按模块对比

| Java 模块 | Rust Crate | Rust Handler | Java @Path | Rust 测试 | 状态 |
|-----------|-----------|-------------|-----------|----------|------|
| x_ai_assemble_control | ai_assemble_control | 32 | 111 | 8 | 部分实现 |
| x_ai_core_entity | ai | 21 | 0 | 32 | 已实现 |
| x_attendance_assemble_control | attendance_assemble_control | 89 | 588 | 5 | 部分实现 |
| x_attendance_core_entity | attendance | 9 | 0 | 8 | 已实现 |
| x_base_core_project | base | 3 | 10 | 3 | 部分实现 |
| x_bbs_assemble_control | bbs_assemble_control | 57 | 354 | 10 | 部分实现 |
| x_bbs_core_entity | bbs | 9 | 0 | 7 | 已实现 |
| x_calendar_assemble_control | calendar_assemble_control | 4 | 96 | 4 | 部分实现 |
| x_calendar_core_entity | calendar | 10 | 0 | 5 | 已实现 |
| x_cms_assemble_control | cms_assemble_control | 311 | 1515 | 4 | 部分实现 |
| x_cms_core_entity | cms_core_entity | 7 | 0 | 10 | 已实现 |
| x_cms_core_express | cms_core_express | 2 | 0 | 3 | 已实现 |
| x_component_assemble_control | component_assemble_control | 10 | 24 | 4 | 部分实现 |
| x_component_core_entity | component | 3 | 0 | 5 | 已实现 |
| x_console | console | 7 | 0 | 10 | 已实现 |
| x_correlation_core_entity | correlation | 3 | 0 | 4 | 已实现 |
| x_correlation_core_express | correlation_core_express | 2 | 0 | 3 | 已实现 |
| x_correlation_service_processing | correlation_service_processing | 20 | 13 | 33 | 已实现 |
| x_file_assemble_control | file_assemble_control | 94 | 336 | 20 | 部分实现 |
| x_file_core_entity | file | 9 | 0 | 9 | 已实现 |
| x_general_assemble_control | general_assemble_control | 61 | 156 | 4 | 部分实现 |
| x_general_core_entity | general | 3 | 0 | 4 | 已实现 |
| x_hotpic_assemble_control | hotpic_assemble_control | 18 | 39 | 10 | 部分实现 |
| x_hotpic_core_entity | hotpic | 3 | 0 | 4 | 已实现 |
| x_jpush_assemble_control | jpush_assemble_control | 16 | 33 | 8 | 部分实现 |
| x_jpush_core_entity | jpush | 6 | 0 | 6 | 已实现 |
| x_meeting_assemble_control | meeting_assemble_control | 59 | 231 | 12 | 部分实现 |
| x_meeting_core_entity | meeting | 9 | 0 | 8 | 已实现 |
| x_message_assemble_communicate | message_assemble_communicate | 58 | 204 | 6 | 部分实现 |
| x_message_core_entity | message | 5 | 0 | 6 | 已实现 |
| x_mind_assemble_control | mind_assemble_control | 9 | 50 | 3 | 部分实现 |
| x_mind_core_entity | mind | 10 | 0 | 3 | 已实现 |
| x_organization_assemble_authentication | auth | 45 | 59 | 42 | 已实现 |
| x_organization_assemble_control | control | 25 | 189 | 15 | 部分实现 |
| x_organization_assemble_express | organization_assemble_express | 4 | 142 | 6 | 部分实现 |
| x_organization_assemble_personal | personal | 22 | 79 | 22 | 部分实现 |
| x_organization_core_entity | organization_core_entity | 24 | 0 | 21 | 已实现 |
| x_organization_core_express | organization_core_express | 3 | 0 | 4 | 已实现 |
| x_portal_assemble_designer | portal_assemble_designer | 56 | 71 | 11 | 已实现 |
| x_portal_assemble_surface | portal_assemble_surface | 48 | 44 | 13 | 已实现 |
| x_portal_core_entity | portal | 11 | 0 | 5 | 已实现 |
| x_processplatform_assemble_bam | processplatform_assemble_bam | 50 | 47 | 10 | 已实现 |
| x_processplatform_assemble_designer | processplatform_assemble_designer | 96 | 127 | 12 | 已实现 |
| x_processplatform_assemble_surface | processplatform_assemble_surface | 487 | 701 | 13 | 已实现 |
| x_processplatform_core_entity | processplatform_core_entity | 6 | 0 | 9 | 已实现 |
| x_processplatform_core_express | processplatform_core_express | 6 | 0 | 8 | 已实现 |
| x_processplatform_service_processing | processplatform_service_processing | 99 | 144 | 10 | 已实现 |
| x_program_center | program_center | 205 | 274 | 7 | 已实现 |
| x_program_center_core_entity | program_center_core_entity | 20 | 0 | 15 | 已实现 |
| x_program_init | program_init | 3 | 20 | 5 | 部分实现 |
| x_query_assemble_designer | query_assemble_designer | 67 | 95 | 9 | 已实现 |
| x_query_assemble_surface | query_assemble_surface | 59 | 77 | 9 | 已实现 |
| x_query_core_entity | query_core_entity | 5 | 0 | 9 | 已实现 |
| x_query_core_express | query_core_express | 4 | 0 | 8 | 已实现 |
| x_query_service_processing | query_service | 3 | 29 | 4 | 部分实现 |

---

## 测试覆盖率详情

| Rust Crate | Handler 数 | 测试数 | 覆盖率 |
|-----------|-----------|--------|--------|
| attendance_assemble_control | 89 | 5 | 6% |
| bbs_assemble_control | 57 | 10 | 18% |
| cms_assemble_control | 311 | 4 | 1% |
| file_assemble_control | 94 | 20 | 21% |
| general_assemble_control | 61 | 4 | 7% |
| ldap | 1 | 0 | 0% |
| meeting_assemble_control | 59 | 12 | 20% |
| message_assemble_communicate | 58 | 6 | 10% |
| organization_assemble_authentication | 2 | 0 | 0% |
| organization_assemble_control | 105 | 8 | 8% |
| organization_assemble_personal | 2 | 0 | 0% |
| portal_assemble_designer | 56 | 11 | 20% |
| processplatform_assemble_designer | 96 | 12 | 12% |
| processplatform_assemble_surface | 487 | 13 | 3% |
| processplatform_service_processing | 99 | 10 | 10% |
| program_center | 205 | 7 | 3% |
| query_assemble_designer | 67 | 9 | 13% |
| query_assemble_surface | 59 | 9 | 15% |

### 覆盖率 <20% 的模块（需优先补测）

- `processplatform_assemble_surface`: 13/487 handlers (3%)
- `cms_assemble_control`: 4/311 handlers (1%)
- `program_center`: 7/205 handlers (3%)
- `organization_assemble_control`: 8/105 handlers (8%)
- `processplatform_service_processing`: 10/99 handlers (10%)
- `processplatform_assemble_designer`: 12/96 handlers (12%)
- `attendance_assemble_control`: 5/89 handlers (6%)
- `query_assemble_designer`: 9/67 handlers (13%)
- `general_assemble_control`: 4/61 handlers (7%)
- `query_assemble_surface`: 9/59 handlers (15%)
- `message_assemble_communicate`: 6/58 handlers (10%)
- `bbs_assemble_control`: 10/57 handlers (18%)
- `portal_assemble_designer`: 11/56 handlers (20%)
- `organization_assemble_authentication`: 0/2 handlers (0%)
- `organization_assemble_personal`: 0/2 handlers (0%)
- `ldap`: 0/1 handlers (0%)

---

## 结论

1. **接口覆盖：** oa4rust 已实现大部分 Java o2server 的核心端点，但部分模块（如 processplatform_service_processing）的 handler 数远少于 Java
2. **测试覆盖：** 整体 handler 级测试覆盖率约 15%，需提升至 ≥95%
3. **零测试 crate：** ldap、organization_assemble_authentication、organization_assemble_personal 完全无测试
4. **下一步：** 基于本报告进行针对性测试补全（里程碑 M2）

---

*本报告由 scripts/o2server_parity_audit_v2.py 自动生成*