---
title: "名字的形状不许静默变形：盘符相对、拼接口径、判决档位与键名语义"
date: 2026-11-11
last_refreshed: 2026-11-11
category: best-practices
module: augmentor/augmentor/validation.py + augmentor/api/routes/system_ops.py + augmentor/augmentor/visualizer.py + docs/augmentor/API.md
problem_type: a_name_is_reinterpreted_or_a_contract_word_is_imprecise
component: name_shape_and_contract_precision
severity: medium
symptoms:
  - "调用方给的 id 带冒号（`C:foo` / `a:b`）时，落盘名的形状被悄悄改掉：回执里的名字再拿去 find，永远找不到"
  - "无请求体的端点上，框架仍自动挂一档 422，而这一档永远不来（只吃契约的客户端为它生成一条死分支）"
  - "同族三只 helper 里两返回解析之后的落点、一返回拼接路径，读代码的人得自己猜哪个是哪一种"
  - "`is_valid` 与「问题清单条数」不互补：`warnings` 也在清单里，客户端以为「无 error」就等于「无问题」"
  - "键名 `unique_instructions` 数的是所选字段的唯一值数，不是指令数"
root_cause: a_name_or_a_contract_key_is_written_once_and_never_re_checked_against_what_the_code_actually_does
resolution_type: reject_the_mutating_shape_at_the_authority + write_the_contract_shape_into_doc_and_openapi
tags: [input-validation, path-shape, drive-letter, openapi, contract-precision, api-contract, doc-accuracy]
applies_when:
  - "调用方给的名字会被拼进路径，而某个平台对某些字符有特殊解释"
  - "同一个响应里多个键来自不同的口径（拼好的 / 解析后的 / 数值不同源）"
  - "判决字段与诊断字段都在一张清单里，客户端按前者判断"
  - "响应键名是既成契约，但实现数的其实是另一个量"
related_components:
  - augmentor/augmentor/validation.py
  - augmentor/api/routes/system_ops.py
  - augmentor/augmentor/visualizer.py
  - docs/augmentor/API.md
  - augmentor/tests/unit/test_colon_shape_l312.py
  - augmentor/tests/unit/test_dir_self_landing_l286.py
---

# 名字的形状不许静默变形：盘符相对、拼接口径、判决档位与键名语义

## 一、形状会在权威那一层被悄悄改掉

`pathlib` 对带冒号的名字有平台相关的解释：`C:foo` 被解析成**盘符相对**的形状。于是

```python
root / "C:foo"      # ⇒ C:\base\foo（基座被丢掉，落点名变形）
root / "a:b"        # ⇒ a:\base\b（基座整个丢掉）
```

判据不在调用点上，而在**拼路径的那把权威**（`require_within_dir`）：

```python
candidate = Path(value)
if candidate.drive:                      # 名字里带冒号 ⇒ 落点名会变形
    raise DataValidationError(
        f"{label} 不能含冒号（`C:` 会被当成盘符，落点名与回执里的 id 会不是一个东西）"
    )
```

三条纪律：

1. **拒的判据是「落点名会不会变形」，不是「名字里有没有某个字符」**。Windows 的盘符只认
   **单字母 + 冒号**，`foo:bar` 的 drive 是空的、落点名不变形 ⇒ **放行**。判据越界就会把一批
   合法名字也拒掉。
2. **文案点名后果，不点名解析后的绝对落点**（A155① 那一族的纪律）：文案里带绝对路径等于把
   服务器布局外发。
3. **同一族里更精确的臂优先**：`C:` 既是「冒号形状」也是「自着陆形状」（落点就是基座本身），
   两条臂都能拒它，但**冒号臂更精确**（它点出「名字里的冒号被当成盘符」）⇒ 归到冒号臂，
   自着陆那一臂只留它本该管的 `.` / `./` 几档。

> 顺带一条平台事实：Windows 上带冒号的文件名本来就落不了盘（NTFS 用 `:` 当流分隔符）⇒
> 这不是新的对外承诺。

## 二、契约上永远不来的那一档，要显式覆盖掉

FastAPI 会在**有请求体参数**的端点上自动挂一档 422。对 list / DELETE / verify 这三只
**无请求体**的备份端点，框架照样挂——而任何值都合法 ⇒ **这一档永远不来**。

只吃契约的客户端（生成器 / SDK）会为它生成一条**死分支**。改法取最小的那一半：**只改说明、
不改可达性、不改公开响应的键集**：

```python
@router.get("/backups", response_model=BackupListResponse,
             responses={422: {"description": NO_BODY_422_DESC}})
```

> 另一半「整档摘掉自动 422」要钉 FastAPI 的版本行为，且属全仓统一的一次活 ⇒ 不在单端点这一轮里做。

## 三、同族三只 helper 的返回值形状，要写在文档里而不是靠猜

`require_within_dir` 返回**解析之后的落点**；同族的 `pipeline.export_dataset` 与
`version_control` 也返回解析之后的落点；而 `visualizer._output_path` 返回**拼接**
（`self.output_dir / output_file`）——它**只借**了权威那句判决，没有把落点解析回来。

这不是 bug，是两个形状并存。正解**不是**把返回值改成解析后的落点（那是对外契约变更，且要连页面
一起改），而是**把两个形状写清楚**：

- docstring 里写明「这一面**只借**那句判决、交回的是**拼接**」；
- API 表里同写一句「同族另两面交回的是**解析之后**的落点」。

## 四、判决字段与诊断字段不在同一档

`validate-config` 回 `is_valid` 与一张问题清单（`errors` / `warnings` / `info` 三档）。
实际判决**只由 `errors` 决定**，另两档是诊断信息。客户端若把「`is_valid` 为真」读成
「没有问题清单」，就会在有 warning 时认为一切正常。

两种改法：① 全局 `RequestValidationError` 处理器把 422 翻成中文同一形状（**要改 `detail` 的形状**
且所有认 422 的用例都要重钉）；② **只把口径写进文档**。本轮取 ②：在 API 表那一行写明
「判决只由 `errors` 那一档决定；`warnings` 与 `info` 不参与判决」。

> 顺带记一条同族的坑：422 那一轨遇**非有限浮点**时是**整个响应没了**（不是 422）⇒ 想靠改
> 前端渲染来救是救不回来的。

## 五、键名是既成契约时，先写清它数的到底是什么

`AugmentationMetrics.unique_instructions` 那一格，字段名数的是**所选字段的唯一值数**
（`len(set(texts))`，`texts` 取自 `self.text_field`），不是「指令数」。

改名的代价是**破坏性变更**（要同时动 SDK dataclass 属性、响应字段、所有按位置构造这两键的调用点）⇒
按纪律要先拍。本轮取**只改口径**：在 API 表的 `dataset/impact` 一行写明「`unique_instructions`
这个键名是既成契约，但它数的是**所选字段的唯一值数**而不是『指令数』」。

## 连带代价（这类改动比看起来贵）

1. **行号位移**：三只产品文件在**中部**插行 ⇒ 全仓指向它们的 `file:行号` 引用全部位移
   （本轮 5 处）⇒ 按**内容守恒**逐条重钉，并复验 a184 的 `dead_line` 硬 0 档没被顶上去。
2. **既有守卫会钉住旧行为**：`l286` 那一支把 `C:` 钉在「自着陆」清单里 ⇒ 同一形状换臂就得
   **同批改钉它的判决**，否则尺子与实现各说各话。
3. **文档 ≠ 装饰**：这一轮五格里只有一格动了产品码，其余四格全是「文档面还没写实话」——文档
   撒谎的代价与代码撒谎同价，只是更慢被发现（客户端已经按错的契约写完了）。
4. **行号引用「数字看着对」也可能已经指错**：本轮那三处 `file:行号` 引用曾经指向**空行与装饰行**
   （脚本把同一段常量插了三份重复 ⇒ 位移算出来是 +30，三处引用全落在装饰行上）⇒ **提交前的
   「内容守恒」逐条断言（旧行内容 == 新行内容）**是唯一抓住它的手段：先把重复去掉（+12），
   再把三处引用钉回**类定义那一行**（`class XxxResponse(BaseModel):`），并复验 a184 的
   `dead_line` 硬 0 档与基线逐位相同。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L312 清单行与日志块；A374 / A336 / A342 / A318 / A284 行的
  L312 收口
- 守卫：`augmentor/tests/unit/test_colon_shape_l312.py`（零 parametrize，十格）
- 同族：`docs/solutions/best-practices/input-validation-pattern.md`、
  `docs/solutions/best-practices/verdict-narrowing-and-failure-honesty.md`、
  `docs/solutions/testing/gfm-table-pipe-escaping.md`
