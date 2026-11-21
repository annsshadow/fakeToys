# R331 文档管理列表软删过滤修复

## 根因

`oa4rust/backend/crates/cms_assemble_control/src/lib.rs` 中 `document_filter_paging_manager_u3` 的过滤 SQL 只有 `ORDER BY create_time DESC LIMIT 200`，未带 `deleted_at IS NULL`，而相邻的 next/prev/paging 三个变体均带该条件。因此文档被软删（`deleted_at` 置为当前时间）后，仍会出现在管理端列表 `POST /api/document/filter/list/{page}/size/{size}/manager` 的返回里。

同一过滤函数 `list_from_table_filtered_inner` 会把 `ORDER BY` 之前的部分当作 WHERE 条件、之后的部分当作 tail 追加，所以把 `deleted_at IS NULL` 放在 `ORDER BY` 之前即可复用同一入口修复，无需改路由、SDK 或前端。

## 修复

- 唯一改动：`document_filter_paging_manager_u3` 的过滤参数从 `"ORDER BY create_time DESC LIMIT 200"` 改为 `"deleted_at IS NULL ORDER BY create_time DESC LIMIT 200"`。
- 未改其他三个变体、路由、软删逻辑或迁移文件。

## 测试（RED → GREEN）

在 `oa4rust/backend/crates/cms_assemble_control/src/tests.rs` 新增行为测试 `r331_manager_list_excludes_soft_deleted_documents`：

1. 建测试 admin（auth_role/auth_person/auth_person_role），建两篇文档；
2. 把其中一篇 `deleted_at` 置为当前时间模拟软删；
3. 直接调用 `document_filter_paging_manager_u3`，断言已删文档不在 ids、未删文档仍在 ids。

修复前：测试 FAILED（返回 2 条，含已删文档）；修复后：`cargo test -p cms_assemble_control --lib` 全绿（348 passed / 0 failed / 4 ignored）。

> 验证命令实际在 `oa4rust/backend` 目录执行（该目录才有 Cargo.toml）。

## 测试服实测

- 后端以 Linux ELF（bullseye 构建，GLIBC ≤ 2.34，与测试服兼容）原子替换并重启，健康检查通过；脚本先备份旧二进制，失败自动回滚。
- 实证补充：首次复用既有 cargo target 卷编译的 ELF 虽 `strings` 含新 SQL，但 manager 接口仍返回软删文档（DB 直查未删仅 2 条、API 返回 5 条）。原因是 cargo 复用了旧的 rlib 缓存。改用全新 target 卷全量重编译（约 12 分钟）后部署，接口行为与源码一致。后续同类后端修复部署务必使用干净 target 卷，避免旧 rlib 混入。
- 真浏览器（Playwright Chromium headless，1440×900）走完整 UI 流程：登录管理员 → 打开文档管理页 → 记录列表 → 新建带唯一标记的文档 → 列表确认可见 → 点击该行「删除」并确认 → 刷新 → 确认标记文档消失、其余文档仍在；页面错误 0、API 失败 0。
- 证据：gitignored `oa4rust/test-results/r331-document-manager-filter/`（5 张截图 + evidence.json），`source` 标注 `Playwright Chromium headless against deployed test server; not a physical device`（本地渲染，非物理真机）。
- 测试域名与主机使用占位符 `<YOUR_TEST_DOMAIN>` / `<YOUR_TEST_HOST>`；登录口令在运行时从环境/`.local.keys` 读取，未写入仓库或本文件。

## 结论

管理端列表不再返回软删文档；其余文档正常保留。本轮仅修过滤 SQL 一处，未触碰前端、SDK、路由或迁移。
