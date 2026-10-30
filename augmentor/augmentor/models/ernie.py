# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""百度 ERNIE 模型后端 - 优化版"""

import asyncio
import time
import logging
import threading
from typing import Any, Optional

from .base import ModelBackend, extract_json_array
from ..config import ModelConfig
from ..exceptions import ModelGenerateError, ModelNotConfiguredError

logger = logging.getLogger(__name__)


class ERNIEBackend(ModelBackend):
    """百度 ERNIE 模型后端 - 优化版"""
    
    # API 端点模板
    TOKEN_URL = "https://aip.baidubce.com/oauth/2.0/token"
    CHAT_URL = "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/completions"

    # 换 access_token 那一支的超时（秒）。**故意不吃 `augmentation.request_timeout`
    # 与模型条目那档**（A74 明确要求单独拍）：token 换取是一次普通的鉴权往返，
    # 与「模型可能很慢」无关，跟着推理档走会把它从 10 s 放大到 120 s —— 于是凭据
    # 错了也要干等两分钟才出声。推理请求的超时走 `self._request_timeout`。
    TOKEN_REQUEST_TIMEOUT = 10.0
    
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
        """初始化 ERNIE 后端

        Args:
            config: 模型配置，必须包含 api_key 和 secret_key
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
        if not config.api_key or not config.secret_key:
            raise ModelNotConfiguredError("ERNIE 后端需要 api_key 和 secret_key")
        self._access_token = None
        self._token_expires_at = 0
        self._token_lock = threading.Lock()
        # 异步客户端与 token 锁（同其他后端；token 缓存 30 天，锁只防首次并发刷新）。
        self._async_client: Optional[Any] = None
        self._async_loop: Optional[Any] = None
        self._async_token_lock: Optional[asyncio.Lock] = None
    
    def _get_access_token(self) -> str:
        """获取访问令牌（带过期检测）
        
        Returns:
            访问令牌
        """
        # 检查 token 是否有效（提前 5 分钟刷新）
        if self._access_token and time.time() < self._token_expires_at - 300:
            return self._access_token
        
        with self._token_lock:
            # 双重检查
            if self._access_token and time.time() < self._token_expires_at - 300:
                return self._access_token
            
            params = {
                "grant_type": "client_credentials",
                "client_id": self.config.api_key,
                "client_secret": self.config.secret_key
            }
            
            session = self._get_session()
            response = session.post(self.TOKEN_URL, params=params,
                                    timeout=self.TOKEN_REQUEST_TIMEOUT)
            response.raise_for_status()
            
            data = response.json()
            self._access_token = data.get("access_token")
            expires_in = data.get("expires_in", 2592000)  # 默认 30 天
            self._token_expires_at = time.time() + expires_in
            
            if not self._access_token:
                raise ModelGenerateError(f"获取访问令牌失败: {data}")
            
            logger.info(f"获取访问令牌成功，有效期 {expires_in} 秒")
            return self._access_token
    
    def _build_request(self, prompt: str) -> tuple:
        """构造 (headers, payload)。同步与异步两条路径用同一份请求体。"""
        headers = {"Content-Type": "application/json"}
        payload = {
            "temperature": self.config.temperature,
            "top_p": self.config.top_p,
            "max_output_tokens": self.config.max_output_tokens,
            "messages": [{"role": "user", "content": prompt}],
        }
        return headers, payload

    @staticmethod
    def _parse_response(data: Any) -> str:
        """从响应体取 result。同步/异步共用。"""
        if data.get("is_truncated"):
            raise ModelGenerateError("响应被截断")
        return data.get("result", "")

    def _call_api(self, prompt: str) -> str:
        """调用 ERNIE API（使用连接池）

        Args:
            prompt: 输入提示

        Returns:
            模型生成的文本
        """
        access_token = self._get_access_token()
        url = f"{self.CHAT_URL}?access_token={access_token}"
        headers, payload = self._build_request(prompt)

        session = self._get_session()
        response = session.post(url, json=payload, headers=headers,
                                timeout=self._request_timeout)
        response.raise_for_status()

        return self._parse_response(response.json())

    async def _aget_access_token(self, httpx_module: Any) -> str:
        """异步获取访问令牌（带过期检测；asyncio.Lock 防并发刷新风暴）。"""
        if self._access_token and time.time() < self._token_expires_at - 300:
            return self._access_token

        if self._async_token_lock is None:
            self._async_token_lock = asyncio.Lock()
        async with self._async_token_lock:
            if self._access_token and time.time() < self._token_expires_at - 300:
                return self._access_token

            client = await self._async_client_for(httpx_module)
            response = await client.post(
                self.TOKEN_URL,
                params={
                    "grant_type": "client_credentials",
                    "client_id": self.config.api_key,
                    "client_secret": self.config.secret_key,
                },
            )
            response.raise_for_status()
            data = response.json()
            token = data.get("access_token")
            expires_in = data.get("expires_in", 2592000)
            if not token:
                raise ModelGenerateError(f"获取访问令牌失败: {data}")
            self._access_token = token
            self._token_expires_at = time.time() + expires_in
            logger.info("获取访问令牌成功（异步），有效期 %s 秒", expires_in)
            return token

    async def _acall_api(self, prompt: str) -> str:
        """原生异步调用 ERNIE API（httpx.AsyncClient）。

        同其他后端：httpx 可选，缺它回退基类线程池实现；AsyncClient 跟随事件循环。
        """
        try:
            import httpx
        except ImportError:
            logger.warning(
                "httpx 不可用，异步调用回退为线程池（功能不受影响，但不省线程）"
            )
            return await super()._acall_api(prompt)

        access_token = await self._aget_access_token(httpx)
        url = f"{self.CHAT_URL}?access_token={access_token}"
        headers, payload = self._build_request(prompt)
        client = await self._async_client_for(httpx)
        response = await client.post(url, json=payload, headers=headers)
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

        复用基类唯一的 robust 解析器。改前本方法只做了括号切片那一半、漏了「先直接
        解析」那一半（A111，同族第三份拷贝）；接上 `extract_json_array` 后与其余后端
        口径一致。

        Args:
            response: 模型响应文本

        Returns:
            JSON 数组
        """
        return extract_json_array(response)
