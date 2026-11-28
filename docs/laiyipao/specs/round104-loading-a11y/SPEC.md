# R104 规范：骨架屏加载态 a11y 语义补全

## §0 轮次定位与范围

- 基线 HEAD=`b4e0194ca46de1bfe022315b289ee5fc9e6a0a9c`（规范产出时核实未变）。
- 上游：t280 R104-A 只读侦察（报告 `docs/laiyipao/recon/ROUND104_A_LOADING_A11Y_RECON.md`），裁定四条件全中、有推荐候选（1 组两类共 10 页）。
- 与 ROUND89_B（四态迁移/布局，无候选）为不同维度：本轮专治**语义**（屏幕阅读器可感知加载态），不重复立案。
- 唯一写域：`docs/laiyipao/specs/round104-loading-a11y/`（SPEC.md / RED-evidence.md / RED-raw.log）与 `laiyipao/miniapp/src/styles/r104LoadingA11y.contract.test.ts`。不得改任何业务 `.vue`、`.gitignore`、令牌或其他契约。

## §1 问题与四条件裁定

静态决定性：miniapp 全仓 `aria-busy` 零命中；全仓 `aria-*` 命中 50（证明 uni-app H5 编译保留 aria-*）。10 页加载根对屏幕阅读器完全不可感知：

- 7 页 `v-if="loading"` 裸 view（「加载中…」），role/busy/live 全 null。
- 3 页 `v-if="r6Loading"` 骨架根带 `aria-hidden="true"`，被显式从无障碍树剥离。

真 Chromium 时序 harness（`AGENT_BROWSER_INIT_SCRIPTS` 延迟 XHR/fetch 响应 8000ms）坐实：DOM 属性全 null + AX snapshot 仅 generic/button/textbox，无加载节点；全非登录页、零横溢。

| 条件 | 判定 | 依据 |
|---|---|---|
| ① 真实用户可见可感知缺陷 | 成立 | 屏幕阅读器用户在加载期听不到任何「数据加载中」播报；r6 三页被 aria-hidden 主动剥离 |
| ② 有权威/同仓口径 | 成立 | MDN aria-busy（signal ongoing changes / loading，true→false）+ 同仓 admin `PageSkeleton.vue:10` role=status aria-busy=true aria-label=数据加载中、`AiInsightCard.vue:19` 动态 aria-busy；守卫 `PageSkeleton.contract.test.ts:26/30`、`AiInsightCard.test.ts:58/60` |
| ③ 改动面小且局部 | 成立 | 10 页各 1 个开标签补属性（B 类另去 1 个 aria-hidden），零新增 CSS 类、零结构改动 |
| ④ 不破坏现有范式 | 成立 | 不动 R60/R100/R101 键盘三件套与 aria-expanded；令牌三锚点不动；业务零 --r104-；契约设不变量 it 锁死 |

裁定：立轮，按下列口径固化契约（RED 先行）。

## §2 修复口径（逐字锁定）

**A. 7 页文字/骨架裸 view 加载根**，各补三属性 `role="status" aria-busy="true" aria-label="数据加载中"`：

- `laiyipao/miniapp/src/pages/announcements/announcements.vue:43`（v-if="loading" class="card"）
- `laiyipao/miniapp/src/pages/bag/bag.vue:36`（同上）
- `laiyipao/miniapp/src/pages/battles/battles.vue:164`（同上）
- `laiyipao/miniapp/src/pages/mastery/mastery.vue:45`（同上）
- `laiyipao/miniapp/src/pages/rank/rank.vue:62`（同上）
- `laiyipao/miniapp/src/pages/shop/shop.vue:140`（同上）
- `laiyipao/miniapp/src/pages/signin/signin.vue:36`（同上）

**B. 3 页 r6Loading 骨架根**，去掉 `aria-hidden="true"` 并补同三属性 `role="status" aria-busy="true" aria-label="数据加载中"`：

- `laiyipao/miniapp/src/pages/equipment/equipment.vue:45`（v-if="r6Loading" class="r6-skeleton r6-enter"）
- `laiyipao/miniapp/src/pages/reactions/reactions.vue:62`（同上）
- `laiyipao/miniapp/src/pages/skills/skills.vue:63`（同上）

理由：骨架 bar 是空 view（仅 `style="width: …"`、无文字内容），去 aria-hidden 后读屏只读 status label，不会读出噪音；零新增 sr-only CSS，符合最小改动。不引入新 CSS 类、不改骨架结构。

**C. 三属性静态写死（非 `:aria-busy` 绑定）**：因 v-if 节点卸载即天然等价 `aria-busy=false`——加载根只在 `loading`/`r6Loading` 为真时挂载，真值期恒 `aria-busy="true"`；为假时整节点从 DOM 与无障碍树同时消失，读屏不再有该 status 区域，语义等价于 busy 结束。故无需动态绑定或额外 sr-only。

**D. 不变量**：不动 R60/R100/R101 键盘三件套（`role="button"` + `tabindex="0"` + `@keydown.enter` 同名 handler）与 `:aria-expanded`；不动令牌三锚点 `tokens.css:14-16`（`--lyp-token-version: lyp-tokens-v1` / `--lyp-r3-version: lyp-tokens-v3-r3` / `--lyp-r6-version: lyp-tokens-r6`）；业务源码零 `--r104-`。

## §3 契约设计

- 文件：`laiyipao/miniapp/src/styles/r104LoadingA11y.contract.test.ts`，真实 `fs.readFileSync` 读 10 个 `.vue` + `tokens.css`。
- 定位：`tagBefore(src, needle, tag)` 按唯一 class + v-if 锚点向前找最近 `<view`、向后找最近 `>`，跨行开标签容错，**不依赖行号**。A 类锚点 `v-if="loading" class="card"`（全仓恰好 7 处），B 类锚点 `v-if="r6Loading" class="r6-skeleton r6-enter"`（全仓恰好 3 处）。
- 13 it = 10 缺口（A×7 + B×3，实施前须失败）+ 3 不变量（令牌三锚点+零 --r104-；键盘三件套抽查 3 处；R101 aria-expanded 与既有空态语义不被误改）。
- 缺口断言：加载根开标签须同时含 `role="status"` / `aria-busy="true"` / `aria-label="数据加载中"`；B 类另断言不含 `aria-hidden="true"`。

## §4 验证命令

```bash
cd laiyipao/miniapp
npx vitest run src/styles/r104LoadingA11y.contract.test.ts --reporter=verbose
```

RED 预期：exit=1，10 failed | 3 passed (13)；失败 it 全为缺口断言，3 不变量 it 通过。GREEN 预期（实施轮）：13/13。

## §5 诚实边界

- 本规范轮只写 SPEC + 契约 + RED 证据，不改任何业务 `.vue`；实施与真浏览器复查另轮。
- battles 页骨架加载态因游客 0 战报在真浏览器未坐实，仅源码决定性（与 R104-A 一致）。
- AX 无加载语义专用 API，侦察以 DOM 属性 + snapshot refs 双证据；实施轮复查沿用。
