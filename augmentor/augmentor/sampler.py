"""主动学习选样模块"""

import logging
from typing import List, Dict, Optional
from dataclasses import dataclass
from collections import Counter
import re

logger = logging.getLogger(__name__)


@dataclass
class SamplingResult:
    """选样结果"""
    recommended_seeds: List[Dict]  # 推荐增强的 seed
    coverage_analysis: Dict  # 覆盖分析
    recommendations: List[str]  # 建议


class ActiveSampler:
    """主动学习选样器"""
    
    def __init__(self):
        """初始化主动学习选样器"""
        self._model = None
    
    def _load_model(self):
        """延迟加载模型"""
        if self._model is None:
            try:
                from sklearn.feature_extraction.text import TfidfVectorizer
                from sklearn.cluster import KMeans
                self._tfidf = TfidfVectorizer(max_features=1000)
                self._use_sklearn = True
                logger.info("使用 sklearn 进行主题分析")
            except ImportError:
                logger.warning("sklearn 未安装，使用简化分析")
                self._use_sklearn = False
    
    def _analyze_question_type(self, text: str) -> str:
        """分析问题类型
        
        Args:
            text: 问题文本
        
        Returns:
            问题类型
        """
        if "为什么" in text:
            return "why"
        elif "什么" in text or "是" in text:
            return "what"
        elif "怎么" in text or "如何" in text:
            return "how"
        elif "能不能" in text or "可以" in text:
            return "can"
        elif "多少" in text or "几" in text:
            return "how_many"
        else:
            return "other"
    
    def _analyze_length_distribution(self, items: List[Dict]) -> Dict:
        """分析长度分布
        
        Args:
            items: 数据列表
        
        Returns:
            长度分布字典
        """
        lengths = [len(item.get("instruction", "")) for item in items]
        
        if not lengths:
            return {"short": 0, "medium": 0, "long": 0}
        
        short = sum(1 for l in lengths if l < 10)
        medium = sum(1 for l in lengths if 10 <= l < 30)
        long_ = sum(1 for l in lengths if l >= 30)
        
        total = len(lengths)
        return {
            "short": short / total,
            "medium": medium / total,
            "long": long_ / total,
            "avg_length": sum(lengths) / total
        }
    
    def _analyze_topic_distribution(self, items: List[Dict]) -> Dict:
        """分析主题分布
        
        Args:
            items: 数据列表
        
        Returns:
            主题分布字典
        """
        # 简单的关键词提取
        keywords = []
        for item in items:
            text = item.get("instruction", "")
            # 提取关键词（简单的基于标点分割）
            words = re.findall(r'[\u4e00-\u9fa5]+', text)
            keywords.extend(words[:5])  # 取前5个关键词
        
        # 统计词频
        word_freq = Counter(keywords)
        total = sum(word_freq.values())
        
        return {
            "unique_words": len(word_freq),
            "top_words": dict(word_freq.most_common(20)),
            "coverage_score": len(word_freq) / max(total, 1)
        }
    
    def _analyze_complexity(self, items: List[Dict]) -> Dict:
        """分析复杂度分布
        
        Args:
            items: 数据列表
        
        Returns:
            复杂度分布字典
        """
        complexities = []
        for item in items:
            text = item.get("instruction", "")
            # 基于句子长度和连接词判断复杂度
            length_score = min(len(text) / 50, 1.0)
            connector_count = sum(1 for c in text if c in "，。；、")
            connector_score = min(connector_count / 5, 1.0)
            complexity = (length_score + connector_score) / 2
            complexities.append(complexity)
        
        if not complexities:
            return {"simple": 0, "medium": 0, "complex": 0}
        
        simple = sum(1 for c in complexities if c < 0.3)
        medium = sum(1 for c in complexities if 0.3 <= c < 0.7)
        complex_ = sum(1 for c in complexities if c >= 0.7)
        
        total = len(complexities)
        return {
            "simple": simple / total,
            "medium": medium / total,
            "complex": complex_ / total,
            "avg_complexity": sum(complexities) / total
        }
    
    def analyze_coverage(self, items: List[Dict]) -> Dict:
        """分析数据覆盖情况
        
        Args:
            items: 数据列表
        
        Returns:
            覆盖分析字典
        """
        self._load_model()
        
        # 问题类型分布
        question_types = Counter()
        for item in items:
            q_type = self._analyze_question_type(item.get("instruction", ""))
            question_types[q_type] += 1
        
        total = len(items) if items else 1
        type_distribution = {k: v / total for k, v in question_types.items()}
        
        # 长度分布
        length_dist = self._analyze_length_distribution(items)
        
        # 主题分布
        topic_dist = self._analyze_topic_distribution(items)
        
        # 复杂度分布
        complexity_dist = self._analyze_complexity(items)
        
        return {
            "total_items": len(items),
            "question_type_distribution": type_distribution,
            "length_distribution": length_dist,
            "topic_distribution": topic_dist,
            "complexity_distribution": complexity_dist
        }
    
    def identify_underrepresented(self, 
                                 items: List[Dict],
                                 threshold: float = 0.1) -> List[str]:
        """识别覆盖不足的类型
        
        Args:
            items: 数据列表
            threshold: 阈值，低于此比例的类型被认为是覆盖不足
        
        Returns:
            覆盖不足的类型列表
        """
        analysis = self.analyze_coverage(items)
        underrepresented = []
        
        # 检查问题类型
        for q_type, ratio in analysis["question_type_distribution"].items():
            if ratio < threshold:
                underrepresented.append(f"question_type:{q_type}")
        
        # 检查长度分布
        for length_type, ratio in analysis["length_distribution"].items():
            if isinstance(ratio, float) and ratio < threshold:
                underrepresented.append(f"length:{length_type}")
        
        return underrepresented
    
    def recommend_seeds(self,
                       items: List[Dict],
                       top_k: int = 10) -> SamplingResult:
        """推荐需要增强的 seed
        
        Args:
            items: 数据列表
            top_k: 推荐数量
        
        Returns:
            SamplingResult 实例
        """
        if not items:
            return SamplingResult(
                recommended_seeds=[],
                coverage_analysis={},
                recommendations=["数据为空，无法提供建议"]
            )
        
        # 分析覆盖情况
        coverage_analysis = self.analyze_coverage(items)
        
        # 识别覆盖不足的类型
        underrepresented = self.identify_underrepresented(items)
        
        # 为每个覆盖不足的类型找到代表性的 seed
        recommended_seeds = []
        for ur_type in underrepresented[:top_k]:
            category, value = ur_type.split(":")
            
            for item in items:
                if category == "question_type":
                    if self._analyze_question_type(item.get("instruction", "")) == value:
                        recommended_seeds.append(item)
                        break
                elif category == "length":
                    text = item.get("instruction", "")
                    if value == "short" and len(text) < 10:
                        recommended_seeds.append(item)
                        break
                    elif value == "medium" and 10 <= len(text) < 30:
                        recommended_seeds.append(item)
                        break
                    elif value == "long" and len(text) >= 30:
                        recommended_seeds.append(item)
                        break
        
        # 生成建议
        recommendations = []
        if underrepresented:
            recommendations.append(f"覆盖不足的类型: {', '.join(underrepresented)}")
        
        type_dist = coverage_analysis.get("question_type_distribution", {})
        if type_dist.get("what", 0) > 0.5:
            recommendations.append("'什么'类问题过多，建议增加其他类型问题")
        if type_dist.get("how", 0) < 0.2:
            recommendations.append("'如何'类问题较少，建议增加操作指导类问题")
        
        return SamplingResult(
            recommended_seeds=recommended_seeds[:top_k],
            coverage_analysis=coverage_analysis,
            recommendations=recommendations
        )
    
    def generate_report(self, items: List[Dict]) -> Dict:
        """生成选样报告
        
        Args:
            items: 数据列表
        
        Returns:
            选样报告字典
        """
        result = self.recommend_seeds(items)
        
        return {
            "total_items": len(items),
            "recommended_count": len(result.recommended_seeds),
            "coverage_analysis": result.coverage_analysis,
            "recommendations": result.recommendations,
            "recommended_seed_indices": [
                items.index(seed) for seed in result.recommended_seeds
                if seed in items
            ]
        }
