---
title: "对外判决面的三处收窄：窄口要同形、失败不许冒充成功、issues 为空不等于没问题"
date: 2026-11-11
last_refreshed: 2026-11-11
category: best-practices
module: augmentor/api/deps.py + augmentor/api/routes/version.py + augmentor/api/routes/export.py + augmentor/augmentor/export.py + augmentor/augmentor/dependency.py
problem_type: client_receives_a_verdict_that_does_not_match_the_fact
component: verdict_narrowing_and_failure_honesty
severity: medium
symptoms:
  - "同一族路由里三种归宿并存：源文件缺失 404、版本 id 缺失 500、删不存在的版本 200 success:true"
  - "导出的默认并行支路把写边拦下的判决咽成空 dict，HTTP 面照样回 success: true"
  - "依赖图只报悬空端点，成环时 issues 为空，页面把 0 读成「依赖关系没问题」"
root_cause: a_single_funnel_decides_every_exception_and_the_empty_result_is_indistinguishable_from_success
resolution_type: narrow_per_file_not_one_cut + raise_instead_of_swallow + report_cycles
tags: [verdict, status-code, narrow-allowlist, single-authority, failure-honesty, swallowed-exception, cycle-detection, api-contract]
applies_when:
  - "一个异常类型在不同调用点上语义不同（不存在 / 不可删除 / 别的）"
  - "并行或批量分支把子任务异常吞掉，回执只剩空结果"
  - "一张校验清单只覆盖了部分失败形态，『零问题』被读成『没问题』"
related_components:
  - augmentor/api/deps.py
  - augmentor/api/routes/version.py
  - augmentor/api/routes/export.py
  - augmentor/augmentor/export.py
  - augmentor/augmentor/dependency.py
  - augmentor/tests/unit/test_verdict_and_dependency_cycles_l311.py
  - augmentor/tests/unit/test_dataset_text_field_l274.py
---

# 对外判决面的三处收窄：窄口要同形、失败不许冒充成功、issues 为空不等于没问题

## 一、逐文件收窄，别一刀切改八个文件

一个路由族把下游异常一刀切收敛成 500 时，「资源不存在」也拿不到 404。正解不是把放行名单整体放宽
（那会把一批域错误一并放进 400），而是**逐文件**把**语义明确**的那一类接进一把窄口：

```python
# api/deps.py —— 与 to_http_error 的同名参数逐格同形（A77：判据住一处）
def raise_boundary_error(exc, *, not_found="资源不存在", not_found_types=()):
    if not_found_types and isinstance(exc, not_found_types):
        raise HTTPException(status_code=404, detail=not_found)
    if isinstance(exc, DataValidationError):
        raise HTTPException(status_code=400, detail=str(exc))
    raise_internal_error(exc)
```

三条纪律：

1. **窄口与既有的那把门口同形**（同名参数、同样的判定顺序、同样的固定文案）⇒ 读代码的人不必在
   两把门口之间做翻译。
2. **文案固定、不回显调用方给的名字** —— 「版本不存在」比「版本不存在: v_ghost」少一层信息，但
   不把调用方给的字符串原样搬进错误体（既有那一族的纪律）。
3. **同一个异常类型在不同调用点上语义不同 ⇒ 逐个端点决定进不进窄口**。本轮就撞到这一格：
   `VersionError` 在版本详情 / 版本数据 / 版本对比三处的唯一抛出点是「版本不存在」⇒ 进窄口；
   而在**删除**那一处它的语义是「不能删除当前版本」⇒ **不进**窄口。
   **收窄不是放宽**：其余域错误一律仍落 500。

## 二、失败不许冒充成功

```python
except Exception as e:
    logger.error(f"批量导出失败: {e}")     # ← 判决被咽在这里
```

写边（`require_within_dir`）**确实拦住了**，回执却看不见那次拦截：调用方拿到 `{}` 而 HTTP 面照样回
`success: true`。**危害面是假回执而不是越界写**——调用方据此「重试 / 不重试」的决策全部反过来。

改法取「整批 raise」那一半（另一半「把失败收进返回值」会改公开键集）：

```python
failures = []
for future in as_completed(futures):
    try:
        ...
    except Exception as e:
        logger.error(f"批量导出失败: {e}")
        failures.append(e)
if failures:
    raise failures[0]
```

路由那一层随之**收窄**（`raise_internal_error` → `raise_boundary_error`）：写边抛的是
`DataValidationError`（名字被当成路径片段用 ⇒ **请求方的错**）⇒ 落 400，而不是一刀切的 500。

**代价照实记**：整批 raise 之前**已经落盘**的那几只产物留在盘上，而「哪个格式失败」只在那一条异常的
文案里。半程方案（回执多一格 `error`）信息更全，但要改公开键集 —— 两档各有所长，**按对外契约面挑**。

## 三、issues 为空不等于没问题

一张校验清单只覆盖了部分失败形态时，**「零问题」会被读成「没问题」**。本轮那一格：依赖图只报悬空端点
（`missing_source` / `missing_target`），成环时 `issues` 为空 ⇒ 而一个环会让下游任何按依赖顺序重建的
流程死循环。

加**DFS 染色**（白 / 灰 / 黑）：撞到**灰**节点即是一条回边，那一段栈就是环；同一个环按环内节点集合
去重只报一条。判决点名环上的**登记 id**（不是登记名）。

> 判据是**可证伪**的：不成环时零新增（别把「有边」当成「有环」），悬空那两档一字未动，
> issue 的三键形状（`type` / `dependency` / `message`）不变 ⇒ 前端键集不用动，只是取值域多一档。

## 连带代价（这类改动比看起来贵）

1. **既有守卫会钉住旧行为**：本轮同批翻了三支 —— 集成用例里「应返回 500」的两支、l274 那支「判据维
   钉死（`VersionError` 仍是 500）」的守卫、以及导出那一族「失败冒充成功」的现量守卫。**同批改**，
   否则守卫会把新行为判成回归。
2. **前端与文档各有一处口径**：卡片上那句「只报悬空端点、不查环」与 TS 类型的取值域注释都要跟着改，
   否则用户看到的仍是旧承诺。
3. **行号位移**：四只产品文件插行 ⇒ 全仓指向它们的 `file:行号` 引用全部位移（本轮 7 处）⇒ 按**内容
   守恒**逐条重钉，并复验 a184 的 `dead_line` 硬 0 档回到原值。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L311 清单行与日志块；A310 / A340 / A360 行的 L311 收口
- 守卫：`augmentor/tests/unit/test_verdict_and_dependency_cycles_l311.py`（零 parametrize）
- 同族：`docs/solutions/best-practices/receipt-artifact-alignment-and-swallowed-items.md`、
  `docs/solutions/best-practices/read-endpoint-artifact-boundary-and-receipt-redaction.md`
