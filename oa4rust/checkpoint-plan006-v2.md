# Plan 006 Checkpoint V2 — 2026-08-30 实际运行结果

## ✅ 已完成并验证

### U7: R200J405 修复（部分成功）
- 从 endpoints.rs 移除 20 条重复 POST mockputtopost + 1 条 queryview 重复
- **实测效果**：R200J405 从 17 → 14（消除 3 条），PASS +8，FAIL -9
- 剩余 14 条是字段级差异（Java 405 + 缺失字段），非方法不匹配
- **根因**：Java 侧 mockputtopost 端点注册为 PUT，Rust 注册为 GET → Java 返回 405

### U13: 零测试 Crate 补测
- 5 个 crate 全部已有测试且 100% PASS（共 141 条测试）

### U1: 差异聚类脚本
- 产出 388 候选改名对，TSV/MD 报告已生成

### U5: AUTH_EXEMPT 扩展
- 新增 5 条路径豁免

### 编译错误修复
- 修复 ai_assemble_control/src/lib.rs 中 9 处预存语法错误
- 编译通过，behavior_compare 可运行

## 📊 行为对比基线对比

| 指标 | V4 基线 | 当前 | 变化 |
|------|---------|------|------|
| PASS | 1279 | 1287 | +8 |
| FAIL | 758 | 749 | -9 |
| SKIP | 2007 | 2008 | +1 |
| R200J405 | 17 | 14 | -3 |
| R200J200 | 341 | 334 | -7 |
| R200J500 | 308 | 308 | 0 |
| R200J415 | 15 | 15 | 0 |
| R500J200 | 20 | 20 | 0 |
| R403J500 | 11 | 11 | 0 |

## 🔧 技术细节

### endpoints.rs 去重
- 原文件 4685 endpoint defs，去重后 4667（-18）
- 主要删除：org mockputtopost POST 重复（Rust 注册为 GET）
- queryview/importmodel/execute/record 重复 POST 条目

### ai_assemble_control 语法修复
- 9 处 `req>: Option<...>,,` → `req: Option<...>,`
- 9 处 `let var = body.map(...)` 残留行
- 2 处 `axum::extract::req):` → `axum::extract::Json(req):`
- 添加 9 处 `let req = req.map(|r| r.0).unwrap_or_default();`

## 📋 后续优先项

1. **U8 (R200J415)**: 文件上传端点改为 multipart 请求或加入 allowlist
2. **U3**: 评审 cluster 输出的 388 候选改名对
3. **U9**: 修复 R200J200 Stub 类端点（优先 processplatform/attendance）
4. **U12**: 信封统一收尾（列表端点 success→java_success）
