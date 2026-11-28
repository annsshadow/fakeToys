# R628a file folder_update/folder_remove 身份盲写 IDOR 真修复审计

## 一、结论
- 判级 P1（据实，不夸大 P0）：任意登录用户凭他人文件夹 id 即可改名/软删，属跨用户写越权；需有效会话且已知 folder id，非匿名、非批量遍历，故不判 P0。
- 缺陷根因：两端点 `require_session(session)?` 只验会话存在即丢弃返回值，写 SQL 无归属谓词，任何登录者都能改/删任意 FILE_FOLDER。
- 修复：保留会话并先按 id 查归属人，经 `require_owner`（本人或 admin，trim/lowercase 归一化）校验后才执行原 UPDATE；查不到维持 folder not found，不泄露存在性。

## 二、锚点与根因
- `crates/file/src/lib.rs` folder_update(:650)：原 :656 丢弃会话；原 :675 `UPDATE FILE_FOLDER SET name=$1,update_time=NOW() WHERE id=$2 AND deleted_at IS NULL` 无 person 谓词。
- `crates/file/src/lib.rs` folder_remove(:711)：原 :717 丢弃会话；原 :728 `UPDATE ... SET deleted_at=NOW() WHERE id=$1 AND deleted_at IS NULL` 无 person 谓词。
- `FILE_FOLDER` DDL(`migrations/008_file_tables.sql`:2-10)含 `person VARCHAR(255) NOT NULL` 归属列，具备归属校验基础。
- 兄弟范本：`file_download`(:527-528) 与 `file_assemble_control::get_file` 已用 `require_owner`；`shared/src/middleware/rbac.rs`:582-590 定义本人/admin 放行。修复与之同口径。
- 路由 `crates/file/src/routes.rs`:22/23 + `backend/src/lib.rs` merge 真实可达；本次路由/Cargo/migrations/前端零改。

## 三、RED→GREEN 真执行证据矩阵
- 契约：`crates/file/tests/r628a_file_folder_owner_fix_contract.rs`，oneshot 直调 file::router，两用例覆盖 update/remove。
- RED（真父 ad4a17f1）：`running 2 tests / 0 passed; 2 failed`——两处均在「alice 跨人改名/软删应 403」断言处失败（实得 200），坐实身份盲写缺陷。
- GREEN（修复后）：`running 2 tests / 2 passed; 0 failed`——alice 跨人操作 403 且 DB 未变、bob 本人 200 且落库、admin 跨人 200、匿名裸调 401。
- 临时实例用后即清；连接参数由环境变量注入，未入库。

## 四、行为变化
- 非 admin：从「可改/删任意文件夹」变为「仅本人文件夹」；跨人请求由 200 变 403，受害者数据保持不变。
- admin：保持可跨人操作（放行）。匿名：仍为 401（会话门保留在首个 DB 访问前）。
- 不存在的 folder id：维持原 folder not found 语义，不因归属校验泄露存在性。

## 五、验证（三 verify 全绿）
- V1 定点：`cargo test -p file --test r628a_file_folder_owner_fix_contract -- --nocapture` → 2 passed / 0 failed。
- V2 包级：`cargo test -p file --all-targets` → lib 10 passed/8 ignored（需运行实例的既有忽略项）、nullable 1 passed、本契约 2 passed，0 failed。
- V3：`cargo fmt -p file -- --check` EXIT0；`cargo clippy -p file --all-targets --no-deps -- -D warnings` EXIT0。
- 修复前曾出现 clippy `bool_assert_comparison`，来自本契约两处 `assert_eq!(bool,true/false)`，已在写域内改为 `assert!` 后复跑全绿。

## 六、SHA 分列（runnerSha 与文件 sha256 分列）
- runnerSha（甄别元数据 JSON sha256）：f6989211247f0da4bccbc34d31038d4fa5f9c0a2ab297629afd868d6270da0e9
- 审计对象 `crates/file/src/lib.rs` sha256：78dacf9db1cf49744cd1d0d6def7b60de15cd3d29fdff584dd6620f93e60863c
- 契约 `crates/file/tests/r628a_file_folder_owner_fix_contract.rs` sha256：171824adb5249c5c1ec777d53658d12f1a2d00750609898e32f9d8cd2b0d0f04
- 文档自身 sha256 于交付报告经 ops_stat 采集，不入本文本避免自引用。

## 七、Fail Loud / 限制
- 无跳过项、无已知失败项。V2 的 8 个 ignored 为仓库既有「需运行实例」单测，非本轮引入，据实披露。
- 判级依据据实，未夸大为可立即遍历全体文件夹（需已知 folder id）。