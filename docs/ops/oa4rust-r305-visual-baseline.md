# R305 视觉深化改造前 PC/H5 令牌消费度真浏览器基线（只读）

- 采集日期：2026-11-21（Asia/Shanghai）
- 分支：`opt1000/oa4rust-pc-ai-refactor`（HEAD `29505ad788215ba3240187beaab5fbee43bd4be0`）
- 任务：t50（R305 改造前独立基线实测，严格只读，未改任何源码/契约测试/清单）
- 目的：为 page-work / shell-eng 的 R305 改造提供**改造前对照基线**，改造后用同脚本同口径复查量化差异。
- 测试服：`https://<YOUR_TEST_DOMAIN>/oa4rust`（PC）、`https://<YOUR_TEST_DOMAIN>/oa4rust-h5/`（H5）；采集时产物 PC `index-CzrixMbH.js` / H5 `index-YBZxDRU-.js`，服务 active。
- 采集方式：Playwright Chromium headless 真浏览器 + 真实 UI 登录（凭据运行时从部署环境变量读取，不入库），PC `1440x900`、H5 `390x844` isMobile/hasTouch/deviceScaleFactor=2。
- **截图性质：无头浏览器渲染，非物理真机**；按真实页面流程逐页截图，未伪造、未用 dev server 或静态假数据。
- **真值声明：本文 0 真值**——域名用 `<YOUR_TEST_DOMAIN>` 占位，口令/私钥/会话数据/真实主机地址均不写入本文件。

## 一、采集范围与方法

| 端 | 路由数 | 截图数 | pageerror | API 失败(400/404/5xx) | 证据目录（gitignored） |
| --- | --- | --- | --- | --- | --- |
| PC | 74/74 | 75（含登录流程 1） | 0 | 0 | `oa4rust/test-results/r305-baseline-pc/` |
| H5 | 19/19 | 20（含登录流程 1） | 0 | 0 | `oa4rust/test-results/r305-baseline-h5/` |

复用 C1/R301 已验证脚本流程（`r301-d1-full-pc.cjs` / `r301-d1-full-h5.cjs`），仅改输出目录与完成标记，路由表、登录姿势、诊断字段原样继承，保证同口径可比。派生脚本：`r305-baseline-pc.cjs` / `r305-baseline-h5.cjs`（均 gitignored）。

诊断字段：PC = `aiPage`（`[class*=ai-page]` 元素数）、`loading`、`errorText`、`emptyState`、`tableRows`、`debugText`、`rawUuid` 等；H5 = 令牌解析 `--ai-gradient-brand`/`--ai-grid-unit`/`--color-primary`、`branded`（`.ai-brand-fill`）、`surface`（`.ai-surface-gradient`）、`skeleton`、`errorText`/`timeoutText`。

源码交叉口径：从 `main.ts` 解析 `path -> import('./views/*.vue')` 映射（74 路由），逐个统计 view 文件内 `var(--ai-*)` 出现次数（脚本 `r305-source-scan.cjs`，结果 `r305-source-scan.json`）。

## 二、PC 令牌消费度基线（74 路由）

### 2.1 DOM aiPage 分布（本轮 R305 基线）

| aiPage（class 含 ai-page 元素数） | 页数 | 占比 |
| --- | --- | --- |
| 0 | 29 | 39.2% |
| 1 | 11 | 14.9% |
| 2 | 16 | 21.6% |
| 3 | 10 | 13.5% |
| 4 | 8 | 10.8% |

DOM 至少 1 个 ai-page 容器：**45/74（60.8%）**；仍为 0 的 **29 页**（改造重点池）。aiPage 仅反映 class 标记，与源码 `var(--ai-*)` 消费并非一一对应（见 2.2）。

### 2.2 双未消费页清单（源码 `var(--ai-*)`=0 **且** DOM aiPage=0，共 20 页）

这是 R305 视觉深化的**首要目标**：既无令牌变量引用，也无 ai-page 容器标记。

| # | 页面 |
| --- | --- |
| 1 | `12-attendance /app/attendance` |
| 2 | `16-search /app/search` |
| 3 | `17-admin-query /app/admin/query` |
| 4 | `20-admin-jpush /app/admin/jpush` |
| 5 | `25-admin-queryview /app/admin/queryview` |
| 6 | `27-admin-form /app/admin/form` |
| 7 | `28-admin-view /app/admin/view` |
| 8 | `29-admin-fileinfo /app/admin/fileinfo` |
| 9 | `34-admin-query-designer /app/admin/query-designer` |
| 10 | `36-admin-portal-designer /app/admin/portal-designer` |
| 11 | `39-admin-query-explorer /app/admin/query-explorer` |
| 12 | `41-admin-query-view-designer /app/admin/query-view-designer` |
| 13 | `55-admin-portal-dict-designer /app/admin/portal-dict-designer` |
| 14 | `56-admin-portal-page-designer /app/admin/portal-page-designer` |
| 15 | `57-admin-portal-script-designer /app/admin/portal-script-designer` |
| 16 | `58-admin-portal-widget-designer /app/admin/portal-widget-designer` |
| 17 | `64-admin-service-invoke-designer /app/admin/service-invoke-designer` |
| 18 | `68-admin-find-designer /app/admin/find-designer` |
| 19 | `71-admin-pdf-viewer /app/admin/pdf-viewer` |
| 20 | `74-admin-common /app/admin/common` |

其中核心业务页 2 个：`/app/attendance`（考勤）、`/app/search`（全局搜索）；其余 18 个为 admin 管理/设计器页。

### 2.3 口径差异页（DOM aiPage=0 但源码已含令牌，非双未消费）

`/app/org` srcVar=4、`/app/im`=12、`/app/file`=12、`/app/bbs`=9、`/app/recycle`=19、`/app/bam`=13、`/app/note`=17、`/app/personal`/`/app/settings` 等：**源码已用 `var(--ai-*)` 但根元素未打 `ai-page` class**，属标记缺口而非令牌缺席，不应误判为完全未消费。`/app/dashboard` srcVar=24 且 DOM aiPage=1。

### 2.4 运行时健康度

- `errorText=true`：**0 页**（无加载失败/连接超时/data is undefined 等文案）。
- `loading=true`：2 页（`/app/admin/hotpic`、`/app/admin/appinfo`，轻微，非本任务修复范围）。
- 空态页：14 页（多为设计器/管理页无种子数据）。
- pageerror 0、API 失败 0，改造前基线运行稳定。

## 三、H5 令牌消费度基线（19 路由）

| 指标 | 结果 |
| --- | --- |
| `--ai-gradient-brand` 解析到渐变 | 19/19 |
| `--ai-grid-unit` = 8px | 19/19 |
| `.ai-surface-gradient` ≥1 | 19/19 |
| `.ai-brand-fill` = 0（branded=0） | **17/19** |
| `.ai-brand-fill` >0 | 2/19（index=2、login=2） |
| 骨架屏 class >0 | **0/19** |
| errorText / timeoutText / pageerror / API 失败 | 全部 0 |

结论：H5 令牌层 100% 注入、根容器浅消费 100%，但**品牌渐变实心仅 2/19**、**骨架屏 0/19**，深度消费是主要缺口。

### 3.1 H5 branded=0 页清单（17 页，改造目标）

| # | 页面 |
| --- | --- |
| 1 | `03-message /pages/message/message` |
| 2 | `04-chat /pages/chat/chat` |
| 3 | `05-process /pages/process/process` |
| 4 | `06-process-start /pages/process/start` |
| 5 | `07-attendance /pages/attendance/attendance` |
| 6 | `08-doc /pages/doc/doc` |
| 7 | `09-contacts /pages/contacts/contacts` |
| 8 | `10-meeting /pages/meeting/meeting` |
| 9 | `11-calendar /pages/calendar/calendar` |
| 10 | `12-bbs /pages/bbs/bbs` |
| 11 | `13-bbs-topic /pages/bbs/topic` |
| 12 | `14-bbs-new /pages/bbs/new` |
| 13 | `15-collect /pages/collect/collect` |
| 14 | `16-browse /pages/browse/browse` |
| 15 | `17-search /pages/search/search` |
| 16 | `18-recycle /pages/recycle/recycle` |
| 17 | `19-mine /pages/mine/mine` |

## 四、与 R301 D1 第二轮实测的量化对照

| 指标 | R301 D1（改造前上轮） | R305 基线（本轮） | 差异 |
| --- | --- | --- | --- |
| PC aiPage 分布(0/1/2/3/4) | 29/11/16/10/8 | 29/11/16/10/8 | 完全一致 |
| PC 至少 1 ai-page | 45/74 | 45/74 | 持平 |
| PC pageerror / API 失败 | 0 / 0 | 0 / 0 | 持平 |
| H5 branded=0 | 17/19 | 17/19 | 持平 |
| H5 令牌解析/栅格/表面 | 19/19 | 19/19 | 持平 |
| H5 pageerror / API 失败 | 0 / 0 | 0 / 0 | 持平 |

两轮基线完全一致，说明 R305 改造前界面令牌消费度处于稳定态，本清单可作为改造后量化提升的可靠参照系。

## 五、给 R305 改造的建议输入（仅建议，本任务不改实现）

1. **P0 双未消费 20 页**：优先 `/app/attendance`、`/app/search` 两个核心业务页落地 `.ai-panel` + `var(--ai-glass-*)` + 8px 栅格 + 骨架屏；admin 18 页按设计器批量模式覆盖。
2. **P1 口径差页**：org/im/file/bbs 等已含令牌但 DOM aiPage=0 的页面，补打 `ai-page` 容器 class 以对齐 DOM 口径（不需重复注入令牌）。
3. **P1 H5 17 页 branded=0**：在统计卡、主按钮、tab 选中态引入 `.ai-brand-fill`，目标 branded ≥1/页；同时补 19 页骨架屏（当前 0/19）。
4. **P2 轻微项**：`/app/admin/hotpic`、`/app/admin/appinfo` loading 常驻，改造时顺带核查。

## 六、证据索引与边界

- PC 逐页证据：`oa4rust/test-results/r305-baseline-pc/evidence.json` + 75 PNG（`01-dashboard.png` … `74-admin-common.png`，另 `00-login-flow.png`）。
- H5 逐页证据：`oa4rust/test-results/r305-baseline-h5/evidence.json` + 20 PNG（`01-index.png` … `19-mine.png`，另 `00-login-flow.png`）。
- 派生物：`r305-baseline-pc.cjs`/`r305-baseline-h5.cjs`（脚本）、`r305-source-scan.json`（源码计数）、`r305-analysis.json`（解析结果），均 gitignored、不入库。
- 目视抽查：`01-dashboard.png`（渐变统计卡、5 待审批）、`02-org.png`（空态、无 ai-page）、H5 `01-index.png`（品牌渐变实心 + 9 待办审批）均与 DOM 诊断一致。
- **边界**：本文是改造前只读基线，不含任何实现改动，不构成项目完成结论；项目按 ≥1000 轮持续迭代，R305 改造后需用同口径脚本复查并量化差异。
