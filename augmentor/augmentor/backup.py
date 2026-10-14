# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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

from .atomic_write import atomic_write_json, path_lock
from .validation import path_lands_within, require_count
from .exceptions import BackupError, BackupIntegrityError, DataValidationError

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
        """初始化备份管理器（构造不落磁盘副作用，目录与索引等第一次真写去建）
        
        Args:
            backup_dir: 备份目录
        """
        self._backup_dir = Path(backup_dir)
        # 构造不碰磁盘（A332 / L249）：旧写法在这里 mkdir，只读面每探活一次就多一个目录
        self._index_file = self._backup_dir / "index.json"
        # L273：读也在这把锁里 —— 别人正把整份索引 `os.replace` 过去时，Windows 会给读者
        # 一个瞬时拒绝访问（`atomic_write_json` 的临时件命名管不到「目标正被读」这一侧）。
        with path_lock(self._index_file):
            self._index = self._load_index()
    
    def _load_index(self) -> Dict:
        """加载索引文件"""
        if self._index_file.exists():
            with open(self._index_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        # 索引不在场就是「还没有备份」：交回内存里的空账，**不落盘**（A332 / L249）。
        # 首次落盘延后到 _save_index，那一只仍走 atomic_write_json（父目录由它自己建），
        # 所以 L162 的原子写判据没有松动——写边从两条收成一条，而读边现在是零写。
        # 本方法无 except 且在 __init__ 必调：读到半份索引仍然响亮抛给调用方。
        return {"backups": []}
    
    def _save_index(self):
        """保存索引文件（L162 收原子写——读侧 _load_index 无 except，坏索引
        = 构造崩，错误答案档；快照与恢复输出两边不收：读侧响亮抛异常，且它们
        的「同名覆盖写中途崩毁既有」窗口是判据口径未覆盖的另一档，记档待裁）"""
        atomic_write_json(self._index_file, self._index)
    
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
        
        now = datetime.now()  # A303 / L250：只读一次钟 ⇒ 秒级 id 与 timestamp 同源
        backup_id = name or f"backup_{now.strftime('%Y%m%d_%H%M%S')}"
        backup_path = self._backup_dir / f"{backup_id}.json"
        # `name` 直接决定落点，而它来自调用方：`../../x` 会写到备份目录之外
        # （HTTP 面实测 200 且真落盘）——白名单只管到 backup_dir 那一格。
        if not path_lands_within(self._backup_dir.resolve(), backup_path.resolve()):
            raise DataValidationError(f"备份名称不能指向备份目录之外: {name}")
        
        # 读取源数据
        with open(source, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        # 保存备份（L170 收原子写：备份的语义就是数据保全——同名覆盖时
        # 写中途崩不该毁掉旧备份；os.replace 前旧内容完好）
        atomic_write_json(backup_path, items)
        
        # 计算文件大小和校验和
        file_size = backup_path.stat().st_size
        checksum = self._calculate_checksum(str(backup_path))
        
        # 创建备份信息
        info = BackupInfo(
            backup_id=backup_id,
            timestamp=now.isoformat(),
            source_path=str(source_path),
            backup_path=str(backup_path),
            item_count=len(items),
            file_size=file_size,
            checksum=checksum
        )
        
        # 更新索引（A298 / L250：同名先摘旧行 ⇒ 行数与磁盘份数一一对应。L273：整段
        # 读-改-写罩在按路径的锁里，**重读也在锁内** —— 少了它，20 次并发创建实测只剩
        # 1 行索引，而盘上实实在在 20 份备份，对 list / restore / delete 全部不可见）
        with path_lock(self._index_file):
            self._index = self._load_index()
            rows = [b for b in self._index["backups"] if b.get("backup_id") != backup_id]
            self._index["backups"] = rows + [info.to_dict()]
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
            raise BackupError(f"备份不存在: {backup_id}")
        
        backup_path = Path(backup_info["backup_path"])
        if not backup_path.exists():
            raise FileNotFoundError(f"备份文件不存在: {backup_path}")
        
        # 验证校验和
        current_checksum = self._calculate_checksum(str(backup_path))
        if current_checksum != backup_info["checksum"]:
            raise BackupIntegrityError(f"备份文件校验和不匹配，文件可能已损坏，未执行恢复: {backup_id}")
        
        # 读取备份数据
        with open(backup_path, 'r', encoding='utf-8') as f:
            items = json.load(f)
        
        # 保存到输出路径（L170 收原子写：恢复动作毁掉用户输出路径上的既有
        # 文件是自相矛盾的——同上，替换前旧内容完好）
        output = Path(output_path)
        output.parent.mkdir(parents=True, exist_ok=True)
        atomic_write_json(output, items)
        
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
        # L273：查找 → 删档 → 重排索引 → 落盘整段在锁内，而且查的是**锁内刚读到的那一份**。
        # 少了它，本方法会拿构造时那份旧账去写索引，把别的线程刚加进去的行原样抹回去。
        with path_lock(self._index_file):
            self._index = self._load_index()
            # 查找备份
            backup_info = None
            for backup in self._index["backups"]:
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
    # 判参必须排在 `DatasetBackup(backup_dir)` **之前**：坏参数不许在磁盘上留下任何东西
    # （L44 立的口径；构造自 L249 起本身已零副作用，这条先后仍然算数，见
    # `tests/unit/test_readonly_no_disk_writes_l249.py` 的 AST 那一档），而 HEAD 的删除
    # 算术 `sorted[:len - max_backups]` 在 -1 / -999 上等于 `sorted[:len+k]` → **删光全部
    # 备份**，日志却写「保留最新 -1 个」
    # （Temp 沙箱 5 个备份实测：-1 → 剩 0 个，与 0 无差别）。0 同样不放行：
    # 「保留 0 个备份」与手滑想打 10 无从分辨，而后果是不可逆删档。
    require_count("max_backups", max_backups, minimum=1)
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
