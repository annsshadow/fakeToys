# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""retry 指数退避重试模块测试

覆盖 compute_delay 指数/封顶/抖动、with_retries 成功/重试耗尽/
不可重试异常、sleeper 注入与 on_retry 回调、RetryStats 统计、
重试计数旋钮（max_retries / attempt）与退避浮点旋钮（base_delay /
factor / max_delay / jitter）的越界入参、两副等待封顶各管一支的口径（A72）、
`jitter` / `rng` 从 `with_retries` 到 `compute_delay` 的透传与「抖动只加在退避一支」
的边界（A65）。
"""

import random

import pytest

from augmentor import (MAX_RETRY_AFTER, RetryStats, compute_delay,
                       should_retry, with_retries)
from augmentor.exceptions import DataValidationError


def _always_fail():
    """恒定失败的目标：只关心「每次重试前睡了多久」"""
    raise RuntimeError("boom")


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


class TestRetryFloatKnobs:
    """退避浮点旋钮（base_delay / factor / max_delay）在 `with_retries` 处的判据

    与 `TestRetryCountKnobs` 成对：那一条管「重试几次」，这一条管「每档等多久」。
    修之前实测的三个症状（Temp `l47_probe.py`，`max_retries` 取 2 或 3）：
    `base_delay='1'` → 第一次真实调用照常打出去，退避时才炸
    `TypeError: can't multiply sequence by non-int of type 'float'`，原始异常丢失；
    `base_delay=True` → 静默读成 1.0，sleeper 收到 `[1.0, 2.0]`；
    `max_delay=-1` → 整条退避被 `min()` 加尾部 `max(0.0, …)` 收成 `[0.0, 0.0]`。
    """

    @pytest.mark.parametrize(
        "kwargs,name",
        [
            ({"base_delay": "1"}, "base_delay"),
            ({"base_delay": -5.0}, "base_delay"),
            ({"base_delay": True}, "base_delay"),
            ({"base_delay": float("nan")}, "base_delay"),
            ({"factor": 0.0}, "factor"),
            ({"factor": -2.0}, "factor"),
            ({"factor": float("nan")}, "factor"),
            ({"max_delay": -1.0}, "max_delay"),
            ({"max_delay": float("nan")}, "max_delay"),
        ],
    )
    def test_bad_knob_fails_before_the_first_request(self, kwargs, name):
        """坏档位不该先换来一次真金白银的请求、再从循环里抛出无关的 TypeError"""
        calls = []

        def never_run():
            calls.append(1)
            return "ok"

        with pytest.raises(DataValidationError, match=name):
            with_retries(never_run, max_retries=2, sleeper=lambda d: None, **kwargs)
        assert calls == []

    def test_zero_and_infinite_delays_stay_legal(self):
        """`base_delay=0` 是「不等待」，`max_delay=inf` 是「交给退避公式」，都合法"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=1, base_delay=0.0,
                         sleeper=slept.append)
        assert slept == [0.0]

        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=1, base_delay=1.0,
                         max_delay=float("inf"), sleeper=slept.append)
        assert slept == [1.0]

    def test_legal_knobs_still_produce_the_same_sequence(self):
        """收紧入参不得改动合法档位的退避数值（与 `compute_delay` 逐档对齐）"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=3, base_delay=0.5, factor=3.0,
                         max_delay=10.0, sleeper=slept.append)
        assert slept == [compute_delay(n, 0.5, 3.0, 10.0) for n in (1, 2, 3)]


class TestTwoDelayCeilings:
    """A72：两副封顶各管一支，且**故意**互不相犯

    一支是退避计算（归调用方传的 `max_delay`），另一支是服务端 `Retry-After`
    （归 `MAX_RETRY_AFTER`）。`models/base.py` 的 `_MAX_RETRY_DELAY` 曾被注释成
    「单次重试等待上限」，而实测 429 + `Retry-After: 45` 时 sleeper 收到 45.0 ——
    那句话说的是这一支管不到的事。口径见 docs/ARCHITECTURE.md §3.22。
    """

    @staticmethod
    def _suggests(seconds):
        return lambda exc: (True, seconds)

    def test_max_delay_does_not_clamp_a_server_instruction(self):
        """把「90 秒后再来」夹成 30 s = 对刚说过别敲门的服务端提前 3 倍敲门"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=2, max_delay=30.0,
                         classify=self._suggests(45.0), sleeper=slept.append)
        assert slept == [45.0, 45.0]

    def test_max_retry_after_clamps_it_instead(self):
        """服务端要 10 天也只等 300 s：这一支并非不设限，只是上限在别处"""
        from augmentor import MAX_RETRY_AFTER

        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=2, max_delay=30.0,
                         classify=self._suggests(864_000.0), sleeper=slept.append)
        assert slept == [MAX_RETRY_AFTER, MAX_RETRY_AFTER]

    def test_max_delay_does_clamp_the_computed_branch(self):
        """对照组：没有服务端建议时，`max_delay` 确实是那一支的上限"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=2, base_delay=100.0,
                         max_delay=30.0, sleeper=slept.append)
        assert slept == [30.0, 30.0]

    @pytest.mark.parametrize("suggested", [-3.0, 0.0])
    def test_nonsense_suggestion_reads_as_immediate_not_error(self, suggested):
        """负数是「时限早已过」的读法：等 0 s 重发，与 `parse_retry_after` 把过去
        的 HTTP-date 收成 0.0 同一口径（实测 `parse_retry_after("-3") == 0.0`、
        过去 2 小时的日期 → 0.0），所以这里刻意不判成坏输入。
        """
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=1,
                         classify=self._suggests(suggested), sleeper=slept.append)
        assert slept == [0.0]

    def test_both_ceilings_match_what_the_docs_promise(self):
        """改任一常数都得同步 §3.22 / §6 / `_MAX_RETRY_DELAY` 的注释，不能让文档漂走"""
        from augmentor import MAX_RETRY_AFTER
        from augmentor.models.base import ModelBackend

        assert (MAX_RETRY_AFTER, ModelBackend._MAX_RETRY_DELAY) == (300.0, 30.0)


class TestJitterPassthrough:
    """A65：`with_retries` 接上 `jitter` / `rng` 并原样透传给 `compute_delay`

    抖动这一支在 `compute_delay` 里早就存在，但 `with_retries` 从来不透传（改前实测
    函数体里 "jitter" / "rng" 各 0 命中），所以对唯一的产品调用点 `models/base.py`
    而言它一直是死代码。把它接上的理由不是「多一个旋钮」，而是本仓有 5 处
    `ThreadPoolExecutor`（`max_workers` 3/4/5）会并发打同一个后端：同一档退避会让
    它们**同时醒来**再撞一次限流，抖一下才散得开。

    两条边界：
    1. 默认 `jitter=0.0` ⇒ 既有档位数值一字不变（下一条与 `TestComputeDelay` 共同钉住）。
    2. 只作用于退避这一支；`classify` 给出建议等待时原样采纳、不叠加抖动 —— 与 §3.22
       记录的「拿到服务端的指示就照办」（A72 方向 ①）同一条口径。
    """

    class _StubRng(random.Random):
        """记录 `uniform` 入参、按固定值回报的随机源：让抖动这一支可断言而非可复现"""

        def __init__(self, value=1.0):
            super().__init__(0)
            self.value = value
            self.calls = []

        def uniform(self, a, b):
            self.calls.append((a, b))
            return self.value

    @staticmethod
    def _failing(n):
        """前 n 次失败、之后成功的目标函数"""
        state = {"i": 0}

        def func():
            state["i"] += 1
            if state["i"] <= n:
                raise RuntimeError("boom")
            return "ok"

        return func

    def test_default_jitter_leaves_the_ladder_untouched(self):
        """零回归锚点：不传 `jitter` 时序列与接参前逐字相同"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=3, sleeper=slept.append)
        assert slept == [1.0, 2.0, 4.0]

    def test_jitter_lands_on_the_backoff_branch(self):
        """透传必须真的落到 `compute_delay` 的抖动项上：叠加在档位**之上**，
        且询问区间是 `[0, jitter * 未抖动档位]`（实测比例 1.0403 那一类形状的来源）。
        """
        rng = self._StubRng(value=1.0)
        _res, stats = with_retries(self._failing(3), max_retries=3, sleeper=lambda d: None,
                                   jitter=0.5, rng=rng)
        assert stats.delays == [2.0, 3.0, 5.0]
        assert rng.calls == [(0.0, 0.5), (0.0, 1.0), (0.0, 2.0)]

    def test_injected_rng_makes_the_shake_reproducible(self):
        """同一种子两次跑出同一串等待，且它**确实抖过**（不等于不抖的档位）——
        只断言「相等」是不够的：摘掉透传后两次都交出 ``[1.0, 2.0, 4.0]``，照样相等。
        """
        a = with_retries(self._failing(3), max_retries=3, sleeper=lambda d: None,
                         jitter=1.0, rng=random.Random(42))[1].delays
        b = with_retries(self._failing(3), max_retries=3, sleeper=lambda d: None,
                         jitter=1.0, rng=random.Random(42))[1].delays
        assert a == b != [1.0, 2.0, 4.0]
        # 抖动加在档位之上、幅度 [0, 1×档位]，所以每档都落在 [base, 2×base]
        assert all(base <= d <= 2 * base for base, d in zip([1.0, 2.0, 4.0], a))

    def test_falsy_but_usable_rng_is_not_silently_replaced(self):
        """判回落用 `is None` 而不是 `or`：实测一个 `__bool__` 为假的种子 rng 被
        `rng or random.Random()` 静默换成全局随机源，同一个「固定种子」对象连续三次
        抽出三个不同值（实测 6.54 / 7.25 / 5.62），测试里就是不可复现。
        """
        class FalsyRandom(random.Random):
            def __bool__(self):
                return False

        first = with_retries(self._failing(3), max_retries=3, sleeper=lambda d: None,
                             jitter=1.0, rng=FalsyRandom(42))[1].delays
        second = with_retries(self._failing(3), max_retries=3, sleeper=lambda d: None,
                              jitter=1.0, rng=FalsyRandom(42))[1].delays
        assert first == second

    def test_jitter_never_stretches_a_server_instruction(self):
        """`Retry-After` 那一支不加抖动：桩随机源若被调用会加 99 s，实测它一次都没被调"""
        rng = self._StubRng(value=99.0)
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=2, max_delay=30.0, jitter=1.0, rng=rng,
                         classify=lambda exc: (True, 45.0), sleeper=slept.append)
        assert slept == [45.0, 45.0]
        assert rng.calls == []

    def test_max_jitter_bounds_the_worst_case_at_twice_the_ceiling(self):
        """`jitter` 判在 0-1 之内换来的那条可证上界：贴顶档位最坏 2 × `max_delay`。
        同时断言「确实越过 30 s」——抖动加在夹逼之后，这正是 30 s 那句口径要补条件的原因。
        """
        rng = random.Random(2026)
        samples = [compute_delay(4, 20.0, 2.0, 30.0, jitter=1.0, rng=rng)
                   for _ in range(2000)]
        assert max(samples) <= 60.0
        assert min(samples) >= 30.0
        assert any(s > 30.0 for s in samples)

    def test_missing_rng_still_shakes_within_the_bound(self):
        """`if rng is None: rng = random.Random()` 这条回落支路的本来用途：不注入时
        照抖，只是值不可预测 ⇒ 断范围而非断精确值（两条入口各走一遍，`with_retries`
        那一支同时证明「透传过去时 rng 就是 None」）。
        """
        assert 4.0 <= compute_delay(3, 1.0, 2.0, 30.0, jitter=0.5) <= 6.0
        delays = with_retries(self._failing(2), max_retries=2, sleeper=lambda d: None,
                              jitter=0.5)[1].delays
        assert [base <= d <= 2 * base for base, d in zip([1.0, 2.0], delays)] == [True, True]

    @pytest.mark.parametrize("bad", [1.5, 2.0, 50.0, -0.1, -5.0, float("nan"), True, "0.5"])
    def test_out_of_range_jitter_is_rejected_before_the_first_request(self, bad):
        """入口判参不是重复劳动：摘掉 `with_retries` 那行 `require_ratio` 后，坏抖动会先
        把 `func` 打出去（一次真实付费调用）再在第二档退避时抛错。
        """
        calls = []
        with pytest.raises(DataValidationError, match="jitter"):
            with_retries(lambda: calls.append(1), max_retries=2, sleeper=lambda d: None,
                         jitter=bad)
        assert calls == []


class TestJitterConfigSurface:
    """抖动与两副封顶在库层的默认档位

    本类原先钉的是「A65 只接到 `with_retries`，**没有**配置面」（A75 未做）。L49 把
    `retry_jitter` / `max_retry_wait` 一并接进 `augmentation.*` 之后，那句前提已经
    不成立，所以改写为本类的主张：库层默认值必须是「不抖 / 300 s」，配置面接上之后
    不改默认档位 ⇒ 不写这两个键的既有配置行为逐字不变。配置面本身的下钻断言在
    `tests/integration/test_pipeline.py::TestRetryKnobWiring`。
    """

    def test_compute_delay_defaults_to_no_shake(self):
        assert compute_delay(4, 20.0, 2.0, 30.0) == 30.0

    def test_module_exports_the_pair_with_documented_defaults(self):
        import inspect

        sig = inspect.signature(compute_delay)
        assert (sig.parameters["jitter"].default,
                sig.parameters["rng"].default) == (0.0, None)
        w_sig = inspect.signature(with_retries)
        assert (w_sig.parameters["jitter"].default,
                w_sig.parameters["rng"].default) == (0.0, None)

class TestServerWaitCeilingKnob:
    """A73 关闭：服务端指令那一支的封顶成了旋钮，但**只能夹小不能放大**

    与 `TestTwoDelayCeilings`（L47）成对：那一轮 300 s 还是写死的常数，本轮把它接成
    `with_retries(max_retry_wait=…)`，默认值仍取 `MAX_RETRY_AFTER`。为什么上界要判在
    运行时而不像 `retry_delay ≤ 60` 那样只判在校验器：这道封顶本身就是对外承诺，
    放开它等于交给配置一个造无界等待的入口（加判据前实测 `inf` 让 `Retry-After:
    3000` 原样睡着 3000 s）。
    """

    @staticmethod
    def _suggests(seconds):
        return lambda exc: (True, seconds)

    def test_default_still_clamps_at_the_documented_ceiling(self):
        """不传新参时与接参前逐字相同：A73 的零回归面"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=2,
                         classify=self._suggests(864_000.0), sleeper=slept.append)
        assert slept == [MAX_RETRY_AFTER, MAX_RETRY_AFTER]

    def test_shrinking_the_ceiling_reaches_the_branch(self):
        """调小真的管用：45 s 档下服务端要 10 天也只等 45 s"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=2, max_retry_wait=45.0,
                         classify=self._suggests(864_000.0), sleeper=slept.append)
        assert slept == [45.0, 45.0]

    def test_a_suggestion_below_the_ceiling_is_adopted_verbatim(self):
        """夹小不等于压低真实指令：90 s 在 120 s 档下照抄 90 s"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=1, max_retry_wait=120.0,
                         classify=self._suggests(90.0), sleeper=slept.append)
        assert slept == [90.0]

    def test_zero_means_stop_honouring_the_instruction(self):
        """`max_retry_wait=0` 的读法与 `max_delay=0` 同一条口径：不等，立刻重发"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=1, max_retry_wait=0.0,
                         classify=self._suggests(45.0), sleeper=slept.append)
        assert slept == [0.0]

    def test_it_does_not_clamp_the_backoff_branch(self):
        """两副封顶互不相犯（对照组）：`max_retry_wait` 管不到退避计算那一支"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=2, base_delay=100.0,
                         max_delay=30.0, max_retry_wait=1.0, sleeper=slept.append)
        assert slept == [30.0, 30.0]

    def test_the_ceiling_itself_is_an_inclusive_boundary(self):
        """300.0 合法：默认值就贴在这个点上，判成严格小于会把自己的默认档拒掉"""
        slept = []
        with pytest.raises(RuntimeError):
            with_retries(_always_fail, max_retries=1, max_retry_wait=300.0,
                         classify=self._suggests(900.0), sleeper=slept.append)
        assert slept == [300.0]

    @pytest.mark.parametrize("bad", [301.0, 1e9, float("inf"), -1.0, True, "300",
                                     float("nan")])
    def test_enlarging_the_ceiling_fails_before_the_first_request(self, bad):
        """放大上界 = 作废「300 s 封顶」这句承诺，所以判在入参处且不烧真实请求"""
        calls = []
        with pytest.raises(DataValidationError, match="max_retry_wait"):
            with_retries(lambda: calls.append(1), max_retries=2,
                         sleeper=lambda d: None, max_retry_wait=bad)
        assert calls == []

    def test_the_knob_is_declared_in_the_signature(self):
        """接线形状：`max_retry_wait` 的默认值必须**引用常数**而不是重抄 300.0，
        否则改常数时这一处会静默留在旧值上。
        """
        import inspect

        param = inspect.signature(with_retries).parameters["max_retry_wait"]
        assert param.default == MAX_RETRY_AFTER
