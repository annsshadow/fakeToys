# R101 语义可访问性补全规范（test-first RED）

- 轮次：R101（合并 R101-A aria-expanded 侦察 + R101-B 结果播报侦察）
- 基线 HEAD：`187aeb0eeaba475f9e2eb4bf5e9b74175699973b`
- 上游报告：`docs/laiyipao/recon/ROUND101_A_ARIA_EXPANDED_RECON.md`、`docs/laiyipao/recon/ROUND101_B_RESULT_ANNOUNCE_RECON.md`
- 队长裁定：A 3 处 + B 1 处合并为一轮，共 5 处精确改点（含 1 处契约守卫增补）

## §0 目标与结论

在**不改业务源码**的规范轮，用真实读源码的契约钉死 4 处语义缺口 + 1 处契约守卫增补，产出可复跑的 RED（缺口真失败、不变量真通过）。实施轮只做精确补属性，不动 handler、tokens、engine、后端。

## §1 四条件裁定

| 条件 | R101-A（3 处 aria-expanded） | R101-B（me nicknameMsg） |
| --- | --- | --- |
| ① 真实可感知缺陷 | 真 Chromium 三视口：视觉展开生效（annClamped 3↔2 / radarWrap 0↔1），DOM `ariaExpanded=null`、AX expanded 0/0 | 三视口 AX 树错误文本仅 StaticText；对照实验 + login 阳性对照排除工具误判 |
| ② 权威口径 | WAI-ARIA 1.2 #aria-expanded / MDN：指示控件展开折叠与受控元素显隐 | WAI-ARIA APG Alert/Status Pattern + 仓内 R61 六处范式 |
| ③ 改动面小 | 3 个开标签各 +1 条模板绑定，复用同源响应式状态 | me.vue:100-107 单节点 +2 属性；r61 NODES +1 项 |
| ④ 不破坏不变量 | 保留 R60/R95/R99/R100 三件套与原 @click；令牌不动 | 不动 submitNickname；令牌不动；不影响键盘契约 |

裁定：**四处缺陷全部成立，纳入本轮**；排除项见 §4。

## §2 权威口径

- `aria-expanded`：WAI-ARIA 1.2 `https://www.w3.org/TR/wai-aria-1.2/#aria-expanded`；MDN `https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded`。uni-app H5 编译保留 `aria-*`：本仓库 `aria-label` / `aria-modal=true` 已实测保留。
- 结果播报：`role=status` + `aria-live="polite"` 用于非紧急状态（与同页 `passwordMsg` 同构）；错误场景 `role=alert` 亦被 R61 接受。`nicknameMsg` 节点以 `:style` 的 `nicknameErr` 兼容成功/失败双态，故选 status+polite。
- 仓内先例：`laiyipao/miniapp/src/styles/r61FormA11y.contract.test.ts:64-71` NODES 六处；`:89-103` state/error 断言；`r100ClickableView.contract.test.ts:68-78` 键盘三件套。

## §3 五处精确改点（实施轮执行）

| # | 文件 | 定位 | 改为 |
| --- | --- | --- | --- |
| 1 | `laiyipao/miniapp/src/pages/index/index.vue` | `:111-129` `.ann-item` 开标签 | 补 `:aria-expanded="String(expanded.has(a.id))"`（同源 `:126`/`:128`） |
| 2 | `laiyipao/miniapp/src/pages/enemies/enemies.vue` | `:102` `.resist-toggle` 开标签 | 补 `:aria-expanded="String(expandedEnemy === en.id)"`（同源 `:103`/雷达 `v-if`） |
| 3 | `laiyipao/miniapp/src/pages/battles/battles.vue` | `:181-190` `.battle-row` 开标签 | 补 `:aria-expanded="String(expandedId === it.id)"`（同源 `:185`/`:208`） |
| 4 | `laiyipao/miniapp/src/pages/me/me.vue` | `:100-107` `{{ nicknameMsg }}` text | 补 `role="status"` `aria-live="polite"`（与 `:603-606` passwordMsg 同构） |
| 5 | `laiyipao/miniapp/src/styles/r61FormA11y.contract.test.ts` | `:64-71` NODES | 增补 `['pages/me/me.vue', '{{ nicknameMsg }}', 'state', 'me nicknameMsg']`（契约守卫，非业务源码） |

## §4 明确排除与不变量

- 排除：`bag.vue:140-145` 查看全部配方（导航非折叠受控）、`me.vue:450` link 详情、`mastery.vue:157` `toggle(n)` 选择态（R100 契约排除项）、`battle.vue:181` 帮助图例（已 `role=dialog`）。`uni.showToast` 系统性框架限制不列。
- 不变量：R60/R95/R99/R100 键盘三件套保留；原 `@click` 入口保留；令牌三锚点（`--lyp-token-version: lyp-tokens-v1`、`--lyp-r3-version: lyp-tokens-v3-r3`、`--lyp-r6-version: lyp-tokens-r6`）不动；业务零 `--r101-`；排除项不被误改。

## §5 验证口径与诚实边界

- 契约：`laiyipao/miniapp/src/styles/r101SemanticA11y.contract.test.ts`，9 it（4 缺口 + 1 已完成的契约守卫增补 + 4 不变量），真实 `fs.readFileSync`，按 class 稳定锚点定位开标签，不依赖行号。
- RED 命令：`cd laiyipao/miniapp && npx vitest run src/styles/r101SemanticA11y.contract.test.ts --reporter=verbose`；期望 exit=1、`4 failed | 5 passed (9)`，失败全为真实语义缺口（index/enemies/battles aria-expanded + me nicknameMsg），r61 NODES 守卫因已在规范轮完成故 GREEN；无 PARSE_ERROR/模块解析错误。
- GREEN 期望：9/9；并复跑 R61 `r61FormA11y.contract.test.ts`、R100 `r100ClickableView.contract.test.ts` 全绿。
- 诚实边界：`aria-*` 编译保留仅证 H5（uni-app H5 产物），小程序原生端需另测；`battles .battle-row` 因游客 0 战报未真浏览器坐实，仅源码决定性 + R101-A 静态证据；`nicknameMsg` 成功态当前不渲染文案，status 方案兼容未来双态。
- 本规范轮未改业务源码、tokens、engine、后端；`TestNoRealSecretsInRepo` 不作为本轮 PASS 证据。
