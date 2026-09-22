"""migration 模块剩余分支测试

覆盖 flatten_conversations 列表分支、自定义规则 transform/默认值、
迁移出错降级（errors 记录）、migrate_file 往返与 add_rule。
"""

import json

from augmentor.migration import (
    DatasetMigrator,
    MigrationRule,
    migrate_dataset,
    migrate_file,
)


class TestFlattenConversations:
    def test_list_of_dicts_flattened(self):
        manager = DatasetMigrator()
        items = [
            {
                "conversations": [
                    {"from": "human", "value": "你好"},
                    {"from": "gpt", "value": "你好！"},
                ]
            }
        ]
        result = manager.migrate(items, ["flatten_conversations"])
        assert result.migrated_items == 1
        # migrate 的转换体现在返回 result；直接验证 _apply_rules 的输出
        transformed = manager._apply_rules(
            items[0],
            [r for r in manager._builtin_rules if r.rule_id == "flatten_conversations"],
        )
        assert "human: 你好" in transformed["instruction"]
        assert "gpt: 你好！" in transformed["instruction"]

    def test_non_list_flattened_to_str(self):
        manager = DatasetMigrator()
        result = manager._flatten_conversations("纯文本")
        assert result == "纯文本"

    def test_missing_role_uses_fallback_keys(self):
        manager = DatasetMigrator()
        result = manager._flatten_conversations(
            [{"role": "user", "content": "问题"}, {"role": "assistant", "content": "回答"}]
        )
        assert "user: 问题" in result
        assert "assistant: 回答" in result


class TestCustomRules:
    def test_transform_func_applied(self):
        manager = DatasetMigrator()
        rule = MigrationRule(
            rule_id="shout",
            name="大写",
            description="转大写",
            source_field="instruction",
            target_field="instruction",
            transform_func=lambda v: v.upper(),
        )
        manager.add_rule(rule)
        transformed = manager._apply_rules({"instruction": "hello"}, [rule])
        assert transformed["instruction"] == "HELLO"

    def test_default_value_when_source_missing(self):
        manager = DatasetMigrator()
        rule = MigrationRule(
            rule_id="fill_output",
            name="补输出",
            description="",
            source_field="output",
            target_field="output",
            default_value="默认回答",
        )
        transformed = manager._apply_rules({"instruction": "q"}, [rule])
        assert transformed["output"] == "默认回答"

    def test_source_present_overrides_default(self):
        manager = DatasetMigrator()
        rule = MigrationRule(
            rule_id="fill_output",
            name="",
            description="",
            source_field="output",
            target_field="output",
            default_value="默认",
        )
        transformed = manager._apply_rules({"output": "已有"}, [rule])
        assert transformed["output"] == "已有"


class TestMigrateErrorRecovery:
    def test_transform_failure_recorded_not_raised(self):
        manager = DatasetMigrator()

        def bad_transform(value):
            raise ValueError("转换失败")

        rule = MigrationRule(
            rule_id="boom", name="", description="",
            source_field="instruction", target_field="instruction",
            transform_func=bad_transform,
        )
        manager.add_rule(rule)
        result = manager.migrate([{"instruction": "a"}, {"instruction": "b"}], [rule.rule_id])
        assert result.failed_items == 2
        assert len(result.errors) == 2
        assert result.errors[0]["index"] == 0
        assert "转换失败" in result.errors[0]["error"]


class TestMigrateFileAndDataset:
    def test_migrate_dataset_builtins_rename(self, tmp_path):
        items = [
            {"instruction": "q", "input": "", "output": "a", "conversations": []}
        ]
        result = migrate_dataset(items)
        # 内置 rename 规则：instruction -> question, output -> answer
        assert "rename_instruction" in result.rules_applied
        assert "rename_output" in result.rules_applied
        assert result.migrated_items == 1

    def test_migrate_file_roundtrip(self, tmp_path):
        src = tmp_path / "src.json"
        src.write_text(json.dumps([{"instruction": "q", "output": "a"}]), encoding="utf-8")
        out = tmp_path / "out.json"
        result = migrate_file(str(src), str(out))
        assert out.exists()
        assert result.source_path == str(src)
        assert result.target_path == str(out)
        reloaded = json.loads(out.read_text(encoding="utf-8"))
        assert len(reloaded) == 1

    def test_migrate_empty_items(self):
        result = migrate_dataset([])
        assert result.total_items == 0
        assert result.migrated_items == 0
        assert result.failed_items == 0

    def test_rule_to_dict(self):
        rule = MigrationRule(
            rule_id="r", name="n", description="d",
            source_field="s", target_field="t", default_value="dv",
        )
        d = rule.to_dict()
        assert d["rule_id"] == "r"
        assert d["default_value"] == "dv"
