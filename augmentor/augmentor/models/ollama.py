# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Ollama 本地模型后端 - 优化版"""

import json
from typing import Optional
from .base import ModelBackend
from ..config import ModelConfig
from ..exceptions import ModelGenerateError, ModelNotConfiguredError, ModelResponseError


class OllamaBackend(ModelBackend):
    """Ollama 本地模型后端 - 优化版"""
    
    def __init__(self,
                 config: ModelConfig,
                 response_cache_dir: Optional[str] = None,
                 response_cache_ttl: Optional[float] = None,
                 response_cache_max_bytes: Optional[int] = None):
        """初始化 Ollama 后端

        Args:
            config: 模型配置，必须包含 base_url
            response_cache_dir: 磁盘响应缓存目录，None 即不启用（见基类说明）
            response_cache_ttl: 磁盘缓存生存时间（秒）
            response_cache_max_bytes: 磁盘缓存容量上限（字节）
        """
        super().__init__(
            config,
            response_cache_dir=response_cache_dir,
            response_cache_ttl=response_cache_ttl,
            response_cache_max_bytes=response_cache_max_bytes,
        )
        if not config.base_url:
            raise ModelNotConfiguredError("Ollama 后端需要 base_url")
        
        self.api_url = f"{config.base_url.rstrip('/')}/api/chat"
    
    def _call_api(self, prompt: str) -> str:
        """调用 Ollama API（使用连接池）
        
        Args:
            prompt: 输入提示
        
        Returns:
            模型生成的文本
        """
        headers = {"Content-Type": "application/json"}
        
        payload = {
            "model": self.config.model,
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "stream": False,
            "options": {
                "temperature": self.config.temperature,
                "top_p": self.config.top_p,
                "num_predict": self.config.max_output_tokens
            }
        }
        
        session = self._get_session()
        response = session.post(self.api_url, json=payload, headers=headers, timeout=120)
        response.raise_for_status()
        
        data = response.json()
        
        if "message" not in data:
            raise ModelGenerateError(f"API 响应异常: {data}")
        
        return data["message"]["content"]
    
    def extract_json_from_response(self, response: str) -> list:
        """从响应中提取 JSON 数组
        
        Args:
            response: 模型响应文本
        
        Returns:
            JSON 数组
        """
        # 尝试直接解析
        try:
            result = json.loads(response)
            if isinstance(result, list):
                return result
        except json.JSONDecodeError:
            pass
        
        # 尝试从文本中提取
        start = response.find('[')
        end = response.rfind(']') + 1
        
        if start >= 0 and end > start:
            json_str = response[start:end]
            try:
                return json.loads(json_str)
            except json.JSONDecodeError as e:
                raise ModelResponseError(f"响应片段无法解析为 JSON 数组: {e}") from e
        
        raise ModelResponseError("无法从响应中提取 JSON 数组")
