---
title: "读侧要点名任务、只读面不许有副作用：把「环境态读法」与「点名读法」分成两条路（A408 / L308）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: best-practices
module: augmentor/augmentor/checkpoint.py + augmentor/api/routes/augment.py
problem_type: read_side_not_bound_to_the_requested_subject
component: task_scoped_progress_read
severity: medium
symptoms:
  - "GET /api/augment/progress 回执里的 task_id 可能是「最后加载断点的那一次运行」，不是调用方问的那一次"
  - "并发两个任务时，进度卡显示的是另一个任务的数"
  - "客户端算不出任务身份（那个 id 按服务端解析后的路径算）⇒ 想问「我这次」也问不了"
root_cause: read_side_falls_back_to_environment_state_and_the_client_cannot_compute_the_subject_id
resolution_type: task_scoped_read_only_accessor_plus_handing_the_id_back
tags: [read-side, task-identity, progress, environment-state, read-only, side-effects, api-contract, frontend-polling]
applies_when:
  - "一个只读端点的语义是「环境态」（当下谁在跑），但调用方其实想问「我这一次的」"
  - "被问的对象的身份由服务端算（路径哈希之类），客户端算不出来"
  - "现有的读函数是**消费型**的（读一次就改状态），不能直接拿来当只读面用"
related_components:
  - augmentor/augmentor/checkpoint.py
  - augmentor/api/routes/augment.py
  - augmentor/augmentor/pipeline.py
  - augmentor/tests/unit/test_progress_task_binding_l308.py
  - augmentor/tests/unit/test_run_scoped_checkpoint_state_l275.py
---

# 读侧要点名任务、只读面不许有副作用：把「环境态读法」与「点名读法」分成两条路

## 现象

A408（L294 立）量到 `GET /api/augment/progress` 的三处对得上：

1. 路由体调 `p.checkpoint_manager.get_progress()` 且**不传**断点对象；
2. `CheckpointManager.get_progress` 的 `None` 分支回落到 `self._current_checkpoint`
   （「当下谁的断点在跑」）；
3. 任务身份的权威是 `AugmentorPipeline.build_task_id`（按**解析后**的输入路径取
   md5 前 12 位）—— 而**客户端算不出**那个 id（它只知道送的文件名，不知道服务端
   解析后的路径）。

⇒ 回执里那格 `task_id` 可能是**最后加载断点的那一次运行**，而不是调用方问的那一次；
并发两个任务时，进度卡显示的是另一个任务的数。

`get_progress` 自己的 docstring 把「环境态读法」写成了明示口径，L275 给**写**侧配的
`_RunState` 归属只治「这次更新属于谁」、没治**读**侧 ⇒ 那一格属**「已声明但没接线」**
不是「藏着」。

## 口径：两条路都要有，但只读的那条不许有副作用

- **环境态读法**（不带 `task_id`）：「当下谁在跑」是个合法问题，**保持原样**（既有
  行为一格未动）。
- **点名读法**（带 `task_id`）：读**那一份**断点的进度 —— 这才是「我这次」的答案。

两条路都要有；但**只读面不能有副作用**，这是这一格的关键。

## 关键：现有的读函数是「消费型」的

`load_checkpoint` 不是只读面 —— 它做两件不可逆/可见的事：

1. **消费增量**：把增量合并结果落盘，然后**清掉增量文件**；
2. **改环境态**：把 `_current_checkpoint` 改指到这一份。

拿它当只读面用 ⇒「看一眼进度」会**把正在跑的增量吃掉**（下次恢复少一段进度）、
并**把别的调用方的指针搬走**（环境态读法读到的变成这一份）。

处置：把 `load_checkpoint` 拆成两层，再加一层只读入口。

```python
def _read_checkpoint_with_deltas(self, task_id) -> Tuple[Optional[CheckpointData], bool]:
    """纯读：主文件 + 内存里合并增量（不落盘、不清增量、不动环境态）"""

def load_checkpoint(self, task_id) -> Optional[CheckpointData]:
    """纯读 + 改环境态 + 落盘并清增量（消费型，行为一字未动）"""

def peek_checkpoint(self, task_id) -> Optional[CheckpointData]:
    """只读：纯读那一层，不改环境态、不清增量"""
```

**消费那一半必须钉住**：守卫里有一支专门断言 `load_checkpoint` 之后增量文件**消失**
—— 少了它，「把消费读改成只读读」会静悄悄地把断点续传的重复累加那一族缺陷放回来
（L100 量到过的那一形）。

## 写侧：身份得由服务端递回来

点名读法要有一个「任务 id」，而客户端算不出来 ⇒ `POST /api/augment/start` 的回执
**带上 `task_id`**（与 `augment_dataset` 内部同一个 `build_task_id` ⇒ 客户端拿它去问
能问到同一份）。

这一格是**加法不是改法**：新键对老客户端是看不见的。

## 404 与空态要分得开

- 问一个**不存在的任务** ⇒ **404**（「问的那个任务没有断点」）；
- 「当下没有任务在跑」⇒ 仍是既有的 `{"status": "no_checkpoint"}`（200）。

两件事不该用同一句话回答。

## 守卫

`augmentor/tests/unit/test_progress_task_binding_l308.py`（**7 支 / 零 parametrize**）：

- 只读性三支：增量在内存里合并但**仍在盘上** / 环境态指针**没被搬走** /
  消费那一半**仍会清增量**；
- HTTP 面四支：点名读法读回来的**必须是点名的那一份**（盘上放两份、环境态指 B、
  点名读 A ⇒ 回执必须是 A）/ 不带参数仍是环境态读法 / 问不存在的任务 ⇒ 404 /
  启动回执里的 `task_id` 与 `build_task_id` 算出来的一致。

**牙齿验证**（两臂注入，跑完按字节复原并复核 sha256 相等）：把路由退回「不点名」
⇒ **2 failed**；把只读面退回「消费型读」⇒ **1 failed**。

## 连带代价

1. **接线名册会动**：l275 那一族的 AST 现量里，路由的 `get_progress` 从 (1, 0) 走到
   **(2, 1)** —— 两发都在（环境态那一发不给身份、点名那一发给），而给了身份的那一发
   从 0 走到 1 ⇒ 这一族量到的正是「身份参有没有给到」。
2. **前端一致性名册要拆两格**：启动端点的响应模型换成了 `AugmentStartResponse`
   （`MessageResponse` + `task_id`），而 `DELETE /api/augment/checkpoints/{task_id}`
   仍用着 `MessageResponse` ⇒ 两个 zod 面必须分开（键集与必填性都被一致性守卫逐格对着
   Python 模型查）。
3. **前端调用的形状**：不点名时**逐字**调 `api.get('/augment/progress')` —— 多传一个
   `undefined` 也会让「按调用形状断言」的既有用例变红，而那是噪声。

## 教训

- **「只读」这个需求常常撞上一个「读」其实是「消费」的函数** —— 先问「这个读有没有
  副作用」，再决定能不能直接用；本仓里 `load_checkpoint` 就属于后者。
- **点名读法要有两个前提**：服务端能算出对象身份（或者把它递回来），且那条路是**纯读**的。
  只做一半（能点名但读的是环境态）等于给了一个假承诺。
- **404 与空态分开**：「问的那个没有」与「当下没有」是两种事实，用同一句话回答会让
  客户端无法区分。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L308 清单行与日志块、A408 行
- 守卫：`augmentor/tests/unit/test_progress_task_binding_l308.py`（7 支 / 零 parametrize）、
  `augmentor/tests/unit/test_run_scoped_checkpoint_state_l275.py`（接线名册那一族）
- 复用：`augmentor/augmentor/pipeline.py` 的 `build_task_id`（任务身份的权威）
- 同族：A402（请求侧「列表长度」那一维无约束）、A408 的兄弟格 A406/A407
