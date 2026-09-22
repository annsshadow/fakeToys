# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""模型后端基类 - 优化版"""

import json
import time
import logging
import threading
from abc import ABC, abstractmethod
from typing import Optional
from ..config import ModelConfig

logger = logging.getLogger(__name__)


def extract_json_array(response: str) -> list:
    """从模型响应中提取 JSON 数组

    先尝试直接解析，失败后截取首个 '[' 到末个 ']' 之间的内容再解析。

    Args:
        response: 模型响应文本

    Returns:
        JSON 数组

    Raises:
        ValueError: 无法提取出 JSON 数组
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
        return json.loads(response[start:end])

    raise ValueError("无法从响应中提取 JSON 数组")


class ModelBackend(ABC):
    """模型后端抽象基类 - 优化版"""
    
    def __init__(self, config: ModelConfig):
        """初始化模型后端
        
        Args:
            config: 模型配置
        """
        self.config = config
        self._request_count = 0
        self._error_count = 0
        self._lock = threading.Lock()
        self._session = None  # 连接池会话
    
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
    
    def generate(self, prompt: str, max_retries: Optional[int] = None, retry_delay: Optional[float] = None) -> str:
        """生成文本（带重试机制）
        
        Args:
            prompt: 输入提示
            max_retries: 最大重试次数
            retry_delay: 重试间隔（秒）
        
        Returns:
            模型生成的文本
        
        Raises:
            RuntimeError: 重试次数用尽后仍失败
        """
        retries = max_retries if max_retries is not None else 3
        delay = retry_delay if retry_delay is not None else 1.0
        
        # 模型缓存复用优化：基于提示内容的简单缓存（避免重复生成相同内容）
        prompt_hash = hash(prompt)
        if hasattr(self, '_generation_cache') and prompt_hash in self._generation_cache:
            logger.debug(f"缓存命中，直接返回结果（提示hash: {prompt_hash})")
            with self._lock:
                self._request_count += 1  # 仍计入请求统计
            return self._generation_cache[prompt_hash]
        
        last_error = None
        for attempt in range(retries):
            try:
                with self._lock:
                    self._request_count += 1
                result = self._call_api(prompt)
                # 缓存生成结果（优化：避免重复生成）
                if not hasattr(self, '_generation_cache'):
                    self._generation_cache = {}
                self._generation_cache[prompt_hash] = result
                return result
            except Exception as e:
                with self._lock:
                    self._error_count += 1
                last_error = e
                logger.warning(f"API 调用失败 (尝试 {attempt + 1}/{retries}): {e}")
                if attempt < retries - 1:
                    time.sleep(delay * (2 ** attempt))  # 指数退避
        
        raise RuntimeError(f"API 调用失败，已重试 {retries} 次: {last_error}")
    
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
