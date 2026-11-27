# R605a 后端申诉状态机三端点状态前置谓词修复审计（t177）

> 脱敏说明：本文所有连接信息以 `<TEST_HOST>:<PORT>` 占位，0 真值；不含任何口令/IP/域名。

## 一、缺陷与根因
- 模块：`attendance_assemble_control` 的 v2 申诉三端点。
- 真父实现中的 UPDATE 仅以 `id` 为条件，无当前状态前置谓词，导致任意状态可被直接跃迁：
  - status=2（已通过）仍可再次 start/process；
  - status=0（待处理）可直接 reset/status 或 end/process 写为通过。
- 这是状态机约束缺失（业务正确性缺陷），不涉及归属越权：`appeal_owner_gate` 归属门保持原样。

## 二、修复方案（最小改动，仅三端点 SQL + n==0 错误分支）
- `v2_appeal_start_process`：WHERE 追加 `AND status = $4`，`$4` 绑定常量 0i32（仅 status=0 可发起/处理）。
- `v2_appeal_reset_status`：WHERE 追加 `AND status = $3`，`$3` 绑定常量 1i32（仅 status=1 可重置）。
- `v2_appeal_end_process`：WHERE 追加 `AND status = $4`，`$4` 绑定常量 1i32（仅 status=1 可写结果）。
- 三端点 `n == 0`（无行命中，即非法跃迁）统一返回 `ActionResult::error("appeal not in expected state")`，保持 envelope 风格；合法路径返回体与状态字段零改。
- diff 精确：lib.rs 9 insertions / 9 deletions，仅上述三端点。

## 三、守卫（零改清单）
- `manager_status` 的 UPDATE（`update_status_admin_person = $3`）零改——管理员端点不走三端点状态机。
- `appeal_owner_gate` 归属门、`start_check` 语义零改。
- 其他端点、routes.rs、Cargo.toml、migrations 全部零改；无新增依赖。
- 合法跃迁 start 0→1、reset 1→0、end 1→2 均成功；非 owner（bob 读 alice）仍 Forbidden。

## 四、RED → GREEN 证据（真父源码覆盖定点 replay）
- 真父备份 sha256 = e59e9b38f22152ef95959fbb08d11eecdcdae0378ef5b355189fac95a9c78491，与任务真父 HEAD 处 lib.rs 字节一致。
- RED：以真父备份覆盖源码并 touch 强制重编译，定点测试 EXIT=101，1 failed / 1 passed；失败点为 status=2 的 start 仍返回 success（status 被写为 1），守卫用例真父通过。
- 修复版 sha256 = 81b243bf5ecde12e9559593d04caa69672e254084a6d2cfba11abd95bc347128；恢复并 touch 后 GREEN EXIT=0，2 passed。
- 契约 sha256 = 7fd427e0fa5c76926f5fb89692a3430c463790c0711199f3a5f946155f1a7537，2 用例：非法跃迁被拒（核心，真父必 RED）+ 合法跃迁与 owner/start_check 守卫。
- 契约以直调 handler + 临时 PG（scratch 库用完即删）真测；seed 5 行覆盖 status 0/1/2 与双 person。

## 五、三 verify（全 EXIT=0）
- V1 定点 `cargo test -p attendance_assemble_control --test r605a_appeal_state_machine_fix`：EXIT=0，2 passed；日志 sha256 = a468fba0b02da17c923c9e52605ab8926bd3d37e203c0d6a23173805a7feab0e。
- V2 全量 `cargo test -p attendance_assemble_control`：EXIT=0（含 lib 与多测试二进制全绿；r605a 契约 2 passed）；日志 sha256 = 3d82f0ee6b53ef4ffe59912686de6aa3cac33bfdf5ca15375d98b14edff10df4。
- V3 `cargo fmt --check -p attendance_assemble_control` EXIT=0（空日志 sha256 = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855）；`cargo clippy -p attendance_assemble_control -- -D warnings` EXIT=0（日志 sha256 = e25dc227630613161429b74468eea2ddd1ae4bc2071ccb94a31333722b322bfe）。
- RED 日志 sha256 = ead637f5daa15b0fb9d3a90a03d98f021807a0a8996ecf528d78c78a76a86c67。

## 六、据实披露
- 当前工作树 HEAD 已由他组提交前移至 a486e62c；三目标文件在该 HEAD 的字节sha与真父备份一致，故真父覆盖 RED replay 仍有效。
- 临时 PG 容器端口以 `<TEST_HOST>:<PORT>` 占位；`POSTGRES_HOST_AUTH_METHOD=trust`，无口令入库；scratch 库与基础库均为临时，用完即删。
- 一次性迁移测试 `tests/r605a_migrate_once.rs` 仅用于为 V2 全量测试准备基础库，验证通过后已删除，未入库。
- 未跑浏览器 GUI：本缺陷为纯后端 SQL 状态机约束，以后端真测 + 源码守卫覆盖，GUI 不适用。

## 七、sha 表（runnerSha 为紧凑 JSON 元数据摘要；文件 sha256sum 为落盘字节摘要）
- runnerSha = af581244237f015cc136fd3b2866e5ce137f911a0cab61b1ef6841aeaeabdfe3
- 修复版 lib.rs = 81b243bf5ecde12e9559593d04caa69672e254084a6d2cfba11abd95bc347128
- 契约 = 7fd427e0fa5c76926f5fb89692a3430c463790c0711199f3a5f946155f1a7537
- GREEN 日志 = a468fba0b02da17c923c9e52605ab8926bd3d37e203c0d6a23173805a7feab0e
- RED 日志 = ead637f5daa15b0fb9d3a90a03d98f021807a0a8996ecf528d78c78a76a86c67
- V2 全量日志 = 3d82f0ee6b53ef4ffe59912686de6aa3cac33bfdf5ca15375d98b14edff10df4
- 本文不含自身 sha（避免自引用悖论）。
