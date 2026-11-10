---
title: "补标要有据：定级格转成闭合形状的三道闸；判据住一处的落地路径是「声明盲区」名册"
date: 2026-11-11
last_refreshed: 2026-11-11
category: best-practices
module: augmentor/OPTIMIZATION_LOOP_2.md + augmentor/augmentor/analytics.py + augmentor/augmentor/visualizer.py + augmentor/augmentor/quality_report.py + augmentor/augmentor/converter.py
problem_type: work_done_but_the_ledger_still_counts_it_as_pending
component: ledger_backlog_restamp_and_blind_roster_wiring
severity: medium
symptoms:
  - "工作早在几十轮前就落地了，账本的定级格却还停在裸字母 ⇒ 尺子一直把它算作「未判决」，待办总数虚高"
  - "用户拿到的那句判决缺主语：「无法从  转换到 JSON」——主语位置插进去的就是那个空串"
  - "CLI 侧同一形退化语料只回一句裸异常消息（`object of type 'int' has no len()`），不点名第几条、哪个字段"
root_cause: a_closure_written_in_the_prose_but_not_in_the_grade_cell_is_not_counted_as_closed
resolution_type: restamp_with_evidence_and_wire_the_declared_blind_spots
tags: [ledger, restamp, grade-cell, evidence, blind-roster, single-authority, verdict-subject, cli, api-contract]
applies_when:
  - "正文里写着「已做（L##）」，而定级格仍是裸字母（尺子只认定级格）"
  - "一个空值或未声明值被拼进判决的主语位置，句子读不出是什么"
  - "同一份退化输入在 HTTP 面给域判决、在 CLI 面给裸异常消息（判据没有接到底）"
related_components:
  - augmentor/OPTIMIZATION_LOOP.md
  - augmentor/OPTIMIZATION_LOOP_2.md
  - augmentor/tests/unit/test_verdict_subject_and_cli_blind_spots_l310.py
  - augmentor/tests/unit/test_dataset_text_field_l274.py
  - augmentor/augmentor/analytics.py
  - augmentor/augmentor/visualizer.py
  - augmentor/augmentor/quality_report.py
  - augmentor/augmentor/converter.py
---

# 补标要有据：定级格转成闭合形状的三道闸；判据住一处的落地路径是「声明盲区」名册

## 现象一：账本有「工作做完了但仍算待办」的一格

账本的两卷 Backlog 表里，每一行的**定级格**是这一行处置结论的唯一权威
（尺子 `test_backlog_mark_vocab_l253.py` 的 `MEASURED_LETTERS` 只数定级格上的
字母形与闭合词）。于是有一类欠账会一直挂着：

> 正文里明写「**已做（L27）**：……（证据、实测、代价都在）」，
> 而**定级格仍是裸的 `S` / `M`** ⇒ 尺子把它算作「未判决」⇒ 待办总数虚高。

本轮量到的现量：**128 格**（裸字母 S 71 + M 56 + L 1），其中**七格**属于这一类
（正文各自点名了落地那一轮与证据，形状却从未转成规范的闭合形状）。

## 补标的三道闸（缺一格就只是抄字面）

把定级格从裸字母改成 `~~X~~ 已收口（L###）` 是**搬一行字**，三道闸才让它算「补标」：

1. **证据仍在**：那一行正文点名的产物/守卫/文档行**今天还在仓内**且内容对得上。
   本轮的守卫（`test_verdict_subject_and_cli_blind_spots_l310.py`）把这条钉成
   「形状对了但证据没了 ⇒ 判红」——否则补标会退化成抄一遍字面。
2. **轮号本卷可查**：闭合形状里那个轮号必须是**本卷声明过**的（清单行或日志块标题），
   否则可追溯性那一格把它算进「两卷都查不到」＝缺陷档。
3. **与尺子同批重钉**：定级格一动，标记面 / 宽面 / 字母形 / 词表 / 组合名册
   七把尺子的常数全部要跟着走（**精确相等**，不是「≤」），少钉一格就当场红。

**教训**：补标的成本不在改字，在**逐条取证**。本轮七格里有两格（A21 / A28）
正文也写着「已做」，但那是**部分完成**（那两行自己写明「①②③ 仍待办」）⇒ 不补标。
判据是「那一行自己声明这格收完了没有」，不是「正文里有没有出现过『已做』两个字」。

## 现象二：判决缺主语

```python
raise DataFormatError(f"无法从 {source_format} 转换到 JSON")
```

`source_format` 的默认是 `None`（＝按扩展名推断），**空串不是 `None`**：
空串一路走到这一行才炸，句子里的主语位置插进去的就是那个空串 ⇒
「无法从  转换到 JSON」——**两个连续空格，说不出是什么格式转不了**；
而**省略**那一格 ⇒ 200（走扩展名推断）⇒ 两者不等价。

**修法两档**：① 空串归一成 `None`（**行为变化**：原本 400 的调用会变 200 ⇒ 按
「破坏性默认值变更需逐项确认」那一族要做确认）；② 保留 400，只把文案换成能读的
（**点名输入文件 + 说清「省略」与「显式声明」两种形状的差别**）。
本轮取②，并让守卫钉三格：点名主语 / 空串**仍被拒**（只改文案、不改可达性）/
省略仍走推断。

## 现象三：同一形退化输入，HTTP 面给判决、CLI 面给裸消息

`[{"instruction": 1, "output": 2}]` 在形状门口完全合法（只判「顶层是 list、元素是 dict」），
于是它一路走进那些把 `item.get(key, "")` 当字符串用的写法。

**HTTP 面**早就接上了 `require_item_text`（L274 那轮，六只端点从 500 收到 400）。
**CLI 面**没有：`analyze` / `quality-report` 两个子命令回 rc=1 + stderr 一句
`错误: object of type 'int' has no len()` —— 不点名第几条、哪个字段。

**落地路径是现成的**：L274 那把尺子早就把这三只写成了 `BLIND_ROSTER`
（**声明盲区**，理由逐条写着「CLI-only 入口，HTTP 面上量不到」），并明写
「谁去接线，这一格当场红，**必须把该条从盲区名册挪进 `WIRING_ROSTER` 并同步账本**，
而不是悄悄留着」。

⇒ **判据住一处（A77）在这条路上的形状是「名册」而不是「约定」**：
接线的成本 = 改产品码一处 + 把那一格从盲区名册挪进接线名册 + 让
「出厂面调用点数 == 名册总和」那一格重新对上。**另写一份 CLI 专属判据**
（候选②）会造出同一判据的两份实现，正是 A77 要防的那件事。

**边界要量清**：接线只动 CLI 面还是连 HTTP 面一起动？本轮量到 HTTP 面**一格未动**
——`GET /api/analyze/{fn}` 仍由**早已接线**的 `sampler.analyze_coverage` 先拦，
`GET /api/visualize/{fn}` 走的是 `generate_all_visualizations` 而**不经过**
`generate_statistics`。**「同一个函数被几条路走到」是要量的，不是推的**。

## 连带代价：行号位移与重钉

三只产品文件插行 ⇒ 指向它们的 `file.py:NNN` 引用全部位移。本轮共 **11 处**，
其中 **5 处**一度把 a184 的 `dead_line` 硬 0 档从 29 顶到 34。

处置按**内容守恒**（`difflib` 等块逐行对位、**逐条断言旧行内容 == 新行内容**，
脚本不合格即拒改）⇒ 重钉后 `dead_line` 回到 29 一格未动。

**纪律**：写账的动作本身会造新账（这一族已第 N 次复现）——改产品码的那一轮
就要把引用重钉排进去，**不是等守卫红了才还**。

## 参考

- 账本：`augmentor/OPTIMIZATION_LOOP_2.md` L310 清单行与日志块；A291 / A359 行的 L310 收口
- 守卫：`augmentor/tests/unit/test_verdict_subject_and_cli_blind_spots_l310.py`（零 parametrize）
- 判据住一处的名册：`augmentor/tests/unit/test_dataset_text_field_l274.py`
  的 `WIRING_ROSTER` / `BLIND_ROSTER`
- 同族：`docs/solutions/best-practices/receipt-artifact-alignment-and-swallowed-items.md`、
  `docs/solutions/testing/ledger-backlog-id-claim-roster.md`
