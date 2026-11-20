# R301 第二轮缺陷收敛审计（真浏览器逐条回归）

- 审计任务：t18 [C9]，attempt `bc277913-be75-432e-b7d1-6be99eacaccd`
- 审计日期：2026-11-21（Asia/Shanghai）
- 工作分支：`opt1000/oa4rust-pc-ai-refactor`（HEAD `cd07e097204ed6b0bd366d8a4a92ac8a6fe5c0de`，与远端追踪一致；本任务只审计，不改任何实现代码）
- 关联前序：`docs/ops/oa4rust-defect-audit-r300.md`（t3 缺陷清单，commit `edebebe54`）；`docs/ops/oa4rust-c1-full-browser-e2e-r300.md`（t10 全页实测，commit `dc95aa8c`）；`oa4rust/docs/ops/oa4rust-r302-dr02-dashboard-consistency.md`、`oa4rust/docs/ops/oa4rust-attendance-stats-fix-r300.md` 等修复记录。
- 本轮为 ≥1000 轮持续迭代中的一环，**非完成态**。

## 来源与取证声明

- 测试服：`https://<YOUR_TEST_DOMAIN>/oa4rust`（PC）与 `https://<YOUR_TEST_DOMAIN>/oa4rust-h5`（H5），部署目录 `<YOUR_TEST_HOST>`，端口 3010；**非本地 dev server、非静态假数据**。
- 真浏览器：Playwright Chromium headless，PC 视口 1440x900、H5 视口 390x844（isMobile/hasTouch），从登录页 UI 真登录。截图为**本地 headless 浏览器对测试服真实页面的渲染证据，非物理真机、非真机截图**。
- 口令：运行期经 ssh 到测试服读取 `/usr/local/oa4rust/.admin-credentials`（脚本 `test-results/extract-remote-creds.py`），不入库、不回显。
- 产物版本：首次采集时 PC 入口 `index-Br4G57bM.js`；核对发现测试服已于 06:11 更新为 `index-iaQwn-FZ.js`（队友并行部署）。为确保结论对应当前版本，已用当前产物**全量重跑** PC/H5 收敛脚本，本文所有结论以重跑证据为准。
- 证据目录（gitignored，不入库）：`oa4rust/test-results/r301-convergence-pc/`（`evidence.json` + 13 张 PNG）、`oa4rust/test-results/r301-convergence-h5/`（`evidence.json` + 14 张 PNG）。脚本 `oa4rust/test-results/r301-convergence-pc.cjs` 与 `r301-convergence-h5.cjs`。

## 回归方法与判定口径

1. 以 t3 缺陷清单与 C1 新发现的两个 P0 为回归项，按同一路径在真浏览器重访，逐页 DOM 诊断（bodySample / loading / errorText / dataIsUndefined / rawUuid / debugText / aiPage）+ 截图 + pageerror/API 失败采集。
2. 收敛判据：原现象不再可见、真值呈现且与数据源同口径、无新增 pageerror/API 失败；否则判未收敛并回流。
3. PC 检查汇总：`R301_PC_DONE {"captured":13,"total":13,"passed":false}`；CHECKS `bamNoError=true / attendanceHardcodeGone=true / calendarNoTdz=true / debugHidden=true / noRawUuidVisible=true / noPageErrors=true / noApiFailures=false`（唯一失败为 org 组树接口 400，见未收敛项）。
4. H5 检查汇总：`R301_H5_DONE {"captured":14,"total":14,"passed":true}`；CHECKS `loggedIn=true / p0TimeoutGone=true / h5CalendarOk=true / emptyGuidesPresent=true / noPageErrors=true / noApiFailures=true`。

## 一、已收敛项（逐条回归结论）

| 编号 | 原缺陷 | 回归路径 | 当前真浏览器证据 | 结论 |
| --- | --- | --- | --- | --- |
| P0-1 | PC BAM 统计 `data is undefined` 错误条 | `/app/bam` | bodySample 含「90 总活动 / 0 活跃中 / 22 已完成 / 10 失败」，`dataIsUndefined=false`、`debugText=0` | 已收敛：统计卡呈现真实数值，错误条消失 |
| P0-2 | PC 考勤统计硬编码 45/42/3/2 | `/app/attendance` | bodySample 为「0 应出勤 / 0 实际出勤 / 0 迟到 / 0 请假」+「暂无考勤记录」；统计与空明细同口径，副标题已改「更多维护操作可展开高级操作」 | 已收敛：commit `13af398d0`，统计由真实明细派生 |
| P0-3 | H5 bbs/search/mine「连接服务器超时」 | `#/pages/bbs/bbs` / `#/pages/search/search` / `#/pages/mine/mine` | 三页 `timeoutText=false`、`loadFail=false`；bbs 有真实主题、search 为正常搜索骨架页（bodyLen=17 仅因文案短）、mine 为个人中心表单 | 已收敛：超时文案消失，页面正常渲染 |
| C1-P0-1 | PC 日历 TDZ `Cannot access 'q' before initialization` | `/app/calendar` | bodySample 为正常月历（「我的日历（）公共日历（0）」+ 日期网格），`pageErrors=[]`；源码 `CalendarApp.vue:337` 声明早于 `:342` 调用 | 已收敛：TDZ 修复后无 pageerror |
| C1-P0-2 | H5 日历 `S.value.map is not a function` | `#/pages/calendar/calendar` | bodySample「日历 今日 本周 我的日历 0 个 + 新建 今日暂无日程」；calendar API HTTP200，`data={myCalendars:[],unitCalendars:[]}` 信封正确消费 | 已收敛：信封解析修复生效 |
| P1-1 | PC 工作台待审批 0 vs H5 9 / 待办列表自相矛盾 | `/app/dashboard` | bodySample「5 待审批」+「我的待办」列表多条；`rawUuid=false`；H5 工作台「9 待办审批」为 H5 自身口径 | 已收敛：统计与待办列表同源（DR-02，`approvalCount` 派生） |
| P1-3 | 端点调试侵入业务页 | 13 页 PC 全查 | `debugText=0`、`rawEndpointVisible=0` 全部通过 | 已收敛：调试入口已收进高级操作 |
| P2-1 | 可见「已切换到：XXX」无障碍提示 | 页面 DOM 诊断 | 核心业务页 bodySample 不再出现该提示文案 | 已收敛（业务页），外壳辅助提示若仍需全局静默另列 polish |
| P2-2 | 流程/待办暴露原始 UUID | `/app/dashboard`、`#/pages/process/process` | PC `rawUuid=false`；待办标题可读化（`displayTodoTitle/foldRawKeySubtext`），H5 审批列表仍显示截断 key 形如 `71d52ed3-6d4…b163700e`（属既有数据形态展示，未见异常报错） | 基本收敛：PC 可读化生效，H5 截断展示保留观察 |
| P1-2a | PC 首屏 directory 0 人 / file/bbs/meeting/note/recycle 空 | 对应 6 页 | 当前月测试服数据本就为空（考勤同样空），页面给出空态文案而非加载失败；H5 contacts 有 E2E001..E2E014 真实人员 | 部分收敛：空态可读性改善，真实数据依赖后端种子，非前端 bug |

## 二、未收敛项（回流下一轮）

### U-1（原 t3 P1-2）PC `/app/org` 组织树 Loading 常驻 + 组树接口 400

- 现象：真浏览器进入组织架构页，左侧「Search...」下持续显示 `Loading...`，右侧 `Select a node to view details`；页面功能不可用。最近一次重跑 bodyLen=234，`loadingNodes>0`，`aiPage=0`，`modernPage=0`。
- 稳定性：`r301-org-probe3.cjs` 连续 4 次刷新 4/4 `loading=true`、`treeText=[Loading..., Select a node, 组织架构]`；截图 `oa4rust/test-results/r301-convergence-pc/org.png`（1440x900）已目视确认。
- API 取证：当前产物重跑捕获一次 `GET /oa4rust-api/api/organization/assemble/control/group/list/like` 返回 HTTP **400**，这是本轮 PC `noApiFailures=false` 的唯一来源；probe2/probe3 未复现该 400（`HITS=[]`），属首屏竞态/偶发，但 Loading 常驻稳定复现。
- 根因方向（源码级）：`OrgDirectoryApp.vue:221` 的 `onMounted` 与 `:403` 的 `api.get('/api/departments/tree')` 负责**组织树**，而 commit `bd7031a9a`（t28 DR-03b）只修了**人员名册**（`loadPersonFilter` / `loadError` / retry / onMounted 名册加载），组织树加载无超时兜底与错误态，请求失败后 `Loading...` 永久残留。
- 回流建议（不在本任务实施）：参照 DR-03b 的兜底模式给组树/部门树请求加超时+catch 落持久错误态+finally 复位 busy+可见重试；对 400 请求核对后端 `group/list/like` 参数契约；test-first RED→GREEN；部署后真浏览器断言树节点可见、Loading 消失、错误态可恢复。

## 三、取证限制与如实标注

- org 400 为一次捕获，未在多次探针中稳定复现，不据此断定后端稳定缺陷；但 Loading 常驻 4/4 复现，判定为稳定未收敛。
- H5 早期基线中的 message/meeting/browse 未落盘属 tabbit 工具链 EVALUATION_TIMEOUT，本轮 Playwright 已 14/14 捕获，不作为业务缺陷。
- 测试服处于队友并行迭代中，产物中途从 `index-Br4G57bM.js` 切换到 `index-iaQwn-FZ.js`；已用后者全量重跑，结论对应当前版本，但后续若再次部署需再次抽检。
- 所有截图均为 headless 浏览器本地渲染证据，非物理真机；口令、真实域名、主机地址不在本文出现（统一 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位）。

## 四、结论

t3/C1 主要 P0（BAM、考勤硬编码、PC/H5 日历、H5 超时）与 P1/P2 多项已真浏览器回归确认收敛；唯一明确未收敛为 **U-1 PC 组织树 Loading 常驻**（伴随偶发组树 400），建议作为下一轮 test-first RED→GREEN fix 的输入。本轮未改实现代码，项目仍处 ≥1000 轮持续迭代中，非完成态。
