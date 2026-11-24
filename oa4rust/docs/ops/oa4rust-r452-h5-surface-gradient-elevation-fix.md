# R452 H5 .ai-surface-gradient 根容器渐变提权修复

## 1. 来源与缺陷
R451（design-lead t306，commit f68b6abe7）真浏览器 390x844 逐页甄别坐实可达真缺陷：
13 个页面根容器 `.ai-surface-gradient` 的全局类特异性 (0,1,0) 被页面 scoped `.page[data-v]` (0,2,0) 的 `background: var(--color-bg-page)` 覆盖，computedStyle `backgroundImage=none`，AI 柔和渐变未生效；另 6 页本已生效。

## 2. 修复（仅 App.vue CSS，业务零改）
仿 App.vue:300-303 既有 `.ai-brand-fill` 提权模式，在 `.ai-surface-gradient` 单点定义后新增：

```css
uni-page-body .ai-surface-gradient.ai-surface-gradient,
body .ai-surface-gradient.ai-surface-gradient {
  background: var(--ai-gradient-surface);
}
```

- 容器前缀 + 重复类达特异性 (0,2,1)，稳定压过 scoped `.page[data-v]` (0,2,0)。
- 令牌词序正确 `--ai-gradient-surface`（与 brand-fill 一致），未引入 R446/R448 已清零的错误令牌 `--ai-surface-gradient`。
- 保留原 `.ai-surface-gradient { background: var(--ai-gradient-surface) }` 单点定义不删；13 页 scoped 背景声明保留（CSS 提权不删业务样式）。
- 19 个页面 .vue、services、接口、data-testid、ai-visual.contract.test.ts 均零改动。

## 3. test-first RED → GREEN
新增专门契约 `oa4rust/mobile/src/surfaceGradientElevation.R452.contract.test.ts`（6 用例）：全局单点定义/令牌词序正确、双前缀重复类提权 (0,2,1)、令牌正确且不引入错误令牌、19 页 ai-surface-gradient 锚点不回退、13 页 scoped 背景仍在、brand-fill 提权不回退。
- 真父 b7629c5ef（无提权规则）RED：2 failed / 4 passed。
- 修复版 GREEN：6 passed / 6。

## 4. 三 verify（exit 0）
1. `cd oa4rust/mobile && pnpm exec vitest run src/surfaceGradientElevation.R452.contract.test.ts` → 6 passed。
2. `cd oa4rust/mobile && pnpm test` → 77 files / 508 tests passed，0 failed。
3. `cd oa4rust/mobile && pnpm typecheck` → tsc --noEmit exit 0。

## 5. 构建与测试服部署
H5 构建成功，入口 `index.html` 引用 `index-DAJC6bDY.css`；提权规则已进入产物 CSS。整目录原子部署测试服（含护栏：当前目录不存在则拒绝切换），远端 index.html 引用与 CSS 提权规则 grep 均命中。

## 6. 真浏览器 E2E（非物理真机）
本地无头 Chromium，390x844、isMobile/hasTouch、DPR=2，忽略证书错误，经 `<YOUR_TEST_DOMAIN>` 测试服真实 UI 登录流程后逐页采样 19 路由（凭据运行时从本机读取，不入库不回显）。
- 13 页原失效页根容器 computedStyle `backgroundImage` 从 `none` 恢复为 `linear-gradient(160deg, rgba(79, 70, 229, 0.06) 0%, rgba(8, 145, 178, 0.02) 100%)`。
- 6 页原生效页保持渐变不回退；brand-fill 渐变与 disabled 灰显业务语义零回归。
- RESULT：loggedIn=true、total=19、bad=0、pageErrors=0、apiFailures=0、passed=true。
- 来源声明：**本地无头浏览器渲染，非物理真机**；非静态/假数据拼图。
- 证据在 gitignored 目录 `oa4rust/test-results/r452-h5-surface-elevation/`（evidence.json + 19 PNG + manifest.md），19 截图 sha256 命令行原始值见 manifest。

## 7. 写域与脱敏
精确 3 文件：`oa4rust/mobile/src/App.vue`、`oa4rust/mobile/src/surfaceGradientElevation.R452.contract.test.ts`、本文档。
真实域名用 `<YOUR_TEST_DOMAIN>` 占位，凭据运行时读取，0 真值入库。

## 8. 迭代声明
本项目 ≥1000 轮持续迭代，本轮为其中一环，非终态。
