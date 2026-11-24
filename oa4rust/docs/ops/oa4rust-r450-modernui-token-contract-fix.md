# R450 PC ModernUI 不可达组件令牌修复与契约收紧

## 缺陷与根因

- 甄别轮 R448 坐实：oa4rust/frontend/apps/desktop/src/components/admin/ModernUI.vue:10 的 .ai-b5-surface 背景误用 var(--ai-surface-gradient)，该令牌在 ai-tokens.css 全部 29 种 --ai-* 定义中不存在（正确名为 --ai-gradient-surface，暗色 :31 / 亮色 :74）。
- 浏览器丢弃无效声明，根容器 backgroundImage 退化为 none；该组件无任何 import / 注册 / 模板渲染入口，6 条相关路由均未挂载 .ai-b5-surface（不可达 L1），属契约正确性与视觉令牌卫生缺陷，非用户可见故障。
- 6 个 R300 shared 原语中，ModernUI 是唯一误引者。

## 修复（业务零改）

- 唯一源码改动：ModernUI.vue:10 background: var(--ai-surface-gradient) → var(--ai-gradient-surface)（git diff 仅此 1 行，index de1a2bbc..62a8d4db9）。
- 模板 / 插槽 / 端点 / payload / 后端 / SDK / 其它 view 全部零改；未夹带 CalendarApp 或任何业务代码。

## 契约收紧（防回归）

- ModernUI.contract.test.ts 新增断言：.ai-b5-surface 规则体背景必须为 var(--ai-gradient-surface)，且全文件不含 --ai-surface-gradient。
- AdminZone.aiStyle.B5.contract.test.ts 新增通用守卫：从 ai-tokens.css 提取定义集，扫描 6 个 shared 原语中无 fallback 的严格 var(--ai-NAME) 引用，要求全部命中定义集；带 fallback 的合法降级写法（如 var(--ai-radius-lg,12px)）不受影响，真父版既有 11 个 it 全过证明未误伤。

## 验证

- 真父 ff205395c（源码未改、仅新增契约）定点 RED = 2 failed / 11 passed（共 13），失败均为真实断言：AdminZone 报 ModernUI.vue:--ai-surface-gradient 应为 []，ModernUI 报错误令牌不匹配正确正则。
- 修复版 GREEN：定点 13/13；根全量 pnpm test 315 files / 3113 tests / 0 failed；desktop pnpm exec tsc -p frontend/apps/desktop/tsconfig.json --noEmit exit 0。三条 verify 全 exit 0。
- 构建：根 pnpm run build exit 0，入口 index-Bk7Zfwmt.js（与 R449 相同：不可达组件被 tree-shake 反证 L1），打包 CSS 中 ai-b5-surface 命中 0。
- 部署：整目录 tar → scp 至测试服 → sha256 比对一致 → 远端 /usr/local/oa4rust 原子替换，DEPLOY_R450_PC_FULL_OK。

## 真浏览器实测（决定性对照）

- Playwright Chromium headless 1440x900（非物理真机），登录测试服后进入 /oa4rust/app/admin/control-panel。因组件被 tree-shake 不可达，采用注入同源 scoped 规则的方式对令牌解析做决定性对照，据实标注：这验证的是令牌解析语义，非用户可见页面缺陷。
- 断言 9 项全通过：错误令牌 computedStyle 解析为空串，正确令牌解析为 linear-gradient(160deg,#4f46e50f 0%,#0891b205 100%)；父版注入背景 none → 修复版 linear-gradient(160deg,rgba(79,70,229,0.06) 0%,rgba(8,145,178,0.02) 100%)；页面标题「控制面板」，0 pageerror。
- 证据：test-results/r450-pc-modernui-token-fix/report.json 与 modernui-token-probe.png（1440x900 真 OA4RUST admin 控制面板，注入渐变探针可见）。截图 sha256=40671cdcba534b87b36799fae13f1fe7fa200dbb6e9cb01c991c5939b7609ea2。
- 测试服地址：<YOUR_TEST_DOMAIN>（主机 <TEST_HOST>）均为占位符，真实值由部署环境注入，未入库。

## 写域与提交

- 精确 4 文件：ModernUI.vue、ModernUI.contract.test.ts、AdminZone.aiStyle.B5.contract.test.ts、本文档；无其它文件改动。
- 按队长统一安排，本轮仅本地 commit、不执行 push（本地领先远端 2 提交，由队长补推）；敏感扫描 0 命中。
