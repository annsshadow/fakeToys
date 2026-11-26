# R308 全页真浏览器基线再审计（修正 loading 探针假阳性）

> 来源声明：本文数据来自部署在测试服 `<YOUR_TEST_DOMAIN>` 的真实运行产物，使用 Playwright Chromium 无头浏览器（PC 1440x900、H5 390x844 isMobile/hasTouch）按真实 UI 登录流程逐页访问并截图。**截图为无头浏览器渲染结果，不是物理真机截图**。登录口令仅在运行时由远端凭据读取，不入库、不回显；域名、主机等环境真值一律以 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位。

## 1. 任务与方法

- 目标：在 R308 视觉深化轮次改造前，对当前 HEAD 测试服产物做独立只读基线实测，量化每页视觉令牌消费、loading、错误分布，作为后续对照基线；并修正历史脚本中 loading 判定的文本正则假阳性。
- 路由表：从 `test-results/r300-c1-full-pc.cjs`（74 条）与 `r300-c1-full-h5.cjs`（19 条）原文提取，与 R300-C1 / R301-D1 / R305 完全同口径可比。
- 每页流程：`goto(domcontentloaded)` → `waitForTimeout(1500)` → `page.evaluate` 采集 DOM 诊断 → `fullPage` 截图；`pageerror` 与 API 失败（400/404/≥500）经事件监听记录。
- **loading 探针修正（本次核心）**：旧口径用纯文本正则 `/加载中|正在加载|Loading/.test(text)`，会误匹配统计卡标签文案；新口径改为检测骨架屏 DOM 节点 `[class*=skeleton],[class*=ai-skeleton],[class*=o2-skeleton]` 数量 > 0。脚本同时保留 `loadingTextOld`（旧口径）与 `skeletonNodes`/`loading`（新口径）以便量化差异。
- 派生脚本（均 gitignored）：`r308-baseline-reaudit-pc.cjs` / `r308-baseline-reaudit-h5.cjs`；证据：`oa4rust/test-results/r308-baseline-reaudit-pc/` 与 `r308-baseline-reaudit-h5/`。

## 2. 量化总表（与 R301 D1 / R305 baseline 对照）

| 指标 | R301 D1（历史） | R305 baseline（历史） | R308 本次实测 | 判定 |
| --- | --- | --- | --- | --- |
| PC 路由覆盖 | 74/74 | 74/74 | 74/74 | 持平 |
| PC pageerror | 0 | 0 | 0 | 持平 |
| PC API 失败 | 0 | 0 | 0 | 持平 |
| PC DOM aiPage gt0 | 45/74 | 45/74 | 70/74 | 提升 |
| PC 双未消费页（源码 var(--ai-*)=0 且 DOM aiPage=0） | — | 20/74 | 0/74 | 收敛（详见 §5.1） |
| PC errorText=true | 0 | 0 | 0 | 持平 |
| PC loading=true（旧文本正则） | 2 | 2 | 2 | 假阳性未消除（口径问题） |
| PC loading=true（新骨架屏口径） | — | — | 1 | 见 §3 |
| H5 路由覆盖 | 19/19 | 19/19 | 19/19 | 持平 |
| H5 pageerror | 0 | 0 | 0 | 持平 |
| H5 API 失败 | 0 | 0 | 0 | 持平 |
| H5 branded(.ai-brand-fill) gt0 | 2/19 | 2/19 | 16/19 | 大幅提升 |
| H5 errorText=true | 0 | 0 | 0 | 持平 |
| H5 timeoutText=true | 0 | 0 | 0 | 持平 |
| H5 骨架屏节点 gt0 | — | — | 0 | 无加载残留 |

## 3. loading 探针修正前后差异（核心）

| 口径 | PC 命中页 | H5 命中页 | 说明 |
| --- | --- | --- | --- |
| 旧：文本正则 `加载中/正在加载/Loading` | 2（19-admin-hotpic、21-admin-appinfo） | 0 | 两页命中的是统计卡标签「加载中」，表格数据已正常渲染，属假阳性 |
| 新：骨架屏 DOM 节点 > 0 | 1（57-admin-portal-script-designer） | 0 | 该页命中 `<div class="script-loading ai-skeleton-line" aria-hidden="true">`，为无条件渲染的静态装饰条，非真实加载态，属另一类假阳性 |
| 真实加载中页面 | 0 | 0 | 两种探针各有假阳性；逐一目视截图与源码核对后，当前无页面处于真实加载中 |

结论：单纯用文本正则或单纯用骨架屏 class 都无法零误报判定 loading。本次实测证明 **R301/R305 长期记录的「PC loading 2 页」是统计卡标签造成的假阳性**，实际页面早已正常渲染。逐页目视（19/21/57 三张 PC 截图）已确认。

## 4. 分布指标

### 4.1 PC（74 页）

- DOM aiPage：gt0 70 页 / zero 4 页。
- loadingTextOld（旧文本口径）true 2 页；skeletonNodes（骨架屏口径）gt0 1 页。
- errorText true 0 页；emptyState true 14 页（多为设计器/管理页无种子数据）；rawUuid true 4 页；debugText true 0 页。
- pageerror 0、API 失败 0。

### 4.2 H5（19 页）

- branded(.ai-brand-fill) gt0 16 页 / zero 3 页；surface/pressable 为品牌化辅助指标。
- skeletonNodes gt0 0 页；loading true 0 页。
- errorText true 0 页；timeoutText true 0 页；emptyState true 9 页。
- 令牌解析：`--ai-gradient-brand` / `--ai-grid-unit` / `--color-primary` 均由 CSS 变量提供（非空即生效，具体值含环境色值不入库）。
- pageerror 0、API 失败 0。

## 5. 未消费页与品牌覆盖清单

### 5.1 PC DOM aiPage=0 页（4 页）

`/app/personal`、`/app/settings`、`/app/recycle`、`/app/note` 四页 DOM aiPage=0，但对应组件源码均含 `var(--ai-*)` 令牌消费（Personal.vue 5 处、Settings.vue 2 处、RecycleApp.vue 8 处、NoteApp.vue 7 处），属「根元素未打 ai-page class」的标记缺口，**并非双未消费**。相较 R305 baseline 的双未消费 20 页，本轮双未消费页为 0。

### 5.2 H5 branded=0 页（3 页）

`/pages/message/message`（03-message）、`/pages/doc/doc`（08-doc）、`/pages/bbs/topic`（13-bbs-topic）。相较 R301/R305 的 branded=0 共 17 页，本轮收敛到 3 页（消息、文档、帖子详情为次级页面，未强加品牌填充）。

## 6. 新真实缺陷

本轮为只读审计，未发现阻断性或功能性真实缺陷（0 pageerror、0 API 失败、0 错误文案）。记录两处低优先级 UI 文案/装饰问题，供后续轮次评估：

- **L1（统计卡标签误导）**：`/app/admin/hotpic`、`/app/admin/appinfo` 的第四张统计卡标签固定为「加载中」，值为 `loading.value ? 1 : 0`，实为「加载中数量」统计卡，命名易被误读为页面仍在加载，也是旧 loading 文本正则假阳性的根源。源码：`oa4rust/frontend/apps/desktop/src/views/HotpicApp.vue:100`、`AppInfoApp.vue:105`。建议改为「加载任务数」或改用图标+aria-label，避免纯文案「加载中」。复现：登录测试服访问上述两页，观察统计卡。
- **L2（静态骨架条无条件渲染）**：`/app/admin/portal-script-designer` 页在脚本列表已正常显示时仍渲染一条 `<div class="script-loading ai-skeleton-line" aria-hidden="true">`，会被骨架屏探针判为加载中。源码：`oa4rust/frontend/apps/desktop/src/views/PortalScriptDesignerApp.vue:11`。建议加 `v-if="loading"` 条件或移除该装饰条。复现：登录测试服访问该页，检查 DOM 中 `.ai-skeleton-line` 节点。

以上两项均为低优先级，不构成本轮回归；本任务严格只读，未修改任何源码。

## 7. 逐页证据索引（截图 gitignored，相对 `oa4rust/test-results/`）

### PC（74 页）

| 页 | 路由 | bodyLen | aiPage | skeletonNodes | loadingTextOld | errorText | 截图 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 01-dashboard | /app/dashboard | 704 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/01-dashboard.png` |
| 02-org | /app/org | 384 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/02-org.png` |
| 03-directory | /app/directory | 716 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/03-directory.png` |
| 04-process | /app/process | 631 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/04-process.png` |
| 05-im | /app/im | 445 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/05-im.png` |
| 06-personal | /app/personal | 379 | 0 | 0 | false | false | `r308-baseline-reaudit-pc/06-personal.png` |
| 07-settings | /app/settings | 325 | 0 | 0 | false | false | `r308-baseline-reaudit-pc/07-settings.png` |
| 08-calendar | /app/calendar | 350 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/08-calendar.png` |
| 09-file | /app/file | 356 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/09-file.png` |
| 10-bbs | /app/bbs | 557 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/10-bbs.png` |
| 11-meeting | /app/meeting | 456 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/11-meeting.png` |
| 12-attendance | /app/attendance | 495 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/12-attendance.png` |
| 13-recycle | /app/recycle | 334 | 0 | 0 | false | false | `r308-baseline-reaudit-pc/13-recycle.png` |
| 14-bam | /app/bam | 249 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/14-bam.png` |
| 15-note | /app/note | 238 | 0 | 0 | false | false | `r308-baseline-reaudit-pc/15-note.png` |
| 16-search | /app/search | 253 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/16-search.png` |
| 17-admin-query | /app/admin/query | 1296 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/17-admin-query.png` |
| 18-admin-portal | /app/admin/portal | 878 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/18-admin-portal.png` |
| 19-admin-hotpic | /app/admin/hotpic | 876 | 3 | 0 | true | false | `r308-baseline-reaudit-pc/19-admin-hotpic.png` |
| 20-admin-jpush | /app/admin/jpush | 943 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/20-admin-jpush.png` |
| 21-admin-appinfo | /app/admin/appinfo | 871 | 3 | 0 | true | false | `r308-baseline-reaudit-pc/21-admin-appinfo.png` |
| 22-admin-category | /app/admin/category | 862 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/22-admin-category.png` |
| 23-admin-document | /app/admin/document | 844 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/23-admin-document.png` |
| 24-admin-program | /app/admin/program | 917 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/24-admin-program.png` |
| 25-admin-queryview | /app/admin/queryview | 1318 | 5 | 0 | false | false | `r308-baseline-reaudit-pc/25-admin-queryview.png` |
| 26-admin-server | /app/admin/server | 899 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/26-admin-server.png` |
| 27-admin-form | /app/admin/form | 1449 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/27-admin-form.png` |
| 28-admin-view | /app/admin/view | 850 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/28-admin-view.png` |
| 29-admin-fileinfo | /app/admin/fileinfo | 845 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/29-admin-fileinfo.png` |
| 30-admin-role | /app/admin/role | 1047 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/30-admin-role.png` |
| 31-admin-process-designer | /app/admin/process-designer | 1893 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/31-admin-process-designer.png` |
| 32-admin-form-designer | /app/admin/form-designer | 1756 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/32-admin-form-designer.png` |
| 33-admin-query-statement-designer | /app/admin/query-statement-designer | 1558 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/33-admin-query-statement-designer.png` |
| 34-admin-query-designer | /app/admin/query-designer | 1311 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/34-admin-query-designer.png` |
| 35-admin-query-manager-deep | /app/admin/query-manager-deep | 1623 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/35-admin-query-manager-deep.png` |
| 36-admin-portal-designer | /app/admin/portal-designer | 1274 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/36-admin-portal-designer.png` |
| 37-admin-log-viewer | /app/admin/log-viewer | 831 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/37-admin-log-viewer.png` |
| 38-admin-query-query | /app/admin/query-query | 2442 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/38-admin-query-query.png` |
| 39-admin-query-explorer | /app/admin/query-explorer | 916 | 5 | 0 | false | false | `r308-baseline-reaudit-pc/39-admin-query-explorer.png` |
| 40-admin-query-table-designer | /app/admin/query-table-designer | 979 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/40-admin-query-table-designer.png` |
| 41-admin-query-view-designer | /app/admin/query-view-designer | 1042 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/41-admin-query-view-designer.png` |
| 42-admin-query-stat-designer | /app/admin/query-stat-designer | 888 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/42-admin-query-stat-designer.png` |
| 43-admin-query-importer-designer | /app/admin/query-importer-designer | 888 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/43-admin-query-importer-designer.png` |
| 44-admin-cms-column | /app/admin/cms-column | 831 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/44-admin-cms-column.png` |
| 45-admin-cms-column-manager | /app/admin/cms-column-manager | 831 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/45-admin-cms-column-manager.png` |
| 46-admin-cms-index | /app/admin/cms-index | 857 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/46-admin-cms-index.png` |
| 47-admin-process-task-center | /app/admin/process-task-center | 4462 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/47-admin-process-task-center.png` |
| 48-admin-process-manager | /app/admin/process-manager | 1850 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/48-admin-process-manager.png` |
| 49-admin-cms-dict-designer | /app/admin/cms-dict-designer | 843 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/49-admin-cms-dict-designer.png` |
| 50-admin-cms-form-designer | /app/admin/cms-form-designer | 11681 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/50-admin-cms-form-designer.png` |
| 51-admin-cms-view-designer | /app/admin/cms-view-designer | 833 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/51-admin-cms-view-designer.png` |
| 52-admin-cms-script-designer | /app/admin/cms-script-designer | 873 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/52-admin-cms-script-designer.png` |
| 53-admin-cms-xform | /app/admin/cms-xform | 11683 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/53-admin-cms-xform.png` |
| 54-admin-cms-module | /app/admin/cms-module | 831 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/54-admin-cms-module.png` |
| 55-admin-portal-dict-designer | /app/admin/portal-dict-designer | 836 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/55-admin-portal-dict-designer.png` |
| 56-admin-portal-page-designer | /app/admin/portal-page-designer | 914 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/56-admin-portal-page-designer.png` |
| 57-admin-portal-script-designer | /app/admin/portal-script-designer | 890 | 2 | 1 | false | false | `r308-baseline-reaudit-pc/57-admin-portal-script-designer.png` |
| 58-admin-portal-widget-designer | /app/admin/portal-widget-designer | 859 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/58-admin-portal-widget-designer.png` |
| 59-admin-process-form-designer | /app/admin/process-form-designer | 915 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/59-admin-process-form-designer.png` |
| 60-admin-process-dict-designer | /app/admin/process-dict-designer | 885 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/60-admin-process-dict-designer.png` |
| 61-admin-process-script-designer | /app/admin/process-script-designer | 860 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/61-admin-process-script-designer.png` |
| 62-admin-process-xform | /app/admin/process-xform | 894 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/62-admin-process-xform.png` |
| 63-admin-process-application | /app/admin/process-application | 925 | 4 | 0 | false | false | `r308-baseline-reaudit-pc/63-admin-process-application.png` |
| 64-admin-service-invoke-designer | /app/admin/service-invoke-designer | 862 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/64-admin-service-invoke-designer.png` |
| 65-admin-design-center | /app/admin/design-center | 1571 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/65-admin-design-center.png` |
| 66-admin-control-panel | /app/admin/control-panel | 997 | 3 | 0 | false | false | `r308-baseline-reaudit-pc/66-admin-control-panel.png` |
| 67-admin-config-designer | /app/admin/config-designer | 941 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/67-admin-config-designer.png` |
| 68-admin-find-designer | /app/admin/find-designer | 1185 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/68-admin-find-designer.png` |
| 69-admin-collect | /app/admin/collect | 825 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/69-admin-collect.png` |
| 70-admin-template | /app/admin/template | 828 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/70-admin-template.png` |
| 71-admin-pdf-viewer | /app/admin/pdf-viewer | 859 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/71-admin-pdf-viewer.png` |
| 72-admin-deployment | /app/admin/deployment | 844 | 1 | 0 | false | false | `r308-baseline-reaudit-pc/72-admin-deployment.png` |
| 73-admin-three-member | /app/admin/three-member | 1480 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/73-admin-three-member.png` |
| 74-admin-common | /app/admin/common | 859 | 2 | 0 | false | false | `r308-baseline-reaudit-pc/74-admin-common.png` |

### H5（19 页）

| 页 | 路由 | branded | skeletonNodes | loading | errorText | timeoutText | 截图 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 01-index | / | 2 | 0 | false | false | false | `r308-baseline-reaudit-h5/01-index.png` |
| 02-login | /pages/login/login | 2 | 0 | false | false | false | `r308-baseline-reaudit-h5/02-login.png` |
| 03-message | /pages/message/message | 0 | 0 | false | false | false | `r308-baseline-reaudit-h5/03-message.png` |
| 04-chat | /pages/chat/chat | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/04-chat.png` |
| 05-process | /pages/process/process | 3 | 0 | false | false | false | `r308-baseline-reaudit-h5/05-process.png` |
| 06-process-start | /pages/process/start | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/06-process-start.png` |
| 07-attendance | /pages/attendance/attendance | 2 | 0 | false | false | false | `r308-baseline-reaudit-h5/07-attendance.png` |
| 08-doc | /pages/doc/doc | 0 | 0 | false | false | false | `r308-baseline-reaudit-h5/08-doc.png` |
| 09-contacts | /pages/contacts/contacts | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/09-contacts.png` |
| 10-meeting | /pages/meeting/meeting | 4 | 0 | false | false | false | `r308-baseline-reaudit-h5/10-meeting.png` |
| 11-calendar | /pages/calendar/calendar | 3 | 0 | false | false | false | `r308-baseline-reaudit-h5/11-calendar.png` |
| 12-bbs | /pages/bbs/bbs | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/12-bbs.png` |
| 13-bbs-topic | /pages/bbs/topic | 0 | 0 | false | false | false | `r308-baseline-reaudit-h5/13-bbs-topic.png` |
| 14-bbs-new | /pages/bbs/new | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/14-bbs-new.png` |
| 15-collect | /pages/collect/collect | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/15-collect.png` |
| 16-browse | /pages/browse/browse | 4 | 0 | false | false | false | `r308-baseline-reaudit-h5/16-browse.png` |
| 17-search | /pages/search/search | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/17-search.png` |
| 18-recycle | /pages/recycle/recycle | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/18-recycle.png` |
| 19-mine | /pages/mine/mine | 1 | 0 | false | false | false | `r308-baseline-reaudit-h5/19-mine.png` |

## 8. 结论

PC 74/74、H5 19/19 全页真浏览器复查完成，0 pageerror、0 API 失败、0 错误文案。视觉令牌消费较 R301/R305 持续增长（PC aiPage gt0 45→70、H5 branded gt0 2→16），PC 双未消费页 20→0。本次核心的 loading 探针修正坐实历史「PC loading 2 页」为统计卡标签假阳性，当前真实加载中页数为 0；同时发现新骨架屏口径对静态装饰条亦有 1 例假阳性（已定位源码与复现路径）。无阻断性回归，项目持续迭代中（非完成态）。
