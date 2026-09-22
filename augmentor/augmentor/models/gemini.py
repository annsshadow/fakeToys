"""Google Gemini 模型后端"""

import logging
from .base import ModelBackend, extract_json_array
from ..config import ModelConfig

logger = logging.getLogger(__name__)


class GeminiBackend(ModelBackend):
    """Google Gemini 模型后端"""

    DEFAULT_BASE_URL = "https://generativelanguage.googleapis.com"
    API_VERSION = "v1beta"

    def __init__(self, config: ModelConfig):
        """初始化 Gemini 后端

        Args:
            config: 模型配置，必须包含 api_key
        """
        super().__init__(config)
        if not config.api_key:
            raise ValueError("Gemini 后端需要 api_key")

        base_url = (config.base_url or self.DEFAULT_BASE_URL).rstrip('/')
        self.api_url = (
            f"{base_url}/{self.API_VERSION}/models/"
            f"{config.model}:generateContent"
        )

    def _call_api(self, prompt: str) -> str:
        """调用 Gemini generateContent API（使用连接池）

        Args:
            prompt: 输入提示

        Returns:
            模型生成的文本
        """
        headers = {"Content-Type": "application/json"}
        params = {"key": self.config.api_key}

        payload = {
            "contents": [
                {"parts": [{"text": prompt}]}
            ],
            "generationConfig": {
                "temperature": self.config.temperature,
                "topP": self.config.top_p,
                "maxOutputTokens": self.config.max_output_tokens
            }
        }

        session = self._get_session()
        response = session.post(
            self.api_url, json=payload, headers=headers, params=params, timeout=60
        )
        response.raise_for_status()

        data = response.json()

        candidates = data.get("candidates")
        if not candidates:
            raise RuntimeError(f"API 响应异常: {data}")

        parts = candidates[0].get("content", {}).get("parts", [])
        return "".join(part.get("text", "") for part in parts)

    def extract_json_from_response(self, response: str) -> list:
        """从响应中提取 JSON 数组

        Args:
            response: 模型响应文本

        Returns:
            JSON 数组
        """
        return extract_json_array(response)
