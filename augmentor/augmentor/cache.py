# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集缓存模块

提供数据集处理结果的缓存功能，避免重复计算。
"""

import functools
import json
import hashlib
import logging
import threading
import time
from typing import Any, Optional, Callable, Dict
from pathlib import Path
from dataclasses import dataclass
from datetime import datetime, timedelta

from .atomic_write import atomic_write_json

logger = logging.getLogger(__name__)


@dataclass
class CacheEntry:
    """缓存条目"""
    key: str
    value: Any
    created_at: float
    ttl: Optional[float]  # 生存时间（秒）
    hit_count: int = 0
    
    @property
    def is_expired(self) -> bool:
        """检查是否过期"""
        if self.ttl is None:
            return False
        return time.time() - self.created_at > self.ttl
    
    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "key": self.key,
            "created_at": self.created_at,
            "ttl": self.ttl,
            "hit_count": self.hit_count,
            "is_expired": self.is_expired
        }


def _synchronized(method):
    """把两只缓存的公开入口串到实例锁上

    两面各有各的账要罩。`DiskCache` 的账本同时是**文件**（`_metadata.json` 与逐条
    `md5(key).json`），一次交错就能让 `del` 落在别人刚删掉的键上抛 `KeyError`，
    或让两份 `_save_metadata` 互相覆盖成半份账（读侧 `_load_metadata` 没有
    except，下一份新构造的 DiskCache 会被那份坏 JSON 当场炸掉构造）。
    `MemoryCache` 改前写的是「豁免面」，理由是「后者只碰进程内的 dict/计数器，竞态
    最坏结果是少统计一次」——L279 把这句话证伪了：`self._cache` 换成一只在首读处停住
    的 dict 替身后，`get` 的 `if key in self._cache: entry = self._cache[key]` 与被删
    掉的键之间是 check-then-act，第二次读当场 `KeyError`；`_evict` 先按
    `sorted(self._cache.keys(), key=…)` 取键名单再逐个回读，名单里任何一键被别人删掉
    同样 `KeyError`。两条都产品可达：`models/base.py:183` 让整池工作线程共用一只
    `_generation_cache`，`context.py:175` 的 `ThreadPoolExecutor` 同时打进来。
    可重入是必需的：`DiskCache.get` 的过期分支会调 `self.delete`。
    """

    @functools.wraps(method)
    def wrapper(self, *args, **kwargs):
        with self._lock:
            return method(self, *args, **kwargs)

    return wrapper


class MemoryCache:
    """内存缓存
    
    基于内存的简单缓存实现。线程安全：公开入口全部串在实例级 RLock 上（L279）。
    """
    
    def __init__(self, max_size: int = 1000, default_ttl: Optional[float] = None):
        """初始化缓存
        
        Args:
            max_size: 最大缓存条目数，至少为 1（0 或负数会导致缓存永不淘汰）
            default_ttl: 默认生存时间（秒）
        """
        self._cache: Dict[str, CacheEntry] = {}
        # max_size <= 0 时 _evict 的 `len >= max_size` 恒真、`sorted_keys[:0]` 又删不掉
        # 任何条目，缓存会退化成无界 dict。这里兜底为 1。
        self._max_size = max(1, max_size)
        self._default_ttl = default_ttl
        self._hits = 0
        self._misses = 0
        # 实例级可重入锁：`models/base.py:183` 把一只 MemoryCache 交给整池工作线程共用，
        # 而 `get` 与 `_evict` 都是「先看键在不在、再回读它」的形状 —— L279 用停在首读处
        # 的 dict 替身把这个窗口钉成必现 `KeyError`（概率锤 8 线程 × 4000 轮打不出）。
        self._lock = threading.RLock()

    @_synchronized
    def __len__(self) -> int:
        """当前条目数（不含已过期但尚未清理的条目）"""
        return len(self._cache)

    @_synchronized
    def __contains__(self, key: str) -> bool:
        """键是否存在且未过期

        与 get() 的区别是不产生命中/未命中统计副作用。
        """
        entry = self._cache.get(key)
        return entry is not None and not entry.is_expired
    
    @_synchronized
    def get(self, key: str) -> Optional[Any]:
        """获取缓存值
        
        Args:
            key: 缓存键
        
        Returns:
            缓存值，不存在或过期时返回 None
        """
        if key in self._cache:
            entry = self._cache[key]
            if not entry.is_expired:
                entry.hit_count += 1
                self._hits += 1
                return entry.value
            else:
                # 过期，删除
                del self._cache[key]
        
        self._misses += 1
        return None
    
    @_synchronized
    def set(self, key: str, value: Any, ttl: Optional[float] = None):
        """设置缓存值
        
        Args:
            key: 缓存键
            value: 缓存值
            ttl: 生存时间（秒），为 None 时使用默认值
        """
        # 检查是否需要清理（覆盖已有键不会增长容量，不必白白淘汰一半条目）
        if key not in self._cache and len(self._cache) >= self._max_size:
            self._evict()
        
        self._cache[key] = CacheEntry(
            key=key,
            value=value,
            created_at=time.time(),
            ttl=ttl if ttl is not None else self._default_ttl  # L186: falsy-zero guard — ttl=0.0 means "expire now", not "unset"
        )
    
    @_synchronized
    def delete(self, key: str) -> bool:
        """删除缓存条目
        
        Args:
            key: 缓存键
        
        Returns:
            是否成功删除
        """
        if key in self._cache:
            del self._cache[key]
            return True
        return False
    
    @_synchronized
    def clear(self):
        """清空缓存"""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
    
    def _evict(self):
        """清理过期和最少使用的条目（`set` 已持锁，这里不再取锁 —— 再取一次就要靠 RLock 的可重入才不自锁）"""
        # 先删除过期条目
        expired_keys = [k for k, v in self._cache.items() if v.is_expired]
        for key in expired_keys:
            del self._cache[key]
        
        # 如果还是满的，删除最少使用的
        if len(self._cache) >= self._max_size:
            sorted_keys = sorted(self._cache.keys(), 
                               key=lambda k: self._cache[k].hit_count)
            # 至少淘汰一条：`len // 2` 在只剩 1 条时是 0，会一条都删不掉，
            # 缓存随即退化成无上限 dict（max_size=1 时必然触发）
            drop = max(1, len(sorted_keys) // 2)
            for key in sorted_keys[:drop]:
                del self._cache[key]
    
    @property
    @_synchronized
    def stats(self) -> Dict:
        """获取缓存统计"""
        total_requests = self._hits + self._misses
        return {
            "size": len(self._cache),
            "max_size": self._max_size,
            "hits": self._hits,
            "misses": self._misses,
            "hit_rate": self._hits / total_requests if total_requests > 0 else 0
        }


class DiskCache:
    """磁盘缓存
    
    基于文件的缓存实现。除 TTL 外可设置容量上限（max_bytes），
    超出时按写入时间淘汰最旧条目。
    """
    
    def __init__(self,
                 cache_dir: str = ".cache",
                 default_ttl: Optional[float] = None,
                 max_bytes: Optional[int] = None):
        """初始化缓存
        
        Args:
            cache_dir: 缓存目录
            default_ttl: 默认生存时间（秒）
            max_bytes: 缓存目录容量上限（字节），为 None 时不限制
        """
        self._cache_dir = Path(cache_dir)
        self._cache_dir.mkdir(parents=True, exist_ok=True)
        self._default_ttl = default_ttl
        self._max_bytes = max_bytes
        self._metadata_file = self._cache_dir / "_metadata.json"
        # 实例级可重入锁：`set` 会经 `_enforce_size_limit` 淘汰并 `del` 同一本
        # 元数据账，两个工作线程同时挤掉同一个旧键时，后到的那一次
        # `del` 当场 `KeyError`——它不是 `OSError`，穿透 `models/base.py` 的
        # 「磁盘写失败不该让已成功的生成变成失败」那一层捕获，把成功的条目记成失败。
        self._lock = threading.RLock()
        self._metadata = self._load_metadata()
    
    def _load_metadata(self) -> Dict:
        """加载元数据

        直接开文件、把 `FileNotFoundError` 读成「还没有账」，不再 `exists()` 之后
        `open()`：判过存在再动手是 check-then-act，索引文件落在两步之间被别人删掉时
        `open` 炸穿构造器，而 `_save_metadata` 走的是原子替换，正常情况下不该有半份档。
        """
        try:
            with open(self._metadata_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        except FileNotFoundError:
            return {}
    
    def _save_metadata(self):
        """保存元数据（L160 收原子写）

        读侧 `_load_metadata` 没有 except 且在 `__init__` 必调：写窗口里任何
        新构造的 DiskCache（共享同一 cache_dir 的第二实例）都会被半份元数据
        当场炸掉构造——按 atomic_write 的判据口径（读侧把坏 JSON 变成错误
        答案的必须收）走原子替换。`set` 的缓存值写边**刻意不收**：其读侧
        `get` 把坏 JSON 按未命中重算（可恢复代价，同口径明文「不必动」）。"""
        atomic_write_json(self._metadata_file, self._metadata)
    
    def _get_cache_path(self, key: str) -> Path:
        """获取缓存文件路径"""
        safe_key = hashlib.md5(key.encode()).hexdigest()
        return self._cache_dir / f"{safe_key}.json"
    
    @_synchronized
    def get(self, key: str) -> Optional[Any]:
        """获取缓存值
        
        Args:
            key: 缓存键
        
        Returns:
            缓存值
        """
        cache_path = self._get_cache_path(key)
        
        if not cache_path.exists():
            return None
        
        # 检查元数据
        meta = self._metadata.get(key, {})
        if meta:
            created_at = meta.get("created_at", 0)
            ttl = meta.get("ttl")
            # L186: falsy-zero guard — ttl=0.0 是「立即过期」，None 才是「不过期」；
            # 旧写法 `if ttl and ...` 把 0.0 读成「不过期」，与 set 侧同族假零。
            if ttl is not None and time.time() - created_at > ttl:
                # 过期，删除
                self.delete(key)
                return None
        
        try:
            with open(cache_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError):
            logger.debug("缓存读取失败，按未命中处理: %s", cache_path, exc_info=True)
            return None
    
    @_synchronized
    def set(self, key: str, value: Any, ttl: Optional[float] = None):
        """设置缓存值
        
        Args:
            key: 缓存键
            value: 缓存值
            ttl: 生存时间
        """
        cache_path = self._get_cache_path(key)
        
        with open(cache_path, 'w', encoding='utf-8') as f:
            json.dump(value, f, ensure_ascii=False, indent=2)
        
        # 更新元数据
        self._metadata[key] = {
            "created_at": time.time(),
            "ttl": ttl if ttl is not None else self._default_ttl  # L186: falsy-zero guard — ttl=0.0 means "expire now", not "unset"
        }
        self._save_metadata()
        self._enforce_size_limit(protect=key)

    def _enforce_size_limit(self, protect: Optional[str] = None):
        """按容量上限淘汰最旧的条目

        原实现只有 TTL 没有容量上限：一旦接进长跑路径（如模型响应缓存），
        磁盘会无限增长——那只是把内存泄漏换成了磁盘泄漏。

        `protect` 是 `set` 刚写下的那一条：它只贡献体积，不进被淘汰的名单。改前
        它自己也在名单里，于是**单条超过 `max_bytes` 的一次写入会先把整本缓存逐空、
        再把自己刚写的那只文件一起 unlink**（实测 `max_bytes=400`、先存三条小条目、
        再写一条 3000 字节：`_metadata` 与目录里的 `*.json` 同时清零，`get` 那条大
        条目读回 `None`），`set` 静默退化成 no-op，而且每写一次超大响应就毁一次缓存。
        改后最坏情况是目录比上限多一条（那一条本身就超线），换成 warning 出声。
        """
        if self._max_bytes is None:
            return

        # 文件路径可由 key 确定性推导（md5(key).json），无需在元数据里另存
        entries = []
        metadata_dirty = False
        protected_size = 0
        for key in list(self._metadata):
            path = self._get_cache_path(key)
            try:
                size = path.stat().st_size
            except FileNotFoundError:
                # 文件已被外部删除，顺手清理孤儿元数据。这里原先是
                # `if not path.exists(): …continue` 再 `path.stat()`，两步之间文件被
                # 别人删掉就当场 `FileNotFoundError`，而调用方 `set` 只把这次写入之前
                # 的账落完了盘——淘汰从此再没跑完过。
                del self._metadata[key]
                metadata_dirty = True
                continue
            if key == protect:
                protected_size = size
                continue
            entries.append((self._metadata[key].get("created_at", 0.0), key, path, size))

        total = sum(item[3] for item in entries) + protected_size
        if protected_size > self._max_bytes:
            logger.warning(
                "磁盘缓存单条 %s 字节已超过上限 %s，本条保留、其余条目不再牵连",
                protected_size, self._max_bytes
            )
        elif total > self._max_bytes:
            entries.sort(key=lambda item: item[0])  # 最旧的先淘汰
            for _, key, path, size in entries:
                if total <= self._max_bytes:
                    break
                path.unlink(missing_ok=True)
                total -= size
                del self._metadata[key]
            metadata_dirty = True
            logger.debug("磁盘缓存超出 %s 字节上限，已淘汰至 %s 字节", self._max_bytes, total)

        if metadata_dirty:
            self._save_metadata()
    
    @_synchronized
    def delete(self, key: str) -> bool:
        """删除缓存条目
        
        Args:
            key: 缓存键
        
        Returns:
            是否成功删除
        """
        cache_path = self._get_cache_path(key)
        # 直接 unlink 带 missing_ok，不在中间留「判过存在再动手」的窗口：本函数的
        # 调用点之一是 `get` 的过期分支，而 `models/base.py` 只把 `set` 那一侧的
        # OSError 罩住了（那句注释写的是「磁盘写入失败不该让已成功的生成变成失败」），
        # 读那一侧没罩 ⇒ 文件在这一步之间消失会把一次本该算未命中的生成整个判失败。
        cache_path.unlink(missing_ok=True)
        
        if key in self._metadata:
            del self._metadata[key]
            self._save_metadata()
        
        return True
    
    @_synchronized
    def clear(self):
        """清空缓存：只删**本缓存自己记账的**条目，目录里别人的 `*.json` 一律不碰"""
        owned = {hashlib.md5(k.encode()).hexdigest() for k in self._metadata}
        for file in self._cache_dir.glob("*.json"):
            if file.stem in owned:
                # glob 与动手之间文件可能已经不在（同一目录下第二个实例、运维手删、
                # 临时清理任务），同 `delete` 的口径走 missing_ok
                file.unlink(missing_ok=True)
        self._metadata.clear()
        self._save_metadata()
    
    @property
    @_synchronized
    def stats(self) -> Dict:
        """获取缓存统计"""
        # 不计入 _metadata.json：它是索引本身而非缓存内容，且 clear() /
        # _enforce_size_limit() 都把它排除在外，统计口径需要保持一致，
        # 否则 total_size_bytes 无法直接与 max_bytes 比较。
        files = [f for f in self._cache_dir.glob("*.json") if f.name != "_metadata.json"]
        # 诊断口径也得按「动手时文件可能已经不在」写：逐只 stat，消失的那只当作已经
        # 不在目录里（本来就不该计入），不让一次外部删除把运维读数炸成异常。
        sizes = []
        for file in files:
            try:
                sizes.append(file.stat().st_size)
            except FileNotFoundError:
                continue
        total_size = sum(sizes)
        
        return {
            "entries": len(self._metadata),
            "total_size_bytes": total_size,
            "max_bytes": self._max_bytes,
            "cache_dir": str(self._cache_dir)
        }


class CachedProcessor:
    """带缓存的处理器
    
    自动缓存处理结果，避免重复计算。
    """
    
    def __init__(self, 
                 processor: Callable,
                 cache: Optional[Any] = None,
                 cache_ttl: Optional[float] = 3600):
        """初始化处理器
        
        Args:
            processor: 处理函数
            cache: 缓存实例
            cache_ttl: 缓存生存时间
        """
        self._processor = processor
        # 必须用 `is not None` 判断：MemoryCache 定义了 __len__，
        # 空缓存是 falsy，`cache or MemoryCache()` 会把调用方传入的
        # 空缓存悄悄丢掉，导致 wrapper.clear_cache() 清的不是同一个对象。
        self._cache = cache if cache is not None else MemoryCache()
        self._cache_ttl = cache_ttl
    
    def _make_key(self, *args, **kwargs) -> str:
        """生成缓存键"""
        key_data = {
            "args": str(args),
            "kwargs": str(sorted(kwargs.items()))
        }
        return hashlib.md5(json.dumps(key_data, default=str).encode()).hexdigest()
    
    def process(self, *args, use_cache: bool = True, **kwargs) -> Any:
        """处理数据
        
        Args:
            *args: 位置参数
            use_cache: 是否使用缓存
            **kwargs: 关键字参数
        
        Returns:
            处理结果
        """
        if not use_cache:
            return self._processor(*args, **kwargs)
        
        cache_key = self._make_key(*args, **kwargs)
        
        # 尝试从缓存获取
        cached = self._cache.get(cache_key)
        if cached is not None:
            logger.debug(f"缓存命中: {cache_key[:8]}...")
            return cached
        
        # 执行处理
        result = self._processor(*args, **kwargs)
        
        # 存入缓存
        self._cache.set(cache_key, result, self._cache_ttl)
        logger.debug(f"缓存存储: {cache_key[:8]}...")
        
        return result


def cached(ttl: Optional[float] = 3600, cache_dir: Optional[str] = None):
    """缓存装饰器
    
    Args:
        ttl: 生存时间（秒）
        cache_dir: 缓存目录（为 None 时使用内存缓存）
    
    Returns:
        装饰器
    """
    def decorator(func: Callable) -> Callable:
        if cache_dir:
            cache = DiskCache(cache_dir, ttl)
        else:
            cache = MemoryCache(default_ttl=ttl)
        
        cached_processor = CachedProcessor(func, cache, ttl)
        
        def wrapper(*args, use_cache: bool = True, **kwargs):
            return cached_processor.process(*args, use_cache=use_cache, **kwargs)
        
        wrapper.cache = cache
        wrapper.clear_cache = cache.clear
        
        return wrapper
    
    return decorator


def create_memory_cache(max_size: int = 1000, default_ttl: Optional[float] = None) -> MemoryCache:
    """创建内存缓存
    
    Args:
        max_size: 最大缓存条目数
        default_ttl: 默认生存时间
    
    Returns:
        内存缓存实例
    """
    return MemoryCache(max_size, default_ttl)


def create_disk_cache(cache_dir: str = ".cache",
                      default_ttl: Optional[float] = None,
                      max_bytes: Optional[int] = None) -> DiskCache:
    """创建磁盘缓存
    
    Args:
        cache_dir: 缓存目录
        default_ttl: 默认生存时间
        max_bytes: 缓存目录容量上限（字节）
    
    Returns:
        磁盘缓存实例
    """
    return DiskCache(cache_dir, default_ttl, max_bytes)
