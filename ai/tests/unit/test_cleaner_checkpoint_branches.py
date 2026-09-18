"""cleaner 规则与 checkpoint 增量分支补测

cleaner：remove_special_chars / remove_long_texts / remove_short_texts /
自定义规则(add_rule)；
checkpoint：update_progress 自动增量保存、损坏增量文件回退重建、
load_checkpoint 合并增量、无增量时的 delta 空读。
"""

from pathlib import Path

import pytest

from augmentor.checkpoint import CheckpointManager
from augmentor.cleaner import DatasetCleaner


class TestCleanerRules:
    def test_remove_special_chars(self):
        cleaner = DatasetCleaner()
        items, modified = cleaner._remove_special_chars(
            [{"instruction": "价格###@@是多少"}], ["instruction"]
        )
        assert "###" not in items[0]["instruction"]
        assert modified is True

    def test_remove_long_texts(self):
        cleaner = DatasetCleaner()
        items = [
            {"instruction": "短", "output": "答"},
            {"instruction": "字" * 1500, "output": "答"},
        ]
        cleaned, modified = cleaner._remove_long_texts(items, ["instruction"])
        assert len(cleaned) == 1
        assert cleaned[0]["instruction"] == "短"

    def test_remove_short_texts(self):
        cleaner = DatasetCleaner()
        items = [
            {"instruction": "很长的正常问题描述", "output": "答"},
            {"instruction": "x", "output": "答"},  # 长度 1 < 2
        ]
        cleaned, modified = cleaner._remove_short_texts(items, ["instruction"])
        assert len(cleaned) == 1
        assert cleaned[0]["instruction"] == "很长的正常问题描述"

    def test_custom_rule_applied(self):
        cleaner = DatasetCleaner()

        def upper_instructions(items):
            for item in items:
                if "instruction" in item:
                    item["instruction"] = item["instruction"].upper()
            return items

        cleaner.add_custom_rule(upper_instructions)
        result = cleaner._custom_rules[0]([{"instruction": "abc"}])
        assert result[0]["instruction"] == "ABC"

    def test_clean_applies_named_rules(self):
        cleaner = DatasetCleaner()
        items = [
            {"instruction": "   重复   ", "output": "甲"},
            {"instruction": "重复", "output": "乙"},
        ]
        cleaned, res = cleaner.clean(items, rules=["remove_duplicates", "normalize_whitespace"])
        assert "remove_duplicates" in res.rules_applied or "normalize_whitespace" in res.rules_applied
        assert res.original_count == 2


class TestCheckpointDelta:
    @pytest.fixture
    def manager(self, tmp_path):
        return CheckpointManager(checkpoint_dir=str(tmp_path))

    def test_update_progress_autosaves_delta(self, manager, tmp_path):
        manager.create_checkpoint("t1", total_items=100)
        for i in range(10):  # auto_save_interval=10
            manager.update_progress(i, success=True)
        delta_path = manager._get_delta_path("t1")
        assert delta_path.exists()

    def test_corrupt_delta_rebuilds_empty(self, manager, tmp_path):
        manager.create_checkpoint("t2", total_items=50)
        for i in range(10):
            manager.update_progress(i, success=True)

        delta_path = manager._get_delta_path("t2")
        delta_path.write_text("{ not valid json", encoding="utf-8")
        delta = manager._read_delta(delta_path)
        assert delta == {"completed": [], "failed": [], "quality_scores": {}}

    def test_read_delta_missing_returns_empty(self, manager, tmp_path):
        missing = manager._get_delta_path("nope")
        assert not missing.exists()
        assert manager._read_delta(missing) == {
            "completed": [], "failed": [], "quality_scores": {},
        }

    def test_load_merges_delta(self, manager, tmp_path):
        manager.create_checkpoint("t3", total_items=100)
        for i in range(10):
            manager.update_progress(i, success=True)
        # 新管理器实例恢复，需合并增量
        fresh = CheckpointManager(checkpoint_dir=str(tmp_path))
        loaded = fresh.load_checkpoint("t3")
        assert loaded is not None
        assert loaded.processed_items >= 10

    def test_load_missing_returns_none(self, manager, tmp_path):
        fresh = CheckpointManager(checkpoint_dir=str(tmp_path))
        assert fresh.load_checkpoint("does_not_exist") is None

    def test_save_checkpoint_removes_delta(self, manager, tmp_path):
        manager.create_checkpoint("t4", total_items=100)
        for i in range(10):
            manager.update_progress(i, success=True)
        manager.save_checkpoint()
        assert not manager._get_delta_path("t4").exists()
