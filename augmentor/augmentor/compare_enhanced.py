"""数据集比较增强模块

提供更全面的数据集比较功能。
"""

import logging
from typing import List, Dict, Optional, Any, Set, Tuple
from dataclasses import dataclass, field

logger = logging.getLogger(__name__)


@dataclass
class ComparisonMetrics:
    """比较指标"""
    size_a: int
    size_b: int
    size_diff: int
    size_ratio: float
    common_items: int
    unique_a: int
    unique_b: int
    similarity_score: float
    field_overlap: float
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "size_a": self.size_a,
            "size_b": self.size_b,
            "size_diff": self.size_diff,
            "size_ratio": self.size_ratio,
            "common_items": self.common_items,
            "unique_a": self.unique_a,
            "unique_b": self.unique_b,
            "similarity_score": self.similarity_score,
            "field_overlap": self.field_overlap
        }


@dataclass
class FieldComparison:
    """字段比较结果"""
    field_name: str
    in_a_only: int
    in_b_only: int
    in_both: int
    type_mismatches: int
    value_differences: List[Dict] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "field_name": self.field_name,
            "in_a_only": self.in_a_only,
            "in_b_only": self.in_b_only,
            "in_both": self.in_both,
            "type_mismatches": self.type_mismatches,
            "value_differences": self.value_differences[:10]  # 只返回前10个
        }


@dataclass
class EnhancedComparisonResult:
    """增强比较结果"""
    dataset_a_name: str
    dataset_b_name: str
    metrics: ComparisonMetrics
    field_comparisons: Dict[str, FieldComparison] = field(default_factory=dict)
    summary: str = ""
    recommendations: List[str] = field(default_factory=list)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "dataset_a_name": self.dataset_a_name,
            "dataset_b_name": self.dataset_b_name,
            "metrics": self.metrics.to_dict(),
            "field_comparisons": {
                name: comp.to_dict()
                for name, comp in self.field_comparisons.items()
            },
            "summary": self.summary,
            "recommendations": self.recommendations
        }


class EnhancedComparator:
    """增强比较器
    
    提供更全面的数据集比较功能。
    """
    
    def __init__(self):
        """初始化比较器"""
        pass
    
    def compare(self, items_a: List[Dict], items_b: List[Dict],
                name_a: str = "dataset_a", name_b: str = "dataset_b",
                key_fields: List[str] = None) -> EnhancedComparisonResult:
        """比较两个数据集
        
        Args:
            items_a: 数据集A
            items_b: 数据集B
            name_a: 数据集A名称
            name_b: 数据集B名称
            key_fields: 用于唯一标识的字段
        
        Returns:
            比较结果
        """
        key_fields = key_fields or ["instruction"]
        
        # 计算指标
        metrics = self._calculate_metrics(items_a, items_b, key_fields)
        
        # 比较字段
        field_comparisons = self._compare_fields(items_a, items_b)
        
        # 生成摘要
        summary = self._generate_summary(metrics, field_comparisons)
        
        # 生成建议
        recommendations = self._generate_recommendations(metrics, field_comparisons)
        
        return EnhancedComparisonResult(
            dataset_a_name=name_a,
            dataset_b_name=name_b,
            metrics=metrics,
            field_comparisons=field_comparisons,
            summary=summary,
            recommendations=recommendations
        )
    
    def _calculate_metrics(self, items_a: List[Dict], items_b: List[Dict],
                          key_fields: List[str]) -> ComparisonMetrics:
        """计算比较指标
        
        Args:
            items_a: 数据集A
            items_b: 数据集B
            key_fields: 关键字段
        
        Returns:
            比较指标
        """
        # 获取唯一标识
        keys_a = set(self._get_keys(items_a, key_fields))
        keys_b = set(self._get_keys(items_b, key_fields))
        
        # 计算交集和差集
        common_keys = keys_a & keys_b
        unique_a = keys_a - keys_b
        unique_b = keys_b - keys_a
        
        # 计算相似度
        all_keys = keys_a | keys_b
        similarity = len(common_keys) / len(all_keys) if all_keys else 0
        
        # 计算字段重叠
        fields_a = set()
        fields_b = set()
        for item in items_a:
            fields_a.update(item.keys())
        for item in items_b:
            fields_b.update(item.keys())
        
        common_fields = fields_a & fields_b
        all_fields = fields_a | fields_b
        field_overlap = len(common_fields) / len(all_fields) if all_fields else 0
        
        return ComparisonMetrics(
            size_a=len(items_a),
            size_b=len(items_b),
            size_diff=len(items_a) - len(items_b),
            size_ratio=len(items_a) / len(items_b) if items_b else float('inf'),
            common_items=len(common_keys),
            unique_a=len(unique_a),
            unique_b=len(unique_b),
            similarity_score=similarity,
            field_overlap=field_overlap
        )
    
    def _get_keys(self, items: List[Dict], key_fields: List[str]) -> List[str]:
        """获取唯一标识
        
        Args:
            items: 数据列表
            key_fields: 关键字段
        
        Returns:
            标识列表
        """
        keys = []
        for item in items:
            key_parts = []
            for field in key_fields:
                value = item.get(field, "")
                key_parts.append(str(value))
            keys.append("|".join(key_parts))
        return keys
    
    def _compare_fields(self, items_a: List[Dict], items_b: List[Dict]) -> Dict[str, FieldComparison]:
        """比较字段
        
        Args:
            items_a: 数据集A
            items_b: 数据集B
        
        Returns:
            字段比较结果
        """
        # 收集所有字段
        all_fields = set()
        for item in items_a:
            all_fields.update(item.keys())
        for item in items_b:
            all_fields.update(item.keys())
        
        field_comparisons = {}
        
        for field in all_fields:
            # 计算字段出现次数
            count_a = sum(1 for item in items_a if field in item)
            count_b = sum(1 for item in items_b if field in item)
            
            in_both = min(count_a, count_b)
            in_a_only = count_a - in_both
            in_b_only = count_b - in_both
            
            # 检查类型不匹配
            type_mismatches = 0
            value_differences = []
            
            # 比较值
            for i, item_a in enumerate(items_a):
                if field in item_a:
                    value_a = item_a[field]
                    type_a = type(value_a).__name__
                    
                    # 在数据集B中查找相同指令
                    instruction = item_a.get("instruction", "")
                    for item_b in items_b:
                        if item_b.get("instruction") == instruction and field in item_b:
                            value_b = item_b[field]
                            type_b = type(value_b).__name__
                            
                            if type_a != type_b:
                                type_mismatches += 1
                            
                            if value_a != value_b:
                                value_differences.append({
                                    "instruction": instruction,
                                    "value_a": str(value_a)[:100],
                                    "value_b": str(value_b)[:100]
                                })
                            break
            
            field_comparisons[field] = FieldComparison(
                field_name=field,
                in_a_only=in_a_only,
                in_b_only=in_b_only,
                in_both=in_both,
                type_mismatches=type_mismatches,
                value_differences=value_differences
            )
        
        return field_comparisons
    
    def _generate_summary(self, metrics: ComparisonMetrics,
                         field_comparisons: Dict[str, FieldComparison]) -> str:
        """生成摘要
        
        Args:
            metrics: 比较指标
            field_comparisons: 字段比较结果
        
        Returns:
            摘要文本
        """
        lines = [
            f"数据集比较摘要:",
            f"  数据集A: {metrics.size_a} 条",
            f"  数据集B: {metrics.size_b} 条",
            f"  相似度: {metrics.similarity_score:.2%}",
            f"  共同数据: {metrics.common_items} 条",
            f"  仅在A中: {metrics.unique_a} 条",
            f"  仅在B中: {metrics.unique_b} 条"
        ]
        
        return "\n".join(lines)
    
    def _generate_recommendations(self, metrics: ComparisonMetrics,
                                 field_comparisons: Dict[str, FieldComparison]) -> List[str]:
        """生成建议
        
        Args:
            metrics: 比较指标
            field_comparisons: 字段比较结果
        
        Returns:
            建议列表
        """
        recommendations = []
        
        # 数据量建议
        if metrics.size_diff > 100:
            recommendations.append(f"数据集A比B多 {metrics.size_diff} 条数据，建议检查数据来源是否一致")
        elif metrics.size_diff < -100:
            recommendations.append(f"数据集B比A多 {abs(metrics.size_diff)} 条数据，建议检查数据来源是否一致")
        
        # 相似度建议
        if metrics.similarity_score < 0.5:
            recommendations.append("数据集相似度较低，建议确认是否为同一数据源")
        elif metrics.similarity_score > 0.9:
            recommendations.append("数据集高度相似，可能存在重复数据")
        
        # 字段建议
        for field_name, comp in field_comparisons.items():
            if comp.type_mismatches > 0:
                recommendations.append(f"字段 '{field_name}' 存在 {comp.type_mismatches} 个类型不匹配")
            if comp.value_differences:
                recommendations.append(f"字段 '{field_name}' 存在 {len(comp.value_differences)} 个值差异")
        
        return recommendations


def compare_datasets_enhanced(items_a: List[Dict], items_b: List[Dict],
                             name_a: str = "dataset_a", name_b: str = "dataset_b",
                             key_fields: List[str] = None) -> EnhancedComparisonResult:
    """增强数据集比较
    
    Args:
        items_a: 数据集A
        items_b: 数据集B
        name_a: 数据集A名称
        name_b: 数据集B名称
        key_fields: 关键字段
    
    Returns:
        比较结果
    """
    comparator = EnhancedComparator()
    return comparator.compare(items_a, items_b, name_a, name_b, key_fields)


def diff_datasets(items_a: List[Dict], items_b: List[Dict],
                 key_fields: List[str] = None) -> Dict:
    """计算数据集差异
    
    Args:
        items_a: 数据集A
        items_b: 数据集B
        key_fields: 关键字段
    
    Returns:
        差异信息
    """
    key_fields = key_fields or ["instruction"]
    
    # 获取唯一标识
    keys_a = set()
    keys_b = set()
    
    for item in items_a:
        key_parts = [str(item.get(f, "")) for f in key_fields]
        keys_a.add("|".join(key_parts))
    
    for item in items_b:
        key_parts = [str(item.get(f, "")) for f in key_fields]
        keys_b.add("|".join(key_parts))
    
    return {
        "only_in_a": list(keys_a - keys_b),
        "only_in_b": list(keys_b - keys_a),
        "in_both": list(keys_a & keys_b),
        "stats": {
            "only_in_a_count": len(keys_a - keys_b),
            "only_in_b_count": len(keys_b - keys_a),
            "in_both_count": len(keys_a & keys_b)
        }
    }
