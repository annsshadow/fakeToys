# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L199（B265）：封闭清单族最后一轮 —— 7 处「枚举形参静默换语义」收口

与 L175（`search_enhanced.search` 的 `method`）、L176（`save_quality_report` 的
`format`）、L189（预设）、L192（cleaner 规则）、L193（迁移规则 id）、L194（测试
套件名）同一族。本轮把调研扫出的剩余 7 处一次收掉；分开做必然出现「修了 6 处漏
1 处」的 L194 型二次欠账。

七处的共同形状：**参数只与一个字面量比一次，比不上就静默走默认分支**。七种变体：

| 站点 | 未知值的后果 |
| --- | --- |
| `visualize_dataset(format=)` | 静默走 text 支，还按 text 落盘 |
| `DatasetIndexer.search(method=)` | 静默回落 contains，而 `index_used` **谎报**实际方法 |
| `DatasetView.to_file(format=)` | 静默写成 JSON 数组 |
| `DataSanitizer.remove_duplicates(keep=)` | 静默按 first 走（与 docstring 承诺相反） |
| `get_dependencies(direction=)` | 三条分支全不命中 ⇒ 返回 `[]`，给出**错的结论** |
| `StreamWriter(mode=/format=)` | 把 JSON 数组写进声称为 csv 的产物 / 追加写出半份 JSON |
| `GateRule(severity=)` | `"warn"` 被 else 分支静默**升为 error 级**门禁 |

**判据一律不新造清单**：清单一律立为模块级常数（A77 单一权威），
`validation.require_choice` 只判「选中项存不存在」。`INDEXER_SEARCH_METHODS`
刻意**不**共引 `search_enhanced.SEARCH_METHODS` —— 后者有 5 项而索引器只实现 3 项，
照抄会让 fuzzy / regex 通过判据后继续静默回落 contains，把 L175 刚堵的口子原样开回来。
"""

import json

import pytest

from augmentor.dependency import DEPENDENCY_DIRECTIONS, DependencyManager
from augmentor.exceptions import DataValidationError
from augmentor.indexer import (
    INDEXER_FILE_FORMATS,
    INDEXER_SEARCH_METHODS,
    DatasetIndexer,
    DatasetView,
)
from augmentor.quality_gate import GATE_SEVERITIES, GateRule
from augmentor.streaming import WRITER_FORMATS, WRITER_MODES, StreamWriter
from augmentor.validation import DEDUP_KEEP_MODES, DataSanitizer
from augmentor.visualize_enhanced import VISUALIZE_FORMATS, visualize_dataset

ITEMS = [
    {"instruction": "如何申请入住？", "input": "", "output": "提交申请。"},
    {"instruction": "如何申请入住？", "input": "", "output": "另一份回答。"},
    {"instruction": "退租流程是什么？", "input": "", "output": "提前 30 天。"},
]

#: (用例 id, 站点标签, 常数) —— 供表面守卫共用
SURFACE = [
    ("visualize", "visualize_enhanced.visualize_dataset", VISUALIZE_FORMATS),
    ("indexer_search", "DatasetIndexer.search", INDEXER_SEARCH_METHODS),
    ("indexer_to_file", "DatasetView.to_file", INDEXER_FILE_FORMATS),
    ("keep", "DataSanitizer.remove_duplicates", DEDUP_KEEP_MODES),
    ("direction", "DependencyManager.get_dependencies", DEPENDENCY_DIRECTIONS),
    ("writer_mode", "StreamWriter", WRITER_MODES),
    ("writer_format", "StreamWriter", WRITER_FORMATS),
    ("severity", "GateRule", GATE_SEVERITIES),
]


def _bad_call(site, value, **kwargs):
    """按站点触发一次「传了坏值」的调用，返回被抛出的异常"""
    if site == "visualize":
        return lambda: visualize_dataset(ITEMS, None, format=value)
    if site == "indexer_search":
        return lambda: DatasetIndexer(ITEMS).search("入住", method=value)
    if site == "indexer_to_file":
        return lambda: DatasetView(ITEMS).to_file(
            str(kwargs["tmp"] / "out.json"), format=value)
    if site == "keep":
        return lambda: DataSanitizer().remove_duplicates(ITEMS, keep=value)
    if site == "direction":
        return lambda: DependencyManager(str(kwargs["tmp"] / "reg")).get_dependencies(
            "x", direction=value)
    if site == "writer_mode":
        return lambda: StreamWriter(kwargs["tmp"] / "o.json", mode=value)
    if site == "writer_format":
        return lambda: StreamWriter(kwargs["tmp"] / "o.json", format=value)
    if site == "severity":
        return lambda: GateRule(name="r", metric_key="pass_rate",
                                operator=">=", value=0.9, severity=value)
    raise AssertionError(site)


class TestUnknownValuesAreRejected:
    """每个站点的未知值都必须抛 DataValidationError，并列出全部合法取值"""

    @pytest.mark.parametrize("site", [s[0] for s in SURFACE])
    def test_unknown_value_raises_with_full_list(self, site, tmp_path):
        const = dict((s[0], s[2]) for s in SURFACE)[site]
        with pytest.raises(DataValidationError) as exc:
            _bad_call(site, "nonsense", tmp=tmp_path)()
        message = str(exc.value)
        for legal in const:
            assert legal in message, (
                f"{site}: 报错文案没列出合法值「{legal}」——列全清单是这条判据的"
                "一半价值，缺了它用户仍要试错"
            )
        assert "nonsense" in message

    @pytest.mark.parametrize("site", [s[0] for s in SURFACE])
    def test_explicit_none_is_not_the_same_as_absent(self, site, tmp_path):
        """显式 null 必须拒：「没传」由默认值负责，「传了 null」是另一件事"""
        with pytest.raises(DataValidationError):
            _bad_call(site, None, tmp=tmp_path)()

    @pytest.mark.parametrize("site", [s[0] for s in SURFACE])
    @pytest.mark.parametrize("shape", [5, ["json"], {"a": 1}, True])
    def test_non_string_shapes_are_rejected(self, site, shape, tmp_path):
        """形状错也要报领域异常（不是 TypeError/AttributeError）"""
        with pytest.raises(DataValidationError):
            _bad_call(site, shape, tmp=tmp_path)()


class TestLegalValuesStillWork:
    """防修过头：合法值一个都不许被这次判据挡下"""

    def test_visualize_each_format(self):
        for fmt in VISUALIZE_FORMATS:
            out = visualize_dataset(ITEMS, None, format=fmt)
            assert isinstance(out, str) and out

    def test_indexer_search_each_method(self):
        indexer = DatasetIndexer(ITEMS)
        for method in INDEXER_SEARCH_METHODS:
            # exact 比的是整个字段值，contains / ngram 比的是子串 ⇒ 用完整指令当查询，
            # 三法都该命中那两条重复记录
            result = indexer.search("如何申请入住？", method=method)
            assert result.total_matches >= 1, f"{method} 本该命中"

    def test_indexer_search_default_is_unchanged(self):
        """默认方法（不传）必须仍是 contains 且命中不变"""
        assert DatasetIndexer(ITEMS).search("入住").total_matches == 2

    def test_keep_each_mode(self):
        items = [
            {"instruction": "同一个问题", "output": "第一次的回答"},
            {"instruction": "同一个问题", "output": "第二次的回答"},
        ]
        first = DataSanitizer().remove_duplicates(items, keep="first")
        last = DataSanitizer().remove_duplicates(items, keep="last")
        assert first[0]["output"] == "第一次的回答"
        assert last[0]["output"] == "第二次的回答"

    def test_direction_each_mode(self, tmp_path):
        manager = DependencyManager(str(tmp_path / "reg"))
        a = manager.register_dataset("a", "A", 10)
        b = manager.register_dataset("b", "B", 20)
        manager.add_dependency(a.dataset_id, b.dataset_id, "derived")
        assert len(manager.get_dependencies(a.dataset_id, "upstream")) == 0
        assert len(manager.get_dependencies(a.dataset_id, "downstream")) == 1
        assert len(manager.get_dependencies(a.dataset_id, "both")) == 1

    def test_direction_default_is_unchanged(self, tmp_path):
        manager = DependencyManager(str(tmp_path / "reg"))
        a = manager.register_dataset("a", "A", 1)
        assert manager.get_dependencies(a.dataset_id) == []

    def test_writer_each_mode_and_format(self, tmp_path):
        """四种 mode × format 组合里三种可用、一种拒绝（L208）

        被拒的那种是 `mode='a' + format='json'`：追加一份已闭合的 JSON 数组
        只会产出非法 JSON（实测 `[{"a": 1}]{"b": 2}`），所以 L208 起构造期就拒。
        """
        for mode in WRITER_MODES:
            for fmt in WRITER_FORMATS:
                path = tmp_path / f"o_{mode}_{fmt}.json"
                if mode == "a" and fmt == "json":
                    with pytest.raises(DataValidationError):
                        StreamWriter(path, mode=mode, format=fmt)
                    continue
                with StreamWriter(path, mode=mode, format=fmt) as writer:
                    writer.write_chunk(ITEMS)
                assert path.exists()

    def test_severity_each_mode(self):
        for sev in GATE_SEVERITIES:
            assert GateRule(name="r", metric_key="k", operator=">=",
                            value=1, severity=sev).severity == sev


class TestIndexerNoLongerLiesAboutTheMethod:
    """L199 最值钱的一条：`index_used` 不许再谎报实际用的方法"""

    def test_index_used_reports_the_executed_method(self):
        result = DatasetIndexer(ITEMS).search("入住", method="ngram")
        assert result.index_used.startswith("ngram:"), result.index_used

    def test_index_used_default_is_contains(self):
        result = DatasetIndexer(ITEMS).search("入住")
        assert result.index_used.startswith("contains:"), result.index_used

    def test_fuzzy_and_regex_are_rejected_not_silently_downgraded(self):
        """索引器没实现这两个方法，必须明说而不是回落 contains

        这一条是 L175 的姊妹判据：`SEARCH_METHODS` 有 5 项而这里只实现 3 项，
        共引那张表会让 fuzzy / regex 通过判据后继续静默回落。
        """
        indexer = DatasetIndexer(ITEMS)
        for method in ("fuzzy", "regex"):
            with pytest.raises(DataValidationError):
                indexer.search("入住", method=method)


class TestConstantsAreTheSingleAuthority:
    """常数必须是唯一产地：API 面若也要判，就共引而不是抄第二份"""

    def test_lists_are_tuples_and_non_empty(self):
        for _, _, const in SURFACE:
            assert isinstance(const, tuple) and const, const

    def test_no_duplicate_entries(self):
        for _, _, const in SURFACE:
            assert len(set(const)) == len(const), f"{const} 里有重复项"

    def test_indexer_methods_are_a_strict_subset_of_search_methods(self):
        """索引器的三法必须是 search_enhanced 五法的子集——否则共引才有可能"""
        from augmentor.search_enhanced import SEARCH_METHODS

        assert set(INDEXER_SEARCH_METHODS) < set(SEARCH_METHODS), (
            "索引器方法集不是 SEARCH_METHODS 的真子集 ⇒ 当初「刻意不共引」的理由"
            "变了，重新拍一次要不要共引"
        )
