# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""OpenAI 模型后端 - 优化版"""

import asyncio
import logging
from typing import Any, Optional

from .base import ModelBackend, extract_json_array
from ..config import ModelConfig
from ..exceptions import ModelGenerateError, ModelNotConfiguredError

logger = logging.getLogger(__name__)


class OpenAIBackend(ModelBackend):
    """OpenAI 模型后端 - 优化版"""
    
    API_URL = "https://api.openai.com/v1/chat/completions"
    
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
        """初始化 OpenAI 后端

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
            raise ModelNotConfiguredError("OpenAI 后端需要 api_key")
        # 异步客户端与它所属的事件循环。**必须记 loop**：httpx 的 AsyncClient
        # 绑定创建它的循环，跨循环复用会抛「Event loop is closed」或静默挂死
        # （测试里 `asyncio.run` 多次就是这个形状）。
        self._async_client: Optional[Any] = None
        self._async_loop: Optional[Any] = None
    
    def _build_request(self, prompt: str) -> tuple:
        """构造 (headers, payload)。同步与异步两条路径**必须**用同一份请求体 ——
        否则"切到异步后模型行为变了"这种问题只能靠肉眼比对两段代码。"""
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
        return headers, payload

    @staticmethod
    def _parse_response(data: Any) -> str:
        """从响应体取首个 choice 的正文。同步/异步共用，避免两处各判一次空。"""
        if "choices" not in data or len(data["choices"]) == 0:
            raise ModelGenerateError(f"API 响应异常: {data}")
        return data["choices"][0]["message"]["content"]

    def _call_api(self, prompt: str) -> str:
        """调用 OpenAI API（使用连接池）
        
        Args:
            prompt: 输入提示
        
        Returns:
            模型生成的文本
        """
        headers, payload = self._build_request(prompt)

        session = self._get_session()
        response = session.post(self.API_URL, json=payload, headers=headers,
                                timeout=self._request_timeout)
        response.raise_for_status()

        return self._parse_response(response.json())

    async def _acall_api(self, prompt: str) -> str:
        """原生异步调用（httpx.AsyncClient）。

        # 为什么 httpx 是**可选**依赖

        `httpx` 目前只声明在 `requirements-dev.txt`（测试链用），生产环境可能
        没有。缺它就回退到基类的线程池实现：功能不受影响，只是等待期间仍占线程。
        这样「异步接口」不会因为一个可选依赖缺失而变成 ImportError。

        # 为什么客户端要跟着事件循环走

        `httpx.AsyncClient` 绑定创建它的循环。跨循环复用（测试里连续
        `asyncio.run`）会报 "Event loop is closed"。这里记下 loop，换了就关掉旧的
        重开 —— 连接池复用在**同一**循环内仍然有效，而那才是生产的形状。
        """
        try:
            import httpx
        except ImportError:
            logger.warning("httpx 不可用，异步调用回退为线程池（功能不受影响，但不省线程）")
            return await super()._acall_api(prompt)

        headers, payload = self._build_request(prompt)
        client = await self._async_client_for(httpx)
        response = await client.post(self.API_URL, json=payload, headers=headers)
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
            except Exception:  # 旧循环可能已关闭，关不掉不影响新客户端
                logger.debug("关闭上一个异步客户端失败（循环可能已关闭）", exc_info=True)
        self._async_client = httpx_module.AsyncClient(timeout=self._request_timeout)
        self._async_loop = loop
        return self._async_client

    async def aclose(self) -> None:
        """关闭异步客户端。与基类 `close()` 同族，只是必须 await。"""
        client, self._async_client = self._async_client, None
        self._async_loop = None
        if client is not None:
            await client.aclose()
    
    def extract_json_from_response(self, response: str) -> list:
        """从响应中提取 JSON 数组

        复用基类唯一的 robust 解析器（直接数组 / ```json 围栏 / 前置说明散文都能
        取，且顶层是对象时抛错而非把 dict 当数组返回）。改前本方法手搓了一份，且
        少了 `isinstance(result, list)` 守卫 ⇒ 纯 JSON 对象响应会违反 `-> list`
        契约返回 dict（A111，与 L59 A108 / L60 A110 同族）。

        Args:
            response: 模型响应文本

        Returns:
            JSON 数组
        """
        return extract_json_array(response)
