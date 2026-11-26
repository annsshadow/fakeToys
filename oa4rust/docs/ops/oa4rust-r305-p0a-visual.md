# R305 P0 视觉深化：组织架构 / 考勤 / BAM / 论坛令牌深度消费

## 背景与口径校正（依据 t50 实测基线）
\`docs/ops/oa4rust-r305-visual-baseline.md\`（t50 真浏览器基线）纠正了早期 t35 审计口径：
- **口径差页** \`/app/org\`(srcVar=4)、\`/app/bam\`(13)、\`/app/bbs\`(9)：源码已注入 \`var(--ai-*)\`，但 DOM aiPage=0（根元素未打 ai-page class）。改造重点是补 ai-page 标记 + 已有令牌下沉到面板/卡片/骨架，**不重复注入令牌**。
- **双未消费页** \`/app/attendance\`（srcVar=0 且 DOM aiPage=0）：才需全套落地 ai-panel 玻璃 + --ai-glass-* + 8px 栅格 + 骨架屏 + 空态引导。

本轮仅改视觉层，不动业务接口、路由、数据逻辑；保留 org loading/端点调试既有修复。

## 改动
- \`oa4rust/frontend/apps/desktop/src/views/OrgViewer.vue\`：根/页头补 ai-page；已有令牌下沉 .ai-page 组织树/详情玻璃、选中节点品牌渐变、骨架令牌、8px 栅格；新增 org-page testid。
- \`oa4rust/frontend/apps/desktop/src/views/AttendanceApp.vue\`（双未消费全套）：根/页头 ai-panel ai-page；统计卡玻璃、主按钮品牌渐变、骨架令牌、8px 栅格；空态「暂无考勤记录，可刷新后查看最新数据」+ 刷新主按钮（复用既有 loadData，不新增业务逻辑）。
- \`oa4rust/frontend/apps/desktop/src/views/BamApp.vue\`：根/页头补 ai-page；裸「加载中...」替换为 data-testid=bam-loading 的 4 段 skeleton-row（令牌骨架）；内容面板玻璃；新增 bam-page testid。
- \`oa4rust/frontend/apps/desktop/src/views/BBSForum.vue\`：根/页头补 ai-page；已有令牌下沉 .ai-page 头图玻璃/主题卡渐变、骨架令牌、新建按钮 8px 栅格。
- \`oa4rust/frontend/apps/desktop/src/views/Pages.aiStyle.R305.contract.test.ts\`（4 条契约）：按两类语义分别断言——共同验收 DOM aiPage≥2/srcVar≥5；口径差页验 ai-page 标记+.ai-page 样式下沉；双未消费页验全套玻璃/栅格/骨架/空态引导；RED（4 failed）→GREEN（4 passed）。

## 验证
- RED：\`cd oa4rust && pnpm exec vitest run frontend/apps/desktop/src/views/Pages.aiStyle.R305.contract.test.ts --reporter=dot\` exit 1（4 failed）。
- GREEN（校正后）：同命令 exit 0（4 passed）。
- \`cd oa4rust && pnpm run typecheck\` exit 0；\`cd oa4rust && pnpm run build\` exit 0（index-CunnpRgh.js / OrgViewer-DuVmEU6c.js / AttendanceApp-Lqpr5sbF.js）。
- 构建产物原子部署测试服后，Playwright Chromium headless 真登录逐页截图 1440x900：\`/app/org\`、\`/app/attendance\`、\`/app/bam\`、\`/app/bbs\` 均 aiPage=2、h1=1、0 pageerror；玻璃节点 org=3/attendance=11/bam=4/bbs=4。证据 \`oa4rust/test-results/r305-p0a-catfix/evidence.json\` + 4 PNG（gitignored），source 明确非物理真机。

## 配置与脱敏
测试服地址、账号、口令一律从 \`.local.keys\` / 环境变量注入，仓库内仅用 \`<YOUR_TEST_DOMAIN>\` 占位；证据文件不入仓。本文 0 真值。

## 提交
首轮提交 48f451d02；队长口径校正后增量精确提交（AttendanceApp.vue/BBSForum.vue/contract/doc）；不改 docs/ops/current-state.md（队长 CAS 台账）。
