# R455 PC 端 4 项 AI 视觉渲染可达真缺陷修复

- 任务：t311（page-core，实现），attempt_id 493d8c71-a445-454a-902d-b658e48cc8be
- 真父/RED 基线：分支 opt1000/oa4rust-pc-ai-refactor，HEAD=UP=86c112bcde253f908efb14bd0dcd64d804c4573f（AB=0_0，工作树干净）
- 依据：t310 只读甄别（文档 oa4rust/docs/ops/oa4rust-r454-pc-ai-visual-render-audit.md，commit a69854c7）坐实的 4 项可达真缺陷
- 测试入口：https://<YOUR_TEST_DOMAIN>/oa4rust/app/...（<TEST_HOST> 真值仅存在于 gitignored 运行脚本，仓库内 0 真值）
- 证据目录：oa4rust/test-results/r455-pc-ai-render-fix/（report.json + 5 PNG，均 gitignored）

## 1 修复范围（业务零改，仅视觉 CSS）

1. P2 渐变文字退化：BBSForum.vue 与 BamApp.vue 的 .ai-gradient-text 提权为 .ai-gradient-text.ai-gradient-text（特异性 0,2,0），使 color:transparent 盖过业务 h1 规则（0,1,1），保留 background:var(--ai-gradient-text) + background-clip:text；h1 文案/字号/字体零改。
2. P3 骨架流光失效：FindDesignerApp.vue 的 .ai-skeleton-line background 由纯色改为三色 linear-gradient（base 25% / shine 50% / base 75%），保留 background-size:200% 100% 与 animation:ai-shimmer。
3. P3 双重 blur 非法：CalendarApp.vue 与 MeetingApp.vue 共 4 处 backdrop-filter:blur(var(--ai-glass-blur)) 改为 backdrop-filter:var(--ai-glass-blur)，避免展开为 blur(blur(12px)) 非法值。
4. P3 BBS header 渐变被覆盖：BBSForum.vue 在高特异性 glass 规则之后新增 .ai-page .bbs-header.bbs-header{background:var(--ai-gradient-surface)}（0,3,0），盖过 ai-panel 与 .ai-page .bbs-header 的 glass-bg。

零改边界：AttendanceApp 骨架 loading 永 false 属不可达 L1，本轮不动；全库另 4 处同型双重 blur（OrgDirectoryApp/ProcessWork）不在写域不触碰；业务结构/接口/data-testid/后端/SDK/main.ts/adminNav.ts/H5/其它 view 一律零改。

## 2 RED → GREEN 与三 verify

- 新契约：oa4rust/frontend/apps/desktop/src/views/R455.aiRenderFix.contract.test.ts（8 it：6 修复断言 + 2 h1 文案字号零改守卫），真执行读源码正则断言，非桩。
- 真父 86c112bcd 定点：RED 6 failed / 2 passed（8），失败全为真实修复断言，0 桩错误；2 零改守卫父版 pass。
- 修复版定点：GREEN 8/8 全过。
- V1 定点契约：pnpm vitest run frontend/apps/desktop/src/views/R455.aiRenderFix.contract.test.ts → exit 0，8/8。
- V2 根全量：pnpm test → exit 0，316 files / 3121 tests，0 failed。
- V3 类型检查：pnpm typecheck → exit 0。
- 构建：oa4rust 根 pnpm run build → exit 0，入口 index-Zx3NjJ00.js；整目录原子部署测试服 DEPLOY_R455_PC_FULL_OK。

## 3 真浏览器实证（非物理真机）

- 来源：Playwright Chromium headless，viewport 1440x900、DPR1，测试服真实页面（NOT a physical device，非物理真机）；凭据 ssh 运行时提取注入，不入库。
- 终轮：R455_DONE total=23 passed=23 failed=[] pageErrors=[]。
- BBS 渐变标题 团队论坛：color=rgba(0,0,0,0)、backgroundImage=linear-gradient(120deg,rgb(67,56,202),rgb(14,116,144))、backgroundClip=text；BBS header backgroundImage=linear-gradient(160deg,...)、backdropFilter=blur(12px)。
- Bam 渐变标题 业务活动监控：color=rgba(0,0,0,0)、linear-gradient、clip=text。
- Calendar 两处（header/scope tabs）与 Meeting 两处（hero/booking）backdropFilter 均 blur(12px)（父版为 none）。
- FindDesigner 骨架（真实业务 API 延迟 4000ms 触发 loading）：backgroundImage=linear-gradient(90deg,base 25%,shine 50%,base 75%)、animation=ai-shimmer。
- 6 页原正常渐变文字零回归：dashboard 早安 admin（H1）、calendar 工作日历（H1）、file（.ai-gradient-text）、meeting 会议管理（H1）、cms-index CMS 索引设计（H2）、deployment 部署管理（H1）均 color=rgba(0,0,0,0) + linear-gradient + clip=text。

### 截图 sha256（命令行 sha256sum 原始值）

- bbs.png 383c5b7ba47e0a3732521819b590a08bdc9585afa3e0dd3b805d6bf3a1704876
- bam.png db9f61789cd8952e790619961dd01111f755f2523f3fcaa3c46cdbd49f3074b7
- calendar.png 4341921be8c0f2e21c47946179de9a03d9f4b480335104100f4724de4edf6192
- meeting.png 41e50eb557ff8c1edee28386ccd7cdcf37b255b6bfb0386d3840bff3fe896b31
- find-designer.png 5fcb3b198d619184482cbad66882ae751c86c8e53f02b0afca7aea691f450a29

目视抽样（1440x900，read_image）：bbs 团队论坛渐变标题清晰、header 渐变底；calendar 工作日历与日期网格完整；find-designer 骨架流光与配置列表完整。

## 4 真值纪律与边界

- 文档内测试域用 <YOUR_TEST_DOMAIN>、<TEST_HOST> 占位，0 真值；无密钥、口令、IP、个人路径。
- 证据目录 gitignored，真域名仅存于 gitignored 运行脚本，不入库、不在文档回显。
- 本轮为 ≥1000 轮迭代中的一环（R455），非终态；后续按评审结论续作。
