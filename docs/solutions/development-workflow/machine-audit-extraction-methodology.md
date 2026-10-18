---
title: "机器提取 + 回源码核验的审计方法论：提取器可信度、基线门禁与口径陷阱"
date: 2026-09-20
last_refreshed: 2026-10-14
category: development-workflow
module: docs/audits/three-ends-2026-09-20/ + oa4rust 三端
problem_type: workflow_improvement
component: audit_tooling
severity: medium
symptoms:
  - "审计结论基于不可信提取器：链式 .post(h).put(h) 漏提 116 条注册；模板表达式先切 ? 再替换 ${...} 截断路径；类型注解 [^=]* 跨行吞整块"
  - "修复后无法证明无回归：A/B/C/D/E 五类缺陷计数谁都可以报，没有人证明门禁真的会变红"
  - "消费率两个口径（域桶 382/4072 vs 全局 391/4638）相差一倍，汇报互相矛盾"
  - "往视图里加注释「消费 /api/program 别名族…」被端点扫描正则提取成裸端点，守卫误报"
root_cause: extractor_defects_and_undefined_baseline_semantics
resolution_type: process_and_tooling
tags: [audit, machine-extraction, baseline-gate, fail-loud, consumption-metrics, contract-audit]
applies_when:
  - "对大型代码库做机器辅助审计（路由/调用/字段/schema 对账）"
  - "为审计结论建立防回归门禁"
  - "汇报「消费率/覆盖率」类指标"
related_components:
  - docs/audits/three-ends-2026-09-20/（extract_routes / extract_calls / compare / schema_audit）
  - .github/workflows/three-ends-contract.yml
  - oa4rust/frontend/apps/desktop/src/views/*.vue（rev116 注释踩坑现场）
---

# 机器提取 + 回源码核验的审计方法论

## Problem

大规模审计的第一性问题不是"发现了多少缺陷"，而是"提取器可信吗"。本仓三端审计过程中，提取器自身的 9 处缺陷（3 处语法级 + 6 处语义级）每一处都曾产出过错误结论；而修复推进后，"基线门禁"若没有可失败的证明，就只是装饰。

## Resolution

### 提取器可信度：fail loud，先反例证伪再修

| 缺陷 | 症状 | 修正 | 验证 |
|---|---|---|---|
| 方法正则排除 `.` | 链式 `.post(h).put(h)` 的 put 全丢 | 只排除 `[A-Za-z0-9_]` | 注册数 4939→5055（+116），`data/work/{id}` 由"仅 POST"纠正为 POST+PUT |
| 先切 `?` 再替换 `${...}` | 含三元表达式的模板路径被砍断 | 先替换模板再切查询串 | 消掉 1 处假 404 |
| 类型注解 `[^=]*` 跨行 | 对象表整块误吞 | 限定 `[^={;\n]*` | 未解析调用 18→2 |
| `&fmt()` 前缀未还原 | bbs 106 条路由前缀残缺 | 还原引用前缀 | 7 处误报回滚 |
| 相对导入匹配不到 | `data.cleaner` 被判死代码（实际 3 处活跃调用方） | 正则同时覆盖相对导入 | 避免误删模块 |

规则：**每个提取器修正必须附带"修正前错在哪里"的数字证据**；提取覆盖率报出来（335/337 静态解析成功 = 99.4%），剩余未解析项人工核验登记。

### 基线门禁：只降不升 + fail-injection 证明可红

- `compare.py --gate` / `schema_audit.py --gate`：A 类（影子/死链）必须为 0；B/C/D/E 不得高于 `schema_baseline.json`（只许降不许升）。
- **门禁可失败必须用注入验证**：人为压低基线 → EXIT=1，恢复 → EXIT=0；同理 fail-injection 注入 bogus 键应使 B 类计数上升。没有这一步的门禁不算数。
- 提取器升级变敏感时，基线条目允许"因提取器变敏感而移动"，但必须在基线文件内留 `_note` 说明——计数移动要可解释。

### 消费率/覆盖率口径：先定义分母，再谈百分比

- 本仓两套口径：域桶口径 33 业务域（分母 4072，排除 566 条跨域基础设施如 /api/permission、/api/commend）vs 全局口径（分母 4638）。**以全局口径汇报更诚实**，域口径只用于逐域推进排序。
-  mock* 兼容别名（407 条）是 O2OA 兼容层，**不可清理**——审计发现"重复"时先判定是否为兼容层再下清理结论。
- 加只读消费点前必须查两道守卫：① 该视图 `*.test.ts` 有无 autoquery-prune 守卫；② 是否"裸路由运行时 500 型"（注册了但无参 handler 裸调即 500，如 BBS list 族）。踩坑实录：JPushApp 只读别名被 string-match 守卫整体封禁，BBS 裸路由撞 500 守卫后 revert。

### 注释与扫描器的交互（rev116 踩坑）

端点扫描用 `/api/[A-Za-z0-9_\-/{}$.]+` 扫源码全文**含注释**。注释里写「消费 /api/program 别名族…」会被提取成裸端点 `/api/program` → 命中"无未注册端点"守卫报红。**注释中勿写以 /api/ 开头且可被截断成裸/短端点的字面量**；改写注释去掉前导斜杠。

同理：字符串拼接生成 URL（`url = 三元` / `'...' + id`）会被调用提取器在 `+` 处截断成 `market/{}` 误判——**视图里消费端点一律用模板字面量**。

## Why This Matters

审计报告的价值随时间衰减：绑定 HEAD 的计数会过期、提取器升级会移动基线、口径不同会让两个汇报互相矛盾。把"提取器可信度证据、门禁可红证明、口径定义"三者固化，审计才能从一次性报告变成持续门禁。

## When to Apply

- 任何「机器提取 + 人工核验」式审计（路由对账、死链检测、schema 校验、覆盖率核算）
- 为质量指标设 CI 门禁
- 汇报跨团队可见的百分比指标前

## Related

- 计划：`docs/archived-plans/2026-09-20-001-audit-oa4rust-three-ends-completeness-plan.md`（§〇 方法与可复现性、§6.8 提取器缺陷、§九 rev6/rev11-12/rev116）
- 绿地切流判据：`../architecture-patterns/greenfield-cutover-replaceability-criteria.md`
- 门禁红绿验证思想同 `../best-practices/test-hygiene-ast-triage-and-coverage-traps.md`
