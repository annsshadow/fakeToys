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
            max_size: 最大缓存条目数
            default_ttl: 默认生存时间（秒）
        """
        self._cache: Dict[str, CacheEntry] = {}
        self._max_size = max_size
        self._default_ttl = default_ttl
        self._hits = 0
        self._misses = 0
    
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
        # 检查是否需要清理
        if len(self._cache) >= self._max_size:
            self._evict()
        
        # 性能优化：记录缓存命中统计（便于监控缓存效率）
        self._stats = getattr(self, '_stats', {"hits": 0, "misses": 0, "sets": 0})
        self._stats["sets"] = self._stats.get("sets", 0) + 1
        
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
            # 删除一半
            for key in sorted_keys[:len(sorted_keys)//2]:
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
    
    基于文件的缓存实现。
    """
    
    def __init__(self, cache_dir: str = ".cache", default_ttl: Optional[float] = None):
        """初始化缓存
        
        Args:
            cache_dir: 缓存目录
            default_ttl: 默认生存时间（秒）
        """
        self._cache_dir = Path(cache_dir)
        self._cache_dir.mkdir(parents=True, exist_ok=True)
        self._default_ttl = default_ttl
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
        files = list(self._cache_dir.glob("*.json"))
        total_size = sum(f.stat().st_size for f in files)
        
        return {
            "entries": len(self._metadata),
            "total_size_bytes": total_size,
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
        self._cache = cache or MemoryCache()
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


def create_disk_cache(cache_dir: str = ".cache", default_ttl: Optional[float] = None) -> DiskCache:
    """创建磁盘缓存
    
    Args:
        cache_dir: 缓存目录
        default_ttl: 默认生存时间
    
    Returns:
        磁盘缓存实例
    """
    return DiskCache(cache_dir, default_ttl)
