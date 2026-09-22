# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""限流中间件测试

覆盖滑动窗口放行/拒绝、Retry-After 计算、IP 键提取（XFF/直连）、
HTTP 429 响应与限流头。
"""

from fastapi import FastAPI, Request
from fastapi.testclient import TestClient

from api.middleware import RateLimiter, RateLimitMiddleware


def _app(limiter=None, max_requests=3, window=60.0):
    app = FastAPI()
    if limiter is None:
        app.add_middleware(RateLimitMiddleware, max_requests=max_requests,
                           window_seconds=window)
    else:
        mw = RateLimitMiddleware(app)
        mw.limiter = limiter
        # 重新构建：直接挂中间件
        app2 = FastAPI()
        app2.add_middleware(RateLimitMiddleware, max_requests=max_requests,
                            window_seconds=window)
        app2.user_middleware  # no-op

    @app.get("/hit")
    def hit(request: Request):
        return {"ok": True}

    return app


class TestRateLimiter:
    def test_allows_up_to_max(self):
        limiter = RateLimiter(max_requests=3, window_seconds=60.0)
        assert all(limiter.allow("ip", now=float(t)) for t in (0, 1, 2))
        assert limiter.allow("ip", now=3.0) is False

    def test_window_slides(self):
        limiter = RateLimiter(max_requests=1, window_seconds=10.0)
        assert limiter.allow("ip", now=0.0) is True
        assert limiter.allow("ip", now=5.0) is False
        # 10s 后窗口滑过，最早命中过期
        assert limiter.allow("ip", now=11.0) is True

    def test_retry_after_at_least_one(self):
        limiter = RateLimiter(max_requests=1, window_seconds=10.0)
        limiter.allow("ip", now=0.0)
        assert limiter.retry_after("ip", now=9.0) >= 1
        assert limiter.retry_after("empty_key", now=0.0) == 1

    def test_independent_keys(self):
        limiter = RateLimiter(max_requests=1, window_seconds=60.0)
        assert limiter.allow("a", now=0.0) is True
        assert limiter.allow("b", now=0.0) is True  # 不同键互不影响
        assert limiter.allow("a", now=1.0) is False

    def test_reset(self):
        limiter = RateLimiter(max_requests=1, window_seconds=60.0)
        limiter.allow("a")
        limiter.reset("a")
        assert limiter.allow("a") is True


class TestRateLimitMiddleware:
    def test_429_after_limit(self):
        client = TestClient(_app(max_requests=2))
        assert client.get("/hit").status_code == 200
        assert client.get("/hit").status_code == 200
        third = client.get("/hit")
        assert third.status_code == 429
        assert "Retry-After" in third.headers
        assert "限制" in third.json()["detail"] or "频繁" in third.json()["detail"]

    def test_rate_limit_headers_present(self):
        client = TestClient(_app(max_requests=5))
        response = client.get("/hit")
        assert response.headers.get("X-RateLimit-Limit") == "5"
        assert "X-RateLimit-Remaining" in response.headers

    def test_xff_client_key(self):
        client = TestClient(_app(max_requests=1))
        r1 = client.get("/hit", headers={"X-Forwarded-For": "1.1.1.1"})
        assert r1.status_code == 200
        # 同 IP 第二次被限流
        r2 = client.get("/hit", headers={"X-Forwarded-For": "1.1.1.1"})
        assert r2.status_code == 429
        # 不同 IP 不受影响
        r3 = client.get("/hit", headers={"X-Forwarded-For": "2.2.2.2"})
        assert r3.status_code == 200
