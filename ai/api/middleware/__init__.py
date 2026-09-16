"""API 中间件"""

import logging
import time

from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request

logger = logging.getLogger(__name__)


class RequestLoggingMiddleware(BaseHTTPMiddleware):
    """请求日志与耗时统计中间件"""

    def __init__(self, app, slow_threshold: float = 2.0):
        """初始化中间件

        Args:
            app: ASGI 应用
            slow_threshold: 慢请求阈值（秒）
        """
        super().__init__(app)
        self.slow_threshold = slow_threshold

    async def dispatch(self, request: Request, call_next):
        """处理请求

        Args:
            request: 请求对象
            call_next: 下游处理函数

        Returns:
            响应对象
        """
        start = time.perf_counter()
        response = await call_next(request)
        elapsed = time.perf_counter() - start

        response.headers["X-Process-Time"] = f"{elapsed:.4f}"

        if elapsed >= self.slow_threshold:
            logger.warning(
                f"慢请求 {request.method} {request.url.path} 耗时 {elapsed:.3f}s"
            )

        return response


__all__ = ["RequestLoggingMiddleware"]
