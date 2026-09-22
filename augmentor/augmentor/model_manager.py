"""共享模型管理器 - 单例模式"""

import threading
import logging
from typing import Optional

logger = logging.getLogger(__name__)


class ModelManager:
    """模型管理器 - 单例模式，共享模型实例"""
    
    _instance = None
    _lock = threading.Lock()
    
    def __new__(cls):
        """单例模式"""
        if cls._instance is None:
            with cls._lock:
                if cls._instance is None:
                    cls._instance = super().__new__(cls)
                    cls._instance._initialized = False
        return cls._instance
    
    def __init__(self):
        """初始化模型管理器"""
        if self._initialized:
            return
        
        self._sentence_model = None
        self._model_lock = threading.Lock()
        self._initialized = True
    
    def get_sentence_model(self):
        """获取 sentence-transformers 模型（线程安全，带缓存复用优化）
        
        Returns:
            SentenceTransformer 模型实例或 None
        """
        if self._sentence_model is not None:
            # 模型缓存复用：直接返回已加载实例，避免重复初始化
            return self._sentence_model
        
        with self._model_lock:
            if self._sentence_model is not None:
                return self._sentence_model
            
            try:
                from sentence_transformers import SentenceTransformer
                self._sentence_model = SentenceTransformer('paraphrase-multilingual-MiniLM-L12-v2')
                logger.info("加载 sentence-transformers 模型成功（共享实例）")
                return self._sentence_model
            except ImportError:
                logger.warning("sentence-transformers 未安装")
                self._sentence_model = "fallback"
                return self._sentence_model
    
    def clear(self):
        """清空模型缓存"""
        with self._model_lock:
            self._sentence_model = None
            logger.info("清空模型缓存")


# 全局模型管理器实例
model_manager = ModelManager()
