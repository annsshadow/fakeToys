# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Google Gemini 模型后端"""

import asyncio
import logging
from typing import Any, Optional

from .base import ModelBackend, extract_json_array
from ..config import ModelConfig
from ..exceptions import ModelGenerateError, ModelNotConfiguredError

logger = logging.getLogger(__name__)


class GeminiBackend(ModelBackend):
    """Google Gemini 模型后端"""

    DEFAULT_BASE_URL = "https://generativelanguage.googleapis.com"
    API_VERSION = "v1beta"

    def __init__(self,
                 config: ModelConfig,
                 response_cache_dir: Optional[str] = None,
                 response_cache_ttl: Optional[float] = None,
                 response_cache_max_bytes: Optional[int] = None,
                 default_attempts: Optional[int] = None,
                 default_retry_delay: Optional[float] = None,
                 default_max_retry_wait: Optional[float] = None,
                 default_retry_jitter: Optional[float] = None,
                 default_request_timeout: Optional[float] = None):
        """初始化 Gemini 后端

        Args:
            config: 模型配置，必须包含 api_key
            response_cache_dir: 磁盘响应缓存目录，None 即不启用（见基类说明）
            response_cache_ttl: 磁盘缓存生存时间（秒）
            response_cache_max_bytes: 磁盘缓存容量上限（字节）
            default_attempts: 重试默认档位（总尝试次数），见基类说明
            default_retry_delay: 退避基数默认值（秒），见基类说明
            default_max_retry_wait: 服务端指令一支的等待上限（秒），见基类说明
            default_retry_jitter: 退避抖动比例（0-1），见基类说明
            default_request_timeout: 单次请求超时（秒），见基类说明
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
            default_request_timeout=default_request_timeout,
        )
        if not config.api_key:
            raise ModelNotConfiguredError("Gemini 后端需要 api_key")

        base_url = (config.base_url or self.DEFAULT_BASE_URL).rstrip('/')
        self.api_url = (
            f"{base_url}/{self.API_VERSION}/models/"
            f"{config.model}:generateContent"
        )
        # 异步客户端与它所属的事件循环（同其他后端）。
        self._async_client: Optional[Any] = None
        self._async_loop: Optional[Any] = None

    def _build_request(self, prompt: str) -> tuple:
        """构造 (headers, payload, params)。同步与异步两条路径用同一份请求体。"""
        headers = {"Content-Type": "application/json"}
        params = {"key": self.config.api_key}
        payload = {
            "contents": [{"parts": [{"text": prompt}]}],
            "generationConfig": {
                "temperature": self.config.temperature,
                "topP": self.config.top_p,
                "maxOutputTokens": self.config.max_output_tokens,
            },
        }
        return headers, payload, params

    @staticmethod
    def _parse_response(data: Any) -> str:
        """从响应体取首个 candidate 的文本。同步/异步共用。"""
        candidates = data.get("candidates")
        if not candidates:
            raise ModelGenerateError(f"API 响应异常: {data}")
        parts = candidates[0].get("content", {}).get("parts", [])
        return "".join(part.get("text", "") for part in parts)

    def _call_api(self, prompt: str) -> str:
        """调用 Gemini generateContent API（使用连接池）

        Args:
            prompt: 输入提示

        Returns:
            模型生成的文本
        """
        headers, payload, params = self._build_request(prompt)
        session = self._get_session()
        response = session.post(
            self.api_url, json=payload, headers=headers, params=params,
            timeout=self._request_timeout,
        )
        response.raise_for_status()

        return self._parse_response(response.json())

    async def _acall_api(self, prompt: str) -> str:
        """原生异步调用 Gemini API（httpx.AsyncClient）。

        同其他后端：httpx 可选，缺它回退基类线程池实现；AsyncClient 跟随事件循环。
        """
        try:
            import httpx
        except ImportError:
            logger.warning(
                "httpx 不可用，异步调用回退为线程池（功能不受影响，但不省线程）"
            )
            return await super()._acall_api(prompt)

        headers, payload, params = self._build_request(prompt)
        client = await self._async_client_for(httpx)
        response = await client.post(
            self.api_url, json=payload, headers=headers, params=params
        )
        response.raise_for_status()
        return self._parse_response(response.json())

    async def _async_client_for(self, httpx_module: Any) -> Any:
        """取当前事件循环可用的 AsyncClient（换循环则重建）。"""
        loop = asyncio.get_running_loop()
        if self._async_client is not None and self._async_loop is loop:
            return self._async_client
        if self._async_client is not None:
            old, self._async_client = self._async_client, None
            try:
                await old.aclose()
            except Exception:
                logger.debug("关闭上一个异步客户端失败（循环可能已关闭）", exc_info=True)
        self._async_client = httpx_module.AsyncClient(timeout=self._request_timeout)
        self._async_loop = loop
        return self._async_client

    async def aclose(self) -> None:
        """关闭异步客户端（与基类 close() 同族，只是必须 await）。"""
        client, self._async_client = self._async_client, None
        self._async_loop = None
        if client is not None:
            await client.aclose()

    def extract_json_from_response(self, response: str) -> list:
        """从响应中提取 JSON 数组

        Args:
            response: 模型响应文本

        Returns:
            JSON 数组
        """
        return extract_json_array(response)
