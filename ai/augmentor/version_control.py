"""数据集版本控制模块

提供数据集的版本控制功能。
"""

import json
import hashlib
import logging
from typing import List, Dict, Optional, Any
from pathlib import Path
from dataclasses import dataclass, field
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class DatasetVersion:
    """数据集版本"""
    version_id: str
    version_number: str
    timestamp: str
    description: str
    item_count: int
    checksum: str
    tags: List[str] = field(default_factory=list)
    metadata: Dict[str, Any] = field(default_factory=dict)
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "version_id": self.version_id,
            "version_number": self.version_number,
            "timestamp": self.timestamp,
            "description": self.description,
            "item_count": self.item_count,
            "checksum": self.checksum,
            "tags": self.tags,
            "metadata": self.metadata
        }


class DatasetVersionManager:
    """数据集版本管理器
    
    提供数据集的版本控制功能。
    """
    
    def __init__(self, versions_dir: str = ".versions"):
        """初始化版本管理器
        
        Args:
            versions_dir: 版本目录
        """
        self._versions_dir = Path(versions_dir)
        self._versions_dir.mkdir(parents=True, exist_ok=True)
        self._index_file = self._versions_dir / "index.json"
        self._index = self._load_index()
    
    def _load_index(self) -> Dict:
        """加载索引文件"""
        if self._index_file.exists():
            with open(self._index_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        # 创建默认索引
        default_index = {"versions": [], "current_version": None}
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
    
    def create_version(self, items: List[Dict], description: str = "",
                      tags: List[str] = None, metadata: Dict[str, Any] = None) -> DatasetVersion:
        """创建新版本
        
        Args:
            items: 数据列表
            description: 版本描述
            tags: 标签列表
            metadata: 元数据
        
        Returns:
            版本信息
        """
        # 生成版本号
        version_number = self._generate_version_number()
        version_id = f"v{version_number}"
        
        # 创建版本目录
        version_dir = self._versions_dir / version_id
        version_dir.mkdir(exist_ok=True)
        
        # 保存数据
        data_file = version_dir / "data.json"
        with open(data_file, 'w', encoding='utf-8') as f:
            json.dump(items, f, ensure_ascii=False, indent=2)
        
        # 计算校验和
        checksum = self._calculate_checksum(str(data_file))
        
        # 创建版本信息
        version = DatasetVersion(
            version_id=version_id,
            version_number=version_number,
            timestamp=datetime.now().isoformat(),
            description=description,
            item_count=len(items),
            checksum=checksum,
            tags=tags or [],
            metadata=metadata or {}
        )
        
        # 保存版本信息
        version_file = version_dir / "version.json"
        with open(version_file, 'w', encoding='utf-8') as f:
            json.dump(version.to_dict(), f, ensure_ascii=False, indent=2)
        
        # 更新索引
        self._index["versions"].append(version.to_dict())
        self._index["current_version"] = version_id
        self._save_index()
        
        logger.info(f"版本创建成功: {version_id}")
        return version
    
    def _generate_version_number(self) -> str:
        """生成版本号
        
        Returns:
            版本号
        """
        if not self._index["versions"]:
            return "1.0.0"
        
        # 获取最新版本号
        latest = self._index["versions"][-1]["version_number"]
        parts = latest.split(".")
        
        # 递增版本号
        major, minor, patch = int(parts[0]), int(parts[1]), int(parts[2])
        patch += 1
        
        # 处理进位
        if patch >= 10:
            patch = 0
            minor += 1
        if minor >= 10:
            minor = 0
            major += 1
        
        return f"{major}.{minor}.{patch}"
    
    def get_version(self, version_id: str) -> Optional[DatasetVersion]:
        """获取版本信息
        
        Args:
            version_id: 版本ID
        
        Returns:
            版本信息
        """
        for version_data in self._index["versions"]:
            if version_data["version_id"] == version_id:
                return DatasetVersion(**version_data)
        return None
    
    def load_version(self, version_id: str) -> List[Dict]:
        """加载版本数据
        
        Args:
            version_id: 版本ID
        
        Returns:
            数据列表
        """
        version_dir = self._versions_dir / version_id
        data_file = version_dir / "data.json"
        
        if not data_file.exists():
            raise FileNotFoundError(f"版本数据不存在: {version_id}")
        
        with open(data_file, 'r', encoding='utf-8') as f:
            return json.load(f)
    
    def list_versions(self) -> List[Dict]:
        """列出所有版本
        
        Returns:
            版本列表
        """
        return self._index["versions"]
    
    def get_current_version(self) -> Optional[str]:
        """获取当前版本
        
        Returns:
            当前版本ID
        """
        return self._index.get("current_version")
    
    def set_current_version(self, version_id: str) -> bool:
        """设置当前版本
        
        Args:
            version_id: 版本ID
        
        Returns:
            是否成功
        """
        for version_data in self._index["versions"]:
            if version_data["version_id"] == version_id:
                self._index["current_version"] = version_id
                self._save_index()
                return True
        return False
    
    def delete_version(self, version_id: str) -> bool:
        """删除版本
        
        Args:
            version_id: 版本ID
        
        Returns:
            是否成功删除
        """
        # 查找版本
        version_index = None
        for i, version_data in enumerate(self._index["versions"]):
            if version_data["version_id"] == version_id:
                version_index = i
                break
        
        if version_index is None:
            return False
        
        # 删除版本目录
        version_dir = self._versions_dir / version_id
        if version_dir.exists():
            import shutil
            shutil.rmtree(version_dir)
        
        # 更新索引
        self._index["versions"].pop(version_index)
        
        # 更新当前版本
        if self._index["current_version"] == version_id:
            if self._index["versions"]:
                self._index["current_version"] = self._index["versions"][-1]["version_id"]
            else:
                self._index["current_version"] = None
        
        self._save_index()
        
        logger.info(f"版本删除成功: {version_id}")
        return True
    
    def compare_versions(self, version_id_a: str, version_id_b: str) -> Dict:
        """比较两个版本
        
        Args:
            version_id_a: 版本A ID
            version_id_b: 版本B ID
        
        Returns:
            比较结果
        """
        items_a = self.load_version(version_id_a)
        items_b = self.load_version(version_id_b)
        
        # 获取唯一标识
        keys_a = set(item.get("instruction", "") for item in items_a)
        keys_b = set(item.get("instruction", "") for item in items_b)
        
        return {
            "version_a": version_id_a,
            "version_b": version_id_b,
            "size_a": len(items_a),
            "size_b": len(items_b),
            "only_in_a": list(keys_a - keys_b),
            "only_in_b": list(keys_b - keys_a),
            "in_both": list(keys_a & keys_b),
            "stats": {
                "only_in_a_count": len(keys_a - keys_b),
                "only_in_b_count": len(keys_b - keys_a),
                "in_both_count": len(keys_a & keys_b)
            }
        }


def create_version(items: List[Dict], versions_dir: str = ".versions",
                  description: str = "", tags: List[str] = None) -> DatasetVersion:
    """创建版本
    
    Args:
        items: 数据列表
        versions_dir: 版本目录
        description: 版本描述
        tags: 标签列表
    
    Returns:
        版本信息
    """
    manager = DatasetVersionManager(versions_dir)
    return manager.create_version(items, description, tags)


def load_version(version_id: str, versions_dir: str = ".versions") -> List[Dict]:
    """加载版本
    
    Args:
        version_id: 版本ID
        versions_dir: 版本目录
    
    Returns:
        数据列表
    """
    manager = DatasetVersionManager(versions_dir)
    return manager.load_version(version_id)


def list_versions(versions_dir: str = ".versions") -> List[Dict]:
    """列出版本
    
    Args:
        versions_dir: 版本目录
    
    Returns:
        版本列表
    """
    manager = DatasetVersionManager(versions_dir)
    return manager.list_versions()
