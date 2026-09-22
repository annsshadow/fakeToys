# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据版本管理模块

分工边界（与 `augmentor.version_control`）
    本模块的 `VersionManager` 与 `AugmentorPipeline` 绑定（pipeline 会自动建快照），
    CLI 的 `version` 命令走这里。
    脱离 pipeline、用独立目录管理版本，用 `version_control` 的模块级函数。
"""

import json
import shutil
import logging
from typing import List, Dict, Optional
from dataclasses import dataclass, asdict
from pathlib import Path
from datetime import datetime

logger = logging.getLogger(__name__)


@dataclass
class VersionInfo:
    """版本信息"""
    version_id: str  # 版本 ID
    label: str  # 版本标签
    description: str  # 版本描述
    created_at: str  # 创建时间
    item_count: int  # 数据条数
    metadata: Dict  # 元数据


@dataclass
class DiffResult:
    """差异对比结果"""
    version1: str  # 版本 1 ID
    version2: str  # 版本 2 ID
    added_count: int  # 新增数量
    removed_count: int  # 移除数量
    modified_count: int  # 修改数量
    added_items: List[Dict]  # 新增的项
    removed_items: List[Dict]  # 移除的项
    modified_items: List[Dict]  # 修改的项


class VersionManager:
    """数据版本管理器"""
    
    def __init__(self, storage_dir: str = "data/versions"):
        """初始化版本管理器
        
        Args:
            storage_dir: 版本存储目录
        """
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        
        self._current_symlink = self.storage_dir / "current"
        self._history_path = self.storage_dir / "history.jsonl"
    
    def _log_history(self, action: str, version_id: str, detail: Optional[Dict] = None):
        """追加一条版本操作历史记录
        
        Args:
            action: 操作类型（create / rollback / delete）
            version_id: 版本 ID
            detail: 附加信息
        """
        entry = {
            "action": action,
            "version_id": version_id,
            "timestamp": datetime.now().isoformat(),
            "detail": detail or {}
        }
        
        try:
            with open(self._history_path, 'a', encoding='utf-8') as f:
                f.write(json.dumps(entry, ensure_ascii=False) + "\n")
        except OSError as e:
            logger.warning(f"写入版本历史失败: {e}")
    
    def get_history(self, limit: Optional[int] = None) -> List[Dict]:
        """获取版本操作历史
        
        Args:
            limit: 最多返回的条数（从最新开始）
        
        Returns:
            历史记录列表（按时间倒序）
        """
        if not self._history_path.exists():
            return []
        
        entries = []
        with open(self._history_path, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    entries.append(json.loads(line))
                except json.JSONDecodeError:
                    logger.warning("跳过无法解析的历史记录行")
        
        entries.reverse()
        return entries[:limit] if limit else entries
    
    def _get_version_dir(self, version_id: str) -> Path:
        """获取版本目录路径
        
        Args:
            version_id: 版本 ID
        
        Returns:
            版本目录路径
        """
        return self.storage_dir / version_id
    
    def _generate_version_id(self) -> str:
        """生成版本 ID
        
        使用微秒级时间戳并做存在性去重，避免同一秒内连续创建导致快照互相覆盖。
        
        Returns:
            版本 ID
        """
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S_%f")
        version_id = f"v_{timestamp}"
        
        suffix = 1
        while self._get_version_dir(version_id).exists():
            version_id = f"v_{timestamp}_{suffix}"
            suffix += 1
        
        return version_id
    
    def create_version(self,
                      items: List[Dict],
                      label: Optional[str] = None,
                      description: str = "",
                      metadata: Optional[Dict] = None) -> VersionInfo:
        """创建新版本
        
        Args:
            items: 数据列表
            label: 版本标签
            description: 版本描述
            metadata: 元数据
        
        Returns:
            VersionInfo 实例
        """
        version_id = self._generate_version_id()
        version_dir = self._get_version_dir(version_id)
        version_dir.mkdir(parents=True, exist_ok=True)
        
        # 保存数据（紧凑格式）
        data_path = version_dir / "data.json"
        with open(data_path, 'w', encoding='utf-8') as f:
            json.dump(items, f, ensure_ascii=False, separators=(',', ':'))
        
        # 保存元数据
        version_info = VersionInfo(
            version_id=version_id,
            label=label or version_id,
            description=description,
            created_at=datetime.now().isoformat(),
            item_count=len(items),
            metadata=metadata or {}
        )
        
        info_path = version_dir / "metadata.json"
        with open(info_path, 'w', encoding='utf-8') as f:
            json.dump(asdict(version_info), f, ensure_ascii=False, indent=2)
        
        # 更新 current 符号链接
        if self._current_symlink.exists():
            if self._current_symlink.is_symlink():
                self._current_symlink.unlink()
            else:
                shutil.rmtree(self._current_symlink)
        
        # 创建新的符号链接（Windows 不支持真正的符号链接，使用目录副本）
        try:
            # 尝试创建符号链接
            self._current_symlink.symlink_to(version_dir)
        except OSError:
            # Windows 无管理员权限时，创建 .txt 文件记录当前版本
            with open(self._current_symlink.with_suffix('.txt'), 'w') as f:
                f.write(version_id)
        
        logger.info(f"创建版本: {version_id}, 数据条数: {len(items)}")
        self._log_history("create", version_id, {
            "label": version_info.label,
            "item_count": version_info.item_count
        })
        return version_info
    
    def load_version(self, version_id: str) -> List[Dict]:
        """加载版本数据
        
        Args:
            version_id: 版本 ID
        
        Returns:
            数据列表
        """
        version_dir = self._get_version_dir(version_id)
        data_path = version_dir / "data.json"
        
        if not data_path.exists():
            raise ValueError(f"版本不存在: {version_id}")
        
        with open(data_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    
    def get_version_info(self, version_id: str) -> VersionInfo:
        """获取版本信息
        
        Args:
            version_id: 版本 ID
        
        Returns:
            VersionInfo 实例
        """
        version_dir = self._get_version_dir(version_id)
        info_path = version_dir / "metadata.json"
        
        if not info_path.exists():
            raise ValueError(f"版本不存在: {version_id}")
        
        with open(info_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        
        return VersionInfo(**data)
    
    def list_versions(self) -> List[VersionInfo]:
        """列出所有版本
        
        Returns:
            版本信息列表
        """
        versions = []
        
        for version_dir in self.storage_dir.iterdir():
            if version_dir.is_dir() and version_dir.name != "current":
                info_path = version_dir / "metadata.json"
                if info_path.exists():
                    try:
                        info = self.get_version_info(version_dir.name)
                        versions.append(info)
                    except Exception as e:
                        logger.warning(f"加载版本信息失败: {version_dir.name}, {e}")
        
        # 按创建时间排序
        versions.sort(key=lambda x: x.created_at, reverse=True)
        return versions
    
    def get_current_version(self) -> Optional[str]:
        """获取当前版本 ID
        
        Returns:
            当前版本 ID，如果没有则返回 None
        """
        # 尝试读取符号链接
        if self._current_symlink.is_symlink():
            target = self._current_symlink.resolve()
            return target.name
        
        # 尝试读取 .txt 文件
        txt_path = self._current_symlink.with_suffix('.txt')
        if txt_path.exists():
            return txt_path.read_text().strip()
        
        return None
    
    def delete_version(self, version_id: str):
        """删除版本
        
        Args:
            version_id: 版本 ID
        """
        current = self.get_current_version()
        if current == version_id:
            raise ValueError("不能删除当前版本")
        
        version_dir = self._get_version_dir(version_id)
        if version_dir.exists():
            shutil.rmtree(version_dir)
            logger.info(f"删除版本: {version_id}")
            self._log_history("delete", version_id)
    
    def diff(self, version1_id: str, version2_id: str) -> DiffResult:
        """对比两个版本的差异
        
        Args:
            version1_id: 版本 1 ID
            version2_id: 版本 2 ID
        
        Returns:
            DiffResult 实例
        """
        items1 = self.load_version(version1_id)
        items2 = self.load_version(version2_id)
        
        # 转换为可比较的格式
        def item_to_key(item):
            return (item.get("instruction", ""), item.get("output", ""))
        
        keys1 = {item_to_key(item): item for item in items1}
        keys2 = {item_to_key(item): item for item in items2}
        
        # 计算差异
        added = []
        removed = []
        modified = []
        
        # 新增的项
        for key, item in keys2.items():
            if key not in keys1:
                added.append(item)
        
        # 移除的项
        for key, item in keys1.items():
            if key not in keys2:
                removed.append(item)
        
        return DiffResult(
            version1=version1_id,
            version2=version2_id,
            added_count=len(added),
            removed_count=len(removed),
            modified_count=len(modified),
            added_items=added,
            removed_items=removed,
            modified_items=modified
        )
    
    def rollback(self, version_id: str) -> bool:
        """回滚到指定版本
        
        Args:
            version_id: 版本 ID
        
        Returns:
            是否成功
        """
        version_dir = self._get_version_dir(version_id)
        
        if not version_dir.exists():
            logger.error(f"版本不存在: {version_id}")
            return False
        
        # 更新 current 符号链接
        if self._current_symlink.exists():
            if self._current_symlink.is_symlink():
                self._current_symlink.unlink()
            else:
                shutil.rmtree(self._current_symlink)
        
        try:
            self._current_symlink.symlink_to(version_dir)
        except OSError:
            with open(self._current_symlink.with_suffix('.txt'), 'w') as f:
                f.write(version_id)
        
        logger.info(f"回滚到版本: {version_id}")
        self._log_history("rollback", version_id)
        return True
    
    def generate_report(self) -> Dict:
        """生成版本管理报告
        
        Returns:
            报告字典
        """
        versions = self.list_versions()
        current = self.get_current_version()
        
        return {
            "total_versions": len(versions),
            "current_version": current,
            "recent_history": self.get_history(limit=10),
            "versions": [
                {
                    "version_id": v.version_id,
                    "label": v.label,
                    "item_count": v.item_count,
                    "created_at": v.created_at
                }
                for v in versions
            ]
        }
