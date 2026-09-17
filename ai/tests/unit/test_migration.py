"""数据集迁移模块测试"""

import json
import pytest
from augmentor.migration import (
    DatasetMigrator, MigrationRule, MigrationResult,
    migrate_dataset, migrate_file
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
    ]


@pytest.fixture
def conversation_dataset():
    """创建对话格式数据集"""
    return [
        {
            "conversations": [
                {"from": "human", "value": "如何申请租房？"},
                {"from": "gpt", "value": "请登录官网申请"}
            ]
        },
        {
            "conversations": [
                {"from": "human", "value": "租房需要什么材料？"},
                {"from": "gpt", "value": "身份证、工作证明"}
            ]
        },
    ]


class TestDatasetMigrator:
    """DatasetMigrator 测试"""
    
    def test_init(self):
        """测试初始化"""
        migrator = DatasetMigrator()
        assert len(migrator._builtin_rules) > 0
    
    def test_migrate(self, sample_dataset):
        """测试迁移"""
        migrator = DatasetMigrator()
        result = migrator.migrate(sample_dataset)
        
        assert isinstance(result, MigrationResult)
        assert result.total_items == 2
        assert result.migrated_items == 2
        assert result.failed_items == 0
    
    def test_migrate_with_rules(self, sample_dataset):
        """测试使用规则迁移"""
        migrator = DatasetMigrator()
        result = migrator.migrate(sample_dataset, rules=["rename_instruction"])
        
        assert isinstance(result, MigrationResult)
        assert "rename_instruction" in result.rules_applied
    
    def test_migrate_file(self, sample_dataset, tmp_path):
        """测试迁移文件"""
        source_path = tmp_path / "source.json"
        target_path = tmp_path / "target.json"
        
        with open(source_path, 'w', encoding='utf-8') as f:
            json.dump(sample_dataset, f, ensure_ascii=False)
        
        migrator = DatasetMigrator()
        result = migrator.migrate_file(str(source_path), str(target_path))
        
        assert target_path.exists()
        assert result.migrated_items == 2
    
    def test_add_rule(self, sample_dataset):
        """测试添加规则"""
        migrator = DatasetMigrator()
        
        rule = MigrationRule(
            rule_id="custom_rule",
            name="自定义规则",
            description="自定义转换",
            source_field="instruction",
            target_field="question"
        )
        
        migrator.add_rule(rule)
        
        result = migrator.migrate(sample_dataset, rules=["custom_rule"])
        assert "custom_rule" in result.rules_applied


class TestMigrationRule:
    """MigrationRule 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        rule = MigrationRule(
            rule_id="test_rule",
            name="测试规则",
            description="测试描述",
            source_field="instruction",
            target_field="question",
            default_value=""
        )
        
        d = rule.to_dict()
        
        assert d["rule_id"] == "test_rule"
        assert d["source_field"] == "instruction"
        assert d["target_field"] == "question"


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_migrate_dataset(self, sample_dataset):
        """测试迁移数据集"""
        result = migrate_dataset(sample_dataset)
        
        assert isinstance(result, MigrationResult)
        assert result.migrated_items == 2
    
    def test_migrate_file(self, sample_dataset, tmp_path):
        """测试迁移文件"""
        source_path = tmp_path / "source.json"
        target_path = tmp_path / "target.json"
        
        with open(source_path, 'w', encoding='utf-8') as f:
            json.dump(sample_dataset, f, ensure_ascii=False)
        
        result = migrate_file(str(source_path), str(target_path))
        
        assert target_path.exists()
        assert result.migrated_items == 2


class TestMigrationResult:
    """MigrationResult 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        result = MigrationResult(
            migration_id="test_id",
            source_path="/source.json",
            target_path="/target.json",
            total_items=100,
            migrated_items=95,
            failed_items=5,
            rules_applied=["rule1", "rule2"],
            errors=[{"index": 0, "error": "test error"}],
            timestamp="2024-01-01T00:00:00"
        )
        
        d = result.to_dict()
        
        assert d["migration_id"] == "test_id"
        assert d["total_items"] == 100
        assert d["migrated_items"] == 95
        assert d["failed_items"] == 5
