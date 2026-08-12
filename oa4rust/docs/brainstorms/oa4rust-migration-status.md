# OA4Rust 迁移状态跟踪 (Migration Status)

> 单一信息源（single source of truth）。由 scripts/gen_inventory.py 自动生成。
> 状态含义：待迁移 / 迁移中 / 已完成。每完成一个 crate 的真实化后重新生成。

## U1-U4

| crate | 状态 | handlers | stub_ | null |
|-------|------|---------:|------:|-----:|
| auth | done | 45 | 0 | 0 |
| control | done | 25 | 0 | 0 |
| personal | done | 22 | 0 | 0 |
| personal_extend | done | 12 | 0 | 0 |
| program_init | done | 3 | 0 | 0 |

## U5

| crate | 状态 | handlers | stub_ | null |
|-------|------|---------:|------:|-----:|
| attendance | done | 9 | 0 | 0 |
| attendance_assemble_control | done | 89 | 0 | 0 |
| attendance_core_entity | done | 9 | 0 | 0 |
| calendar | done | 10 | 0 | 0 |
| calendar_assemble_control | done | 4 | 0 | 0 |
| calendar_core_entity | done | 10 | 0 | 0 |
| file | done | 9 | 0 | 0 |
| file_assemble_control | done | 94 | 0 | 0 |
| file_core_entity | done | 7 | 0 | 0 |
| general | done | 3 | 0 | 0 |
| general_assemble_control | done | 61 | 0 | 0 |
| general_core_entity | done | 21 | 0 | 0 |

## U6

| crate | 状态 | handlers | stub_ | null |
|-------|------|---------:|------:|-----:|
| cms_assemble_control | done | 311 | 0 | 0 |
| cms_control | done | 2 | 0 | 0 |
| cms_core_entity | done | 7 | 0 | 0 |
| cms_core_express | done | 2 | 0 | 0 |
| cms_express | done | 5 | 0 | 0 |
| meeting | done | 9 | 0 | 0 |
| meeting_assemble_control | done | 59 | 0 | 0 |
| meeting_core_entity | done | 11 | 0 | 0 |
| message | done | 5 | 0 | 0 |
| message_assemble_communicate | done | 58 | 0 | 0 |
| message_core_entity | done | 3 | 0 | 0 |
| portal | done | 11 | 0 | 0 |
| portal_assemble_designer | done | 56 | 0 | 0 |
| portal_assemble_surface | done | 48 | 0 | 0 |
| portal_core_entity | done | 8 | 0 | 0 |
| process_designer | done | 7 | 0 | 0 |
| process_express | done | 3 | 0 | 0 |
| process_surface | done | 3 | 0 | 0 |
| processplatform_assemble_bam | done | 50 | 0 | 0 |
| processplatform_assemble_designer | done | 96 | 0 | 0 |
| processplatform_assemble_surface | done | 487 | 0 | 0 |
| processplatform_core_entity | done | 6 | 0 | 0 |
| processplatform_core_express | done | 6 | 0 | 0 |
| processplatform_service_processing | doing | 99 | 0 | 3 |
| query_assemble_designer | done | 67 | 0 | 0 |
| query_assemble_surface | done | 59 | 0 | 0 |
| query_core_entity | done | 5 | 0 | 0 |
| query_core_express | done | 4 | 0 | 0 |
| query_express | done | 2 | 0 | 0 |
| query_service | done | 3 | 0 | 0 |
| query_service_processing | done | 4 | 0 | 0 |

## U7

| crate | 状态 | handlers | stub_ | null |
|-------|------|---------:|------:|-----:|
| ai | done | 21 | 0 | 0 |
| ai_assemble_control | done | 32 | 0 | 0 |
| ai_core_entity | done | 3 | 0 | 0 |
| base | done | 3 | 0 | 0 |
| bbs | done | 9 | 0 | 0 |
| bbs_assemble_control | done | 57 | 0 | 0 |
| bbs_core_entity | done | 15 | 0 | 0 |
| component | done | 3 | 0 | 0 |
| component_assemble_control | done | 10 | 0 | 0 |
| component_core_entity | done | 3 | 0 | 0 |
| console | done | 7 | 0 | 0 |
| correlation | done | 3 | 0 | 0 |
| correlation_core_entity | done | 4 | 0 | 0 |
| correlation_core_express | done | 2 | 0 | 0 |
| correlation_service_processing | done | 20 | 0 | 0 |
| express | done | 10 | 0 | 0 |
| hotpic | done | 3 | 0 | 0 |
| hotpic_assemble_control | done | 18 | 0 | 0 |
| hotpic_core_entity | done | 5 | 0 | 0 |
| jpush | done | 6 | 0 | 0 |
| jpush_assemble_control | done | 16 | 0 | 0 |
| jpush_core_entity | done | 5 | 0 | 0 |
| mind | done | 10 | 0 | 0 |
| mind_assemble_control | done | 9 | 0 | 0 |
| mind_core_entity | done | 11 | 0 | 0 |
| organization_assemble_authentication | done | 2 | 0 | 0 |
| organization_assemble_control | done | 102 | 0 | 0 |
| organization_assemble_express | done | 4 | 0 | 0 |
| organization_assemble_personal | done | 2 | 0 | 0 |
| organization_core_entity | done | 24 | 0 | 0 |
| organization_core_express | done | 3 | 0 | 0 |
| program_center | done | 205 | 0 | 0 |
| program_center_core_entity | done | 20 | 0 | 0 |

## 说明

- `done`：无 stub_ / Value::Null 桩，真实 handler 已暴露。
- `doing`：已有真实 handler 但仍有桩标记待清除或 router 未完全暴露。
- `todo`：仅有占位 handler，尚未真实化。
- 基础设施 crate（mcp_server、shared、ldap、orm）排除在外，不统计。
- 回滚/灰度见 deploy/nginx.conf、deploy/rollback-playbook.md、deploy/toggle_module.sh。
