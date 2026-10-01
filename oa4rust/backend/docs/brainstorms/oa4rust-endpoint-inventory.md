# OA4Rust 端点清单与实现状态 (Endpoint Inventory)

> 由 scripts/gen_inventory.py 基于源码静态扫描自动生成，每次真实化后重新运行。
> 字段：routes=路由注册数, handlers=真实 handler 数, stub_=`stub_` 标记数,
> null=纯 Value::Null 桩数, todo=TODO 占位数, db_touch=含DB调用的handler数, db_touch_rate=DB真实化率%, status=done/doing/todo。

| crate | wave | routes | handlers | stub_ | null | todo | db_touch | db_touch_rate |
|-------|------|-------:|---------:|------:|-----:|-----:|---------:|:--------------|
| auth | U1-U4 | 46 | 51 | 0 | 0 | 0 | 9 | 17.6% |
| control | U1-U4 | 25 | 25 | 0 | 0 | 0 | 1 | 4.0% |
| personal | U1-U4 | 19 | 22 | 0 | 0 | 0 | 6 | 27.3% |
| personal_extend | U1-U4 | 6 | 12 | 0 | 0 | 0 | 4 | 33.3% |
| program_init | U1-U4 | 3 | 3 | 0 | 0 | 0 | 2 | 66.7% |
| attendance | U5 | 9 | 9 | 0 | 0 | 0 | 6 | 66.7% |
| attendance_assemble_control | U5 | 87 | 89 | 0 | 0 | 0 | 17 | 19.1% |
| attendance_core_entity | U5 | 8 | 9 | 0 | 0 | 0 | 0 | 0.0% |
| calendar | U5 | 10 | 10 | 0 | 0 | 0 | 2 | 20.0% |
| calendar_assemble_control | U5 | 4 | 4 | 0 | 0 | 0 | 3 | 75.0% |
| calendar_core_entity | U5 | 10 | 10 | 0 | 0 | 0 | 0 | 0.0% |
| file | U5 | 9 | 9 | 0 | 0 | 0 | 3 | 33.3% |
| file_assemble_control | U5 | 12 | 94 | 0 | 0 | 0 | 9 | 9.6% |
| file_core_entity | U5 | 7 | 7 | 0 | 0 | 0 | 0 | 0.0% |
| general | U5 | 3 | 3 | 0 | 0 | 0 | 2 | 66.7% |
| general_assemble_control | U5 | 61 | 61 | 0 | 0 | 0 | 11 | 18.0% |
| general_core_entity | U5 | 21 | 21 | 0 | 0 | 0 | 0 | 0.0% |
| cms_assemble_control | U6 | 42 | 312 | 0 | 0 | 0 | 34 | 10.9% |
| cms_control | U6 | 2 | 2 | 0 | 0 | 0 | 2 | 100.0% |
| cms_core_entity | U6 | 6 | 7 | 0 | 0 | 0 | 0 | 0.0% |
| cms_core_express | U6 | 2 | 2 | 0 | 0 | 0 | 1 | 50.0% |
| cms_express | U6 | 5 | 5 | 0 | 0 | 0 | 1 | 20.0% |
| meeting | U6 | 9 | 9 | 0 | 0 | 0 | 4 | 44.4% |
| meeting_assemble_control | U6 | 42 | 59 | 0 | 0 | 0 | 13 | 22.0% |
| meeting_core_entity | U6 | 11 | 11 | 0 | 0 | 0 | 0 | 0.0% |
| message | U6 | 5 | 5 | 0 | 0 | 0 | 0 | 0.0% |
| message_assemble_communicate | U6 | 58 | 58 | 0 | 0 | 0 | 6 | 10.3% |
| message_core_entity | U6 | 3 | 3 | 0 | 0 | 0 | 0 | 0.0% |
| portal | U6 | 7 | 11 | 0 | 0 | 0 | 6 | 54.5% |
| portal_assemble_designer | U6 | 12 | 56 | 0 | 0 | 0 | 11 | 19.6% |
| portal_assemble_surface | U6 | 8 | 48 | 0 | 0 | 0 | 3 | 6.2% |
| portal_core_entity | U6 | 8 | 8 | 0 | 0 | 0 | 0 | 0.0% |
| process_designer | U6 | 7 | 7 | 0 | 0 | 0 | 2 | 28.6% |
| process_express | U6 | 3 | 3 | 0 | 0 | 0 | 1 | 33.3% |
| process_surface | U6 | 3 | 3 | 0 | 0 | 0 | 1 | 33.3% |
| processplatform_assemble_bam | U6 | 5 | 50 | 0 | 0 | 0 | 5 | 10.0% |
| processplatform_assemble_designer | U6 | 6 | 96 | 0 | 0 | 0 | 13 | 13.5% |
| processplatform_assemble_surface | U6 | 7 | 487 | 0 | 0 | 0 | 27 | 5.5% |
| processplatform_core_entity | U6 | 6 | 6 | 0 | 0 | 0 | 0 | 0.0% |
| processplatform_core_express | U6 | 6 | 6 | 0 | 0 | 0 | 0 | 0.0% |
| processplatform_service_processing | U6 | 21 | 113 | 0 | 0 | 0 | 6 | 5.3% |
| query_assemble_designer | U6 | 5 | 67 | 0 | 0 | 0 | 8 | 11.9% |
| query_assemble_surface | U6 | 9 | 59 | 0 | 0 | 0 | 3 | 5.1% |
| query_core_entity | U6 | 5 | 5 | 0 | 0 | 0 | 3 | 60.0% |
| query_core_express | U6 | 4 | 4 | 0 | 0 | 0 | 0 | 0.0% |
| query_express | U6 | 2 | 2 | 0 | 0 | 0 | 0 | 0.0% |
| query_service | U6 | 3 | 3 | 0 | 0 | 0 | 1 | 33.3% |
| query_service_processing | U6 | 4 | 4 | 0 | 0 | 0 | 2 | 50.0% |
| ai | U7 | 21 | 21 | 0 | 0 | 0 | 7 | 33.3% |
| ai_assemble_control | U7 | 37 | 32 | 0 | 0 | 0 | 8 | 25.0% |
| ai_core_entity | U7 | 3 | 3 | 0 | 0 | 0 | 0 | 0.0% |
| base | U7 | 3 | 3 | 0 | 0 | 0 | 1 | 33.3% |
| bbs | U7 | 18 | 9 | 0 | 0 | 0 | 2 | 22.2% |
| bbs_assemble_control | U7 | 17 | 57 | 0 | 0 | 0 | 17 | 29.8% |
| bbs_core_entity | U7 | 15 | 15 | 0 | 0 | 0 | 0 | 0.0% |
| component | U7 | 3 | 3 | 0 | 0 | 0 | 2 | 66.7% |
| component_assemble_control | U7 | 10 | 10 | 0 | 0 | 0 | 5 | 50.0% |
| component_core_entity | U7 | 3 | 3 | 0 | 0 | 0 | 2 | 66.7% |
| console | U7 | 7 | 7 | 0 | 0 | 0 | 1 | 14.3% |
| correlation | U7 | 3 | 3 | 0 | 0 | 0 | 3 | 100.0% |
| correlation_core_entity | U7 | 4 | 4 | 0 | 0 | 0 | 0 | 0.0% |
| correlation_core_express | U7 | 2 | 2 | 0 | 0 | 0 | 2 | 100.0% |
| correlation_service_processing | U7 | 20 | 20 | 0 | 0 | 0 | 2 | 10.0% |
| express | U7 | 10 | 10 | 0 | 0 | 0 | 1 | 10.0% |
| hotpic | U7 | 3 | 3 | 0 | 0 | 0 | 0 | 0.0% |
| hotpic_assemble_control | U7 | 36 | 18 | 0 | 0 | 0 | 5 | 27.8% |
| hotpic_core_entity | U7 | 5 | 5 | 0 | 0 | 0 | 0 | 0.0% |
| jpush | U7 | 6 | 6 | 0 | 0 | 0 | 2 | 33.3% |
| jpush_assemble_control | U7 | 32 | 16 | 0 | 0 | 0 | 4 | 25.0% |
| jpush_core_entity | U7 | 5 | 5 | 0 | 0 | 0 | 0 | 0.0% |
| mind | U7 | 10 | 10 | 0 | 0 | 0 | 1 | 10.0% |
| mind_assemble_control | U7 | 8 | 9 | 0 | 0 | 0 | 2 | 22.2% |
| mind_core_entity | U7 | 10 | 11 | 0 | 0 | 0 | 0 | 0.0% |
| organization_assemble_authentication | U7 | 11 | 12 | 0 | 0 | 0 | 0 | 0.0% |
| organization_assemble_control | U7 | 105 | 105 | 0 | 0 | 0 | 24 | 22.9% |
| organization_assemble_express | U7 | 4 | 4 | 0 | 0 | 0 | 3 | 75.0% |
| organization_assemble_personal | U7 | 2 | 2 | 0 | 0 | 0 | 0 | 0.0% |
| organization_core_entity | U7 | 18 | 24 | 0 | 0 | 0 | 0 | 0.0% |
| organization_core_express | U7 | 3 | 3 | 0 | 0 | 0 | 3 | 100.0% |
| program_center | U7 | 7 | 205 | 0 | 0 | 0 | 29 | 14.1% |
| program_center_core_entity | U7 | 20 | 22 | 0 | 0 | 0 | 0 | 0.0% |
| captcha_store | U? | 0 | 0 | 0 | 0 | 0 | 0 | 0.0% |
| empower | U? | 14 | 14 | 0 | 0 | 0 | 4 | 28.6% |
| parity | U? | 1 | 0 | 0 | 0 | 0 | 0 | 0.0% |
| preview | U? | 2 | 2 | 0 | 0 | 0 | 0 | 0.0% |
| process_bam | U? | 3 | 3 | 0 | 0 | 0 | 3 | 100.0% |
| realtime | U? | 3 | 8 | 0 | 0 | 0 | 0 | 0.0% |
| search | U? | 0 | 3 | 0 | 0 | 0 | 3 | 100.0% |
| signature | U? | 2 | 2 | 0 | 0 | 0 | 0 | 0.0% |
| sms | U? | 0 | 1 | 0 | 0 | 0 | 0 | 0.0% |

## 汇总

- crate 总数: 90
- 已完成(done): 88 | 迁移中(doing): 0 | 待迁移(todo): 2
- 真实 handler 总数: 2645
- 残留 stub_ 标记: 0 | 纯 Value::Null 桩: 0 | TODO 占位: 0
- DB 调用 handler 数: 364 | 平均 DB 真实化率: 13.8%
