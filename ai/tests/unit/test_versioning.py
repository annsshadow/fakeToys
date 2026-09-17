"""版本管理单元测试

版本快照、对比、回滚与历史记录是数据可追溯性的基础。
"""

import pytest

from augmentor.versioning import VersionManager, VersionInfo, DiffResult


class TestCreateVersion:
    """创建版本"""

    def test_creates_and_lists(self, tmp_path):
        """创建后必须能在列表中查到"""
        manager = VersionManager(storage_dir=str(tmp_path))
        info = manager.create_version(
            [{"instruction": "a"}, {"instruction": "b"}],
            label="测试版本",
            description="desc"
        )

        assert isinstance(info, VersionInfo)
        assert info.item_count == 2
        assert info.label == "测试版本"

        versions = manager.list_versions()
        assert len(versions) == 1
        assert versions[0].version_id == info.version_id

    def test_label_defaults_to_version_id(self, tmp_path):
        """未指定标签时使用版本 ID，避免空标签"""
        manager = VersionManager(storage_dir=str(tmp_path))
        info = manager.create_version([])

        assert info.label == info.version_id

    def test_multiple_versions_sorted_desc(self, tmp_path):
        """版本列表需按时间倒序，便于查看最新"""
        manager = VersionManager(storage_dir=str(tmp_path))
        first = manager.create_version([{"instruction": "a"}], label="first")
        second = manager.create_version([{"instruction": "b"}], label="second")

        versions = manager.list_versions()
        assert versions[0].version_id == second.version_id
        assert versions[1].version_id == first.version_id


class TestLoadVersion:
    """加载版本"""

    def test_load_round_trip(self, tmp_path):
        """保存的数据必须能原样加载"""
        manager = VersionManager(storage_dir=str(tmp_path))
        items = [{"instruction": "a", "output": "b"}]
        info = manager.create_version(items)

        assert manager.load_version(info.version_id) == items

    def test_missing_version_raises(self, tmp_path):
        """版本不存在必须报错"""
        manager = VersionManager(storage_dir=str(tmp_path))
        with pytest.raises(ValueError):
            manager.load_version("v_not_exists")

    def test_get_version_info_missing_raises(self, tmp_path):
        """版本信息不存在必须报错"""
        manager = VersionManager(storage_dir=str(tmp_path))
        with pytest.raises(ValueError):
            manager.get_version_info("v_not_exists")


class TestDiff:
    """版本对比"""

    def test_detects_added_and_removed(self, tmp_path):
        """对比需正确区分新增与移除项"""
        manager = VersionManager(storage_dir=str(tmp_path))
        v1 = manager.create_version([{"instruction": "a", "output": "1"}])
        v2 = manager.create_version([
            {"instruction": "a", "output": "1"},
            {"instruction": "b", "output": "2"},
        ])

        diff = manager.diff(v1.version_id, v2.version_id)

        assert isinstance(diff, DiffResult)
        assert diff.added_count == 1
        assert diff.removed_count == 0

    def test_identical_versions_have_no_diff(self, tmp_path):
        """内容相同的两个版本差异应为 0"""
        manager = VersionManager(storage_dir=str(tmp_path))
        items = [{"instruction": "a", "output": "1"}]
        v1 = manager.create_version(items)
        v2 = manager.create_version(items)

        diff = manager.diff(v1.version_id, v2.version_id)
        assert (diff.added_count, diff.removed_count) == (0, 0)


class TestRollback:
    """回滚"""

    def test_rollback_switches_current(self, tmp_path):
        """回滚后当前版本应指向目标版本"""
        manager = VersionManager(storage_dir=str(tmp_path))
        first = manager.create_version([{"instruction": "a"}])
        manager.create_version([{"instruction": "b"}])

        assert manager.rollback(first.version_id) is True
        assert manager.get_current_version() == first.version_id

    def test_rollback_missing_version_returns_false(self, tmp_path):
        """回滚不存在的版本应返回 False 而不是抛异常"""
        manager = VersionManager(storage_dir=str(tmp_path))
        assert manager.rollback("v_missing") is False

    def test_current_version_is_latest_after_create(self, tmp_path):
        """创建新版本后当前版本应更新为最新"""
        manager = VersionManager(storage_dir=str(tmp_path))
        manager.create_version([{"instruction": "a"}])
        latest = manager.create_version([{"instruction": "b"}])

        assert manager.get_current_version() == latest.version_id


class TestDelete:
    """删除版本"""

    def test_delete_non_current(self, tmp_path):
        """非当前版本可被删除"""
        manager = VersionManager(storage_dir=str(tmp_path))
        first = manager.create_version([{"instruction": "a"}])
        manager.create_version([{"instruction": "b"}])

        manager.delete_version(first.version_id)
        assert all(v.version_id != first.version_id for v in manager.list_versions())

    def test_cannot_delete_current(self, tmp_path):
        """当前版本不允许删除，避免数据无回滚点"""
        manager = VersionManager(storage_dir=str(tmp_path))
        current = manager.create_version([{"instruction": "a"}])

        with pytest.raises(ValueError):
            manager.delete_version(current.version_id)


class TestHistory:
    """历史记录"""

    def test_records_create_and_rollback(self, tmp_path):
        """创建与回滚都应被记录，形成可审计轨迹"""
        manager = VersionManager(storage_dir=str(tmp_path))
        first = manager.create_version([{"instruction": "a"}])
        manager.create_version([{"instruction": "b"}])
        manager.rollback(first.version_id)

        history = manager.get_history()

        assert len(history) == 3
        actions = [entry["action"] for entry in history]
        # 最新记录在前
        assert actions[0] == "rollback"
        assert "create" in actions

    def test_records_delete(self, tmp_path):
        """删除操作也应被记录"""
        manager = VersionManager(storage_dir=str(tmp_path))
        first = manager.create_version([{"instruction": "a"}])
        manager.create_version([{"instruction": "b"}])
        manager.delete_version(first.version_id)

        assert any(e["action"] == "delete" for e in manager.get_history())

    def test_history_limit(self, tmp_path):
        """limit 需生效，避免历史无限增长返回"""
        manager = VersionManager(storage_dir=str(tmp_path))
        for index in range(5):
            manager.create_version([{"instruction": str(index)}])

        assert len(manager.get_history(limit=2)) == 2

    def test_empty_history(self, tmp_path):
        """无历史时返回空列表"""
        manager = VersionManager(storage_dir=str(tmp_path))
        assert manager.get_history() == []

    def test_history_survives_reload(self, tmp_path):
        """历史需持久化，重启后仍可追溯"""
        manager = VersionManager(storage_dir=str(tmp_path))
        manager.create_version([{"instruction": "a"}])

        reloaded = VersionManager(storage_dir=str(tmp_path))
        assert len(reloaded.get_history()) == 1


class TestReport:
    """版本报告"""

    def test_report_fields(self, tmp_path):
        """报告需包含版本总数、当前版本与最近历史"""
        manager = VersionManager(storage_dir=str(tmp_path))
        manager.create_version([{"instruction": "a"}])
        report = manager.generate_report()

        assert report["total_versions"] == 1
        assert report["current_version"] is not None
        assert len(report["recent_history"]) == 1


class TestVersioningExtended:
    """版本管理扩展测试"""

    def test_create_version_empty_data(self, tmp_path):
        """创建空数据版本"""
        manager = VersionManager(storage_dir=str(tmp_path))
        info = manager.create_version([])
        assert info.item_count == 0

    def test_create_version_large_dataset(self, tmp_path):
        """创建大数据集版本"""
        manager = VersionManager(storage_dir=str(tmp_path))
        items = [{"instruction": f"q{i}"} for i in range(100)]
        info = manager.create_version(items)
        assert info.item_count == 100

    def test_load_version_data_integrity(self, tmp_path):
        """加载版本数据完整性"""
        manager = VersionManager(storage_dir=str(tmp_path))
        items = [
            {"instruction": "问题1", "output": "回答1", "input": "上下文"},
            {"instruction": "问题2", "output": "回答2"},
        ]
        info = manager.create_version(items)
        loaded = manager.load_version(info.version_id)
        assert loaded == items

    def test_diff_with_removed_items(self, tmp_path):
        """对比有移除项的情况"""
        manager = VersionManager(storage_dir=str(tmp_path))
        v1 = manager.create_version([
            {"instruction": "a"},
            {"instruction": "b"},
            {"instruction": "c"},
        ])
        v2 = manager.create_version([{"instruction": "a"}])
        diff = manager.diff(v1.version_id, v2.version_id)
        assert diff.removed_count == 2

    def test_rollback_to_self(self, tmp_path):
        """回滚到当前版本应返回 True"""
        manager = VersionManager(storage_dir=str(tmp_path))
        v1 = manager.create_version([{"instruction": "a"}])
        assert manager.rollback(v1.version_id) is True

    def test_delete_last_version(self, tmp_path):
        """删除最后一个版本（非当前）"""
        manager = VersionManager(storage_dir=str(tmp_path))
        v1 = manager.create_version([{"instruction": "a"}])
        v2 = manager.create_version([{"instruction": "b"}])
        # v2 is current, delete v1 (non-current)
        manager.delete_version(v1.version_id)
        assert len(manager.list_versions()) == 1

    def test_history_with_corrupted_line(self, tmp_path):
        """历史文件包含损坏行时应跳过"""
        manager = VersionManager(storage_dir=str(tmp_path))
        manager.create_version([{"instruction": "a"}])
        # Append corrupted line
        with open(manager._history_path, 'a', encoding='utf-8') as f:
            f.write("not valid json\n")
        history = manager.get_history()
        assert len(history) >= 1

    def test_get_version_info(self, tmp_path):
        """获取版本信息"""
        manager = VersionManager(storage_dir=str(tmp_path))
        info = manager.create_version([{"instruction": "a"}], label="test")
        version_info = manager.get_version_info(info.version_id)
        assert version_info.label == "test"

    def test_report_empty(self, tmp_path):
        """空版本管理器报告"""
        manager = VersionManager(storage_dir=str(tmp_path))
        report = manager.generate_report()
        assert report["total_versions"] == 0
        assert report["current_version"] is None
        assert report["recent_history"] == []
        assert len(report["versions"]) == 0
