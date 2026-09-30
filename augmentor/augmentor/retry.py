# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""重试与指数退避模块

为模型调用等易失败的外部操作提供统一的重试策略：指数退避 + 两副封顶
（退避计算一支归 `max_delay`，服务端 `Retry-After` 一支归 `max_retry_wait`，
默认即 `MAX_RETRY_AFTER`）+ 可选抖动，sleeper 可注入以便测试。
"""

import logging
import random
import time
from dataclasses import dataclass, field
from typing import Any, Callable, Optional, Tuple, Type

from .validation import require_count, require_positive, require_ratio, require_seconds

logger = logging.getLogger(__name__)

try:
    import requests as _requests
except ImportError:  # pragma: no cover - requests 是核心依赖，缺失时降级为状态码判断
    _requests = None

# 值得重试的 HTTP 状态码：限流、请求超时、上游临时故障
RETRYABLE_STATUS_CODES = frozenset({408, 425, 429, 500, 502, 503, 504})

# 服务端 Retry-After 的采纳上限（秒）。避免异常大的值让流水线长时间挂起。
#
# 这是**第二副封顶**，与调用方传的 `max_delay`（退避计算那一支的上限）互不相犯：
# 服务端明确说「90 秒后再来」时就等 90 s —— 它不被 `max_delay` 压低（夹成 30 s 等
# 于对刚说过「别敲门」的服务端提前 3 倍敲门），但被本常数夹住（服务端要 10 天也只
# 等 300 s）。一次 `generate()` 的最坏总等待因此是
# `(总尝试次数 - 1) × 300`，而不是 `(总尝试次数 - 1) × max_delay`。
# 口径的来龙去脉见 docs/ARCHITECTURE.md §3.22。
#
# 自 L49 起本常数是 `with_retries(max_retry_wait=…)` 的**默认值**而不是唯一值：
# 用户可以把它调小（`augmentation.max_retry_wait`），但**不能调大** —— 判据里那道
# `maximum=MAX_RETRY_AFTER` 守的就是这句话，「300 s 封顶」是对外承诺，放开它等于
# 交给配置一个造无界等待的入口。
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
        attempt: 已失败次数（从 1 开始）。必须是 **不小于 1 的整数**：`attempt - 1`
            是指数，传 0 或负数不会报「等太久」，而是静默把退避算成 2 的负次幂
            （实测 `attempt=0` → 基础的 0.5 倍、`-3` → 0.0625 倍），即「第 0 次重试
            比第 1 次等得更短」这种无对应现实的档位。
        base_delay: 基础等待（秒）。判据 `require_seconds`：实测 `base_delay=-5` 时
            首尾两道夹逼会把它静默收成 ``0.0``（想要「等 5 秒」拿到「立刻重试」），
            `NaN` 收成 ``max_delay``，所以只能在入参处判。
        factor: 指数因子。判据 `require_positive`（**必须大于 0**）：实测 `0` 给出
            ``[1.0, 0.0, 0.0]``、`-2` 给出 ``[1.0, 0.0, 4.0]`` 这种隔档不等待的非
            单调序列，`NaN` 给出 ``[1.0, 30.0, 30.0]``，字符串与 `None` 则在幂运算
            处炸成与「参数写错」无关的 `TypeError`。
        max_delay: 等待上限（秒），只封顶**退避计算**这一支；服务端 `Retry-After`
            走 `with_retries` 的另一支、封顶在它的 `max_retry_wait`（默认
            `MAX_RETRY_AFTER`，见该常数的注释）。
            判据 `require_seconds`：实测 `max_delay=-1` 会把整条退避清零（`min()` 交出
            它再被尾部夹 0），而 `max_delay=0` 是合法的「不等待」请求，故下界取 0。
        jitter: 随机抖动比例（0-1），在指数值**之上**叠加 [0, jitter*delay]。判据
            `require_ratio`：抖动加在 `max_delay` 那道夹**之后**，所以它一旦非 0 就
            成了退避这一支真正的上限来源（实测同一贴顶档位 `jitter=1.0` 抽出
            30.05 ~ 59.87 s、`50.0` 抽出 32.27 ~ 1523.54 s，且 100% 越过 `max_delay`）；
            判在 1 以内可证地把这一支的最坏等待封在 2 × `max_delay`。负值与 NaN 过不了
            `if jitter > 0` 那道门，会与「没传参数」逐字同答（实测 `-5.0` / `NaN` 都交
            ``[1.0, 2.0, 4.0]``）—— 那也是一种静默，所以照样判。
        rng: 随机数生成器（可注入以便测试）。只有 `None` 才新建一个；回落判据用
            `is None` 而不是 `or` —— 实测一个 `__bool__` 为假的种子 rng（Mock 的常见
            形状）会被 `rng or random.Random()` 静默换成全局随机源，同一个「固定种子」
            对象连续三次调用抽出三个不同值，「可注入以便测试」当场失效。

    Returns:
        等待秒数（>= 0）

    Raises:
        DataValidationError: `attempt` 不是整数或小于 1；`base_delay` / `max_delay`
            不是不小于 0 的秒数；`factor` 不是大于 0 的数值；`jitter` 不在 0-1 之间
    """
    require_count("attempt", attempt, minimum=1)
    require_seconds("base_delay", base_delay, minimum=0.0)
    require_positive("factor", factor)
    require_seconds("max_delay", max_delay, minimum=0.0)
    require_ratio("jitter", jitter)
    delay = min(max_delay, base_delay * (factor ** (attempt - 1)))
    if jitter > 0:
        if rng is None:
            rng = random.Random()
        delay += rng.uniform(0.0, jitter * delay)
    return max(0.0, delay)


def with_retries(func: Callable[..., Any],
                  *args,
                  max_retries: int = 3,
                  base_delay: float = 1.0,
                  factor: float = 2.0,
                  max_delay: float = 30.0,
                  max_retry_wait: float = MAX_RETRY_AFTER,
                  jitter: float = 0.0,
                  rng: Optional[random.Random] = None,
                  retry_on: Tuple[Type[BaseException], ...] = (Exception,),
                  sleeper: Callable[[float], None] = time.sleep,
                  on_retry: Optional[Callable[[int, BaseException, float], None]] = None,
                  classify: Optional[Callable[[BaseException], Tuple[bool, Optional[float]]]] = None,
                  **kwargs) -> Tuple[Any, RetryStats]:
    """带指数退避重试地执行 func

    Args:
        func: 目标可调用对象
        *args: 位置参数
        max_retries: 最大重试次数（不含首次调用）。必须是 **不小于 0 的整数**，
            0 是合法请求（「只调用一次，失败就别再试」）。负数会让
            `range(0, max_retries + 1)` 直接为空：一次都不调用 func，然后
            `raise last_exc` 变成 `raise None` —— 实测症状是
            `TypeError: exceptions must derive from BaseException`，一个与
            「参数写错了」毫无关系的报错。
        base_delay/factor/max_delay: 退避参数，判据与 `compute_delay` 同名三参一致
            （`require_seconds` / `require_positive`）。在这里判一遍**不是重复劳动**：
            `compute_delay` 只在第一次失败之后才被走到，坏值会先白烧一次真实请求，
            再从循环里抛出与「参数写错」无关的 `TypeError`（实测 `base_delay='1'` 的
            症状），而那时原始异常已经彻底丢了。
        jitter/rng: 随机抖动比例与其随机源，原样透传给 `compute_delay`（同名两参，
            判据也两处一致：`require_ratio`）。默认 `jitter=0.0` ⇒ 既有档位数值
            一字不变（实测五档仍是 ``[1.0, 2.0, 4.0, 8.0, 16.0]``）。
            **只作用于退避这一支**：`classify` 给出建议等待（如 HTTP ``Retry-After``）时
            原样采纳、不再叠加抖动 —— 与 A72 拍的「拿到服务端的指示就照办」同一条口径。
        max_retry_wait: **服务端指令那一支**的等待上限（秒），默认 `MAX_RETRY_AFTER`
            （300 s）。判据 `require_seconds(..., maximum=MAX_RETRY_AFTER)` —— 上界不是
            防手滑，而是「300 s 封顶」这句承诺本身，所以本参数**只能把封顶夹小、不能放大**
            （加判据前实测：一旦允许放大，`Retry-After: 3000` 会原样睡着 3000 s）。与 `max_delay`
            的关系保持不变：两副封顶各管一支，`max_retry_wait` 管不到退避计算，
            `max_delay` 也管不到它。取 `0.0` 是合法档位，语义是「不再尊重服务端指令、
            失败就立刻重发」，与 `max_delay=0` 在退避一支的读法同一条口径。
        retry_on: 触发重试的异常类型元组
        sleeper: 等待函数（测试可注入）
        on_retry: 每次重试前的回调 (attempt, exc, delay)
        classify: 错误分类函数，返回 ``(是否可重试, 建议等待秒数)``。
            返回 ``(False, _)`` 时立即放弃重试并抛出原异常——用于区分
            「限流/网络抖动」（值得重试）与「鉴权失败/参数错误」（重试纯属浪费）。
            建议等待秒数非 None 时**替换**退避计算结果（如 HTTP ``Retry-After``），
            并且只受 `max_retry_wait` 封顶（默认 `MAX_RETRY_AFTER`）—— 上面那组
            `base_delay/factor/max_delay` 在这一支全部参不到场，两副封顶互不相犯
            （细节见 `MAX_RETRY_AFTER` 的注释与 docs/ARCHITECTURE.md §3.22）。
        **kwargs: 关键字参数

    Returns:
        (func 的返回值, RetryStats)

    Raises:
        DataValidationError: `max_retries` 不是不小于 0 的整数，或 `base_delay` /
            `max_delay` / `factor` / `jitter` / `max_retry_wait` 非法（在调用 `func`
            **之前**判掉）
        重试耗尽后抛出最后一次异常；被 classify 判定为不可重试时立即抛出
    """
    # 判参排在循环之前：`func` 可能是一次真实的付费 API 调用，坏档位不该先把它打出去
    require_count("max_retries", max_retries, minimum=0)
    require_seconds("base_delay", base_delay, minimum=0.0)
    require_positive("factor", factor)
    require_seconds("max_delay", max_delay, minimum=0.0)
    require_seconds("max_retry_wait", max_retry_wait, minimum=0.0,
                    maximum=MAX_RETRY_AFTER)
    require_ratio("jitter", jitter)
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

            # 两支的封顶不同，且这是刻意的：服务端指令一支只归 `max_retry_wait` 管，
            # **不夹 `max_delay`** —— 把「90 秒后再来」夹成 30 s 等于提前 3 倍敲门。
            if suggested is not None:
                delay = min(max_retry_wait, max(0.0, float(suggested)))
            else:
                delay = compute_delay(attempt + 1, base_delay, factor, max_delay,
                                      jitter=jitter, rng=rng)

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
