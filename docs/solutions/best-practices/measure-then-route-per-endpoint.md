---
title: "逐处先实测再决定放行口径：收窄 8 格、保持 10 格，同一把门口按类型分档"
date: 2026-11-11
last_refreshed: 2026-11-11
category: best-practices
module: augmentor/api/routes/data.py + augmentor/api/routes/augment.py + augmentor/api/routes/export.py + augmentor/api/routes/multimodal.py + augmentor/api/deps.py
problem_type: a_catch_all_branch_decides_a_status_code_before_measuring_what_it_catches
component: measure_then_route_per_endpoint
severity: medium
symptoms:
  - "客户端指了一份坏文件（不是合法 JSON / 顶层不是数组），端点回 500 + 「服务器内部错误」——而 SDK 明明给了一句可行动的判决"
  - "客户端把一个任务 id 写成了盘符相对形状（`C:evil`），端点回 500；同一族的另一处已经回 400"
  - "同一个错误在同一个文件里落两种码：预览那头 400、导出那头 500"
  - "内部的 `ValueError` 被回成 400 判决，**异常原文原样发给客户端**"
root_cause: the_catch_all_branch_is_written_before_anyone_asks_what_each_endpoint_can_actually_raise
resolution_type: measure_each_endpoint_then_route_by_named_type_and_keep_the_rest_at_500
tags: [error-contract, http-status, boundary, per-endpoint, measurement-first, api-contract]
applies_when:
  - "一个路由文件里有若干条 `except Exception` 收尾分支，它们现在是同一个形状"
  - "要把一刀切的 500 改掉，但不清楚每个端点下游到底会抛什么"
  - "状态码是对外契约，改一格就要同批改钉着它的守卫"
related_components:
  - augmentor/api/deps.py
  - augmentor/api/routes/data.py
  - augmentor/api/routes/augment.py
  - augmentor/api/routes/export.py
  - augmentor/api/routes/multimodal.py
  - augmentor/tests/unit/test_boundary_narrowing_l314.py
---

# 逐处先实测再决定放行口径：收窄 8 格、保持 10 格，同一把门口按类型分档

## 一、「一刀切改八个文件」是错的问法

A280 那一族的正确修法是**逐文件收窄**，但逐文件收窄之后还有第二个问题：**同一族里每一处
的下游抛的东西并不一样**。把一个文件里的收尾分支**整族**换成同一把门口，等于用一刀切
换掉一刀切。

L314 那一批把 18 处一刀切 500 逐处量了一遍，结论是：

| 文件 | 收窄 | 保持 500（理由） |
| --- | --- | --- |
| `api/routes/data.py` | 3 | 2（域判决已被 `HTTPException` 接住；写边 `OSError` 是服务端状态） |
| `api/routes/augment.py` | 2 | 2（域判决在 `HTTPException` 里；有一处没有客户端入参） |
| `api/routes/config.py` | 0 | 3（域判决那一档已在另一支接住；其余是服务端配置坏了） |
| `api/routes/export.py` | 2 | 1（能力清单，没有客户端入参） |
| `api/routes/multimodal.py` | 1 | 1（域判决在 `HTTPException` 里） |
| `api/routes/privacy.py` | 0 | 1（SDK 不抛域异常） |

⇒ **收窄 8 格、保持 500 十格**。保持的那十格不是「漏了」，而是**量出来的**：那一支的下游
根本没有域判决，把它改成 400 只会在「本来该 500 的内部故障」上多开一个 400 的口子。

## 二、三条纪律

1. **先实测，后改码**。每个端点先问一句「下游到底抛什么」，再决定放行口径。判据是**实测**
   （探针逐格），不是「看起来像参数错」。
2. **保持 500 的格子也要写进账本**。理由不是「没来得及改」，而是「量过了，那一支没有域判决」
   ⇒ 否则下一轮会把它当成漏网之鱼再改一遍。
3. **改状态码就要同批改钉着它的守卫**。A280 那一族里有一支守卫的字面就写着「**谁去收 A280，
   这一支要同批改**」⇒ 收口那一轮把它从「500 + 固定文案」的现量钉翻成**正判决**（400 + 那句
   可行动的判决），并把「文案不泄漏内部细节」那半句留在原处。

## 三、坏 JSON 那一档要**共源**，不是各写一句

客户端那份文件不是合法 JSON 时，**两把门口**（`to_http_error` 与 `raise_boundary_error`）
都要给出同一句判决 ⇒ 文案抽成一只 helper、两处共用：

```python
def _json_decode_detail(exc: json.JSONDecodeError) -> str:
    return f"数据文件不是合法 JSON（第 {exc.lineno} 行第 {exc.colno} 列）"
```

不共源的代价是**同一个错在两条路上有两句话** ⇒ 客户端没法用一句话认领这一类错。

## 四、摘掉泛档的副作用要一起量

`except ValueError → 400 + str(exc)` 这条泛档有两个坏处：它把**内部** `ValueError` 也回成
400 判决、且**把内部原文发出去**（A386 那一族）。摘掉它之前先量一遍：**哪些格原来是靠这条泛档
拿到 400 的**？L314 那一次摘的是多模态扫描那一支：目录不存在（`DataLoadError`）与坏图片
（`DataFormatError`）原本靠泛档拿 400 ⇒ 改成**具名窄口**（`boundary_types=(DataLoadError,
DataFormatError)`）后**那两格仍是 400、同一句判决**，而其余异常落回 500 + 固定文案。

## 五、还剩什么（照实记）

- **`GET /api/analyze/{filename}` 给一份顶层不是数组的文件仍 500**：根因在 SDK
  （`analyze_dataset` 直接 `json.load`，那一层**没有形状闸**）⇒ 门口无从分辨「客户端的坏文件」
  与「服务端内部错」⇒ 要在 SDK 那一层补形状闸（另一轮）。
- **`POST /api/data/export` 给一个不支持的格式仍 500**：孢子抛的是**裸** `ValueError` ⇒ 按类型
  放行接不住它；把它包成域判决要动 SDK 的公开契约（五支既有守卫钉着）⇒ 另一轮。
  ⇒ 这一族因此仍是「同一类输入两种码」：预览那一支 400、导出这一支 500。

## 六、守卫怎么写才有牙

- **名册面**（AST）：逐文件点名一刀切调用点数与边界口调用点数 ⇒ 收窄那一族「18 → 10」是**现量**
  而不是许愿。
- **判决面**：翻面的格子逐格点名（400 + 那句判决），**没翻的格子也逐格点名**（目录不存在仍 400、
  缺文件仍 404、合法请求仍 200）⇒ 收窄不许偷跑到别的格上。
- **收窄面**：注入一个内部异常 ⇒ 必须 500 + 固定文案，且**响应里不含内部原文**。
- **反向臂**：把具名窄口摘掉 ⇒ 那一族回到 500 ⇒ 窄口是承重的。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L314 清单行与日志块；A280 行的 `S 半修（L314）`
- 守卫：`augmentor/tests/unit/test_boundary_narrowing_l314.py`（零 parametrize，17 支）
- 同族：`docs/solutions/best-practices/narrow-by-type-not-by-superclass.md`、
  `docs/solutions/best-practices/verdict-narrowing-and-failure-honesty.md`
