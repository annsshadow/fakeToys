"""数据集备份模块

提供数据集的备份和恢复功能。
"""

import json
import shutil
import hashlib
import logging
from typing import List, Dict, Optional, Any
from pathlib import Path
from dataclasses import dataclass
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class BackupInfo:
    """备份信息"""
    backup_id: str
    timestamp: str
    source_path: str
    backup_path: str
    item_count: int
    file_size: int
    checksum: str
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "backup_id": self.backup_id,
            "timestamp": self.timestamp,
            "source_path": self.source_path,
            "backup_path": self.backup_path,
            "item_count": self.item_count,
            "file_size": self.file_size,
            "checksum": self.checksum
        }


class DatasetBackup:
    """数据集备份管理器
    
    提供数据集的备份和恢复功能。
    """
    
    def __init__(self, backup_dir: str = ".backups"):
        """初始化备份管理器
        
        Args:
            backup_dir: 备份目录
        """
        self._backup_dir = Path(backup_dir)
        self._backup_dir.mkdir(parents=True, exist_ok=True)
        self._index_file = self._backup_dir / "index.json"
        self._index = self._load_index()
    
    def _load_index(self) -> Dict:
        """加载索引文件"""
        if self._index_file.exists():
            with open(self._index_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        # 创建默认索引
        default_index = {"backups": []}
        with open(self._index_file, 'w', encoding='utf-8') as f:
            json.dump(default_index, f, ensure_ascii=False, indent=2)
        return default_index
    
    def _save_index(self):
        """保存索引文件"""
        with open(self._index_file, 'w', encoding='utf-8') as f:
            json.dump(self._index, f, ensure_ascii=False, indent=2)
    
    def _calculate_checksum(self, file_path: str) -> str:
        """计算文件校验和
        
        Args:
            file_path: 文件路径
        
        Returns:
            校验和
        """
        sha256_hash = hashlib.sha256()
        with open(file_path, "rb") as f:
            for byte_block in iter(lambda: f.read(4096), b""):
                sha256_hash.update(byte_block)
        return sha256_hash.hexdigest()
    
    def backup(self, source_path: str, name: str = None) -> BackupInfo:
        """创建备份
        
        Args:
            source_path: 源文件路径
            name: 备份名称
        
        Returns:
            备份信息
        """
        source = Path(source_path)
        if not source.exists():
            raise FileNotFoundError(f"源文件不存在: {source_path}")
        
        # 生成备份ID和路径
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        backup_id = name or f"backup_{timestamp}"
        backup_path = self._backup_dir / f"{backup_id}.json"
        
        # 读取源数据
        with open(source, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        # 保存备份
        with open(backup_path, 'w', encoding='utf-8') as f:
            json.dump(items, f, ensure_ascii=False, indent=2)
        
        # 计算文件大小和校验和
        file_size = backup_path.stat().st_size
        checksum = self._calculate_checksum(str(backup_path))
        
        # 创建备份信息
        info = BackupInfo(
            backup_id=backup_id,
            timestamp=datetime.now().isoformat(),
            source_path=str(source_path),
            backup_path=str(backup_path),
            item_count=len(items),
            file_size=file_size,
            checksum=checksum
        )
        
        # 更新索引
        self._index["backups"].append(info.to_dict())
        self._save_index()
        
        logger.info(f"备份创建成功: {backup_id}")
        return info
    
    def restore(self, backup_id: str, output_path: str) -> Dict:
        """恢复备份
        
        Args:
            backup_id: 备份ID
            output_path: 输出路径
        
        Returns:
            恢复结果
        """
        # 查找备份
        backup_info = None
        for backup in self._index["backups"]:
            if backup["backup_id"] == backup_id:
                backup_info = backup
                break
        
        if not backup_info:
            raise ValueError(f"备份不存在: {backup_id}")
        
        backup_path = Path(backup_info["backup_path"])
        if not backup_path.exists():
            raise FileNotFoundError(f"备份文件不存在: {backup_path}")
        
        # 验证校验和
        current_checksum = self._calculate_checksum(str(backup_path))
        if current_checksum != backup_info["checksum"]:
            logger.warning("备份文件校验和不匹配，文件可能已损坏")
        
        # 读取备份数据
        with open(backup_path, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        # 保存到输出路径
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        
        with open(output, 'w', encoding='utf-8') as f:
            json.dump(items, f, ensure_ascii=False, indent=2)
        
        return {
            "backup_id": backup_id,
            "output_path": str(output_path),
            "item_count": len(items)
        }
    
    def list_backups(self) -> List[Dict]:
        """列出所有备份
        
        Returns:
            备份列表
        """
        return self._index["backups"]
    
    def delete_backup(self, backup_id: str) -> bool:
        """删除备份
        
        Args:
            backup_id: 备份ID
        
        Returns:
            是否成功删除
        """
        # 查找备份
        backup_info = None
        for i, backup in enumerate(self._index["backups"]):
            if backup["backup_id"] == backup_id:
                backup_info = backup
                break
        
        if not backup_info:
            return False
        
        # 删除备份文件
        backup_path = Path(backup_info["backup_path"])
        if backup_path.exists():
            backup_path.unlink()
        
        # 更新索引
        self._index["backups"] = [
            b for b in self._index["backups"]
            if b["backup_id"] != backup_id
        ]
        self._save_index()
        
        logger.info(f"备份删除成功: {backup_id}")
        return True
    
    def get_backup_info(self, backup_id: str) -> Optional[Dict]:
        """获取备份信息
        
        Args:
            backup_id: 备份ID
        
        Returns:
            备份信息
        """
        for backup in self._index["backups"]:
            if backup["backup_id"] == backup_id:
                return backup
        return None


def create_backup(source_path: str, backup_dir: str = ".backups", name: str = None) -> BackupInfo:
    """创建备份
    
    Args:
        source_path: 源文件路径
        backup_dir: 备份目录
        name: 备份名称
    
    Returns:
        备份信息
    """
    manager = DatasetBackup(backup_dir)
    return manager.backup(source_path, name)


def restore_backup(backup_id: str, output_path: str, backup_dir: str = ".backups") -> Dict:
    """恢复备份
    
    Args:
        backup_id: 备份ID
        output_path: 输出路径
        backup_dir: 备份目录
    
    Returns:
        恢复结果
    """
    manager = DatasetBackup(backup_dir)
    return manager.restore(backup_id, output_path)


def list_backups(backup_dir: str = ".backups") -> List[Dict]:
    """列出所有备份
    
    Args:
        backup_dir: 备份目录
    
    Returns:
        备份列表
    """
    manager = DatasetBackup(backup_dir)
    return manager.list_backups()


def delete_backup(backup_id: str, backup_dir: str = ".backups") -> bool:
    """删除备份
    
    Args:
        backup_id: 备份ID
        backup_dir: 备份目录
    
    Returns:
        是否成功删除
    """
    manager = DatasetBackup(backup_dir)
    return manager.delete_backup(backup_id)


def clean_old_backups(backup_dir: str = ".backups", max_backups: int = 10) -> int:
    """清理旧备份（增强功能：自动维护备份数量限制）
    
    Args:
        backup_dir: 备份目录
        max_backups: 最大保留备份数
    
    Returns:
        删除的备份数量
    """
    manager = DatasetBackup(backup_dir)
    backups = manager.list_backups()
    if len(backups) <= max_backups:
        return 0
    
    # 按时间戳排序，删除最旧的备份
    sorted_backups = sorted(backups, key=lambda b: b.get("timestamp", ""))
    to_delete = sorted_backups[:len(sorted_backups) - max_backups]
    deleted_count = 0
    
    for backup in to_delete:
        if manager.delete_backup(backup["backup_id"]):
            deleted_count += 1
    
    logger.info(f"自动清理旧备份完成: 删除 {deleted_count} 个，保留最新 {max_backups} 个")
    return deleted_count


def verify_backup_integrity(backup_dir: str = ".backups", backup_id: Optional[str] = None) -> Dict:
    """验证备份完整性（增强功能：批量校验备份文件）
    
    Args:
        backup_dir: 备份目录
        backup_id: 指定备份ID，为 None 时验证所有备份
    
    Returns:
        验证结果（包含每个备份的校验状态）
    """
    manager = DatasetBackup(backup_dir)
    backups = manager.list_backups()
    results = {}
    
    for backup in backups:
        if backup_id is not None and backup["backup_id"] != backup_id:
            continue
        
        backup_path = Path(backup["backup_path"])
        is_valid = backup_path.exists() and backup_path.stat().st_size > 0
        current_checksum = manager._calculate_checksum(str(backup_path)) if is_valid else None
        checksum_match = current_checksum == backup.get("checksum") if is_valid and current_checksum else False
        
        results[backup["backup_id"]] = {
            "exists": is_valid,
            "checksum_match": checksum_match,
            "item_count": backup.get("item_count", 0),
            "timestamp": backup.get("timestamp", "")
        }

    return results
