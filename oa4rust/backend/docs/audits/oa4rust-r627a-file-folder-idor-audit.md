# R627a file folder_update/folder_remove 身份盲写 IDOR 甄别审计

## 一、结论
判级 P1（据实，不夸大 P0）。file 模块 folder_update（改名）与 folder_remove（软删）两个写端点仅校验会话是否存在、丢弃会话身份，且 UPDATE 语句不含 person 归属谓词：任意已登录用户凭他人文件夹 id 即可改名、软删他人文件夹，落库真改。非匿名、非批量遍历、软删可恢复，故不判 P0。

## 二、锚点根因
- crates/file/src/lib.rs:20-25 crate 内 require_session(Option<Extension<Session>>)->Session 仅在 None 时返回 Unauthorized；返回值被丢弃，只验存在不验归属。
- folder_update（约 650-695）：require_session(session)? 后直接 UPDATE FILE_FOLDER SET name=$1,update_time=NOW() WHERE id=$2 AND deleted_at IS NULL，无 person 过滤；result==0 才报 folder not found。
- folder_remove（约 711-747）：同样丢弃会话，UPDATE FILE_FOLDER SET deleted_at=NOW() WHERE id=$1 AND deleted_at IS NULL，软删无归属谓词。
- 底层 DDL 008_file_tables.sql:2-9 的 FILE_FOLDER 已有 person VARCHAR NOT NULL 归属列，两条 UPDATE 均未使用。
- 路由 file/src/routes.rs:22/23 与 backend/src/lib.rs:132 真实挂载，外部可达。

## 三、真执行证据矩阵
oneshot 真调 handler（临时独占库，运行后即清理；占位连接串不入仓）：
| 臂 | 结果 | 说明 |
| --- | --- | --- |
| alice 凭 bob 文件夹 id 改名 | 200 | DB name 真改为测试串 |
| admin 跨用户改名 | 200 | admin 预期放行 |
| alice 凭 bob 文件夹 id 软删 | 200 | DB deleted_at 真置非空 |
| 匿名裸调 update/remove | 401 | require_session 拒绝 |
| 兄弟 file_download 冒名读 bob 文件 | 403 | 归属门对照成立 |
| admin 读 bob 文件 | 200 | 归属门 admin 放行 |
定点：cargo test -p file --test r627a_file_folder_idor_audit → 2 passed / 0 failed。
包级：cargo test -p file --all-targets → lib 10 passed / 8 ignored + nullable 1 + 本契约 2，0 failed。
格式：cargo fmt -p file -- --check EXIT 0；clippy -p file --all-targets --no-deps -- -D warnings EXIT 0。

## 四、三分据实
- 可达性：需有效会话且知 folder id，非匿名、非遍历枚举；全局 auth 仅验登录。
- 危害：任意登录者可篡改/软删他人文件夹，破坏数据完整性与可用性；软删可恢复，危害有限。
- 限制：前端消费点全仓检索零命中（命中均后端），未定位 UI 调用链；按 R611a 先例前端未定位不影响后端可达落库。
- 旧审计 2026-10-14-security-audit-round4.md:167/:1033 曾列文件夹族/会话档冒名残留缺口，非完全无人提及，据实披露。
- 判级：P1，不夸大 P0。

## 五、兄弟对照与修复方向
- 兄弟门 file_download（lib.rs 约 515-528）读 FILE_FILE owner 后调用 shared::middleware::require_owner(&pool,&session,&owner)，本人/admin 放行否则 403；folder_update/folder_remove 缺失同口径门。
- 修复方向：两端点保留 require_session 返回值并取会话 person_unique，UPDATE 追加 AND person=$N（admin 走 is_admin 分支放行）；或复用 require_owner 对查到的 folder owner 做归属校验；GET 路由/Cargo/migrations/前端零改。

## 六、SHA256（文件 sha 与 runnerSha 分列）
- crates/file/src/lib.rs（审计对象，未修改）：01f19678f0193056c6b05fedcbe74943defbad250797b90887dea46da68e91da
- 临时 oneshot 契约（用后即删）：4b1251cdbfaaf5b4b013b95085bfea34a8dfae10b718a33edfa9d91c6786b18d
- runnerSha（甄别元数据，非文件哈希）：ab2c15e63c71095b08c1a33e8a7f4c47bf1e20b739bc494a72012f0c958df339
- 文档自身 sha 于交付报告经 ops_stat 采集，此处不自引用。
