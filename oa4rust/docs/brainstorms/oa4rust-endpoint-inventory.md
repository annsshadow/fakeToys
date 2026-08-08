# OA4Rust 端点清单与实现状态 (Endpoint Inventory)

> 由 scripts/gen_inventory.py 基于源码静态扫描自动生成，每次真实化后重新运行。
> 字段：routes=路由注册数, handlers=真实 handler 数, stub_=`stub_` 标记数,
> null=纯 Value::Null 桩数, todo=TODO 占位数, status=done/doing/todo。

| crate | wave | routes | handlers | stub_ | null | todo | status |
|-------|------|-------:|---------:|------:|-----:|-----:|:------|
| auth | U1-U4 | 22 | 24 | 0 | 0 | 0 | done |
| control | U1-U4 | 25 | 25 | 0 | 0 | 0 | done |
| personal | U1-U4 | 8 | 7 | 0 | 0 | 0 | done |
| personal_extend | U1-U4 | 6 | 6 | 0 | 0 | 0 | done |
| program_init | U1-U4 | 3 | 3 | 0 | 0 | 0 | done |
| attendance | U5 | 9 | 9 | 0 | 0 | 0 | done |
| attendance_assemble_control | U5 | 87 | 89 | 0 | 0 | 0 | done |
| attendance_core_entity | U5 | 8 | 8 | 0 | 0 | 0 | done |
| calendar | U5 | 10 | 0 | 0 | 0 | 0 | todo |
| calendar_assemble_control | U5 | 4 | 4 | 0 | 0 | 0 | done |
| calendar_core_entity | U5 | 10 | 10 | 0 | 0 | 0 | done |
| file | U5 | 9 | 10 | 0 | 0 | 0 | done |
| file_assemble_control | U5 | 8 | 94 | 0 | 0 | 0 | done |
| file_core_entity | U5 | 4 | 4 | 0 | 0 | 0 | done |
| general | U5 | 3 | 3 | 0 | 0 | 0 | done |
| general_assemble_control | U5 | 61 | 61 | 0 | 0 | 0 | done |
| general_core_entity | U5 | 21 | 21 | 0 | 0 | 0 | done |
| cms_assemble_control | U6 | 3 | 310 | 0 | 0 | 0 | done |
| cms_control | U6 | 2 | 2 | 0 | 0 | 0 | done |
| cms_core_entity | U6 | 6 | 6 | 0 | 0 | 0 | done |
| cms_core_express | U6 | 2 | 2 | 0 | 0 | 0 | done |
| cms_express | U6 | 3 | 3 | 0 | 0 | 0 | done |
| meeting | U6 | 9 | 9 | 0 | 0 | 0 | done |
| meeting_assemble_control | U6 | 42 | 59 | 0 | 0 | 0 | done |
| meeting_core_entity | U6 | 11 | 11 | 0 | 0 | 0 | done |
| message | U6 | 5 | 5 | 0 | 0 | 0 | done |
| message_assemble_communicate | U6 | 58 | 58 | 0 | 0 | 0 | done |
| message_core_entity | U6 | 3 | 3 | 0 | 0 | 0 | done |
| portal | U6 | 8 | 11 | 0 | 0 | 0 | done |
| portal_assemble_designer | U6 | 12 | 56 | 0 | 0 | 0 | done |
| portal_assemble_surface | U6 | 8 | 48 | 0 | 0 | 0 | done |
| portal_core_entity | U6 | 8 | 8 | 0 | 0 | 0 | done |
| process_designer | U6 | 7 | 5 | 0 | 0 | 0 | done |
| process_express | U6 | 3 | 0 | 0 | 0 | 0 | todo |
| process_surface | U6 | 3 | 0 | 0 | 0 | 0 | todo |
| processplatform_assemble_bam | U6 | 5 | 50 | 0 | 0 | 0 | done |
| processplatform_assemble_designer | U6 | 6 | 96 | 0 | 0 | 0 | done |
| processplatform_assemble_surface | U6 | 7 | 487 | 0 | 0 | 0 | done |
| processplatform_core_entity | U6 | 6 | 6 | 0 | 0 | 0 | done |
| processplatform_core_express | U6 | 6 | 6 | 0 | 0 | 0 | done |
| processplatform_service_processing | U6 | 6 | 98 | 0 | 0 | 0 | done |
| query_assemble_designer | U6 | 5 | 67 | 0 | 0 | 0 | done |
| query_assemble_surface | U6 | 6 | 59 | 0 | 0 | 0 | done |
| query_core_entity | U6 | 5 | 5 | 0 | 0 | 0 | done |
| query_core_express | U6 | 4 | 4 | 0 | 0 | 0 | done |
| query_express | U6 | 1 | 1 | 0 | 0 | 0 | done |
| query_service | U6 | 3 | 3 | 0 | 0 | 0 | done |
| query_service_processing | U6 | 4 | 4 | 0 | 0 | 0 | done |
| ai | U7 | 21 | 21 | 0 | 0 | 0 | done |
| ai_assemble_control | U7 | 36 | 31 | 0 | 0 | 0 | done |
| ai_core_entity | U7 | 3 | 3 | 0 | 0 | 0 | done |
| base | U7 | 3 | 2 | 0 | 0 | 0 | done |
| bbs | U7 | 18 | 9 | 0 | 0 | 0 | done |
| bbs_assemble_control | U7 | 17 | 57 | 0 | 0 | 0 | done |
| bbs_core_entity | U7 | 15 | 15 | 0 | 0 | 0 | done |
| component | U7 | 3 | 3 | 0 | 0 | 0 | done |
| component_assemble_control | U7 | 10 | 10 | 0 | 0 | 0 | done |
| component_core_entity | U7 | 3 | 3 | 0 | 0 | 0 | done |
| console | U7 | 7 | 7 | 0 | 0 | 0 | done |
| correlation | U7 | 3 | 3 | 0 | 0 | 0 | done |
| correlation_core_entity | U7 | 2 | 2 | 0 | 0 | 0 | done |
| correlation_core_express | U7 | 2 | 2 | 0 | 0 | 0 | done |
| correlation_service_processing | U7 | 20 | 20 | 0 | 0 | 0 | done |
| express | U7 | 3 | 3 | 0 | 0 | 0 | done |
| hotpic | U7 | 3 | 3 | 0 | 0 | 0 | done |
| hotpic_assemble_control | U7 | 36 | 18 | 0 | 0 | 0 | done |
| hotpic_core_entity | U7 | 3 | 3 | 0 | 0 | 0 | done |
| jpush | U7 | 6 | 6 | 0 | 0 | 0 | done |
| jpush_assemble_control | U7 | 32 | 16 | 0 | 0 | 0 | done |
| jpush_core_entity | U7 | 5 | 5 | 0 | 0 | 0 | done |
| mind | U7 | 10 | 10 | 0 | 0 | 0 | done |
| mind_assemble_control | U7 | 8 | 9 | 0 | 0 | 0 | done |
| mind_core_entity | U7 | 10 | 10 | 0 | 0 | 0 | done |
| organization_assemble_control | U7 | 101 | 101 | 0 | 0 | 0 | done |
| organization_assemble_express | U7 | 4 | 4 | 0 | 0 | 0 | done |
| organization_core_entity | U7 | 6 | 6 | 0 | 0 | 0 | done |
| organization_core_express | U7 | 3 | 3 | 0 | 0 | 0 | done |
| program_center | U7 | 7 | 205 | 0 | 0 | 0 | done |
| program_center_core_entity | U7 | 7 | 5 | 0 | 0 | 0 | done |
| mcp_server | U? | 2 | 0 | 0 | 0 | 0 | todo |
| openapi | U? | 0 | 0 | 0 | 0 | 0 | todo |
| process_bam | U? | 3 | 3 | 0 | 0 | 0 | done |
| shared | U? | 11 | 0 | 0 | 0 | 0 | todo |

## 汇总

- crate 总数: 83
- 已完成(done): 77 | 迁移中(doing): 0 | 待迁移(todo): 6
- 真实 handler 总数: 2458
- 残留 stub_ 标记: 0 | 纯 Value::Null 桩: 0 | TODO 占位: 0
