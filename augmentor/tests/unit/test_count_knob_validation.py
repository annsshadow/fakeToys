# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""「取前 N 条」这类计数旋钮的同构判据

这些旋钮（`limit` / `offset` / `top_k` / `preview_size` / `batch_size`）在实现里
几乎都落到下标切片，而切片对越界值不报错、只换语义。本文件把 `search()` 早已
对 `fuzzy_threshold` / `ngram_n` 采用的判据推广到整个家族，判据本身只有
`augmentor.validation.require_count` 一处。

三条一起成立的主张：
1. 负数报错。以前 `[:top_k]` 读成「丢掉末尾 |top_k| 个」，`[-limit:]` 读成
   「去掉最前面 |limit| 条」，请求与答案正好相反，且没有任何信号。
2. `0` 是合法答案，不是「没传参数」。以前走 `value or default` 的调用点会把
   「一条都不要」静默换成默认条数（预览给 5 条、主动学习给 50 条），
   `x[-0:]` 更是直接等于「全部」。
3. 判参先于数据短路。空输入 + 坏参数必须仍然报参数错，否则坏参数会被
   空结果掩护掉。
"""

import json

import pytest

from augmentor.active_learning import ActiveLearningLoop
from augmentor.analytics import analyze_dataset_fast
from augmentor.cleaner import extract_keywords
from augmentor.dedup import Deduplicator
from augmentor.exceptions import DataValidationError
from augmentor.preview import PreviewGenerator
from augmentor.quality_monitor import QualityMonitor
from augmentor.search_enhanced import search_dataset
from augmentor.validation import require_count
from augmentor.versioning import VersionManager

ITEMS = [{"instruction": f"如何租房{i}", "output": f"登录官网办理第{i}步"} for i in range(12)]
KEYWORD_TEXT = "租房合同 租房合同 租房合同 押金 押金 中介 中介 房东 合同 签约 违约" * 2


class TestRequireCount:
    """判据本体"""

    @pytest.mark.parametrize("value", [0, 1, 10 ** 6, None])
    def test_allows_non_negative_integers_and_none(self, value):
        """`None` 交由调用点回落默认值，这里不替它决定"""
        assert require_count("limit", value) == value

    @pytest.mark.parametrize("value", [-1, -100, "5", 1.0, True, [], 2.5])
    def test_rejects_negative_and_non_integer(self, value):
        """报错而不是换语义：`True` 也算非整数，它会让「要 1 条」写成「要 True 条」"""
        with pytest.raises(DataValidationError):
            require_count("limit", value)

    def test_error_message_names_the_parameter_and_the_value(self):
        """同一处判据服务八个调用点，报错必须指名是哪个旋钮、当前是什么"""
        with pytest.raises(DataValidationError, match=r"top_k.*-3"):
            require_count("top_k", -3)

    def test_minimum_can_be_raised_to_one(self):
        """每轮必须选出样本的调用点用得到下界 1"""
        with pytest.raises(DataValidationError, match="batch_size"):
            require_count("batch_size", 0, minimum=1)

    def test_is_a_value_error_too(self):
        """API 侧靠 `ValueError → 400` 映射，判据必须留在该继承链上"""
        assert issubclass(DataValidationError, ValueError)


class TestSearchPagination:
    """分页窗口：`sorted_indices[offset:offset + limit]`"""

    def test_negative_limit_is_rejected(self):
        """以前 `[:0 + -1]` = 「除了最后一条」，条数比 `total_matches` 还多"""
        with pytest.raises(DataValidationError, match="limit"):
            search_dataset(ITEMS, "租房", ["instruction"], limit=-1)

    def test_negative_offset_is_rejected(self):
        """以前 `[-1:...]` 让窗口起点落到列表末尾，翻页翻出 `IndexError`"""
        with pytest.raises(DataValidationError, match="offset"):
            search_dataset(ITEMS, "租房", ["instruction"], offset=-1)

    def test_zero_limit_reports_the_total_without_items(self):
        """0 条是合法请求：`total_matches` 本来就是分页前的全集条数"""
        result = search_dataset(ITEMS, "租房", ["instruction"], limit=0)
        assert result.total_matches == len(ITEMS)
        assert result.items == []

    def test_non_integer_limit_is_rejected(self):
        """`"5"` 会在切片处抛 TypeError，绕过本模块的错误口径"""
        with pytest.raises(DataValidationError, match="limit"):
            search_dataset(ITEMS, "租房", ["instruction"], limit="5")


class TestKeywordTopK:
    """`sorted_words[:top_k]`"""

    def test_negative_top_k_is_rejected(self):
        """以前要 1 个关键词会拿到「除了最弱那个」的全部"""
        assert len(extract_keywords(KEYWORD_TEXT, 100)) > 3
        with pytest.raises(DataValidationError, match="top_k"):
            extract_keywords(KEYWORD_TEXT, -1)

    def test_zero_top_k_returns_no_keywords(self):
        assert extract_keywords(KEYWORD_TEXT, 0) == []

    def test_positive_top_k_still_truncates(self):
        """判据不能顺手把正常路径改成不截断"""
        assert len(extract_keywords(KEYWORD_TEXT, 2)) == 2


class TestAnalyticsTopK:
    """`Counter.most_common(top_k)`"""

    def test_negative_top_k_is_rejected(self):
        """以前 `most_common(-1)` 静默返回空列表，读起来像「语料没有词」"""
        with pytest.raises(DataValidationError, match="top_k"):
            analyze_dataset_fast(ITEMS, top_k=-1)

    def test_zero_top_k_keeps_the_rest_of_the_report(self):
        result = analyze_dataset_fast(ITEMS, top_k=0)
        assert result["top_words"] == []
        assert result["sample_size"] == len(ITEMS)


class TestSimilarPairsTopK:
    """`find_similar_pairs` 的保留集裁剪窗口"""

    def test_negative_top_k_is_rejected(self):
        """以前负数会以「裁掉保留集末尾」的身份穿过剪枝，把有解的请求答成空列表"""
        near = [{"instruction": f"这是用来比较的第{i}句甲乙丙丁戊"} for i in range(6)]
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        assert len(dedup.find_similar_pairs(near, top_k=3)) > 0
        with pytest.raises(DataValidationError, match="top_k"):
            dedup.find_similar_pairs(near, top_k=-1)

    def test_zero_top_k_returns_no_pairs(self):
        """0 在剪枝窗口里也必须被忠实读到（`kept.size > 0` 恒真，靠的是裁到 0）"""
        near = [{"instruction": f"这是用来比较的第{i}句甲乙丙丁戊"} for i in range(6)]
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        assert dedup.find_similar_pairs(near, top_k=0) == []

    def test_bad_knob_fails_even_when_the_input_is_too_short(self):
        """判参先于 `len(texts) < 2: return []`，否则坏旋钮被空结果掩护掉"""
        dedup = Deduplicator(threshold=0.9)
        dedup._model = "fallback"
        with pytest.raises(DataValidationError, match="top_k"):
            dedup.find_similar_pairs([{"instruction": "只有一条"}], top_k=-1)


class TestPreviewSize:
    """`items[:size]` 与 `preview_size or self.preview_size`"""

    def test_zero_does_not_fall_back_to_the_default(self):
        """以前 `0 or 5` 回落到 5：「只要格式信息」变成「给我 5 条样本」"""
        preview = PreviewGenerator().preview(ITEMS, "jsonl", 0)
        assert preview.original_data == []
        assert preview.format_info

    def test_zero_on_the_instance_is_honoured_too(self):
        assert PreviewGenerator(preview_size=0).preview(ITEMS, "jsonl").original_data == []

    def test_none_still_means_use_the_default(self):
        """`0` 与 `None` 必须是两个读得出来的状态——这条正是区分两者的那一刀"""
        assert len(PreviewGenerator(preview_size=2).preview(ITEMS, "jsonl", None).original_data) == 2

    @pytest.mark.parametrize("size", [-1, -12, "3", 1.5])
    def test_out_of_range_sizes_are_rejected(self, size):
        with pytest.raises(DataValidationError, match="preview_size"):
            PreviewGenerator().preview(ITEMS, "jsonl", size)

    def test_instance_default_is_validated_on_construction(self):
        """默认值也要判：`PreviewGenerator(-3)` 会一直影响之后每次调用"""
        with pytest.raises(DataValidationError, match="preview_size"):
            PreviewGenerator(preview_size=-3)


class TestHistoryLimit:
    """`self._snapshots[-limit:]` 与 `entries[:limit] if limit else entries`"""

    @pytest.fixture
    def monitor(self):
        m = QualityMonitor()
        for _ in range(3):
            m.check_quality(ITEMS)
        return m

    def test_history_zero_means_empty_not_everything(self, monitor):
        """以前 `-0:` 等价于 `[0:]`，「一条都不要」答成「全部 3 条」"""
        assert len(monitor.get_history(3)) == 3
        assert monitor.get_history(0) == []

    def test_history_negative_is_rejected(self, monitor):
        """以前 `[-(-2):]` = `[2:]`，「要 2 条」答成「丢掉最前面 2 条」"""
        with pytest.raises(DataValidationError, match="limit"):
            monitor.get_history(-2)

    def test_trend_shares_the_same_throat(self, monitor):
        assert monitor.get_trend("completeness", 0) == []
        with pytest.raises(DataValidationError, match="limit"):
            monitor.get_trend("completeness", -2)

    def test_version_history_zero_means_empty(self, tmp_path):
        manager = VersionManager(storage_dir=str(tmp_path))
        (tmp_path / "history.jsonl").write_text(
            "\n".join(json.dumps({"action": "create", "version_id": str(i)}) for i in range(3)),
            encoding="utf-8",
        )
        assert len(manager.get_history()) == 3
        assert manager.get_history(limit=0) == []

    def test_version_history_negative_is_rejected(self, tmp_path):
        manager = VersionManager(storage_dir=str(tmp_path))
        with pytest.raises(DataValidationError, match="limit"):
            manager.get_history(limit=-1)


class TestActiveLearningBatchSize:
    """`min(batch_size or self.batch_size, len(data))` 后的 `[:size]`"""

    def test_per_call_batch_size_cannot_bypass_the_constructor_rule(self):
        """构造器早就拒绝 0/负数，逐次覆盖参数以前是绕过它的那道侧门"""
        loop = ActiveLearningLoop(batch_size=2)
        with pytest.raises(DataValidationError, match="batch_size"):
            loop.select_samples(ITEMS, batch_size=-3)
        with pytest.raises(DataValidationError, match="batch_size"):
            loop.select_samples(ITEMS, batch_size=0)

    def test_bad_knob_fails_on_an_empty_pool(self):
        """判参先于 `if not data: return []`"""
        loop = ActiveLearningLoop(batch_size=2)
        with pytest.raises(DataValidationError, match="batch_size"):
            loop.select_samples([], batch_size=-1)

    def test_constructor_rejects_non_integers(self):
        """以前 `"50" <= 0` 抛 TypeError，绕开了本模块的错误口径"""
        with pytest.raises(DataValidationError, match="batch_size"):
            ActiveLearningLoop(batch_size="50")

    def test_valid_override_still_clamps(self):
        loop = ActiveLearningLoop(batch_size=2)
        assert len(loop.select_samples(ITEMS, batch_size=4)) == 4
        assert len(loop.select_samples(ITEMS[:2], batch_size=4)) == 2
