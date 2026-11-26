# R302 论坛发帖回显缺陷修复实测记录

- 轮次：R302（PC 论坛发帖不回显修复）
- 分支：`opt1000/oa4rust-pc-ai-refactor`（非 main）
- 来源声明：Playwright Chromium headless 访问测试服 `https://<YOUR_TEST_DOMAIN>/oa4rust`，通过登录页真实 UI 登录，在论坛页真实新建版块→发帖→刷新并观察列表回显；截图为浏览器渲染产物，**非物理真机**。
- 证据目录：`oa4rust/test-results/r302-bbs-create/`（运行产物，gitignored，不入库）。

## 缺陷与根因

- 现象：R300 B4 真浏览器流程中，论坛新建版块与发帖接口成功，但刷新后新主题不出现在列表。
- 根因：`oa4rust/backend/crates/bbs/src/subject.rs` 的 `POST /api/bbs/subject/create` 写入遗留表 `bbs_subject_info`，而 PC 默认/我的/版块筛选列表查询现役表 `x_bbs_topic`。写入表与读取表不一致导致新帖不可见。
- 修复：create handler 改为写 `x_bbs_topic`；字段映射对齐现有 admin create handler：`id/title/content/author_id/creator/forum_id/section_id/section_name/subject_type/create_time=NOW()`。
- 安全保持：`require_session` 与 `reject_impersonation` 门禁保留；`author_id`、`creator` 均以会话身份为权威，请求体仅提供展示元数据，接口路径与前端调用面不变。

## 验证

- 后端回归契约：`cargo test -p bbs test_subject_create_writes_active_bbs_topic_table` RED 1 failed（明确提示必须写 `x_bbs_topic`）；修复后 `cargo test -p bbs -p bbs_assemble_control` GREEN，45 passed / 0 failed。
- 前端契约：`pnpm exec vitest run src/views/BBSForum.test.ts --reporter=dot` 9 passed / 0 failed；固定前端仍调用注册端点 `/api/bbs/subject/create`，作者身份字段不变。
- 测试服：后端 release 构建与二进制原子替换后重启服务，健康检查与页面登录成功。
- 真浏览器：新建版块可见、发帖刷新后新主题在列表中回显、`pageerror=[]`；逐步截图与 `evidence.json` 落在 gitignored 证据目录。

## 范围说明

- 本轮只修正发帖写入表与对应测试，不变更前端业务流程、不新增接口、不变更权限模型。
- 项目为 ≥1000 轮持续迭代，本轮不是终态。
