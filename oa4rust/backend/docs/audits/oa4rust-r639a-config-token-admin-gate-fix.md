# R639a config_token 系统令牌裸读补 admin 门真修复报告

- 对象：`program_center` 服务 `config_token` 接口
- 性质：真修复（RED→GREEN），判级沿用 R638c 甄别 P2，非新判级
- 写域：仅修改 `crates/program_center/src/lib.rs`，新增本报告；临时回归契约跑完不入库

## 一、修复结论

在 `config_token` 入口施加与写侧一致的 admin 门：签名新增 `session: Extension<shared::session::Session>`，首个 DB 访问（`acquire`）之前调用 `require_admin(&pool, &session).await?`。匿名请求因缺会话扩展无法进入 handler，非管理员返回 403，管理员保持原回传形状；SQL、字段映射与路由均未改动。R638c 甄别的“认证后越权读系统令牌配置”缺陷已消除。

## 二、锚点与修复点

- 对象文件：`crates/program_center/src/lib.rs`（修复后 13669 行）。
- 修复前（原 SHA `21985a04b5147b487cf1edabce965616f838967ec92b84d7e82ecf6ea8b6c94b`）：`config_token` 约 3842 行仅 `pool: Extension<Pool>`，约 3843 行直接 `acquire`，约 3846-3849 行查询 `system.token`，约 3853-3881 行回传整行含 `value`，无任何身份门。
- 修复后：`config_token` 约 3842-3847 行为三行签名（新增会话扩展）并首行 `require_admin`，再 `acquire`；约 3849-3887 行 SQL 与回传保持原样。
- 复用同文件现成门：`require_admin`（约 9346-9352 行，内部 `is_admin` fail-closed）与 `config_save`（约 3752-3758 行，`require_admin` 置于 `acquire` 前）范本。
- 写侧一致性：`u2_config_token_put`（约 9846-9852 行）经 `u2_config_domain_put`（约 9777-9783 行首行 `require_admin`）更新 `system.token`，读写门控现已对称。
- 路由零改：`crates/program_center/src/routes.rs` 约 422 行 GET 仍挂 `config_token`。

## 三、RED→GREEN 真执行证据

- 方法：独占临时 PostgreSQL 实例，oneshot 直调 `program_center::router`；用非真的占位夹具值注入 `system.token`，四臂断言；另有源码守卫断言签名、门控顺序与查询键。
- RED（修复前）：单测 panic 于普通会话读取断言——期望 403、实际 200，失败 1/1，退出码 101，缺陷坐实。
- GREEN（修复后）：`running 1 test / 1 passed; 0 failed / EXIT 0`，终端输出 `R639A four arms OK: non-admin 403, admin 200 with value, no-session blocked, source guard passed`。四臂为：
  1. 普通登录（非 admin）用户读令牌返回 403，且响应不含占位 `value`；
  2. 管理员读令牌返回 200 且 `data.value` 为占位值；
  3. 不注入会话层时请求不能成功返回，且不泄露 `value`；
  4. 源码守卫确认签名含 `Extension<shared::session::Session>`、`require_admin` 位于 `acquire` 之前、仍查询 `system.token`。

## 四、验证矩阵（Fail Loud）

| 验证 | 命令 | 结果 |
| --- | --- | --- |
| V1 定点回归 | `cargo test -p program_center --test r639a_config_token_admin_gate_contract -- --test-threads=1` | PASS：1 passed / 0 failed，EXIT 0 |
| V2 包级 | `cargo test -p program_center -- --test-threads=1` | 310 passed / 12 failed，EXIT 101；12 个失败均为写域外基线 fixture 问题 |
| V3 格式 | `cargo fmt --check -p program_center` | PASS，EXIT 0 |
| V3 静态检查 | `cargo clippy -p program_center --all-targets --no-deps -- -D warnings` | PASS，EXIT 0 |

V2 失败据实披露（非本次修改引入）：`invoke_token_tests` 在 `lib.rs:13596` 因 scratch 库未建表报 `relation "x_program_invoke" does not exist`；`security_gate_tests` 在 `shared/src/testing.rs:133` 报 `relation "auth_person" does not exist`；`security_gate_tests_idor_b3` 7 项在 `security_gate_tests_idor_b3.rs:34` 报 `relation "auth_role"/"x_program_collect"/"x_program_module" does not exist`，另 1 项在 `:409` 断言 500≠200；`tests_u2` 3 项在 `tests_u2.rs:372/481/612` 断言 500≠200。上述 panic 点均远离 `config_token`（3842 附近），根因为 scratch 库缺 migration / 既有夹具不一致，不能归因为本次 admin 门修复；本项未按“全绿”包装。

## 五、行为变化与复现

- 修复前：任意已登录用户 GET `/api/program_center/config/token` 可读 `system.token` 的整行含 `value`。
- 修复后：非管理员会话返回 403 且不泄露 `value`；管理员维持 200 与原字段；无会话请求在扩展解析阶段即被拒。
- 复现前提：测试库经 `<TEST_HOST>` 占位连接注入，令牌值为明确非真占位值；真值由部署环境注入，不入仓、不回显。
- 据实披露：生产环境全局 `auth_middleware` 仍先拦截匿名；oneshot 直调 `router` 不经该层，用于验证 handler 自身门控，不代表生产匿名可达。

## 六、SHA 分列

- 审计对象 `crates/program_center/src/lib.rs` SHA-256：`3d56e8503feb015a0bb1396c8580c4092c5ad5c0eca09d3b96220a2e1205d048`（本次修复后，唯一源码改动）。
- runnerSha（临时回归契约 SHA-256）：`8ce31317ec9b577881231bdff402f7f6de77313ef36984eed0b6551af7865482`（验证型临时文件，跑完删除，不入库）。
- docSha（本报告全文 SHA-256）：以提交时文件内容为准，见提交记录。
