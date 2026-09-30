# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""自动领域扩展模块 - 优化版"""

import json
import logging
from typing import List, Dict, Optional
from dataclasses import dataclass
from concurrent.futures import ThreadPoolExecutor, as_completed
from .models.base import ModelBackend, extract_json_array
from .exceptions import DataValidationError
from .validation import require_count

logger = logging.getLogger(__name__)


@dataclass
class ExpansionResult:
    """扩展结果"""
    original_topics: List[str]  # 原始主题
    expanded_topics: List[Dict]  # 扩展的主题
    strategy: str  # 使用的策略
    suggestions: List[str]  # 建议


class DomainExpander:
    """领域扩展器 - 优化版"""
    
    def __init__(self, model_backend: ModelBackend, max_workers: int = 3):
        """初始化领域扩展器
        
        Args:
            model_backend: 模型后端
            max_workers: 最大并发数
        """
        self.model_backend = model_backend
        self.max_workers = max_workers
    
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
    
    @staticmethod
    def _parse_topic_list(response: str, num_topics: int) -> List[str]:
        """把模型返回的主题响应解析成列表：先当 JSON 数组（含 ```json 围栏与前置
        一句说明的散文形态），解析不出再按行分割并剥序号。

        改前三处主题生成器都用手搓判据 `if topics_text.startswith('[')` ⇒ 模型把
        数组包进代码围栏或先说一句「好的，结果如下：」时首字符不是 `[`，直接掉进
        「按行分割」，把 "```json" / '["租房","公寓"]' 这类碎片当成主题塞进结果、
        污染下游（A110，与 L59 的 A108 同构）。走 `extract_json_array`（仓库里各
        adapter 早已在用）后：围栏/散文包裹的合法数组能正常出主题。

        对「解析不出」的两类响应维持改前口径、不新增回归：文本以 `[` 开头却解析
        失败 = 内容坏了 ⇒ 返回空（改前是 `json.loads` 抛异常被外层 except 收成空）；
        不以 `[` 开头 = 提示词 eliciting 的纯文本列表 ⇒ 照旧按行剥序号。
        """
        text = response.strip()
        try:
            topics = extract_json_array(response)
        except Exception:
            if text.startswith('['):
                return []
            return [t.strip().strip('0123456789.-、 ')
                    for t in text.split('\n') if t.strip()][:num_topics]
        return topics[:num_topics]
    
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
            return self._parse_topic_list(response, num_topics)
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
            return self._parse_topic_list(response, num_topics)
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
            return self._parse_topic_list(response, num_topics)
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
            num_topics: 生成数量，不小于 0 的整数（判据见 `require_count`）
        
        Returns:
            ExpansionResult 实例
        """
        # 判参先于模型调用：负数会被原样写进提示词，再被 `[:-1]` 读成「除了末位」
        require_count("num_topics", num_topics)

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
            raise DataValidationError(f"不支持的扩展策略: {strategy}")
        
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
                    topics_per_strategy: int = 5,
                    use_parallel: bool = True) -> List[Dict]:
        """批量扩展（使用多种策略，优化版）
        
        Args:
            items: 数据列表
            strategies: 策略列表
            topics_per_strategy: 每种策略生成的主题数，不小于 0 的整数
                （判据见 `require_count`）
            use_parallel: 是否使用并行处理
        
        Returns:
            扩展结果列表
        """
        require_count("topics_per_strategy", topics_per_strategy)

        if strategies is None:
            strategies = ["similar", "related", "scenario"]
        
        if not use_parallel or len(strategies) <= 1:
            # 串行处理
            all_expanded = []
            for strategy in strategies:
                result = self.expand(items, strategy, topics_per_strategy)
                all_expanded.extend(result.expanded_topics)
            return all_expanded
        
        # 并行处理
        all_expanded = []
        
        def expand_with_strategy(strategy):
            return strategy, self.expand(items, strategy, topics_per_strategy)
        
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = [executor.submit(expand_with_strategy, s) for s in strategies]
            
            for future in as_completed(futures):
                try:
                    strategy, result = future.result()
                    all_expanded.extend(result.expanded_topics)
                    logger.info(f"完成策略 {strategy} 的扩展")
                except Exception as e:
                    logger.error(f"扩展失败: {e}")
        
        return all_expanded
    
    def generate_seeds_from_topics(self,
                                   topics: List[Dict],
                                   num_questions_per_topic: int = 3,
                                   use_parallel: bool = True) -> List[Dict]:
        """从扩展的主题生成种子问答对（优化版）
        
        Args:
            topics: 扩展的主题列表
            num_questions_per_topic: 每个主题生成的问题数，不小于 0 的整数
                （判据见 `require_count`）
            use_parallel: 是否使用并行处理
        
        Returns:
            种子问答对列表
        """
        require_count("num_questions_per_topic", num_questions_per_topic)

        if not use_parallel or len(topics) <= 1:
            # 串行处理
            all_seeds = []
            for topic_info in topics:
                seeds = self._generate_seeds_for_topic(topic_info, num_questions_per_topic)
                all_seeds.extend(seeds)
            return all_seeds
        
        # 并行处理
        all_seeds = []
        
        def generate_for_topic(topic_info):
            return self._generate_seeds_for_topic(topic_info, num_questions_per_topic)
        
        with ThreadPoolExecutor(max_workers=self.max_workers) as executor:
            futures = [executor.submit(generate_for_topic, t) for t in topics]
            
            for future in as_completed(futures):
                try:
                    seeds = future.result()
                    all_seeds.extend(seeds)
                except Exception as e:
                    logger.error(f"生成种子失败: {e}")
        
        return all_seeds
    
    def _generate_seeds_for_topic(self,
                                  topic_info: Dict,
                                  num_questions: int) -> List[Dict]:
        """为单个主题生成种子问答对
        
        Args:
            topic_info: 主题信息
            num_questions: 生成的问题数
        
        Returns:
            种子问答对列表
        """
        topic = topic_info.get("topic", "")
        
        prompt = f"""基于以下主题，生成 {num_questions} 个用户可能会问的问题，以及对应的客服回答。

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
            # 提示词明确要求「格式为 JSON 数组」⇒ 走仓库自带 robust 解析（直接数组 /
            # ```json 围栏 / 前置一句说明的散文都能取到），而不是改前那句手搓判据
            # `if response.startswith('[')` ⇒ 后者让围栏/散文包裹的合法数组静默返回
            # 0 条种子、连 warning 都不发（A110，与 L59 的 A108 同构）。解析不出即抛
            # ModelResponseError，由下面的 except 收成一行 warning。
            seeds = extract_json_array(response)
            return seeds[:num_questions]
        except Exception as e:
            logger.warning(f"从主题生成种子失败: {topic}, {e}")
        
        return []
    
    def generate_adaptive_expansion(self,
                                     items: List[Dict],
                                     target_domain: Optional[str] = None) -> ExpansionResult:
        """领域自适应增强（根据数据内容自动适配领域）
        
        Args:
            items: 数据列表
            target_domain: 目标领域（可选），为 None 时自动识别
        
        Returns:
            适配后的扩展结果
        """
        original_topics = self._extract_topics(items)
        
        # 自动识别领域（简化实现）
        detected_domain = target_domain or self._detect_domain(items)
        
        # 根据领域调整策略
        if detected_domain == "租房服务":
            strategy = "scenario"
            num_topics = 8
        elif detected_domain == "合同法律":
            strategy = "related"
            num_topics = 6
        else:
            strategy = "similar"
            num_topics = 5
        
        expanded_topics = self._generate_similar_topics(original_topics, num_topics)
        
        # 添加领域适配建议
        suggestions = [
            f"检测到领域: {detected_domain}",
            f"自动选择策略: {strategy}",
            f"生成 {len(expanded_topics)} 个扩展主题"
        ]
        
        return ExpansionResult(
            original_topics=original_topics,
            expanded_topics=[{"topic": t, "strategy": strategy, "adapted_domain": detected_domain} for t in expanded_topics],
            strategy=strategy,
            suggestions=suggestions
        )
    
    def _detect_domain(self, items: List[Dict]) -> str:
        """自动检测领域类型（简化实现）
        
        Args:
            items: 数据列表
        
        Returns:
            检测到的领域名称
        """
        all_text = " ".join(str(item.get("instruction", "")) for item in items)
        domain_keywords = {
            "租房服务": ["租房", "房源", "看房", "签约"],
            "合同法律": ["合同", "法律", "条款", "维权", "投诉"],
            "维修服务": ["维修", "保养", "设备", "故障"],
            "费用相关": ["租金", "押金", "费用", "账单"]
        }
        
        best_domain = "通用领域"
        best_score = 0
        
        for domain, keywords in domain_keywords.items():
            score = sum(1 for kw in keywords if kw in all_text)
            if score > best_score:
                best_score = score
                best_domain = domain
        
        return best_domain
    
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
