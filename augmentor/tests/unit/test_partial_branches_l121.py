# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L121（A205）：偏支逐支审计——analytics / config_validator / sampler 三模块

**可达、补测（本文件）**
- analytics.py 182->181      停用词夹在 bigram 对中 ⇒ `if wl[i] not in sw and wl[i+1] not in sw:`
                             假支（成对不成立则跳过该对）
- analytics.py 192->190      纯空白文本 ⇒ `_get_sentences` 返空 ⇒ `if sentences:` 假支
- analytics.py 266->276      数据集落在 [100, 1000) ⇒ `if <100 / elif >=1000` 两档全不中，
                             数据量洞察整段跳过
- analytics.py 422->418      重复候选里有一方 instruction 为空 ⇒ `if inst_i and inst_j:` 假支
- config_validator.py 753->752  列表项里的环境变量引用**已设置** ⇒ `if var not in os.environ:`
                             假支（不设才告警，设了走回边）
- config_validator.py 759->exit  `_validate_dict` 喂真字典 ⇒ 类型守卫假支（直调白盒）
- config_validator.py 764->exit  `_validate_list` 喂真列表 ⇒ 类型守卫假支（直调白盒）
- sampler.py 279->282        覆盖充分的数据集 ⇒ `underrepresented` 为空 ⇒
                             `if underrepresented:` 假支

**判定为结构性不可达、记档不测（不许用绕过入口的构造硬凑）**
- analytics.py 349->353     `_calculate_quality_score` 入口 :325 已对空 `_items` 早退
                             （`return 0.0`），走到 :349 时 `self._items` 恒非空 ⇒
                             `if self._items:` 假支是冗余防御（同 quality.py 222->230 一族）。
- sampler.py 41->exit       `self._model` 只在 `__init__` 置 None，`_load_model` 实际设置的是
                             `_tfidf` / `_use_sklearn`，**从不**回填 `_model` ⇒
                             `if self._model is None:` 恒真，假支是死守卫（置上再测等于
                             给产品代码造一个只活测试里存在的赋值，违反本轮纪律）。
- sampler.py 265->260      `recommend_seeds` 里 `elif category == "length":` 的假支要
                             category 既非 question_type 也非 length 才走到，而
                             `identify_underrepresented`（:216 / :221）的产出口径只有这两个
                             词 ⇒ 公共路径 category 词表封闭，elif 恒真支，假支不可达。
"""

from augmentor.analytics import DatasetAnalyzer
from augmentor.config_validator import ConfigValidator, ValidationResult
from augmentor.sampler import ActiveSampler


# ---------------------------------------------------------------------------
# analytics.py
# ---------------------------------------------------------------------------

class TestAnalyticsBigramStopwordPair:
    """analytics.py 182->181：bigram 对中有一方是停用词 ⇒ 该对被跳过"""

    def test_stopword_inside_pair_yields_no_bigram(self):
        # 「房间 的 价格」分词为 房间/的/价格，「的」是停用词：
        # 两对（房间,的）与（的,价格）全部被 :182 的判据挡掉 ⇒ 无 bigram
        report = DatasetAnalyzer([{"instruction": "房间 的 价格", "output": "答"}]).analyze()
        stats = report.field_statistics["instruction"]
        assert stats.top_bigrams == []
        # 对照组：无停用词的连词对必须产出 bigram（判据写反/删掉时此断言先红）
        report2 = DatasetAnalyzer([{"instruction": "房间 价格", "output": "答"}]).analyze()
        assert report2.field_statistics["instruction"].top_bigrams[0] == ("房间价格", 1)


class TestAnalyticsBlankTextHasNoSentences:
    """analytics.py 192->190：纯空白文本句法切分后无句子 ⇒ `if sentences:` 假支"""

    def test_blank_text_contributes_no_sentence(self):
        report = DatasetAnalyzer([{"instruction": "   ", "output": "答"}]).analyze()
        stats = report.field_statistics["instruction"]
        assert stats.avg_sentence_length == 0
        assert stats.total_words == 0


class TestAnalyticsMidSizeDatasetSkipsSizeInsight:
    """analytics.py 266->276：100 ≤ n < 1000 ⇒ 「数据量较少/充足」两档都不触发"""

    def test_size_insight_absent_for_150_items(self):
        items = [{"instruction": f"问题{i}", "output": f"答案{i}"} for i in range(150)]
        report = DatasetAnalyzer(items).analyze()
        assert report.dataset_size == 150
        categories = [insight.category for insight in report.insights]
        assert "数据量" not in categories


class TestAnalyticsDuplicateCandidateWithEmptyInstruction:
    """analytics.py 422->418：一方 instruction 为空的 pair 被 `if inst_i and inst_j:` 挡掉

    用 threshold=0.0 把「挡掉」变成可观测行为：不挡的话 (0,1) 以 0.0 相似度入围，
    判据删掉/写反时结果会从 1 条变成 2 条。
    """

    def test_empty_instruction_pair_excluded(self):
        items = [
            {"output": "x"},
            {"instruction": "你好", "output": "好"},
            {"instruction": "你好", "output": "好"},
        ]
        candidates = DatasetAnalyzer(items).get_duplicate_candidates(threshold=0.0)
        # (1,2) 相似度 1.0 入围；(0,1) 因 instruction 为空被 :422 假支跳过
        assert candidates == [(1, 2, 1.0)]


# ---------------------------------------------------------------------------
# config_validator.py
# ---------------------------------------------------------------------------

class TestConfigValidatorEnvRefInListItem:
    """config_validator.py 753->752：列表项里的 `${VAR}` 引用，VAR 已设置 ⇒ 不走告警支"""

    def test_set_env_ref_in_list_item_raises_no_warning(self, monkeypatch):
        monkeypatch.setenv("L121_SET_REF", "x")
        result = ConfigValidator().validate_config({
            "models": {"default": "ernie"},
            "api_keys": ["${L121_SET_REF}"],
        })
        assert not any("L121_SET_REF" in w.message for w in result.warnings)

    def test_unset_env_ref_in_list_item_raises_warning(self, monkeypatch):
        # 反例对照：同一个位置、变量没设 ⇒ 753 真支必须告警（假支测试不是空转的证据）
        monkeypatch.delenv("L121_UNSET_REF", raising=False)
        result = ConfigValidator().validate_config({
            "models": {"default": "ernie"},
            "api_keys": ["${L121_UNSET_REF}"],
        })
        assert any("L121_UNSET_REF" in w.message for w in result.warnings)


class TestConfigValidatorTypeGuardsPositive:
    """config_validator.py 759->exit / 764->exit：类型守卫喂**正确**类型 ⇒ 零错误

    记档：`__init__` 里登记的 `self._validators` 分发表（dict→_validate_dict 等六格）
    在 `_validate_known_fields` 里没有走查，类型检查是就地 isinstance 做的 ⇒ 这六个
    `_validate_*` 方法当前**只能被测试直调**触达（死分发表，后续轮次可考虑删表收编）。
    """

    def test_validate_dict_with_real_dict_adds_nothing(self):
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        validator._validate_dict({"a": 1}, {}, "p", result)
        assert result.errors == []

    def test_validate_list_with_real_list_adds_nothing(self):
        validator = ConfigValidator()
        result = ValidationResult(is_valid=True)
        validator._validate_list([1], {}, "p", result)
        assert result.errors == []


# ---------------------------------------------------------------------------
# sampler.py
# ---------------------------------------------------------------------------

class TestSamplerWellCoveredDataset:
    """sampler.py 279->282：问题类型与长度分布全覆盖 ⇒ `underrepresented` 为空

    10 条数据：六种问题类型各占 0.1（why/what/how/can/how_many + other×5），
    长度三档 0.5 / 0.3 / 0.2、平均长度 15.5，全部 ≥ 0.1 阈值 ⇒ 无覆盖不足项，
    走 `if underrepresented:` 假支（:279->282）。
    """

    @staticmethod
    def _well_covered_items():
        return [
            {"instruction": "为什么卖?"},
            {"instruction": "这个房子的档次是什么样的"},
            {"instruction": "怎么办理过户" + "流程" * 27},
            {"instruction": "可以贷款吗"},
            {"instruction": "多少钱"},
            {"instruction": "请推荐一个好小区"},
            {"instruction": "看房时间方便安排"},
            {"instruction": "小区停车位收费标准说明"},
            {"instruction": "房屋面积与户型布局介绍"},
            {"instruction": "配套" * 15 + "说明"},
        ]

    def test_no_underrepresented_flag(self):
        sampler = ActiveSampler()
        assert sampler.identify_underrepresented(self._well_covered_items()) == []

    def test_recommendation_skips_coverage_gap_line(self):
        result = ActiveSampler().recommend_seeds(self._well_covered_items())
        assert result.recommended_seeds == []
        assert not any("覆盖不足" in line for line in result.recommendations)
