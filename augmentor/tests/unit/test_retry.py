"""retry 指数退避重试模块测试

覆盖 compute_delay 指数/封顶/抖动、with_retries 成功/重试耗尽/
不可重试异常、sleeper 注入与 on_retry 回调、RetryStats 统计。
"""

import random

import pytest

from augmentor import RetryStats, compute_delay, should_retry, with_retries


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
