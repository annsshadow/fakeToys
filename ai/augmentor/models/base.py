"""模型后端基类"""

import time
import logging
from abc import ABC, abstractmethod
from typing import Optional
from ..config import ModelConfig

logger = logging.getLogger(__name__)


class ModelBackend(ABC):
    """模型后端抽象基类"""
    
    def __init__(self, config: ModelConfig):
        """初始化模型后端
        
        Args:
            config: 模型配置
        """
        self.config = config
        self._request_count = 0
        self._error_count = 0
    
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
                self._request_count += 1
                result = self._call_api(prompt)
                return result
            except Exception as e:
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
        self._request_count = 0
        self._error_count = 0
