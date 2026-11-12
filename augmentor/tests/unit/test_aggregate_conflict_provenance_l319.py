# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L319（A297）：一致性聚合的冲突明细要点名**分歧发生在同一个来源内部还是跨来源**

**改前那一格错在哪**：`aggregate_consistent()` 的判据是「同一个 key 下的所有
(来源, 答案) 对，答案集合大小为 1 才落地」—— 它**不区分**分歧是同一个来源内部
（同问不同答）还是两个来源给的答案不一样。两种分歧的后果却完全不同：

| 形状 | `sources` | 改前的判决 | 后果 |
| --- | --- | --- | --- |
| 同一份文件里两条同 key 不同答案 | `["甲", "甲"]` | 一条冲突 | 那两条**永不落地**，`aggregated_count` 是 **0** |
| 两个来源对同一个 key 给了不同答案 | `["甲", "乙"]` | 一条冲突 | 同上，但这是**跨来源交叉验证**本该做的事 |

而 docstring 写的是「仅保留各源对相同 key 答案一致的数据」⇒ 用户读成「跨来源交叉
验证」，实现是「同 key 的所有出现都必须一致，包括同一份文件里的重复提问」⇒ 一份
内部有重复提问的数据集会让整组产物为空，而回执只说 `aggregated_count: 0` 加一串
冲突，**不告诉你冲突的两边其实是同一个来源**。

**这一轮的处置（自决，记理由）**：取 A297 记的修法②「在冲突明细里显式标注『同源
内部』」—— **不取**修法①「冲突判据改成按来源去重后再比」：那会把同源自比的分歧从
「冲突」里抹掉，于是那两条重复提问会**悄悄落地**（同一份文件里同问不同答的两条
都进产物），那是**更坏**的一种错（产物里多出互相矛盾的样本）。

**三面判据**：

1. **行为面**（SDK 直构，零 HTTP 依赖）：同源分歧 ⇒ `same_source is True` 且条目
   **不落地**；跨来源分歧 ⇒ `same_source is False`；同 key 同答案但 `input` 不同 ⇒
   不算冲突、正常落地 1 条。
2. **HTTP 面**：端点回执里的 `conflicts` 逐项带这一格（`conflicts` 是自由字典，
   所以契约面不改键集，只多一格内容）。
3. **类型与页面面**：TS 的 `AggregateConflict` 有 `same_source: boolean`，且页面按它
   分两种文案渲染（「同一个来源内部的分歧」/「跨来源的分歧」）。

**注入臂**（在内存里做字符串手术，**零写入**）：把 SDK 里那一格从源码里摘掉 ⇒
「回执逐项带这一格」这一面当场红。

跑法：`python -m pytest tests/unit/test_aggregate_conflict_provenance_l319.py -q --no-cov`
"""

import pathlib

from augmentor.aggregator import DataAggregator

ROOT = pathlib.Path(__file__).resolve().parents[2]
AGGREGATOR_PY = ROOT / "augmentor" / "aggregator.py"
TS_API = ROOT / "web" / "src" / "types" / "api.ts"
DATA_MANAGEMENT = ROOT / "web" / "src" / "pages" / "DataManagement.tsx"


def _agg(source: str, items: list) -> dict:
    """造一只聚合器（键与值字段都用出厂默认：`instruction` / `output`）"""
    return DataAggregator().aggregate({source: items}, strategy="consistent")


class TestTheProvenanceKey:
    """行为面：同源内部分歧与跨来源分歧必须能被回执分开"""

    def test_a_same_source_disagreement_is_marked_and_does_not_land(self):
        result = _agg("甲", [
            {"instruction": "同一个问题", "output": "答案一"},
            {"instruction": "同一个问题", "output": "答案二"},
        ])
        assert len(result.conflicts) == 1, result.conflicts
        assert result.conflicts[0]["same_source"] is True, result.conflicts[0]
        assert result.conflicts[0]["sources"] == ["甲", "甲"], result.conflicts[0]
        # 那两条**永不落地**（这一格是 A297 立账时的实测读数）
        assert result.aggregated == [], result.aggregated

    def test_a_cross_source_disagreement_is_marked_as_cross_source(self):
        result = DataAggregator().aggregate(
            {"甲": [{"instruction": "同一个问题", "output": "答案一"}],
             "乙": [{"instruction": "同一个问题", "output": "答案二"}]},
            strategy="consistent",
        )
        assert len(result.conflicts) == 1, result.conflicts
        assert result.conflicts[0]["same_source"] is False, result.conflicts[0]
        assert result.conflicts[0]["sources"] == ["甲", "乙"], result.conflicts[0]
        assert result.aggregated == [], result.aggregated

    def test_same_answer_different_input_is_not_a_conflict(self):
        """只比 `output`：`input` 不同而同答案 ⇒ 正常落地 1 条、不算冲突"""
        result = _agg("甲", [
            {"instruction": "同一个问题", "input": "", "output": "同一个答案"},
            {"instruction": "同一个问题", "input": "补充材料", "output": "同一个答案"},
        ])
        assert result.conflicts == [], result.conflicts
        assert len(result.aggregated) == 1, result.aggregated

    def test_the_other_three_strategies_still_have_no_conflicts(self):
        """其余三档实测恒为 `[]`（这一格不因本轮改动而变）"""
        for strategy in ("union", "intersection", "weighted"):
            result = DataAggregator().aggregate(
                {"甲": [{"instruction": "同一个问题", "output": "答案一"}],
                 "乙": [{"instruction": "同一个问题", "output": "答案二"}]},
                strategy=strategy,
            )
            assert result.conflicts == [], (strategy, result.conflicts)


class TestTheReceiptCarriesIt:
    """契约面：冲突明细逐项带这一格（`conflicts` 是自由字典 ⇒ 契约键集不变）"""

    def test_every_conflict_item_carries_the_key(self):
        result = _agg("甲", [
            {"instruction": "同一个问题", "output": "答案一"},
            {"instruction": "同一个问题", "output": "答案二"},
        ])
        for item in result.conflicts:
            assert "same_source" in item, item
        assert result.to_dict()["conflicts"] == result.conflicts

    def test_the_sdk_source_really_writes_the_key(self):
        source = AGGREGATOR_PY.read_text(encoding="utf-8")
        assert '"same_source": len(set(sources)) == 1,' in source, (
            "SDK 那一格被摘掉了 ⇒ 冲突明细不再点名分歧的来源形状"
        )

    def test_injection_removing_the_key_turns_the_source_face_red(self):
        """注入臂：把那一格从源码里摘掉（**只在内存里**）⇒ 本面当场红"""
        source = AGGREGATOR_PY.read_text(encoding="utf-8")
        mutated = source.replace('                    "same_source": len(set(sources)) == 1,\n', "", 1)
        assert mutated != source, "注入臂没生效：源码形状变了"
        assert '"same_source": len(set(sources)) == 1,' not in mutated


class TestTheTypeAndThePage:
    """类型与页面面：TS 类型有这一格，页面按它分两种文案"""

    def test_the_ts_type_declares_the_key(self):
        text = TS_API.read_text(encoding="utf-8")
        assert "export interface AggregateConflict {" in text
        block = text.split("export interface AggregateConflict {", 1)[1].split("}", 1)[0]
        assert "same_source: boolean" in block, block

    def test_the_page_renders_the_two_shapes(self):
        text = DATA_MANAGEMENT.read_text(encoding="utf-8")
        assert "conflict.same_source" in text, "页面没有按这一格分两种文案"
        assert "同一个来源内部的分歧" in text, "页面上没有同源内部分歧那一句"
        assert "跨来源的分歧" in text, "页面上没有跨来源分歧那一句"
