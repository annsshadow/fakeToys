# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Anthropic Claude 模型后端"""

import logging
from typing import Optional
from .base import ModelBackend, extract_json_array
from ..config import ModelConfig
from ..exceptions import ModelGenerateError, ModelNotConfiguredError

logger = logging.getLogger(__name__)


class ClaudeBackend(ModelBackend):
    """Anthropic Claude 模型后端"""

    DEFAULT_BASE_URL = "https://api.anthropic.com"
    API_VERSION = "2023-06-01"

    def __init__(self,
                 config: ModelConfig,
                 response_cache_dir: Optional[str] = None,
                 response_cache_ttl: Optional[float] = None,
                 response_cache_max_bytes: Optional[int] = None,
                 default_attempts: Optional[int] = None,
                 default_retry_delay: Optional[float] = None,
                 default_max_retry_wait: Optional[float] = None,
                 default_retry_jitter: Optional[float] = None):
        """初始化 Claude 后端

        Args:
            config: 模型配置，必须包含 api_key
            response_cache_dir: 磁盘响应缓存目录，None 即不启用（见基类说明）
            response_cache_ttl: 磁盘缓存生存时间（秒）
            response_cache_max_bytes: 磁盘缓存容量上限（字节）
            default_attempts: 重试默认档位（总尝试次数），见基类说明
            default_retry_delay: 退避基数默认值（秒），见基类说明
            default_max_retry_wait: 服务端指令一支的等待上限（秒），见基类说明
            default_retry_jitter: 退避抖动比例（0-1），见基类说明
        """
        super().__init__(
            config,
            response_cache_dir=response_cache_dir,
            response_cache_ttl=response_cache_ttl,
            response_cache_max_bytes=response_cache_max_bytes,
            default_attempts=default_attempts,
            default_retry_delay=default_retry_delay,
            default_max_retry_wait=default_max_retry_wait,
            default_retry_jitter=default_retry_jitter,
        )
        if not config.api_key:
            raise ModelNotConfiguredError("Claude 后端需要 api_key")

        base_url = (config.base_url or self.DEFAULT_BASE_URL).rstrip('/')
        self.api_url = f"{base_url}/v1/messages"

    def _call_api(self, prompt: str) -> str:
        """调用 Claude Messages API（使用连接池）

        Args:
            prompt: 输入提示

        Returns:
            模型生成的文本
        """
        headers = {
            "x-api-key": self.config.api_key,
            "anthropic-version": self.API_VERSION,
            "Content-Type": "application/json"
        }

        payload = {
            "model": self.config.model,
            "max_tokens": self.config.max_output_tokens,
            "temperature": self.config.temperature,
            "top_p": self.config.top_p,
            "messages": [
                {"role": "user", "content": prompt}
            ]
        }

        session = self._get_session()
        response = session.post(self.api_url, json=payload, headers=headers, timeout=60)
        response.raise_for_status()

        data = response.json()

        content = data.get("content")
        if not content:
            raise ModelGenerateError(f"API 响应异常: {data}")

        # content 为块列表，拼接所有 text 块
        texts = [block.get("text", "") for block in content if block.get("type") == "text"]
        return "".join(texts)

    def extract_json_from_response(self, response: str) -> list:
        """从响应中提取 JSON 数组

        Args:
            response: 模型响应文本

        Returns:
            JSON 数组
        """
        return extract_json_array(response)
