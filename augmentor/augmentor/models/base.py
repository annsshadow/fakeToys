# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""模型后端基类 - 优化版"""

import hashlib
import json
import logging
import threading
from abc import ABC, abstractmethod
from typing import Optional
from ..config import ModelConfig
from ..cache import MemoryCache, DiskCache
from ..exceptions import ModelGenerateError, ModelResponseError
from ..retry import with_retries, classify_error
from ..validation import require_count

logger = logging.getLogger(__name__)


def extract_json_array(response: str) -> list:
    """从模型响应中提取 JSON 数组

    先尝试直接解析，失败后截取首个 '[' 到末个 ']' 之间的内容再解析。

    Args:
        response: 模型响应文本

    Returns:
        JSON 数组

    Raises:
        ModelResponseError: 无法提取出 JSON 数组
    """
    try:
        result = json.loads(response)
        if isinstance(result, list):
            return result
    except json.JSONDecodeError:
        pass

    start = response.find('[')
    end = response.rfind(']') + 1

    if start >= 0 and end > start:
        try:
            return json.loads(response[start:end])
        except json.JSONDecodeError as e:
            raise ModelResponseError(f"响应片段无法解析为 JSON 数组: {e}") from e

    raise ModelResponseError("无法从响应中提取 JSON 数组")


class ModelBackend(ABC):
    """模型后端抽象基类 - 优化版"""

    # 生成结果缓存上限。原实现是无上限的 dict：批量增强上万条数据时，
    # 每次响应都会永久留在内存里（与 dedup 的 O(n²) 同类的长跑内存问题）。
    # 现在统一复用 cache.MemoryCache，容量与淘汰逻辑只维护一处。
    _GENERATION_CACHE_MAX = 512

    # 单次重试等待上限（秒），防止退避时间无界增长
    _MAX_RETRY_DELAY = 30.0

    # 磁盘响应缓存默认容量上限（256 MB）。磁盘缓存必须比内存缓存更保守：
    # 没有上限的磁盘缓存只是把内存泄漏换成了磁盘泄漏。
    DEFAULT_RESPONSE_CACHE_MAX_BYTES = 256 * 1024 * 1024

    def __init__(self,
                 config: ModelConfig,
                 response_cache_dir: Optional[str] = None,
                 response_cache_ttl: Optional[float] = None,
                 response_cache_max_bytes: Optional[int] = None):
        """初始化模型后端

        Args:
            config: 模型配置
            response_cache_dir: 磁盘响应缓存目录。**默认 None 即不启用**——
                启用意味着把 prompt 与模型响应写入磁盘，属于需要显式选择的行为
                （既涉及磁盘占用，也涉及数据落盘的隐私语义）
            response_cache_ttl: 磁盘缓存生存时间（秒），None 表示不按时间过期
            response_cache_max_bytes: 磁盘缓存容量上限（字节），
                None 时用 DEFAULT_RESPONSE_CACHE_MAX_BYTES
        """
        self.config = config
        self._request_count = 0
        self._error_count = 0
        self._lock = threading.Lock()
        self._session = None  # 连接池会话
        self._generation_cache = MemoryCache(max_size=self._GENERATION_CACHE_MAX)
        self._response_cache = self._build_response_cache(
            response_cache_dir, response_cache_ttl, response_cache_max_bytes
        )

    def _build_response_cache(self,
                              cache_dir: Optional[str],
                              ttl: Optional[float],
                              max_bytes: Optional[int]) -> Optional[DiskCache]:
        """构造磁盘响应缓存（未启用或目录不可用时返回 None）

        `DiskCache.__init__` 会 `mkdir(parents=True)`，目录不可写/被占用时会抛
        OSError。缓存只是优化手段，不该因为它建不起来就让整个后端不可用，
        因此这里降级为「不缓存」并留一条 warning。
        """
        if not cache_dir:
            return None
        try:
            return DiskCache(
                cache_dir,
                default_ttl=ttl,
                max_bytes=(
                    max_bytes if max_bytes is not None
                    else self.DEFAULT_RESPONSE_CACHE_MAX_BYTES
                ),
            )
        except OSError:
            logger.warning(
                "磁盘响应缓存目录不可用，已降级为不启用: %s", cache_dir, exc_info=True
            )
            return None
    
    def _get_session(self):
        """获取或创建 HTTP 会话（线程安全）
        
        Returns:
            requests.Session 或 httpx.Client
        """
        if self._session is None:
            with self._lock:
                if self._session is None:
                    try:
                        import requests
                        from requests.adapters import HTTPAdapter
                        from urllib3.util.retry import Retry
                        
                        session = requests.Session()
                        
                        # 配置连接池和重试
                        retry_strategy = Retry(
                            total=3,
                            backoff_factor=0.1,
                            status_forcelist=[429, 500, 502, 503, 504]
                        )
                        
                        adapter = HTTPAdapter(
                            max_retries=retry_strategy,
                            pool_connections=10,
                            pool_maxsize=20
                        )
                        
                        session.mount("http://", adapter)
                        session.mount("https://", adapter)
                        
                        self._session = session
                        logger.info("创建 HTTP 会话（带连接池）")
                    except ImportError:
                        logger.warning("requests 未安装，使用基础连接")
                        import requests
                        self._session = requests.Session()
        
        return self._session
    
    @abstractmethod
    def _call_api(self, prompt: str) -> str:
        """调用模型 API（子类实现）
        
        Args:
            prompt: 输入提示
        
        Returns:
            模型生成的文本
        """
        pass
    
    def _cache_key(self, prompt: str) -> str:
        """生成缓存键（内存缓存与磁盘缓存共用）

        两个硬约束：

        1. **必须覆盖所有影响输出的输入**——provider / model / 采样参数 / prompt。
           只用 prompt 做键的话，同一个后端实例改了模型或温度之后，会命中上一个
           配置留下的响应，且不会有任何报错。
        2. **必须是跨进程稳定的摘要**。内置 `hash()` 对 str 带进程随机盐
           （PYTHONHASHSEED），拿它当磁盘键会让缓存跨进程永远不命中，
           表现为「磁盘缓存文件越来越多但命中率为 0」。
        """
        payload = "\x00".join([
            str(self.config.type or ""),
            str(self.config.model or ""),
            repr(self.config.temperature),
            repr(self.config.top_p),
            repr(self.config.max_output_tokens),
            prompt,
        ])
        return hashlib.sha256(payload.encode("utf-8")).hexdigest()

    @property
    def response_cache(self) -> Optional[DiskCache]:
        """磁盘响应缓存实例（未启用时为 None），便于查看 stats"""
        return self._response_cache

    def _cache_get(self, key):
        """读取生成结果缓存（未命中返回 None）"""
        cache = getattr(self, "_generation_cache", None)
        if cache is None:
            return None
        return cache.get(key)

    def _cache_put(self, key, value):
        """写入生成结果缓存（容量由 MemoryCache 依据 max_size 保证有界）"""
        cache = getattr(self, "_generation_cache", None)
        if cache is None:
            cache = MemoryCache(max_size=self._GENERATION_CACHE_MAX)
            self._generation_cache = cache
        cache.set(key, value)

    def generate(self, prompt: str, max_retries: Optional[int] = None, retry_delay: Optional[float] = None) -> str:
        """生成文本（带指数退避重试）

        Args:
            prompt: 输入提示
            max_retries: **总尝试次数**（含首次调用），默认 3。必须是不小于 0 的
                整数；0 读作「不重试」（即只调用一次）。
            retry_delay: 首次重试的基础等待（秒），默认 1.0
        
        Returns:
            模型生成的文本
        
        Raises:
            ModelGenerateError: 重试次数用尽后仍失败，或遇到不可重试的错误
                （如 401/403/404 —— 重试只会浪费配额）
            DataValidationError: max_retries 为负数或非整数
        """
        # max_retries 的既有语义是「总尝试次数」，不是「额外重试次数」
        require_count("max_retries", max_retries, minimum=0)
        attempts = max(1, max_retries if max_retries is not None else 3)
        delay = retry_delay if retry_delay is not None else 1.0

        # 缓存复用优化：内存缓存 → 磁盘缓存 → 真正调用
        cache_key = self._cache_key(prompt)
        cached = self._cache_get(cache_key)
        if cached is not None:
            logger.debug("内存缓存命中，直接返回结果")
            with self._lock:
                self._request_count += 1  # 命中仍计入统计（既有语义）
            return cached

        disk_cached = (
            self._response_cache.get(cache_key) if self._response_cache is not None else None
        )
        if isinstance(disk_cached, str):
            logger.debug("磁盘缓存命中，回填内存缓存")
            self._cache_put(cache_key, disk_cached)
            with self._lock:
                self._request_count += 1
            return disk_cached

        def _attempt() -> str:
            with self._lock:
                self._request_count += 1
            try:
                return self._call_api(prompt)
            except Exception:
                with self._lock:
                    self._error_count += 1
                raise

        try:
            # 统一走 retry.py：退避有上限，且由 classify_error 区分
            # 「限流/网络抖动」（可重试）与「鉴权/参数错误」（立即放弃）
            result, _stats = with_retries(
                _attempt,
                max_retries=attempts - 1,
                base_delay=delay,
                max_delay=self._MAX_RETRY_DELAY,
                classify=classify_error,
            )
        except Exception as e:
            raise ModelGenerateError(
                f"API 调用失败，共尝试 {attempts} 次: {e}"
            ) from e

        self._cache_put(cache_key, result)
        if self._response_cache is not None:
            # 磁盘写入失败不应让已经成功的生成变成失败
            try:
                self._response_cache.set(cache_key, result)
            except OSError:
                logger.warning("磁盘响应缓存写入失败，本次结果不回填磁盘", exc_info=True)
        return result
    
    @property
    def request_count(self) -> int:
        """请求次数"""
        return self._request_count
    
    @property
    def error_count(self) -> int:
        """错误次数"""
        return self._error_count
    
    def reset_stats(self):
        """重置统计信息"""
        with self._lock:
            self._request_count = 0
            self._error_count = 0
    
    def close(self):
        """关闭连接

        用 getattr 兜底：__init__ 在赋值 _session 之前就抛异常（例如子类校验
        密钥失败）时，对象仍会被 GC 回收并触发本方法，直接访问 self._session
        会再抛 AttributeError 并污染解释器退出流程。
        """
        session = getattr(self, "_session", None)
        if session is not None:
            try:
                session.close()
            except Exception:  # 关闭失败不应影响回收
                logger.debug("关闭 HTTP 会话失败", exc_info=True)
            self._session = None

    def __del__(self):
        """析构函数

        析构中抛出的异常会被解释器忽略并打印，但会干扰测试（pytest 会报
        PytestUnraisableExceptionWarning），因此这里整体兜底。
        """
        try:
            self.close()
        except Exception:
            pass
