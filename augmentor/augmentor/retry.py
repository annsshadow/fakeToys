# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""重试与指数退避模块

为模型调用等易失败的外部操作提供统一的重试策略：
指数退避 + 上限封顶 + 可选抖动，sleeper 可注入以便测试。
"""

import logging
import random
import time
from dataclasses import dataclass, field
from typing import Any, Callable, Optional, Tuple, Type

logger = logging.getLogger(__name__)

try:
    import requests as _requests
except ImportError:  # pragma: no cover - requests 是核心依赖，缺失时降级为状态码判断
    _requests = None

# 值得重试的 HTTP 状态码：限流、请求超时、上游临时故障
RETRYABLE_STATUS_CODES = frozenset({408, 425, 429, 500, 502, 503, 504})

# 服务端 Retry-After 的采纳上限（秒）。避免异常大的值让流水线长时间挂起。
MAX_RETRY_AFTER = 300.0


@dataclass
class RetryStats:
    """重试统计"""

    attempts: int = 0
    succeeded: bool = False
    delays: list = field(default_factory=list)

    @property
    def total_wait(self) -> float:
        return sum(self.delays)

    def to_dict(self) -> dict:
        return {
            "attempts": self.attempts,
            "succeeded": self.succeeded,
            "delays": self.delays,
            "total_wait": self.total_wait,
        }


def compute_delay(attempt: int,
                  base_delay: float = 1.0,
                  factor: float = 2.0,
                  max_delay: float = 30.0,
                  jitter: float = 0.0,
                  rng: Optional[random.Random] = None) -> float:
    """计算第 attempt 次重试前的等待时长

    Args:
        attempt: 已失败次数（从 1 开始）
        base_delay: 基础等待（秒）
        factor: 指数因子
        max_delay: 等待上限（秒）
        jitter: 随机抖动比例（0-1），在指数值上叠加 [0, jitter*delay]
        rng: 随机数生成器（可注入以便测试）

    Returns:
        等待秒数（>= 0）
    """
    delay = min(max_delay, base_delay * (factor ** (attempt - 1)))
    if jitter > 0:
        rng = rng or random.Random()
        delay += rng.uniform(0.0, jitter * delay)
    return max(0.0, delay)


def with_retries(func: Callable[..., Any],
                  *args,
                  max_retries: int = 3,
                  base_delay: float = 1.0,
                  factor: float = 2.0,
                  max_delay: float = 30.0,
                  retry_on: Tuple[Type[BaseException], ...] = (Exception,),
                  sleeper: Callable[[float], None] = time.sleep,
                  on_retry: Optional[Callable[[int, BaseException, float], None]] = None,
                  classify: Optional[Callable[[BaseException], Tuple[bool, Optional[float]]]] = None,
                  **kwargs) -> Tuple[Any, RetryStats]:
    """带指数退避重试地执行 func

    Args:
        func: 目标可调用对象
        *args: 位置参数
        max_retries: 最大重试次数（不含首次调用）
        base_delay/factor/max_delay: 退避参数
        retry_on: 触发重试的异常类型元组
        sleeper: 等待函数（测试可注入）
        on_retry: 每次重试前的回调 (attempt, exc, delay)
        classify: 错误分类函数，返回 ``(是否可重试, 建议等待秒数)``。
            返回 ``(False, _)`` 时立即放弃重试并抛出原异常——用于区分
            「限流/网络抖动」（值得重试）与「鉴权失败/参数错误」（重试纯属浪费）。
            建议等待秒数非 None 时优先于退避计算结果（如 HTTP ``Retry-After``）。
        **kwargs: 关键字参数

    Returns:
        (func 的返回值, RetryStats)

    Raises:
        重试耗尽后抛出最后一次异常；被 classify 判定为不可重试时立即抛出
    """
    stats = RetryStats()
    last_exc: Optional[BaseException] = None

    for attempt in range(0, max_retries + 1):
        stats.attempts = attempt + 1
        try:
            result = func(*args, **kwargs)
            stats.succeeded = True
            return result, stats
        except retry_on as exc:
            last_exc = exc

            retryable = True
            suggested: Optional[float] = None
            if classify is not None:
                retryable, suggested = classify(exc)

            if not retryable:
                logger.warning(
                    "第 %d 次调用失败（%s），判定为不可重试，立即放弃",
                    attempt + 1, type(exc).__name__,
                )
                break
            if attempt >= max_retries:
                break

            if suggested is not None:
                delay = min(MAX_RETRY_AFTER, max(0.0, float(suggested)))
            else:
                delay = compute_delay(attempt + 1, base_delay, factor, max_delay)

            stats.delays.append(delay)
            logger.warning(
                "第 %d 次调用失败（%s），%.2fs 后重试",
                attempt + 1, type(exc).__name__, delay,
            )
            if on_retry:
                on_retry(attempt + 1, exc, delay)
            sleeper(delay)

    raise last_exc


def should_retry(exc: BaseException, retry_on: Tuple[Type[BaseException], ...]) -> bool:
    """判断异常是否应触发重试

    Args:
        exc: 异常实例
        retry_on: 可重试异常类型

    Returns:
        是否可重试
    """
    return isinstance(exc, retry_on)


# ---------- 错误分类：区分「值得重试」与「重试纯属浪费」 ----------

def http_status_code(exc: BaseException) -> Optional[int]:
    """从异常中提取 HTTP 状态码

    兼容 `requests.exceptions.HTTPError`（状态码挂在 `exc.response` 上）
    以及自行携带 `status_code` 属性的异常。

    Args:
        exc: 异常实例

    Returns:
        状态码；无法判定时返回 None
    """
    response = getattr(exc, "response", None)
    status = getattr(response, "status_code", None)
    if isinstance(status, int):
        return status
    status = getattr(exc, "status_code", None)
    return status if isinstance(status, int) else None


def parse_retry_after(value: Any) -> Optional[float]:
    """解析 Retry-After 头的值

    支持两种合法形式：秒数（``"120"``）与 HTTP-date（``"Wed, 21 Oct 2026 07:28:00 GMT"``）。

    Args:
        value: 头部原始值

    Returns:
        等待秒数（>= 0）；无法解析时返回 None
    """
    if value is None:
        return None
    text = str(value).strip()
    if not text:
        return None

    try:
        return max(0.0, float(text))
    except ValueError:
        pass

    # HTTP-date 形式
    try:
        import datetime as _datetime
        from email.utils import parsedate_to_datetime

        when = parsedate_to_datetime(text)
        if when is None:
            return None
        now = _datetime.datetime.now(when.tzinfo) if when.tzinfo else _datetime.datetime.now()
        return max(0.0, (when - now).total_seconds())
    except (TypeError, ValueError) as e:
        logger.debug("无法解析 Retry-After=%r: %s", value, e)
        return None


def retry_after_seconds(exc: BaseException) -> Optional[float]:
    """从异常的响应头中读取 Retry-After

    Args:
        exc: 异常实例

    Returns:
        等待秒数；无该头或无法解析时返回 None
    """
    response = getattr(exc, "response", None)
    headers = getattr(response, "headers", None)
    if not headers:
        return None
    try:
        raw = headers.get("Retry-After")
    except (AttributeError, TypeError):
        return None
    return parse_retry_after(raw)


def classify_error(exc: BaseException) -> Tuple[bool, Optional[float]]:
    """判断异常是否值得重试，并给出建议等待时长

    规则（顺序敏感）：

    1. 网络层错误（连接失败 / 超时）→ **可重试**，无建议等待；
    2. 可重试状态码（限流 429、请求超时 408、上游 5xx）→ **可重试**，
       若有 ``Retry-After`` 则优先采用；
    3. 其它 4xx（400 / 401 / 403 / 404 / 422…）→ **不可重试**：
       这类是请求本身有问题，重试只会浪费配额与时间；
    4. 无法判定状态码的异常 → **可重试**（保持既有的宽松行为）。

    Args:
        exc: 异常实例

    Returns:
        ``(是否可重试, 建议等待秒数)``
    """
    if _requests is not None and isinstance(
        exc, (_requests.exceptions.ConnectionError, _requests.exceptions.Timeout)
    ):
        return True, None

    status = http_status_code(exc)
    if status is None:
        return True, None

    if status in RETRYABLE_STATUS_CODES:
        return True, retry_after_seconds(exc)
    if 400 <= status < 500:
        return False, None
    if 500 <= status < 600:
        return True, retry_after_seconds(exc)

    # 1xx/3xx 等不应由 raise_for_status 触发；保守起见不重试
    return False, None
