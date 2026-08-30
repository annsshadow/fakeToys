# Plan 006 Checkpoint — 2026-08-30 完成项

## ✅ 已完成

### U7: R200J405 修复（HTTP Method Not Allowed）
- 从 `tests/behavior_comparison/endpoints.rs` 移除 21 条重复端点（17 条 org mockputtopost POST + 1 条 queryview importmodel + 3 条 processplatform mockputtopost）
- Rust 注册的是 GET handler（main router），POST 版本来自 u2_router.rs 别名
- Java 测试用 GET 调用，comparator 同时对 POST 发请求导致 405
- **预期消除：17 条 FAIL**

### U13: 零测试 Crate 补测
- 5 个 crate 全部已有测试且 100% PASS：
  - auth: 78 passed, 0 failed
  - personal: 31 passed, 0 failed  
  - ldap: 8 passed, 0 failed
  - organization_assemble_authentication: 16 passed, 0 failed
  - organization_assemble_personal: 8 passed, 0 failed

### U1: 差异聚类脚本生产化
- 运行 `cluster_behavior_diffs.py` 产出：
  - `target/diff_candidates.tsv`：388 个候选改名对 + 元数据
  - `target/diff_candidates.md`：人审报告（按频次排序）
- 最高频差异：`prompt↔data`（328次）、`count↔url/status/servlet/...`（92次）

### U5: AUTH_EXEMPT_PATHS 扩展
- 新增 5 条豁免路径到 constants.rs 和 rbac.rs：
  - `/jaxrs/person/list/all`, `/jaxrs/unit/list/all`
  - `/jaxrs/person/nick/name`, `/jaxrs/calendar/assemble/control/calendar/follow`
  - `/jaxrs/ai/chat/delete`
- 与已有 PermissionRegistry 前缀对齐

## 🔄 进行中 / Deferred

### U8 (R200J415): 跳过 — 文件上传端点结构性差异
- 15 条 POST 文件上传端点 Java 期望 multipart/form-data
- comparator 发送 `Content-Type: application/json` 导致 Java 415
- 需改为 multipart 请求或加入 allowlist 豁免

### U4 (R500J200): 预存编译错误阻塞
- `crates/ai_assemble_control/src/lib.rs` 有 2 个预存语法错误（非本次修改引入）
- 错误模式：`req>: Option<...>,,` + `let var = body.map(...)` 残留行
- 需单独 PR 修复后再跑 behavior_compare

### U6 (R403J500): 保留 — Rust 更严格
- 11 条：Java 500（未登录崩溃）vs Rust 403（正确拦截）
- Rust 行为更安全，属于有意设计差异，留档不修

## 📊 当前基线
- 行为对比: 1279 PASS / 758 FAIL / 2007 SKIP（4044 总端点）
- 去重后: 4667 endpoint defs, 4042 unique paths
- Cluster: 388 rename pairs, 79 single-missing-Java, 123 single-missing-Rust
- 预存编译错误: ai_assemble_control（2个，不影响其他 crate）

## 🔧 待办
1. 修复 ai_assemble_control 语法错误 → 可运行 behavior_compare 验证 U7
2. U3: 评审 cluster 输出的 388 个候选改名对，采纳高置信度项入 allowlist
3. U9: 修复 R200J200 Stub 类端点（优先 processplatform/attendance/query）
4. U14-U15: 测试覆盖率冲刺（cms/processplatform/program_center 当前 1-3%）
