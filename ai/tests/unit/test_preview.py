"""导出预览单元测试

预览是导出前的最后一道校验，必须准确暴露字段缺失等问题。
"""

import pytest

from augmentor.preview import PreviewGenerator, ExportPreview


class TestPreviewBasics:
    """预览基础行为"""

    def test_returns_original_and_converted(self):
        """预览需同时给出原始与转换后数据，便于人工比对"""
        generator = PreviewGenerator(preview_size=2)
        items = [{"instruction": "问题", "output": "回答"}]
        preview = generator.preview(items, "alpaca")

        assert isinstance(preview, ExportPreview)
        assert preview.format == "alpaca"
        assert len(preview.original_data) == 1
        assert len(preview.converted_data) == 1
        assert preview.converted_data[0]["instruction"] == "问题"

    def test_jsonl_converted_to_dicts(self):
        """JSONL 内部是字符串行，预览必须转成字典以便前端渲染"""
        generator = PreviewGenerator()
        preview = generator.preview([{"instruction": "问题", "output": "回答"}], "jsonl")

        assert isinstance(preview.converted_data[0], dict)

    def test_preview_size_limits_items(self):
        """预览条数受 preview_size 限制，避免响应体过大"""
        generator = PreviewGenerator(preview_size=2)
        items = [{"instruction": f"问题{i}", "output": "回答"} for i in range(10)]
        preview = generator.preview(items, "alpaca")

        assert len(preview.converted_data) == 2
        assert preview.format_info["total_items"] == 10
        assert preview.format_info["preview_items"] == 2

    def test_explicit_size_overrides_default(self):
        """调用时可覆盖默认预览条数"""
        generator = PreviewGenerator(preview_size=5)
        items = [{"instruction": f"问题{i}", "output": "回答"} for i in range(10)]
        preview = generator.preview(items, "alpaca", preview_size=3)

        assert len(preview.converted_data) == 3

    def test_truncates_long_text(self):
        """超长文本应被截断，避免预览响应过大"""
        generator = PreviewGenerator()
        long_text = "很长的内容" * 200
        preview = generator.preview([{"instruction": long_text, "output": "回答"}], "alpaca")

        assert len(preview.converted_data[0]["instruction"]) < len(long_text)
        assert preview.converted_data[0]["instruction"].endswith("...")


class TestWarnings:
    """告警规则"""

    def test_warns_on_empty_dataset(self):
        """空数据集必须告警，否则用户会导出空文件却毫无察觉"""
        generator = PreviewGenerator()
        preview = generator.preview([], "jsonl")

        assert any("为空" in w for w in preview.warnings)

    def test_warns_on_missing_output_field(self):
        """output 为空会导致训练样本无监督信号，必须告警"""
        generator = PreviewGenerator()
        items = [{"instruction": "问题", "output": ""}]
        preview = generator.preview(items, "alpaca")

        assert any("output" in w for w in preview.warnings)

    def test_warns_on_missing_history_for_sharegpt(self):
        """多轮格式缺少 history 会退化为单轮，需提示用户"""
        generator = PreviewGenerator()
        items = [{"instruction": "问题", "output": "回答"}]
        preview = generator.preview(items, "sharegpt")

        assert any("history" in w for w in preview.warnings)

    def test_no_history_warning_when_present(self):
        """存在 history 时不应出现退化告警"""
        generator = PreviewGenerator()
        items = [{
            "instruction": "问题",
            "output": "回答",
            "history": [{"role": "user", "content": "上一轮"}]
        }]
        preview = generator.preview(items, "sharegpt")

        assert not any("history" in w for w in preview.warnings)


class TestErrors:
    """错误处理"""

    def test_invalid_format_raises(self):
        """未知格式必须报错并列出可用格式"""
        generator = PreviewGenerator()
        with pytest.raises(ValueError) as excinfo:
            generator.preview([{"instruction": "x"}], "unknown")

        assert "不支持的导出格式" in str(excinfo.value)

    def test_default_format_is_jsonl(self):
        """未指定格式时默认 jsonl"""
        preview = PreviewGenerator().preview([{"instruction": "x", "output": "y"}])
        assert preview.format == "jsonl"

    def test_to_dict_roundtrip(self):
        """to_dict 输出必须可直接序列化为 JSON"""
        import json

        preview = PreviewGenerator().preview([{"instruction": "x", "output": "y"}], "alpaca")
        assert json.loads(json.dumps(preview.to_dict(), ensure_ascii=False))


class TestTruncate:
    """截断逻辑测试"""

    def test_truncate_non_string_values(self):
        """非字符串值应原样返回"""
        gen = PreviewGenerator()
        assert gen._truncate(123) == 123
        assert gen._truncate(None) is None
        assert gen._truncate([1, 2, 3]) == [1, 2, 3]

    def test_truncate_short_string(self):
        """短字符串不应被截断"""
        gen = PreviewGenerator()
        assert gen._truncate("hello") == "hello"

    def test_truncate_nested_record(self):
        """嵌套字典应递归截断"""
        gen = PreviewGenerator()
        long_text = "x" * 300
        record = {"a": {"b": long_text}, "c": [long_text, "short"]}
        result = gen._truncate_record(record)
        assert result["a"]["b"].endswith("...")
        assert result["c"][0].endswith("...")
        assert result["c"][1] == "short"

    def test_truncate_record_with_int_values(self):
        """记录中包含整数值时不应报错"""
        gen = PreviewGenerator()
        record = {"count": 42, "name": "test"}
        result = gen._truncate_record(record)
        assert result["count"] == 42

    def test_chatml_preview(self):
        """ChatML 格式预览应生成 messages 结构"""
        generator = PreviewGenerator()
        items = [{"instruction": "问题", "output": "回答"}]
        preview = generator.preview(items, "chatml")
        assert preview.format == "chatml"
        assert "messages" in preview.converted_data[0]

    def test_chatml_warns_no_history(self):
        """ChatML 无 history 应告警"""
        generator = PreviewGenerator()
        items = [{"instruction": "问题", "output": "回答"}]
        preview = generator.preview(items, "chatml")
        assert any("history" in w for w in preview.warnings)

    def test_csv_preview(self):
        """CSV 格式预览应生成字段结构"""
        generator = PreviewGenerator()
        items = [{"instruction": "问题", "output": "回答"}]
        preview = generator.preview(items, "csv")
        assert preview.format == "csv"
        assert "instruction" in preview.converted_data[0]
