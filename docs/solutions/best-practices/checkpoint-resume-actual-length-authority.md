---
title: "断点续传的全集权威：盘上 total_items vs 本轮 actual_length（A406 / L295）"
date: 2026-11-09
last_refreshed: 2026-11-09
category: best-practices
module: augmentor/augmentor/checkpoint.py + augmentor/augmentor/pipeline.py
problem_type: bug_pattern
component: checkpoint_resume
severity: medium
symptoms:
  - "断点 total_items=10、completed=[0,1,2]，本轮输入只有 6 条 ⇒ remaining 含 6..9，随后 items[idx] 抛 IndexError: list index out of range"
  - "断点 total_items=6、completed=[0,1]，本轮输入 10 条 ⇒ 新增尾部 6..9 永不进入处理集合"
root_cause: stale_persisted_state_treated_as_current_authority
resolution_type: code_fix_with_guard
tags: [checkpoint, resume, stale-state, actual-length, total-items, ast-wiring, injection-evidence]
applies_when:
  - "断点续传/任务恢复用落盘条数当全集，输入文件可能在两次运行之间变化"
  - "需要同时覆盖『输入变短』与『输入变长』两半，而不是只裁越界下标"
  - "同一逻辑有同步/异步两个入口，需要钉住两处接线"
related_components:
  - augmentor/augmentor/checkpoint.py
  - augmentor/augmentor/pipeline.py
  - augmentor/tests/unit/test_checkpoint_total_items_l295.py
---

# 断点续传的全集权威：盘上 `total_items` vs 本轮 `actual_length`

## Problem

`CheckpointManager.get_remaining_indices` 把断点里落盘的 `total_items` 当成「全集大小」：

```python
all_indices = set(range(checkpoint.total_items))
processed = set(checkpoint.completed_indices + checkpoint.failed_indices)
return sorted(all_indices - processed)
```

`total_items` 只是**上一轮**写断点时的记录，本轮输入文件变了它不会跟着变。两个方向都会坏：

1. **输入变短**：断点 `total_items=10`、`completed=[0,1,2]`，本轮只有 6 条 ⇒ `remaining=[3,4,5,6,7,8,9]`，调用方执行 `items[idx]` 时逐字 `IndexError: list index out of range`。
2. **输入变长**：断点 `total_items=6`、`completed=[0,1]`，本轮 10 条 ⇒ `remaining=[2,3,4,5]`，新增的 6..9 四条永不进入处理集合。`create_checkpoint` 只在 `ckpt is None` 那一支调用，旧断点不会按本轮条数重建。

同步 `augment_dataset` 与异步 `augment_async` 两个入口同形，修复必须同时覆盖两处。

## Root Cause

**把「持久化的旧状态」误当成「当前输入的权威」。** 断点的职责是记住「哪些下标处理过」，不是记住「全集有多大」；全集大小只能由本轮实际读入的 `len(items)` 回答。

另一个容易走错的中间修法是：

```python
effective_total = min(checkpoint.total_items, actual_length)
```

它只修「输入变短」，输入变长时仍会漏尾巴——因为 `min` 仍然把盘上的旧条数当上界。**两半必须一起修**。

## Resolution

`get_remaining_indices` 新增可选参数 `actual_length`，并让它成为全集权威：

```python
def get_remaining_indices(self, checkpoint=None, actual_length=None):
    ...
    effective_total = actual_length if actual_length is not None else checkpoint.total_items
    all_indices = set(range(effective_total))
    processed = set(checkpoint.completed_indices + checkpoint.failed_indices)
    return sorted(all_indices - processed)
```

- **输入变短**：只取 `0..actual_length-1`，不再越界。
- **输入变长**：新增尾巴一并纳入处理集合。
- **不传 `actual_length`**：保持旧行为，兼容直接调用方与既有测试。

两个调用点都传本轮条数：

- 同步：`augmentor/pipeline.py` 的 `augment_dataset`
- 异步：同一文件的 `augment_async`

```python
remaining_indices = self.checkpoint_manager.get_remaining_indices(
    ckpt, actual_length=len(items)
)
```

`items[idx]` 的落点：同步在 `pipeline.py:622` / `:669`，异步在共享的 `_run_batch_async` `:378`（以 L295 落盘时为准）。

## Guard and Evidence

新增 `augmentor/tests/unit/test_checkpoint_total_items_l295.py` 共 16 支，覆盖：

- 签名面（`actual_length` 可选、默认 `None`）
- 输入变短不越界
- 输入变长补尾（含「断点只有 5 条、本轮 100 条」的极端档）
- 边界：0 条、全完成、失败项
- `actual_length=None` 旧形兼容
- **AST 接线臂**：同步与异步两个调用点都必须传 `actual_length=len(items)`，摘掉任一处的参数即红

两臂注入取证（改完逐字节复原）：

| 注入 | 读数 | 点名 |
|---|---|---|
| 退回 `min(total_items, actual_length)` | `3 failed, 16 passed` | `test_new_tail_items_included` / `test_tail_after_failed_items` / `test_actual_length_extremely_long` |
| 摘掉任一调用点的 `actual_length` | `1 failed, 15 passed` | `TestPipelineWiringL295` |

定向复跑 `71 passed`（本守卫 + `test_async_checkpoint_resume_l277` + `test_checkpoint` + `test_run_scoped_checkpoint_state_l275`）。

全量定标：`39 failed, 10222 passed, 6 skipped, 711.00s`，覆盖率 `99.62%`。红项是 `main` 既有基线债（同批 HEAD 对照可复现 38 项，a184 的语料/工作树差异另计），本轮未扩大修复范围，不声称全绿。

## Lessons

1. **「谁回答全集大小」要单独写清**：落盘状态只回答「处理过什么」，当前输入回答「一共有多少」。两者混用就会在输入变化时坏掉。
2. **变短与变长是两个判据**：只裁越界（`min` / `[:len(items)]`）只修一半；测试必须同时钉「不越界」与「不漏尾」。
3. **接线也要有守卫**：逻辑函数修对了，调用点漏传参数照样复发。用 AST 钉住每个入口的调用形状，比只测函数本身可靠。
4. **注入取证要能点名**：每个注入臂都应报出预期的那几支测试，而不是只报「变红了」。
5. **对外可观测面零变更**：本轮只改待办下标集合，回执形状、状态码、产物字节、落盘路径未动，因此不需要产品口径拍板。

## References

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L295 清单行与日志块，A406 行（`~~M~~ **已收口（L295）**`）
- 提交：`d98af7723`（`fix,test,docs(augmentor): L295 A406 checkpoint actual_length 校平`）
- 相关守卫：`augmentor/tests/unit/test_async_checkpoint_resume_l277.py`（monkeypatch lambda 接受 `actual_length=None`）
