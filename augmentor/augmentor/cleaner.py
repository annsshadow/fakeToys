"""数据清洗增强模块

提供数据清洗、标准化和增强功能。

分工边界（与 `augmentor.data.cleaner`）
    本模块提供 `DatasetCleaner` + `TextNormalizer` 与规则化清洗
    （`clean_dataset` / `clean_batch_optimized`）。CLI 的 `clean-enhanced` 命令走这里。
    `augmentor.data.cleaner.DataCleaner` 是另一套实现，由 CLI 的 `clean` 命令与
    REST API 的清洗端点使用；两者是**同一能力的两种实现**，
    `CleaningResult` 与 `CleanResult` 也是同一概念的两种命名——这一对才是真正
    值得合并的（合并需先统一结果类型，登记为后续任务）。
"""

import re
import logging
from typing import List, Dict, Optional, Any, Callable
from dataclasses import dataclass, field
from pathlib import Path

logger = logging.getLogger(__name__)


@dataclass
class CleaningRule:
    """清洗规则"""
    name: str
    description: str
    enabled: bool = True
    priority: int = 0


@dataclass
class CleaningResult:
    """清洗结果"""
    original_count: int
    cleaned_count: int
    removed_count: int
    modified_count: int
    rules_applied: List[str]
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "original_count": self.original_count,
            "cleaned_count": self.cleaned_count,
            "removed_count": self.removed_count,
            "modified_count": self.modified_count,
            "rules_applied": self.rules_applied
        }


class DatasetCleaner:
    """数据集清洗器
    
    提供数据清洗、标准化和增强功能。
    """
    
    def __init__(self):
        """初始化清洗器"""
        self._rules: List[CleaningRule] = []
        self._custom_rules: List[Callable] = []
        
        # 默认规则
        self._default_rules = {
            "remove_empty": self._remove_empty,
            "remove_duplicates": self._remove_duplicates,
            "normalize_whitespace": self._normalize_whitespace,
            "remove_special_chars": self._remove_special_chars,
            "trim_whitespace": self._trim_whitespace,
            "remove_long_texts": self._remove_long_texts,
            "remove_short_texts": self._remove_short_texts,
            "normalize_punctuation": self._normalize_punctuation,
        }
    
    def add_rule(self, rule: CleaningRule):
        """添加清洗规则
        
        Args:
            rule: 清洗规则
        """
        self._rules.append(rule)
    
    def add_custom_rule(self, rule_func: Callable):
        """添加自定义清洗规则
        
        Args:
            rule_func: 清洗函数
        """
        self._custom_rules.append(rule_func)
    
    def clean(self, 
              items: List[Dict], 
              fields: List[str] = None,
              rules: List[str] = None) -> tuple:
        """清洗数据集
        
        Args:
            items: 数据列表
            fields: 要清洗的字段列表
            rules: 要应用的规则列表
        
        Returns:
            (清洗后的数据, 清洗结果)
        """
        fields = fields or ["instruction", "output", "input"]
        rules = rules or list(self._default_rules.keys())
        
        original_count = len(items)
        cleaned_items = items.copy()
        applied_rules = []
        modified_count = 0
        
        # 应用规则
        for rule_name in rules:
            if rule_name in self._default_rules:
                rule_func = self._default_rules[rule_name]
                cleaned_items, was_modified = rule_func(cleaned_items, fields)
                if was_modified:
                    applied_rules.append(rule_name)
                    modified_count += 1
        
        # 应用自定义规则
        for rule_func in self._custom_rules:
            cleaned_items = rule_func(cleaned_items)
            applied_rules.append(rule_func.__name__)
        
        result = CleaningResult(
            original_count=original_count,
            cleaned_count=len(cleaned_items),
            removed_count=original_count - len(cleaned_items),
            modified_count=modified_count,
            rules_applied=applied_rules
        )
        
        return cleaned_items, result
    
    def _remove_empty(self, items: List[Dict], fields: List[str]) -> tuple:
        """移除空数据
        
        Args:
            items: 数据列表
            fields: 字段列表
        
        Returns:
            (清洗后的数据, 是否修改)
        """
        original_count = len(items)
        cleaned = [item for item in items if any(item.get(field) for field in fields)]
        return cleaned, len(cleaned) != original_count
    
    def _remove_duplicates(self, items: List[Dict], fields: List[str]) -> tuple:
        """移除重复数据
        
        Args:
            items: 数据列表
            fields: 字段列表
        
        Returns:
            (清洗后的数据, 是否修改)
        """
        original_count = len(items)
        seen = set()
        cleaned = []
        
        for item in items:
            # 生成唯一标识
            key = tuple(item.get(field, "") for field in fields)
            if key not in seen:
                seen.add(key)
                cleaned.append(item)
        
        return cleaned, len(cleaned) != original_count
    
    def _normalize_whitespace(self, items: List[Dict], fields: List[str]) -> tuple:
        """标准化空白字符
        
        Args:
            items: 数据列表
            fields: 字段列表
        
        Returns:
            (清洗后的数据, 是否修改)
        """
        modified = False
        
        for item in items:
            for field in fields:
                if field in item and isinstance(item[field], str):
                    # 将多个空白字符替换为单个空格
                    new_value = re.sub(r'\s+', ' ', item[field])
                    if new_value != item[field]:
                        item[field] = new_value
                        modified = True
        
        return items, modified
    
    def _remove_special_chars(self, items: List[Dict], fields: List[str]) -> tuple:
        """移除特殊字符
        
        Args:
            items: 数据列表
            fields: 字段列表
        
        Returns:
            (清洗后的数据, 是否修改)
        """
        modified = False
        
        for item in items:
            for field in fields:
                if field in item and isinstance(item[field], str):
                    # 保留中文、英文、数字和常用标点
                    new_value = re.sub(r'[^\u4e00-\u9fff\w\s.,!?;:、。，！？；：\u201c\u201d\u2018\u2019（）\[\]【】]', '', item[field])
                    if new_value != item[field]:
                        item[field] = new_value
                        modified = True
        
        return items, modified
    
    def _trim_whitespace(self, items: List[Dict], fields: List[str]) -> tuple:
        """去除首尾空白
        
        Args:
            items: 数据列表
            fields: 字段列表
        
        Returns:
            (清洗后的数据, 是否修改)
        """
        modified = False
        
        for item in items:
            for field in fields:
                if field in item and isinstance(item[field], str):
                    new_value = item[field].strip()
                    if new_value != item[field]:
                        item[field] = new_value
                        modified = True
        
        return items, modified
    
    def _remove_long_texts(self, items: List[Dict], fields: List[str], max_length: int = 1000) -> tuple:
        """移除过长文本
        
        Args:
            items: 数据列表
            fields: 字段列表
            max_length: 最大长度
        
        Returns:
            (清洗后的数据, 是否修改)
        """
        original_count = len(items)
        
        cleaned = []
        for item in items:
            has_long = False
            for field in fields:
                if field in item and isinstance(item[field], str) and len(item[field]) > max_length:
                    has_long = True
                    break
            if not has_long:
                cleaned.append(item)
        
        return cleaned, len(cleaned) != original_count
    
    def _remove_short_texts(self, items: List[Dict], fields: List[str], min_length: int = 2) -> tuple:
        """移除过短文本
        
        Args:
            items: 数据列表
            fields: 字段列表
            min_length: 最小长度
        
        Returns:
            (清洗后的数据, 是否修改)
        """
        original_count = len(items)
        
        cleaned = []
        for item in items:
            has_short = False
            for field in fields:
                if field in item and isinstance(item[field], str) and 0 < len(item[field]) < min_length:
                    has_short = True
                    break
            if not has_short:
                cleaned.append(item)
        
        return cleaned, len(cleaned) != original_count
    
    def _normalize_punctuation(self, items: List[Dict], fields: List[str]) -> tuple:
        """标准化标点符号
        
        Args:
            items: 数据列表
            fields: 字段列表
        
        Returns:
            (清洗后的数据, 是否修改)
        """
        modified = False
        
        # 标点映射
        punctuation_map = {
            '，': ',', '。': '.', '！': '!', '？': '?',
            '；': ';', '：': ':', '\u201c': '"', '\u201d': '"',
            '\u2018': "'", '\u2019': "'", '（': '(', '）': ')',
            '【': '[', '】': ']',
        }
        
        for item in items:
            for field in fields:
                if field in item and isinstance(item[field], str):
                    new_value = item[field]
                    for old, new in punctuation_map.items():
                        new_value = new_value.replace(old, new)
                    if new_value != item[field]:
                        item[field] = new_value
                        modified = True
        
        return items, modified


class TextNormalizer:
    """文本标准化器
    
    提供文本标准化功能。
    """
    
    def __init__(self):
        """初始化标准化器"""
        pass
    
    def normalize(self, text: str) -> str:
        """标准化文本
        
        Args:
            text: 输入文本
        
        Returns:
            标准化后的文本
        """
        if not text:
            return text
        
        # 去除首尾空白
        text = text.strip()
        
        # 标准化空白字符
        text = re.sub(r'\s+', ' ', text)
        
        # 标准化标点
        text = self._normalize_punctuation(text)
        
        return text
    
    def _normalize_punctuation(self, text: str) -> str:
        """标准化标点符号
        
        Args:
            text: 输入文本
        
        Returns:
            标准化后的文本
        """
        # 简单的标点标准化
        # 将连续的标点替换为单个
        text = re.sub(r'([。！？.!?])\1+', r'\1', text)
        
        return text
    
    def extract_keywords(self, text: str, top_k: int = 10) -> List[str]:
        """提取关键词
        
        Args:
            text: 输入文本
            top_k: 返回数量
        
        Returns:
            关键词列表
        """
        # 简单的关键词提取
        # 使用TF-IDF思想，选择出现频率适中的词
        words = re.findall(r'[\u4e00-\u9fff]+|[a-zA-Z]+', text)
        
        # 词频统计
        word_freq = {}
        for word in words:
            if len(word) > 1:
                word_freq[word] = word_freq.get(word, 0) + 1
        
        # 按频率排序，选择前top_k个
        sorted_words = sorted(word_freq.items(), key=lambda x: x[1], reverse=True)
        
        return [word for word, _ in sorted_words[:top_k]]


def clean_dataset(items: List[Dict], 
                  fields: List[str] = None,
                  rules: List[str] = None) -> tuple:
    """清洗数据集
    
    Args:
        items: 数据列表
        fields: 要清洗的字段列表
        rules: 要应用的规则列表
    
    Returns:
        (清洗后的数据, 清洗结果)
    """
    cleaner = DatasetCleaner()
    return cleaner.clean(items, fields, rules)


def normalize_text(text: str) -> str:
    """标准化文本
    
    Args:
        text: 输入文本
    
    Returns:
        标准化后的文本
    """
    normalizer = TextNormalizer()
    return normalizer.normalize(text)


def extract_keywords(text: str, top_k: int = 10) -> List[str]:
    """提取关键词
    
    Args:
        text: 输入文本
        top_k: 返回数量
    
    Returns:
        关键词列表
    """
    normalizer = TextNormalizer()
    return normalizer.extract_keywords(text, top_k)


def clean_batch_optimized(items: List[Dict],
                           fields: List[str] = None,
                           rules: List[str] = None,
                           batch_size: int = 100) -> tuple:
    """批量清洗优化（增强功能：分批处理大数据集，减少内存峰值）
    
    Args:
        items: 数据列表
        fields: 要清洗的字段列表
        rules: 要应用的规则列表
        batch_size: 批处理大小
    
    Returns:
        (清洗后的数据, 清洗结果汇总)
    """
    cleaner = DatasetCleaner()
    cleaned_items = []
    total_modified = 0
    total_removed = 0
    
    # 批量处理优化：分批清洗避免一次性处理大数据集
    for i in range(0, len(items), batch_size):
        batch = items[i:i + batch_size]
        cleaned_batch, result = cleaner.clean(batch, fields, rules)
        cleaned_items.extend(cleaned_batch)
        total_modified += result.modified_count
        total_removed += result.removed_count
    
    final_result = CleaningResult(
        original_count=len(items),
        cleaned_count=len(cleaned_items),
        removed_count=total_removed,
        modified_count=total_modified,
        rules_applied=rules or []
    )
    return cleaned_items, final_result
