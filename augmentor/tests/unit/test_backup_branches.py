# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""backup 模块剩余分支测试

覆盖 create/list/restore/delete 全生命周期、缺文件恢复报错、
clean_old_backups 保留/清理、verify_backup_integrity 多备份回归
（原实现 return 误缩进在循环内，多备份时只校验首个）。
"""

import json

import pytest

from augmentor.backup import (
    DatasetBackup,
    clean_old_backups,
    create_backup,
    delete_backup,
    list_backups,
    restore_backup,
    verify_backup_integrity,
)


@pytest.fixture
def source_file(tmp_path):
    path = tmp_path / "data.json"
    items = [{"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"} for i in range(3)]
    path.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return path


class TestBackupLifecycle:
    def test_create_list_restore_delete(self, source_file, tmp_path):
        bdir = str(tmp_path / "bk")
        info = create_backup(str(source_file), bdir, name="v1")
        assert info.item_count == 3
        assert info.backup_id == "v1"

        listed = list_backups(bdir)
        assert any(b["backup_id"] == "v1" for b in listed)

        out = tmp_path / "restored.json"
        result = restore_backup("v1", str(out), bdir)
        assert result["item_count"] == 3
        assert json.loads(out.read_text(encoding="utf-8"))

        assert delete_backup("v1", bdir) is True
        assert delete_backup("ghost", bdir) is False

    def test_restore_missing_backup_raises(self, source_file, tmp_path):
        bdir = str(tmp_path / "bk2")
        info = create_backup(str(source_file), bdir, name="v2")
        # 删除备份文件本体（索引仍在），恢复需抛 FileNotFoundError
        import pathlib

        pathlib.Path(info.backup_path).unlink()
        with pytest.raises(FileNotFoundError):
            restore_backup("v2", str(tmp_path / "o.json"), bdir)

    def test_restore_unknown_id_raises_value(self, source_file, tmp_path):
        bdir = str(tmp_path / "bk3")
        create_backup(str(source_file), bdir, name="ok")
        with pytest.raises(ValueError, match="备份不存在"):
            restore_backup("nope", str(tmp_path / "o.json"), bdir)

    def test_backup_missing_source_raises(self, tmp_path):
        with pytest.raises(FileNotFoundError, match="源文件不存在"):
            DatasetBackup(str(tmp_path / "b")).backup(str(tmp_path / "nope.json"))

    def test_get_backup_info(self, source_file, tmp_path):
        manager = DatasetBackup(str(tmp_path / "bk4"))
        info = manager.backup(str(source_file), name="m1")
        found = manager.get_backup_info("m1")
        assert found["backup_id"] == "m1"
        assert manager.get_backup_info("missing") is None

    def test_backup_info_to_dict(self, source_file, tmp_path):
        manager = DatasetBackup(str(tmp_path / "bk5"))
        info = manager.backup(str(source_file), name="m2")
        d = info.to_dict()
        assert d["backup_id"] == "m2"
        assert "checksum" in d


class TestCleanOldBackups:
    def test_within_limit_returns_zero(self, source_file, tmp_path):
        bdir = str(tmp_path / "bk")
        for i in range(3):
            create_backup(str(source_file), bdir, name=f"c{i}")
        assert clean_old_backups(bdir, max_backups=10) == 0
        assert len(list_backups(bdir)) == 3

    def test_exceeding_limit_removes_oldest(self, source_file, tmp_path):
        bdir = str(tmp_path / "bk")
        for i in range(5):
            create_backup(str(source_file), bdir, name=f"d{i}")
        deleted = clean_old_backups(bdir, max_backups=2)
        assert deleted == 3
        remaining = {b["backup_id"] for b in list_backups(bdir)}
        assert len(remaining) == 2


class TestVerifyBackupIntegrity:
    def test_all_backups_verified(self, source_file, tmp_path):
        """多备份时 verify 需返回全部（回归：原实现误在循环内 return）"""
        bdir = str(tmp_path / "bk")
        for i in range(3):
            create_backup(str(source_file), bdir, name=f"v{i}")
        results = verify_backup_integrity(bdir)
        assert len(results) == 3
        assert all(v["exists"] for v in results.values())
        assert all(v["checksum_match"] for v in results.values())

    def test_filter_by_backup_id(self, source_file, tmp_path):
        bdir = str(tmp_path / "bk")
        for i in range(3):
            create_backup(str(source_file), bdir, name=f"v{i}")
        results = verify_backup_integrity(bdir, backup_id="v1")
        assert list(results) == ["v1"]

    def test_corrupted_file_fails_checksum(self, source_file, tmp_path):
        bdir = str(tmp_path / "bk")
        info = create_backup(str(source_file), bdir, name="v1")
        import pathlib

        pathlib.Path(info.backup_path).write_text("garbage", encoding="utf-8")
        results = verify_backup_integrity(bdir, backup_id="v1")
        assert results["v1"]["checksum_match"] is False
