# o2server

## Responsibility

考勤管控模块，处理考勤规则、排班管理和申诉审批流程。

## Core Classes and Interfaces

- com.x.attendance.assemble.common.date.DateOperation
- com.x.attendance.assemble.control.AbstractFactory
- com.x.attendance.assemble.control.ApplicationServletContextListener
- com.x.attendance.assemble.control.Business
- com.x.attendance.assemble.control.CacheUtil
- com.x.attendance.assemble.control.CriteriaQueryTools
- com.x.attendance.assemble.control.ExceptionDingdingFindNoArgumentError
- com.x.attendance.assemble.control.ExceptionDingDingRequest
- com.x.attendance.assemble.control.ExceptionPersonHasNoIdentity
- com.x.attendance.assemble.control.ExceptionQywxFindNoArgumentError

## Dependencies



- x_base_core_project
- x_attendance_core_entity
- x_organization_core_express
- x_processplatform_core_entity
- x_general_core_entity

## REST Endpoints



- `GET /jaxrs/attendance/assemble/control/attendanceadmin/list/all`
- `GET /jaxrs/attendance/assemble/control/attendanceadmin/{id}`
- `POST /jaxrs/attendance/assemble/control/attendanceappealInfo/appeal/{id}`
- `POST /jaxrs/attendance/assemble/control/attendanceappealInfo/archive/{id}`
- `POST /jaxrs/attendance/assemble/control/attendanceappealInfo/audit`
- `POST /jaxrs/attendance/assemble/control/attendanceappealInfo/check`
- `GET /jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/next/{count}`
- `GET /jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/prev/{count}`
- `GET /jaxrs/attendance/assemble/control/attendanceappealInfo/manager/list/{id}/next/{count}`
- `POST /jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/appeal/{id}`
- `POST /jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync`
- `GET /jaxrs/attendance/assemble/control/attendanceappealInfo/{id}`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/analyse`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/id/{id}`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/redo`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/{startDate}/{endDate}`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/archive/{id}`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/{cycleYear}/{cycleMonth}`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/topUnit`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/unit`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/user`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/next/{count}`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/prev/{count}`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/list/persons/nonesign`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/list/{file_id}`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/mobile/filter/list/page/{page}/count/{count}`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/mobile/mobilepreview`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/mobile/my`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/mobile/recive`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/mobile/{id}`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/recive`
- `POST /jaxrs/attendance/assemble/control/attendancedetail/reciveSingle`
- `GET /jaxrs/attendance/assemble/control/attendancedetail/{id}`
- `GET /jaxrs/attendance/assemble/control/attendanceemployeeconfig/list/all`
- `GET /jaxrs/attendance/assemble/control/attendanceemployeeconfig/{id}`
- `GET /jaxrs/attendance/assemble/control/attendanceimportfileinfo/list/all`
- `GET /jaxrs/attendance/assemble/control/attendanceimportfileinfo/{id}`
- `GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/all`
- `GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/topUnit/{name}`
- `GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/unit/{name}`
- `GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/{id}`
- `GET /jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/next/{count}`
- `GET /jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/prev/{count}`
- `GET /jaxrs/attendance/assemble/control/attendanceselfholiday/list/all`
- `GET /jaxrs/attendance/assemble/control/attendanceselfholiday/{id}`
- `GET /jaxrs/attendance/assemble/control/attendancesetting/code/{code}`
- `POST /jaxrs/attendance/assemble/control/attendancesetting/enable/type`
- `GET /jaxrs/attendance/assemble/control/attendancesetting/list/all`
- `GET /jaxrs/attendance/assemble/control/attendancesetting/{id}`
- `GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/cycleDetail/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/list/all`
- `GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/{id}`
- `GET /jaxrs/attendance/assemble/control/attendancestatisticrequirelog/list/all`
- `GET /jaxrs/attendance/assemble/control/attendancestatisticrequirelog/{id}`
- `POST /jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter`
- `GET /jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all`
- `GET /jaxrs/attendance/assemble/control/attendanceworkdayconfig/{id}`
- `GET /jaxrs/attendance/assemble/control/rule/list`
- `POST /jaxrs/attendance/assemble/control/rule/{id}/toggle`
- `GET /jaxrs/attendance/assemble/control/selfholidaysimple/docId/{docId}`
- `POST /jaxrs/attendance/assemble/control/statistic/do`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/next/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/prev/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/next/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/prev/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/next/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/prev/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/next/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/prev/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/next/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/prev/{count}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/person/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/persons/unit/subnested/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/persons/unit/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/topUnit/day/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/topUnit/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/topUnit/{name}/{date}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{date}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/unit/subnested/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/unit/sum/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/unit/topUnit/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/statisticshow/unit/{name}/{year}/{month}`
- `GET /jaxrs/attendance/assemble/control/uuid/random`
- `GET /jaxrs/attendance/assemble/control/workplace/list/all`
- `GET /jaxrs/attendance/assemble/control/workplace/{id}`
