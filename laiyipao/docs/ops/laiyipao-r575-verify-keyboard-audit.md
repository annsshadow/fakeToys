# R575 laiyipao/miniapp 战绩验真页 verify.vue 4 类裸 `@click` 伪按钮键盘可达性甄别（只读）

## 结论
判定：可达真缺陷 1 项 P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。战绩验真页「取复现信息」「本地重放并提交验真」「提交伪造哈希」「本次验真记录回填行」4 类控件键盘完全不可达、辅助技术语义 generic/name=null。属与已修 battles/rank/stage/enemies/index/signin/mastery 七页零重叠的新实例，不因同 SC 维度已覆盖而判 0，亦非据静态预判。

## 范围与入口
- 目标：laiyipao/miniapp/src/pages/verify/verify.vue（sha256 b6e8dba0050b7d4d4dfd36f8ffa8e1aacb4d0158b46ba9bf7c8c46eaee9810c7，558 行）。
- 4 类锚点（全无 role/tabindex/aria/`@keydown`，全文件仅此 4 处 `@click`）：:47-54 view.btn `@click=fetchReplay`（取复现信息，空态常驻零造数决定性切口，空 battle_id 点击即 errMsg+input-error 红框+shake 文本三重反馈，:326-328）；:125-127 view.btn.btn-primary `@click=doReplay`（本地重放并提交验真，v-if replayInfo）；:130-137 view.btn `@click=doFalsify`（提交伪造哈希，v-if replayInfo）；:165-174 view.kv.vh-row.tap v-for verifyHistory `@click=refillBattleId(h.id)`（本次验真记录回填行）。
- 全页 grep 无原生 button、无 `@confirm/@longpress`、无 role/tabindex/`@keydown/aria-label` 等价入口。
- 入口：index/index.vue:409 验真 cell（codexMenus 第 8 项）一跳 navigateTo，pages.json 注册；onMounted(:411-413) 未登录先 store.login() 自动登录，探针预置令牌跳过 guestLogin。

## 可达性分层（非死代码确认，据实不夸大）
- :47 取复现信息为决定性样本：零业务造数常驻，空 battle_id 点击即有 errMsg「请输入有效的 battle_id」+ .input-error 红框 + .muted.shake 抖动文本三重反馈，纯客户端零写请求。函数 fetchReplay 仍在 fetchReplay(575) 成功取回 replayInfo 并渲染复现信息卡，证按钮非空壳。
- :125 本地重放提交验真与 :134 提交伪造哈希需 getReplay 成功产出 replayInfo 后才渲染；二者各触发一次 POST /battle/verify。本轮 build.skills 为空，doReplay 走 replay.ts:302 的 clamp-note 分支并仍产出「无法重放：构筑快照里没有任何已装备技能（全部 slot < 0），无法重放」反馈，足证非死代码；伪造哈希按钮提交后 verifyCalls 由 [verify] 增至 [verify,verify]，功能真命中。
- :169 验真记录行需先 submit 造 verifyHistory（stub 造数分层据实披露，严禁夸大为全部零造数）；回填行 Enter 后 battle_id 由 999 回填为 575。

## 方法
- 自起 pnpm dev:h5（默认端口被他组残留连接占用，Vite 自动切 <PORT>，真值不入库；报告 base 写 <TEST_HOST>:<PORT>）；Playwright chromium headless 390x844 DPR2 isMobile/hasTouch；stub 仅拦 `**/api/v1/**`，造 auth/config/wallet/me/stars/battle/:id/replay/battle/verify（verifyCalls 计数）。
- 决定性判据：DOM.describeNode 取 backendNodeId → getPartialAXTree({backendNodeId}) + getFullAXTree 按 backendDOMNodeId 映射读 role/name；DOM.focus 后读 document.activeElement；过滤 getFullAXTree focused=true 且非 RootWebArea 节点。严禁探宿主 wrapper（R535 假阳性教训），UNI-VIEW/UNI-TEXT 身份独立确认。
- 4 组静态 light/dark × withData/empty（fatals=0、pageErrors=0）+ func（3 case，1 fatal 系 v-for 重建致标记丢失，非页面缺陷）+ func2（补探针 fatals=0 pe=0）+ pos（正面对照 fatals=0 pe=0）。

## 决定性证据
- 原始 DOM：4 类 role/tabindex/aria 全 null；partial+full AX role=generic name=null ignored=false；DOM.focus 后 activeElement=BODY、isBody=true、isTarget=false、axFocused=[]。
- 干净 Tab 30 次：4 组 hits{fetch,replay,falsify,history} 全 0，distinct 仅 [INPUT:uni-input-input,BODY:]，候选 0 命中。
- ★注入对照（决定性去伪）：注入 role=button + tabindex=0 + aria-label + keydown 后 4 类 AX=button name 可读（R575-inj-fetch/replay/falsify/history0）；Tab 命中 fetch=10、四类各 5，distinct 含 UNI-VIEW:btn、UNI-VIEW:btn btn-primary、UNI-VIEW:kv vh-row tap（身份独立确认非 wrapper）。
- ★功能真命中（非仅焦点移动）：空态 fetch Enter 致 errMsg=请输入有效的 battle_id + shakeClass=muted shake + inputError=true 三重反馈；withData replay Enter 致 clamp-note 反馈 + verifyCalls=[verify]，falsify Space 致 verifyCalls=[verify,verify]；func2 中 history0 Enter 致 battleId 由 999 回填为 575（injHistory AX=button）；鼠标点 fetch 恢复复现信息卡（非死代码）。
- 等价入口排除：input 回车只更新 v-model battleId，errMsg 仍空、replayCard=false，不触发 fetchReplay。
- 正面对照：signin.vue 已修 role=button/tabindex=0/aria-label 控件 count=6、Tab 命中 23、5 项 AX role=button name 可读（七日签到已完成/离线收益已领完/每日/每周/成就），证 AX 通道正常、检测器有效。

## Fail Loud（据实披露探针缺陷，均已修复）
主 func 探针 1 fatal：withData case 切 falsify 后 v-for 重建致 history0 标记丢失（focus null），非页面缺陷；func2 探针在每步操作前重跑 mark+inject 覆盖，final fatals=0 pe=0，:51 三重反馈与 :169 回填全部坐实。4 组静态与 pos 探针均 fatals=0 pe=0。

## 最小修复方向
4 类逐一显式补 role=button + tabindex=0 + 动态 aria-label + `@keydown.enter/.space.prevent`，复用原 handler（fetchReplay／doReplay／doFalsify／refillBattleId(h.id)），业务零改；验真记录行可用 role=button + aria-label=回填战报 #id。沿用 R561–R574 范式。

## 证据（gitignored：oa4rust/test-results/r575/，借用已忽略目录存放 laiyipao 证据，据实披露）
runner sha = crypto.sha256(JSON.stringify(report)) 紧凑串；文件 sha256sum = 美化写盘 JSON 的字节哈希，两者分列不可混用。
| 项 | runner 紧凑 JSON sha | 文件 sha256sum |
| --- | --- | --- |
| 主报告 r575-report.json（4 组） | bef695361b3f58fb6959c6f9b33208ca57ad9e33103257185efe0b33ae3dc9d9 | 6aab84df757332b6b52aabab99b47e26f9b417caccacd61061600cb29d8a20d4 |
| 功能探针 r575-func.cjs（3 case，1 fatal 非页面缺陷） | e7ee6830613b09d1384c2fd3355eb76bfb3b14f40d8a4f7bcf37758ee916fba6 | 5e2586cd994b99268bb6da508c6dcd197d43a5e55c0ea72f5d4bf6f335dd577e |
| 功能补探针 r575-func2（-fatals=0） | b16cc1fba145c2972fdb1068e5dd2911e0ee345c337e1f987e3c635d56f927d4 | 63e939d6bc6bedfb65dfb71762b1b3f64f110cccf4e07b76dc416fbf86ee9bca |
| 正面对照 r575-pos-report.json | bc9539819e7444906d4a2cdf9b7147d5f37ae2f56add329f0517164336cf7598 | e7589583b9e5e4ac43754eb384c7e66071852fbae10e79bc700a990a00c0a45c |
| 截图 r575-withData-light.png | — | f4babc6bb32fa291b7cf21811b3a71f8c451e3325b74b20bd0f755e4ec4185d1 |
| 截图 r575-withData-dark.png | — | 38821da8c9475ced33c0be3d17a877ce3061ec1d9d68654641f4ea0cd8ef78c3 |
| 截图 r575-empty-light/dark.png（同 sha） | — | 0fda1e78fe58a0e109002202608b9b0b3a26253a51c7bda5a4c21b3c46c9e2a0 |
| 源码 verify.vue | — | b6e8dba0050b7d4d4dfd36f8ffa8e1aacb4d0158b46ba9bf7c8c46eaee9810c7 |
| 源码 index.vue | — | 91d0052edd24f86bd6e6e88387215ddeb2824828367f5e6b316d7de72b8f9a36 |
| 源码 api/client.ts | — | cbfe2a5ca9ba88f6e541a6d49712e4864d9b6cd6663600c25983eb85406c25a2 |
| 源码 game/replay.ts | — | 5f9693319d063d0e040f1de8082da46b782021d71e7db15998de90b92e38b137 |
| 基线 HEAD | — | d53c987a0cc28dc546bfeb0b1291dadc48b906e7 |

截图 read_image 目视有效：withData-light（战绩验真标题+三步条+输入 battle_id 575+取复现信息按钮+复现信息卡（battle_id/关卡/波次/种子/构筑快照/哈希）+本地重放两按钮+服务端裁定「验真失败 — 分数被判定为不可复现」+本次验真记录 #575 不一致）；empty-light（输入 battle_id 占位「例如 1」+取复现信息按钮+空态，步骤条 2/3 未点亮）。miniapp H5 无主题切换，empty light/dark 文件同 sha 据实披露。
