# R301 U-1 组织树/部门树加载兜底实测记录

## 背景与根因
- t18 收敛审计后 U-1 未收敛：/app/org 组织架构树常驻 Loading，且 group/list/like 偶发 HTTP 400；/app/directory 部门树缺少超时与错误兜底。
- 后端核对：group/list/like 的正确路由是 PUT（u2_router.rs:125），而 OrgViewer 首屏误用 GET → HTTP 400 / SPA fallback HTML；/api/departments/tree 是真实 GET 路由（u2_router.rs:75, u2_org.rs:1252），问题在前端直接 collect 吞掉异常且 runAll 无 finally。
- 本轮按 test-first RED→GREEN 修复，业务逻辑与接口语义不变（仅把错误方法改为后端既有 PUT）。

## 修改文件
- oa4rust/frontend/apps/desktop/src/views/OrgViewer.vue：首屏 loadOrgTree 改用 api.put(group/list/like, {}) 并在 withTimeout 内；保留中文错误态、重试按钮、空态与节点详情中文化。
- oa4rust/frontend/apps/desktop/src/views/OrgDirectoryApp.vue：新增 10s withTimeout 包裹 GET /api/departments/tree，deptTreeLoading/deptTreeError/deptTree 状态、finally 复位 busy、首屏自动加载、可见「加载部门树 / 重试加载」按钮与中文空态/错误条、玻璃卡片样式；角色/目录面板改用同一兜底加载。
- 契约：OrgViewer.loadingFallback.contract.test.ts 与 OrgDirectoryApp.r301DeptTree.contract.test.ts 覆盖超时、PUT 方法、错误态、重试、空态与 loading 复位。

## 验证
- RED：新 U-1 契约首轮 3 failed/1 passed（缺 loadDeptTree / 错误态 / PUT）。
- GREEN：org 相关 6 files / 26 tests 全通过；桌面 typecheck exit 0；build exit 0。
- 测试服原子部署：生产静态产物哈希经部署脚本 grep 校验命中。
- 真浏览器：Playwright Chromium headless 访问 <YOUR_TEST_DOMAIN>/oa4rust，登录后依次进入 /app/org 与 /app/directory，执行正常加载、请求 abort 注入失败、点击重试，共 6 张截图 + evidence.json。
- evidence source 标注：Playwright Chromium headless against deployed test server; not a physical device（非物理真机）。证据目录为 gitignored 的本地测试产物，不入库。

## 实测结论
- /app/org：加载结束无 Loading 常驻；失败注入后出现明确中文错误条与「重试加载」；无英文提示、无端点调试面板；PUT 修复后不再出现 400 参数问题（返回空数据时显示中文空态引导）。
- /app/directory：名册仍为 21 人（真实 UI 数据）；部门树首屏加载后无常驻 loading；失败注入后错误条+重试可见，点重试后恢复为空态且无错误。
- 项目为 ≥1000 轮持续迭代，本报告只记录本轮改动，不代表项目完成。
