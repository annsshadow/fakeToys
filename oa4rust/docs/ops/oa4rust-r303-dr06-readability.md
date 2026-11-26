# R302-DR06-R 工作台待办与 IM 会话名可读性修复记录

> 轮次：R303 修复轮（源自 t30 次轮设计评审 finding R302-DR06-R，medium）
> 项目：≥1000 轮持续迭代，本轮非完成态。

## 1. 问题与依据

设计评审复核坐实两处机器键直出：

1. 工作台「我的待办」主标题使用 item.title || item.processName || 新审批，测试服真实接口 /api/processplatform/assemble/surface/task/list/my/paging/1/size/20 回带的 title 是 start:work-… 原始流程实例键，主视觉直接暴露 UUID（修复前截图见 t30 评审证据）。
2. IM 会话名 displayConversationName 对机器键降级为「单聊 · jf05」随机尾号，聊天头 im-chat-name 直接渲染 selectedChat.name，摘要可能回带原始键。

## 2. 契约先行（RED → GREEN）

- RED：新建 src/views/Dashboard.todoTitle.contract.test.ts、src/views/IMChat.name.contract.test.ts，首次运行精确复现「旧模板直出原始键」与「随机尾号 jf05 当会话名」两处失败。
- 共享工具：抽取 src/utils/processTitle.ts，复用 t33 的 isRawProcessKey/shortId 思路（非复制：新增 displayTodoTitle/foldRawKeySubtext/isRawConversationName/displayConversationName/displayConversationPreview/conversationOrdinal）。
- GREEN：三份契约共 7/7 通过；R303Dr06.contract.test.ts 汇总保护原始键不进入主标题。

## 3. 修复内容

- Dashboard.vue：待办主标题改 displayTodoTitle(item.title, item.processName)，业务名优先；原始键回退为「工作实例 / 运行闭环 / 流程任务」可读名；原始键折叠为副文本「流程 · 尾号」（foldRawKeySubtext），保留 data-testid="dashboard-todo-list" 等稳定探针。统计卡（DR-02 的 approvalCount）未改动。
- IMChat.vue：会话列表与聊天头统一走共享 displayConversationName；机器键降级为「单聊/群聊 + 由 id 稳定派生的序号」，不再出现随机尾号；摘要走 displayConversationPreview，真实 lastMessage 原样显示，仅原始键回退引导文案。Conversation 接口补可选 type 字段。

## 4. 验证

- 契约：pnpm --filter @oa4rust/desktop exec vitest run src/views/Dashboard.todoTitle.contract.test.ts src/views/IMChat.name.contract.test.ts src/views/R303Dr06.contract.test.ts --reporter=dot → 3 files / 7 tests passed。
- 类型检查：pnpm --filter @oa4rust/desktop typecheck → exit 0；pnpm run typecheck（工作区根）→ 见 commandsRun。
- 构建：pnpm --filter @oa4rust/desktop build → exit 0；入口 index-CnmHc9Ov.js / index-BOp8VUI6.css。
- 测试服：按队长权威步骤原子部署（备份 web → web.incoming → 原子替换 web/web.old），远端 index.html grep 两哈希一致。
- 真浏览器：Playwright Chromium headless 1440×900 登录 <YOUR_TEST_DOMAIN>/oa4rust，逐页断言并 fullPage 截图：
  - 工作台：5 条待办主标题均为「运行闭环」，副文本为「流程 · 折叠尾号」，无原始 work-/start: UUID 在主标题；统计卡「待审批 = 5」与列表一致。
  - IM：11 个会话名全为「单聊 N」稳定序号；选中会话聊天头同样为可读名；摘要为真实 lastMessage 或引导文案。
  - rawTitleLeak = { dashboard:false, im:false }，pageErrors = []。
- 证据：test-results/r303-dr06/evidence.json、01-dashboard.png、02-im.png（gitignored，来源标注 Playwright Chromium headless against deployed test server; not a physical device，非物理真机、非静态假数据）。

## 5. 约束遵循

- 未改接口、未改业务逻辑；仅 UI 呈现层与共享纯函数。
- 未触碰 out-of-scope 文件（OrgDirectoryApp.vue、后端源码、H5）。
- 凭据只从本地 gitignored 脚本调用远端读取脚本获取，仓库与提交信息无真域名/账号/口令。
- 精确 git add 单任务单提交推送，工作分支 opt1000/oa4rust-pc-ai-refactor。
