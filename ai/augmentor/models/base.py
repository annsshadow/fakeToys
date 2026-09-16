"""模型后端基类 - 优化版"""

import time
import logging
import threading
from abc import ABC, abstractmethod
from typing import Optional
from ..config import ModelConfig

logger = logging.getLogger(__name__)


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
        
        last_error = None
        for attempt in range(retries):
            try:
                with self._lock:
                    self._request_count += 1
                result = self._call_api(prompt)
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
        """关闭连接"""
        if self._session:
            self._session.close()
            self._session = None
    
    def __del__(self):
        """析构函数"""
        self.close()
