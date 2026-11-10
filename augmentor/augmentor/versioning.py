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
from .atomic_write import path_lock
from .exceptions import VersionError
from .validation import require_count, require_within_dir

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

    def __init__(self, storage_dir: str = "data/versions", max_versions: int = 10):
        """初始化版本管理器

        Args:
            storage_dir: 版本存储目录
            max_versions: **保留份数**（不是删除份数）—— A409：自动快照这一路每
                跑一次增强就落一份**全量副本**，而这一族此前没有任何一侧封顶，
                于是 N 次增强在盘上留 N 份全量（实测 5 次 × 200 条 ⇒ 5 份完整
                副本）。这一格是**保留份数**语义，与备份族的 `max_backups` 同款
                口径：超出时删最旧的（删旧不拒新，见 `prune_versions`）。
        """
        # 判参排在 mkdir **之前**：坏参数不许在磁盘上留下任何东西（L44 立的口径）
        require_count("max_versions", max_versions, minimum=1)
        self.storage_dir = Path(storage_dir)
        self.storage_dir.mkdir(parents=True, exist_ok=True)
        self.max_versions = max_versions

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
            limit: 最多返回的条数（从最新开始）。`None` 表示「全部」，这是它特有的
                一档语义；0 表示「一条都不要」，与 `None` 不是一回事。负数与非整数
                报错，判据见 `augmentor.validation.require_count`
        
        Returns:
            历史记录列表（按时间倒序）
        """
        require_count("limit", limit)

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
        # 以前是 `if limit else entries`：`limit=0` 于是被读成「没传参数」而返回全部
        return entries[:limit] if limit is not None else entries
    
    def _get_version_dir(self, version_id: str) -> Path:
        """获取版本目录路径
        
        Args:
            version_id: 版本 ID
        
        Returns:
            版本目录路径
        """
        return require_within_dir("版本 ID", version_id, self.storage_dir)
    
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
        # L273：版本号申领、快照落盘、current 标记这三步是一体的，必须在一把按存储目录
        # 的锁里做完。少了它，并发 create_version 撞出两类实测后果：`_generate_version_id`
        # 的「先看存不存在、再 mkdir」分处两个时刻，两个线程能领到同一个 ID 而后者覆盖前者
        # 的快照；current 标记那一段更直接 —— 一支线程正写着 current.txt，另一支的
        # `_clear_current_link` 把它 unlink 掉，Windows 当场拒绝访问（24 支并发实测 1 支
        # PermissionError 而整支请求变 500，它已建好的版本目录却留在盘上，谁都不认它）。
        with path_lock(self.storage_dir):
            # A409：建版**之前**的 current 是要保护的那一版（见 `prune_versions`）
            previous_current = self.get_current_version()
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
        
            # 更新 current 符号链接：判存在与清除都住进 _clear_current_link。Windows 上对断链
            # 调 exists() 会抛 WinError 1920 而不是返回 False —— 本机 data/versions/current 曾是
            # WSL 在 drvfs 上留下的 LX 链接，is_symlink() 与 os.readlink() 都认不出来，于是
            # 「那里有没有东西」这个判断自己就能把整个 create_version 炸掉。
            self._clear_current_link()
        
            # 创建新的符号链接（Windows 无管理员特权时建不出来，落到下面的 .txt）。目标只写
            # 版本目录名：链接按自己所在目录解析，写整段相对路径会双重前缀而恒断链。
            try:
                self._current_symlink.symlink_to(version_dir.name)
            except OSError:
                # Windows 无管理员权限时（本机实测 WinError 1314）建不出符号链接，
                # 创建 .txt 文件记录当前版本 —— 读侧 get_current_version 两条都认
                with open(self._current_symlink.with_suffix('.txt'), 'w', encoding='utf-8') as f:
                    f.write(version_id)
        
            logger.info(f"创建版本: {version_id}, 数据条数: {len(items)}")
            self._log_history("create", version_id, {
                "label": version_info.label,
                "item_count": version_info.item_count
            })
            # A409：自动快照这一路每跑一次就落一份**全量副本**，而这一族此前
            # 没有任何一侧封顶 ⇒ N 次增强在盘上留 N 份全量。封顶在这里做
            # （建完新版本之后），而不是在 pipeline 侧另开一条路：谁建版本谁
            # 负责收旧，免得「自动快照封了、手工建的版本无限堆」这一族只治一半。
            # **要保护的是「建这一版之前的 current」**：建版过程已经把 current
            # 改指到新版本了，此刻再读 current 读到的是新版本 ⇒ 用户回滚到
            # 旧版本后跑一次增强，那份被回滚指着的版本会被当成「最旧」删掉，
            # current 就成了孤儿指针（读侧仍报得出那个 id，目录却没了）。
            self.prune_versions(protect=previous_current)
        return version_info

    def prune_versions(self, protect: Optional[str] = None) -> int:
        """按保留份数删掉最旧的版本，返回删除份数

        **保留份数语义**（`max_versions` 是「留几份」而不是「删几份」），
        与备份族的 `max_backups` 同款口径：超出时**删旧不拒新**——刚建的那一
        份永远在保留集里，被删的永远是最旧的。理由：这一族的用户动作是「增强
        结束自动打点」，拒新会让打点静默失败（而调用方拿不到任何信号），
        删旧则是可逆的（用户仍可从 history 知道发生过什么）。

        `protect` 是**要额外保护、不许删的那一版**（`create_version` 传的是建版
        之前的 current）。它自己占掉一个保留名额 ⇒ 保留集恒为 N 份。

        锁与 `create_version` 同一把（`path_lock(self.storage_dir)`）：判当前
        版本与动手删必须在同一趟里，否则并发下会把刚被设成 current 的那一版
        当成「最旧」删掉（L273 / A397 那一族的同款口径）。

        Returns:
            删除的版本份数（0 表示没超上限）
        """
        with path_lock(self.storage_dir):
            versions = self.list_versions()
            if len(versions) <= self.max_versions:
                return 0

            # 要保护的那一版：`protect`（建版之前的 current）优先；`protect` 为
            # None 时退回「此刻的 current」。两者都是**不许删**的那一版。
            protected = protect if protect is not None else self.get_current_version()
            # `list_versions` 已按 created_at 倒序 ⇒ 末尾那些是最旧的。
            # **受保护的那一版先从待删集里摘出去**：删掉它会让 `current` 指向不存在的
            # 目录，而读侧 `get_current_version` 仍报得出那个 id（孤儿指针）。
            # 保留集 = 受保护的那一版 + 最新的 (N-1) 份 ⇒ 待删 = 其余。受保护的
            # 那一版自己**占掉一个保留名额**，否则保留集是 N+1 份，封顶就没封住。
            candidates = [v for v in versions if v.version_id != protected]
            keep_others = max(0, self.max_versions - 1)
            to_delete = candidates[keep_others:]
            deleted = 0
            for info in to_delete:
                version_dir = self._get_version_dir(info.version_id)
                if not version_dir.exists():
                    continue
                shutil.rmtree(version_dir)
                self._log_history("prune", info.version_id, {
                    "kept": self.max_versions
                })
                deleted += 1

            if deleted:
                logger.info("版本封顶清理: 删除 %d 份，保留最新 %d 份",
                            deleted, self.max_versions)
            return deleted
    
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
            raise VersionError(f"版本不存在: {version_id}")
        
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
            raise VersionError(f"版本不存在: {version_id}")
        
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
            return txt_path.read_text(encoding='utf-8').strip()
        
        return None
    
    def delete_version(self, version_id: str):
        """删除版本
        
        Args:
            version_id: 版本 ID
        """
        with path_lock(self.storage_dir):  # L290 / A397：判 current 与动手必须同趟
            current = self.get_current_version()
            if current == version_id:
                raise VersionError("不能删除当前版本")
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

        # 身份键只用 instruction（一条 SFT 记录的自然主键）。若把 output 也算进键，
        # 「改了某条的输出」在 diff 里就会拆成「删掉旧 (instr, out1) + 新增 (instr, out2)」
        # 两条，而 modified 永远空——本方法承诺要能报出「修改数」（见 DiffResult 契约、
        # docs/augmentor/API.md 的 examples）。同一条 instruction 在单版本里出现多次时保留首条
        # （与原有 dict 折叠语义一致，不在本轮扩大范围）。
        def key_of(item):
            return item.get("instruction", "")

        keys1 = {key_of(item): item for item in items1}
        keys2 = {key_of(item): item for item in items2}

        # 计算差异
        added = []
        removed = []
        modified = []

        # 新增的项：instruction 只出现在 v2
        for key, item in keys2.items():
            if key not in keys1:
                added.append(item)

        # 移除的项：instruction 只出现在 v1
        for key, item in keys1.items():
            if key not in keys2:
                removed.append(item)

        # 修改的项：同一 instruction 两版都有，但整条记录不同（主要是 output 变了）
        for key, item2 in keys2.items():
            item1 = keys1.get(key)
            if item1 is not None and item1 != item2:
                modified.append(item2)

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
        with path_lock(self.storage_dir):  # L290 / A397：与 create_version 同抢一把
            version_dir = self._get_version_dir(version_id)
            if not version_dir.exists():
                logger.error(f"版本不存在: {version_id}")
                return False
            # 更新 current 符号链接：同 create_version。判存在与清除都住进 _clear_current_link，
            # 因为断链上 exists() 会抛 WinError 1920；目标同样只写版本目录名，写整段相对路径
            # 会得到 data/versions/data/versions/… 这种双重前缀的恒断链。
            self._clear_current_link()
            
            try:
                self._current_symlink.symlink_to(version_dir.name)
            except OSError:
                # 建不出链接（Windows 无管理员特权）时退回 .txt；_clear_current_link 已经把
                # 上一次那份 .txt 一起清掉了，所以读侧不会停在一个更早的版本上
                with open(self._current_symlink.with_suffix('.txt'), 'w', encoding='utf-8') as f:
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
    
    def _clear_current_link(self):
        """清掉 current 那一格：符号链接、跨子系统链接、真实目录，以及配套的 .txt
        
        为什么不用 `if exists()` 起头：Windows 上对**断链**调 `Path.exists()` 抛的是
        WinError 1920（ERROR_CANT_ACCESS_FILE）而不是返回 False；而 WSL 在 drvfs 上建出来的
        LX 链接连 `is_symlink()` 都认不出来（`os.readlink` 报 not a symbolic link）⇒ 这一格上
        任何「先看有没有东西」的写法都可能把调用方整个炸掉。所以先 lstat（is_symlink）再
        stat（exists），stat 抛出来就当作「那里有东西」。
        
        清除按「先当文件删、删不掉再当目录删」两步走：unlink 能掉符号链接、LX 链接与 .txt，
        真目录要走 rmtree。两步都失败就放过 —— 这一格是便利索引而不是数据，建不出新的也不该
        让 create_version / rollback 失败（随后的 symlink_to 会拿到 FileExistsError，落回 .txt）。
        """
        for path in (self._current_symlink, self._current_symlink.with_suffix('.txt')):
            try:
                stale = path.is_symlink() or path.exists()
            except OSError:
                stale = True
            if not stale:
                continue
            try:
                path.unlink()
            except OSError:
                shutil.rmtree(path, ignore_errors=True)
