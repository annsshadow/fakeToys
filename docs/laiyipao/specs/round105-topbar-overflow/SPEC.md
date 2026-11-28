# R105 规范：index 顶栏 768 横溢修复（test-first RED）

- 任务：t290 / attempt_id=bb80c86b-73ca-4901-bb34-9893ab704d8e
- 基线 HEAD：0151ce97e1216d70efd899e85d72e7ad2f93b750
- 上游：t289 R105-C 只读验证侦察 docs/laiyipao/recon/ROUND105_C_TOPBAR_OVERFLOW_VERIFY.md（四条件全中）
- 性质：规范 + 契约 RED。本轮不改业务 vue / 令牌 / 其它契约 / .gitignore。

## §0 裁定与权威口径

采纳 R105-C 结论：768 平板宽度下 index 顶栏真实布局缺陷成立（真实量级 coin=52000 即 sw=780 > cw=768，溢出 12px；合成 7 位加剧至 810），非超长数字伪影。权威口径取 docs/laiyipao/design/AI_VISUAL_SYSTEM.md §7「H5 限宽 560px 居中 + P7 响应式支柱」与历史 R98–R104 零横溢必测项（全库 39 处）。A 报告 ROUND105_A 称无 AI_VISUAL_SYSTEM.md 属事实错误，不采纳。

## §1 修复目标（逐字锁死）

- 768×1024 视口下 index 顶栏：`iw=cw=sw`、`hOverflow=false`。
- 玩家 nick 与「游客账号」不得异常断行；第四钥匙胶囊「🔑N」不得被裁切。
- 390×844 与 1280×800 保持零横溢不回归。
- 不改任何业务模板文本、密钥令牌、页面结构；只改 index.vue 三处局部 CSS 声明。

## §2 根因（R105-C §4 决定性）

`index.vue:744-757 .topbar` 在 H5 被限宽 560px 居中（实测 w=560 / left=104 / right=664），`display:flex; justify-content:space-between` 下左侧 `.player`（含 `.player-info`:779-782 无 `min-width:0`）与右侧 `.res-row`（:789-794，`white-space:nowrap`:793 强制四胶囊不换行）合计约 651px > 560px；flex 子项默认 `min-width:auto` 不肯收缩，内容撑破限宽容器：resRow.right=780 > topbar.right=664，其中 12px 越出视口 768。

## §3 唯一精确改点（三处，均在 laiyipao/miniapp/src/pages/index/index.vue 的 <style scoped> 内）

规范轮定夺采用 R105-C 建议的三点组合，理由：三点各自对应根因的一个环节（左不可缩 / 右硬撑 / 容器无兜底），互相独立且都不触碰模板结构；相比「单处 flex-wrap 于 .topbar」会把整个 res-row 挤到第二行、改变首屏视觉结构，本组合在窄屏只让胶囊内部换行、桌面/手机态零改动，风险更低。不采用单处改法。

1. `.player-info`（基线 779-782）增 `min-width: 0;` —— 允许玩家信息在窄容器收缩，解除 flex 子项 min-width:auto。
2. 新增 `@media (max-width: 900px)` 媒体块，内含 `.res-row { flex-wrap: wrap; }` —— 平板宽度允许四胶囊换行，解除硬撑；基础块 `white-space: nowrap` 保留不动（390 手机态与 1280 桌面态零横溢已被坐实）。媒体块中 `flex-wrap: wrap` 为唯一指定目标（契约锁定）；若实施轮发现 `white-space: normal` 或缩 `gap` 等价更优，须先回报队长改规范后实施，不得静默替换。
3. `.topbar`（基线 744-757）增 `max-width: 100%; box-sizing: border-box;` —— 限宽容器内溢出兜底。

改点须保持：`.topbar` 的 `display: flex` 与 `justify-content: space-between`；`.res-row` 基础块 `white-space: nowrap`；四胶囊插值 `store.wallet.coin/gem/energy/keys` 原样保留。

## §4 不变量（契约必须守）

- 不改 index.vue 任何模板文本与键盘三件套：R99 `go(d.hint)`、R102 retry / ann-all / goStage / go(aiInsight.hint) 的 `role="button"` + `tabindex="0"` + `@keydown.enter` 同 handler 全部保留。
- R100/R101 `.ann-item` 三件套 + `:aria-expanded="String(expanded.has(a.id))"` 保留。
- tokens.css:14-16 三锚点 `--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6` 不动。
- 业务源码（index.vue / tokens.css）零 `--r105-` 标记；不动其它页、其它契约、.gitignore。
- 既有 a11y 契约 r60 / r99 / r100 / r101 / r104 保持通过（实施轮 QA 复跑）。

## §5 契约与验证口径

- 契约：`laiyipao/miniapp/src/styles/r105TopbarOverflow.contract.test.ts`，真实 `fs.readFileSync` 读 `pages/index/index.vue` 与 `styles/tokens.css`，以唯一 class 锚点（`.topbar {` / `.player-info {` / `.res-row {` 各全文件恰好出现一次，否则定位返回 null 使测试自身失败）定位 CSS 块，并按 `@media (max-width: 900px)` 定位媒体块；不依赖行号。
- 10 it = 3 缺口 it + 7 不变量 it；RED 期望 exit=1、3 failed | 7 passed；缺口 it 全部真实失败（因业务尚未改），不变量 it 全绿。
- RED 命令：`cd laiyipao/miniapp && npx vitest run src/styles/r105TopbarOverflow.contract.test.ts --reporter=verbose`，原始输出落 `docs/laiyipao/specs/round105-topbar-overflow/RED-raw.log`，逐 it 期望/实际落 `RED-evidence.md`。
- 实施轮 + QA 须以真 Chromium 三视口 390/768/1280 复验 `iw=cw=sw`、`hOverflow=false`，并复跑 §4 全部既有契约。

## §6 诚实边界

- 本轮仅规范 + RED，业务 CSS 尚未修改；契约断言的是源码声明存在性，不替代真浏览器横溢实测（实测属实施轮 QA）。
- 令牌三锚点、既有 a11y 契约、业务结构均声明为不变量，实施轮若违反即判回退。
