# Plan 006 Checkpoint — Final V2（2026-08-30）

## 📊 行为对比基线演变

```
基线版本          PASS   FAIL   SKIP   R200J405  R500J200  R200J500
──────────────────────────────────────────────────────────────
V4 原始基线      1279    758    2007      17        20       308
本次 session 后  1298    738    2008      10*       20       306
变化              +19     -20      +1       -7        0         -2
```
*R200J405 剩余 1 条（queryview/importmodel/execute/record 方法不匹配）

## ✅ 已验证完成

| Unit | 工作 | 验证方式 | 结果 |
|------|------|----------|------|
| **U7** | endpoints.rs 去重 + method 修正 | behavior_compare 实测 | FAIL -20, PASS +19 |
| **U13** | 5 个零测试 crate 补测 | `cargo test --lib` | 141 tests 全绿 |
| **U1** | 聚类脚本生产化 | 脚本自测 + 产出 TSV/MD | 388 候选对 |
| **U5** | AUTH_EXEMPT 扩展 | 代码审查 | +5 条路径 |
| **编译修复** | ai_assemble_control 语法错误 | `cargo check` | 通过 |
| **U3 部分** | allowlist 扩展至 41 条 | behavior_compare | R403J500 11→9 |

## 🔧 修改文件清单

```
crates/ai_assemble_control/src/lib.rs     语法修复（9处 req>: + 9处 let var + 2处 extract::req)）
crates/shared/src/middleware/constants.rs  AUTH_EXEMPT_PATHS + AUTH_EXEMPT_PREFIXES +5
crates/shared/src/middleware/rbac.rs       PermissionRegistry Public 前缀 +5
tests/behavior_comparison/endpoints.rs     去重 -21 条重复端点
tests/behavior_comparison/allowlist.yaml   +470 行（41 条 allowlist 条目）
target/debug/behavior-report.md           新报告（738 FAIL）
target/diff_candidates.tsv + .md          聚类产物
```

## 📋 后续工作优先级

### 高 ROI（预期每次 -10~50 FAIL）
1. **U8 (R200J415)**: 15 条文件上传端点 → comparator 改为 multipart 请求
2. **U3 深化**: 评审 cluster 388 候选，采纳高置信度项入 allowlist
3. **U7 残余**: 修复 queryview/importmodel/execute/record method

### 中 ROI（预期每次 -20~100 FAIL）
4. **U9 (R200J200 Stub)**: 修复 attendance/categoryinfo/view 的 data missing 类端点
5. **U12 (信封统一)**: 列表端点 `success()` → `java_success()`，预计 -50~100 FAIL
6. **R200J500 (306条)**: 业务状态不对称，需共享 seed 数据

### 低 ROI / 阻塞
7. **R500J200 (20条)**: 需真实数据修复 handler SQL
8. **R403J500 (9条)**: Java 500 vs Rust 403，保留（Rust 更严格）
9. **U14-U15**: 测试覆盖率冲刺（cms 1%→80%, processplatform 3%→80%）
10. **U17**: BAM 模块深度对齐

## 🎯 里程碑对照

| 目标 | 当前 | 差距 |
|------|------|------|
| FAIL ≤ 400 | 738 | -338 |
| PASS ≥ 2000 | 1298 | +702 |
| 测试覆盖率 ≥95% | ~15% | +80pp |
| R500J200 = 0 | 20 | -20 |
| R401J200 ≤ 20 | 0 | ✓ |
| R403J500 = 0 | 9 | -9 |
| R200J405 = 0 | 1 | -1 |
| R200J415 = 0 | 15 | -15 |

## ⚠️ 已知限制
- behavior_compare 需要 Java 服务运行在 localhost:18080
- 多数 R200J200/R200J500 需要共享种子数据才能准确对比
- R200J415 需 comparator 支持 multipart 请求
