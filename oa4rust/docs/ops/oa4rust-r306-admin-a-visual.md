# R306 管理页视觉深化 A：query / jpush / queryview / form / view / fileinfo

## 背景与口径
依据 R306-A 任务分派，本轮聚焦六个系统管理页：QueryManager、JPushApp、QueryViewApp、FormApp、ViewApp、FileInfoApp。改造前这六页源码 \`var(--ai-*)\` 数量不足且根 DOM 缺少 \`ai-page\` 标记（双未消费状态）。本轮仅改视觉层：复用 t1 \`--ai-*\` 设计令牌与 t4 外壳，不修改业务接口、路由、数据逻辑或其他任务写域文件。

## 改动
- 六页根容器补 \`modern-page ai-page\`；页头/主面板补 \`ai-panel ai-page\`；空态补 \`ai-empty\` + 可读引导文案。
- CSS 落地玻璃面板（\`--ai-glass-bg/border/blur/shadow\`）、品牌渐变强调（\`--ai-gradient-brand/surface\`）、8px 栅格（\`--ai-grid-1..6\`）、骨架屏令牌（\`--ai-skeleton-base/shine\` + \`ai-shimmer\`）、主按钮动效（\`--ai-motion-fast\` + \`--ai-ease-spring\`）。
- 契约 \`oa4rust/frontend/apps/desktop/src/views/AdminR306A.contract.test.ts\` 先 RED（4 failed）后 GREEN（4 passed），用 \`hasAiRule\` 逐选择器断言真实令牌规则 + \`aiVarCount >= 5\`，不是假断言。
- 真 bug 修复（test-first）：QueryViewApp.vue 模板引用 \`<AdminPageHeader>\`、\`<AdminFormSection>\`、\`<AdminToolbar>\` 但脚本从未 import，仅以 \`void\` 占位，导致真浏览器运行时 \`ReferenceError: AdminFormSection is not defined\`、整页白屏（HEAD 既有缺陷）。新增回归契约 \`QueryViewApp.imports.R306.contract.test.ts\`，RED（2 failed）后改为 barrel 显式导入并删除 \`void\` 占位，GREEN（2 passed）。

## 验证
- \`cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/AdminR306A.contract.test.ts\` exit 0；外加 import 回归契约共 2 files / 6 tests passed。
- \`cd oa4rust && pnpm run typecheck\` exit 0；\`cd oa4rust && pnpm --filter @oa4rust/desktop build\` exit 0（最终入口 \`index-BiyhjNmp.js\` / \`index-BMFkaD73.css\`）。
- 构建产物按权威步骤原子部署测试服：本地 tar → SSH 备份 web → web.bak-\$STAMP → scp /tmp → web.incoming-\$STAMP 原子替换 → grep 新哈希 → Playwright Chromium headless 真登录逐页截图。
- 真浏览器证据 \`oa4rust/test-results/r306-admin-a/evidence.json\`（gitignored，source 标注非物理真机）：六页 aiPage = 2/2/5/2/2/2（均 ≥2），glass = 4/4/5/13/3/3，errors=[]；六张 1440x900 PNG（01-query 至 06-fileinfo）。

## 配置与脱敏
测试服地址、账号、口令均从 \`.local.keys\` / 环境变量注入；仓库内仅使用 \`<YOUR_TEST_DOMAIN>\` 占位。本文 0 真值，证据文件与部署脚本均不入仓。

## 提交
精确 git add 八个 inScope 文件及必要的 QueryViewApp 导入回归契约，单提交推送；不改 \`docs/ops/current-state.md\`（队长 CAS 台账）。
