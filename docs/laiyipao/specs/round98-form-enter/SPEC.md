# R98 规范：表单 Enter 提交键盘入口（`@confirm` 双入口对齐）

- 轮次：R98（test-first 规范任务）
- 基线 HEAD：`358901febaaf92ff5543e4f20f379ece52c4ea21`
- 上游：docs/laiyipao/recon/ROUND98_B_SEARCH_ENTER_RECON.md（真 Chromium 三视口坐实）
- 性质：只写规范/契约/RED 证据；不改业务 vue、令牌、其他契约、.gitignore

## §0 背景与四条件裁定

R98-B 侦察在 H5 搜索输入框的 Enter/确认键提交路径上列 **1 个推荐候选（四条件全中）**：3 个「输入即提交」表单缺少 `@confirm` 键盘入口，输入后按 Enter 无任何反应；真 Chromium 三视口实测 signin 按 Enter `reqs=[]`，而点击「兑换」按钮发 `/api/v1/redeem`。login 阳性对照 `@confirm="submit"` 按 Enter 触发 `/api/v1/auth/login`，证明 H5 编译产物下 `@confirm` 有效。

- ① 真实用户可见缺陷：成立——键盘用户填完表单按 Enter 无提交反馈，鼠标用户可点按钮，路径不对等。
- ② 可定位到具体可修复点：成立——3 处 input 各缺 1 行 `@confirm`。
- ③ 修复不越界：成立——每处仅 +1 行模板属性，复用现有 handler，零新增 DOM/逻辑/依赖。
- ④ 不与既有范式冲突：成立——同仓 login/me 已是「Enter + 按钮」双入口范式（r69 契约明文守卫）。

裁定：立案。以 test-first 先落 RED 契约，再交由实施轮补 3 行 `@confirm`。

## §1 权威口径

- `r69NicknameBusy.contract.test.ts:86-88`：明文守卫 `@confirm="submitNickname"` 与 `@click="submitNickname"` 双入口并存。
- `login.vue:56`：`@confirm="submit"`（阳性对照，真浏览器坐实有效）。
- `me.vue:95/592/644`：`@confirm="submitNickname"/"submitPassword"/"submitUpgrade"` 三处双入口。
- `r61FormA11y.contract.test.ts:43-61`：只守 17 处 input 的 `aria-label`，补 `@confirm` 不触碰该不变量。
- `r55VisualReadability` / `r5:151`：分别守 placeholder 与 `fetchReplay` 存在，补 `@confirm` 不影响。

## §2 精确改点（实施轮执行，每处仅 +1 行）

| 页面 | input（v-model） | 补属性 | 复用 handler | 原按钮入口 |
|---|---|---|---|---|
| laiyipao/miniapp/src/pages/signin/signin.vue:203-211 | code | `@confirm="doRedeem"` | doRedeem | :212 `@click="doRedeem"` |
| laiyipao/miniapp/src/pages/verify/verify.vue:53-61 | battleId | `@confirm="fetchReplay"` | fetchReplay（:451 有 `if(fetching.value)return` 守卫） | :69 `@click="fetchReplay"` |
| laiyipao/miniapp/src/pages/bag/bag.vue:405-411 | presetName | `@confirm="doSavePreset"` | doSavePreset | :416 `@click="doSavePreset"` |

零新增 DOM/逻辑/依赖；`@confirm` 属性值与 `@click` 指向同一 handler，保持双入口一致。

## §3 不变量清单

1. 三处原 `@click` 提交入口仍在（signin `doRedeem` / verify `fetchReplay` / bag `doSavePreset`）。
2. r69 双入口范式仍成立（me 三处 + login 一处 `@confirm` 均在）。
3. 令牌三锚点不动：tokens.css:14/15/16 的 `--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`。
4. 业务源码零 `--r98-` 令牌（不引入新视觉令牌）。
5. `aria-label` / `placeholder` / `type=number` 等既有属性不动（r61/r55 不变量）。

## §4 验证口径

- 契约：`pnpm vitest run src/styles/r98FormEnter.contract.test.ts --config vitest.config.ts`（在 laiyipao/miniapp 下）。
- RED（本轮）：3 项缺口失败（`@confirm` 缺失），6 项不变量通过；exit code 1。
- GREEN（实施轮）：9/9 通过，exit 0。
- 全量回归：admin/miniapp 相关契约与 type-check 由复查/终审轮负责。

## §5 RED 证据摘要

见 docs/laiyipao/specs/round98-form-enter/RED-evidence.md（逐字日志、exit code、failed/passed 计数）。
契约真实 `fs.readFileSync` 读三页源码，禁止内嵌实现副本；用 `inputBlock`（r61 同款）稳健定位 input 开标签块。

## §6 诚实边界

- 本轮只产出规范/契约/RED 证据，未改业务源码；三处 `@confirm` 缺口真实存在，GREEN 由后续实施轮达成。
- R98-B 侦察的浏览器证据为上游材料，本轮未重跑浏览器；真浏览器 GREEN 坐实由复查轮负责。
- 全程零真值入库，未回显 token / 域名 / IP / 个人路径。
