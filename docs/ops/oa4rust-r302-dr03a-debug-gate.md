# R302 DR-03a/DR-07 端点调试门控与标题语义修复实测记录

- 分支：`opt1000/oa4rust-pc-ai-refactor`（非 main）
- 测试入口：`https://<YOUR_TEST_DOMAIN>/oa4rust`（仅占位符，真值由本地环境注入）
- 截图来源：Playwright Chromium headless 对已部署测试服的真实页面流程截图，非物理真机；截图与原始 evidence 位于 gitignored 的 `oa4rust/test-results/r302-dr03a/`。
- 本轮范围：评审项 DR-03a（端点调试面板生产/测试构建可见）与 DR-07（管理区 h1/h2 语义层级、部署管理页 aiPageB5=0）。仅改 PC 组件/页面样式与模板语义，业务逻辑与接口不变。

## 问题与根因

1. DR-03a：`EndpointFold.vue` 的 trigger 在生产/测试构建中始终渲染，标签为“端点调试”，把内部端点调试入口暴露给最终用户；6 个业务/管理页还硬编码 `label="端点调试"`。
2. DR-07：`AdminLayout.vue:38` 提供全局 shell `<h1>{{ pageTitle }}</h1>`，本轮早前又在部署/CMS索引/流程实例/流程任务中心 4 页新增页内 `<h1>`，渲染后 h1Count=2，破坏标题语义层级；部署管理页未挂 `.ai-page-b5` 容器。

## 契约 RED → GREEN

- 新增 `oa4rust/frontend/apps/desktop/src/views/EndpointFold.devGate.B302.contract.test.ts`（4 用例）：`EndpointFold.vue` 仅在 `import.meta.env.DEV` 下以 `v-if` 渲染、`devToolsEnabled` prop 默认开、默认 label 为“高级操作”、6 个调用页不再出现“端点调试”标签；部署页 h1 唯一且有 ai-page-b5 与 h2；管理页页内不含 h1、保留关键 testid。
- RED：`pnpm exec vitest run src/views/EndpointFold.devGate.B302.contract.test.ts --reporter=dot` → 4 失败。
- 收紧标题层级断言后再次 RED：2 失败（ProcessTaskCenterApp 等仍为 h1）。
- GREEN：4 文件 18 用例全通过；`pnpm run typecheck` exit 0；`pnpm --filter @oa4rust/desktop build` exit 0。

## 修复要点

- `EndpointFold.vue`：`devToolsEnabled: import.meta.env.DEV`，根 `<section v-if="devToolsEnabled">`；默认 label 由“端点调试”改“高级操作”，hint 由“展开查看全部端点”改“展开查看全部操作”。
- 6 个调用页（BamApp / CmsIndexApp / NoteApp / OrgViewer / ProcessManagerApp / ProcessTaskCenterApp）label 统一为“高级操作”；AttendanceApp 副标题移除“端点调试请展开高级操作”，改“更多维护操作可展开高级操作”。
- 部署管理页：根容器加 `modern-page ai-panel ai-page-b5`，标题区改 glass-card/modern-hero，页内标题降为 h2（保留 shell 唯一 h1），次级标题 h2，去掉技术 mono 列样式，新增可测 testid。
- CmsIndexApp / ProcessManagerApp / ProcessTaskCenterApp：页内标题降为 h2，配套样式用 `--ai-gradient-text`，保留 key testid。

## 测试服部署与真实浏览器流程

- 构建产物 `index-CV1Nj9o4.js` / `index-BOp8VUI6.css`，经静态目录原子替换部署到测试服（gitignored 脚本 `test-results/deploy-pc-r302-dr03a-0354.sh`），远端 index.html 哈希校验一致，无服务重启。
- 干净复验 harness（gitignored `test-results/r302-dr03a-check-final.cjs`）：每页新建 browser context、无缓存 query 参数、运行时从本地环境读取登录凭据，Playwright Chromium headless 登录并逐页直开 6 页截图 1440x900：01-bam、02-note、03-cms-index、04-process-manager、05-deployment、06-attendance。
- 结果：6 页 h1Count 均为 1，debugText=0，rawEndpointVisible=0；部署页 modernPage=1、aiPanel=2、h1=部署管理、subtitle=维护应用部署记录与运行状态；考勤页副标题显示新文案；actions 3 项通过（生产调试面板隐藏、部署页 AI 标题、考勤副标题）；errors=[]。

## 诚实标注

- 首次 harness 曾因 `?_r302=` query 破坏 admin 鉴权重定向而卡在第 3 页，导致 `06-attendance.png` 为旧截图且 evidence 与截图不一致。已废弃该 harness，改用每页独立 context 重跑并全量重拍 6 张截图，evidence 由脚本原生生成，未手工改写。
- harness 的 `consoleErrors` 字段实现为累计页序号（0..5），并非错误计数，属字段命名误标；权威结论以 `errors=[]` 为准（零 pageerror）。此实现问题仅存在于 gitignored 本地 harness，未入库。
- 截图均为测试服渲染，不冒充物理真机；真域名、主机名、凭据与本地绝对路径不入库。
- BBS 发帖写入表与列表查询表不一致（既有缺陷）不属于本轮范围，留待后续轮次处理。项目为 ≥1000 轮持续迭代，本轮非完成态。
