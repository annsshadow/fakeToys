---
title: "测试卫生：AST 分诊假测试、覆盖率≠行为验证、版本号单一来源"
date: 2026-09-21
last_refreshed: 2026-10-14
category: best-practices
module: augmentor/tests + augmentor/api + cli
problem_type: workflow_improvement
component: test_suite
severity: high
symptoms:
  - "99.46% 语句覆盖率下，F1（质量评分恒 pass）/F2（符号遮蔽）/F3（任意文件读）三个真实缺陷全部漏网"
  - "28 个占位文件 / 68 个空洞断言（assert True、两个字面量互比、assert X or True）混入 3061 个测试"
  - "augmentor.__init__.py 同名二次导入：QualityReport 被 quality_report 版静默遮蔽，ReportGenerator.generate() 返回类型 isinstance 判定为 False"
  - "版本号 2.2.0 / 2.0.0 / 1.0 三处并存；ARCHITECTURE.md 称 7 个 router（实际 11）、11 个子命令（实际 44）"
root_cause: coverage_measures_execution_not_behavior
resolution_type: process_and_code_fix
tags: [test-hygiene, coverage, ast-triage, symbol-shadowing, version-consistency, branch-coverage]
applies_when:
  - "高覆盖率下仍出生产缺陷，需要审计测试套件本身"
  - "整理大型遗留测试目录（识别占位/空洞测试）"
  - "维护公开包导出面与版本号"
related_components:
  - augmentor/scripts/triage_tests.py
  - augmentor/tests/unit/test_test_hygiene.py
  - augmentor/augmentor/__init__.py
  - augmentor/augmentor/export.py / export_enhanced.py
---

# 测试卫生：AST 分诊假测试、覆盖率≠行为验证、版本号单一来源

## Problem

覆盖率度量的是**语句执行**，不是**行为验证**。本仓实测：99.46% 语句覆盖的套件漏掉 3 个真实缺陷；删掉 68 个假测试后覆盖率反而上升（99.41%→99.50%）——假测试不但不保护正确性，还稀释覆盖率的可信度。

## Resolution

### AST 分诊：机器判据替代人工感觉，且必须先做误报校准

`scripts/triage_tests.py` 逐函数 AST 分析（常量传播），判据是「**这条测试能否失败**」，不是「有没有 assert」：

| 判据 | 说明 |
|------|------|
| 占位文件 | 无任何有效检查点、又无未被 try 包裹的调用 → 空壳 |
| 空洞断言 | `assert True`；`assert <字面量> <op> <字面量>`（如 `assert 94.17 >= 80.0`）；`assert <单次绑定常量> == <字面量>` |
| 合法保留 | `writer.close()` 这类"不抛异常"测试——close 若开始抛异常测试就会红 |

**两次自我纠错（必须写进判据设计）**：
1. "出现过 `name = <字面量>`"≠常量——`call_count = 0`（后续闭包 `+=`）、`missing = []`（后续 `append`）会被误判。修正：**单次绑定 + 从不被变异 + 不作为参数传出**才算常量。第一次误报若不查，会删掉 210 个真实测试（test_round*/test_micro_branches 家族 210 个全是真测试，命名难看≠无效）。
2. `assert X or True` 更隐蔽：`assert hasattr(cache,'_stats') or True`、`assert "quality_a" in result.quality_diff or True`（该键根本不存在）——**改真后 3 处暴露了产品代码的真实情况**（quality_diff 键名、AppConfig 无 _load_cache、_stats 是死状态），不是删掉假装没看见。

配套门禁：`tests/unit/test_test_hygiene.py`（4 条）+ `triage_tests.py --check` 接 CI，并用临时探针验证过门禁确实会红（探针本身要先干净——我第一次写的 lint 探针恰好合法，"门禁没拦住"的结论是错的）。

### 覆盖率门禁升级

- `--cov-branch`：本仓语句 99.50% / 分支 98.63%，只差 0.87pp（经验上分支通常低 10-20pp）——**执行是扎实的，病在断言空洞**。分支覆盖不足以抓假测试，两者要组合看。
- 复核"伪问题"再动手：判 `pragma: no cover` 滥用 → 生产源码 0 处（唯一一处在测试文件的合法用途）；判 augmentor 缺 .gitignore → 根规则已全覆盖。每个"显而易见的问题"先复核再修。

### 回归测试纪律：修复前必须失败（红→绿）

F1/F2/F3 每条修复的回归测试，必须验证「`git stash` 掉修复后测试确实变红」。实测：临时还原 F1/F3/F4 修复后 `test_api_security.py` 8 例失败，恢复后 24 例全过。红→绿不是仪式，是证明测试锁定缺陷的唯一方式。顺带修掉"固化错误契约"的测试（缺文件应 404 而非 500）。

### 符号遮蔽：同名导入是类型契约断裂

`__init__.py` 中 `QualityReport` / `ValidationResult` 各被导入两次，后者静默覆盖前者：`ReportGenerator.generate()` 返回 report.QualityReport，`isinstance(r, augmentor.QualityReport)` 为 False，`.pass_rate` 直接 AttributeError。

处置：真正冲突的四个名字改显式别名（`PipelineQualityReport` / `DataValidationResult` / `ConfigValidationResult` / `DatasetQualityReport`），旧名保留 PEP 562 惰性别名 + DeprecationWarning——**避免 `__all__` 209 项大面积改名打穿下游**。教训：`Enum.__call__` 按成员身份匹配，跨枚举类的同值成员一律不认——两个同名 `ExportFormat`（6 成员 vs 13 成员）让公开枚举 13 个成员**无一能传给** `pipeline.export_dataset`（连值相同的 CHATML 也不行），用户唯一可用写法是裸字符串。修法是删本地枚举、复用唯一来源 + `NATIVE_FORMATS` 显式声明支持边界，并注意三处必须同改（只删枚举会让非原生格式静默不写文件却返回成功——比 ValueError 更难排查）。

### 版本号单一来源 + 文档不漂移

- 全仓唯一权威：`augmentor.__version__`；api/main.py、status.py、web/package.json、docker-compose、CD 示例全部引用它；元测试 `TestVersionConsistency` 断言 app.version == __version__ == package.json.version。
- 注意两个版本轴：Docker 镜像 tag 来自**发布 git tag**（`ai-platform-v*`，CD 剥离前缀），不是包版本——发布 2.2.0 必须打 `ai-platform-v2.2.0`。
- 文档中所有会漂移的**数字**改为「见 `ls api/routes/`」式指引或不写具体行数，否则 7 router→11、11 命令→44 的漂移必然重演。

## Why This Matters

测试套件自己会腐烂：假测试混进来、符号被遮蔽、版本号分叉，而覆盖率全绿。本组实践的共同主线是**验证手段本身要先被验证**（分诊脚本做误报校准、门禁做红绿探针、回归测试做 stash 验证）——不验证的度量比没有度量更危险。

## When to Apply

- 审计任何"覆盖率很高但仍出缺陷"的测试套件
- 大型测试目录清理（先机器分诊再人工）
- 维护 Python 包公开导出面 / monorepo 版本号

## Related

- 计划：`docs/plans/2026-09-21-001-audit-augmentor-fullstack-optimization-plan.md`（F2/F6/F7/F10/F13、Phase 0/1 全部偏差登记）
- 变异测试：`../testing/mutation-testing-operational-pitfalls.md`
- API 安全：`../security-issues/api-path-whitelist-and-write-authz.md`
- 机器审计方法：`../development-workflow/machine-audit-extraction-methodology.md`
