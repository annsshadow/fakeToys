# o2server

## Responsibility

推送管控模块，处理推送配置、应用和消息管理。

## Core Classes and Interfaces

- com.x.jpush.assemble.control.AbstractFactory
- com.x.jpush.assemble.control.ApplicationServletContextListener
- com.x.jpush.assemble.control.Business
- com.x.jpush.assemble.control.JpushConst
- com.x.jpush.assemble.control.MimeTypeDefinition
- com.x.jpush.assemble.control.ThisApplication
- com.x.jpush.assemble.control.factory.PushDeviceFactory
- com.x.jpush.assemble.control.huawei.CollectionUtils
- com.x.jpush.assemble.control.huawei.ValidatorUtils
- com.x.jpush.assemble.control.huawei.android.AndroidConfig

## Key Flows

- 设备绑定管理：`device/bind` INSERT `x_jpush`（title=设备名/content=设备类型/target=推送渠道）；`device/unbind*` 按 title/content(/target) 软删；`admin/unbind/all/person` 按 creator 物理删除
- 消息推送网关：`message/test/send` 取最近 10 条记录经 `PushGateway` trait 发送，提供 Mock/Console/JPush 三实现——JPushGateway 以 Basic 认证 POST `https://api.jpush.cn/v3/push`（ios+android 通知，alias 定向）
- 推送配置与应用：`jpush/create|save|delete|list|get` 维护消息记录；`list/control/apps` 按 target GROUP BY 派生应用列表；config 由记录数推导 enabled

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_jpush_core_entity
- jiguang-sdk

**Rust（oa4rust/crates/jpush_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、async-trait、reqwest（json + rustls-tls）、base64、chrono、uuid、futures-util

## REST Endpoints



- `GET /jaxrs/jpush/assemble/control/config`
- `GET /jaxrs/jpush/assemble/control/device/admin/unbind/all/person`
- `POST /jaxrs/jpush/assemble/control/device/bind`
- `GET /jaxrs/jpush/assemble/control/device/check/deviceName/deviceType/pushType`
- `GET /jaxrs/jpush/assemble/control/device/config/push/type`
- `GET /jaxrs/jpush/assemble/control/device/list/pushType`
- `POST /jaxrs/jpush/assemble/control/device/unbind/deviceName/deviceType`
- `POST /jaxrs/jpush/assemble/control/device/unbind/new/deviceName/deviceType/pushType`
- `GET /jaxrs/jpush/assemble/control/list/control/apps`
- `GET /jaxrs/jpush/assemble/control/message/test/send`
- `POST /jaxrs/jpush/assemble/control/update/control/config`
- `POST /jaxrs/jpush/create`
- `POST /jaxrs/jpush/delete/{id}`
- `GET /jaxrs/jpush/get/{id}`
- `GET /jaxrs/jpush/list`
- `POST /jaxrs/jpush/save/{id}`
- `GET /jaxrs/jpush_assemble_control/create/jpush`
- `GET /jaxrs/jpush_assemble_control/delete/jpush`
- `GET /jaxrs/jpush_assemble_control/device/admin/unbind/all/person`
- `GET /jaxrs/jpush_assemble_control/device/bind`
- `GET /jaxrs/jpush_assemble_control/device/check/deviceName/deviceType/pushType`
- `GET /jaxrs/jpush_assemble_control/device/config/push/type`
- `GET /jaxrs/jpush_assemble_control/device/list/pushType`
- `GET /jaxrs/jpush_assemble_control/device/unbind/deviceName/deviceType`
- `GET /jaxrs/jpush_assemble_control/device/unbind/new/deviceName/deviceType/pushType`
- `GET /jaxrs/jpush_assemble_control/get/control/config`
- `GET /jaxrs/jpush_assemble_control/get/jpush`
- `GET /jaxrs/jpush_assemble_control/list/control/apps`
- `GET /jaxrs/jpush_assemble_control/list/jpushs`
- `GET /jaxrs/jpush_assemble_control/message/test/send`
- `GET /jaxrs/jpush_assemble_control/save/jpush`
- `GET /jaxrs/jpush_assemble_control/update/control/config`
