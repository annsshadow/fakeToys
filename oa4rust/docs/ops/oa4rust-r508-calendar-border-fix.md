# R508 日历新建表单边界对比度修复（WCAG 2.1 AA 1.4.11）

## 1. 背景与缺陷

- R506 甄别判 P2-1：日历新建表单三个功能控件（`.cal-create-input`、`.cal-create-color`、`.cal-create-cancel`）边框使用低 alpha 的 `--ai-glass-border`，实测 light 1.23 / dark 1.29 < 3:1；边框是唯一边界指示器（boxShadow / outline / backgroundImage 皆 none），违反 WCAG 2.1 AA 1.4.11 非文本对比度。
- 全局 token 文件（`ai-tokens.css` / `dark.css`）被 129 处引用，严禁修改；`dark.css:40/:110` 的 `--border-subtle` 已由 `nonTextContrast.contract.test.ts` 验证合成到三种背景均 ≥3:1，是本轮复用目标。

## 2. 修复范围（精确3文件）

- `oa4rust/frontend/apps/desktop/src/views/CalendarApp.vue`：仅 `<style scoped>` 内 L818-820 三控件 `border` 颜色令牌 `var(--ai-glass-border)` → `var(--border-subtle)`；L816 装饰性表单容器边框保持原样；border-width/style、border-radius、padding、background、color、模板、事件、数据绑定、data-testid、业务处理器零改。
- `oa4rust/frontend/apps/desktop/src/views/R508.calendarBorderFix.contract.test.ts`：5 条源码字符串断言（2 核心 + 3 守卫）。
- 本文档。

## 3. RED → GREEN 与 verify

- 真父 `719216aee9ac466ce8c3a8d02e5f79f3b7a5ee59` 定点 RED：`pnpm exec vitest run frontend/apps/desktop/src/views/R508.calendarBorderFix.contract.test.ts` EXIT=1，1 failed（核心三控件令牌）/ 4 passed（3 守卫全过，无桩错）。
- 修复版 GREEN：同命令 EXIT=0，5/5 passed。
- V2 `pnpm test`：331 files / 3203 tests，0 failed，EXIT=0。
- V3 `pnpm typecheck`：6 项目 `tsc --noEmit` Done，EXIT=0。

## 4. 真浏览器实测（Playwright headless Chromium 1440×900 DPR1，非物理真机）

- 测试服部署入口 `index-DziFTgzw.js`，同一归档 SHA256 `75e1b815ecd67944a87c03309b3b4d5e006d37d26722f6d6ff760cc3c57f8b3f` 本地/远端一致，DEPLOY_R508_PC_FULL_OK。
- 双主题真实登录 → 日历页 → 点击“新建日历”打开表单 → `getComputedStyle` 取三控件边框颜色，按 WCAG 混合公式合成到外层背景计算对比度：`R508_VERIFY_DONE groups=2 shots=2 errs=0`，`itemsTotal=6, borderMissing=0, notIdentifiable=0, minRatio=3.25`。
- light 三控件 borderRatio=3.25 / dark 三控件 borderRatio=3.46（对照 R506 同三控件 light 1.23 / dark 1.29）；容器 `.cal-create-form` 仍为 `rgba(15, 23, 42, 0.1)`，与 token `--ai-glass-border` 一致，装饰性边框未改。
- CDP AXTree 复核 `.cal-create-cancel` 可达（role/name 由 verify.json 保留）；0 pageerror。
- 截图（真实页面，非物理真机）：`pc-calendar-create-light-r508.png`、`pc-calendar-create-dark-r508.png`，均 1440×900。

## 5. 绕弯披露

- 首次构建误用 `pnpm --filter web build`，无项目命中；改用仓库根目录脚本 `pnpm run build` 后正常产出。非代码缺陷。
- E2E 首轮汇总标记 `formContainerPreserved=false` 是脚本把计算值 `rgba(15, 23, 42, 0.1)` 与 token 原始字面 `#0f172a1a` 直接字符串比较所致；verify.json 明细显示容器边框未改（见上节），属脚本比较口径问题，非回归。

## 6. 证据 SHA256

- `pc-calendar-create-light-r508.png`: `afd1a95202bbc398ae6ae5c54829042feaa42e1ff492a8fa1b7f4f81cc14867c`
- `pc-calendar-create-dark-r508.png`: `13dbaac08c3d41c62cff8987addfe7bdd271cc34062a0a10a5beec3b5fe71166`
- `verify.json`: `17d3fd669477c1d037f446ec06ee54aa33820793fda6d0190a9c51c048b11db4`
- `r508-red.log`: `7c523b175220209b261db4eb36b496b52134ec2aa96dce1b96246bdb779e8886`
- `r508-v1.log`: `38a5dfabee1dda44f05e34bbe6fd6ed588758de53292dbf35f33c890a913ffb5`
- `r508-v2-full.log`: `984836857b2508874ef22421a6ec5c9f4b1529beac4192d33e12af79efa1bb60`
- `r508-v3-typecheck.log`: `f1d438d95cb7702b21ec49e10aa04a5c5c8c298980ffa43ff0e9289c42e53a62`
- `r508-build.log`: `e5c0e1de661aaa61fc56f54a2e878133f8116157bdec3865bf032121dc179b76`
- `r508-pc-deploy.tar.gz`: `75e1b815ecd67944a87c03309b3b4d5e006d37d26722f6d6ff760cc3c57f8b3f`

## 7. 脱敏说明

- 测试服地址按 `<YOUR_TEST_DOMAIN>` / `<TEST_HOST>` 占位；真实域名、账号口令只从远端凭据文件与环境变量读取，未写入仓库、日志或提交。证据目录与脚本均为 gitignored。
