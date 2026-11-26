# R301 D1 第二轮全页真浏览器实测与第一轮量化对照

> 来源声明：数据来自部署在测试服 <YOUR_TEST_DOMAIN> 的真实运行产物，使用 Playwright Chromium 无头浏览器（PC 1440x900、H5 390x844 isMobile/hasTouch）按真实 UI 登录流程逐页访问并截图。截图为**无头浏览器渲染结果，不是物理真机截图**。登录口令仅在运行时由远端凭据读取，不入库、不回显。

> 迭代声明：本项目处于 ≥1000 轮持续迭代中，本轮仅是其中一环，不代表项目完成。

## 1. 实测方法

- 两轮使用**同一路由表**（第一轮 `test-results/r300-c1-full-pc.cjs` 74 条、H5 `r300-c1-full-h5.cjs` 19 条），第二轮脚本从第一轮脚本原文提取路由数组，保证可比性。
- 第二轮脚本：`test-results/r301-d1-full-pc.cjs`、`test-results/r301-d1-full-h5.cjs`（均 gitignored）。
- 每页流程：`goto(domcontentloaded)` → `waitForTimeout(1500)` → `page.evaluate` 采集 DOM 诊断 → `fullPage` 截图；页面错误与 API 失败（400/404/≥500）经事件监听记录。
- 第一轮耗时无逐页埋点，按截图文件 mtime 相邻差值估算；第二轮逐页记录 navMs/shotMs/totalMs。口径不同，仅作量级参考。
- 原始证据（gitignored，不入库）：`test-results/r301-d1-full-pc/`、`test-results/r301-d1-full-h5/`、`test-results/r301-d1-comparison.json`。

## 2. 量化对照总表

| 指标 | 第一轮 R300-C1 | 第二轮 R301-D1 | 变化 |
|---|---|---|---|
| PC 路由覆盖 | 74/74 | 74/74 | 持平 |
| PC 截图数 | 75 | 75 | 持平 |
| PC pageerror | 1 | 0 | 1→0（日历 TDZ 收敛） |
| PC API 失败 | 0 | 0 | 持平为 0 |
| PC 逐页均耗时 | ~1615ms（mtime 估算） | 1626ms（实测，nav 1559/shot 68） | 量级持平 |
| H5 路由覆盖 | 19/19 | 19/19 | 持平 |
| H5 截图数 | 20 | 20 | 持平 |
| H5 pageerror | 0 | 0 | 持平为 0 |
| H5 API 失败 | 0 | 0 | 持平为 0 |
| H5 逐页均耗时 | ~1682ms（mtime 估算） | 1678ms（实测） | 量级持平 |

## 3. 缺陷收敛结论

- **已收敛 P0-1（PC 日历 TDZ）**：第一轮 `/app/calendar` 抛 `Cannot access 'q' before initialization`（1 次 pageerror）；第二轮同页 0 pageerror，正常渲染「工作日历」。
- **已收敛 P0-2（H5 日历信封）**：第一轮 `#/pages/calendar/calendar` 可见「日程加载失败：S.value.map is not a function」（errorText=true）；第二轮同页 errorText=false，显示「我的日历 0 个 / 今日暂无日程」正常空态。
- **已收敛 P1（PC 组织页 Loading 常驻）**：第一轮 `/app/org` loading=true 常驻；第二轮同页 loading=false，显示「暂无组织架构数据，请点击「重试加载」」+ 重试入口（见 `test-results/r301-d1-full-pc/02-org.png`）。注：数据仍为空，但卡死态已消除，根因方向指向组织树接口错误兜底（详见 R301 收敛审计 U-1）。
- **已收敛 P1（PC 工作台待审批口径）**：第一轮 0 待审批；第二轮显示「5 待审批 ↑较昨日」，与待办列表同源（见 `test-results/r301-d1-full-pc/01-dashboard.png`）。
- **已收敛 P2（端点调试侵入/原始 UUID）**：BAM 页端点调试按钮进入折叠区，工作台待办不再展示原始 UUID。
- **持续为 0**：两轮 API 失败均为 0（第二轮监听口径更严，含 400）。

## 4. 分布指标对照（页数为单位）

| 端 | 指标 | 第一轮 | 第二轮 |
|---|---|---|---|
| PC | loading | {"false":71,"true":3} | {"false":72,"true":2} |
| PC | errorText | {"false":74} | {"false":74} |
| PC | emptyState | {"false":60,"true":14} | {"false":60,"true":14} |
| PC | aiPage | {"zero":31,"gt0":43} | {"gt0":45,"zero":29} |
| H5 | errorText | {"false":18,"true":1} | {"false":19} |
| H5 | emptyState | {"true":8,"false":11} | {"true":9,"false":10} |
| H5 | branded | {"gt0":2,"zero":17} | {"gt0":2,"zero":17} |
| H5 | timeoutText | {"undefined":19} | {"false":19} |

说明：PC `aiPage` gt0 页 43→45；H5 `branded` gt0 维持 2（品牌渐变实心填充仅工作台/登录两页），`timeoutText` 全 false（H5 连接服务器超时已消除）。PC loading true 仍有 2 页（19-admin-hotpic、21-admin-appinfo），属次要项。

## 5. 逐页证据索引（截图相对路径，均 gitignored）

### PC（74 页）

| 页 | 路由 | bodyLen 一轮→二轮 | 二轮耗时 | 关键变化 | 截图 |
|---|---|---|---|---|---|
| 01-dashboard | /app/dashboard | 716→704 | 1637ms | aiPage 0->1 | `r301-d1-full-pc/01-dashboard.png` |
| 02-org | /app/org | 255→221 | 1589ms | loading true->false | `r301-d1-full-pc/02-org.png` |
| 03-directory | /app/directory | 365→715 | 1602ms | — | `r301-d1-full-pc/03-directory.png` |
| 04-process | /app/process | 631→631 | 1623ms | — | `r301-d1-full-pc/04-process.png` |
| 05-im | /app/im | 622→445 | 1593ms | — | `r301-d1-full-pc/05-im.png` |
| 06-personal | /app/personal | 379→379 | 1606ms | — | `r301-d1-full-pc/06-personal.png` |
| 07-settings | /app/settings | 325→325 | 1590ms | — | `r301-d1-full-pc/07-settings.png` |
| 08-calendar | /app/calendar | 376→350 | 1603ms | — | `r301-d1-full-pc/08-calendar.png` |
| 09-file | /app/file | 288→356 | 1612ms | — | `r301-d1-full-pc/09-file.png` |
| 10-bbs | /app/bbs | 309→557 | 1612ms | — | `r301-d1-full-pc/10-bbs.png` |
| 11-meeting | /app/meeting | 456→456 | 1615ms | — | `r301-d1-full-pc/11-meeting.png` |
| 12-attendance | /app/attendance | 492→479 | 1635ms | — | `r301-d1-full-pc/12-attendance.png` |
| 13-recycle | /app/recycle | 264→334 | 1595ms | — | `r301-d1-full-pc/13-recycle.png` |
| 14-bam | /app/bam | 270→249 | 1600ms | — | `r301-d1-full-pc/14-bam.png` |
| 15-note | /app/note | 247→238 | 1602ms | — | `r301-d1-full-pc/15-note.png` |
| 16-search | /app/search | 226→226 | 1589ms | — | `r301-d1-full-pc/16-search.png` |
| 17-admin-query | /app/admin/query | 1296→1296 | 1607ms | — | `r301-d1-full-pc/17-admin-query.png` |
| 18-admin-portal | /app/admin/portal | 878→878 | 1617ms | — | `r301-d1-full-pc/18-admin-portal.png` |
| 19-admin-hotpic | /app/admin/hotpic | 865→876 | 1619ms | — | `r301-d1-full-pc/19-admin-hotpic.png` |
| 20-admin-jpush | /app/admin/jpush | 918→918 | 1614ms | — | `r301-d1-full-pc/20-admin-jpush.png` |
| 21-admin-appinfo | /app/admin/appinfo | 860→871 | 1616ms | — | `r301-d1-full-pc/21-admin-appinfo.png` |
| 22-admin-category | /app/admin/category | 851→862 | 1850ms | — | `r301-d1-full-pc/22-admin-category.png` |
| 23-admin-document | /app/admin/document | 833→844 | 2097ms | — | `r301-d1-full-pc/23-admin-document.png` |
| 24-admin-program | /app/admin/program | 917→917 | 1619ms | — | `r301-d1-full-pc/24-admin-program.png` |
| 25-admin-queryview | /app/admin/queryview | 834→834 | 1615ms | — | `r301-d1-full-pc/25-admin-queryview.png` |
| 26-admin-server | /app/admin/server | 899→899 | 1620ms | — | `r301-d1-full-pc/26-admin-server.png` |
| 27-admin-form | /app/admin/form | 1449→1449 | 1606ms | — | `r301-d1-full-pc/27-admin-form.png` |
| 28-admin-view | /app/admin/view | 832→832 | 1637ms | — | `r301-d1-full-pc/28-admin-view.png` |
| 29-admin-fileinfo | /app/admin/fileinfo | 830→830 | 1616ms | — | `r301-d1-full-pc/29-admin-fileinfo.png` |
| 30-admin-role | /app/admin/role | 1037→1047 | 1647ms | — | `r301-d1-full-pc/30-admin-role.png` |
| 31-admin-process-designer | /app/admin/process-designer | 1893→1893 | 1622ms | — | `r301-d1-full-pc/31-admin-process-designer.png` |
| 32-admin-form-designer | /app/admin/form-designer | 1756→1756 | 1627ms | — | `r301-d1-full-pc/32-admin-form-designer.png` |
| 33-admin-query-statement-designer | /app/admin/query-statement-designer | 1558→1558 | 1647ms | — | `r301-d1-full-pc/33-admin-query-statement-designer.png` |
| 34-admin-query-designer | /app/admin/query-designer | 1311→1311 | 1618ms | — | `r301-d1-full-pc/34-admin-query-designer.png` |
| 35-admin-query-manager-deep | /app/admin/query-manager-deep | 1623→1623 | 1600ms | — | `r301-d1-full-pc/35-admin-query-manager-deep.png` |
| 36-admin-portal-designer | /app/admin/portal-designer | 1274→1274 | 1613ms | — | `r301-d1-full-pc/36-admin-portal-designer.png` |
| 37-admin-log-viewer | /app/admin/log-viewer | 831→831 | 1624ms | — | `r301-d1-full-pc/37-admin-log-viewer.png` |
| 38-admin-query-query | /app/admin/query-query | 2431→2442 | 1609ms | — | `r301-d1-full-pc/38-admin-query-query.png` |
| 39-admin-query-explorer | /app/admin/query-explorer | 836→836 | 1624ms | — | `r301-d1-full-pc/39-admin-query-explorer.png` |
| 40-admin-query-table-designer | /app/admin/query-table-designer | 979→979 | 1611ms | — | `r301-d1-full-pc/40-admin-query-table-designer.png` |
| 41-admin-query-view-designer | /app/admin/query-view-designer | 836→836 | 1635ms | — | `r301-d1-full-pc/41-admin-query-view-designer.png` |
| 42-admin-query-stat-designer | /app/admin/query-stat-designer | 877→888 | 1610ms | — | `r301-d1-full-pc/42-admin-query-stat-designer.png` |
| 43-admin-query-importer-designer | /app/admin/query-importer-designer | 877→888 | 1614ms | — | `r301-d1-full-pc/43-admin-query-importer-designer.png` |
| 44-admin-cms-column | /app/admin/cms-column | 831→831 | 1629ms | — | `r301-d1-full-pc/44-admin-cms-column.png` |
| 45-admin-cms-column-manager | /app/admin/cms-column-manager | 831→831 | 1627ms | — | `r301-d1-full-pc/45-admin-cms-column-manager.png` |
| 46-admin-cms-index | /app/admin/cms-index | 869→857 | 1617ms | aiPage 2->3 | `r301-d1-full-pc/46-admin-cms-index.png` |
| 47-admin-process-task-center | /app/admin/process-task-center | 3535→4462 | 1641ms | — | `r301-d1-full-pc/47-admin-process-task-center.png` |
| 48-admin-process-manager | /app/admin/process-manager | 1865→1850 | 1620ms | aiPage 3->4 | `r301-d1-full-pc/48-admin-process-manager.png` |
| 49-admin-cms-dict-designer | /app/admin/cms-dict-designer | 843→843 | 1621ms | — | `r301-d1-full-pc/49-admin-cms-dict-designer.png` |
| 50-admin-cms-form-designer | /app/admin/cms-form-designer | 11670→11681 | 1625ms | — | `r301-d1-full-pc/50-admin-cms-form-designer.png` |
| 51-admin-cms-view-designer | /app/admin/cms-view-designer | 833→833 | 1623ms | — | `r301-d1-full-pc/51-admin-cms-view-designer.png` |
| 52-admin-cms-script-designer | /app/admin/cms-script-designer | 873→873 | 1612ms | — | `r301-d1-full-pc/52-admin-cms-script-designer.png` |
| 53-admin-cms-xform | /app/admin/cms-xform | 11672→11683 | 1611ms | — | `r301-d1-full-pc/53-admin-cms-xform.png` |
| 54-admin-cms-module | /app/admin/cms-module | 831→831 | 1623ms | — | `r301-d1-full-pc/54-admin-cms-module.png` |
| 55-admin-portal-dict-designer | /app/admin/portal-dict-designer | 836→836 | 1629ms | — | `r301-d1-full-pc/55-admin-portal-dict-designer.png` |
| 56-admin-portal-page-designer | /app/admin/portal-page-designer | 914→914 | 1613ms | — | `r301-d1-full-pc/56-admin-portal-page-designer.png` |
| 57-admin-portal-script-designer | /app/admin/portal-script-designer | 840→840 | 1601ms | — | `r301-d1-full-pc/57-admin-portal-script-designer.png` |
| 58-admin-portal-widget-designer | /app/admin/portal-widget-designer | 836→836 | 1616ms | — | `r301-d1-full-pc/58-admin-portal-widget-designer.png` |
| 59-admin-process-form-designer | /app/admin/process-form-designer | 904→915 | 1609ms | — | `r301-d1-full-pc/59-admin-process-form-designer.png` |
| 60-admin-process-dict-designer | /app/admin/process-dict-designer | 874→885 | 1616ms | — | `r301-d1-full-pc/60-admin-process-dict-designer.png` |
| 61-admin-process-script-designer | /app/admin/process-script-designer | 860→860 | 1626ms | — | `r301-d1-full-pc/61-admin-process-script-designer.png` |
| 62-admin-process-xform | /app/admin/process-xform | 883→894 | 1609ms | — | `r301-d1-full-pc/62-admin-process-xform.png` |
| 63-admin-process-application | /app/admin/process-application | 914→925 | 1614ms | — | `r301-d1-full-pc/63-admin-process-application.png` |
| 64-admin-service-invoke-designer | /app/admin/service-invoke-designer | 838→838 | 1619ms | — | `r301-d1-full-pc/64-admin-service-invoke-designer.png` |
| 65-admin-design-center | /app/admin/design-center | 1571→1571 | 1617ms | — | `r301-d1-full-pc/65-admin-design-center.png` |
| 66-admin-control-panel | /app/admin/control-panel | 997→997 | 1638ms | — | `r301-d1-full-pc/66-admin-control-panel.png` |
| 67-admin-config-designer | /app/admin/config-designer | 941→941 | 1612ms | — | `r301-d1-full-pc/67-admin-config-designer.png` |
| 68-admin-find-designer | /app/admin/find-designer | 1164→1164 | 1609ms | — | `r301-d1-full-pc/68-admin-find-designer.png` |
| 69-admin-collect | /app/admin/collect | 825→825 | 1623ms | — | `r301-d1-full-pc/69-admin-collect.png` |
| 70-admin-template | /app/admin/template | 828→828 | 1614ms | — | `r301-d1-full-pc/70-admin-template.png` |
| 71-admin-pdf-viewer | /app/admin/pdf-viewer | 871→871 | 1630ms | — | `r301-d1-full-pc/71-admin-pdf-viewer.png` |
| 72-admin-deployment | /app/admin/deployment | 830→844 | 1622ms | aiPage 0->1 | `r301-d1-full-pc/72-admin-deployment.png` |
| 73-admin-three-member | /app/admin/three-member | 1480→1480 | 1625ms | — | `r301-d1-full-pc/73-admin-three-member.png` |
| 74-admin-common | /app/admin/common | 834→834 | 1615ms | — | `r301-d1-full-pc/74-admin-common.png` |

### H5（19 页）

| 页 | 路由 | bodyLen 一轮→二轮 | 二轮耗时 | 关键变化 | 截图 |
|---|---|---|---|---|---|
| 01-index | / | 142→158 | 1758ms | — | `r301-d1-full-h5/01-index.png` |
| 02-login | /pages/login/login | 142→158 | 1766ms | — | `r301-d1-full-h5/02-login.png` |
| 03-message | /pages/message/message | 337→128 | 1655ms | — | `r301-d1-full-h5/03-message.png` |
| 04-chat | /pages/chat/chat | 20→20 | 1632ms | — | `r301-d1-full-h5/04-chat.png` |
| 05-process | /pages/process/process | 709→628 | 1672ms | — | `r301-d1-full-h5/05-process.png` |
| 06-process-start | /pages/process/start | 433→620 | 1649ms | — | `r301-d1-full-h5/06-process-start.png` |
| 07-attendance | /pages/attendance/attendance | 179→179 | 1660ms | — | `r301-d1-full-h5/07-attendance.png` |
| 08-doc | /pages/doc/doc | 68→262 | 1662ms | — | `r301-d1-full-h5/08-doc.png` |
| 09-contacts | /pages/contacts/contacts | 535→535 | 1667ms | — | `r301-d1-full-h5/09-contacts.png` |
| 10-meeting | /pages/meeting/meeting | 26→44 | 1637ms | — | `r301-d1-full-h5/10-meeting.png` |
| 11-calendar | /pages/calendar/calendar | 60→29 | 1636ms | errorText true->false | `r301-d1-full-h5/11-calendar.png` |
| 12-bbs | /pages/bbs/bbs | 736→1484 | 1737ms | — | `r301-d1-full-h5/12-bbs.png` |
| 13-bbs-topic | /pages/bbs/topic | 15→15 | 1734ms | — | `r301-d1-full-h5/13-bbs-topic.png` |
| 14-bbs-new | /pages/bbs/new | 36→36 | 1728ms | — | `r301-d1-full-h5/14-bbs-new.png` |
| 15-collect | /pages/collect/collect | 9→43 | 1729ms | — | `r301-d1-full-h5/15-collect.png` |
| 16-browse | /pages/browse/browse | 16→45 | 1636ms | — | `r301-d1-full-h5/16-browse.png` |
| 17-search | /pages/search/search | 17→17 | 1644ms | — | `r301-d1-full-h5/17-search.png` |
| 18-recycle | /pages/recycle/recycle | 9→42 | 1641ms | — | `r301-d1-full-h5/18-recycle.png` |
| 19-mine | /pages/mine/mine | 119→119 | 1646ms | — | `r301-d1-full-h5/19-mine.png` |

## 6. 未收敛与遗留

- PC 组织树数据仍为空（显示重试入口而非真实部门树）；本轮只验证卡死态消除，数据可用性留待后续真实 UI 补数/接口修复轮次。
- PC `19-admin-hotpic`、`21-admin-appinfo` 仍有 loading 标记（bodyLen 与首轮接近，疑常驻骨架/加载条），优先级低。
- PC 首轮 `/app/calendar` 的 pageerror 本轮 0 次，但仅单轮采样，持续稳定性留待后续轮次回归。
- H5 截图为无头浏览器移动视口渲染（390x844 isMobile），`fullPage` 产出高长图；非物理真机，用户手势/性能未覆盖。

## 7. 复现命令

- PC：`cd oa4rust && node test-results/r301-d1-full-pc.cjs`
- H5：`cd oa4rust && node test-results/r301-d1-full-h5.cjs`
- 对照：`cd oa4rust && node test-results/r301-d1-compare.cjs`

> 真值边界：测试域名、远端凭据、SSH 目标均以占位符或运行时读取表示；截图与证据文件位于 gitignored 的 `test-results/`，不入库。
