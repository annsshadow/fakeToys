# OA4Rust R325 H5 流程/待办页深度功能审计（首次覆盖）

- 轮次：R325（H5 流程待办页 process.vue 首次深度功能审计，只读，不改源码/契约/配置、禁部署禁修复）
- 分支：opt1000/oa4rust-pc-ai-refactor（非 main）
- 相关任务：t119 本文审计；据实立项——R316 深度审计仅覆盖 PC ProcessWork.vue，R317（H5 六闭环）与 R321（H5 七模块）均未含 H5 流程/待办页
- 测试服构建：H5 入口 index-BUQSTyeU.js（构建哈希，非地址真值）
- 真浏览器：Playwright Chromium headless 390x844 isMobile/hasTouch DPR2；本地无头浏览器渲染，非物理真机。

## 1. 来源与边界

- 站点地址、账号口令、会话数据一律不入库；本文统一用 <YOUR_TEST_DOMAIN> / <YOUR_TEST_HOST> 占位，口令仅运行期经 gitignored 脚本读取不回显。
- 证据截图、E2E/补测/探查 JSON 与脚本落 gitignored 目录 oa4rust/test-results/r325-h5-process/ 与 oa4rust/test-results/r325-*.cjs，均标注非物理真机。
- 判定纪律：①真缺陷需真 UI 可复现 + 真请求/响应/DOM + 源码定位；②取证限制（无 UI 入口、账号态、无头布局差异、观测窗口错位）不臆造；③产品语义差异（不同 API base/表族不同源）不判 bug。

## 2. 源码侦察（端点与 DOM 入口）

- 页面 oa4rust/mobile/src/pages/process/process.vue：三 tab（待办/已办/我发起，:14-19）；fetchPage :144-150 分派 pendingList/completedList/startedList；loadFirst/loadMore :152-195 用 createLatestGate 防旧响应覆盖，失败文案 describeProcessError 与空态可区分；act/runAction :218-253 审批通过/驳回走 uni.showModal 收集意见后 POST /api/task/{id}/complete|reject。
- 详情浮层 openDetail :49-80 并发拉 attachments/records/worklogs 三节；模板 :299-332 底部 sheet，:333-336 发起流程 fab（@tap=goStart → navigateTo /pages/process/start）。
- 服务 oa4rust/mobile/src/services/index.ts:75-143 processApi：待办 GET task/list/my/paging/{p}/size/{s}、已办 GET taskcompleted/list/my/paging、我发起 POST work/list/my/paging（注释标明此路由仅 POST）、审批 POST /api/task/{id}/complete|reject、详情 attachment/record/worklog 三端点、发起 designer/list/all 与 processing/work。H5 fetch 前缀 /oa4rust-api。
- 路由 oa4rust/mobile/src/pages.json:6-7 注册 pages/process/process 与 pages/process/start，:31 tabBar 含审批入口。
- 分页辅助 oa4rust/mobile/src/pages/process/processList.ts:13-38 PROCESS_PAGE_SIZE=50、loadProcessPage；mergeProcessRows 按 id 去重；失败给非空文案。

## 3. 真浏览器实测结果（七段，pageerror 全程 0）

| 段 | 场景 | 真请求/响应 | DOM 回显 | 判定 |
|---|---|---|---|---|
| P1 | 待办列表分页 | GET task/list/my/paging/1/size/50 → 200（evidence.api 坐实） | 7 条待办卡片、footer '已加载 7 条'、标题/时间/驳回通过按钮渲染 | PASS（脚本首报 LIMITATION 系观测窗口在 tap 后才开始且同 tab tap 被 switchTab 的 active===key 短路未重发；api 200 + DOM 7 条已证实加载成功） |
| P2 | 待办详情打开 | 三端点 attachment/list/work/{id}、record/list/workorworkcompleted/{id}、worklog/list/workorworkcompleted/{id} 均 200 | sheet 浮层打开，detailTitle='S3 运行闭环'，附件(0)/流转记录(0)/工作日志(0) 三节，errorBanner=0 | PASS（详情入口与三节真请求到位，空数据是数据态非缺陷） |
| P3 | 审批驳回（真实写） | POST /api/task/{id}/reject → 200 | uni-modal 弹出（'驳回 填写处理意见（可选） 取消 确定'），确认后 toast '已驳回'，待办列表刷新（首条由 1f6ee23c 变为 1c1cd4c2） | PASS（真 UI 可复现的真写闭环，任务由待办移除） |
| P4 | 已办列表 | GET taskcompleted/list/my/paging/1/size/50 → 200 | 3 条已办（流程 #1/#2/#3，含 2026-11-21 完成时间），footer '已加载 3 条' | PASS |
| P5 | 我发起列表 | POST work/list/my/paging/1/size/50 → 200 | 49 条我发起（R316flow/R314流程/R309补测流程等），分页加载 | PASS（POST 端点口径与源码注释一致） |
| P6 | 发起流程入口 | 直连 #/pages/process/start → GET designer/list/all 200，12 个可选流程渲染 | fab 元素存在可见（'＋发起流程'），但 headless tap 后 URL/body 未跳转、0 新增请求 | 取证限制 + 潜在布局风险候选（详见第 4 节） |
| P7 | 加载失败兜底 | route 强制 GET task/list/my/paging → HTTP 500 后 reload | .tip-error '流程加载失败：HTTP 500 重试'，retry-btn 可见，与真空态 '暂无待办' 可区分 | PASS |

## 4. 判定（三分）

- 真缺陷（P0/P1/P2）：0。P1/P2/P3/P4/P5/P7 真请求 + 真响应 + DOM 回显齐全；其中 P3 为真实写闭环（驳回 200 + toast + 列表刷新），证明审批链路可用。
- 取证限制：①P6 发起流程入口——fab 元素真实存在且 computed position=fixed/pointerEvents=auto，但补测 elementFromPoint(fab 中心) 命中 uni-tabbar-border，说明无头 Chromium 下 meta viewport 使布局视口变 980px、fab（bottom:64rpx）与底部 tabbar 重叠遮挡，Playwright tap/mouse.click/dispatchEvent 均未触发 goStart；而直连 #/pages/process/start 完全正常（designer/list/all 200 + 12 流程渲染），证明路由与页面实现无损。此属无头布局/事件派发取证限制，非可坐实的功能缺陷；同时登记为潜在布局风险候选供产品侧在真机复核（真机 safe-area-inset-bottom 非 0，fab 通常位于 tabbar 上方，本无头环境不复现真机条件）。
- 产品语义差异：无（三个列表端点同属 processplatform assemble/surface 族，审批走 service/processing /api/task，符合源码设计，未发现跨表族不同源）。
- 加载失败与空态可区分：符合 processList.ts 设计意图，无 RED 迹象。

## 5. 修复方向与验证方式

- 本次无真缺陷，无强制修复项。P6 若需在无头自动化中覆盖发起写闭环，建议：验证脚本先滚动到页底或用真机/带触摸的浏览器环境点 fab，或临时隐藏 uni-tabbar-border 后点击；真机端建议产品侧确认 fab 的 bottom 值在含 tabbar 的页面上是否始终高于 tabbar（当前 64rpx + safe-area-inset-bottom 在 meta viewport 980px 的无头环境不足）。
- 验证方式：保留 gitignored 脚本 r325-h5-process.cjs / r325-supplement.cjs / r325-fab-probe.cjs，后续轮次可直接重跑比对；证据见 oa4rust/test-results/r325-h5-process/（evidence.json、supplement.json、fab-probe.json 与 P1..P7 截图、发起入口截图）。

## 6. 复现路径与证据

- 环境：运行期从 gitignored 部署脚本取 H5 测试地址与口令（不回显），Playwright Chromium 390x844 isMobile/hasTouch 登录 H5 → 审批(tabBar) → process.vue。
- 主 E2E：oa4rust/test-results/r325-h5-process.cjs（证据 evidence.json + P1-pending-list.png / P2-detail.png / P3-approve-modal.png / P3-after-action.png / P4-done.png / P5-started.png / P6-start-page.png / P7-load-failure.png）。
- 发起入口补测：oa4rust/test-results/r325-supplement.cjs（supplement.json + P6-supp-after-fab-tap.png / P6-supp-direct-start.png）。
- fab 点击探查：oa4rust/test-results/r325-fab-probe.cjs（fab-probe.json：viewport/elementFromPoint/computed 证据）。
- 以上截图均为本地无头浏览器渲染，非物理真机；站点地址与口令均不入库。

## 7. 结论

R325 H5 流程/待办页首次深度审计完成：待办列表、详情三节、审批驳回真写、已办、我发起、加载失败兜底六段真浏览器 PASS（0 pageerror），发起入口直连路由正常但无头 fab 点击受 tabbar 遮挡属取证限制并登记潜在布局风险候选；0 新增 P0/P1/P2 真缺陷，无产品语义差异。项目 ≥1000 轮持续迭代，非终态。