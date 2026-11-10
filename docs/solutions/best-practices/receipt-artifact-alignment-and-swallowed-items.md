---
title: "回执与产物对齐：改回执不改产物；以及「被吃掉的东西」要在回执里被点名（A403 / A401 / L307）"
date: 2026-11-10
last_refreshed: 2026-11-10
category: best-practices
module: augmentor/augmentor/dataset_ops.py + augmentor/augmentor/data/multimodal.py + augmentor/api/routes/multimodal.py
problem_type: receipt_reports_a_different_fact_than_the_artifact
component: receipt_artifact_alignment
severity: medium
symptoms:
  - "merge_files(config=None) 的回执说「没去重」，而产物按 MergeConfig() 默认去重了（五条进、产物一条出）"
  - "多模态扫描目录里 4 只文件 + 1 个子目录 ⇒ 只出 3 条记录，而回执里 0 条告警"
  - "同名同模态的后一只静默覆盖前一只；子目录整只静默跳过"
root_cause: receipt_counts_a_different_thing_than_the_artifact_produces
resolution_type: align_receipt_with_artifact_and_surface_swallowed_items
tags: [receipt, artifact, truthfulness, dedup-count, warning-surface, name-anchor, data-loss, api-contract]
applies_when:
  - "同一个动作的『回执计数』与『产物事实』在某个参数档位下会分家"
  - "扫描 / 归并类操作会静默丢掉输入项（同名冲突、递归跳过），而回执没有这一维"
  - "改回执口径与改产物行为是两件事，后者的代价大得多"
related_components:
  - augmentor/augmentor/dataset_ops.py
  - augmentor/augmentor/data/multimodal.py
  - augmentor/api/routes/multimodal.py
  - augmentor/tests/unit/test_scan_receipt_facts_l307.py
  - augmentor/tests/unit/test_merge_truncation_attribution_l291.py
---

# 回执与产物对齐：改回执不改产物；以及「被吃掉的东西」要在回执里被点名

## 现象

两格形状不同、根子相近：**回执在报一件与产物不同的事**。

### A403：合并回执说「没去重」，产物去了重

`merge_files(..., config=None)` 的产物侧走 `_merge_and_count` 里的 `config or MergeConfig()`
⇒ 按默认去重；而回执那一格刻意写成 `removed_duplicates = dedup_removed if config is not None else 0`
⇒ **不传配置就不报去重数**。

实测：5 条全同的输入 ⇒ 产物 1 条，回执 `{total_input: 5, total_output: 1, removed_duplicates: 0}`
—— 读者读回执得到「一条都没去掉」，读产物得到「去掉了 4 条」。**同一件事两种读法**。

### A401：扫描回执里 0 条告警，而输入少了两只

`multimodal.process_directory` 按 `file_path.stem` 分组、槽位是模态 ⇒

- 同名**同模态**（`sample.jpg` 与 `sample.png`）落进同一个槽位，`sorted` 序**后者胜**
  ⇒ 前一只**静默消失**；
- **子目录整只跳过**（只扫顶层）⇒ `nested/` 里的媒体一只都不处理。

docstring 承诺的是「同名文件（如 sample.jpg + sample.wav）会被融合为一条记录」——那是**跨模态**融合，
是**特性**；**同模态**的互相覆盖不在那句承诺里。而这两件事在回执里各占 0 条 ⇒ 读者以为扫了
4 只、实际只处理 3 只。

## 口径

**A403：改回执，不改产物。**

产物侧去重是既有行为（`config or MergeConfig()`），改它属**更重的行为变更**（会改交付内容）；
回执报真数则与产物、与 HTTP / CLI 两个产品面**同口径**。本仓的纪律是「产品面零影响优先」：
`POST /api/dataset/merge` 与 `cli merge` **都显式传 `MergeConfig`** ⇒ 它们的读数一格未动，
只有 SDK 直调 `merge_files(..., None)` 这一条路的读数变了 —— 而那条路读到的本来就是
「产物真去了多少」。

**A401：加告警面，不改处理行为。**

只扫顶层、`sorted` 序后者胜，都是既有语义；改它们会把这一格从「回执缺一维事实」变成
「扫描语义变更」（那是另一族）。所以改的是**回执**：把「被吃掉的东西」逐条点名。

## 三条设计要点

1. **同名跨模态不算告警。** 那是 docstring 承诺的特性；把它算成告警就是把特性当缺陷。
   判据落在**槽位**上：`image` 与 `audio` 是两个槽，不冲突；同槽才是覆盖。
2. **不支持的扩展名不算告警。** 那是正常过滤，不是「吃掉东西」。子目录与同模态覆盖才是
   —— 前者可能装着媒体，后者是数据丢失。
3. **告警清单不截断。** `errors` 那一格硬截 20 且无截断旗是 **A402** 那一族（另一格，本轮未动）；
   这一批的规模由「目录里有多少只文件」决定，而同一条响应的 `records` 本来就是全量回传 ⇒
   两边口径一致，不在本轮另造一个同族缺陷。

## 守卫

`augmentor/tests/unit/test_scan_receipt_facts_l307.py`（**11 支 / 零 parametrize**）：

- **A403 五支**：`config=None` 报真数 / **独立复算**（重读输入文件与产物文件算一遍真数再跟
  回执比，**不拿产品自己报的数当依据**）/ 截断那一格恒 0 / 显式 `MergeConfig` 那一路同读数 /
  **接线普查**（AST 咬住 HTTP 路由与 CLI 两处 `merge_files` 的 config 实参是 `MergeConfig(...)`
  —— 哪天有人改成传 `None`，那一路的读数就与另一路分家）；
- **A401 六支**：被覆盖的那一对被点名 / 被跳过的子目录被点名 / 跨模态同名不算告警 /
  **承重臂**（`generate_report(records, notes=[])` 必须给 0 条 ⇒ 告警来自扫描而不是回执自造）/
  告警逐次扫描重置（第二次扫干净目录 ⇒ 0 条）/ **HTTP 面**那一支（响应带 `warning_count` /
  `warnings` 且点名了被覆盖的那一只）。

**牙齿验证**（两臂注入，跑完按字节复原并复核 sha256 相等）：

- 把 `removed_duplicates` 退回「`config=None` 记 0」⇒ **2 failed**（点名
  `test_config_none_reports_the_true_removed_count` 与
  `test_the_receipt_matches_the_artifacts_independently`）；
- 把同名同模态的覆盖告警摘掉 ⇒ **2 failed**（点名 `test_the_overwritten_file_is_named` 与
  `test_the_http_face_carries_the_warning_face`）。

## 连带代价：改回执会顶到别的守卫

A403 改口径后，`tests/unit/test_merge_truncation_attribution_l291.py` 里两支要跟：

1. `test_the_formula_still_cites_the_ceiling` 按「赋值号右边含 `max_items is None`」判牙齿 ⇒
   **公式那一行必须保持单行**（拆成多行那一支就读不到判决）。本轮为此把公式还原成单行，
   只让 `removed_duplicates` 那一格变。
2. `test_config_none_reports_no_truncation` 原来断言 `removed_duplicates == 0`（旧口径）⇒
   改成真数。**这一支的形状也变了**：去重数成真数后，`config=None` 档的残差恒为 0 ⇒
   「残差撞截断」这一族缺陷在该档**不再能复现** ⇒ 该档的牙齿迁到「公式形状」与「带 `max_items`
   的各档独立复算」两支。本轮把这一变化**写在守卫的 docstring 里**，不假装它还兼着那一维。

## 行号位移：内容守恒不够，还得对内容

产品码在三只文件插行 ⇒ 按**内容守恒**（`difflib` 等块逐行对位、逐行断言旧行内容 == 新行内容）
重钉，**9 行 / 2 个文件**。两类漏网：

1. **裸文件名形状**（`dataset_ops.py:NNN` 不带 `augmentor/` 前缀）**不在那套脚本的匹配面内**
   —— 本轮补了一次全仓核对（三个改动文件的全路径 + 裸文件名两种形状，逐格判「落点是不是
   代码行」），补钉 4 处。
2. **指向了别的代码的引用**（内容守恒只会把它搬到「同样错的地方」）：本轮量到 A292 / A296 两行
   的引用**在 HEAD 上就指向了错的代码**（例如「`elif config.ratio is not None:`」那一格指着
   `step = max(...)` 那一行）。这类要**按引用内容校正**：找到那一行代码的现量行号再钉上去。

账本里的**历史读数**（旧行号）按 l79 注释里那族「降级名锚」处置：事实与链条留着、行号不钉
—— 写行号断言会被后续插行顶成死引用，本轮补洞时那几处正是被这一族咬住的。

## 余面与教训

- **「回执少一维」与「行为有 bug」是两件事**：A401 的处理行为（只扫顶层、后者胜）本身不算错，
  错的是**回执没把这两件事说出来**。先分清是哪一族，再决定改哪一层。
- **回执口径优先于产物口径**：改回执是「让账对上事实」，改产物是「换事实」—— 后者要先拍。
- **新增回执键要过三侧**：后端 Pydantic 模型、OpenAPI 契约测试的键名册、前端 TS 类型 + zod
  schema + 页面渲染（本轮还补了一条前端用例点名两条告警都被渲染）。前端用 zod 的
  `z.object`（非 strict）⇒ 后端多出的键不会让前端解析失败，但**仍要显式补上**，否则前端
  拿不到那一维。
- **文档示例会骗人**：`docs/augmentor/API.md` 那个端点的示例里写着一个根本不存在的键
  （`modality_distribution`）⇒ 本轮顺带改成真实键名。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L307 清单行与日志块、A401 / A403 行
- 守卫：`augmentor/tests/unit/test_scan_receipt_facts_l307.py`（11 支 / 零 parametrize）
- 复用：`augmentor/tests/unit/test_merge_truncation_attribution_l291.py`（归因那一族的既有口径）、
  `augmentor/tests/unit/test_sample_aggregate_rag_wiring_l230.py`（接线普查那一族的形状）
- 同族：A402（请求侧「列表长度」那一维无约束 + `errors` 硬截 20 无截断旗）、
  A400（截断与去重分报那一格）、A408（进度回执没绑任务身份）
