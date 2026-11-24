# R446 PC R300 B5 契约 6 页 AI 风格现状与三标记体系一致性甄别（只读）

- 轮次：R446（任务 t298）｜角色：qa-e2e 只读甄别（design-lead 执行）
- 分支：opt1000/oa4rust-pc-ai-refactor｜真父提交：b39235326c1aca63a3e86a6a34e604550efcd2f5
- 性质：只读审计。不改任何业务/样式代码，仅产出本文档；发现问题据实记录，修复留后续轮次。
- 测试入口域名：`<YOUR_TEST_DOMAIN>`（占位，真值不入库）
- 截图来源标注：Playwright Chromium headless 1440x900 → 测试服真实页面（NOT a physical device，非物理真机）

## 1. 甄别范围与三分判据

R441–R445 已把 28 个管理页纳入 `modern-page ai-page-b5 ai-panel` 三标记体系（样式由外部 ui 包承载）。另有 6 页走 R300 独立 B5 方案：各自根容器带 `ai-page-b5`，并内嵌完整 `<style scoped>` `.ai-page-b5` 样式块。

| 页面 | 路由（/oa4rust/app/admin/） | 根容器 class | data-testid | scoped 样式块 |
| --- | --- | --- | --- | --- |
| RoleManager.vue | role | role-view modern-page ai-page-b5 | role-page | L235 |
| ThreeMemberApp.vue | three-member | crud-view ai-page-b5 | three-member-page | L250 |
| CmsDictDesignerApp.vue | cms-dict-designer | crud-view ai-page-b5 | cms-dict-page | L220 |
| ControlPanelApp.vue | control-panel | nav-view ai-page-b5 | control-panel-page | L122 |
| ConfigDesignerApp.vue | config-designer | editor-view ai-page-b5 | config-designer-page | L507 |
| LogViewerApp.vue | log-viewer | crud-view ai-page-b5 | log-viewer-page | L117 |

三分判据：①视觉与标记已自洽 B5 → 判 0 缺陷（不重复套用三标记）；②存在视觉未生效/标记不一致/有实质改进价值 → 据实列具体问题页与证据；③追加标记有契约破坏或样式冲突风险 → 据实记录。逐页区分模板结构/可达性/只读展示/死代码，EndpointFold 折叠区内写链不据以判视觉。

## 2. 契约保护现状（静态核实，非最终结论）

`frontend/apps/desktop/src/views/AdminZone.aiStyle.B5.contract.test.ts`（83 行）锁定 6 页：:60 断言模板 class 含 `ai-page-b5`、:61 含 `ai-gradient-text`、:65-74 保留 data-testid（role-page / role-refresh / role-create / role-members / role-edit / role-delete / log-viewer-page / config-designer-page）、:76-82 模板不含 `/api/` 与 `{{ item.personId }}`。实测 `cd oa4rust && pnpm exec vitest run ...AdminZone.aiStyle.B5.contract.test.ts` → 5 passed / 5，EXIT=0。

6 页在 `frontend/apps/desktop/src/main.ts`（role L206-209 / log-viewer L248-251 / cms-dict-designer L320-323 / control-panel L422-425 / config-designer L428-431 / three-member L464-467）均注册中文 meta.title 路由；`components/admin/adminNav.ts`（L24/L25/L44/L102/L106/L108）有对应导航项。6 页真实可达。

## 3. 真浏览器逐页实测（6 路由，headless 1440x900，非物理真机）

登录态从测试服凭据文件经 ssh 运行时读取并注入环境变量（真值不入库、不回显）。E2E 主脚本 EXIT=0，`errors=[]`（0 pageerror）。逐页结果：

| 页面 | rootCount | b5TitleCount | 渐变标题文字 | b5TitleGradient | aiGradientTextCount | glassCardCount | skelCount |
| --- | --- | --- | --- | --- | --- | --- | --- |
| role | 1 | 1 | 管理角色、Flag 与成员分配 | true | 3 | 12 | 0 |
| three-member | 1 | 1 | 三方成员管理 | true | 3 | 3 | 0 |
| cms-dict-designer | 1 | 1 | CMS 字典设计器 | true | 3 | 3 | 0 |
| control-panel | 1 | 1 | 控制面板 | true | 3 | 3 | 0 |
| config-designer | 1 | 1 | 配置设计器 | true | 3 | 3 | 0 |
| log-viewer | 1 | 1 | 日志查看器 | true | 3 | 3 | 0 |

说明：`skelCount=0` 为页面数据加载完成后的稳态，骨架屏仅在加载中短暂出现，属正常，不判缺陷。

## 4. 决定性发现：根容器渐变令牌词序颠倒（实质样式失配）

截图目视 6 页整体已是现代浅色 B5 风格（渐变标题文字、卡片边框、柔和阴影、圆角、导航/表格/空态完整，无视觉崩坏）；但 computedStyle 深挖坐实一处样式失配：

- 6 页 scoped 样式块根容器统一写 `background: var(--ai-surface-gradient)`；`frontend/packages/ui/src/theme/ai-tokens.css` 全文没有 `--ai-surface-gradient` 定义，真实令牌是 `--ai-gradient-surface`（词序相反，:31 暗色 / :74 亮色，值为 `linear-gradient(160deg, rgba(99,102,241,0.12), rgba(34,211,238,0.04))`）。
- 浏览器实测（role 页根容器）：`getComputedStyle(root).getPropertyValue('--ai-surface-gradient')` = 空字符串；`--ai-gradient-surface` = `linear-gradient(160deg, #4f46e50f 0%, #0891b205 100%)`。根容器 `backgroundImage=none`、`backgroundColor=rgba(0,0,0,0)` → 根容器渐变背景未生效。
- 同块内其余令牌均生效：根容器 `border=1px solid rgba(15,23,42,0.1)`、`borderRadius=12px`、`boxShadow=rgba(15,23,42,0.1) 0 4px 16px`（对应 `--ai-glass-border`/`--ai-shadow-2`/`--ai-radius-lg` 缺失时的 12px fallback）；渐变标题 `.b5-title` `backgroundImage=linear-gradient(120deg,#4338ca,#0e7490)`、`webkitTextFillColor=transparent`（对应 `--ai-gradient-text`，渐变文字清晰可见）。
- 归因：scoped 样式块引用了不存在的令牌名，浏览器把 `background: var(--ai-surface-gradient)` 判为无效声明并丢弃；其余令牌名正确故生效。影响级别：轻微（卡片本身已有层次，肉眼观感仍现代），但属真实、可复现、可据证的样式失配，非臆造。
- 排除项（不作为判据）：① `deep2` 中 `rootGlass=null` —— role 页根容器内无 `.glass-card` 目标元素，E2E 主脚本 `glassCardCount 3~12` 统计的是 AppShell 全局卡片，scoped `.glass-card` 的 `backdrop-filter` 规则在这些页面无作用对象；② `b5CssRuleCount=0` —— Vite 生产把 scoped CSS 外链到 `/assets/*.css`，`document.styleSheets` 跨源不可遍历，不能据此判 scoped 未生效（边框/阴影/圆角/渐变标题正是 scoped 块规则且已生效，构成反证）。

## 5. 逐页判定与标记建议

| 页面 | 视觉 B5 化 | 渐变标题 | 玻璃/卡片质感 | 渐变背景 | 建议追加 modern-page/ai-panel | 理由 |
| --- | --- | --- | --- | --- | --- | --- |
| RoleManager | 是（完整 B5 观感） | 生效 | 生效（边框/阴影/圆角） | 未生效（令牌失配） | 否 | 已有 role-view modern-page + 完整 scoped B5；叠加 ai-panel 会与外部 ui 的 padding/background 冲突 |
| ThreeMemberApp | 是 | 生效 | 生效 | 未生效（同因） | 否 | crud-view ai-page-b5 自洽；两套体系语义重复 |
| CmsDictDesignerApp | 是 | 生效 | 生效 | 未生效（同因） | 否 | 同上 |
| ControlPanelApp | 是 | 生效 | 生效 | 未生效（同因） | 否 | nav-view ai-page-b5 自洽；stats-row 也带 b5 标记 |
| ConfigDesignerApp | 是 | 生效 | 生效 | 未生效（同因） | 否 | editor-view ai-page-b5 自洽 |
| LogViewerApp | 是 | 生效 | 生效 | 未生效（同因） | 否 | crud-view ai-page-b5 自洽 |

共性判定：6 页同源于 R300 独立 B5 体系，问题是同一个令牌词序颠倒（6 处 scoped 块都写错），不是 6 个独立缺陷。修复方向应为后续轮次把 6 页 scoped 块的 `var(--ai-surface-gradient)` 统一改为 `var(--ai-gradient-surface)`（或令牌层补别名），而非追加三标记。本轮为只读甄别，不改码。

## 6. 追加三标记的契约/样式冲突风险评估

- 契约破坏风险：低。`AdminZone.aiStyle.B5.contract.test.ts` 断言的是保留 ai-page-b5 与 data-testid，追加标记不删原标记，断言不破。
- 样式冲突风险：中。6 页 scoped 块用 `.ai-page-b5.ai-page-b5, .ai-page-b5 .ai-page-b5` 高特异性选择器锁背景/边框/阴影/圆角；外部 ui 的 `.ai-panel` 带 padding/背景/玻璃语义，叠加后可能覆盖或双重叠加本地视觉；且 R441-R445 三标记是给裸 ai-page 页补齐的，这 6 页本就有完整 B5 样式，追加属体系重复。
- 结论：不建议追加 `modern-page`/`ai-panel`。保持 R300 独立 B5 体系，仅修复令牌失配即可。若队长决策统一三标记，需先处理作用域样式优先级并补契约回归。

## 7. 证据：6 截图 sha256（命令行 `sha256sum *.png` 原始值，非物理真机）

```
18ca1af4e1c6bd35e7c7a629e61788821253729e72052837ffe813dd0bcaf526  cms-dict-designer.png
8da9f20583f2f4d78a900acf8b09bbd6ad4da692b7b44203e921ded82b3c29e4  config-designer.png
12a660cc6e782d5fb5ccd1b447ea2b2dc747ac41657ed5047fb2d775366ff569  control-panel.png
8c3ebd8351f85ed1e362194be1f9d5f2aea73f18ea0c45797869b47212e0f2dc  log-viewer.png
3047556bd5e5cb52328d655b944cd0fe971e1b1d783b5789271787bc8068fff3  role.png
55b636d1b6b6509e49c96c2508c0ca8d927fb114ecb285f8fee402585a1f00fa  three-member.png
```

证据目录：`oa4rust/test-results/r446-r300-b5-audit/`（gitignored，6 png + report.json + deep.json + deep2.json）。

## 8. 结论

6 页 R300 B5 页面在真浏览器 headless 1440x900 下整体已是现代 B5 风格、无视觉崩坏，渐变标题/边框/阴影/圆角/导航/空态均实际生效；判据②成立：存在 1 个共性问题——6 处 scoped 根容器渐变误引 `--ai-surface-gradient`（应为 `--ai-gradient-surface`）导致根容器渐变背景未生效，已用 computedStyle + 令牌实测 + 源码三向坐实。判据③成立：不建议追加三标记（契约不破但样式冲突风险中、体系重复）。本轮只读不改码，修复建议留后续轮次。≥1000 轮持续迭代中一环，非终态。

## 9. 真值纪律

- 文档用占位域名 `<YOUR_TEST_DOMAIN>`；口令/私钥/会话数据/真实域名与 IP 不入库，凭据经 ssh 运行时注入环境变量。
- 截图与临时脚本留在 gitignored `test-results/` 证据目录，来源标注非物理真机。