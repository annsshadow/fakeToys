# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Ollama 本地模型后端 - 优化版"""

import asyncio
import logging
from typing import Any, Optional

from .base import ModelBackend, extract_json_array
from ..config import ModelConfig
from ..exceptions import ModelGenerateError, ModelNotConfiguredError

logger = logging.getLogger(__name__)


class OllamaBackend(ModelBackend):
    """Ollama 本地模型后端 - 优化版"""
    
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
        """初始化 Ollama 后端

        Args:
            config: 模型配置，必须包含 base_url
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
        if not config.base_url:
            raise ModelNotConfiguredError("Ollama 后端需要 base_url")

        self.api_url = f"{config.base_url.rstrip('/')}/api/chat"
        # 异步客户端与它所属的事件循环（同 OpenAIBackend）：httpx.AsyncClient
        # 绑定创建它的循环，跨循环复用会抛「Event loop is closed」或静默挂死。
        self._async_client: Optional[Any] = None
        self._async_loop: Optional[Any] = None

    def _build_request(self, prompt: str) -> tuple:
        """构造 (headers, payload)。同步与异步两条路径用同一份请求体，
        避免「切到异步后模型行为变了」只能靠肉眼比对两段代码。"""
        headers = {"Content-Type": "application/json"}
        payload = {
            "model": self.config.model,
            "messages": [{"role": "user", "content": prompt}],
            "stream": False,
            "options": {
                "temperature": self.config.temperature,
                "top_p": self.config.top_p,
                "num_predict": self.config.max_output_tokens,
            },
        }
        return headers, payload
    
    def _call_api(self, prompt: str) -> str:
        """调用 Ollama API（使用连接池）

        Args:
            prompt: 输入提示

        Returns:
            模型生成的文本
        """
        headers, payload = self._build_request(prompt)
        session = self._get_session()
        response = session.post(self.api_url, json=payload, headers=headers,
                                timeout=self._request_timeout)
        response.raise_for_status()

        data = response.json()

        if "message" not in data:
            raise ModelGenerateError(f"API 响应异常: {data}")

        return data["message"]["content"]

    async def _acall_api(self, prompt: str) -> str:
        """原生异步调用 Ollama API（httpx.AsyncClient）。

        与 OpenAIBackend 同款设计：httpx 是可选依赖，缺它回退基类线程池实现
        （功能不受影响，只是等待时仍占线程）；AsyncClient 跟随事件循环，
        换循环则关闭重建，连接池复用在同一循环内仍然有效。
        """
        try:
            import httpx
        except ImportError:
            logger.warning(
                "httpx 不可用，异步调用回退为线程池（功能不受影响，但不省线程）"
            )
            return await super()._acall_api(prompt)

        headers, payload = self._build_request(prompt)
        client = await self._async_client_for(httpx)
        response = await client.post(self.api_url, json=payload, headers=headers)
        response.raise_for_status()

        data = response.json()
        if "message" not in data:
            raise ModelGenerateError(f"API 响应异常: {data}")
        return data["message"]["content"]

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

        复用基类唯一的 robust 解析器，删掉本方法此前逐字复制的同一段直接解析 +
        括号切片逻辑（与 `extract_json_array` 一字不差 ⇒ 纯重复，A111）。

        Args:
            response: 模型响应文本

        Returns:
            JSON 数组
        """
        return extract_json_array(response)
