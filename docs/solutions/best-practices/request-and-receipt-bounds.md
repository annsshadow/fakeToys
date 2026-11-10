---
title: "长度要有上限、截断要点名：请求侧与回执侧的两类资源闸（A402 / L309）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: best-practices
module: augmentor/api/routes/dataset_tools.py + augmentor/api/routes/multimodal.py + augmentor/augmentor/data/multimodal.py
problem_type: unbounded_request_dimension_and_silent_truncation
component: request_and_receipt_bounds
severity: medium
symptoms:
  - "一次请求可以点名任意多个输入文件，而每一只都要读进内存（请求体小、内存占用大）"
  - "扫描端点把整份 records 塞回响应，响应体随目录线性膨胀"
  - "errors 硬截 20 却没有任何旗 ⇒ 「全给了」与「被截了」两种读法在回执里不可分"
root_cause: untrusted_input_dimensions_are_unbounded_and_truncation_is_silent
resolution_type: factory_default_length_caps_with_rejection_and_truncation_flags
tags: [request-bounds, resource-cap, reject-not-truncate, truncation-flag, receipt, api-contract, untrusted-input]
applies_when:
  - "请求体只带路径、但服务端要把每一项读进内存（未受信输入的放大面）"
  - "响应体按输入规模线性增长（列表回传型端点）"
  - "回执里的某个列表被截断了，但客户端看不出自己拿到的是不是全部"
related_components:
  - augmentor/api/routes/dataset_tools.py
  - augmentor/api/routes/multimodal.py
  - augmentor/augmentor/data/multimodal.py
  - augmentor/tests/unit/test_request_and_receipt_bounds_l309.py
  - augmentor/backup.py
---

# 长度要有上限、截断要点名：请求侧与回执侧的两类资源闸

## 现象

A402（L291 立）量到三件同族但形状不同的事：

1. **请求侧「列表长度」那一维全族没有约束**：`MergeRequest.inputs` 与
   `AggregateRequest.datasets` 的字段 `metadata` 现量各 0 条 ⇒ 一次请求可以点名
   任意多个输入文件，而每一只都要读进内存（合并 / 聚合的产物与输入同阶）⇒
   **请求体很小、内存占用很大**。
2. **`POST /api/multimodal/scan` 把整份 `records` 塞回响应** ⇒ 响应体随目录线性膨胀。
3. **`generate_report` 的 `errors` 硬截 20 而没有截断旗** ⇒ 25 条错误时回执是
   `error_count: 25 / len(errors): 20`，「全给了」与「被截了」两种读法不可分。

## 口径：三格各按自己的形状

### 请求侧长度 ⇒ 出厂上限 + **拒**（不截断）

- **上限从哪来** = **出厂默认**（100），不是配置键、也不是请求参数。理由：这一维是
  **未受信输入**的闸 —— 请求体只带路径、内容在盘上、产物与输入同阶；客户端自己
  抬高上限就等于绕过闸门。配置键那一族留给「内容量」那类需要按部署调的维度。
- **超了是拒还是截** = **拒**（400）。理由：这一族的交付物是**数据文件本身**，截断
  会静默改变交付的内容（正是「我送了 200 个、你只处理了 100 个」那种静默）。
  与仓库既有约定一致（请求面用 400 而不是 FastAPI 的 422，见 `dataset_tools.py` 的
  同名注释）。
- 请求面**不接受覆盖**这个上限。

### 响应侧记录 ⇒ 封顶 + `records_truncated` 旗

`total_records` 仍是真数 ⇒ 客户端能自己看出「回执只给了前 N 条」；截断**不静默**。

### 错误清单 ⇒ 补 `errors_truncated` 旗

`errors` 本来就截 20，缺的是「被截了没有」这一维。旗为假时 `error_count` 就等于
`len(errors)`。

## 一条容易误判的事实

探针里那句「同一只文件点名两次 ⇒ 交付字节翻倍」**不算缺陷**：去重开着时产物本来
就该去重（默认档），`deduplicate=False` 那一档是调用方明确要「不去重」⇒ 那是语义
选择不是静默行为。本轮的长度上限把「同一只点 1000 次」那一档一并挡住了。

## 守卫

`augmentor/tests/unit/test_request_and_receipt_bounds_l309.py`（**7 支 / 零 parametrize**）：

- 请求侧三支：超限 ⇒ 400 且 `detail` 里出现上限与实数 / **盘面一格未动**（被拒的请求
  不许落产物）/ 上限之内照旧 200 ⇒ 上限不是「把端点关掉」；
- 截断旗四支：`errors` 25 条 ⇒ 只给 20 条且旗为真 / 错误不多 ⇒ 旗为假且
  `error_count == len(errors)`（旗不能与真数脱节）/ 逐条记录封顶 ⇒ 旗为真且
  `len(records) < total_records` / 记录不多 ⇒ 旗为假且一条不少。

**牙齿验证**（三臂注入，跑完按字节复原并复核 sha256 相等）：摘掉请求侧长度闸 ⇒
**1 failed**；把 `errors_truncated` 写死为假 ⇒ **1 failed**；摘掉逐条记录的封顶 ⇒
**1 failed**。

## 连带代价（这类改动比看起来贵）

1. **前端三侧**：TS 类型、zod schema 各多两格旗；**页面还要给一条说明** ——
   否则「表里只有 20 行」会被读成「目录里只有 20 只文件」（截断旗只在数据里，
   用户看不见）。前端用例的夹具也要补那两格旗。
2. **后端契约名册**：OpenAPI 契约测试逐条点名每个端点的响应键 ⇒ 两格新键要进名册。
3. **文档**：三个端点各补一句（上限与 400 档 / 两格旗）。
4. **一致性名册**：`full-schemas.ts` 的覆盖面现量（模型数、字段数）按现量重钉。

## 教训：两套重钉不能对同一批引用先后都跑

本轮踩到一处**次序陷阱**：内容守恒重钉（按 **HEAD 的行号**做内容映射）与按内容校正
（按**类名找现量行**）对同一批引用**会叠加** —— 实测 `full-schemas.ts` 的 16 处先按
内容校正、再被内容守恒重钉一遍，**每一处多偏 10 行**。

处置：**按内容校正那一路最后跑**，并用一条独立核对（逐条比对注释里的行号与类定义的
现量）收口 ⇒ 余 0 处不一致。

另一条同族的：**手钉的行号必须是最后一步**。本轮 A408 那一行的手钉（`augment.py:256`）
在之后又被一次插行顶掉了 3 行 ⇒ 补钉回 `:259`。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L309 清单行与日志块、A402 行
- 守卫：`augmentor/tests/unit/test_request_and_receipt_bounds_l309.py`（7 支 / 零 parametrize）
- 复用：`augmentor/backup.py` 的 `max_backups`（保留份数那一族的既有形状）、
  `augmentor/api/routes/dataset_tools.py` 里「请求面用 400 而不是 422」的既有约定
- 同族：A409 / A399（产物与账的封顶）、A401 / A403（回执与产物对齐）
