---
title: "CSV / TSV 表头也是公式注入面：危险列名拒收而不是静默加前缀（A391② / L297）"
date: 2026-11-09
last_refreshed: 2026-11-10
category: security-issues
module: augmentor/augmentor/validation.py + augmentor/augmentor/{converter,export,export_enhanced,csv_excel_import}.py
problem_type: security_issue
component: csv_tsv_header_injection
severity: medium
symptoms:
  - "CSV / TSV 导出的表头把 =cmd|' /C calc'!A0 这类列名原样写出，接收方打开文件即可能执行"
  - "给表头加前导单引号后，读回侧 csv.DictReader 拿到的键变成 '=cmd...，同一份数据集写出前后 schema 漂移"
  - "值面已有 A389 的前缀转义，表头面没有对应判据；xlsx 靠类型位、json/jsonl 没有公式解释器，只有 CSV/TSV 两条文本容器暴露"
root_cause: header_and_value_share_formula_leaders_but_header_is_a_schema_key
resolution_type: code_fix_with_guard
tags: [csv-injection, formula-injection, cwe-1236, header, schema-drift, fail-loud, ast-wiring, injection-evidence]
applies_when:
  - "导出 CSV / TSV / 电子表格，列名或单元格值来自用户可控数据"
  - "考虑对表头做转义，但同一列名同时是读回侧配行的键"
  - "同一族判据要覆盖多条写边（pandas to_csv、csv.DictWriter、原生导出器）"
related_components:
  - augmentor/augmentor/validation.py
  - augmentor/augmentor/converter.py
  - augmentor/augmentor/export.py
  - augmentor/augmentor/export_enhanced.py
  - augmentor/augmentor/csv_excel_import.py
  - augmentor/tests/unit/test_sheet_header_injection_l297.py
  - augmentor/tests/unit/test_sheet_formula_injection_l287.py
---

# CSV / TSV 表头也是公式注入面

## Problem

A389（L287）按拍板① 给**单元格值**加了电子表格公式前导的转义：命中 `= + - @ 制表 回车` 时，
在值前加一个单引号，让 Excel / LibreOffice 把它当文本而不是公式。

那一轮刻意没有动**表头**，并把原因写在账上：列名和值不是同一种东西。列名同时是读回侧
`csv.DictReader` 用来配行的键。如果照值面的做法给列名加 `'` 前缀，同一份数据写出前键是
`=cmd|' /C calc'!A0`，读回后键变成 `'=cmd|' /C calc'!A0` ⇒ 写出去的文件在自己的读边上配不上行，
schema 在往返途中漂移。

于是表头这一面留成了缺口：危险列名仍然原样交付给接收方。用户 2026-11-09 对 A391 拍板②：
**拒收危险列名，不改写**。

## Root Cause

**表头和值共用同一族公式前导判据，但表头承担的是 schema 键的角色。**

- 值面可以无损加前缀：值的职责是「被阅读」，加一个文本标记不改变业务含义。
- 表头面不能静默改写：列名的职责是「被程序配行」，改一个字符就换了一个键。
- 因此表头面的正确处置不是「转义」，而是「拒绝」——让调用方改列名或改用 xlsx / jsonl，
  而不是交付一份出去和读回不一致的文件。

同族判据已经在 `validation.py` 里有唯一产地：`is_sheet_formula_text` / `SHEET_FORMULA_LEADERS`
（六档：`= + - @ \t \r`）。表头缺的不是判据本身，而是一条**只判不改**的入口，以及五条写边的接线。

## Resolution

在 `augmentor/validation.py` EOF 新增唯一产地：

```python
def require_sheet_safe_fieldnames(fieldnames, container) -> List[str]:
    names = list(fieldnames)
    forged = [name for name in names if is_sheet_formula_text(name)]
    if forged:
        listed = "、".join(repr(name) for name in forged)
        raise DataValidationError(
            f"{container} 表头有会被电子表格当公式执行的列名：{listed}；"
            f"列名同时是读回侧配行的键，静默改写会让 schema 漂移。"
            f"请去掉列名开头的 {leaders} 之一，或改用 xlsx / jsonl 导出"
        )
    return names
```

五个写边共引：

| 写边 | 接入点 | 备注 |
|---|---|---|
| `converter.py` csv | `csv.DictWriter` 之前 | 用户可控列名的主路径 |
| `converter.py` tsv | `csv.DictWriter` 之前 | 同上 |
| `export_enhanced.py` csv | `csv.DictWriter` 之前 | 用户可控列名的主路径 |
| `export_enhanced.py` tsv | `csv.DictWriter` 之前 | 同上 |
| `csv_excel_import.export_to_csv` | pandas `to_csv` 之前 | 判在 `mkdir` / 写盘之前 ⇒ 拒收不留空文件 |

`export.py` 原生 CSV 的表头固定归一为 `instruction/input/output`，任意列名到不了表头；
那里加的包裹是防御性接线，不是本轮真正承重的写边。

不动的三档：

- **xlsx**：表头格子已经由 `text_only_cell` 强制成文本类型位（`t="inlineStr"`、零 `<f>`），
  同一列名照收且不会执行。
- **json / jsonl**：没有公式解释器，列名原样保留。
- **值面**：继续走 A389 的前缀转义，与本轮的表头拒收分工明确。

行为矩阵（改后实测）：

| 输入 | CSV / TSV | xlsx | json / jsonl |
|---|---|---|---|
| 列名 `=cmd|' /C calc'!A0` | `DataValidationError` ⇒ API **400**，文案点名列名 | 200，格子 `data_type == "s"`、零 `<f>` | 200，列名原样 |
| 列名 `+cmd` / `-cmd` / `@cmd` / 制表 / 回车开头 | 逐档拒收并点名 | 同上 | 同上 |
| 普通列名 | 原序原值放行 | 原样 | 原样 |
| 多条危险列名 | 一条报错列出全部危险列名，普通列名不进文案 | 同上 | 同上 |

`DataValidationError` 是 `ValueError` 子类，经 `api/deps.py::to_http_error` 的 `ValueError` 支回
**400**，原文照回；pandas 那条边因为判在 `mkdir` 之前，拒收时连空文件都不留。

## Guard and Evidence

新增 `augmentor/tests/unit/test_sheet_header_injection_l297.py` 共 **25 支 / 5 处 parametrize**，六组面：

1. **判据本体**：六档前导逐档拒收、一条报错列出全部危险列名、普通列名原序原值放行、
   `DataValidationError` 经 `to_http_error` 映射 400 且原文点名。
2. **五条写边真跑**：converter csv/tsv、export_enhanced csv/tsv、pandas `to_csv` 各自抛
   `DataValidationError`，且产物里零危险表头字节；pandas 边连空文件都不留。
3. **负面对照**：xlsx 同一列名照收但强制文本类型位；json / jsonl 原样保留。
4. **接线普查**：产品树每一处 `csv.DictWriter(fieldnames=...)` 都必须过权威（现量 5 处 / 3 个文件），
   产品树唯一一处 `DataFrame.to_csv` 必须在 `mkdir` / 写盘之前调用权威；两把 AST 普查各带合成牙齿臂，
   数不到不是干净。
5. **牙齿**：把权威换成恒等 ⇒ 危险表头当场回到磁盘。

两臂注入取证（改完逐字节复原，`validation.py` 前后 sha256 `d9ead53868047ee9…` 一致）：

| 注入 | 读数 | 点名 |
|---|---|---|
| helper 恒盲 | `15 failed, 10 passed` | 判据本体、五条写边、负面对照、牙齿面 |
| helper 只认 `=` | `6 failed, 19 passed` | `+` / `-` / `@` / 制表 / 回车五档与全列名点名支 |

两臂都证明承重点在「六档前导」而不是只认等号。

定向复跑：L297 + l287 + `test_csv_excel_export` + `test_csv_excel_import` ⇒ **59 passed**；
扩展产品批（converter / export / export_enhanced / 原子写 / L287 / L297 等十文件）⇒ **401 passed**。
全量定标：`37 failed, 10261 passed, 6 skipped, 494.70s`，覆盖率 `99.62%`；37 项红全部落在既有基线族
（端点计数、门禁、TS 契约、a184 注释引用、l200 文档数等），本轮未扩大修复范围，不声称全绿。

## Lessons

1. **同一段文本在不同位置承担不同职责**：值可以转义，键不可以。判据可以共用，处置必须分开。
2. **拒收也是安全默认值**：静默改写会让故障延迟到读回侧，而且更难查；fail-loud 把代价放回产生危险列名的那一步。
3. **守卫要对着接线，不只对着函数**：函数单测全绿不代表每条写边都接了权威。AST 普查 + 合成牙齿臂才能钉住五条写边。
4. **负面对照是范围判据的一部分**：xlsx 有类型位、json/jsonl 没有公式解释器，这三档必须证明「不误伤」，
   否则下一次有人会把判据扩到所有容器。
5. **写盘前的判据位置很重要**：pandas 边判在 `mkdir` 之前，拒收才不留空文件；判在 `to_csv` 那一行就晚了。

## References

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L297 清单行与日志块，A391 行（`~~S~~ **已收口（L297）**`）
- 提交：`75342b9e1`（`fix,test,docs(augmentor): L297 A391② CSV/TSV 表头拒收公式前导`）
- 守卫：`augmentor/tests/unit/test_sheet_header_injection_l297.py`
- 同族前一轮：账本 L287 / A389（值面前缀转义）
- 契约：`docs/augmentor/API.md` `/api/dataset/convert` 行（CSV / TSV 危险列名回 400）
