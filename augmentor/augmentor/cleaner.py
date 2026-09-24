# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据清洗增强模块

提供数据清洗、标准化和增强功能。

分工边界（与 `augmentor.data.cleaner`）
    本模块提供 `DatasetCleaner` + `TextNormalizer` 与规则化清洗
    （`clean_dataset` / `clean_batch_optimized`）。**CLI 的 `clean` 命令走这里**。

    3.0 起 CLI 只用本模块。为此 `data.cleaner.DataCleaner` 的能力已被**并进来**
    而不是丢掉：它的噪声清除三件套对应本模块的 `remove_urls` /
    `remove_html_tags` / `remove_control_chars` 三条规则，`remove_urls=True`
    等价于默认规则集里带 `remove_urls`，`--no-url-removal` 等价于把
    `remove_urls` 从 `--rules` 里去掉。`data.cleaner` 仍由 REST API 的清洗端点使用。

    注意 `remove_urls` 与 `remove_special_chars` **不是**同一件事：
    `remove_special_chars` 删的是「非中文/英文/数字/常用标点」的字符，
    它只会把 `https://a.com` 削成 `https:a.com`，并不能移除 URL。
"""

import re
import logging
from typing import List, Dict, Optional, Any, Callable
from dataclasses import dataclass, field
from pathlib import Path

from .validation import require_count

logger = logging.getLogger(__name__)

# 噪声清除用的模式。与 `augmentor.data.cleaner` 保持一致，避免同一份数据
# 在 CLI 与 REST API 两条路径上被清成不同结果。
URL_PATTERN = re.compile(r"https?://\S+|www\.\S+")
HTML_TAG_PATTERN = re.compile(r"<[^>]+>")
# 控制字符 + 零宽字符（零宽空格/连接符/不连字符/BOM）
CONTROL_CHAR_PATTERN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f\x7f\u200b\u200c\u200d\ufeff]")


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
        
        # 默认规则。字典顺序即 `clean(rules=None)` 的执行顺序，因此噪声清除
        # （URL / HTML 标签 / 控制字符）必须排在 `normalize_whitespace` 之前：
        # 删掉 URL 会在原位留下多余空白，先删后归一才不用归一两次。
        self._default_rules = {
            "remove_empty": self._remove_empty,
            "remove_duplicates": self._remove_duplicates,
            "remove_urls": self._remove_urls,
            "remove_html_tags": self._remove_html_tags,
            "remove_control_chars": self._remove_control_chars,
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
        # 必须逐条浅拷贝：规则是就地改 `item[field]` 的，只做 `items.copy()`
        # （浅拷贝）会把调用方传进来的 dict 一起改掉——`clean_dataset(items)`
        # 之后 `items` 就不是原数据了。
        cleaned_items = [dict(item) for item in items]
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
    
    def _remove_urls(self, items: List[Dict], fields: List[str]) -> tuple:
        """移除 URL

        移植自 `data.cleaner.DataCleaner.remove_noise`：那条路径默认移除 URL，
        合并到规则式实现时若丢掉这条规则，`clean` 的默认行为就会静默变化。

        Args:
            items: 数据列表
            fields: 字段列表

        Returns:
            (清洗后的数据, 是否修改)
        """
        return self._sub_in_fields(items, fields, URL_PATTERN, " ")

    def _remove_html_tags(self, items: List[Dict], fields: List[str]) -> tuple:
        """移除 HTML 标签

        Args:
            items: 数据列表
            fields: 字段列表

        Returns:
            (清洗后的数据, 是否修改)
        """
        return self._sub_in_fields(items, fields, HTML_TAG_PATTERN, " ")

    def _remove_control_chars(self, items: List[Dict], fields: List[str]) -> tuple:
        """移除控制字符与零宽字符

        Args:
            items: 数据列表
            fields: 字段列表

        Returns:
            (清洗后的数据, 是否修改)
        """
        return self._sub_in_fields(items, fields, CONTROL_CHAR_PATTERN, "")

    def _sub_in_fields(self, items: List[Dict], fields: List[str],
                       pattern, replacement: str) -> tuple:
        """对每个字段做一次正则替换（三条噪声清除规则的公共实现）

        Args:
            items: 数据列表
            fields: 字段列表
            pattern: 已编译的正则
            replacement: 替换文本

        Returns:
            (清洗后的数据, 是否修改)
        """
        modified = False

        for item in items:
            for field in fields:
                if field in item and isinstance(item[field], str):
                    new_value = pattern.sub(replacement, item[field])
                    if new_value != item[field]:
                        item[field] = new_value
                        modified = True

        return items, modified

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
            top_k: 返回数量，不小于 0 的整数；0 是「一个关键词都不要」，
                与「没传参数」是两件事。越界值报错而不是落到 `[:top_k]` 上被
                读成「丢掉末尾 |top_k| 个」（见 `augmentor.validation.require_count`）
        
        Returns:
            关键词列表
        """
        require_count("top_k", top_k)
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
        batch_size: 批处理大小，不小于 1 的整数（判据见 `require_count`）
    
    Returns:
        (清洗后的数据, 清洗结果汇总)
    """
    # `batch_size` 是 `range()` 的步长，越界值在这行不报错、只改产物：0 抛裸
    # `ValueError: range() arg 3 must not be zero`（经 API 就是 500 而不是 400），
    # -1 则**静默交出空数据集**而清洗报告仍写 original_count=10（实测 10 条进 0 条出）。
    # 步长没有「0 条」这种合法读法，故 minimum=1。
    require_count("batch_size", batch_size, minimum=1)

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
