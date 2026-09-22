# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据分析模块

提供数据集的深度分析和洞察功能。

分工边界（与 `augmentor.statistics`）
    本模块做**整体分析**：质量 / 多样性 / 完整性分数与可执行的改进建议
    （`AnalysisReport.insights`），入口 `analyze_dataset`。CLI 的 `analyze-data` 走这里。
    要逐字段的填充率 / 长度 / 唯一值，用 `statistics`。
"""

import re
import logging
from typing import List, Dict, Optional, Any, Set, Tuple
from dataclasses import dataclass, field
from collections import Counter, defaultdict
from pathlib import Path

logger = logging.getLogger(__name__)


@dataclass
class TextStatistics:
    """文本统计信息"""
    total_chars: int
    avg_chars: float
    min_chars: int
    max_chars: int
    total_words: int
    avg_words: float
    min_words: int
    max_words: int
    avg_sentence_length: float
    vocabulary_size: int
    top_words: List[Tuple[str, int]]
    top_bigrams: List[Tuple[str, int]]


@dataclass
class DataInsight:
    """数据洞察"""
    category: str
    title: str
    description: str
    severity: str  # info, warning, critical
    recommendation: str
    data: Optional[Dict] = None


@dataclass
class AnalysisReport:
    """分析报告"""
    dataset_size: int
    field_statistics: Dict[str, TextStatistics]
    insights: List[DataInsight]
    quality_score: float
    diversity_score: float
    completeness_score: float
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "dataset_size": self.dataset_size,
            "field_statistics": {
                field: {
                    "total_chars": stats.total_chars,
                    "avg_chars": stats.avg_chars,
                    "total_words": stats.total_words,
                    "avg_words": stats.avg_words,
                    "vocabulary_size": stats.vocabulary_size
                }
                for field, stats in self.field_statistics.items()
            },
            "insights": [
                {
                    "category": i.category,
                    "title": i.title,
                    "description": i.description,
                    "severity": i.severity,
                    "recommendation": i.recommendation
                }
                for i in self.insights
            ],
            "scores": {
                "quality": self.quality_score,
                "diversity": self.diversity_score,
                "completeness": self.completeness_score
            }
        }


class DatasetAnalyzer:
    """数据集分析器
    
    提供数据集的深度分析和洞察功能。
    """
    
    def __init__(self, items: List[Dict] = None):
        """初始化分析器
        
        Args:
            items: 数据列表
        """
        self._items = items or []
        self._stopwords: Set[str] = {
            "的", "了", "是", "在", "我", "有", "和", "就", "不", "人",
            "都", "一", "一个", "上", "也", "很", "到", "说", "要", "去",
            "你", "会", "着", "没有", "看", "好", "自己", "这", "他", "她"
        }
    
    def load(self, items: List[Dict]):
        """加载数据
        
        Args:
            items: 数据列表
        """
        self._items = items
    
    def _tokenize(self, text: str) -> List[str]:
        """分词（简单版本）"""
        # 简单的中文分词：按字符和标点分割
        tokens = re.findall(r'[\u4e00-\u9fff]+|[a-zA-Z]+|\d+', text)
        return tokens
    
    def _get_words(self, text: str) -> List[str]:
        """获取词列表"""
        tokens = self._tokenize(text)
        return [t for t in tokens if len(t) > 1 or t.isalpha()]
    
    def _get_sentences(self, text: str) -> List[str]:
        """获取句子列表"""
        sentences = re.split(r'[。！？.!?]+', text)
        return [s.strip() for s in sentences if s.strip()]
    
    def _calculate_text_statistics(self, texts: List[str]) -> TextStatistics:
        """计算文本统计信息
        
        Args:
            texts: 文本列表
        
        Returns:
            文本统计信息
        """
        if not texts:
            return TextStatistics(
                total_chars=0, avg_chars=0, min_chars=0, max_chars=0,
                total_words=0, avg_words=0, min_words=0, max_words=0,
                avg_sentence_length=0, vocabulary_size=0,
                top_words=[], top_bigrams=[]
            )
        
        char_lengths = [len(t) for t in texts]
        word_lists = [self._get_words(t) for t in texts]
        word_lengths = [len(wl) for wl in word_lists]
        
        # 词汇统计
        all_words = []
        for wl in word_lists:
            all_words.extend([w for w in wl if w not in self._stopwords])
        
        word_counter = Counter(all_words)
        vocab_size = len(word_counter)
        top_words = word_counter.most_common(20)
        
        # Bigram统计
        bigrams = []
        for wl in word_lists:
            for i in range(len(wl) - 1):
                if wl[i] not in self._stopwords and wl[i+1] not in self._stopwords:
                    bigrams.append(f"{wl[i]}{wl[i+1]}")
        
        bigram_counter = Counter(bigrams)
        top_bigrams = bigram_counter.most_common(20)
        
        # 句子长度
        sentence_lengths = []
        for text in texts:
            sentences = self._get_sentences(text)
            if sentences:
                sentence_lengths.append(sum(len(s) for s in sentences) / len(sentences))
        
        avg_sentence_length = sum(sentence_lengths) / len(sentence_lengths) if sentence_lengths else 0
        
        return TextStatistics(
            total_chars=sum(char_lengths),
            avg_chars=sum(char_lengths) / len(char_lengths),
            min_chars=min(char_lengths),
            max_chars=max(char_lengths),
            total_words=sum(word_lengths),
            avg_words=sum(word_lengths) / len(word_lengths),
            min_words=min(word_lengths),
            max_words=max(word_lengths),
            avg_sentence_length=avg_sentence_length,
            vocabulary_size=vocab_size,
            top_words=top_words,
            top_bigrams=top_bigrams
        )
    
    def analyze(self, fields: List[str] = None) -> AnalysisReport:
        """分析数据集
        
        Args:
            fields: 要分析的字段列表
        
        Returns:
            分析报告
        """
        fields = fields or ["instruction", "output", "input"]
        
        # 计算每个字段的统计信息
        field_statistics = {}
        for field in fields:
            texts = [item.get(field, "") for item in self._items if item.get(field)]
            field_statistics[field] = self._calculate_text_statistics(texts)
        
        # 生成洞察
        insights = self._generate_insights(field_statistics)
        
        # 计算分数
        quality_score = self._calculate_quality_score()
        diversity_score = self._calculate_diversity_score()
        completeness_score = self._calculate_completeness_score(fields)
        
        return AnalysisReport(
            dataset_size=len(self._items),
            field_statistics=field_statistics,
            insights=insights,
            quality_score=quality_score,
            diversity_score=diversity_score,
            completeness_score=completeness_score
        )
    
    def _generate_insights(self, field_statistics: Dict[str, TextStatistics]) -> List[DataInsight]:
        """生成数据洞察
        
        Args:
            field_statistics: 字段统计信息
        
        Returns:
            洞察列表
        """
        insights = []
        
        # 检查数据量
        if len(self._items) < 100:
            insights.append(DataInsight(
                category="数据量",
                title="数据量较少",
                description=f"当前数据集只有 {len(self._items)} 条数据",
                severity="warning",
                recommendation="建议至少收集100条以上的数据以确保模型训练效果"
            ))
        elif len(self._items) >= 1000:
            insights.append(DataInsight(
                category="数据量",
                title="数据量充足",
                description=f"当前数据集有 {len(self._items)} 条数据",
                severity="info",
                recommendation="数据量充足，可以进行模型训练"
            ))
        
        # 检查instruction字段
        if "instruction" in field_statistics:
            stats = field_statistics["instruction"]
            
            # 检查平均长度
            if stats.avg_chars < 10:
                insights.append(DataInsight(
                    category="问题质量",
                    title="问题过短",
                    description=f"问题平均长度只有 {stats.avg_chars:.1f} 个字符",
                    severity="warning",
                    recommendation="建议问题描述更加详细，至少包含10个以上字符"
                ))
            elif stats.avg_chars > 200:
                insights.append(DataInsight(
                    category="问题质量",
                    title="问题过长",
                    description=f"问题平均长度达到 {stats.avg_chars:.1f} 个字符",
                    severity="warning",
                    recommendation="建议问题描述更加简洁，避免过长的问题"
                ))
            
            # 检查词汇多样性
            if stats.vocabulary_size < 50:
                insights.append(DataInsight(
                    category="词汇多样性",
                    title="词汇多样性不足",
                    description=f"问题词汇量只有 {stats.vocabulary_size} 个",
                    severity="warning",
                    recommendation="建议增加问题的多样性，使用不同的表达方式"
                ))
        
        # 检查output字段
        if "output" in field_statistics:
            stats = field_statistics["output"]
            
            # 检查回答长度
            if stats.avg_chars < 20:
                insights.append(DataInsight(
                    category="回答质量",
                    title="回答过短",
                    description=f"回答平均长度只有 {stats.avg_chars:.1f} 个字符",
                    severity="warning",
                    recommendation="建议回答内容更加详细，提供更完整的信息"
                ))
        
        return insights
    
    def _calculate_quality_score(self) -> float:
        """计算质量分数"""
        if not self._items:
            return 0.0
        
        score = 1.0
        
        # 检查数据完整性
        complete_items = sum(1 for item in self._items 
                          if item.get("instruction") and item.get("output"))
        completeness_ratio = complete_items / len(self._items)
        score *= completeness_ratio
        
        # 检查问题和回答的一致性
        consistent_items = 0
        for item in self._items:
            instruction = item.get("instruction", "")
            output = item.get("output", "")
            if instruction and output:
                # 简单检查：如果问题和回答太相似，可能质量不高
                if len(set(instruction) & set(output)) / max(len(set(instruction)), 1) < 0.8:
                    consistent_items += 1
        
        if self._items:
            consistency_ratio = consistent_items / len(self._items)
            score *= (0.5 + 0.5 * consistency_ratio)
        
        return min(max(score, 0.0), 1.0)
    
    def _calculate_diversity_score(self) -> float:
        """计算多样性分数"""
        if not self._items:
            return 0.0
        
        # 收集所有问题
        instructions = [item.get("instruction", "") for item in self._items if item.get("instruction")]
        
        if not instructions:
            return 0.0
        
        # 计算词汇多样性
        all_words = []
        for inst in instructions:
            words = self._get_words(inst)
            all_words.extend(words)
        
        if not all_words:
            return 0.0
        
        unique_ratio = len(set(all_words)) / len(all_words)
        
        # 计算长度多样性
        lengths = [len(inst) for inst in instructions]
        length_std = (sum((l - sum(lengths)/len(lengths))**2 for l in lengths) / len(lengths)) ** 0.5
        length_diversity = min(length_std / 50, 1.0)  # 归一化
        
        # 综合分数
        diversity_score = (unique_ratio * 0.6 + length_diversity * 0.4)
        
        return min(max(diversity_score, 0.0), 1.0)
    
    def _calculate_completeness_score(self, fields: List[str]) -> float:
        """计算完整性分数"""
        if not self._items:
            return 0.0
        
        required_fields = ["instruction", "output"]
        total_score = 0.0
        
        for field in required_fields:
            if field in fields:
                filled_count = sum(1 for item in self._items if item.get(field))
                field_score = filled_count / len(self._items)
                total_score += field_score
        
        return total_score / len(required_fields) if required_fields else 0.0
    
    def get_duplicate_candidates(self, threshold: float = 0.8) -> List[Tuple[int, int, float]]:
        """获取重复候选
        
        Args:
            threshold: 相似度阈值
        
        Returns:
            重复候选列表 [(idx1, idx2, similarity), ...]
        """
        candidates = []
        
        for i in range(len(self._items)):
            for j in range(i + 1, len(self._items)):
                inst_i = self._items[i].get("instruction", "")
                inst_j = self._items[j].get("instruction", "")
                
                if inst_i and inst_j:
                    # 简单的字符重叠相似度
                    set_i = set(inst_i)
                    set_j = set(inst_j)
                    similarity = len(set_i & set_j) / max(len(set_i | set_j), 1)
                    
                    if similarity >= threshold:
                        candidates.append((i, j, similarity))
        
        return sorted(candidates, key=lambda x: x[2], reverse=True)
    
    def get_quality_distribution(self) -> Dict[str, int]:
        """获取质量分布
        
        Returns:
            质量分布 {quality_level: count}
        """
        distribution = {"excellent": 0, "good": 0, "fair": 0, "poor": 0}
        
        for item in self._items:
            instruction = item.get("instruction", "")
            output = item.get("output", "")
            
            # 简单的质量评估
            if len(instruction) >= 10 and len(output) >= 50:
                distribution["excellent"] += 1
            elif len(instruction) >= 5 and len(output) >= 20:
                distribution["good"] += 1
            elif len(instruction) >= 3 and len(output) >= 10:
                distribution["fair"] += 1
            else:
                distribution["poor"] += 1
        
        return distribution
    
    def get_category_distribution(self, field: str = "instruction") -> Dict[str, int]:
        """获取类别分布
        
        Args:
            field: 字段名
        
        Returns:
            类别分布 {category: count}
        """
        # 简单的关键词分类
        categories = {
            "租房": ["租房", "租", "租赁", "房东", "房客"],
            "买房": ["买房", "购房", "房产", "楼盘", "置业"],
            "贷款": ["贷款", "房贷", "按揭", "利率", "还款"],
            "物业": ["物业", "小区", "维修", "保洁", "安全"],
            "政策": ["政策", "规定", "法规", "条例", "办法"],
            "其他": []
        }
        
        distribution = defaultdict(int)
        
        for item in self._items:
            text = item.get(field, "").lower()
            
            matched = False
            for category, keywords in categories.items():
                if any(kw in text for kw in keywords):
                    distribution[category] += 1
                    matched = True
                    break
            
            if not matched:
                distribution["其他"] += 1
        
        return dict(distribution)


def analyze_dataset(items: List[Dict], fields: List[str] = None) -> AnalysisReport:
    """分析数据集
    
    Args:
        items: 数据列表
        fields: 要分析的字段列表
    
    Returns:
        分析报告
    """
    analyzer = DatasetAnalyzer(items)
    return analyzer.analyze(fields)


def get_dataset_insights(items: List[Dict]) -> List[DataInsight]:
    """获取数据集洞察
    
    Args:
        items: 数据列表
    
    Returns:
        洞察列表
    """
    analyzer = DatasetAnalyzer(items)
    report = analyzer.analyze()
    return report.insights


def analyze_dataset_fast(items: List[Dict], top_k: int = 5) -> Dict:
    """快速数据分析（增强功能：优化分析性能，适用于大数据集预览）
    
    Args:
        items: 数据列表
        top_k: 返回前k个关键词
    
    Returns:
        快速分析结果（包含基本统计、关键词、趋势简要信息）
    """
    from collections import Counter
    
    # 快速提取基础信息（避免完整分析的计算开销）
    instructions = [str(item.get("instruction", "")) for item in items if item.get("instruction")]
    outputs = [str(item.get("output", "")) for item in items if item.get("output")]
    
    # 简单词频统计（优化：只计算前100条数据避免大数据集开销）
    sample_size = min(100, len(instructions))
    sample_text = " ".join(instructions[:sample_size])
    words = [w for w in sample_text.split() if len(w) > 1]
    word_freq = Counter(words)
    
    return {
        "fast_analysis": True,
        "sample_size": len(items),
        "analyzed_sample": sample_size,
        "top_words": [word for word, _ in word_freq.most_common(top_k)],
        "avg_instruction_length": sum(len(s) for s in instructions) / max(len(instructions), 1) if instructions else 0,
        "avg_output_length": sum(len(s) for s in outputs) / max(len(outputs), 1) if outputs else 0,
        "has_empty_fields": any(not item.get("instruction") or not item.get("output") for item in items)
    }
