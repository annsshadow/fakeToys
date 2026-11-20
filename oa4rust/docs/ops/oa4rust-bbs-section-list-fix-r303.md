# R303 BBS 版块筛选列表现役表修复

## 背景

PC 论坛按版块筛选调用 GET /api/bbs/subject/list/{sectionId}。t36 已把发帖接口改为写入现役表 x_bbs_topic，但该列表端点仍 SELECT 遗留表 bbs_subject_info，导致真实 UI 新建版块、发帖、刷新、按版块筛选时新帖不回显。

## 修改

- oa4rust/backend/crates/bbs/src/subject.rs: subject::list 改读 x_bbs_topic，筛选 section_id = $1 AND deleted_at IS NULL，对齐 u2 列表的字段映射；响应键 id/title/authorId/sectionId/replyCount/viewCount/isTop/createTime 与前端调用面保持不变。
- oa4rust/backend/crates/bbs/src/tests.rs: 新增回归契约，断言 create 写 x_bbs_topic、list 读 x_bbs_topic 且过滤 section_id 与软删除，防止两处再次分叉。
- oa4rust/frontend/apps/desktop/src/views/BBSForum.test.ts: 钉住版块筛选 GET 路径不变与 authorId 到 author 的归一逻辑。

## 验证

- RED: 新增后端契约在修复前失败，报 GET /api/bbs/subject/list/{sectionId} must read active table x_bbs_topic（cargo test -p bbs：8 passed; 1 failed）。
- GREEN: cargo test -p bbs -p bbs_assemble_control 全绿（bbs 9 passed、assemble_control 45 passed）；pnpm --filter @oa4rust/desktop exec vitest run src/views/BBSForum.test.ts --reporter=dot 10 passed。
- 测试服原子部署 Linux ELF：构建产物 file 检查为 ELF 后替换，替换前备份，替换后 systemd active。
- 真浏览器：Playwright Chromium headless 访问部署测试服，真实新建版块、发帖、刷新、按版块筛选，新帖标题回显，0 pageerror。截图与 evidence.json 落在 gitignored 的 oa4rust/test-results/r303-bbs-section-list/，来源标注为非物理真机。
- 测试服地址使用占位符 https://<YOUR_TEST_DOMAIN>/oa4rust；口令仅运行时从测试服本地凭据文件读取，不入库。

本项目持续迭代至至少 1000 轮，本轮不是终态。
