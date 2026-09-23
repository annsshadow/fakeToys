# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集缓存模块

提供数据集处理结果的缓存功能，避免重复计算。
"""

import json
import hashlib
import logging
import time
from typing import Any, Optional, Callable, Dict
from pathlib import Path
from dataclasses import dataclass
from datetime import datetime, timedelta

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


class MemoryCache:
    """内存缓存
    
    基于内存的简单缓存实现。
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

    def __len__(self) -> int:
        """当前条目数（不含已过期但尚未清理的条目）"""
        return len(self._cache)

    def __contains__(self, key: str) -> bool:
        """键是否存在且未过期

        与 get() 的区别是不产生命中/未命中统计副作用。
        """
        entry = self._cache.get(key)
        return entry is not None and not entry.is_expired
    
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
            ttl=ttl or self._default_ttl
        )
    
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
    
    def clear(self):
        """清空缓存"""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
    
    def _evict(self):
        """清理过期和最少使用的条目"""
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
        self._metadata = self._load_metadata()
    
    def _load_metadata(self) -> Dict:
        """加载元数据"""
        if self._metadata_file.exists():
            with open(self._metadata_file, 'r', encoding='utf-8') as f:
                return json.load(f)
        return {}
    
    def _save_metadata(self):
        """保存元数据"""
        with open(self._metadata_file, 'w', encoding='utf-8') as f:
            json.dump(self._metadata, f, ensure_ascii=False, indent=2)
    
    def _get_cache_path(self, key: str) -> Path:
        """获取缓存文件路径"""
        safe_key = hashlib.md5(key.encode()).hexdigest()
        return self._cache_dir / f"{safe_key}.json"
    
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
            if ttl and time.time() - created_at > ttl:
                # 过期，删除
                self.delete(key)
                return None
        
        try:
            with open(cache_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError):
            logger.debug("缓存读取失败，按未命中处理: %s", cache_path, exc_info=True)
            return None
    
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
            "ttl": ttl or self._default_ttl
        }
        self._save_metadata()
        self._enforce_size_limit()

    def _enforce_size_limit(self):
        """按容量上限淘汰最旧的条目

        原实现只有 TTL 没有容量上限：一旦接进长跑路径（如模型响应缓存），
        磁盘会无限增长——那只是把内存泄漏换成了磁盘泄漏。
        """
        if self._max_bytes is None:
            return

        # 文件路径可由 key 确定性推导（md5(key).json），无需在元数据里另存
        entries = []
        metadata_dirty = False
        for key in list(self._metadata):
            path = self._get_cache_path(key)
            if not path.exists():
                # 文件已被外部删除，顺手清理孤儿元数据
                del self._metadata[key]
                metadata_dirty = True
                continue
            entries.append((self._metadata[key].get("created_at", 0.0), key, path,
                            path.stat().st_size))

        total = sum(item[3] for item in entries)
        if total > self._max_bytes:
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
    
    def delete(self, key: str) -> bool:
        """删除缓存条目
        
        Args:
            key: 缓存键
        
        Returns:
            是否成功删除
        """
        cache_path = self._get_cache_path(key)
        
        if cache_path.exists():
            cache_path.unlink()
        
        if key in self._metadata:
            del self._metadata[key]
            self._save_metadata()
        
        return True
    
    def clear(self):
        """清空缓存"""
        for file in self._cache_dir.glob("*.json"):
            if file.name != "_metadata.json":
                file.unlink()
        self._metadata.clear()
        self._save_metadata()
    
    @property
    def stats(self) -> Dict:
        """获取缓存统计"""
        # 不计入 _metadata.json：它是索引本身而非缓存内容，且 clear() /
        # _enforce_size_limit() 都把它排除在外，统计口径需要保持一致，
        # 否则 total_size_bytes 无法直接与 max_bytes 比较。
        files = [f for f in self._cache_dir.glob("*.json") if f.name != "_metadata.json"]
        total_size = sum(f.stat().st_size for f in files)
        
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
