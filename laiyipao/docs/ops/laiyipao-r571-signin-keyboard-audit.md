# R571 laiyipao/miniapp 签到页 5 类裸 @click 伪按钮键盘可达性甄别（只读）

## 结论
判定：可达真缺陷 1 项 P2（WCAG 2.1 SC 2.1.1 Keyboard A，关联 SC 4.1.2 Name Role Value A）。signin.vue 签到／离线收益／周期 tab／任务领取／兑换 5 类控件键盘完全不可达、辅助技术语义 generic/name=null。属与已修 battles/rank/stage/enemies/index 不同的新实例，不因同 SC 维度已覆盖而判 0，亦非据静态预判。

## 范围与入口
- 目标：laiyipao/miniapp/src/pages/signin/signin.vue（sha256 ae447d22dc6e7aa8c1a13638ffee23ac79b635753df2317ba5c83c91bbe5b794）。
- 5 类锚点（全无 role/tabindex/aria/@keydown）：:42 view.btn.btn-primary @click=doSignIn（领取今日奖励）；:88 view.offline-claim @click=claimOffline（领取离线收益）；:120 text v-for SCOPES @click=switchScope(s.key)（每日／每周／成就）；:155 view.btn @click=claim(t.id)（任务领取）；:173 view.btn @click=doRedeem（兑换）。
- 全页 grep 仅 5 条 @click，等价入口全空（无原生 button、无 @confirm/@longpress、无 role/tabindex/@keydown/aria-label）。5 处 handler 均有完整函数体非死代码。
- 入口：index/index.vue:404 签到 cell（playMenus 第 5 项）一跳 navigateTo；全项目无 tabBar。onMounted(:445-451) 未登录先 store.login() 再并发拉 4 类数据。

## 可达性分层（非死代码确认）
- :120 switchScope 空态常驻纯前端状态切换（:119 scope===s.key 时 style color #58a6ff），零造数可测，决定性样本。
- 其余 4 处需登录 + 服务端造数：stub /signin/status、/me/offline-reward、/tasks、/signin/calendar，均坐实 handler 非死代码。造数分层据实披露，未夸大为常驻零造数。

## 方法
- 自起 pnpm dev:h5 --port <PORT>（真值不入库，报告 base 写 <TEST_HOST>:<PORT>）；Playwright chromium headless 390x844 DPR2 isMobile/hasTouch；stub 仅拦 **/api/v1/**，预置 lyp_token 跳过 guestLogin。
- 决定性判据：DOM.describeNode 取 backendNodeId → getPartialAXTree({backendNodeId}) + getFullAXTree 按 backendDOMNodeId 映射读 role/name；DOM.focus 后读 document.activeElement；过滤 getFullAXTree focused=true 且非 RootWebArea 节点。严禁探宿主 wrapper（R535 假阳性教训），UNI-VIEW/UNI-TEXT 身份独立确认。
- 6 组静态（light/dark × withData/empty/noReward）+ func／mouse／claim2／pos 补充探针，pageErrors 全 0。

## 决定性证据
- 原始 DOM：5 类 role/tabindex/aria 全 null；partial+full AX role=generic name=null ignored=false；DOM.focus 后 activeElement=BODY、isBody=true、isTarget=false、axFocused=[]。
- :155 claim0 由独立 claim2 探针坐实（主探针 stub 路由未渲染该按钮）：attrs 三属性 null，partial/full AX generic/null，focus 后 BODY，干净 Tab30 命中 0。
- 干净 Tab 30 次：6 组 hits 全 0，distinct 仅 [INPUT:uni-input-input, BODY:]，候选 0 命中。
- ★注入对照（决定性去伪）：注入 role=button + tabindex=0 + aria-label + keydown 后 8 类 AX=button name 可读，Tab 命中 sign/offline/scope0/1/2/claim0/redeem 各 3 次，distinct 含 UNI-VIEW/UNI-TEXT（身份独立确认非 wrapper）；focus 后 tag=UNI-VIEW/UNI-TEXT、isBody=false、isTarget=true、axFocused=[button]。
- ★功能真命中（非仅焦点移动）：Enter 触发 claim→writes[claim:1]、sign→文案变 ✓今日已签到＋签到成功金币3000、offline→writes[offlineClaim]、redeem→兑换成功钻石10；scope2 Enter→色标由 scope0 转 scope2＋scopeCalls[daily,achievement] load 重拉。
- 鼠标对照：点 scope1→色标转移 calls[daily,weekly] taskCount 2→1；点 sign→已签到 writes[signin]。证 5 类 handler 非死代码。
- 正面对照：battles.vue 已修 role=button/tabindex=0/aria-label 控件 count=6、Tab 命中 26、AX role=button name 可读，证 AX 通道正常、检测器有效。

## Fail Loud（据实披露探针缺陷，均已修复）
主探针 func case 曾 fatal（claim0 未渲染时 focus null）、func 探针曾 fatal（uni-input 不支持 fill）；非页面缺陷，分别由 func2（改用内部 input.fill）与 claim2（独立造 /tasks 数据）补探针覆盖，最终 func2/claim2/pos fatals=0 pe=0；原始 6 组静态 fatals=0 pe=0。

## 最小修复方向
5 类逐一显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent，复用原 handler（doSignIn/claimOffline/switchScope/claim/doRedeem），业务零改；:120 周期 tab 可选 role=tab + aria-selected（容器 role=tablist）语义更贴切。沿用 R561–R570 范式。

## 证据（gitignored：oa4rust/test-results/r571/，借用已忽略目录存放 laiyipao 证据，据实披露）
runner sha = crypto.sha256(JSON.stringify(report)) 紧凑串；文件 sha256sum = 美化写盘 JSON 的字节哈希，两者分列不可混用。
| 项 | runner 紧凑 JSON sha | 文件 sha256sum |
| --- | --- | --- |
| 主报告 r571-report.json（6 静态组） | 4b4c36b0eb609e4afc34f33b755b8e73974beba924c6423faebdc0c569fb5af2 | b508f8d67020121371ca8d292dc1153127f4575b3e560e2e0e4469a353024c4d |
| 功能探针 r571-func-report.json | a5bc349cbe3dd52a745ea9130db583eb468d11f9060b55345b36a26be733b7cf | e1637512b86311c9d760480f6bde01025b111c28f0a7c222d02a3fdc0e469346 |
| 功能补探针 r571-func2-report.json | c97ffcb390de3d0f834ed872d35f97cac24d22c152b9ecf8776e61beda86756e | 6cf355bad00f7c9b7942fee1391f79f765be0352fcc1d395487de4bbd94407a8 |
| claim 原始态 r571-claim2-report.json | 094e14123d33b4140166ad4cdd24a966246e80197187e0768cd95e924a9eded4 | f45258b51d77b92d79ec653a40cc4e52380e31d2d5276c252d89221f5e609dc5 |
| 正面对照 r571-pos-report.json | 53a5cd87c430fa4e1a44e81c2c95a625e4d7a2336a42a72713d6c31e7ea6cc83 | 2b4524de822205a2f233a43b1a904b188e8755406f367deceaf53365e07d02df |
| 截图 r571-withData-light.png | — | 358efffe5b2e1a64238dc3791a66529991f6ae159a13856160459afbddd14de7 |
| 截图 r571-noReward-dark.png | — | 0a022393450d99f180252a458f64f8d5444a9a53653743050c46b65cc4c198a2 |
| 源码 signin.vue | — | ae447d22dc6e7aa8c1a13638ffee23ac79b635753df2317ba5c83c91bbe5b794 |
| 源码 index.vue | — | 91d0052edd24f86bd6e6e88387215ddeb2824828367f5e6b316d7de72b8f9a36 |
| 源码 api/client.ts | — | cbfe2a5ca9ba88f6e541a6d49712e4864d9b6cd6663600c25983eb85406c25a2 |
| 基线 HEAD | — | d0fe6e30e55bb00db21bd382bd1038e707b8d98e |

截图 read_image 目视有效：withData-light（签到标题＋每日签到领取按钮＋离线 500 金币领取＋每日/每周/成就 tab＋兑换码），noReward-dark（✓今日已签到＋离线 0 已领完）。miniapp H5 无主题切换，light/dark 截图内容一致但文件 sha 因渲染微差不同。