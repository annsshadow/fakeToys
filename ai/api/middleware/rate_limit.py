"""API 限流中间件（滑动窗口，按客户端 IP）

防止滥用与雪崩。基于进程内滑动窗口计数，超限返回 429 与
Retry-After 头。测试可注入时钟与存储实现确定性。
"""

import threading
import time
from collections import defaultdict, deque
from typing import Deque, Dict

from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import JSONResponse, Response


class RateLimiter:
    """滑动窗口限流器"""

    def __init__(self, max_requests: int, window_seconds: float):
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self._hits: Dict[str, Deque[float]] = defaultdict(deque)
        self._lock = threading.Lock()

    def allow(self, key: str, now: float = None) -> bool:
        """判断某键当前是否放行

        Args:
            key: 客户端标识（如 IP）
            now: 当前时间戳（可注入）

        Returns:
            是否放行
        """
        now = now if now is not None else time.time()
        with self._lock:
            window = self._hits[key]
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
            window = self._hits[key]
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

    def __init__(self, app, max_requests: int = 100, window_seconds: float = 60.0):
        """初始化中间件

        Args:
            app: ASGI 应用
            max_requests: 窗口内最大请求数
            window_seconds: 窗口长度（秒）
        """
        super().__init__(app)
        self.limiter = RateLimiter(max_requests, window_seconds)

    def _client_key(self, request: Request) -> str:
        forwarded = request.headers.get("X-Forwarded-For")
        if forwarded:
            return forwarded.split(",")[0].strip()
        return request.client.host if request.client else "unknown"

    async def dispatch(self, request: Request, call_next) -> Response:
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
        remaining = max(0, self.limiter.max_requests - len(self.limiter._hits.get(key, ())))
        response.headers[self.HEADER_REMAINING] = str(remaining)
        return response


__all__ = ["RateLimiter", "RateLimitMiddleware"]
