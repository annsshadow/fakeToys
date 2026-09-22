# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""OpenAI 模型后端 - 优化版"""

import json
from .base import ModelBackend
from ..config import ModelConfig


class OpenAIBackend(ModelBackend):
    """OpenAI 模型后端 - 优化版"""
    
    API_URL = "https://api.openai.com/v1/chat/completions"
    
    def __init__(self, config: ModelConfig):
        """初始化 OpenAI 后端
        
        Args:
            config: 模型配置，必须包含 api_key
        """
        super().__init__(config)
        if not config.api_key:
            raise ValueError("OpenAI 后端需要 api_key")
    
    def _call_api(self, prompt: str) -> str:
        """调用 OpenAI API（使用连接池）
        
        Args:
            prompt: 输入提示
        
        Returns:
            模型生成的文本
        """
        headers = {
            "Authorization": f"Bearer {self.config.api_key}",
            "Content-Type": "application/json"
        }
        
        payload = {
            "model": self.config.model,
            "temperature": self.config.temperature,
            "top_p": self.config.top_p,
            "max_tokens": self.config.max_output_tokens,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        }
        
        session = self._get_session()
        response = session.post(self.API_URL, json=payload, headers=headers, timeout=60)
        response.raise_for_status()
        
        data = response.json()
        
        if "choices" not in data or len(data["choices"]) == 0:
            raise RuntimeError(f"API 响应异常: {data}")
        
        return data["choices"][0]["message"]["content"]
    
    def extract_json_from_response(self, response: str) -> list:
        """从响应中提取 JSON 数组
        
        Args:
            response: 模型响应文本
        
        Returns:
            JSON 数组
        """
        # 尝试直接解析
        try:
            return json.loads(response)
        except json.JSONDecodeError:
            pass
        
        # 尝试从文本中提取
        start = response.find('[')
        end = response.rfind(']') + 1
        
        if start >= 0 and end > start:
            json_str = response[start:end]
            return json.loads(json_str)
        
        raise ValueError("无法从响应中提取 JSON 数组")
