# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""「取前 N 条」这类计数旋钮的同构判据

这些旋钮（`limit` / `offset` / `top_k` / `preview_size` / `batch_size` / `size` /
`n` / `diversity_sample_size` / `max_backups` / `chunk_size` / `target_size` /
`num_topics` / `topics_per_strategy` / `num_questions_per_topic`）在实现里几乎都落到
下标切片或算术，而切片对越界值不报错、只换语义。本文件把 `search()` 早已对
`fuzzy_threshold` / `ngram_n` 采用的判据推广到整个家族，判据本身只有
`augmentor.validation.require_count` 一处。

五条一起成立的主张：
1. 负数报错。以前 `[:top_k]` 读成「丢掉末尾 |top_k| 个」，`[-limit:]` 读成
   「去掉最前面 |limit| 条」，请求与答案正好相反，且没有任何信号。
2. `0` 是合法答案，不是「没传参数」。以前走 `value or default` 的调用点会把
   「一条都不要」静默换成默认条数（预览给 5 条、主动学习给 50 条），
   `x[-0:]` 更是直接等于「全部」。
3. 判参先于数据短路。空输入 + 坏参数必须仍然报参数错，否则坏参数会被
   空结果掩护掉。
4. 判参先于副作用。构造备份管理器会 mkdir 并写 index.json，向模型发提示词会真花钱，
   这类站点必须先判参再走副作用，否则一次坏参数调用就在磁盘上留下空目录、
   或者把「生成 -1 个」这种自相矛盾的请求发出去。
5. `0` 在不可逆语义下不放行。删备份的 `max_backups=0` 与手滑想打的 10 无从分辨，
   而后果是删光全部备份，所以那类旋钮的下界是 1（见 `minimum=1`）。分块步长
   `chunk_size` 同型：它没有「每块 0 条」这种合法读法。

自 L45 起本文件还覆盖**时长旋钮**的浮点判据 `require_seconds`：主张 1/2/4 同样成立，
只是坏值的症状从「换条数」变成「换一档等待」——见 `TestRequireSeconds`。
L47 加了 `require_positive`（无量纲倍率），L48 加了 `require_ratio`（闭区间比例，
如退避抖动 `jitter`）：比例坏值的症状是「上限从别处冒出来」，见 `TestRequireRatio`。
"""

import json
import math

import numpy as np
import pytest
import yaml

from augmentor.active_learning import ActiveLearningLoop
from augmentor.aggregator import DataAggregator
from augmentor.analytics import analyze_dataset_fast
from augmentor.backup import DatasetBackup, clean_old_backups
from augmentor.cleaner import clean_batch_optimized, extract_keywords
from augmentor.config import ModelConfig
from augmentor.dedup import Deduplicator
from augmentor.exceptions import DataValidationError
from augmentor.expander import DomainExpander
from augmentor.indexer import DatasetView
from augmentor.models.openai_model import OpenAIBackend
from augmentor.preview import PreviewGenerator
from augmentor.quality import QualityScorer
from augmentor.quality_monitor import QualityMonitor
from augmentor.retry import compute_delay
from augmentor.sampler import ActiveSampler
from augmentor.search_enhanced import search_dataset
from augmentor.dataset_ops import DatasetOperations, SampleConfig
from augmentor.streaming import StreamReader, StreamAugmentor
from augmentor.validation import (require_count, require_positive, require_ratio,
                                  require_seconds, require_string,
                                  require_string_list)
from augmentor.vector.faiss import FAISSDB
from augmentor.versioning import VersionManager

ITEMS = [{"instruction": f"如何租房{i}", "output": f"登录官网办理第{i}步"} for i in range(12)]
KEYWORD_TEXT = "租房合同 租房合同 租房合同 押金 押金 中介 中介 房东 合同 签约 违约" * 2
# 覆盖分析要的是「存在比例低于 0.1 的类型」，18 中 + 1 短 + 1 长给出 3 个欠采样类型
# （question_type:other / length:short / length:long），于是 top_k 在 1/2/3 上有区分度。
SEED_CORPUS = ([{"instruction": f"如何办理第{i}步租房合同续签手续并保存凭证材料"} for i in range(18)]
               + [{"instruction": "租房?"},
                  {"instruction": "请详细说明在跨城市搬迁时如何分阶段处理租房合同、押金转移、"
                                  "中介对接、违约条款审查以及后续报销凭证归档的全部流程细节"}])


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

    @pytest.mark.parametrize("value", [0, 1, 20, 21, 10 ** 6, None])
    def test_omitting_maximum_keeps_the_legacy_verdict(self, value):
        """不传 `maximum` 时逐字同答：第四参不得改动既有那八个调用点

        L51 之前 `require_count` 根本没有上界，于是 `config.AugmentationConfig`
        拿不到「`max_retries` 最大 20」这条 —— 校验器有、运行时无（A77）。
        """
        assert require_count("limit", value) == value

    @pytest.mark.parametrize("value", [0, 1, 20, None])
    def test_ceiling_passes_within_range(self, value):
        """边界值 20 放行：默认档 `max_retries=3` 之外，贴着上界也是合法写法"""
        assert require_count("max_retries", value, maximum=20) == value

    @pytest.mark.parametrize("value", [21, 10 ** 6, -1, True, "20", 1.0])
    def test_ceiling_rejects_outside_range(self, value):
        with pytest.raises(DataValidationError, match="max_retries"):
            require_count("max_retries", value, maximum=20)

    def test_ceiling_error_message_names_the_bound_and_the_value(self):
        with pytest.raises(DataValidationError,
                           match=r"max_retries.*20.*1000000"):
            require_count("max_retries", 10 ** 6, maximum=20)

    def test_is_a_value_error_too(self):
        """API 侧靠 `ValueError → 400` 映射，判据必须留在该继承链上"""
        assert issubclass(DataValidationError, ValueError)


class TestRequireSeconds:
    """时长旋钮的浮点判据（`require_count` 的对应物）

    自 L45 起 `augmentation.retry_delay` 真的会流到退避计算，坏值路径从
    「无人读取」变成「有人执行」，所以门槛必须同步建起来。
    """

    @pytest.mark.parametrize("value", [0, 0.0, 0.5, 1, 60.0, float("inf"), None])
    def test_allows_non_negative_numbers_and_none(self, value):
        """`int` 也放行：YAML 的 `retry_delay: 1` 读进来是整数而非浮点"""
        assert require_seconds("retry_delay", value) == value

    @pytest.mark.parametrize("value", [-0.1, -5, "1.0", True, [], {}, float("nan")])
    def test_rejects_negative_non_numeric_and_nan(self, value):
        with pytest.raises(DataValidationError):
            require_seconds("retry_delay", value)

    def test_error_message_names_the_parameter(self):
        with pytest.raises(DataValidationError, match=r"retry_delay.*-2.5"):
            require_seconds("retry_delay", -2.5)

    def test_nan_is_rejected_even_though_yaml_produces_it(self):
        """YAML 的 `.nan` 是合法浮点字面量，模板漏填值就会留下 NaN"""
        assert math.isnan(yaml.safe_load("retry_delay: .nan")["retry_delay"])
        with pytest.raises(DataValidationError, match="NaN"):
            require_seconds("retry_delay", float("nan"))

    def test_compute_delay_keeps_the_good_shape(self):
        """判据搬进 `compute_delay` 之后，合法值必须照原样算出退避"""
        assert compute_delay(1, 5.0, 2.0, 30.0) == 5.0

    @pytest.mark.parametrize("base_delay", [float("nan"), -5.0])
    def test_compute_delay_rejects_what_it_used_to_hide(self, base_delay):
        """L45 立这条时的理由是「深处不响」：退避公式首尾各有一道夹逼

        `min(max_delay, base * factor ** n)` 与尾部 `max(0.0, delay)` 实测把 NaN 收
        成 30.0、把 -5.0 收成 0.0 —— 症状从「报错」降级成「静默换一档等待」。既然
        深处不响，就只能在前台判；L47 起判据搬到了 `compute_delay` 自己的入参处，
        于是同一批坏值现在是抛 `DataValidationError`，不再有机会换形状。
        """
        with pytest.raises(DataValidationError, match="base_delay"):
            compute_delay(1, base_delay, 2.0, 30.0)

    def test_generate_rejects_bad_delay_before_asking_the_model(self):
        """判参先于副作用：坏 retry_delay 不该换来一次真金白银的 API 调用"""
        backend = OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"))
        backend._call_api = lambda prompt: "ok"
        with pytest.raises(DataValidationError, match="retry_delay"):
            backend.generate("p", retry_delay=-1.0)
        assert backend._request_count == 0

    def test_backend_constructor_rejects_bad_defaults(self):
        for kwargs in ({"default_attempts": -1}, {"default_retry_delay": -1.0},
                       {"default_retry_delay": float("nan")}):
            with pytest.raises(DataValidationError):
                OpenAIBackend(ModelConfig(type="openai", api_key="k", model="m"),
                              **kwargs)


class TestRequirePositive:
    """无量纲倍率旋钮（退避 `factor`）的判据，`require_seconds` 的同族

    为什么不复用 `require_seconds`：它的文案是「必须是数值秒数 / 不小于 X 的秒数」，
    拿去做一个没有单位的倍率会把根因说错（L46 刚为「文案指错根因」记过一笔）。
    为什么下界固定成「严格大于 0」而不像两个同族那样可配：这类值只有「每档乘多少」
    一种合法读法，0 与负数在幂运算里都不是「等得更久/更短」而是换形状。实测
    （`retry.compute_delay`，`base_delay=1`、`max_delay=30`，取第 1/2/3 档）：
    `factor=0` → ``[1.0, 0.0, 0.0]``、`-2` → ``[1.0, 0.0, 4.0]``、
    `NaN` → ``[1.0, 30.0, 30.0]``，三条都不抛异常。
    """

    @pytest.mark.parametrize("value", [0.5, 1, 1.0, 2.0, float("inf"), None])
    def test_allows_positive_numbers_and_none(self, value):
        """`0 < factor < 1` 放行：实测退避 ``[1.0, 0.5, 0.25]``，非负、单调、有界"""
        assert require_positive("factor", value) == value

    @pytest.mark.parametrize(
        "value", [0, 0.0, -0.0, -1, -2.0, "2", True, [], float("nan")],
    )
    def test_rejects_zero_negative_non_numeric_and_nan(self, value):
        with pytest.raises(DataValidationError):
            require_positive("factor", value)

    def test_error_message_names_the_parameter(self):
        with pytest.raises(DataValidationError, match=r"factor.*-2\.0"):
            require_positive("factor", -2.0)

    def test_bool_is_not_one(self):
        """`True` 会被幂运算安静读成 1.0（倍率恰好等于「不指数增长」），必须拒"""
        with pytest.raises(DataValidationError, match="factor"):
            require_positive("factor", True)

    def test_zero_factor_would_silently_become_busy_retry(self):
        """判据接在 `compute_delay` 上：0 因子若不判，第 2 档起完全不等待"""
        with pytest.raises(DataValidationError, match="factor"):
            compute_delay(2, 1.0, 0.0, 30.0)

    def test_negative_factor_would_break_monotonicity(self):
        with pytest.raises(DataValidationError, match="factor"):
            compute_delay(3, 1.0, -2.0, 30.0)

    def test_inf_is_bounded_by_the_ceiling_so_it_stays_legal(self):
        """与 `require_seconds` 同口径：`+inf` 一路上限是可预期行为（实测 30.0）"""
        assert compute_delay(2, 1.0, float("inf"), 30.0) == 30.0


class TestRequireRatio:
    """闭区间比例旋钮（退避抖动 `jitter`）的判据，判据族第四员

    为什么要有第四员而不是复用 `require_seconds`：抖动的单位是「乘在自己那一档上
    的比例」，不是秒；文案写「必须是数值秒数」会把根因说错（L46 为这件事记过一笔）。
    为什么上界也要判（`require_seconds` / `require_positive` 都只管下界）：抖动是加在
    `max_delay` 那道夹**之后**的，所以它一旦大到超出区间，调用方以为的等待上限就不再
    是上限。实测同一贴顶档位（`base_delay=20`、`attempt=3` ⇒ `min(30, 80)` 正好夹在 30.0，
    各抽 500 次，Temp `l48_probe2.py`）：
    `jitter=0.5` → 30.02 ~ 44.94 s、`1.0` → 30.05 ~ 59.87 s、`2.0` → 30.09 ~ 89.74 s、
    `50.0` → 32.27 ~ **1523.54 s**，四档越过 30 s 的比例都是 1.000。判在 0-1 之间可把
    这一支最坏等待证死在 2 × `max_delay`。
    下界的 `-1` 与 `NaN` 不判也行吗？不行：实测两者与「根本没传 `jitter`」**逐字同答**
    （都交 ``[1.0, 2.0, 4.0]``，因为过不了 `if jitter > 0` 那道门）——「传了参数等于没传」
    是 L33 禁掉的 `or` 回落同族的静默，照样判。
    """

    @pytest.mark.parametrize("value", [0, 0.0, 0.5, 1, 1.0, None])
    def test_allows_in_range_numbers_and_none(self, value):
        """`0`（不抖）与 `1`（最大抖动）都是合法档位，`None` 表示「没传」"""
        assert require_ratio("jitter", value) == value

    @pytest.mark.parametrize(
        "value", [-0.1, -1.0, 1.0000001, 1.5, 2.0, 50.0, "0.5", True, [],
                  float("nan")],
    )
    def test_rejects_out_of_range_non_numeric_and_nan(self, value):
        with pytest.raises(DataValidationError):
            require_ratio("jitter", value)

    def test_error_message_names_the_parameter_and_the_interval(self):
        with pytest.raises(DataValidationError, match=r"jitter.*0\.0 到 1\.0.*1\.5"):
            require_ratio("jitter", 1.5)

    def test_bool_is_max_jitter_not_one(self):
        """`True` 在算术里读成 1.0，也就是「直接把抖动开到最大」——判据落地前实测它与
        `1.0` 抽样逐字同答（同一种子五次：7.38 / 4.54 / 7.82 / 4.95 / 4.94），所以它不是
        「大概是想要抖动吧」的善意笔误，而是把最坏档位悄悄拉满，必须单独判掉。
        """
        with pytest.raises(DataValidationError, match="jitter"):
            require_ratio("jitter", True)

    @pytest.mark.parametrize("value", [-5.0, float("nan")])
    def test_bad_jitter_used_to_answer_exactly_like_no_jitter(self, value):
        """本条钉住「静默」这个旧症状：判据落地前 `-5.0` 与 `NaN` 都交 ``[1.0, 2.0, 4.0]``，
        与「根本没传 `jitter`」一字不差（过不了 `if jitter > 0` 那道门）。
        """
        with pytest.raises(DataValidationError, match="jitter"):
            compute_delay(3, 1.0, 2.0, 30.0, jitter=value)

    @pytest.mark.parametrize("bounds", [(1.0, 3.0), (0.0, 0.2)])
    def test_interval_is_configurable_for_other_ratio_knobs(self, bounds):
        """族里其它比例旋钮（如未来的比例阈值）可复用同一判据，不必再造文案"""
        lo, hi = bounds
        assert require_ratio("ratio", (lo + hi) / 2, lo, hi) == (lo + hi) / 2
        with pytest.raises(DataValidationError, match=f"ratio.*{lo} 到 {hi}"):
            require_ratio("ratio", lo - 1.0, lo, hi)

    def test_none_means_not_passed_and_short_circuits_before_type_check(self):
        """`None` 直接放行：调用方（`with_retries` 的默认值）绝大多数时候不传抖动，
        这一支不构造异常也不做 isinstance，实测 59.7 ns vs 传值 235.3 ns。
        """
        assert require_ratio("jitter", None) is None


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


class TestSampleSizeKnob:
    """`DatasetOperations.sample` 的 `config.size`：`if config.size:` + 三条 method"""

    def test_negative_size_is_rejected_on_every_method(self):
        """以前 random / stratified 抛裸 `ValueError: Sample larger than population`
        （经 API 是 500），systematic 更是把 -1 读成「丢掉末条」交出 6901 条"""
        for method in ("random", "systematic", "stratified"):
            with pytest.raises(DataValidationError, match="size"):
                DatasetOperations().sample(ITEMS, SampleConfig(method=method, size=-1))

    def test_zero_size_is_not_read_as_no_size(self):
        """`if config.size:` 把 0 当成「没传参数」→ 采 0 条交付整份数据集
        （真实 6902 条实测 size=0 → 6902 条，三种 method 一样）"""
        for method in ("random", "systematic", "stratified"):
            assert DatasetOperations().sample(ITEMS, SampleConfig(method=method, size=0)) == []

    def test_none_still_means_whole_dataset(self):
        """反向护栏：判据不许把「没给 size」一起改掉"""
        assert len(DatasetOperations().sample(ITEMS, SampleConfig(size=None))) == len(ITEMS)
        assert len(DatasetOperations().sample(ITEMS)) == len(ITEMS)

    def test_valid_size_still_truncates(self):
        assert len(DatasetOperations().sample(ITEMS, SampleConfig(size=5))) == 5
        assert len(DatasetOperations().sample(ITEMS, SampleConfig(size=999))) == len(ITEMS)

    def test_zero_sample_size_short_circuits_before_the_method_dispatch(self):
        """`ratio` 小到 int() 归零时 systematic 那支是 `len(items) // 0`：
        实测 HEAD 在真实语料上 ratio=0.0001 → 裸 ZeroDivisionError，
        同一个 0 在三种 method 下给出三种后果（0 条 / 0 条 / 除零）"""
        for method in ("random", "systematic", "stratified"):
            ops = DatasetOperations().sample(ITEMS, SampleConfig(method=method, ratio=0.0001))
            assert ops == []


class TestHeadTailSliceKnobs:
    """`items[:n]` 与 `items[-n:]`：同一对表达式在 dataset_ops 与 DatasetView 各写一遍"""

    def test_negative_n_is_rejected(self):
        """`[-1:]` 不是「倒数第 1 条」的反面，而是「丢掉第 1 条」：实测真实语料
        head(-1) → 6901 条、tail(-1) → 6901 条"""
        ops = DatasetOperations()
        with pytest.raises(DataValidationError, match="n"):
            ops.head(ITEMS, -1)
        with pytest.raises(DataValidationError, match="n"):
            ops.tail(ITEMS, -1)

    def test_tail_zero_is_empty_not_everything(self):
        """`[-0:] == [0:]`：实测 HEAD 里 tail(0) 在真实语料上返回全部 6902 条"""
        ops = DatasetOperations()
        assert ops.head(ITEMS, 0) == []
        assert ops.tail(ITEMS, 0) == []

    def test_view_head_and_tail_share_the_rule(self):
        """视图版实测（100 条）：head(-1)→99、tail(0)→100、sample(-1)→裸 ValueError"""
        view = DatasetView(SEED_CORPUS, "probe")
        with pytest.raises(DataValidationError, match="n"):
            view.head(-1)
        with pytest.raises(DataValidationError, match="n"):
            view.tail(-1)
        with pytest.raises(DataValidationError, match="n"):
            view.sample(-1)
        with pytest.raises(DataValidationError, match="n"):
            view.head("3")

    def test_view_tail_zero_is_empty(self):
        assert len(DatasetView(SEED_CORPUS, "probe").tail(0)) == 0

    def test_valid_window_still_slices(self):
        """反向护栏：正常路径照旧，且 head/tail 互为补集"""
        ops = DatasetOperations()
        assert ops.head(ITEMS, 3) == ITEMS[:3]
        assert ops.tail(ITEMS, 3) == ITEMS[-3:]
        view = DatasetView(SEED_CORPUS, "probe")
        assert len(view.head(2)) == 2
        assert len(view.sample(2)) == 2
        assert len(view.sample(999)) == len(SEED_CORPUS)


class TestBatchStepSize:
    """`clean_batch_optimized` 的 `batch_size`：它是 `range()` 的步长"""

    def test_zero_step_is_a_client_error_not_a_crash(self):
        """实测 HEAD 抛裸 `ValueError: range() arg 3 must not be zero`，
        经 API 那道映射不成 400"""
        with pytest.raises(DataValidationError, match="batch_size"):
            clean_batch_optimized(ITEMS, batch_size=0)

    def test_negative_step_silently_returns_an_empty_dataset(self):
        """实测 HEAD：10 条进 0 条出，清洗报告仍写 original_count=10 ——
        调用方按报告以为洗干净了"""
        with pytest.raises(DataValidationError, match="batch_size"):
            clean_batch_optimized(ITEMS, batch_size=-1)

    def test_bad_step_fails_before_any_cleaner_is_built(self):
        """判参先于副作用：坏步长不得留下半成品产物"""
        with pytest.raises(DataValidationError, match="batch_size"):
            clean_batch_optimized([], batch_size=0)

    def test_smallest_legal_step_produces_the_same_rows(self):
        """反向护栏：步长只该影响内存峰值，不该影响产物条数"""
        rows, report = clean_batch_optimized(ITEMS, batch_size=1)
        assert len(rows) == len(ITEMS)
        assert report.original_count == len(ITEMS)


class TestSeedRecommendationTopK:
    """`recommend_seeds` 的 `top_k`：同一表达式裁了两遍"""

    def test_the_corpus_actually_has_underrepresented_types(self):
        """用例前提：欠采样类型有 3 个，否则「裁到 2」与「裁到 3」无从分辨"""
        assert len(ActiveSampler().recommend_seeds(SEED_CORPUS, top_k=3).recommended_seeds) == 3

    def test_negative_top_k_is_rejected(self):
        """实测真实 6902 条：-1 → 2 个种子、-3 → 0 个，同一个负数随数据换答案"""
        with pytest.raises(DataValidationError, match="top_k"):
            ActiveSampler().recommend_seeds(SEED_CORPUS, top_k=-1)

    def test_zero_top_k_returns_no_seeds(self):
        assert ActiveSampler().recommend_seeds(SEED_CORPUS, top_k=0).recommended_seeds == []

    def test_bad_knob_fails_on_an_empty_dataset(self):
        """判参先于 `if not items:` 短路"""
        with pytest.raises(DataValidationError, match="top_k"):
            ActiveSampler().recommend_seeds([], top_k=-1)

    def test_valid_top_k_still_clamps_to_the_ceiling(self):
        assert len(ActiveSampler().recommend_seeds(SEED_CORPUS, top_k=1).recommended_seeds) == 1
        assert len(ActiveSampler().recommend_seeds(SEED_CORPUS, top_k=99).recommended_seeds) == 3


class TestDiversityWindowSize:
    """`QualityScorer.diversity_sample_size`：`existing_generated[:n]` 的窗口"""

    def test_zero_window_declares_a_duplicate_fully_diverse(self):
        """承 A41：实测候选与参照逐字相同时，窗口 0 → 空参照 → 判成「完全多样」，
        diversity 0.0 → 1.0，总分 0.063492 → 0.492063（+0.428571）"""
        with pytest.raises(DataValidationError, match="diversity_sample_size"):
            QualityScorer(diversity_sample_size=0)

    def test_negative_window_is_rejected_too(self):
        with pytest.raises(DataValidationError, match="diversity_sample_size"):
            QualityScorer(diversity_sample_size=-1)

    def test_smallest_window_still_scores(self):
        """反向护栏：下界 1 不得让正常打分路径换口径——窗口 1 与默认 30
        在这份 2 条参照上都覆盖全部参照"""
        reference = ["甲乙丙丁戊己", "戊己庚辛壬癸"]
        narrow = QualityScorer(diversity_sample_size=1).score(
            "甲乙丙丁戊己", "甲乙丙丁戊己", "戊己庚辛壬癸",
            existing_generated=reference, include_semantic=False)
        wide = QualityScorer().score("甲乙丙丁戊己", "甲乙丙丁戊己", "戊己庚辛壬癸",
                                     existing_generated=reference, include_semantic=False)
        assert narrow.diversity == wide.diversity == 0.0
        assert narrow.total_score == wide.total_score


class TestBackupRetention:
    """`clean_old_backups` 的 `max_backups`：`sorted[:len - max_backups]`"""

    @staticmethod
    def _sandbox(tmp_path, count=5):
        source = tmp_path / "src.json"
        source.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
        manager = DatasetBackup(str(tmp_path / "bk"))
        for i in range(count):
            manager.backup(str(source), name=f"bk{i}")
        return tmp_path / "bk"

    @pytest.mark.parametrize("max_backups", [-1, -999])
    def test_negative_retention_deletes_every_backup(self, tmp_path, max_backups):
        """实测 Temp 沙箱 5 个备份：-1 与 -999 都删到剩 0 个，日志却写「保留最新 -1 个」"""
        directory = self._sandbox(tmp_path)
        with pytest.raises(DataValidationError, match="max_backups"):
            clean_old_backups(str(directory), max_backups)
        assert len(list(directory.glob("bk*.json"))) == 5

    def test_zero_retention_is_not_a_legal_request(self, tmp_path):
        """「保留 0 个」与手滑想打的 10 无从分辨，而后果不可逆，故 minimum=1。
        实测 HEAD：0 → 5 个备份全删"""
        directory = self._sandbox(tmp_path)
        with pytest.raises(DataValidationError, match="max_backups"):
            clean_old_backups(str(directory), 0)
        assert len(list(directory.glob("bk*.json"))) == 5

    def test_guard_runs_before_the_directory_is_created(self, tmp_path):
        """判参先于副作用：`DatasetBackup(backup_dir)` 会 mkdir 并写 index.json，
        一次坏参数调用不该在磁盘上留下空目录"""
        target = tmp_path / "never-created"
        with pytest.raises(DataValidationError, match="max_backups"):
            clean_old_backups(str(target), -1)
        assert not target.exists()

    def test_valid_retention_still_prunes(self, tmp_path):
        """反向护栏：5 个备份、保留 2 → 删 3 个"""
        directory = self._sandbox(tmp_path)
        assert clean_old_backups(str(directory), 2) == 3
        assert len(list(directory.glob("bk*.json"))) == 2

    def test_retention_above_the_current_count_is_a_noop(self, tmp_path):
        directory = self._sandbox(tmp_path)
        assert clean_old_backups(str(directory), 5) == 0
        assert len(list(directory.glob("bk*.json"))) == 5


class StubTopicBackend:
    """`DomainExpander` 只调用 `.generate(prompt)`：这里不联网，只记提示词，
    并固定交回 20 条候选，好让 `[:num_topics]` 的末位裁剪看得出来。"""

    def __init__(self):
        self.prompts = []

    def generate(self, prompt, **kwargs):
        self.prompts.append(prompt)
        return json.dumps([f"扩展主题{i}" for i in range(20)], ensure_ascii=False)


EXPANDER_CORPUS = [{"instruction": "晨阳计划第1问"}, {"instruction": "租金相关第2问"},
                   {"instruction": "押金相关第3问"}, {"instruction": "合同相关第4问"},
                   {"instruction": "维修服务第5问"}]

# 三个源、每源 10 条、内容互不相同（否则 `_key_of` 去重会让后两个源一条都进不来）。
# 12 是 3 的整数倍，于是每源配额 4 条，不受取整损失影响。
AGG_SOURCES = {f"s{i}": [{"instruction": f"第{i}号来源第{j}问租房流程",
                          "output": f"答{i}-{j}"} for j in range(10)] for i in range(3)}


class TestStreamChunkSize:
    """`StreamReader` 的分块步长：`if len(chunk) >= size`"""

    @pytest.fixture
    def corpus(self, tmp_path):
        path = tmp_path / "stream.jsonl"
        path.write_text("".join(json.dumps(i, ensure_ascii=False) + "\n" for i in ITEMS),
                        encoding="utf-8")
        return path

    def test_zero_step_is_rejected_because_it_is_a_step_not_a_count(self, corpus):
        """承主张 5：步长没有「每块 0 条」的合法读法。实测真实 6902 条数据集，
        HEAD 把 0 静默当成 1 —— 7 块变 6902 块，端到端 64.0 ms → 103.2 ms（+61%）"""
        with pytest.raises(DataValidationError, match="chunk_size"):
            StreamReader(str(corpus), 0)

    @pytest.mark.parametrize("size", [-1, -8])
    def test_negative_step_is_rejected(self, corpus, size):
        """实测 HEAD：-1 与 0 的产物逐条相同（6902 块 / 6902 条），全程无信号"""
        with pytest.raises(DataValidationError, match="chunk_size"):
            StreamReader(str(corpus), size)

    @pytest.mark.parametrize("size", ["2", 1.5, True])
    def test_non_integer_step_is_rejected(self, corpus, size):
        """`"2"` 在 HEAD 会走到 `len(chunk) >= "2"`，报裸 TypeError 而不是领域错误"""
        with pytest.raises(DataValidationError, match="chunk_size"):
            StreamReader(str(corpus), size)

    def test_smallest_legal_step_is_one(self, corpus):
        """反向护栏：下界 1 不得把「逐条切」一起拒掉"""
        assert [len(c) for c in StreamReader(str(corpus), 1).read_chunks()] == [1] * len(ITEMS)

    def test_valid_step_still_slices(self, corpus):
        assert [len(c) for c in StreamReader(str(corpus), 2).read_chunks()] == [2] * 6

    def test_the_error_arrives_at_construction_not_after_a_full_read(self, tmp_path, corpus):
        """判参先于花钱的副作用：坏步长在 `__init__` 就报，不是读完一整份才报。
        实测 HEAD 的 `StreamWriter.__init__` 并不落盘（`open()` 在 `__enter__`），
        所以可证的是「构造期即拒 + 磁盘无产物」，reader/writer 的先后顺序不可测"""
        out = tmp_path / "never-written.jsonl"
        with pytest.raises(DataValidationError, match="chunk_size"):
            StreamAugmentor(str(corpus), str(out), lambda items: items, chunk_size=0)
        assert not out.exists()


class TestAggregateTargetSize:
    """`aggregate_weighted` 的 `target_size`：`int(target_size * 权重占比)`"""

    def test_negative_target_is_rejected(self):
        """实测 HEAD：-1 与 -50 都静默产出 0 条，`aggregated_count` 照报 0，
        与「三源皆空」同形"""
        with pytest.raises(DataValidationError, match="target_size"):
            DataAggregator().aggregate_weighted(AGG_SOURCES, target_size=-1)

    def test_zero_target_is_a_legal_request(self):
        """0 条 = 「只要总数，不要条目」，`source_counts` 仍如实"""
        result = DataAggregator().aggregate_weighted(AGG_SOURCES, target_size=0)
        assert result.aggregated == []
        assert result.source_counts == {"s0": 10, "s1": 10, "s2": 10}

    @pytest.mark.parametrize("size", ["10", 1.5, True])
    def test_non_integer_target_names_the_knob(self, size):
        """HEAD 是裸 TypeError（`can't multiply sequence by non-int of type 'float'`），
        绕开本模块的错误口径"""
        with pytest.raises(DataValidationError, match="target_size"):
            DataAggregator().aggregate_weighted(AGG_SOURCES, target_size=size)

    def test_an_explicit_none_means_the_default(self):
        """API 的 `AggregateRequest.target_size` 是 `Optional[int] = None`，路由把
        「未提供」原样透传 → HEAD 在 `int(None * 占比)` 处炸裸 TypeError，
        即 `POST /api/dataset/aggregate` 走 weighted 又省略这个可选字段就是 500"""
        with_default = DataAggregator().aggregate_weighted(AGG_SOURCES)
        via_dispatch = DataAggregator().aggregate(AGG_SOURCES, "weighted", target_size=None)
        assert len(with_default.aggregated) == len(via_dispatch.aggregated) == 30

    def test_valid_target_still_splits_by_weight(self):
        """反向护栏：12 条按三等分 → 每源 4 条，判据不得顺手改掉配额算式"""
        result = DataAggregator().aggregate_weighted(AGG_SOURCES, target_size=12)
        assert len(result.aggregated) == 12


class TestExpanderTopicKnobs:
    """`DomainExpander` 的三个「生成 N 个」旋钮，全部落到 `candidates[:N]`"""

    def test_negative_num_topics_is_rejected(self):
        """实测桩后端 20 条候选：-1 → 19 项（`[:-1]` 丢掉末位候选）"""
        with pytest.raises(DataValidationError, match="num_topics"):
            DomainExpander(StubTopicBackend()).expand(EXPANDER_CORPUS, num_topics=-1)

    def test_the_model_is_never_asked_for_a_negative_number(self):
        """判参先于花钱的副作用：实测 HEAD 会把 `生成 -1 个相似的主题` 原样发进
        提示词（桩后端确实收到 1 次调用）"""
        backend = StubTopicBackend()
        with pytest.raises(DataValidationError, match="num_topics"):
            DomainExpander(backend).expand(EXPANDER_CORPUS, num_topics=-1)
        assert backend.prompts == []

    @pytest.mark.parametrize("strategy", ["similar", "related", "scenario"])
    def test_all_three_strategies_share_the_throat(self, strategy):
        """三条私有路径各写了一次 `[:num_topics]`，判据必须在公共入口收口"""
        backend = StubTopicBackend()
        with pytest.raises(DataValidationError, match="num_topics"):
            DomainExpander(backend).expand(EXPANDER_CORPUS, strategy=strategy, num_topics=-2)
        assert backend.prompts == []

    def test_zero_topics_is_a_legal_request(self):
        result = DomainExpander(StubTopicBackend()).expand(EXPANDER_CORPUS, num_topics=0)
        assert result.expanded_topics == []

    def test_valid_num_topics_still_truncates(self):
        backend = StubTopicBackend()
        result = DomainExpander(backend).expand(EXPANDER_CORPUS, num_topics=3)
        assert len(result.expanded_topics) == 3
        assert "生成 3 个" in backend.prompts[0]

    def test_batch_expand_knob_is_rejected(self):
        """实测 HEAD：`topics_per_strategy=-1` 在串行路径给 19 项"""
        loop = DomainExpander(StubTopicBackend(), max_workers=1)
        with pytest.raises(DataValidationError, match="topics_per_strategy"):
            loop.batch_expand(EXPANDER_CORPUS, strategies=["similar"],
                              topics_per_strategy=-1, use_parallel=False)

    def test_parallel_batch_expand_fails_before_the_pool_opens(self):
        """实测 HEAD：并行路径 3 个策略各 19 项 = 57 项，坏旋钮被线程池吞掉"""
        loop = DomainExpander(StubTopicBackend(), max_workers=3)
        with pytest.raises(DataValidationError, match="topics_per_strategy"):
            loop.batch_expand(EXPANDER_CORPUS, topics_per_strategy=-1)

    def test_seeds_per_topic_knob_is_rejected(self):
        """同一个切片家族：`seeds[:num_questions]`，实测 HEAD -1 → 19 条种子"""
        loop = DomainExpander(StubTopicBackend(), max_workers=1)
        with pytest.raises(DataValidationError, match="num_questions_per_topic"):
            loop.generate_seeds_from_topics([{"topic": "押金"}],
                                           num_questions_per_topic=-1, use_parallel=False)

    def test_seeds_knob_fails_before_the_model_is_called(self):
        backend = StubTopicBackend()
        loop = DomainExpander(backend, max_workers=2)
        with pytest.raises(DataValidationError, match="num_questions_per_topic"):
            loop.generate_seeds_from_topics(
                [{"topic": "押金"}, {"topic": "合同"}], num_questions_per_topic=-1)
        assert backend.prompts == []


class TestVectorSearchTopK:
    """`FAISSDB.search(top_k)`：`k = min(top_k, len(ids))` 后连吃两次末位裁剪"""

    @staticmethod
    def _four_vectors():
        return ([np.eye(4, dtype=np.float32)[i] for i in range(4)],
                [{"i": i} for i in range(4)], [f"id{i}" for i in range(4)])

    @pytest.fixture
    def db(self, tmp_path):
        store = FAISSDB(dimension=4, storage_dir=str(tmp_path / "faiss"))
        vectors, metadata, ids = self._four_vectors()
        store.add_vectors(vectors, metadata, ids)
        return store

    def test_negative_top_k_is_rejected(self, db):
        """实测 4 条向量：-1 → 2 条命中（`argsort(...)[:-1]` 再 `best_pairs[:-1]`），
        -3 → 0 条。同一个旋钮，两种错法，都没有信号"""
        with pytest.raises(DataValidationError, match="top_k"):
            db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=-1)

    def test_bad_knob_fails_on_an_empty_database(self, tmp_path):
        """判参先于 `if not self._ids: return []`，否则坏旋钮被空库掩护掉"""
        empty = FAISSDB(dimension=4, storage_dir=str(tmp_path / "empty"))
        with pytest.raises(DataValidationError, match="top_k"):
            empty.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=-1)

    @pytest.mark.parametrize("size", ["5", 2.5, True])
    def test_non_integer_top_k_is_rejected(self, db, size):
        """HEAD：字符串在 `min()` 处比大小 → 裸 TypeError"""
        with pytest.raises(DataValidationError, match="top_k"):
            db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=size)

    def test_zero_top_k_returns_no_rows(self, db):
        assert db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=0) == []

    def test_valid_top_k_above_the_count_still_caps(self, db):
        assert len(db.search(np.array([1, 0, 0, 0], dtype=np.float32), top_k=100)) == 4


class TestRequireSecondsUpperBound:
    """`require_seconds` 的第四参 `maximum`：只给「旋钮本身就是封顶」的那些参（L49 / A73）

    为什么默认不判上界（族里另外三员也都不判）：`retry_delay` 这类值越界只是「等得久」，
    实现里另有 `min(max_delay, …)` 兜着，而它的天花板（≤ 60）是**校验器独有**的口径
    （L45 记过一笔）。`with_retries(max_retry_wait=…)` 是这一族的例外：这个参数自己
    **就是**那道夹逼，它自己越界时无处可夹 —— 加判据前实测 `inf` 让 `Retry-After: 3000`
    原样睡着 3000 s。所以对这一类参，上界是承诺本身，必须放在运行时入口。
    """

    @pytest.mark.parametrize("value", [301.0, 1e9, float("inf")])
    def test_omitting_maximum_keeps_the_legacy_verdict(self, value):
        """不传 `maximum` 时逐字同答：新判据不得改动既有全部调用点"""
        assert require_seconds("retry_delay", value) == value

    @pytest.mark.parametrize("value", [0, 0.0, 45, 45.0, 300.0, 300, None])
    def test_in_range_values_pass(self, value):
        """`int` 写法与边界值 300.0 都放行（默认档就贴在边界上）"""
        assert require_seconds("max_retry_wait", value, maximum=300.0) == value

    @pytest.mark.parametrize("value", [300.0000001, 301.0, 1e9, float("inf"),
                                       -1.0, True, "300", float("nan")])
    def test_out_of_range_values_rejected(self, value):
        with pytest.raises(DataValidationError):
            require_seconds("max_retry_wait", value, maximum=300.0)

    def test_error_message_names_the_bound_and_the_value(self):
        with pytest.raises(DataValidationError, match=r"max_retry_wait.*300\.0.*301"):
            require_seconds("max_retry_wait", 301.0, maximum=300.0)


class TestRequireString:
    """字符串旋钮（L51 / A80）：配置里的 `host` / `static_dir` 这类单值

    与标量家族最关键的区别是**不放行 `None`**：函数入参有「没传」这一态，配置
    字段没有 —— YAML 写了键却没给值，读进来就是 `None`，下游 `Path(str(p))`
    会把它变成一个**名叫 `None`** 的目录。
    """

    @pytest.mark.parametrize("value", ["0.0.0.0", "web/dist", "/", "a"])
    def test_non_empty_strings_pass(self, value):
        assert require_string("host", value) == value

    @pytest.mark.parametrize("value", [None, "", 8000, True, [], {}, 1.5])
    def test_rejects_none_empty_and_non_string(self, value):
        """`None` 与 `""` 都在这里判掉：前者是「写了键没给值」，后者是无意义值"""
        with pytest.raises(DataValidationError):
            require_string("host", value)

    def test_error_message_distinguishes_empty_from_wrong_type(self):
        """两种错法的修法不同（改类型 / 补值），报错不能同一句话"""
        with pytest.raises(DataValidationError, match=r"host.*不能是空字符串"):
            require_string("host", "")
        with pytest.raises(DataValidationError, match=r"host.*必须是字符串.*NoneType"):
            require_string("host", None)


class TestRequireStringList:
    """字符串列表旋钮（L51 / A80 + A83）：目录白名单与跨源白名单

    立项用例是 `cors_origins` 写成标量。实测（Temp `l51q/probe2.py` 与
    `out2b.txt`，starlette 1.6.0 与 1.2.1 同形）：`allow_origins` 是字符串时
    `origin in allow_origins` 走**子串**匹配，配置 `"https://api.corp.example"`
    会放行 `https://api.corp` 与 `https://api` 两个别的主机。访问控制项被写成标量
    就静默变成前缀匹配 —— 收紧意图拿到放宽结果，这是本判据存在的最硬理由。
    """

    @pytest.mark.parametrize("value", [
        [], ["data"], ["data", "data/sub"], ["/api/health", "/docs"],
    ])
    def test_lists_of_non_empty_strings_pass(self, value):
        """空列表放行：`cors_origins: []` 是 L50 的出厂默认，是显式选择不是漏写"""
        assert require_string_list("data_roots", value) == value

    @pytest.mark.parametrize("value", [
        "data", "", None, 8000, True, {"a": 1}, [None], [123], [True], [""],
        ["data", ""], ["data", None],
    ])
    def test_rejects_scalar_and_bad_elements(self, value):
        with pytest.raises(DataValidationError):
            require_string_list("data_roots", value)

    def test_scalar_message_tells_the_user_how_to_write_yaml(self):
        """报错要给出修法：`data_roots: data` 的作者多半以为自己在写列表"""
        with pytest.raises(DataValidationError,
                           match=r"data_roots.*必须是列表.*序列.*str"):
            require_string_list("data_roots", "data")

    def test_element_error_names_the_offending_item(self):
        with pytest.raises(DataValidationError,
                           match=r"data_roots.*每一项必须是字符串.*None"):
            require_string_list("data_roots", [None])

    def test_content_semantics_stay_out(self):
        """只判形状不判语义：非法 URL / 不存在的路径都由各消费点自己管"""
        assert require_string_list("cors_origins", ["not a url", "/no/such/dir"]) == \
            ["not a url", "/no/such/dir"]

