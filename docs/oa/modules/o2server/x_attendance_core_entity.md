# o2server

## Responsibility

考勤核心实体模块，定义打卡记录、排班规则等考勤数据模型。

## Core Classes and Interfaces

- com.x.attendance.entity.AppealConfig
- com.x.attendance.entity.AttendanceAdmin
- com.x.attendance.entity.AttendanceAdmin_
- com.x.attendance.entity.AttendanceAppealAuditInfo
- com.x.attendance.entity.AttendanceAppealAuditInfo_
- com.x.attendance.entity.AttendanceAppealInfo
- com.x.attendance.entity.AttendanceAppealInfo_
- com.x.attendance.entity.AttendanceDetail
- com.x.attendance.entity.AttendanceDetailMobile
- com.x.attendance.entity.AttendanceDetailMobile_

## Key Flows

- 记录列表：`GET /jaxrs/attendance/core/entity/record/list` → `record_list` 查 `x_attendance_record`（sea-orm，CreateTime 倒序 limit 20），输出 id/userId/checkInTime/status，checkOutTime 存在时附加
- 记录创建：`POST .../record/create` → `record_create` 从 payload 取 userId（键名为带引号的 `"\"userId\""`）/checkInTime/status（默认 normal）→ uuid v4 生成 id → ActiveModel INSERT 后 find_by_id 回读返回
- 记录更新：`POST .../record/{id}/update` → `record_update` 先查原记录（无则 NotFound），仅覆盖 checkOutTime 与 status（缺省沿用旧值）后 update 回读
- 记录删除：`GET .../record/{id}/delete` → `record_delete` delete_by_id，rows_affected=0 时返回 error("attendance record not found")，否则 `{id, deleted}`
- 规则 CRUD：`rule_list`（Name 升序 limit 20）、`rule_create`/`rule_update`（startTime 键为 `"\"startTime\""`，endTime 兼容 `"\"endTime\""` 与 "EndTime" 两种键）、`rule_delete`，模式与记录一致
- 路由注册：`attendance_core_entity_router(_pool)` 挂 8 条路由（record/rule 各 list/create/{id}/update/{id}/delete）；routes.rs 委托回 lib.rs

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/attendance_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
