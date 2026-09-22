# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集备份模块测试"""

import json
import pytest
from pathlib import Path
from augmentor.backup import (
    DatasetBackup, BackupInfo,
    create_backup, restore_backup, list_backups, delete_backup
)


@pytest.fixture
def sample_dataset():
    """创建测试数据集"""
    return [
        {"instruction": "如何申请租房？", "input": "", "output": "请登录官网申请"},
        {"instruction": "租房需要什么材料？", "input": "", "output": "身份证、工作证明"},
        {"instruction": "租房流程是什么？", "input": "", "output": "选房、签约、付款"},
    ]


@pytest.fixture
def sample_file(tmp_path, sample_dataset):
    """创建测试文件"""
    file_path = tmp_path / "dataset.json"
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(sample_dataset, f, ensure_ascii=False)
    return str(file_path)


class TestDatasetBackup:
    """DatasetBackup 测试"""
    
    def test_init(self, tmp_path):
        """测试初始化"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        assert backup_dir.exists()
        assert manager._index_file.exists()
    
    def test_backup(self, tmp_path, sample_file):
        """测试创建备份"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        info = manager.backup(sample_file, "test_backup")
        
        assert isinstance(info, BackupInfo)
        assert info.backup_id == "test_backup"
        assert info.item_count == 3
        assert info.file_size > 0
        assert info.checksum
    
    def test_restore(self, tmp_path, sample_file):
        """测试恢复备份"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        info = manager.backup(sample_file, "test_backup")
        
        output_path = tmp_path / "restored.json"
        result = manager.restore("test_backup", str(output_path))
        
        assert output_path.exists()
        assert result["item_count"] == 3
        
        with open(output_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        assert len(data) == 3
    
    def test_list_backups(self, tmp_path, sample_file):
        """测试列出备份"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        manager.backup(sample_file, "backup1")
        manager.backup(sample_file, "backup2")
        
        backups = manager.list_backups()
        
        assert len(backups) == 2
    
    def test_delete_backup(self, tmp_path, sample_file):
        """测试删除备份"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        manager.backup(sample_file, "test_backup")
        
        assert manager.delete_backup("test_backup") is True
        assert manager.get_backup_info("test_backup") is None
    
    def test_get_backup_info(self, tmp_path, sample_file):
        """测试获取备份信息"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        manager.backup(sample_file, "test_backup")
        
        info = manager.get_backup_info("test_backup")
        
        assert info is not None
        assert info["backup_id"] == "test_backup"


class TestConvenienceFunctions:
    """便捷函数测试"""
    
    def test_create_backup(self, tmp_path, sample_file):
        """测试创建备份"""
        backup_dir = tmp_path / "backups"
        
        info = create_backup(sample_file, str(backup_dir), "test_backup")
        
        assert isinstance(info, BackupInfo)
        assert info.backup_id == "test_backup"
    
    def test_restore_backup(self, tmp_path, sample_file):
        """测试恢复备份"""
        backup_dir = tmp_path / "backups"
        
        create_backup(sample_file, str(backup_dir), "test_backup")
        
        output_path = tmp_path / "restored.json"
        result = restore_backup("test_backup", str(output_path), str(backup_dir))
        
        assert output_path.exists()
        assert result["item_count"] == 3
    
    def test_list_backups(self, tmp_path, sample_file):
        """测试列出备份"""
        backup_dir = tmp_path / "backups"
        
        create_backup(sample_file, str(backup_dir), "backup1")
        create_backup(sample_file, str(backup_dir), "backup2")
        
        backups = list_backups(str(backup_dir))
        
        assert len(backups) == 2
    
    def test_delete_backup(self, tmp_path, sample_file):
        """测试删除备份"""
        backup_dir = tmp_path / "backups"
        
        create_backup(sample_file, str(backup_dir), "test_backup")
        
        assert delete_backup("test_backup", str(backup_dir)) is True


class TestBackupInfo:
    """BackupInfo 测试"""
    
    def test_to_dict(self):
        """测试转换为字典"""
        info = BackupInfo(
            backup_id="test",
            timestamp="2024-01-01T00:00:00",
            source_path="/source.json",
            backup_path="/backup.json",
            item_count=100,
            file_size=1024,
            checksum="abc123"
        )
        
        d = info.to_dict()
        
        assert d["backup_id"] == "test"
        assert d["item_count"] == 100
        assert d["file_size"] == 1024


class TestBackupEdgeCases:
    """备份模块边界测试"""

    def test_calculate_checksum(self, tmp_path, sample_file):
        """校验和应一致"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        checksum1 = manager._calculate_checksum(sample_file)
        checksum2 = manager._calculate_checksum(sample_file)
        
        assert checksum1 == checksum2
        assert len(checksum1) == 64  # SHA256 hex length

    def test_backup_nonexistent_file(self, tmp_path):
        """备份不存在的文件应抛出异常"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        with pytest.raises(FileNotFoundError):
            manager.backup(str(tmp_path / "nonexistent.json"))

    def test_restore_nonexistent_backup(self, tmp_path):
        """恢复不存在的备份应抛出异常"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        with pytest.raises(ValueError):
            manager.restore("nonexistent", str(tmp_path / "output.json"))

    def test_restore_checksum_mismatch(self, tmp_path, sample_file):
        """校验和不匹配时应发出警告但仍恢复"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        info = manager.backup(sample_file, "test_backup")
        
        # 篡改校验和记录
        for b in manager._index["backups"]:
            if b["backup_id"] == "test_backup":
                b["checksum"] = "tampered"
        
        output_path = tmp_path / "restored.json"
        result = manager.restore("test_backup", str(output_path))
        
        assert output_path.exists()

    def test_delete_nonexistent_backup(self, tmp_path):
        """删除不存在的备份应返回 False"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        assert manager.delete_backup("nonexistent") is False

    def test_get_backup_info_nonexistent(self, tmp_path):
        """获取不存在的备份信息应返回 None"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        assert manager.get_backup_info("nonexistent") is None

    def test_backup_with_custom_name(self, tmp_path, sample_file):
        """使用自定义名称备份"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        info = manager.backup(sample_file, "custom_name")
        assert info.backup_id == "custom_name"

    def test_backup_auto_name(self, tmp_path, sample_file):
        """自动生成备份名称"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        info = manager.backup(sample_file)
        assert info.backup_id.startswith("backup_")

    def test_restore_creates_output_dir(self, tmp_path, sample_file):
        """恢复时应自动创建输出目录"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        manager.backup(sample_file, "test_backup")
        
        output_path = tmp_path / "subdir" / "nested" / "restored.json"
        result = manager.restore("test_backup", str(output_path))
        
        assert output_path.exists()

    def test_list_backups_empty(self, tmp_path):
        """空备份目录应返回空列表"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        assert manager.list_backups() == []

    def test_multiple_backups(self, tmp_path, sample_file):
        """多个备份应正确管理"""
        backup_dir = tmp_path / "backups"
        manager = DatasetBackup(str(backup_dir))
        
        manager.backup(sample_file, "backup1")
        manager.backup(sample_file, "backup2")
        manager.backup(sample_file, "backup3")
        
        assert len(manager.list_backups()) == 3
        
        manager.delete_backup("backup2")
        assert len(manager.list_backups()) == 2
        assert manager.get_backup_info("backup2") is None

    def test_backup_custom_subdir(self, tmp_path, sample_file):
        """备份目录支持嵌套子目录"""
        custom = tmp_path / "nested" / "backups"
        manager = DatasetBackup(str(custom))
        info = manager.backup(sample_file, "x")
        assert custom.exists()
        assert info.backup_id == "x"

    def test_backup_index_persists_across_instances(self, tmp_path, sample_file):
        """备份索引应跨实例持久化"""
        manager1 = DatasetBackup(str(tmp_path / "backups"))
        manager1.backup(sample_file, "persisted")

        manager2 = DatasetBackup(str(tmp_path / "backups"))
        ids = [b["backup_id"] for b in manager2.list_backups()]
        assert "persisted" in ids

    def test_restore_missing_backup_raises(self, tmp_path, sample_file):
        """恢复不存在的备份应报错"""
        manager = DatasetBackup(str(tmp_path / "backups"))
        with pytest.raises(ValueError, match="备份不存在"):
            manager.restore("no_such_backup", str(tmp_path / "out.json"))

    def test_delete_missing_returns_false(self, tmp_path, sample_file):
        """删除不存在的备份应返回 False"""
        manager = DatasetBackup(str(tmp_path / "backups"))
        assert manager.delete_backup("ghost") is False
