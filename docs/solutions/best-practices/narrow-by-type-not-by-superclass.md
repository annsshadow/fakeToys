---
title: "按类型放行，不按父类放行：域判决 400、具名窄口、其余 500"
date: 2026-11-11
last_refreshed: 2026-11-11
category: best-practices
module: augmentor/api/routes/quality.py + augmentor/api/deps.py
problem_type: a_domain_verdict_is_flattened_into_a_server_fault_by_a_catch_all_branch
component: narrow_by_type_not_by_superclass
severity: medium
symptoms:
  - "调用方把一个旋钮写坏了（NaN / 无穷 / 越界），端点回 500 + 「服务器内部错误」——而 SDK 明明给了一句点名那个旋钮的中文判决"
  - "同一个域判决在同一个文件的两个端点上落两种码：一处 400、一处 500"
  - "内部异常（裸 `ValueError`）被回成 400 判决，**异常原文原样发给客户端**"
  - "改一处状态码要同时改三处断言：守卫的现量表、注入用例的期望值、文档那一行"
root_cause: a_catch_all_branch_collapses_every_exception_into_one_code_and_never_looks_at_what_the_sdk_already_decided
resolution_type: narrow_per_file_by_named_types_through_one_shared_door
tags: [error-contract, http-status, boundary, narrow-port, domain-verdict, api-contract]
applies_when:
  - "路由用一条 `except Exception` 把下游异常一刀切收敛成 500"
  - "下游 SDK 抛的是**具名的**域判决异常（`DataValidationError` 家族或模块自己的异常）"
  - "同一个异常家族在别的端点已经按 400 处理，只有这一族被折成 500"
  - "错误体里回传的是 `str(exc)` 原文"
related_components:
  - augmentor/api/deps.py
  - augmentor/api/routes/quality.py
  - augmentor/exceptions.py
  - augmentor/validation.py
  - augmentor/tests/unit/test_quality_boundary_l313.py
  - augmentor/tests/unit/test_nonfinite_float_knobs_l272.py
---

# 按类型放行，不按父类放行：域判决 400、具名窄口、其余 500

## 一、形状：SDK 已经判过了，门口把它折掉

`augmentor/quality.py::QualityScorer.__init__` 调 `validation.require_ratio("quality.threshold", ...)`
⇒ 坏值会得到一句**点名旋钮、带实际值**的中文判决：

```python
raise DataValidationError(f"{name} 必须是有限数值，当前是 {'NaN' if math.isnan(value) else '无穷'}")
```

但 `api/routes/quality.py` 那一族的收尾是一条一刀切：

```python
except Exception as e:
    raise_internal_error(e)      # ⇒ 500 + 「服务器内部错误，详情见服务端日志」
```

⇒ **调用方自己写错了参数，服务端回一句「服务器内部错误」**。这不是无声 bug，是**判决被降级**
——SDK 那句可行动的判决原样躺在服务端日志里，客户端拿到的是一句没有主语的故障提示。

## 二、三条都错的中间路，与第三条路

| 放行口径 | 域判决（`DataValidationError` 家族） | 内部异常（裸 `ValueError`） | 结论 |
| --- | --- | --- | --- |
| 一刀切 500 | 500（客户端的错说成服务端的故障） | 500 + 固定文案 | **太宽**：把域判决也折掉 |
| `ValueError` 泛档 400 | 400 + 原文 | **400 + 内部原文外发** | **太宽**：内部细节出网关 |
| **按类型放行**（本轮取法） | 400 + 域判决原文 | 500 + 固定文案 | 两头都对 |

第三条路的落点是**共享的那把门口**（A77「判据住一处」），并且**按类型**放行、不按父类放行：

```python
def raise_boundary_error(exc, *, not_found=..., not_found_types=(),
                         boundary_types=()):        # ← 第二只窄口（400 档）
    if not_found_types and isinstance(exc, not_found_types):
        raise HTTPException(status_code=404, detail=not_found)
    if isinstance(exc, DataValidationError) or (
        boundary_types and isinstance(exc, boundary_types)
    ):
        raise HTTPException(status_code=400, detail=str(exc))
    raise_internal_error(exc)
```

**为什么要第二只窄口**：有的模块把域判决**折成了自己的异常**（`augmentor/dedup.py` 把
`require_ratio` 那句判决折成 `DedupError`）⇒ 按家族放行接不住它 ⇒ 只能按**具名类型**点名放行。
窄口住在共享门口上 ⇒ 调用点只写「这一支的域异常是哪个」，判据本身不复制。

## 三、三条纪律

1. **逐文件收窄，不一刀切改八个文件**。状态码是对外契约 ⇒ 一次只动一族，改完同批把**钉着旧
   行为**的守卫改准（本轮：现量表两行由 500 翻成 400、注入用例由「内部异常 ⇒ 400 + 原文」
   翻成「内部异常 ⇒ 500 + 固定文案」）。
2. **同一族里不许有两种放行规则**。本轮那一族原本就有两条 `except ValueError` 泛档（⇒ 内部
   异常原文被回成 400 判决）⇒ 与其余七只端点的规则不一致 ⇒ 一并摘掉，让域判决改由一把门口出。
3. **只动本来就该动的那一格**：原本就 400 的格子（方法名不认、字段不存在、空数据集…）一格未动；
   缺文件仍 404；合法请求仍 200。

## 四、守卫怎么写才有牙

- **名册面**（AST）：那个文件里「一刀切」调用点数 = 0、共享门口调用点数 = N、泛档臂数 = 0 ——
  形状退回去时当场红。
- **判决面**：域判决逐格点名（状态码 + 判决文本里那个旋钮名）。
- **收窄面**：注入一个内部异常 ⇒ 必须 500 + 固定文案，且**响应里不含内部原文**。
- **反向臂**：把窄口摘掉（换成忽略 `boundary_types` 的那把门口）⇒ 那一族立刻回到 500
  ⇒ **窄口是承重的**，不是「表里写了 400 所以永远绿」。

## 五、代价照实记

- **状态码是对外契约**：改一族就是改一族，客户端能看见 ⇒ 这一格必须同批改文档与钉着它的守卫。
- **有的判决天生不点名**：去重那一支的 400 detail 是 SDK 既成契约那句「阈值必须在 0-1 之间」
  （不点名字段名、也不回显实际值）⇒ 改它要动 SDK 的公开契约 ⇒ 本轮不改，只照实记。
- **同族若有别处照着旧形状写的文档/用例，一并翻**（本轮翻了 l272 那张现量表与一支注入用例）。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L313 清单行与日志块；A280 行的 `S 半修（L313）`
- 守卫：`augmentor/tests/unit/test_quality_boundary_l313.py`（零 parametrize，17 支）
- 同族：`docs/solutions/best-practices/verdict-narrowing-and-failure-honesty.md`、
  `docs/solutions/best-practices/input-validation-pattern.md`
