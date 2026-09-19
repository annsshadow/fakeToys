"""多语言支持单元测试

翻译依赖模型后端，因此后端缺失、语言不支持等边界必须明确报错。
"""

import pytest

from augmentor.multilingual import MultilingualSupport


class FakeBackend:
    """模拟翻译后端"""

    def __init__(self):
        self.calls = 0

    def generate(self, prompt: str) -> str:
        """返回带标记的假翻译结果

        Args:
            prompt: 提示词

        Returns:
            假翻译结果
        """
        self.calls += 1
        return f"[translated]{prompt.splitlines()[-1]}"


class TestDetection:
    """语言检测"""

    @pytest.mark.parametrize("text,expected", [
        ("这是中文", "zh"),
        ("this is english", "en"),
        ("中文 english 混合", "mixed"),
    ])
    def test_detects(self, text, expected):
        """中/英/混合需被区分，用于决定是否需要翻译"""
        assert MultilingualSupport().detect_language(text) == expected

    def test_empty(self):
        """空文本为 unknown"""
        assert MultilingualSupport().detect_language("") == "unknown"

    def test_non_string_input(self):
        """非字符串输入应返回 unknown"""
        assert MultilingualSupport().detect_language(None) == "unknown"
        assert MultilingualSupport().detect_language(123) == "unknown"

    def test_pure_numbers_returns_unknown(self):
        """纯数字应返回 unknown（无 CJK 也无 Latin 字母）"""
        assert MultilingualSupport().detect_language("12345") == "unknown"

    def test_mixed_ratio_both_below_threshold(self):
        """当 CJK 和 Latin 比例都 > 0.1 但都 < 0.8 时应为 mixed"""
        # 5 CJK + 5 Latin = 50% each -> mixed
        text = "你好世界testabcd"
        assert MultilingualSupport().detect_language(text) == "mixed"

    def test_is_mixed(self):
        """混合检测便捷方法"""
        support = MultilingualSupport()
        assert support.is_mixed("中文 english 混合") is True
        assert support.is_mixed("纯中文内容") is False


class TestTranslate:
    """单条翻译"""

    def test_requires_backend(self):
        """未配置后端时必须报错，而不是返回原文造成静默错误"""
        with pytest.raises(RuntimeError):
            MultilingualSupport().translate("你好", "en")

    def test_rejects_unsupported_target(self):
        """不支持的目标语言必须报错并列出支持项"""
        support = MultilingualSupport(model_backend=FakeBackend())
        with pytest.raises(ValueError):
            support.translate("你好", "ja")

    def test_same_language_short_circuits(self):
        """源语言与目标语言一致时不应调用模型，节省配额"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend)

        assert support.translate("这是中文", "zh") == "这是中文"
        assert backend.calls == 0

    def test_translates_via_backend(self):
        """不同语言时调用后端并返回结果"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend)

        result = support.translate("你好世界", "en")

        assert result.startswith("[translated]")
        assert backend.calls == 1

    def test_empty_text_returns_empty(self):
        """空文本无需翻译"""
        support = MultilingualSupport(model_backend=FakeBackend())
        assert support.translate("", "en") == ""


class TestBatchTranslate:
    """批量翻译"""

    def test_translates_all_items(self):
        """批量翻译结果数量需与输入一致"""
        support = MultilingualSupport(model_backend=FakeBackend())
        items = [{"instruction": f"中文问题{i}"} for i in range(3)]

        result = support.batch_translate(items, target_lang="en")

        assert len(result) == 3
        assert all(r["language"] == "en" for r in result)

    def test_keeps_original_when_requested(self):
        """保留原文便于人工核对，默认开启"""
        support = MultilingualSupport(model_backend=FakeBackend())
        result = support.batch_translate([{"instruction": "中文问题"}], target_lang="en")

        assert result[0]["instruction_original"] == "中文问题"

    def test_does_not_mutate_input(self):
        """批量翻译不得修改调用方传入的数据（避免隐蔽副作用）"""
        support = MultilingualSupport(model_backend=FakeBackend())
        items = [{"instruction": "中文问题"}]
        support.batch_translate(items, target_lang="en")

        assert items[0]["instruction"] == "中文问题"
        assert "instruction_original" not in items[0]

    def test_empty_dataset(self):
        """空数据集返回空列表"""
        support = MultilingualSupport(model_backend=FakeBackend())
        assert support.batch_translate([]) == []

    def test_rejects_unsupported_target(self):
        """不支持的目标语言必须报错"""
        support = MultilingualSupport(model_backend=FakeBackend())
        with pytest.raises(ValueError):
            support.batch_translate([{"instruction": "x"}], target_lang="ja")

    def test_failed_items_fall_back_to_original(self):
        """翻译失败时应回退原文，保证输出长度与输入对齐"""
        class FailingBackend:
            def generate(self, prompt):
                raise RuntimeError("backend down")

        support = MultilingualSupport(model_backend=FailingBackend())
        items = [{"instruction": "中文问题"}]
        result = support.batch_translate(items, target_lang="en")

        assert len(result) == 1
        assert result[0]["instruction"] == "中文问题"

    def test_keep_original_false(self):
        """keep_original=False 时不保留原文字段"""
        support = MultilingualSupport(model_backend=FakeBackend())
        result = support.batch_translate(
            [{"instruction": "中文问题"}], target_lang="en", keep_original=False
        )
        assert "instruction_original" not in result[0]
        assert result[0]["language"] == "en"

    def test_custom_text_key(self):
        """自定义 text_key 应翻译指定字段"""
        support = MultilingualSupport(model_backend=FakeBackend())
        result = support.batch_translate(
            [{"question": "中文问题"}], target_lang="en", text_key="question"
        )
        assert result[0]["language"] == "en"
        assert result[0].get("question_original") == "中文问题"

    def test_parallel_translation(self):
        """并行翻译应返回正确结果"""
        support = MultilingualSupport(model_backend=FakeBackend(), max_workers=2)
        items = [{"instruction": f"问题{i}"} for i in range(5)]
        result = support.batch_translate(items, target_lang="en")
        assert len(result) == 5


class TestAnalyzeLanguages:
    """语言分布统计"""

    def test_distribution(self):
        """分布统计需覆盖全部样本"""
        support = MultilingualSupport()
        report = support.analyze_languages([
            {"instruction": "中文"},
            {"instruction": "english"},
            {"instruction": "中文"},
        ])

        assert report["total_items"] == 3
        assert report["distribution"]["zh"] == 2
        assert report["ratios"]["zh"] == pytest.approx(2 / 3)

    def test_empty_dataset(self):
        """空数据集不应除零"""
        report = MultilingualSupport().analyze_languages([])
        assert report["total_items"] == 0
        assert report["ratios"] == {}

    def test_translate_non_string(self):
        """非字符串输入翻译"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend)
        assert support.translate(123, "en") == ""
        assert backend.calls == 0

    def test_batch_translate_empty_items(self):
        """空项目列表"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend)
        result = support.batch_translate([])
        assert result == []

    def test_detect_language_pure_latin(self):
        """纯拉丁文检测"""
        support = MultilingualSupport()
        assert support.detect_language("hello world test") == "en"

    def test_batch_translate_unsupported_target(self):
        """批量翻译不支持目标语言"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend)
        with pytest.raises(ValueError):
            support.batch_translate([{"instruction": "x"}], "ja")

    def test_batch_translate_keep_false(self):
        """不保留原文"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend)
        result = support.batch_translate(
            [{"instruction": "中文问题"}], target_lang="en", keep_original=False
        )
        assert "instruction_original" not in result[0]

    def test_custom_text_key_translate(self):
        """自定义文本键批量翻译"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend)
        result = support.batch_translate(
            [{"question": "中文问题"}], target_lang="en", text_key="question"
        )
        assert result[0]["language"] == "en"
        assert result[0].get("question_original") == "中文问题"

    def test_parallel_translation_result(self):
        """并行翻译结果"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend, max_workers=2)
        items = [{"instruction": f"问题{i}"} for i in range(5)]
        result = support.batch_translate(items, target_lang="en")
        assert len(result) == 5

    def test_lang_name_exists(self):
        """语言名称存在"""
        support = MultilingualSupport()
        assert "zh" in support.supported_langs
        assert support._lang_name("zh") == "中文"

    def test_translate_same_language_no_backend_call(self):
        """相同语言无后端调用"""
        backend = FakeBackend()
        support = MultilingualSupport(model_backend=backend)
        support.translate("中文内容", "zh")
        assert backend.calls == 0
