# R318 BBS 版块跨端数据源桥接修复

## §1 背景与根因

- t96 真浏览器甄别判定 B（真缺陷，P2）：PC 端 `section/create` 写入 `x_bbs_assemble_control_section`，而 H5 发帖 picker 数据源 `forum/view/all` 读 `x_bbs_forum`，两侧无同步或映射；现存 `section_viewforum_forum_id` 端点读第三张表 `x_bbs_section`，不构成有效桥接。结果：PC 新建版块对 H5 发帖完全不可见（`pcNameInH5=false`）。
- 根因属写读链不同源（同 r317 M1/M2 家族），非产品语义差异：主题表 `x_bbs_topic` 以 `forum_id` 为主归档，桥接必须保持 forum id 与 section id 一致。
- 修复方向（队长选定方向二）：在 `section_create` 成功后同步写入 `x_bbs_forum`，forum id 复用 section id；不改 PC `section/list` 读取链、不动伪桥接第三表、不迁移历史数据、不削弱 admin 门或 creator=会话人语义。

## §2 修复实现（源码定位）

- `oa4rust/backend/crates/bbs_assemble_control/src/lib.rs:263-332` `section_create`：
  - 保留 admin 门（:270-275 无会话 401/非 admin 403）与 creator 缺省会话人（:280-290）原语义；
  - :295-304 在 `crud_create` 前提取 `bridged_name`/`bridged_creator`（避免 obj moved）；
  - :305 `shared::crud_create(&pool, &section_create_spec(), ...)` 写 section 并取得 `id`；
  - :306-317 async 块 `shared::db::acquire(&pool).client.execute("INSERT INTO x_bbs_forum (id, name, description, creator, create_time) VALUES ($1, $2, $3, $4, NOW())", &[&id, &bridged_name, &String::new(), &bridged_creator])` —— forum id 与 section id 同源，无重复 forum；
  - :318-325 若桥接失败：补偿硬删 section（`DELETE FROM x_bbs_assemble_control_section WHERE id=$1`）并 `return bridge_result` fail loud，杜绝“section 建成但 forum 缺失”半状态；
  - :326-331 返回 `{id, created:true}`，响应结构未变。
- 未改：`list_control_sections`（:209/:216 读 x_bbs_assemble_control_section）、`forum_view_all`（:566/:570 读 x_bbs_forum）、`section_viewforum_forum_id`（:834/:842 读 x_bbs_section）；历史遗留版块不做迁移（已知边界：迁移前旧 PC 版块仍不在 H5 picker 中，需经既有 user/forum 端点补入）。

## §3 Test-first RED → GREEN

- RED 契约测试（新增于 `oa4rust/backend/crates/bbs_assemble_control/src/tests_generated.rs`，真实 `test_pool()` live-gated）：
  - `test_section_create_bridges_to_forum`（:270-322）：幂等建 admin 探针（auth_person/auth_role admin/auth_person_role），带会话 POST `section/create {name: marker}` → 200 取 `data.id`，GET `forum/view/all` 在 `data` 数组按同 id 找同名 forum，断言 `found.is_some()` 且 name==marker，末尾清理两表。
  - RED 命令：`cd oa4rust/backend && cargo test -p bbs_assemble_control --lib test_section_create_bridges_to_forum -- --nocapture` → FAILED at tests_generated.rs:317「RED: section/create must bridge to x_bbs_forum with SAME id; id=... not in forum/view/all (count=0)」。
  - 首两次曾因 cargo 增量未识别新增测试出现 running 0 tests；`touch tests_generated.rs lib.rs` 后正确执行 RED。
- GREEN 实现后同命令 1 passed；追加静态防回归 `test_section_forum_bridge_invariants`（:326-336）断言函数体含 `INSERT INTO x_bbs_forum`、`&[&id, &bridged_name`（id 复用无重复）、`DELETE FROM x_bbs_assemble_control_section WHERE id=$1`（补偿）、`bridge_result.is_err()`（fail loud）。
- 全量：`cd oa4rust/backend && cargo test -p bbs_assemble_control --lib` → 47 passed / 0 failed / 0 ignored（基线 45 + 新增 2），既有 section/forum 用例不回退。

## §4 测试服部署与真浏览器复验

- 部署：测试服地址仅经环境/SSH 别名注入，文档不写真值；`rust:1.98-bullseye + cargo +1.98.0 --offline --target-dir /workspace/target/linux` 产出兼容 Linux ELF（禁 Windows PE32+）；SSH 备份现有二进制后原子替换，健康检查失败自动回滚。
- 复验脚本（gitignored）：`oa4rust/test-results/r318-bbs-bridge-verify.cjs` 及同名 -run.sh，真登录 PC 与 H5 双端（凭据运行期读取不回显），PC 真 UI 建版块 → H5 `forum/view/all` 可见且 id 一致 → H5 发帖 → PC `topic/list/forum/{id}` 可见，断言 `pcNameInH5=false→true`。
- 证据（gitignored）：`oa4rust/test-results/r318-bbs-bridge-fix/evidence.json` + PNG，source 标注本地无头 Chromium 渲染、非物理真机。
- 实测结果（evidence.json）：`verdict=BRIDGE_VERIFIED`，`pcSection.found=true`（PC 新建版块出现在 section/list），`pcNameInH5=true`（H5 `forum/view/all` 已可见，t96 基线为 false），桥接后同名 forum 的 `forumId` 与 PC 新建 section id 一致（`896eaf44-35b9-47d4-85dc-8dc7805eb1b8`），pickerText 含新名；H5 真发帖 `topic.status=200`，PC `topic/list/forum/{id}.found=true`，`apiFailures=0`、`errors=0`。
- 证据：`oa4rust/test-results/r318-bbs-bridge-fix/evidence.json` + 4 PNG（01-PC-forum-before.png / 02-PC-section-created.png / 03-H5-picker.png / 04-PC-topic-visible.png，本地无头 Chromium 渲染，非物理真机）。首轮复验曾因脚本把对象当 URL 拼接导致 `r.json()` 收到 HTML 报错，已在 gitignored harness 修正（Node 侧解析 forums.body），与源码无关。

## §5 安全与脱敏

- 不含口令、证书、真实域名/IP/端口、SSH 真值；测试服地址统一 `<YOUR_TEST_DOMAIN>` 占位，凭据仅运行期从 `.local.keys`/环境变量读取。
- 入库仅本任务 inScope 4 文件：`lib.rs`、`tests.rs`（本轮未改，无需提交）、`tests_generated.rs`、本文档；测试脚本/二进制产物/证据目录均 gitignored。

## §6 已知边界与后续

- 历史遗留 PC 版块本轮不迁移（按纪律禁迁移），迁移前仍只在 PC section/list 可见；如需统一，另派任务设计 id 映射回填方案。
- 项目持续迭代，非终态。
