"""自动领域扩展模块"""

import json
import logging
from typing import List, Dict, Optional
from dataclasses import dataclass
from .models.base import ModelBackend

logger = logging.getLogger(__name__)


@dataclass
class ExpansionResult:
    """扩展结果"""
    original_topics: List[str]  # 原始主题
    expanded_topics: List[Dict]  # 扩展的主题
    strategy: str  # 使用的策略
    suggestions: List[str]  # 建议


class DomainExpander:
    """领域扩展器"""
    
    def __init__(self, model_backend: ModelBackend):
        """初始化领域扩展器
        
        Args:
            model_backend: 模型后端
        """
        self.model_backend = model_backend
    
    def _extract_topics(self, items: List[Dict]) -> List[str]:
        """从数据中提取主题
        
        Args:
            items: 数据列表
        
        Returns:
            主题列表
        """
        topics = set()
        for item in items:
            instruction = item.get("instruction", "")
            # 简单的主题提取：基于关键词
            if "晨阳" in instruction:
                topics.add("晨阳计划")
            elif "租金" in instruction or "房租" in instruction:
                topics.add("租金相关")
            elif "换房" in instruction:
                topics.add("换房服务")
            elif "押金" in instruction:
                topics.add("押金相关")
            elif "合同" in instruction:
                topics.add("合同相关")
            elif "维修" in instruction:
                topics.add("维修服务")
            elif "投诉" in instruction:
                topics.add("投诉建议")
        
        return list(topics) if topics else ["通用问题"]
    
    def _generate_similar_topics(self, 
                                topics: List[str],
                                num_topics: int = 5) -> List[str]:
        """生成相似主题（同类扩展）
        
        Args:
            topics: 原始主题列表
            num_topics: 生成数量
        
        Returns:
            扩展的主题列表
        """
        prompt = f"""基于以下租房相关主题，生成 {num_topics} 个相似的主题。

原始主题：
{json.dumps(topics, ensure_ascii=False)}

要求：
1. 与原始主题属于同一领域（租房、公寓服务）
2. 具有实际意义
3. 不要与原始主题重复
4. 只输出主题列表，不要解释

扩展主题："""
        
        try:
            response = self.model_backend.generate(prompt)
            # 解析响应
            topics_text = response.strip()
            # 尝试解析 JSON 格式
            if topics_text.startswith('['):
                expanded = json.loads(topics_text)
                if isinstance(expanded, list):
                    return expanded[:num_topics]
            # 否则按行分割
            expanded = [t.strip().strip('0123456789.-、 ') for t in topics_text.split('\n') if t.strip()]
            return expanded[:num_topics]
        except Exception as e:
            logger.warning(f"生成相似主题失败: {e}")
            return []
    
    def _generate_related_topics(self,
                                topics: List[str],
                                num_topics: int = 5) -> List[str]:
        """生成相关主题（关联扩展）
        
        Args:
            topics: 原始主题列表
            num_topics: 生成数量
        
        Returns:
            扩展的主题列表
        """
        prompt = f"""基于以下租房相关主题，生成 {num_topics} 个相关但不同的主题。

原始主题：
{json.dumps(topics, ensure_ascii=False)}

要求：
1. 与原始主题相关但不完全相同
2. 可以是原始主题的子话题或延伸话题
3. 具有实际意义
4. 只输出主题列表，不要解释

相关主题："""
        
        try:
            response = self.model_backend.generate(prompt)
            topics_text = response.strip()
            if topics_text.startswith('['):
                expanded = json.loads(topics_text)
                if isinstance(expanded, list):
                    return expanded[:num_topics]
            expanded = [t.strip().strip('0123456789.-、 ') for t in topics_text.split('\n') if t.strip()]
            return expanded[:num_topics]
        except Exception as e:
            logger.warning(f"生成相关主题失败: {e}")
            return []
    
    def _generate_scenario_topics(self,
                                 topics: List[str],
                                 num_topics: int = 5) -> List[str]:
        """生成场景主题（场景扩展）
        
        Args:
            topics: 原始主题列表
            num_topics: 生成数量
        
        Returns:
            扩展的主题列表
        """
        prompt = f"""基于以下租房相关主题，生成 {num_topics} 个具体使用场景。

原始主题：
{json.dumps(topics, ensure_ascii=False)}

要求：
1. 描述具体的使用场景或情况
2. 用户在这些场景下可能会有问题
3. 具有实际意义
4. 只输出场景列表，不要解释

使用场景："""
        
        try:
            response = self.model_backend.generate(prompt)
            topics_text = response.strip()
            if topics_text.startswith('['):
                expanded = json.loads(topics_text)
                if isinstance(expanded, list):
                    return expanded[:num_topics]
            expanded = [t.strip().strip('0123456789.-、 ') for t in topics_text.split('\n') if t.strip()]
            return expanded[:num_topics]
        except Exception as e:
            logger.warning(f"生成场景主题失败: {e}")
            return []
    
    def expand(self,
              items: List[Dict],
              strategy: str = "similar",
              num_topics: int = 5) -> ExpansionResult:
        """执行领域扩展
        
        Args:
            items: 数据列表
            strategy: 扩展策略 (similar/related/scenario)
            num_topics: 生成数量
        
        Returns:
            ExpansionResult 实例
        """
        # 提取原始主题
        original_topics = self._extract_topics(items)
        
        # 根据策略生成扩展主题
        if strategy == "similar":
            expanded_topics = self._generate_similar_topics(original_topics, num_topics)
        elif strategy == "related":
            expanded_topics = self._generate_related_topics(original_topics, num_topics)
        elif strategy == "scenario":
            expanded_topics = self._generate_scenario_topics(original_topics, num_topics)
        else:
            raise ValueError(f"不支持的扩展策略: {strategy}")
        
        # 构建扩展结果
        expanded = [
            {"topic": topic, "strategy": strategy}
            for topic in expanded_topics
        ]
        
        # 生成建议
        suggestions = []
        if len(original_topics) < 3:
            suggestions.append("原始主题较少，建议增加更多种子数据")
        if not expanded_topics:
            suggestions.append("扩展失败，建议检查模型配置或调整策略")
        
        return ExpansionResult(
            original_topics=original_topics,
            expanded_topics=expanded,
            strategy=strategy,
            suggestions=suggestions
        )
    
    def batch_expand(self,
                    items: List[Dict],
                    strategies: Optional[List[str]] = None,
                    topics_per_strategy: int = 5) -> List[Dict]:
        """批量扩展（使用多种策略）
        
        Args:
            items: 数据列表
            strategies: 策略列表
            topics_per_strategy: 每种策略生成的主题数
        
        Returns:
            扩展结果列表
        """
        if strategies is None:
            strategies = ["similar", "related", "scenario"]
        
        all_expanded = []
        
        for strategy in strategies:
            result = self.expand(items, strategy, topics_per_strategy)
            all_expanded.extend(result.expanded_topics)
        
        return all_expanded
    
    def generate_seeds_from_topics(self,
                                  topics: List[Dict],
                                  num_questions_per_topic: int = 3) -> List[Dict]:
        """从扩展的主题生成种子问答对
        
        Args:
            topics: 扩展的主题列表
            num_questions_per_topic: 每个主题生成的问题数
        
        Returns:
            种子问答对列表
        """
        all_seeds = []
        
        for topic_info in topics:
            topic = topic_info.get("topic", "")
            
            prompt = f"""基于以下主题，生成 {num_questions_per_topic} 个用户可能会问的问题，以及对应的客服回答。

主题：{topic}

要求：
1. 问题要自然、真实
2. 回答要专业、有帮助
3. 格式为 JSON 数组
4. 每个元素包含 "instruction" 和 "output" 字段

示例格式：
[
    {{"instruction": "问题1", "output": "回答1"}},
    {{"instruction": "问题2", "output": "回答2"}}
]

生成结果："""
            
            try:
                response = self.model_backend.generate(prompt)
                # 解析响应
                if response.strip().startswith('['):
                    seeds = json.loads(response.strip())
                    if isinstance(seeds, list):
                        all_seeds.extend(seeds[:num_questions_per_topic])
            except Exception as e:
                logger.warning(f"从主题生成种子失败: {topic}, {e}")
        
        return all_seeds
    
    def generate_report(self, items: List[Dict]) -> Dict:
        """生成扩展报告
        
        Args:
            items: 数据列表
        
        Returns:
            扩展报告字典
        """
        original_topics = self._extract_topics(items)
        
        return {
            "original_topic_count": len(original_topics),
            "original_topics": original_topics,
            "supported_strategies": ["similar", "related", "scenario"],
            "recommendations": [
                "使用 similar 策略扩展相似主题",
                "使用 related 策略扩展相关主题",
                "使用 scenario 策略扩展使用场景"
            ]
        }
