# R300 B4 业务工作域页面实测记录

- 轮次：R300 B4（业务工作域页面重构）
- 分支：`opt1000/oa4rust-pc-ai-refactor`（非 main）
- 来源声明：Playwright Chromium headless 访问测试服 `https://<YOUR_TEST_DOMAIN>/oa4rust`，通过登录页真实 UI 登录后逐页导航/刷新；截图为浏览器渲染产物，**非物理真机**。
- 证据目录：`oa4rust/test-results/r300-b4-business/`（运行产物，gitignored，不入库）。

## 改动范围

- `ProcessWork.vue` / `FileManager.vue` / `BBSForum.vue` / `MeetingApp.vue` / `BamApp.vue` / `NoteApp.vue` / `RecycleApp.vue` / `ProcessTaskCenterApp.vue`：按 t1 AI 令牌与 t4 外壳统一为 `modern-page + ai-panel + modern-hero + ai-gradient-text + glass-card` 体系；业务接口与逻辑不变；原始 UUID/编号改为业务语义展示。
- `BamApp.vue`：修正 queryFn 未返回数据导致 TanStack Query 报 `data is undefined` 的真实缺陷（契约先 RED 后 GREEN）。

## 真浏览器流程与结论

- 8 个业务页面全部通过侧栏/直达路由访问：工作流、文件、论坛、会议、BAM、笔记、回收站、流程任务中心；每页 `modernPage=1`、AI 面板 `aiPanel>=1`、页面标题正确，`pageerror=[]`。
- 论坛：通过真实 UI 新建版块 → 再次真实 UI 发帖 → 刷新列表；`/api/bbs/subject/create` 成功但列表查询口径不回显新帖，属既有查询口径差异，不阻塞本轮样式与接口保持验证。
- BAM：修复后统计卡真实呈现 `90 / 0 / 22 / 10`，`BAM 统计加载失败` 出现次数为 0。

## 验证

- `pnpm exec vitest run src/views/BusinessWork.aiStyle.B4.contract.test.ts`：5 passed。
- `pnpm exec vitest run src/views/BamApp.summaryData.B4.contract.test.ts`：RED 1 failed → GREEN 1 passed。
- `pnpm run typecheck`：exit 0；`pnpm --filter @oa4rust/desktop build`：exit 0。
- 部署后远端入口 hash 与本地构建一致，浏览器实测截图与 evidence.json 落在上述 gitignored 目录。
