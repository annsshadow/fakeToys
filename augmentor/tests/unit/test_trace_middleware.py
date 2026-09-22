# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""请求追踪中间件测试

覆盖 X-Request-ID 自动生成、入站透传、空白头回退生成。
"""

import uuid

from fastapi import FastAPI
from fastapi.testclient import TestClient

from api.middleware import RequestTraceMiddleware


def _app() -> FastAPI:
    from fastapi import Request

    app = FastAPI()
    app.add_middleware(RequestTraceMiddleware)

    @app.get("/probe")
    def probe(request: Request):
        return {"inbound": request.headers.get("X-Request-ID", "")}

    return app


class TestRequestTraceMiddleware:
    def test_generates_id_when_absent(self):
        client = TestClient(_app())
        response = client.get("/probe")
        assert response.status_code == 200
        rid = response.headers.get("X-Request-ID")
        assert rid is not None
        # 应为合法 hex uuid（32 字符）
        uuid.UUID(rid.replace("-", ""))

    def test_passes_through_incoming_id(self):
        client = TestClient(_app())
        response = client.get("/probe", headers={"X-Request-ID": "trace-abc-123"})
        assert response.headers["X-Request-ID"] == "trace-abc-123"

    def test_blank_incoming_falls_back_to_generated(self):
        client = TestClient(_app())
        response = client.get("/probe", headers={"X-Request-ID": "   "})
        rid = response.headers["X-Request-ID"]
        assert rid and rid.strip()
        assert rid != "   "

    def test_unique_across_requests(self):
        client = TestClient(_app())
        ids = {client.get("/probe").headers["X-Request-ID"] for _ in range(5)}
        assert len(ids) == 5
