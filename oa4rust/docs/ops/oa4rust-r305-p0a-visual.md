# R305 P0 视觉深化：组织架构 / 考勤 / BAM / 论坛令牌深度消费

## 背景
审计 \`docs/ops/oa4rust-token-consumption-audit-r302.md\` 第 5 节 P0 标记四页「双重未消费」：
DOM aiPage=0 且源码 var(--ai-*) 不足。本轮仅改视觉层，不动业务接口、路由、数据逻辑。

## 改动
- \`oa4rust/frontend/apps/desktop/src/views/OrgViewer.vue\`：根/页头加 ai-page；玻璃头图、组织树/详情玻璃、选中节点品牌渐变、骨架令牌、8px 栅格；新增 org-page testid。
- \`oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue\`：根/页头加 ai-page；统计卡玻璃、主按钮品牌渐变、骨架令牌、8px 栅格；保留 attendance-page testid。
- \`oa4rust/frontend/apps/desktop/src/views/BamApp.vue\`：根/页头加 ai-page；裸「加载中...」替换为 data-testid=bam-loading 的 4 段 skeleton-row（令牌骨架）；内容面板玻璃；新增 bam-page testid。
- \`oa4rust/frontend/apps/desktop/src/views/BBSForum.vue\`：根/页头加 ai-page；sk-item/skeleton-row 用 --ai-skeleton-base/shine 渐变；新建按钮 gap 用 --ai-grid-1。
- \`oa4rust/frontend/apps/desktop/src/views/Pages.aiStyle.R305.contract.test.ts\`（新增）：4 条契约，先 RED（4 failed）后 GREEN（4 passed）。

## 验证
- RED：\`cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/Pages.aiStyle.R305.contract.test.ts --reporter=dot\` exit 1（4 failed）。
- GREEN：同命令 exit 0（4 passed）。
- \`cd oa4rust && pnpm run typecheck\` exit 0；\`cd oa4rust && pnpm run build\` exit 0（index-CAE6OlwX.js）。
- 构建产物原子部署测试服后，Playwright Chromium headless 真登录逐页截图 1440x900：\`/app/org\`、\`/app/attendance\`、\`/app/bam\`、\`/app/bbs\` 均 aiPage=2、h1=1、0 pageerror。证据 \`oa4rust/test-results/r305-p0a-visual/evidence.json\` 与 4 张 PNG（gitignored），source 明确非物理真机。

## 配置与脱敏
测试服地址、账号、口令一律从 \`.local.keys\` / 环境变量注入，仓库内仅用 \`<YOUR_TEST_DOMAIN>\` 占位；证据文件不入仓。

## 提交
精确 git add 六文件单提交推送；不改变 docs/ops/current-state.md（队长 CAS 台账）。
