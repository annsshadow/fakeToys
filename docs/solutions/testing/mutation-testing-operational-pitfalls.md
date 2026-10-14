---
title: "变异测试的运维陷阱：假 100% 杀死率、等价变异体与逐模块门槛"
date: 2026-09-21
last_refreshed: 2026-10-14
category: testing
module: augmentor（quality/dedup/export）
problem_type: testing_methodology
component: mutation_testing
severity: medium
symptoms:
  - "cosmic-ray 首轮报告 export 51.7% 杀死率，但合计 60.1%「达标」——quality/dedup 把薄弱模块拖过线"
  - "28 个存活变异体里约 20 个是等价变异体（indent=1/3、ensure_ascii=True、except 永不抛出的异常…），得分被系统性压低"
  - "test-command 写裸 python 时所有变异体被误判 killed——托管解释器没装 pytest，ModuleNotFoundError 退出码非 0"
  - "PATH 前置 venv Scripts 也无效；cosmic-ray 用 shlex posix 模式解析，反斜杠被当转义符吃掉"
root_cause: toolchain_and_metric_design
resolution_type: methodology
tags: [mutation-testing, cosmic-ray, mutmut, equivalent-mutants, kill-rate, windows]
applies_when:
  - "引入变异测试验证「测试是否验证行为」"
  - "用变异杀死率做质量指标或 CI 门禁"
  - "Windows 环境跑变异测试"
related_components:
  - augmentor/scripts/mutation/run.py
  - augmentor/scripts/mutation/sample.py
---

# 变异测试的运维陷阱

## Problem

变异测试是唯一能度量「测试是否验证行为」的自动化手段（语句覆盖只度量执行），但工具链的三个坑会让它产出**比失败更危险的假好消息**：假的 100% 杀死率、被等价变异体压低的得分、被合计掩盖的单模块失败。

## Resolution

### 陷阱 1：验尸官本身要先验证（假 100% killed）

裸 `python` 作 test-command 时，cosmic-ray 在托管解释器上跑 → `ModuleNotFoundError: No module named pytest` → 退出码非 0 → **每个变异体都被判 killed**，报告 100% 杀死率。更隐蔽的是：把 venv 的 `Scripts` 前置到 PATH 也无效（仍解析到托管版）。

对策（`scripts/mutation/run.py`）：
- TOML test-command 用 `{python}` 占位符，运行时替换为 `sys.executable`；**缺失占位符直接报错退出**。
- cosmic-ray 用 `shlex.split`（posix 模式）解析命令，**路径必须写正斜杠**（反斜杠被转义吃掉）。
- 小样本（`--per-family 1`）先行端到端验证；用 A/B 对照验证退出码管道：`-c "import pytest"` / `-c "pass"` / `-c "raise SystemExit(1)"` 应分别产生 killed/survived/killed。

**教训：变异测试必须先验证"验尸官"本身可信，否则 100% 杀死率看起来像好消息。**

### 陷阱 2：等价变异体系统性压低得分

| 等价类型 | 为什么杀不死 |
|---|---|
| `json.dump(indent=2)` → `indent=1/3`（6 个） | 纯缩进，解析型断言测不出 |
| `ensure_ascii=False` → `True`（4 个） | json.load 往返解码后相同 |
| `use_parallel=True` → `False`（2 个） | 串行/并行输出本就等价 |
| `except Exception` → `except CosmicRayTestingException`（2 个） | 该异常永不抛出 |
| `max_workers=4` → `5`（1 个） | 线程数不影响输出 |

cosmic-ray 默认算子集（尤其 `NumberReplacer`、`ReplaceBinaryOperator_*`，占 quality.py 变异体约 70%）对这类代码产生大量等价变异体。

对策：
- 按**算子族分层等距抽样**（1320 → 173：quality 52 / dedup 63 / export 58），全量约 92 分钟且 70% 等价，信息量低。
- 得分**只作参考不作 CI 门禁**；存活变异体逐条人工甄别"真盲区 vs 等价变异"。
- 抽样前校验"单模块测试文件对该模块的语句覆盖率"（97%/99%/98%），确保"每变异体只跑本模块测试"不虚低。

### 陷阱 3：合计分数掩盖单模块失败

首轮合计 60.1% 达标，但 export 单独只有 51.7%——quality/dedup 把它拖过线。**门槛必须逐模块判定**（本仓：≥60% each），汇总只作参考。补 7 条契约测试（每条注明杀死的变异体）后 export 51.7%→69.0%。

存活的真盲区典型形态——「断言被错误来源满足」：
- `if not use_parallel:` 翻转：测试只断言"6 种格式都导出了"，两条路径输出一致，条件写反不可观测；
- `ext = ".csv" if fmt == CSV else ".json"` 翻转：只断言 `Path(path).exists()`，而返回值就是刚写的路径，后缀错照样存在；
- `for turn in history:` → `for turn in []:`：`assert "user" in roles` 被"当前问题"那条满足，历史全丢也通过。

修复方向：断言**输出内容**而非"调用发生过/文件存在过"。

### 平台注意

`mutmut` 3.8 在原生 Windows 直接拒绝运行（要求 WSL）；WSL 被本机安全策略阻断时改用 `cosmic-ray`（跨平台、在维护）。

## Why This Matters

"覆盖率达标"与"测试能杀死变异体"之间隔着一个数量级的信心。但变异测试的数字同样是工具链的产物——退出码管道错了，它比覆盖率更会说谎。三条纪律：验尸官先自证、逐模块门槛、等价变异体人工甄别。

## When to Apply

- 评估测试套件的真实保护力（超越覆盖率）
- 引入/运维变异测试工具链
- 审计"测试全绿但仍出生产缺陷"的根因

## Related

- 计划：`docs/plans/2026-09-21-001-audit-augmentor-fullstack-optimization-plan.md`（T1.4 及「工具链上的一个坑」「发现 1/2/3」）
- 测试卫生：`../best-practices/test-hygiene-ast-triage-and-coverage-traps.md`
- API 契约断言模式：`../security-issues/api-path-whitelist-and-write-authz.md`（api.ts 测试写法）
