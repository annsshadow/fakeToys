# o2server

## Responsibility

推送核心实体模块，定义设备和模板等推送数据模型。

## Core Classes and Interfaces

- com.x.jpush.core.entity.PersistenceProperties
- com.x.jpush.core.entity.PushDevice
- com.x.jpush.core.entity.PushDevice_

## Key Flows

- 设备列表：`GET /jaxrs/jpush/core/entity/device/list` → `device_list` 查 `x_jpush_device`，CreateTime 倒序 limit 20，输出 id/userId/platform/token（userId 键名为带引号的 `"\"userId\""`）
- 设备详情：`GET .../device/{id}` → find_by_id 无则返回 AppError::NotFound（非 ActionResult error）
- 设备创建：`POST .../device/create` → uuid v4、字段缺省空串、create_time=NotSet；请求体 userId 键名同样为带引号的 `"\"userId\""`
- 模板列表/详情：`GET .../template/list` Name 升序 limit 20；`GET .../template/{id}` 无则 AppError::NotFound
- 路由注册：`jpush_core_entity_router(_pool)` 挂 device 3 条 + template 2 条共 5 条路由

## Dependencies



- x_base_core_project

**Rust（oa4rust/crates/jpush_core_entity）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、sea-orm、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints

<!-- Generated from Swagger annotations or action JSON files. Omit this section if the module has no REST endpoints. -->

- [Endpoint list]
