# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集迁移模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.migration import (
    DatasetMigrator, MigrationRule, MigrationResult,
    BUILTIN_MIGRATION_RULE_IDS,
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


class TestMigrationExtended:
    """DatasetMigrator 扩展测试"""

    def test_migrate_empty_dataset(self):
        """迁移空数据集"""
        migrator = DatasetMigrator()
        result = migrator.migrate([])
        assert result.migrated_items == 0

    def test_migrate_with_transform_func(self):
        """使用转换函数迁移"""
        migrator = DatasetMigrator()
        rule = MigrationRule(
            rule_id="custom",
            name="custom",
            description="custom",
            source_field="instruction",
            target_field="question",
            transform_func=lambda x: x.upper()
        )
        migrator.add_rule(rule)
        data = [{"instruction": "hello"}]
        result = migrator.migrate(data, rules=["custom"])
        assert result.migrated_items == 1

    def test_migrate_with_default_value(self):
        """使用默认值迁移"""
        migrator = DatasetMigrator()
        rule = MigrationRule(
            rule_id="default_rule",
            name="default",
            description="default",
            source_field="missing_field",
            target_field="new_field",
            default_value="default_value"
        )
        migrator.add_rule(rule)
        data = [{"instruction": "q1"}]
        result = migrator.migrate(data, rules=["default_rule"])
        assert result.migrated_items == 1

    def test_migrate_save_to_file(self, tmp_path):
        """迁移保存到文件"""
        migrator = DatasetMigrator()
        data = [{"instruction": "q1"}]
        output_path = str(tmp_path / "output.json")
        result = migrator.migrate(data, output_path=output_path)
        assert result.migrated_items == 1

    def test_flatten_conversation_list(self):
        """展平对话列表"""
        migrator = DatasetMigrator()
        conversations = [
            {"from": "human", "value": "问题"},
            {"from": "gpt", "value": "回答"}
        ]
        result = migrator._flatten_conversations(conversations)
        assert "human: 问题" in result
        assert "gpt: 回答" in result

    def test_flatten_conversation_string(self):
        """展平字符串对话"""
        migrator = DatasetMigrator()
        result = migrator._flatten_conversations("简单文本")
        assert result == "简单文本"

    def test_migrate_error_handling(self):
        """迁移错误处理"""
        migrator = DatasetMigrator()
        rule = MigrationRule(
            rule_id="error_rule",
            name="error",
            description="error",
            source_field="instruction",
            target_field="question",
            transform_func=lambda x: 1/0
        )
        migrator.add_rule(rule)
        data = [{"instruction": "q1"}]
        result = migrator.migrate(data, rules=["error_rule"])
        assert result.failed_items == 1


class TestMigrationRulesAndFiles:
    """DatasetMigrator 扩展测试（覆盖剩余分支）"""

    def test_builtin_rules_exist(self):
        """内置规则应包含重命名与展平"""
        migrator = DatasetMigrator()
        rule_ids = [r.rule_id for r in migrator._builtin_rules]
        assert "rename_instruction" in rule_ids
        assert "rename_output" in rule_ids
        assert "flatten_conversations" in rule_ids

    def test_rename_instruction_applies(self, sample_dataset):
        """rename_instruction 应将 instruction 重命名为 question"""
        migrator = DatasetMigrator()
        migrated = migrator._apply_rules(
            sample_dataset[0],
            [r for r in migrator._builtin_rules if r.rule_id == "rename_instruction"]
        )
        assert "question" in migrated
        assert "instruction" not in migrated

    def test_rename_output_applies(self, sample_dataset):
        """rename_output 应将 output 重命名为 answer"""
        migrator = DatasetMigrator()
        migrated = migrator._apply_rules(
            sample_dataset[0],
            [r for r in migrator._builtin_rules if r.rule_id == "rename_output"]
        )
        assert "answer" in migrated
        assert "output" not in migrated

    def test_migrate_empty_dataset(self):
        """空数据集迁移应为 0"""
        migrator = DatasetMigrator()
        result = migrator.migrate([])
        assert result.total_items == 0
        assert result.migrated_items == 0

    def test_migrate_unknown_rule_rejected(self, sample_dataset):
        """L193 红字重述：未知规则 id 不再被静默跳过，`migrate()` 入口拒并列出
        全部合法规则 id（改前本例钉的是「跳过不崩溃」）。"""
        from augmentor.exceptions import DataValidationError

        migrator = DatasetMigrator()
        with pytest.raises(DataValidationError) as ei:
            migrator.migrate(sample_dataset, rules=["nonexistent_rule"])
        msg = str(ei.value)
        for rid in BUILTIN_MIGRATION_RULE_IDS:
            assert rid in msg, f"报错文案必须列出全部合法规则 id，缺 {rid!r}：{msg}"
        assert "nonexistent_rule" in msg

    def test_migrate_with_output_path_writes_file(self, sample_dataset, tmp_path):
        """带输出路径的迁移应写文件"""
        migrator = DatasetMigrator()
        output = str(tmp_path / "out.json")
        result = migrator.migrate(sample_dataset, output_path=output)
        assert Path(output).exists()
        assert result.migrated_items == 2

    def test_migrate_file_missing_source_raises(self, tmp_path):
        """源文件不存在应报错"""
        migrator = DatasetMigrator()
        with pytest.raises(FileNotFoundError):
            migrator.migrate_file(str(tmp_path / "nope.json"), str(tmp_path / "out.json"))

    def test_migrate_file_creates_parent_dirs(self, sample_dataset, tmp_path):
        """迁移文件应自动创建父目录"""
        migrator = DatasetMigrator()
        source = tmp_path / "src.json"
        source.write_text(json.dumps(sample_dataset), encoding="utf-8")
        target = tmp_path / "nested" / "deep" / "out.json"
        migrator.migrate_file(str(source), str(target))
        assert target.exists()

    def test_flatten_conversations_mixed_types(self):
        """混合类型对话展平"""
        migrator = DatasetMigrator()
        result = migrator._flatten_conversations("文本")
        assert result == "文本"

    def test_migration_result_to_dict_limits_errors(self):
        """MigrationResult.to_dict 应限制错误数量"""
        result = MigrationResult(
            migration_id="m1", source_path="s", target_path="t",
            total_items=10, migrated_items=5, failed_items=5,
            rules_applied=["r1"],
            errors=[{"i": i} for i in range(20)]
        )
        d = result.to_dict()
        assert len(d["errors"]) == 10

    def test_convenience_migrate_dataset(self, sample_dataset):
        """便捷函数 migrate_dataset 应返回 MigrationResult"""
        result = migrate_dataset(sample_dataset)
        assert isinstance(result, MigrationResult)
        assert result.total_items == 2

    def test_convenience_migrate_file(self, sample_dataset, tmp_path):
        """便捷函数 migrate_file 应完成迁移"""
        src = tmp_path / "s.json"
        src.write_text(json.dumps(sample_dataset), encoding="utf-8")
        target = tmp_path / "t.json"
        result = migrate_file(str(src), str(target))
        assert target.exists()
        assert result.migrated_items == 2


class TestMigrationRuleClosedListL193:
    """L193（B263）：迁移规则 id 封闭清单下沉 SDK 直构面。

    改前 `migrate(rules=[...])` 用 `r.rule_id in rules` 过滤，未知 id 静默丢弃
    （全拼错时迁移照跑、rules_applied 空数组，「迁移跑了个寂寞」；L175/L176/L189/
    L192 封闭清单族同式）。清单权威住 `BUILTIN_MIGRATION_RULE_IDS`（A77），
    API 面共引同一份；自定义规则 id 仍可经 `add_rule` 注册后被接受。
    """

    def test_builtin_ids_constant_tracks_built_in_rules(self):
        """共引钉：常量必须与 `_create_builtin_rules` 的 rule_id 集合逐一对上。"""
        assert set(BUILTIN_MIGRATION_RULE_IDS) == {
            r.rule_id for r in DatasetMigrator()._builtin_rules
        }
        assert len(BUILTIN_MIGRATION_RULE_IDS) == len(set(BUILTIN_MIGRATION_RULE_IDS))

    def test_unknown_rule_id_rejected_with_full_valid_list(self):
        from augmentor.exceptions import DataValidationError

        items = [{"instruction": "q", "output": "a"}]
        with pytest.raises(DataValidationError) as ei:
            DatasetMigrator().migrate(items, rules=["rename_instructon"])
        msg = str(ei.value)
        for rid in BUILTIN_MIGRATION_RULE_IDS:
            assert rid in msg
        assert "rename_instructon" in msg

    def test_bad_shape_rule_ids_rejected(self):
        from augmentor.exceptions import DataValidationError

        items = [{"instruction": "q", "output": "a"}]
        for bad in (None, 5, ["rename_output"]):
            with pytest.raises(DataValidationError):
                DatasetMigrator().migrate(items, rules=["rename_output", bad])

    def test_custom_rule_id_still_accepted(self):
        """自定义规则经 add_rule 注册后是合法规则 id，拒判据没有修过头。"""
        migrator = DatasetMigrator()
        migrator.add_rule(MigrationRule(
            rule_id="l193_custom",
            name="custom",
            description="custom",
            source_field="instruction",
            target_field="question",
        ))
        result = migrator.migrate([{"instruction": "q"}], rules=["l193_custom"])
        assert "l193_custom" in result.rules_applied

    def test_api_face_shares_sdk_builtin_ids(self):
        """A77 共引钉：API 路由的 MIGRATION_RULES 是 SDK 常量同一份（is 钉）。"""
        import api.routes.system_ops as ops

        assert ops.MIGRATION_RULES is BUILTIN_MIGRATION_RULE_IDS
