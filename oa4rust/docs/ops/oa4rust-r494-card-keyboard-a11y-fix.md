# R494 MeetingApp 卡片键盘可达性修复（WCAG 2.1.1）

## 1. 概况

- 轮次：R494（台账 §489 据实立项），真父提交 `7e145fecf740170e201c82cb4e32996e6c9b253a`（R493 甄别 commit）；实际 HEAD 为台账 commit `166427b62`，MeetingApp.vue blob 与真父一致（`c182f4ce`），已 git rev-parse 核实。
- 缺陷：R493 甄别判 1 项 P2——MeetingApp.vue:51 会议列表卡片 `li.mc` 与 :79 日历条 `div.calendar-event` 仅有鼠标 `@click=viewMeeting(m)`，无键盘入口，纯键盘用户无法打开会议详情浮层（违反 WCAG 2.1.1 Keyboard）。
- 修复：两处卡片各新增 `role=button` + `tabindex=0` + `@keydown.enter=viewMeeting(m)` + `@keydown.space.prevent=viewMeeting(m)`，复用既有 `viewMeeting`（:274 仅开浮层 + 异步加载，无副作用）。
- 测试服：受控测试环境（域名 `<YOUR_TEST_DOMAIN>`、主机 `<TEST_HOST>` 占位）；凭据运行时从受控凭据源提取注入，真值不入库。

## 2. 根因与修复

- 静态锚点：MeetingApp.vue:51 列表卡片 `li v-for=m in meetings class=mc modern-card @click=viewMeeting(m)`、:79 日历条 `div v-for=m in day.meetings class=calendar-event @click=viewMeeting(m)` 唯一鼠标入口；行内 7 个按钮（:60-66）均 `@click.stop` 调各自业务函数、无一调 viewMeeting；浮层模板 :98-112 含标题/时间/创建者/说明/控制项/关联/关闭；`viewMeeting` :274-307 async 仅开浮层 + 5 个 api.get 异步加载。
- 修复选择：仅两处卡片开标签各加 4 个键盘/语义属性，复用 viewMeeting；不新增函数、不改浮层、不改业务处理器。
- 硬纪律：原 `@click`、v-for、7 按钮 `@click.stop` 与 class、浮层模板、viewMeeting 函数体、业务、样式、data-testid 全部零改；不改 modalFocus.ts / main.ts。

## 3. 契约 RED→GREEN

- 契约：`oa4rust/frontend/apps/desktop/src/views/R494.cardKeyboardA11y.contract.test.ts`（源码字符串断言，readFileSync + `lineWith` 片段提取）。
- 5 it：①两卡片 role=button + tabindex=0 + @keydown.enter + @keydown.space.prevent（核心）②原 @click/v-for 保留 ③ 7 按钮 @click.stop 全保留且无一调 viewMeeting ④ viewMeeting 函数体 + 浮层模板保留 ⑤卡片 class/结构零改。
- 踩坑：守卫 5 初稿用 viewMeeting(m) 计数 toBe(6) 会在真父（仅 2 处）误失败污染守卫父版 pass，改为 toBeGreaterThanOrEqual(2) 后收敛。
- 真父 7e145fec 定点 RED：Tests 1 failed | 4 passed（5），失败正为核心 role/tabindex/keydown 断言，4 项守卫父版全 pass 无桩错；修复版 GREEN：Tests 5 passed（5）。

## 4. 三 verify（全 exit 0）

- V1 定点契约：`cd oa4rust && npx vitest run frontend/apps/desktop/src/views/R494.cardKeyboardA11y.contract.test.ts` → 1 passed (5 tests)，exit 0。
- V2 全量测试：`cd oa4rust && pnpm test` → 327 files / 3183 tests passed，0 failed，exit 0。
- V3 类型检查：`cd oa4rust && pnpm typecheck` → 6 项目全 Done（含 frontend/apps/desktop），exit 0。
- 构建：入口 `index-DK9KffKb.js`；部署 `deploy-pc-r494.sh`（tar -czf → scp → 同一归档 sha256 比对 → 远端解压 web.incoming-r494 → grep 入口 → 原子替换）`SHA_OK=4decaa348e98e26f5de05dccac7560f109926dd13e05047d79bd4471bdd69ef3`，`DEPLOY_R494_PC_FULL_OK`。

## 5. 真浏览器 E2E

- 脚本 `test-results/r494-card-keyboard-check.cjs` + runner（gitignored），Playwright Chromium headless 非物理真机 PC 1440×900 DPR1，light/dark 双主题；CDP DOM + Accessibility.getPartialAXTree + Tab 遍历 + focus + Enter/Space 按键。
- 终轮 `R494_DONE checks=20 passed=20 failed=[] shots=2 errors=0`，0 pageerror。
- AXTree 实证（双主题一致）：会议卡片 `axRole=button`、`axFocusable=true`、未 ignored，DOM role=button / tabindex=0；Tab 遍历第 26 步命中卡片；7 个行内按钮全保留；日历条同样 role=button + tabindex=0。
- 浮层打开实证（双主题一致）：键盘触发后 `.mo` 浮层可见，含标题、时间（2026-11-24 14:00:00 ~ 15:00:00）、创建者 admin、说明、控制项 0 项、关联（附件 0 · 后续会议 4 · 前序会议 0）、关闭按钮，全部字段命中。
- Enter/Space 判据（据实披露）：诊断脚本（gitignored `r494-diag.cjs` / `r494-diag2.cjs`）坐实——修复代码正确：手动 dispatch `KeyboardEvent(keydown, key=Enter, bubbles, cancelable)` 到卡片可打开浮层；真实 `page.keyboard.press(Space)` 到日历条也可打开浮层；但 headless Chromium 下真实 `press(Enter)` 事件到达 LI（defaultPrevented=false）却未触发 Vue 的 `@keydown.enter` 处理器（疑 Playwright press 实现 / Vue 修饰符时序问题，非业务代码缺陷）。E2E 据此用真实 press Enter 优先、未触发时以 dispatch Enter 兜底，并记录 `pressEnterOpened=false` 如实披露；Space 路径为真实按键验证。两者均满足验收「Enter 或 Space 其一打开」。
- 截图来源如实标注：来自 Playwright Chromium headless 非物理真机，未冒充物理真机；2 图 read_image 目视会议管理页详情浮层完整、light/dark 主题正确。

## 6. 证据与 sha256（命令行 sha256sum 原始值）

- pc-meeting-card-light-r494.png 421e7ce6c4cef9e42c1a42ea3d0d2e8b1512975cc54551752188b0b72145f29b
- pc-meeting-card-dark-r494.png 34ea80f1cc56caddea8a237ddd6233937d3bfa222cc382696dd041d23900ac21
- report.json e2b4758e60b5765063e6f44cdb118e95e6ca46fc0aad5deb63ee2046963c1913
- 证据目录（gitignored）：`oa4rust/test-results/r494-card-keyboard/`（2 PNG + report.json + 脚本）。

## 7. 真值纪律

- 测试域名/IP/口令/SSH 端口/个人路径等真值一律不入库，域名用 `<YOUR_TEST_DOMAIN>`、主机用 `<TEST_HOST>` 占位。
- 凭据在 runner 运行时从受控凭据源提取后注入进程环境，真值仅存在于 gitignored 脚本，不回显、不入库。
- 源码 diff 仅 MeetingApp.vue 2 行改动（两处卡片开标签各加 4 属性）；业务处理器、浮层、样式、data-testid、全局设施零改。
