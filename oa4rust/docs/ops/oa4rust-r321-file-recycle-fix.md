# R321 D1 文件删除回收站回显修复

> 证据来源：Playwright Chromium headless 对已部署测试服的真实页面流程截图，非物理真机。测试域名/主机在下文一律用 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>` 占位，真值仅在本地 gitignored 脚本运行期读取，不入库。

## 1. 缺陷来源

t105 R321 PC 深度功能审计（docs/ops/oa4rust-r321-pc-deep-audit.md）坐实真缺陷 D1：文件管理「可清理文件」区明确提示『删除后进入回收站』，但删除成功后右侧「回收」区始终为空，页面承诺与事实不符。队长源码级复核确认根因。

## 2. 根因（源码定位）

- 新建：frontend/apps/desktop/src/views/FileManager.vue:210-226，`POST /api/file/core/entity/file`，后端 writes FILE_FILE 并返回同族 id。
- 删除：FileManager.vue:888-907 `deleteMutation`（`DELETE /api/file/assemble/control/file/{id}`）命中后端 u2_file_delete_by_id，对 FILE_FILE 执行 `SET deleted_at=NOW()` 软删，归属/id 校验已有。
- 回收站读取：FileManager.vue:878-884 `loadRecycleBin()` 调 `GET /api/recycle/list`，后端读 `FILE_FILE WHERE deleted_at IS NOT NULL AND person=会话人`。
- 根因：删除成功的两个 onSuccess（外侧 :890-894、内侧 :900-902）只做 `invalidateQueries(['file',currentFolder])`、`loadFiles()`、`recyclableFiles` 本地移除与 toast，**从未调用 `loadRecycleBin()`**；软删虽已落库，页面回收站不刷新，仍显示「回收站为空」。

## 3. 最小修复（只改前端）

在 `deleteFile` 传给 `deleteMutation.mutate` 的 onSuccess 内，本地移除可回收项后调用 `loadRecycleBin()`，再 toast 成功。复用既有真实读端点 `GET /api/recycle/list`，按真实软删结果刷新回收站；未改后端、软删端点、归属校验、新建逻辑与表族。

说明：onSuccess 是普通同步回调，故调用 `loadRecycleBin()` 而不在其前加 `await`（保留既有无等待风格，内部自带 catch 与错误 toast）。

## 4. test-first RED → GREEN

契约：frontend/apps/desktop/src/views/FileManager.recycleRefresh.contract.test.ts（源码级真断言）
- it1 `loadRecycleBin` 必须使用 `api.get('/api/recycle/list')` 并写 `recycleFiles.value`。
- it2 `deleteFile` 的 mutate onSuccess 必须调用 `loadRecycleBin()`。
- it3 该调用须为直接调用（非注释/字符串）且位于成功 toast 之前。
- it4 模板回收列表绑定 `recycleFiles` 且保留刷新入口。

RED（修复前）：2 failed / 2 passed，失败点精确为 it2/it3 找不到 `loadRecycleBin()`。
GREEN（修复后）：4 passed / 4。
回归：FileManager 既有 4 份契约 17 tests 全通过；`pnpm run typecheck` exit=0（Scope 6/7）。

## 5. 构建与部署

- `pnpm --filter @oa4rust/desktop build` exit=0，入口 `index-zrKXNNVT.js` / `index-BMFkaD73.css`。
- 测试服原子部署（gitignored 脚本 test-results/deploy-pc-r321-file-recycle.sh，唯一 STAMP）：备份 web → scp 临时包 → web.incoming 解包 → 原子 mv 替换 → 远端 grep 新哈希校验通过（DEPLOY_R321_FILE_RECYCLE_HASH_OK）。

## 6. 真浏览器复验（非物理真机）

脚本 test-results/r321-file-recycle-check.cjs（gitignored）：运行期经 ssh 读取凭据，Playwright Chromium headless 1440×900，登录 `<YOUR_TEST_DOMAIN>/oa4rust` → `/app/file`：
1. 点「新建可清理文件」→ 真创建同名文件；
2. 点删除 → 自定义确认弹窗确认 → toast「文件已删除」；
3. 右侧「回收」区由空变为出现同名条目（页面自动刷新，无需手点刷新）；
4. 调真实 `POST /api/recycle/resume/{id}` 恢复 → 刷新回收站 → 同名条目消失。

结果（evidence.json）：7 项断言全 true —— createdName / recycleHasSameName / recycleListCalledAfterDelete / resumeOk / resumeGoneFromRecycle / noPageErrors / noApiFailures；pageErrors=[]，apiFailures=[]，删除后 `/api/recycle/list` 被调用（recycleCalls=3）。

## 7. 证据索引（gitignored）

- test-results/r321-file-recycle-fix/01-before.png：删除前。
- test-results/r321-file-recycle-fix/02-after-delete.png：1440×900 / 137084B / sha256:ecd14908e73b8cb21a2e004bd1da2d00ae08148093e2ea6511581591c20e8e9c，回收区出现同名条目 + 删除成功 toast。
- test-results/r321-file-recycle-fix/03-after-resume.png：1440×900 / 132849B / sha256:c442a32ac4400abeb6131d32de266b431406bcf029da46d4203b740eed1616f1，同名条目已从回收站消失。
- test-results/r321-file-recycle-fix/evidence.json：source 标注 `Playwright Chromium headless against deployed test server; not a physical device`。