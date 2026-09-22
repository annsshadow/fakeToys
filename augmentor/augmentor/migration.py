# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集迁移模块

提供数据集迁移和转换功能。
"""

import json
import logging
from typing import List, Dict, Optional, Any, Callable
from dataclasses import dataclass, field
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class MigrationRule:
    """迁移规则"""
    rule_id: str
    name: str
    description: str
    source_field: str
    target_field: str
    transform_func: Optional[Callable] = None
    default_value: Any = None
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "rule_id": self.rule_id,
            "name": self.name,
            "description": self.description,
            "source_field": self.source_field,
            "target_field": self.target_field,
            "default_value": self.default_value
        }


@dataclass
class MigrationResult:
    """迁移结果"""
    migration_id: str
    source_path: str
    target_path: str
    total_items: int
    migrated_items: int
    failed_items: int
    rules_applied: List[str]
    errors: List[Dict] = field(default_factory=list)
    timestamp: str = ""
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "migration_id": self.migration_id,
            "source_path": self.source_path,
            "target_path": self.target_path,
            "total_items": self.total_items,
            "migrated_items": self.migrated_items,
            "failed_items": self.failed_items,
            "rules_applied": self.rules_applied,
            "errors": self.errors[:10],  # 只返回前10个错误
            "timestamp": self.timestamp
        }


class DatasetMigrator:
    """数据集迁移器
    
    提供数据集迁移和转换功能。
    """
    
    def __init__(self):
        """初始化迁移器"""
        self._rules: List[MigrationRule] = []
        self._builtin_rules = self._create_builtin_rules()
    
    def _create_builtin_rules(self) -> List[MigrationRule]:
        """创建内置规则
        
        Returns:
            规则列表
        """
        return [
            MigrationRule(
                rule_id="rename_instruction",
                name="重命名instruction",
                description="将instruction字段重命名为question",
                source_field="instruction",
                target_field="question"
            ),
            MigrationRule(
                rule_id="rename_output",
                name="重命名output",
                description="将output字段重命名为answer",
                source_field="output",
                target_field="answer"
            ),
            MigrationRule(
                rule_id="flatten_conversations",
                name="展平对话",
                description="将对话格式展平为问答对",
                source_field="conversations",
                target_field="instruction",
                transform_func=self._flatten_conversations
            ),
        ]
    
    def _flatten_conversations(self, value: Any) -> str:
        """展平对话
        
        Args:
            value: 对话数据
        
        Returns:
            展平后的文本
        """
        if isinstance(value, list):
            parts = []
            for item in value:
                if isinstance(item, dict):
                    role = item.get("from", item.get("role", ""))
                    content = item.get("value", item.get("content", ""))
                    parts.append(f"{role}: {content}")
            return "\n".join(parts)
        return str(value)
    
    def add_rule(self, rule: MigrationRule):
        """添加规则
        
        Args:
            rule: 迁移规则
        """
        self._rules.append(rule)
    
    def migrate(self, items: List[Dict], rules: List[str] = None,
               output_path: str = None) -> MigrationResult:
        """执行迁移
        
        Args:
            items: 数据列表
            rules: 要应用的规则列表
            output_path: 输出路径
        
        Returns:
            迁移结果
        """
        import hashlib
        
        # 获取要应用的规则
        if rules:
            applied_rules = [r for r in self._rules + self._builtin_rules if r.rule_id in rules]
        else:
            applied_rules = self._rules + self._builtin_rules
        
        migrated_items = []
        errors = []
        
        for i, item in enumerate(items):
            try:
                new_item = self._apply_rules(item, applied_rules)
                migrated_items.append(new_item)
            except Exception as e:
                errors.append({
                    "index": i,
                    "error": str(e),
                    "item": str(item)[:100]
                })
        
        # 保存到文件
        if output_path:
            output = Path(output_path)
            output.parent.mkdir(parents=True, exist_ok=True)
            with open(output, 'w', encoding='utf-8') as f:
                json.dump(migrated_items, f, ensure_ascii=False, indent=2)
        
        result = MigrationResult(
            migration_id=hashlib.md5(str(datetime.now()).encode()).hexdigest()[:8],
            source_path="",
            target_path=output_path or "",
            total_items=len(items),
            migrated_items=len(migrated_items),
            failed_items=len(errors),
            rules_applied=[r.rule_id for r in applied_rules],
            errors=errors,
            timestamp=datetime.now().isoformat()
        )
        
        return result
    
    def _apply_rules(self, item: Dict, rules: List[MigrationRule]) -> Dict:
        """应用规则
        
        Args:
            item: 数据项
            rules: 规则列表
        
        Returns:
            转换后的数据项
        """
        new_item = item.copy()
        
        for rule in rules:
            if rule.source_field in new_item:
                value = new_item.pop(rule.source_field)
                
                # 应用转换函数
                if rule.transform_func:
                    value = rule.transform_func(value)
                
                # 设置目标字段
                new_item[rule.target_field] = value
            elif rule.default_value is not None:
                new_item[rule.target_field] = rule.default_value
        
        return new_item
    
    def migrate_file(self, source_path: str, target_path: str,
                    rules: List[str] = None) -> MigrationResult:
        """迁移文件
        
        Args:
            source_path: 源文件路径
            target_path: 目标文件路径
            rules: 规则列表
        
        Returns:
            迁移结果
        """
        with open(source_path, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        result = self.migrate(items, rules, target_path)
        result.source_path = source_path
        
        return result


def migrate_dataset(items: List[Dict], rules: List[str] = None,
                   output_path: str = None) -> MigrationResult:
    """迁移数据集
    
    Args:
        items: 数据列表
        rules: 规则列表
        output_path: 输出路径
    
    Returns:
        迁移结果
    """
    migrator = DatasetMigrator()
    return migrator.migrate(items, rules, output_path)


def migrate_file(source_path: str, target_path: str,
                rules: List[str] = None) -> MigrationResult:
    """迁移文件
    
    Args:
        source_path: 源文件路径
        target_path: 目标文件路径
        rules: 规则列表
    
    Returns:
        迁移结果
    """
    migrator = DatasetMigrator()
    return migrator.migrate_file(source_path, target_path, rules)
