"""自动标注单元测试

标注结果会作为元数据进入训练流程，实体边界与意图判定必须稳定。
"""

import pytest

from augmentor.data import AutoAnnotator, AnnotationResult


class TestEntityExtraction:
    """实体抽取"""

    def test_extracts_phone(self):
        """手机号是客服语料的高频实体"""
        entities = AutoAnnotator().extract_entities("联系电话 13812345678")
        types = [e["type"] for e in entities]
        assert "phone" in types

    def test_extracts_email(self):
        """邮箱实体识别"""
        entities = AutoAnnotator().extract_entities("请发到 service@example.com")
        assert any(e["type"] == "email" for e in entities)

    def test_extracts_url(self):
        """URL 实体识别"""
        entities = AutoAnnotator().extract_entities("详见 https://example.com/help")
        assert any(e["type"] == "url" for e in entities)

    def test_extracts_money(self):
        """金额实体识别，含中文单位"""
        entities = AutoAnnotator().extract_entities("押金 3000 元")
        assert any(e["type"] == "money" for e in entities)

    def test_extracts_date(self):
        """日期实体识别"""
        entities = AutoAnnotator().extract_entities("签约日期 2026-09-16")
        assert any(e["type"] == "date" for e in entities)

    def test_entity_offsets_are_ordered(self):
        """实体按出现位置排序，便于前端高亮渲染"""
        text = "邮箱 a@b.com，电话 13812345678"
        entities = AutoAnnotator().extract_entities(text)
        starts = [e["start"] for e in entities]

        assert starts == sorted(starts)
        for entity in entities:
            assert text[entity["start"]:entity["end"]] == entity["value"]

    def test_no_entities(self):
        """无实体时返回空列表"""
        assert AutoAnnotator().extract_entities("普通文本") == []

    def test_empty_text(self):
        """空文本不应抛异常"""
        assert AutoAnnotator().extract_entities("") == []


class TestIntentClassification:
    """意图分类"""

    @pytest.mark.parametrize("text,expected", [
        ("如何申请入住？", "how_to"),
        ("什么是押金？", "what_is"),
        ("为什么被拒绝了？", "why"),
        ("两种房型有什么区别？", "comparison"),
        ("一个月多少钱？", "price"),
        ("退租政策是什么规定？", "policy"),
        ("App 报错了无法登录", "troubleshooting"),
        ("你好", "greeting"),
    ])
    def test_classifies_intent(self, text, expected):
        """各类意图需被正确识别，供数据分布分析使用"""
        assert AutoAnnotator().classify_intent(text) == expected

    def test_unknown_intent_falls_back(self):
        """无法归类时回退到 other，而不是硬塞到某个类别"""
        assert AutoAnnotator().classify_intent("嗯") == "other"

    def test_empty_text(self):
        """空文本回退 other"""
        assert AutoAnnotator().classify_intent("") == "other"


class TestSentiment:
    """情感分析"""

    def test_positive(self):
        """正向表达应被识别"""
        assert AutoAnnotator().analyze_sentiment("服务很好，非常满意") == "positive"

    def test_negative(self):
        """负向表达应被识别"""
        assert AutoAnnotator().analyze_sentiment("体验很差，要投诉") == "negative"

    def test_neutral(self):
        """无明显情感词时为中性"""
        assert AutoAnnotator().analyze_sentiment("我想了解租金情况") == "neutral"

    def test_empty(self):
        """空文本为中性"""
        assert AutoAnnotator().analyze_sentiment("") == "neutral"


class TestAnnotateBatch:
    """批量标注"""

    def test_attaches_annotation_field(self):
        """标注结果需挂在 annotation 字段下，不污染原始字段"""
        items = [{"instruction": "如何申请？", "output": "通过 App"}]
        result = AutoAnnotator().annotate(items)

        assert isinstance(result, AnnotationResult)
        annotation = result.items[0]["annotation"]
        assert set(annotation.keys()) == {"entities", "intent", "sentiment"}
        assert result.items[0]["output"] == "通过 App"

    def test_counts_entities(self):
        """实体总数统计需累加所有样本"""
        items = [
            {"instruction": "电话 13812345678"},
            {"instruction": "邮箱 a@b.com"},
        ]
        result = AutoAnnotator().annotate(items)

        assert result.entity_count == 2

    def test_distributions_cover_all_items(self):
        """意图与情感分布之和应等于样本数"""
        items = [{"instruction": f"问题{i}"} for i in range(4)]
        result = AutoAnnotator().annotate(items)

        assert sum(result.intent_distribution.values()) == 4
        assert sum(result.sentiment_distribution.values()) == 4

    def test_empty_dataset(self):
        """空数据集不应抛异常"""
        result = AutoAnnotator().annotate([])
        assert result.items == []
        assert result.entity_count == 0

    def test_report_fields(self):
        """报告需给出人均实体数，便于判断信息密度"""
        report = AutoAnnotator().generate_report([
            {"instruction": "电话 13812345678"},
            {"instruction": "无实体"},
        ])

        assert report["total_items"] == 2
        assert report["avg_entities_per_item"] == pytest.approx(0.5)
