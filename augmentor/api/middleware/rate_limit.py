# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""API 限流中间件（滑动窗口，按客户端 IP）

防止滥用与雪崩。基于进程内滑动窗口计数，超限返回 429 与
Retry-After 头。测试可注入时钟与存储实现确定性。

两条口径是本文件的全部取向，都记了实测：

1. **客户端标识只取连接的对端地址，不读 `X-Forwarded-For`**。那条头是客户端自报的：
   换一个新值就买到一个全新的计数桶（实测 `max_requests=2` 时同一台机器用伪造头
   发 50 个请求，全部 200、零个 429）。服务跑在反代后面时用 ASGI 服务器自己那一层
   （uvicorn `--proxy-headers --forwarded-allow-ips=...`），它会把 `scope["client"]`
   改写成真实来源，本闸读到的就是可信值，不需要再信一遍头。
2. **键空间有封顶、过期键会被清扫**。从前 `defaultdict` 只按单键修剪时间戳、键本身
   永不回收（实测一个窗口之后仍存着 100 001 个键），于是「绕过限流」这个动作本身
   就能打爆服务端内存。

`_hits` 因此是普通 dict + 每窗口至多一次清扫；达到 `max_keys` 时按最近命中时间
丢最旧的键（宁可让一个长期闲置的桶重来，也不让字典无界长大）。
"""

import threading
import time
from collections import deque
from typing import Deque, Dict

from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse, Response


class RateLimiter:
    """滑动窗口限流器"""

    #: 键空间上限。默认 10 000 相对 `max_requests=300` 是 300 万个请求的账，
    #: 正常使用远碰不到；它挡的是「伪造源疯狂建键」这一族。
    DEFAULT_MAX_KEYS = 10000

    def __init__(self, max_requests: int, window_seconds: float,
                 max_keys: int = None):
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self.max_keys = max_keys if max_keys is not None else self.DEFAULT_MAX_KEYS
        self._hits: Dict[str, Deque[float]] = {}
        self._last_sweep = 0.0
        self._lock = threading.Lock()

    def _sweep_locked(self, now: float, keep: str = None) -> None:
        """丢掉「窗口里没有任何仍在计数的命中」的键，并把键数压回上限（调用方持锁）

        `keep` 是调用方正在服务的那个键：新建的桶此刻还是空窗，按「最旧」排序会被
        自己刚建的键顶替掉，那一支的计数就此丢失（等于对那个键永久放行），所以它
        不参与清扫。
        """
        stale = [key for key, window in self._hits.items()
                 if key != keep and (not window or now - window[-1] > self.window_seconds)]
        for key in stale:
            del self._hits[key]
        overflow = len(self._hits) - self.max_keys
        if overflow > 0:
            counted = [(key, window) for key, window in self._hits.items() if window]
            oldest = sorted(counted, key=lambda kv: kv[1][-1])[:overflow]
            for key, _ in oldest:
                del self._hits[key]

    def allow(self, key: str, now: float = None) -> bool:
        """判断某键当前是否放行

        Args:
            key: 客户端标识（连接对端地址）
            now: 当前时间戳（可注入）

        Returns:
            是否放行
        """
        now = now if now is not None else time.time()
        with self._lock:
            if now - self._last_sweep > self.window_seconds:
                self._last_sweep = now
                self._sweep_locked(now)
            window = self._hits.get(key)
            if window is None:
                window = deque()
                self._hits[key] = window
                # 新键把总量顶过上限时当场压回来：只按时间扫会留一整个窗口的空档，
                # 而那段空档正好够「疯狂换源」把字典撑大
                if len(self._hits) > self.max_keys:
                    self._sweep_locked(now, keep=key)
            while window and now - window[0] > self.window_seconds:
                window.popleft()
            if len(window) >= self.max_requests:
                return False
            window.append(now)
            return True

    def retry_after(self, key: str, now: float = None) -> int:
        """计算建议的 Retry-After 秒数（窗口内最早命中过期时间）

        Args:
            key: 客户端标识
            now: 当前时间戳（可注入）

        Returns:
            建议等待秒数（至少 1）
        """
        now = now if now is not None else time.time()
        with self._lock:
            # get 而不是 `self._hits[key]`：默认工厂在这里会凭空建一个键，
            # 于是「只查不买」的读侧也在长大键空间
            window = self._hits.get(key)
            if not window:
                return 1
            oldest = window[0]
            return max(1, int(self.window_seconds - (now - oldest)) + 1)

    def reset(self, key: str = None):
        """重置限流计数（测试用）

        Args:
            key: 指定键或 None 表示全部
        """
        with self._lock:
            if key is None:
                self._hits.clear()
            else:
                self._hits.pop(key, None)


class RateLimitMiddleware(BaseHTTPMiddleware):
    """基于滑动窗口的请求限流中间件"""

    HEADER_LIMIT = "X-RateLimit-Limit"
    HEADER_REMAINING = "X-RateLimit-Remaining"

    def __init__(self, app, max_requests: int = 300, window_seconds: float = 60.0,
                 limiter: RateLimiter = None, exempt_paths=None):
        """初始化中间件

        Args:
            app: ASGI 应用
            max_requests: 窗口内最大请求数；<= 0 表示关闭限流
            window_seconds: 窗口长度（秒）
            limiter: 复用外部限流器实例（便于测试注入与逐用例重置）；
                为 None 时按 max_requests / window_seconds 新建
            exempt_paths: 免限流路径前缀列表
        """
        super().__init__(app)
        self.limiter = limiter if limiter is not None else RateLimiter(
            max_requests, window_seconds
        )
        self.exempt_paths = list(exempt_paths or ())
        self.enabled = self.limiter.max_requests > 0

    def _is_exempt(self, path: str) -> bool:
        """判断路径是否免限流

        Args:
            path: 请求路径

        Returns:
            是否豁免
        """
        return any(path.startswith(prefix) for prefix in self.exempt_paths)

    def _client_key(self, request: Request) -> str:
        # 不读 `X-Forwarded-For`（理由与实测见模块 docstring）。拿不到对端地址时
        # 也不能退回「按头猜」—— 那是同一个洞的另一扇门，所以折成一个共享桶。
        return request.client.host if request.client else "unknown"

    async def dispatch(self, request: Request, call_next) -> Response:
        if not self.enabled or self._is_exempt(request.url.path):
            return await call_next(request)

        key = self._client_key(request)
        now = time.time()
        if not self.limiter.allow(key, now):
            retry_after = self.limiter.retry_after(key, now)
            response = JSONResponse(
                status_code=429,
                content={"detail": "请求过于频繁，请稍后再试"},
            )
            response.headers["Retry-After"] = str(retry_after)
            return response

        response = await call_next(request)
        response.headers[self.HEADER_LIMIT] = str(self.limiter.max_requests)
        with self.limiter._lock:
            window = self.limiter._hits.get(key)
            used = len(window) if window else 0
        remaining = max(0, self.limiter.max_requests - used)
        response.headers[self.HEADER_REMAINING] = str(remaining)
        return response


__all__ = ["RateLimiter", "RateLimitMiddleware"]
