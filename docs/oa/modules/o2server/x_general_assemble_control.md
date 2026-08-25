# o2server

## Responsibility

通用管控模块，处理参会范围、区域管理、发票和二维码等综合配置。

## Core Classes and Interfaces

- com.x.general.assemble.control.AbstractFactory
- com.x.general.assemble.control.ApplicationServletContextListener
- com.x.general.assemble.control.Business
- com.x.general.assemble.control.ThisApplication
- com.x.general.assemble.control.jaxrs.ActionApplication
- com.x.general.assemble.control.jaxrs.AreaJaxrsFilter
- com.x.general.assemble.control.jaxrs.EcnetJaxrsFilter
- com.x.general.assemble.control.jaxrs.ExcelJaxrsFilter
- com.x.general.assemble.control.jaxrs.FileJaxrsFilter
- com.x.general.assemble.control.jaxrs.InvoiceJaxrsFilter

## Key Flows

- 工作日历计算：`worktime/*` 系列端点对 `x_general_assemble_worktime` 做聚合查询——节假日 COUNT、工作分钟 SUM(is_worktime)、按日期区间/顺推天数列出记录
- 区域与参会范围：`area/*` 对 `x_general_assemble_area` 做省→市→区三级 CRUD；`attendscope/*` 维护 `x_general_attend_scope`
- 综合业务配置：`invoice/*` 上传/分页/状态更新 `x_general_assemble_invoice`；excel/generalfile/office 分别落 `x_general_assemble_excel*`、`x_general_assemble_general_file`、`x_general_assemble_office`；`status/update` 与 `permissions/{module}` 读写 `x_general_assemble_control_config`/`x_general_assemble_control_permission`；qrcode 生成存 `x_general_assemble_qrcode`

## Dependencies



- x_base_core_project
- x_organization_core_entity
- x_organization_core_express
- x_general_core_entity
- x_processplatform_core_entity

**Rust（oa4rust/crates/general_assemble_control）：**

- 内部 path 依赖：shared
- 关键外部依赖：axum、tokio、deadpool-postgres、serde/serde_json、uuid、tower

## REST Endpoints



- `POST /jaxrs/general/assemble/control/area/create`
- `POST /jaxrs/general/assemble/control/area/delete/{id}`
- `GET /jaxrs/general/assemble/control/area/list`
- `GET /jaxrs/general/assemble/control/area/list/province/{province}`
- `GET /jaxrs/general/assemble/control/area/list/province/{province}/city/{city}`
- `GET /jaxrs/general/assemble/control/area/list/province/{province}/city/{city}/district/{district}`
- `POST /jaxrs/general/assemble/control/area/update/{id}`
- `GET /jaxrs/general/assemble/control/area/{id}`
- `POST /jaxrs/general/assemble/control/attendscope/create`
- `POST /jaxrs/general/assemble/control/attendscope/delete/{id}`
- `GET /jaxrs/general/assemble/control/attendscope/list`
- `POST /jaxrs/general/assemble/control/attendscope/save/{id}`
- `GET /jaxrs/general/assemble/control/attendscope/{id}`
- `GET /jaxrs/general/assemble/control/ecnet/check`
- `GET /jaxrs/general/assemble/control/excel/result/flag/{flag}`
- `POST /jaxrs/general/assemble/control/excel/upload`
- `POST /jaxrs/general/assemble/control/excel/upload/with/url`
- `GET /jaxrs/general/assemble/control/excel/{excelName}`
- `GET /jaxrs/general/assemble/control/excel/{excelName}/sheetList`
- `GET /jaxrs/general/assemble/control/generalfile/download/flag/{flag}`
- `GET /jaxrs/general/assemble/control/generalfile/flag/{flag}`
- `GET /jaxrs/general/assemble/control/generalfile/flag/{flag}/binary/base64`
- `POST /jaxrs/general/assemble/control/invoice/create`
- `POST /jaxrs/general/assemble/control/invoice/delete/{id}`
- `GET /jaxrs/general/assemble/control/invoice/download/flag/{flag}`
- `GET /jaxrs/general/assemble/control/invoice/get/{id}`
- `GET /jaxrs/general/assemble/control/invoice/list/paging/{page}/size/{size}`
- `POST /jaxrs/general/assemble/control/invoice/update/apply/status/{id}`
- `POST /jaxrs/general/assemble/control/invoice/update/{id}`
- `POST /jaxrs/general/assemble/control/invoice/upload`
- `POST /jaxrs/general/assemble/control/invoice/upload/for/create`
- `POST /jaxrs/general/assemble/control/invoice/upload/with/url`
- `POST /jaxrs/general/assemble/control/office/html/to/word`
- `GET /jaxrs/general/assemble/control/office/html/to/word/result/flag/{flag}`
- `GET /jaxrs/general/assemble/control/permissions/{module}`
- `POST /jaxrs/general/assemble/control/qrcode/delete/{id}`
- `GET /jaxrs/general/assemble/control/qrcode/list`
- `POST /jaxrs/general/assemble/control/qrcode/width/{width}/height/{height}/text/{text}`
- `GET /jaxrs/general/assemble/control/qrcode/{id}`
- `POST /jaxrs/general/assemble/control/securityclearance/create`
- `POST /jaxrs/general/assemble/control/securityclearance/delete/{id}`
- `POST /jaxrs/general/assemble/control/securityclearance/enable`
- `GET /jaxrs/general/assemble/control/securityclearance/object`
- `GET /jaxrs/general/assemble/control/securityclearance/subject`
- `GET /jaxrs/general/assemble/control/securityclearance/system`
- `POST /jaxrs/general/assemble/control/securityclearance/update/{id}`
- `GET /jaxrs/general/assemble/control/securityclearance/{id}`
- `GET /jaxrs/general/assemble/control/status`
- `POST /jaxrs/general/assemble/control/status/update`
- `GET /jaxrs/general/assemble/control/upgrade/2021090901`
- `GET /jaxrs/general/assemble/control/upgrade/2021090902`
- `GET /jaxrs/general/assemble/control/worktime/between/holiday/count/start/{startDate}/end/{endDate}`
- `GET /jaxrs/general/assemble/control/worktime/between/minutes/start/{start}/end/{end}`
- `GET /jaxrs/general/assemble/control/worktime/forward/days/start/{start}/days/{days}`
- `GET /jaxrs/general/assemble/control/worktime/forward/minutes/start/{start}/minutes/{minutes}`
- `GET /jaxrs/general/assemble/control/worktime/indefined/holiday/{date}`
- `GET /jaxrs/general/assemble/control/worktime/indefined/workday/{date}`
- `GET /jaxrs/general/assemble/control/worktime/is/holiday/{date}`
- `GET /jaxrs/general/assemble/control/worktime/is/workday/{date}`
- `GET /jaxrs/general/assemble/control/worktime/is/worktime/{date}`
- `GET /jaxrs/general/assemble/control/worktime/minutes/of/workday`
