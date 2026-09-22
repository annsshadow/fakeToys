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
        **kwargs: 关键字参数

    Returns:
        (func 的返回值, RetryStats)

    Raises:
        重试耗尽后抛出最后一次异常
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
            if attempt >= max_retries:
                break
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
