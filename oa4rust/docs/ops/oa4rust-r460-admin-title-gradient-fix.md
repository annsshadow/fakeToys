# R460 修复 AdminLayout 管理页标题渐变文字被重复 color 遮蔽（P2）

- 任务：t318（page-core，Kind=implementation），attempt_id 43de32c4-9b2e-4a86-9f34-40ce0484bbc7
- 真父：f2292abb8eecd1f79f38901c45a9573280e1e844（台账 §455），HEAD=UP 同值，AB=0 0，工作树干净
- 依据：R459 跨页 AI 风格一致性甄别（t317，commit 7f6d5a86，文档 oa4rust-r459-ai-consistency-audit.md）判定 1 处可达真缺陷 P2，队长独立复核坐实
- 测试入口：https://<YOUR_TEST_DOMAIN>/oa4rust/app/...（<TEST_HOST> 真值仅存 gitignored runner，不入库）
- 证据目录：oa4rust/test-results/r460-admin-title-gradient/（14 PNG + report.json，均 gitignored）

## 1 缺陷根因

共享组件 AdminLayout.vue 的 .admin-title.ai-gradient-text 规则块内存在两条 color 声明：:199 color:transparent 被 :203 后置 color:var(--text-primary) 覆盖（同一规则块后置声明胜出）。致使用 AdminLayout 渲染标题且页面自身无更高特异性救回规则的管理页（典型 common「公共组件库」）h1.admin-title.ai-gradient-text 的 color 为实色 rgb(15,23,42)/dark rgb(240,246,252)，虽然 backgroundImage=linear-gradient 与 backgroundClip=text 已生效，渐变文字被实心色遮蔽。meeting/process-work 因页面自身 .ai-page .meeting-hero .admin-title 等高特异性规则救回故渐变正常，形成同名类跨页表现不一致。构建产物 index-BMFkaD73.css 同值，非部署时序问题。

## 2 修复方案（surgical，业务零改）

仅删除 AdminLayout.vue:203 那一行重复的 color: var(--text-primary);，保留 :199 color:transparent + :196 background:var(--ai-gradient-text) + :197-198 background-clip:text + :200-202 margin/font。
其它规则块的 color:var(--text-primary)（.admin-sidebar-header / .admin-nav-item:hover / .admin-nav-item.active）属普通文本色，严禁误删，契约第 4 条 it 守卫。模板 h1.admin-title.ai-gradient-text 保留。业务结构/接口/data-testid/其它 view 一律零改。

## 3 契约判据（test-first，4 真实 it 无桩）

新契约 oa4rust/frontend/apps/desktop/src/components/admin/R460.adminTitleGradient.contract.test.ts：
- it1 标题规则块保留 color:transparent + background:var(--ai-gradient-text) + background-clip:text
- it2 同规则块内不得有后置 color:var(--text-primary) 覆盖 transparent（捕获意图，真父此条失败 RED）
- it3 模板 h1 仍带 admin-title 与 ai-gradient-text 类
- it4 其它导航规则块 text-primary 文本色不受影响

真父 f2292abb8 定点 RED：1 failed / 3 passed（4），失败为 it2 真实断言；修复版 GREEN：4/4 全过。

## 4 三 verify（逐项真实执行）

- V1 定点：cd oa4rust/frontend/apps/desktop && pnpm exec vitest run src/components/admin/R460.adminTitleGradient.contract.test.ts → exit 0，4/4。
- V2 全量：cd oa4rust && pnpm test → exit 0，318 files / 3138 tests，0 failed。据实说明：契约 Verify 栏写的 cd oa4rust/frontend/apps/desktop && pnpm test 在 desktop 子项目无 test script（package.json 仅 dev/build/preview/typecheck，执行 exit 1 且无测试输出），全量入口为 oa4rust 根 pnpm test，故以根全量作为 V2 真实证据。
- V3 类型检查：cd oa4rust/frontend/apps/desktop && pnpm typecheck → exit 0。

## 5 构建与部署

- 构建：oa4rust 根 pnpm run build exit 0，入口 index-XFi0y1XK.js。
- 部署：整目录 tar → scp → sha256 比对一致 → 远端 /usr/local/oa4rust 建 web.incoming-r460 → grep 入口 → mv 原子替换，DEPLOY_R460_PC_FULL_OK。

## 6 真浏览器实证（1440x900 headless DPR1 非物理真机）

- 来源：Playwright Chromium headless 1440x900 → 测试服真实页面（NOT a physical device，非物理真机）；凭据运行时经 ssh 从 .admin-credentials 提取注入，不入库；主题经 localStorage[oa4rust_theme] + html[data-theme] 切换。
- 结果：R460_DONE total=57 passed=57 failed=0 errors=0，pageerror=0。
- common 浅色 h1.admin-title.ai-gradient-text：color=rgba(0,0,0,0)、backgroundImage=linear-gradient(120deg,rgb(67,56,202),rgb(14,116,144))、backgroundClip=text —— 渐变文字真正生效（R459 时 color 为 rgb(15,23,42) 实色遮蔽）。
- common 深色：color=rgba(0,0,0,0)、linear-gradient(120deg,rgb(165,180,252),rgb(103,232,249))、clip=text —— 深色色标渐变正常。
- 救回页 meeting/process 与采样页 dashboard/file/bbs/calendar 双主题标题渐变全部 color=transparent + linear-gradient + clip=text，零回归。
- 2 张关键图肉眼确认 1440x900：common-light 浅色「公共组件库」呈蓝紫渐变；common-dark 深色同标题呈青紫渐变（对比 R459 实心色，修复确证）。

## 7 截图 sha256（sha256sum 命令行原始值，14 PNG）

- bbs-dark.png 73989e84056b2b3fbf1876be08fd93c0c7f4e66365aaf5ca14d076fecb21d786
- bbs-light.png 7f3838288cbc34c335f454f3757c279defaaac9ed5ff6d8402dc1a9eb72334ae
- calendar-dark.png 806838833b14f266720eef378c6b2a34127a7c83a55ab07af3e97698ecae16f6
- calendar-light.png 58bc1b43aaa049014fc1cf3a00bac5955f94de87df49724b8479c208f205c783
- common-dark.png afe79b29a25873e32fabc99ddcb7729ec6aab8faffcec317d88c0adfaca33424
- common-light.png 33f73040fdde480b96841e417ee4fee1d06884845ba16e4303365e3d9b0db073
- dashboard-dark.png 67a3a155a193cfd61bba192beac454c17406f95bdb97ae4c5a46f8621820d42e
- dashboard-light.png d761e1c30758505feb9537c1e2e39401c9174603591ea4458372f3554ae8645d
- file-dark.png 94f4985914fc20808246a883da8c61e2039f00414e8696cbcdc9b2096adaf47c
- file-light.png 4e90cf9d64d8be9a6313a236d00963ac98bb80d47660e8a3f2af54c1220fdddf
- meeting-dark.png 2479f8324e5474929a39312dd02d866348b945d747fe6a1df678fa237a9253f1
- meeting-light.png 358123648fd15b37bbaf1674c33d6c466156cf3a6fb225e30da62d5b558f7345
- process-dark.png 8e2e85abe079633c53472721891178eae5d76a5545422e13b06dde09ea4b9866
- process-light.png f5aeb6247712bd1654c1b1c9a5dce1e0574a81575a5fce301e5eda9af83bca90

## 8 真值纪律与边界

- 测试域用 <YOUR_TEST_DOMAIN>、<TEST_HOST> 占位，0 真值；无密钥、口令、IP、个人路径；凭据路径只写来源不写内容。
- 本轮为 ≥1000 轮迭代中的一环（R460），非终态。

