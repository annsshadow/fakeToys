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
