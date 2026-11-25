# R527 内层按钮键盘冒泡劫持修复（WCAG 2.1 AA / 2.1.1）

- 类型：实现修复（改码）—— R526 甄别判定 P2 项的落地修复
- 目标：WCAG 2.1 SC 2.1.1 Keyboard —— MeetingApp.vue 外层 role=button 卡片的 keydown 处理器冒泡截获内层七按钮的 Enter/Space
- 真父：22993f76d3c6604577b35026ca9096cb42f3c429（台账 §518）
- 写域（精确 3 文件，业务零改）：M oa4rust/frontend/apps/desktop/src/views/MeetingApp.vue；A oa4rust/frontend/apps/desktop/src/views/R527.innerKeyboardA11y.contract.test.ts；A 本文档
- 测试域：<TEST_HOST>（占位；E2E 用本地 Vite dev server 加载修复源码 + API stub，无真值、无真实写库）
- 探针：oa4rust/test-results/r527-e2e.cjs + oa4rust/test-results/r527-run.sh + oa4rust/test-results/r527-redgreen.sh（均被 .gitignore:28 忽略）
- 源码 SHA-256：1381feea87c2c8030bd248bd097ac28d2dc5151a2259fd59cc3324e27f450205（MeetingApp.vue 修复后）
- 契约 SHA-256：2d5d651807aac5a20b54d73009e369a408ba5f76e20cab595e3b4408e5a40977
- 报告 SHA-256：bf4a1a387184c39e9b5509009c7a85dec8551e1fc772d9eb8ffb25338645b479

## 一、缺陷来源（R526 甄别结论）

- MeetingApp.vue:51 外层 li.mc modern-card role=button tabindex=0，绑定 @click=viewMeeting(m)、@keydown.enter=viewMeeting(m)、@keydown.space.prevent=viewMeeting(m)。
- :60-66 七个原生 button.bsm，各带 @click.stop：加入、参会人、邀请、邀请管理、改时间、标记完成、签到码。
- 缺陷：keydown 从内层按钮冒泡到外层卡片处理器，键盘用户用 Enter/Space 激活内层按钮自身动作时同时触发 viewMeeting 打开详情；Space 还额外打开详情 modal，导致键盘无法只用内层按钮自身动作（SC 2.1.1）。
- 对照：同文件 :79 的 div.calendar-event role=button 无内层嵌套；ProcessWork.vue:40/62/84 原生 li 无 role，均无此冒泡问题。
- 可达性：loadMeetings(:204-215) 经 api.get 拉取真实会议列表，onMounted(:611) 触发，路由 /app/meeting 可达，非死代码。

## 二、修复方案（最小侵入）

- 七个内层 button 各加 @keydown.enter.stop 与 @keydown.space.stop：在冒泡离开按钮前拦截 keydown，保留原生 button 对 Enter/Space 的默认 click 激活。
- 业务零改：@click.stop 处理器、viewMeeting / joinMeeting 等业务函数、模板结构、样式、他页、鼠标行为逐字未改（契约五守卫断言锁定）。
- 为何不改父处理器：父卡片键盘可达性须保留（E2E 证实父卡片 Enter 开详情、Space 开详情 modal 仍有效）；改父会破坏该既有行为，内层 stop 是最小侵入点。

## 三、契约 RED → GREEN

- 契约读 .vue 源码字符串断言：cardOuter 取 li.mc 块，innerButtons 正则取七个 button.bsm。
- 核心断言（七按钮均带两个 keydown.stop，或父处理器带 target 守卫）：真父覆盖 RED_EXIT=1，Tests 1 failed / 5 passed（失败项为核心断言，缺 @keydown.enter.stop）；修复后 GREEN_EXIT=0，Tests 6 passed。
- 五守卫断言（RED 与 GREEN 均 pass）：七按钮 @click.stop 处理器逐字零改 / 七按钮文案逐字零改 / 外层 role+tabindex+keydown=viewMeeting 零改 / 业务函数签名与核心端点零改 / calendar-event 对照零改。
- replay 脚本 exit=0、RED_GREEN_OK；执行后回读 MeetingApp.vue:60-66 确认七处 stop 完整保留（RESTORED_STOP_COUNT=7）。

## 四、三 verify（全 exit 0）

- 定点 vitest：pnpm exec vitest run R527.innerKeyboardA11y.contract.test.ts → 6 passed（6）。
- 桌面全量：pnpm test → Test Files 335 passed（335），Tests 3227 passed（3227），Duration 32.41s。
- 类型检查：pnpm --recursive run typecheck → Scope 6 of 7，frontend/apps/desktop typecheck Done。

## 五、真浏览器 E2E 证据（双主题 36/36）

- 方法：本地 Vite dev server（v8.3.5，base /oa4rust/，port 5173）加载修复源码；context.route 拦截 /oa4rust-api 全部端点返回 stub（占位数据，无真值、无真实写库）；page.on(request) 记录按键期间实际发出的 API，区分按钮自身动作端点与父详情端点 GET assemble/control/meeting/〈id〉。headless Chromium，1440×900 DPR1，light/dark 双主题。
- 每主题 18 场景：14 内层（7 按钮 × Enter/Space）+ 2 父卡片（Enter/Space）+ 2 鼠标（加入/签到码），双主题合计 36。
- 结果：groups=2，fatals=0，pageerror=0，checksPass=36/36。DOM 核验 mcCount=3、innerCount=7、labels 七项正确、cardRole=button、cardTabindex=0。
- 七按钮 × Enter/Space 共 14 场景全部 actionFired=true、parentDetail=false、detailReqs=[]、detailModal=false、confirm=false——父详情未被误触、Space 无 modal（修复成功核心证据；R526 缺陷态这些场景应为 parentDetail=true / modal=true）。
- 各自的自身动作端点均实际发出且成功：加入 POST meeting/〈id〉/accept；参会人 GET participant/list；邀请 POST participant/add；邀请管理 POST add|delete/invite（自定义确认遮罩点确认后）；改时间 POST modify/starttime；标记完成 POST manual/completed（确认后）；签到码 POST meeting/checkin/code/〈id〉。toast 分别显示已加入会议 / 参会人数：0 / 已邀请 / 已添加邀请 / 会议时间已更新 / 已标记完成 / 签到码占位值。
- 父卡片对照：Enter → parentDetail=true、modal=false；Space → parentDetail=true、modal=true——父卡片键盘可达性保留，修复未误伤外层。
- 鼠标对照：加入 act=true、par=false；签到码 act=true、par=false——鼠标行为零回归。
- 首轮 22/36 的 5 个未通过均为探针自身问题（邀请管理/标记完成未点自定义确认框、签到码动作正则路径写反、报告 OUT 写成相对路径落重复目录），非代码缺陷；修正探针后重跑达 36/36。

## 六、截图与 SHA-256

- 报告：oa4rust/test-results/r527/r527-e2e-report.json，sha=bf4a1a387184c39e9b5509009c7a85dec8551e1fc772d9eb8ffb25338645b479。
- 截图（均 1440×900，列表态）：light = 03319f5dbb15ace10c1ed76e140350630776e9b9603705dac5cdfcbdbcbd09f1（133014 B）；dark = 350ee02f277929e6dcfcdc9dd7a47a0102b6949665a1c06173ed12edc9ddd671（135033 B）。

## 七、脱敏与写域自检

- 全文真值 0：域名用 <TEST_HOST> 占位，无凭据、无真实 URL/IP/个人路径；E2E 账号与会议数据均为占位 stub。
- git status 仅见 M MeetingApp.vue 与 ?? 契约测试（本文档随精确 add 入库）；他人未跟踪文件 augmentor/web/pnpm-lock.yaml 未触碰。
- 证据脚本、报告与截图全部位于 .gitignore:28 忽略的 oa4rust/test-results/r527-*，不入库。
- 精确 git add 仅写域 3 文件，未使用 -A 或 .；提交与推送按队长契约执行。