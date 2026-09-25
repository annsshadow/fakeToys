# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""retry 指数退避重试模块测试

覆盖 compute_delay 指数/封顶/抖动、with_retries 成功/重试耗尽/
不可重试异常、sleeper 注入与 on_retry 回调、RetryStats 统计、
重试计数旋钮（max_retries / attempt）的越界入参。
"""

import random

import pytest

from augmentor import RetryStats, compute_delay, should_retry, with_retries
from augmentor.exceptions import DataValidationError


class TestComputeDelay:
    def test_exponential_growth(self):
        assert compute_delay(1, 1.0, 2.0, 30.0) == 1.0
        assert compute_delay(2, 1.0, 2.0, 30.0) == 2.0
        assert compute_delay(3, 1.0, 2.0, 30.0) == 4.0

    def test_capped_at_max(self):
        assert compute_delay(10, 1.0, 2.0, 30.0) == 30.0

    def test_jitter_within_bound(self):
        rng = random.Random(42)
        base = compute_delay(3, 1.0, 2.0, 30.0)
        with_jitter = compute_delay(3, 1.0, 2.0, 30.0, jitter=0.5, rng=rng)
        assert base <= with_jitter <= base * 1.5

    def test_zero_attempt_positive(self):
        assert compute_delay(1, 0.0, 2.0, 30.0) == 0.0


class TestWithRetries:
    def test_success_first_try(self):
        calls = {"n": 0}

        def ok():
            calls["n"] += 1
            return 42

        result, stats = with_retries(ok, max_retries=3, sleeper=lambda d: None)
        assert result == 42
        assert stats.succeeded is True
        assert stats.attempts == 1
        assert calls["n"] == 1

    def test_retries_then_success(self):
        state = {"n": 0}
        delays = []

        def flaky():
            state["n"] += 1
            if state["n"] < 3:
                raise ConnectionError("暂不可用")
            return "ok"

        result, stats = with_retries(
            flaky, max_retries=3,
            base_delay=1.0, factor=2.0,
            sleeper=lambda d: delays.append(d),
        )
        assert result == "ok"
        assert stats.attempts == 3
        assert delays == [1.0, 2.0]
        assert stats.total_wait == pytest.approx(3.0)

    def test_exhausted_raises_last(self):
        attempts = {"n": 0}

        def always_fail():
            attempts["n"] += 1
            raise TimeoutError("超时")

        with pytest.raises(TimeoutError):
            with_retries(always_fail, max_retries=2, sleeper=lambda d: None)
        assert attempts["n"] == 3  # 首次 + 2 次重试

    def test_non_retryable_raises_immediately(self):
        calls = {"n": 0}

        def bad():
            calls["n"] += 1
            raise ValueError("参数错误")

        with pytest.raises(ValueError):
            with_retries(bad, max_retries=3, retry_on=(ConnectionError,),
                        sleeper=lambda d: None)
        assert calls["n"] == 1  # 不可重试，不重试

    def test_on_retry_callback_invoked(self):
        state = {"n": 0}
        events = []

        def flaky():
            state["n"] += 1
            if state["n"] < 2:
                raise ConnectionError("x")
            return "done"

        def record(attempt, exc, delay):
            events.append((attempt, type(exc).__name__, delay))

        with_retries(
            flaky, max_retries=2, base_delay=0.5,
            on_retry=record, sleeper=lambda d: None,
        )
        assert len(events) == 1
        assert events[0][0] == 1
        assert events[0][1] == "ConnectionError"

    def test_args_and_kwargs_passed(self):
        def add(a, b, c=0):
            return a + b + c

        result, _ = with_retries(add, 1, 2, c=3, max_retries=0, sleeper=lambda d: None)
        assert result == 6


class TestRetryStats:
    def test_to_dict_and_total_wait(self):
        stats = RetryStats(attempts=3, succeeded=True, delays=[1.0, 2.0])
        d = stats.to_dict()
        assert d["attempts"] == 3
        assert d["total_wait"] == pytest.approx(3.0)


class TestShouldRetry:
    def test_matching_type(self):
        assert should_retry(ConnectionError("x"), (ConnectionError,)) is True

    def test_non_matching(self):
        assert should_retry(ValueError("x"), (ConnectionError,)) is False

    def test_subclass_match(self):
        class _Sub(ConnectionError):
            pass

        assert should_retry(_Sub("x"), (ConnectionError,)) is True


class _FakeResponse:
    """模拟 requests 响应对象（只需 status_code / headers）"""

    def __init__(self, status_code, headers=None):
        self.status_code = status_code
        self.headers = headers or {}


class _FakeHTTPError(Exception):
    """模拟 requests.exceptions.HTTPError：状态码挂在 .response 上"""

    def __init__(self, status_code, headers=None):
        super().__init__(f"HTTP {status_code}")
        self.response = _FakeResponse(status_code, headers)


class TestParseRetryAfter:
    """Retry-After 解析"""

    def test_seconds(self):
        from augmentor import parse_retry_after

        assert parse_retry_after("120") == 120.0
        assert parse_retry_after("0") == 0.0
        assert parse_retry_after(" 2.5 ") == 2.5

    def test_negative_clamped_to_zero(self):
        from augmentor import parse_retry_after

        assert parse_retry_after("-5") == 0.0

    def test_http_date(self):
        """HTTP-date 形式应换算为剩余秒数"""
        from email.utils import format_datetime
        import datetime

        from augmentor import parse_retry_after

        future = datetime.datetime.now(datetime.timezone.utc) + datetime.timedelta(seconds=90)
        seconds = parse_retry_after(format_datetime(future))
        assert seconds is not None
        assert 80 <= seconds <= 95

    def test_unparseable_returns_none(self):
        from augmentor import parse_retry_after

        assert parse_retry_after(None) is None
        assert parse_retry_after("") is None
        assert parse_retry_after("不是时间") is None


class TestClassifyError:
    """错误分类：区分「值得重试」与「重试纯属浪费」"""

    def test_rate_limit_is_retryable_and_honors_retry_after(self):
        from augmentor import classify_error

        retryable, suggested = classify_error(_FakeHTTPError(429, {"Retry-After": "7"}))
        assert retryable is True
        assert suggested == 7.0

    def test_server_error_is_retryable(self):
        from augmentor import classify_error

        assert classify_error(_FakeHTTPError(503)) == (True, None)

    def test_auth_error_is_not_retryable(self):
        """401/403 重试只会浪费配额与时间"""
        from augmentor import classify_error

        assert classify_error(_FakeHTTPError(401))[0] is False
        assert classify_error(_FakeHTTPError(403))[0] is False

    def test_bad_request_is_not_retryable(self):
        from augmentor import classify_error

        assert classify_error(_FakeHTTPError(400))[0] is False
        assert classify_error(_FakeHTTPError(404))[0] is False
        assert classify_error(_FakeHTTPError(422))[0] is False

    def test_unknown_error_defaults_to_retryable(self):
        """无法判定状态码时保持既有宽松行为"""
        from augmentor import classify_error

        assert classify_error(RuntimeError("boom")) == (True, None)

    def test_connection_error_is_retryable(self):
        import requests

        from augmentor import classify_error

        assert classify_error(requests.exceptions.ConnectionError("x"))[0] is True
        assert classify_error(requests.exceptions.Timeout("x"))[0] is True


class TestWithRetriesClassify:
    """with_retries 的错误分类接入"""

    def test_non_retryable_stops_immediately(self):
        """被判为不可重试时应立即放弃：只调用一次且不 sleep"""
        from augmentor import classify_error

        calls = []
        slept = []

        def always_401():
            calls.append(1)
            raise _FakeHTTPError(401)

        with pytest.raises(_FakeHTTPError):
            with_retries(
                always_401,
                max_retries=5,
                classify=classify_error,
                sleeper=slept.append,
            )

        assert len(calls) == 1, "不可重试的错误不应重试"
        assert slept == [], "不可重试的错误不应等待"

    def test_retry_after_overrides_backoff(self):
        """有 Retry-After 时应优先采用，而不是指数退避值"""
        from augmentor import classify_error

        slept = []
        calls = []

        def flaky():
            calls.append(1)
            if len(calls) == 1:
                raise _FakeHTTPError(429, {"Retry-After": "3"})
            return "ok"

        result, stats = with_retries(
            flaky,
            max_retries=2,
            base_delay=100.0,  # 若走了退避计算，等待会是 100s
            classify=classify_error,
            sleeper=slept.append,
        )

        assert result == "ok"
        assert slept == [3.0]

    def test_retry_after_is_capped(self):
        """异常大的 Retry-After 应被上限截断，避免流水线长时间挂起"""
        from augmentor import MAX_RETRY_AFTER, classify_error

        slept = []
        calls = []

        def flaky():
            calls.append(1)
            if len(calls) == 1:
                raise _FakeHTTPError(429, {"Retry-After": "99999"})
            return "ok"

        with_retries(
            flaky,
            max_retries=2,
            classify=classify_error,
            sleeper=slept.append,
        )

        assert slept == [MAX_RETRY_AFTER]

    def test_classify_absent_keeps_legacy_behavior(self):
        """不传 classify 时行为与旧版一致（任何异常都重试）"""
        calls = []

        def always_fail():
            calls.append(1)
            raise RuntimeError("x")

        with pytest.raises(RuntimeError):
            with_retries(always_fail, max_retries=2, sleeper=lambda d: None)

        assert len(calls) == 3


class TestRetryCountKnobs:
    """重试计数旋钮（max_retries / attempt）的越界读法

    这两个旋钮都只有一种合法读法，而越界值在原实现里不是报错就是换语义：
    `max_retries=-1` 让 `range(0, 0)` 为空 → 一次都不调用 func，最后
    `raise last_exc` 变成 `raise None`，抛出与参数错误毫无关系的 TypeError；
    `compute_delay(attempt=0)` 把 `attempt - 1` 当指数算出 2 的负次幂，
    于是「第 0 次重试」比第 1 次等得更短。两者都当场判掉。
    """

    def test_negative_max_retries_fails_before_calling_func(self):
        """越界必须在调用 func 之前报出来，而不是让请求凭空消失"""
        calls = []

        def never_run():
            calls.append(1)
            return "ok"

        with pytest.raises(DataValidationError, match="max_retries"):
            with_retries(never_run, max_retries=-1, sleeper=lambda d: None)
        assert calls == []

    def test_zero_max_retries_is_a_legal_request(self):
        """0 =「只调用一次，失败不再试」，必须与「没传参数」（默认 3 次）区分开"""
        calls = []

        def fail_once():
            calls.append(1)
            raise RuntimeError("boom")

        with pytest.raises(RuntimeError):
            with_retries(fail_once, max_retries=0, sleeper=lambda d: None)
        assert calls == [1]

    def test_non_integer_max_retries_rejected(self):
        """bool 与数字字符串都不是次数：True 会被 range 静默读成「重试 1 次」"""
        with pytest.raises(DataValidationError, match="max_retries"):
            with_retries(lambda: "ok", max_retries=True, sleeper=lambda d: None)
        with pytest.raises(DataValidationError, match="max_retries"):
            with_retries(lambda: "ok", max_retries="2", sleeper=lambda d: None)

    @pytest.mark.parametrize("attempt,expected", [(1, 1.0), (2, 2.0), (3, 4.0)])
    def test_legal_attempts_keep_existing_backoff(self, attempt, expected):
        """收紧入参不得改动合法档位的退避数值"""
        assert compute_delay(attempt, 1.0, 2.0, 30.0) == expected

    def test_attempt_below_one_is_rejected(self):
        """attempt 是「已失败次数」，从 1 起算；0/负数没有对应的现实"""
        with pytest.raises(DataValidationError, match="attempt"):
            compute_delay(0, 1.0, 2.0, 30.0)
        with pytest.raises(DataValidationError, match="attempt"):
            compute_delay(-3, 1.0, 2.0, 30.0)
