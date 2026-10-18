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

    def test_expired_hits_inside_a_live_key_stop_counting(self):
        """`max_requests=1` 那一只看不到 `:91-92` 的 popleft：整键过期先被清扫接管

        上一臂里 `now=11.0` 那一次走的是 `_sweep_locked` 把**整个键**删掉的路（命中只有
        一格，键的 newest 就是它自己），于是「键内还有更老的过期命中要摘」那圈 popleft
        恒不执行 ⇒ L279 前的覆盖率读数点名 `api/middleware/rate_limit.py:92`。
        这一臂把窗口灌成 `[0, 5, 9]`（`max_requests=3`）再打到 `now=11.0`：清扫判键不老
        （`11 - 9 = 2 ≤ 10`）⇒ 键留下，`0.0` 那一格由 popleft 摘掉 ⇒ 第三格腾出来，放行。
        判据带牙齿：摘掉 popleft，这次请求数到三只命中 ⇒ 回 False（429）。
        """
        limiter = RateLimiter(max_requests=3, window_seconds=10.0)
        assert [limiter.allow("ip", now=t) for t in (0.0, 5.0, 9.0)] == [True, True, True]
        assert limiter.allow("ip", now=11.0) is True, "过期命中还在计数 ⇒ 窗口没有真的滑动"
        assert list(limiter._hits["ip"]) == [5.0, 9.0, 11.0], "摘错格子（该摘最旧那只）"
        assert limiter.allow("ip", now=11.5) is False, "窗口里三只都在数 ⇒ 这一格必须拒"
        assert limiter.retry_after("ip", now=11.5) == 4, "Retry-After 该按最旧那只（5.0）算"

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

    def test_expired_keys_are_swept(self):
        """过期键要被回收：从前 `defaultdict` 让每个新键永久占一格（实测窗口后仍存
        100 001 个键），于是「绕过限流」这个动作本身就能打爆内存"""
        limiter = RateLimiter(max_requests=2, window_seconds=60.0)
        for i in range(200):
            limiter.allow(f"k{i}", now=0.0)
        assert len(limiter._hits) == 200
        # 下一个窗口之后：所有键都已过期 ⇒ 清扫把它们全丢掉
        assert limiter.allow("k0", now=120.0) is True
        assert len(limiter._hits) <= 2

    def test_max_keys_caps_the_dict(self):
        """清扫后仍然太多（真实高并发）时按最近命中丢最旧的键，键数封顶"""
        limiter = RateLimiter(max_requests=5, window_seconds=60.0, max_keys=10)
        for i in range(50):
            limiter.allow(f"k{i}", now=float(i))
        assert len(limiter._hits) <= 10

    def test_retry_after_does_not_create_keys(self):
        limiter = RateLimiter(max_requests=1, window_seconds=60.0)
        limiter.retry_after("nobody")
        assert "nobody" not in limiter._hits

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

    def test_forwarded_header_cannot_buy_a_new_bucket(self):
        """`X-Forwarded-For` 是客户端自报的，换值不许买到一个新计数桶

        改前这一格钉的是「按 XFF 分桶」，等于把绕过路径写成契约（实测
        `max_requests=2` 时同一台机器换 50 个伪造值全部 200、零个 429）。
        反代部署改用 uvicorn `--proxy-headers`，那一条会把 `scope["client"]`
        改写成真实来源，本闸读的就是可信值。
        """
        client = TestClient(_app(max_requests=1))
        assert client.get("/hit").status_code == 200
        spoofed = client.get("/hit", headers={"X-Forwarded-For": "2.2.2.2"})
        assert spoofed.status_code == 429
