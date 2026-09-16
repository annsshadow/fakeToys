"""百度 ERNIE 模型后端"""

import json
import requests
from .base import ModelBackend
from ..config import ModelConfig


class ERNIEBackend(ModelBackend):
    """百度 ERNIE 模型后端"""
    
    # API 端点模板
    TOKEN_URL = "https://aip.baidubce.com/oauth/2.0/token"
    CHAT_URL = "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions"
    
    def __init__(self, config: ModelConfig):
        """初始化 ERNIE 后端
        
        Args:
            config: 模型配置，必须包含 api_key 和 secret_key
        """
        super().__init__(config)
        if not config.api_key or not config.secret_key:
            raise ValueError("ERNIE 后端需要 api_key 和 secret_key")
        self._access_token = None
    
    def _get_access_token(self) -> str:
        """获取访问令牌
        
        Returns:
            访问令牌
        """
        if self._access_token:
            return self._access_token
        
        params = {
            "grant_type": "client_credentials",
            "client_id": self.config.api_key,
            "client_secret": self.config.secret_key
        }
        
        response = requests.post(self.TOKEN_URL, params=params)
        response.raise_for_status()
        
        data = response.json()
        self._access_token = data.get("access_token")
        
        if not self._access_token:
            raise RuntimeError(f"获取访问令牌失败: {data}")
        
        return self._access_token
    
    def _call_api(self, prompt: str) -> str:
        """调用 ERNIE API
        
        Args:
            prompt: 输入提示
        
        Returns:
            模型生成的文本
        """
        access_token = self._get_access_token()
        url = f"{self.CHAT_URL}?access_token={access_token}"
        
        payload = {
            "temperature": self.config.temperature,
            "top_p": self.config.top_p,
            "max_output_tokens": self.config.max_output_tokens,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        }
        
        headers = {"Content-Type": "application/json"}
        
        response = requests.post(url, json=payload, headers=headers)
        response.raise_for_status()
        
        data = response.json()
        
        if data.get("is_truncated"):
            raise RuntimeError("响应被截断")
        
        return data.get("result", "")
    
    def extract_json_from_response(self, response: str) -> list:
        """从响应中提取 JSON 数组
        
        Args:
            response: 模型响应文本
        
        Returns:
            JSON 数组
        """
        start = response.find('[')
        end = response.rfind(']') + 1
        
        if start >= 0 and end > start:
            json_str = response[start:end]
            return json.loads(json_str)
        
        raise ValueError("无法从响应中提取 JSON 数组")
