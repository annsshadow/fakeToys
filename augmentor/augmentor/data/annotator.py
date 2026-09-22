# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""自动标注模块

基于规则实现实体识别、意图分类与情感分析，无需外部模型即可运行。
"""

import logging
import re
from dataclasses import dataclass, field
from typing import List, Dict, Any

logger = logging.getLogger(__name__)

# 实体识别规则：实体类型 -> 正则
ENTITY_PATTERNS: Dict[str, re.Pattern] = {
    "phone": re.compile(r"(?<!\d)(?:1[3-9]\d{9}|0\d{2,3}-?\d{7,8})(?!\d)"),
    "email": re.compile(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}"),
    "url": re.compile(r"https?://\S+|www\.\S+"),
    "money": re.compile(r"(?:[¥￥$]\s?\d+(?:\.\d+)?|\d+(?:\.\d+)?\s?(?:元|块|万元|美元))"),
    "date": re.compile(r"\d{4}[-/年]\d{1,2}[-/月]\d{1,2}日?|\d{1,2}月\d{1,2}日"),
    "id_card": re.compile(r"(?<!\d)\d{17}[\dXx](?!\d)")
}

# 意图分类规则：意图 -> 关键词
INTENT_KEYWORDS: Dict[str, List[str]] = {
    "how_to": ["怎么", "如何", "怎样", "步骤", "操作", "方法"],
    "what_is": ["是什么", "什么是", "定义", "含义", "介绍"],
    "why": ["为什么", "原因", "为何"],
    "comparison": ["区别", "对比", "哪个好", "差异", "相比"],
    "price": ["价格", "多少钱", "费用", "收费", "报价"],
    "policy": ["政策", "规定", "规则", "标准", "条款"],
    "troubleshooting": ["报错", "失败", "无法", "不能用", "异常", "故障"],
    "greeting": ["你好", "您好", "hi", "hello", "在吗"]
}

# 情感词典
POSITIVE_WORDS = ["好", "满意", "喜欢", "推荐", "优秀", "感谢", "谢谢", "不错", "棒", "赞"]
NEGATIVE_WORDS = ["差", "糟糕", "不满", "投诉", "失望", "问题", "错误", "垃圾", "讨厌", "不行"]


@dataclass
class AnnotationResult:
    """标注结果"""
    entity_count: int = 0
    intent_distribution: Dict[str, int] = field(default_factory=dict)
    sentiment_distribution: Dict[str, int] = field(default_factory=dict)
    items: List[Dict] = field(default_factory=list)


class AutoAnnotator:
    """自动标注器"""

    def __init__(self, text_key: str = "instruction"):
        """初始化标注器

        Args:
            text_key: 参与标注的文本字段
        """
        self.text_key = text_key

    def extract_entities(self, text: str) -> List[Dict[str, Any]]:
        """抽取实体

        Args:
            text: 文本

        Returns:
            实体列表 [{"type": ..., "value": ..., "start": ..., "end": ...}]
        """
        if not isinstance(text, str) or not text:
            return []

        entities: List[Dict[str, Any]] = []
        for entity_type, pattern in ENTITY_PATTERNS.items():
            for match in pattern.finditer(text):
                entities.append({
                    "type": entity_type,
                    "value": match.group(),
                    "start": match.start(),
                    "end": match.end()
                })

        entities.sort(key=lambda e: e["start"])
        return entities

    def classify_intent(self, text: str) -> str:
        """分类意图

        Args:
            text: 文本

        Returns:
            意图标签，未命中时为 other
        """
        if not isinstance(text, str) or not text:
            return "other"

        lowered = text.lower()
        best_intent = "other"
        best_hits = 0

        for intent, keywords in INTENT_KEYWORDS.items():
            hits = sum(1 for kw in keywords if kw in lowered)
            if hits > best_hits:
                best_hits = hits
                best_intent = intent

        return best_intent

    def analyze_sentiment(self, text: str) -> str:
        """分析情感倾向

        Args:
            text: 文本

        Returns:
            positive / negative / neutral
        """
        if not isinstance(text, str) or not text:
            return "neutral"

        positive_hits = sum(1 for w in POSITIVE_WORDS if w in text)
        negative_hits = sum(1 for w in NEGATIVE_WORDS if w in text)

        if positive_hits > negative_hits:
            return "positive"
        if negative_hits > positive_hits:
            return "negative"
        return "neutral"

    def annotate_text(self, text: str) -> Dict[str, Any]:
        """对单条文本执行完整标注

        Args:
            text: 文本

        Returns:
            标注结果字典
        """
        entities = self.extract_entities(text)
        return {
            "entities": entities,
            "intent": self.classify_intent(text),
            "sentiment": self.analyze_sentiment(text)
        }

    def annotate(self, items: List[Dict]) -> AnnotationResult:
        """批量标注数据

        Args:
            items: 数据列表

        Returns:
            AnnotationResult 实例
        """
        result = AnnotationResult()
        intent_distribution: Dict[str, int] = {}
        sentiment_distribution: Dict[str, int] = {}

        for item in items:
            text = item.get(self.text_key, "")
            annotation = self.annotate_text(text)

            result.entity_count += len(annotation["entities"])
            intent = annotation["intent"]
            sentiment = annotation["sentiment"]
            intent_distribution[intent] = intent_distribution.get(intent, 0) + 1
            sentiment_distribution[sentiment] = sentiment_distribution.get(sentiment, 0) + 1

            annotated_item = dict(item)
            annotated_item["annotation"] = annotation
            result.items.append(annotated_item)

        result.intent_distribution = intent_distribution
        result.sentiment_distribution = sentiment_distribution

        logger.info(
            f"标注完成: {len(items)} 条，抽取实体 {result.entity_count} 个"
        )
        return result

    def generate_report(self, items: List[Dict]) -> Dict[str, Any]:
        """生成标注报告

        Args:
            items: 数据列表

        Returns:
            报告字典
        """
        result = self.annotate(items)
        total = len(items)

        return {
            "total_items": total,
            "entity_count": result.entity_count,
            "avg_entities_per_item": result.entity_count / total if total else 0.0,
            "intent_distribution": result.intent_distribution,
            "sentiment_distribution": result.sentiment_distribution,
            "text_key": self.text_key
        }
