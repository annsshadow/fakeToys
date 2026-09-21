"""请求追踪中间件

为每个请求注入/透传 X-Request-ID，便于日志与下游调用串联排障。
"""

import uuid

from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request


class RequestTraceMiddleware(BaseHTTPMiddleware):
    """请求追踪 ID 中间件"""

    HEADER = "X-Request-ID"

    async def dispatch(self, request: Request, call_next):
        """处理请求

        Args:
            request: 请求
            call_next: 下游处理函数

        Returns:
            响应对象（带 X-Request-ID 头）
        """
        incoming = request.headers.get(self.HEADER)
        request_id = incoming if incoming and incoming.strip() else uuid.uuid4().hex
        response = await call_next(request)
        response.headers[self.HEADER] = request_id
        return response


__all__ = ["RequestTraceMiddleware"]
