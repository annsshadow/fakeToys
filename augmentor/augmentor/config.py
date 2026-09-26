# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置管理模块"""

import logging
import os
import yaml
from dataclasses import dataclass, field, fields
from typing import Any, Dict, Optional
from pathlib import Path
from .exceptions import ConfigError, DataValidationError
from .logging_setup import (apply_logging_config, assert_format_renderable,
                            level_number)
from .retry import MAX_RETRY_AFTER
from .validation import (require_bool, require_count, require_ratio,
                         require_seconds, require_string, require_string_list)

# `augmentation` / `web` 两节的取值区间。**校验器与运行时判据共用这一批常量**：
# A77 的根因就是同一个上界在两边各抄一遍（抄完还漏），一边改了另一边不知道，
# 于是出现「校验器独有天花板」（`validate-config` 报红的配置其实跑得起来）与
# 「运行时独有判据」（绿灯跑进流水线才炸）两种症状。区间留在这里而不是校验器里，
# 因为它们是配置对象自己的契约 —— 不经 `validate-config` 的 SDK 直构同样要认。
VARIANTS_PER_SEED_RANGE = (1, 100)
NUM_THREADS_RANGE = (1, 100)
AUTO_SAVE_INTERVAL_MIN = 1
MAX_RETRIES_RANGE = (0, 20)
RETRY_DELAY_RANGE = (0.0, 60.0)
# 单次请求超时（秒）的区间（A74）。下界 1 不是防手滑，是实测出来的硬界：
# `requests` 对 `timeout=0` 抛 `ValueError: Attempted to set connect timeout to 0,
# but the timeout cannot be set to a value less than or equal to 0`（L71 实测，
# py314 + 本机 requests），而它发生在**第一次生成调用**上，症状是「校验绿灯的
# 配置在几小时后跑炸」。上界 600 与 `docs/DEPLOYMENT.md:226-227` 的 nginx
# `proxy_read_timeout 600s` 对齐 —— 客户端等得比网关还久没有意义，只会把
# 「网关已断」读成「模型很慢」。
REQUEST_TIMEOUT_RANGE = (1.0, 600.0)
# 单次请求超时的**唯一**默认档（A74）。取改前六个硬编码里的最大值（ollama 120，
# claude / gemini / openai / ernie 推理都是 60），而不是取「最常见的那档」：三值
# 口径要收敛成一值，方向只能朝「不新增任何一次超时截断」那侧 —— 被掐短是崩溃面，
# 被放长只是失败路多等，代价不对称。
# 这一档同时是 `AugmentationConfig` 的字段默认值与模型后端的类默认值：两边都
# **引用**本常数而不各自重抄（A77 的教训：抄第二份就等于给「两处悄悄不同」留位置）。
DEFAULT_REQUEST_TIMEOUT = 120.0
# 模型条目里三个采样键的取值区间（A113 / L72）。与上面那批同级：**校验器规格与
# `ModelConfig.__post_init__` 共用这几个常数**，不在 `config_validator` 里重抄一遍
# （A77 的根因就是同一条界抄两处，抄完还漏）。
#
# `temperature` 上界取 2.0 是「四家公开契约里最宽的那档」（OpenAI / Gemini 0-2、
# Claude 0-1、ERNIE 0-1），它**不是崩溃界**：`base.py` 只把它 `repr()` 进缓存键、
# 五个后端只把它塞进请求体，实测改前 `temperature: 999` 三面全绿（Temp
# `l72q/probe_before.json`），代价是请求被服务端拒成 400、再被 `classify_error`
# 读成「后端不可用」。判在 2 的**代价如实记**：ollama 一档（llama.cpp）本地不夹
# 这个值，于是「今天跑得起来的 >2 写法」本轮起加载即拒 —— 与本仓既有口径同类，
# `num_threads ≤ 100`、`variants_per_seed ≤ 100` 也不是崩溃界，而是「越界即无意义」
# 的天花板；要写 >2 在本仓没有合法表达。
TEMPERATURE_RANGE = (0.0, 2.0)
# `top_p` 的界取闭区间 0-1：上界 1 是各家一致；下界**含 0** 是刻意的松弛 —— 「0 是否
# 等价于关闭 nucleus 采样」五家说法不一致（有的直接拒），本地无从裁定，而 `< 0`
# 才是无争议的坏值。判据族的 `require_ratio` 默认闭区间与此一致，不为其开特例。
TOP_P_RANGE = (0.0, 1.0)
# `max_output_tokens` 只判下界与「必须是整数」，**不设上界**（A113 行的口径决定）：
# 五家的上限各不相同且随模型而变，任何本地天花板都是凭空造的第二个权威；实测它
# 只进请求体与缓存键（`grep max_output_tokens augmentor/` 五后端各 1 处 + `base.py:289`），
# 不参与任何分配或循环 ⇒ 越大的代价是「服务端拒」，与 `max_retries` 那种
# 「越界 = 本地量级失控」不同类。下界 1 与整型两刀都要：实测 `0` / `-5` / `2048.5` /
# `True` 改前全部原样进 HTTP body。
MAX_OUTPUT_TOKENS_MIN = 1
# `models.<名字>.type` 的封闭清单（A115 / L74）。三个消费者：本模块的
# `ModelConfig.__post_init__`、`config_validator.MODEL_ENTRY_FIELDS` 的
# `choices` 规格、`models/factory.py` 那句「支持的类型」文案。
#
# 为什么清单不住工厂而住这里：真正的权威是工厂那张「类型名 → 后端类」的
# 分发表，但 `factory.py` 本身要 `from ..config import ModelConfig`，
# 反过来 import 会成循环导入。于是这里与工厂各持一份、由
# `tests/unit/test_model_type_choices_l74.py` 把「两集相等」钉成断言
# （形状同 `MAX_RETRY_AFTER` / `LOGGING_LEVELS`：一处定义，两侧共引）。
#
# `ernie` **不是**合法取值：类叫 `ERNIEBackend`、出厂模板里那条也叫
# `ernie`，但它的 `type` 写的是 `baidu`；`models.default: ernie` 那一行
# 更不是 type，它是「默认用哪个条目名」的指针（`load_config` 里
# `name == 'default'` 直接跳过）。这两个混淆是本条缺陷存在的原因。
MODEL_TYPES = ("baidu", "openai", "ollama", "claude", "gemini")
# 模型条目的消息占位前缀（A120 / L75）。`ModelConfig.__post_init__` 不知道自己挂在
# 哪个条目名下 —— 条目名是 `load_config` 那一层的局部变量，而 dataclass 手里只有
# `self`。改前两面的不对称是：静态面按点分路径报错、天然带条目名，运行时那一律写死
# `models.<名字>.xxx` ⇒ 一份有两个坏条目的配置里两次抛错的消息**逐字相同**（实测
# `Temp/l75q/before.json`：`bad-temp-entryA` 与只有一条时同串）。症状不是判错而是
# 「判对了但指不出位置」。修法只搬文案、不动判据：判据仍在 dataclass（唯一权威），
# 加载层捕获后把这一串换成 `models.<真条目名>`。
MODEL_ENTRY_PLACEHOLDER = "models.<名字>"
# `quality` / `dedup` 两节评分阈值的闭区间（A118 / L76）。同一道界此刻住在三个地方，
# 所以它必须只有一个产地：
# - 校验器那一侧早就写了（`quality.threshold` 的规格历史上是**裸数字** `0.0/1.0`），
#   本轮把它换成引这两个常数；`dedup.threshold` 此前**根本没有规格**，本轮补上。
# - 运行时那一侧：`dedup.Deduplicator.__init__` 自己判 `< 0 or > 1` 并抛 `DedupError`
#   （改前那是唯一一道界，但它发生在**建对象时**而不是**读配置时**，且对非数值直接
#   `TypeError`）；`quality` 那一侧改前**两侧都没有**界，实测 `threshold=5.0` 让
#   闸门对任何样本恒判不通过、`-1.0` 让闸门静默失效（`total >= -1.0` 永真）。
# 取 0-1 闭区间是因为「总分是三档 0-1 指标按和为 1 的权重加权」⇒ 阈值落在 [0,1]
# 之外没有任何可读语义。与 `dedup` 那一支的**既有**判据逐字同集合（含两端），
# 所以对齐它不会改变任何一份今天能加载的配置。
QUALITY_THRESHOLD_RANGE = (0.0, 1.0)
DEDUP_THRESHOLD_RANGE = (0.0, 1.0)
PORT_RANGE = (1, 65535)
RATE_LIMIT_MIN_REQUESTS = 0
RATE_LIMIT_MIN_WINDOW_SECONDS = 0.0

logger = logging.getLogger(__name__)


def _reject_null_fields(section: str, obj: Any,
                        names: Optional[tuple] = None) -> None:
    """配置对象的字段没有「未提供」这种状态：空值就是 `None`。

    实测（L51 改前）YAML 写 `web: {port: }` 之后 `load_config` 把 `None` 原样放进
    字段，而校验器对同一批空值逐个报「类型错误: 期望 int, 实际 NoneType」⇒ 不判
    就是「校验器判红的配置照样能加载」，且下游 `Path(str(p))` 会把 `None` 变成一
    个**名叫 `None` 的白名单根目录**。

    `names` 是可选的白名单（L72 / A113 加）：只在这几个字段上判 null。它是给
    `ModelConfig` 用的 —— 模型条目里 `api_key` / `secret_key` / `base_url` 的 `None`
    与空串是**合法状态**（ollama 就没有 api_key，实测出厂模板给的是 `''`），
    `request_timeout` 的 `None` 更是「不覆盖全局档」这一档本身，所以那一节不能
    整节套用本判据，只能点名判采样三键。
    """
    field_names = obj.__dataclass_fields__ if names is None else names
    for name in field_names:
        if getattr(obj, name) is None:
            raise DataValidationError(
                f"{section}.{name} 不能是 null（配置里写了这个键却没有给值）"
            )


@dataclass
class ModelConfig:
    """模型配置"""
    type: str
    api_key: Optional[str] = None
    secret_key: Optional[str] = None
    base_url: Optional[str] = None
    model: str = "default"
    temperature: float = 0.99
    top_p: float = 0.95
    max_output_tokens: int = 2048
    # 该模型的单次请求超时（秒），覆盖 `augmentation.request_timeout` 的全局档。
    # `None` 是合法值且**有意为之**：本节与 `api_key` 同族允许「未提供」，因为它
    # 走的是 `conf.get(..., None)` 而非 `_load_section`，没有「写了键没给值」的
    # 混淆面（那个判据在 `augmentation` 节由 `_reject_null_fields` 守）。
    request_timeout: Optional[float] = None

    def __post_init__(self):
        """模型条目的取值判据（L71 / A74 + L72 / A113 + L74 / A115 + L75 / A114）。

        运行时这一侧的界全部引本模块常数（`TEMPERATURE_RANGE` / `TOP_P_RANGE` /
        `MAX_OUTPUT_TOKENS_MIN` / `REQUEST_TIMEOUT_RANGE`），静态那一侧的规格
        （`config_validator.MODEL_ENTRY_FIELDS`）引的是同一批 —— A77 立的规矩：
        一条界只住一个地方，两侧同批改，否则就会长出「校验器绿、加载时抛」或
        反过来。为什么必须在**这里**判而不是等后端：实测五个后端只是把这三个值
        塞进请求体（`temperature` / `top_p` / `max_output_tokens` 各 5 处），
        坏值的症状是「服务端 400 → `classify_error` 读成后端不可用」，真因永远
        看不见（L71 取证，Temp `l72q/probe_before.json` 十五例改前全绿）。

        `None` 的读法**按键分档**，不是整节统一：采样三键的 `null` 拒（它们有默认值，
        写了键不给值是手滑，与 `augmentation` 节同一口径），而 `request_timeout`
        的 `None` 是「本模型不覆盖全局档」这一档本身，必须放行（`require_seconds`
        对 `None` 短路）。

        消息里的 `models.<名字>` 一律是 `MODEL_ENTRY_PLACEHOLDER`（A120）：本方法拿不到
        条目名，加载层在换名之后再往上抛。
        """
        if self.type not in MODEL_TYPES:
            # 封闭清单而不是区间：`type` 是模型条目里唯一一个「值有一张名单、
            # 两侧都不判」的键。改前实测 `type: openaii` 构造成功，工厂抛
            # `ConfigError` 之后被 `pipeline._init_components` 那个
            # 「模型没配好就降级」的 `except Exception` 吞掉 ⇒
            # `model_backend = None`，服务照起、日志只有一条 WARNING。
            # 空串一并拒：它是 `load_config` 对「这条没写 type」的回落值
            # （`config.py` 里 `conf.get('type', '')`），语义上就是没配。
            raise DataValidationError(
                f"{MODEL_ENTRY_PLACEHOLDER}.type 不支持: {self.type!r}。"
                f"支持的类型: {', '.join(MODEL_TYPES)}"
            )
        # `model` 是第二个「值没有任何判据」的键（A114 的 `model` 分支）。改前实测
        # `model: ''` / `model:`（null）/ `model: 123` / `model: true` 四形状**两侧全绿**
        # （`Temp/l75q/before.json` 的 `model_shapes` 档），而它是要直发后端的：
        # openai / claude / ollama 的请求体带 `"model": ""`，gemini 更糟 —— 模型名在
        # **URL 里**，实测得到 `.../v1beta/models/:generateContent`。症状与 A113 同族：
        # 换回一条服务端 400，真看不见是配置。
        # 只判「在场值」的形状，**不**判必填：条目不写 `model` 时由 dataclass 的默认档
        # 兜住（`_model_entry`，A114 的单一权威），而把「必须有」判上去会凭空拒掉
        # baidu 那一档 —— `ERNIEBackend` 从不读 `config.model`（端点写死，新记 A121）。
        require_string(f"{MODEL_ENTRY_PLACEHOLDER}.model", self.model)
        _reject_null_fields(MODEL_ENTRY_PLACEHOLDER, self,
                            ("temperature", "top_p", "max_output_tokens"))
        lo, hi = TEMPERATURE_RANGE
        require_ratio(f"{MODEL_ENTRY_PLACEHOLDER}.temperature", self.temperature,
                      minimum=lo, maximum=hi)
        lo, hi = TOP_P_RANGE
        require_ratio(f"{MODEL_ENTRY_PLACEHOLDER}.top_p", self.top_p, minimum=lo, maximum=hi)
        require_count(f"{MODEL_ENTRY_PLACEHOLDER}.max_output_tokens",
                      self.max_output_tokens, minimum=MAX_OUTPUT_TOKENS_MIN)
        lo, hi = REQUEST_TIMEOUT_RANGE
        require_seconds(f"{MODEL_ENTRY_PLACEHOLDER}.request_timeout",
                        self.request_timeout, minimum=lo, maximum=hi)


@dataclass
class AugmentationConfig:
    """增强配置"""
    variants_per_seed: int = 5
    num_threads: int = 40
    auto_save_interval: int = 10
    max_retries: int = 3
    retry_delay: float = 1.0
    # 服务端 `Retry-After` 那一支的等待上限（秒）。只能夹小不能放大：
    # retry.MAX_RETRY_AFTER（300 s）是对外承诺的天花板，判据两处一致。
    max_retry_wait: float = 300.0
    # 退避的随机抖动比例（0-1，闭区间）。默认 0 ⇒ 各档等待与接参前逐字相同；
    # 非 0 会把退避一支的最坏等待上界放大为 max_delay × (1 + 本值)。
    retry_jitter: float = 0.0
    # 单次请求超时（秒）的全局档（A74）。默认值与判据上界都引本模块常数，
    # 后端类级默认值引同一个（`DEFAULT_REQUEST_TIMEOUT` 的注释解释了为什么是 120
    # 而不是 60）。各模型可用 `models.<名字>.request_timeout` 单独覆盖；ernie 换
    # token 那一支**不吃这两档**，它有自己的类常数 10 s —— token 换取本就该短，
    # 与推理共用一个数会把它放大到 12 倍（A74 收口时明确拍下的独立参数）。
    request_timeout: float = DEFAULT_REQUEST_TIMEOUT

    def __post_init__(self):
        """运行时判据（L51 / A77 + A82）：区间与校验器规格逐个同源。

        改前这一节只有校验器那一半：实测 `AugmentationConfig(max_retries=10**6,
        retry_delay=10**6, retry_jitter=50.0)` 无判据构造成功，而校验器对同一批值
        报 6 条错 ⇒ 不经 `validate-config` 的写法拿到的是「报红的配置其实跑得
        起来」，按 §3.24 的等待公式那是 999,999 × 300 s ≈ 83,333 h 的最坏预算。
        `auto_save_interval` 是本轮新立的两侧同判（改前两侧**都**没有判据：实测
        0 与 −1 都和 1 逐字同答，20 条样本各触发 20 次增量存盘，默认档 10 只 2 次）。
        """
        _reject_null_fields("augmentation", self)
        lo, hi = VARIANTS_PER_SEED_RANGE
        require_count("augmentation.variants_per_seed", self.variants_per_seed,
                      minimum=lo, maximum=hi)
        lo, hi = NUM_THREADS_RANGE
        require_count("augmentation.num_threads", self.num_threads,
                      minimum=lo, maximum=hi)
        require_count("augmentation.auto_save_interval", self.auto_save_interval,
                      minimum=AUTO_SAVE_INTERVAL_MIN)
        lo, hi = MAX_RETRIES_RANGE
        require_count("augmentation.max_retries", self.max_retries,
                      minimum=lo, maximum=hi)
        lo, hi = RETRY_DELAY_RANGE
        require_seconds("augmentation.retry_delay", self.retry_delay,
                        minimum=lo, maximum=hi)
        require_seconds("augmentation.max_retry_wait", self.max_retry_wait,
                        minimum=0.0, maximum=MAX_RETRY_AFTER)
        require_ratio("augmentation.retry_jitter", self.retry_jitter)
        lo, hi = REQUEST_TIMEOUT_RANGE
        require_seconds("augmentation.request_timeout", self.request_timeout,
                        minimum=lo, maximum=hi)


@dataclass
class QualityConfig:
    """质量评分配置"""
    enabled: bool = True
    threshold: float = 0.6
    weights: list = field(default_factory=lambda: [0.3, 0.4, 0.3])

    def __post_init__(self):
        """运行时判据（A118 / L76）：开关与阈值两键两侧同判。

        改前这一节是 A118 名单里最要紧的一处「契约面有旋钮、加载面零判据」：实测
        （`Temp/l76q/before.json`，提交态 b63648018；**Python 字典面**读数 —— 值直接
        喂给 `_load_section`，YAML 的拼法差异见 `validation.require_bool` 文案）

        - `enabled: 'no'` 加载放行，字段值是字符串 `'no'` ⇒ 下游 `if config.quality.enabled:`
          按真值走，用户想关掉的闸门**关不掉**，而 `validate_config` 对同一条报
          「期望布尔类型, 实际 str」⇒ 校验红 / 加载绿。
        - `threshold: 5.0` 加载放行，实测三档各 0.5 的样本总分 0.5 判 `passed=False`
          ⇒ 闸门把所有数据判成不合格（与 A118 当初记的「放行全部低质数据」正好相反，
          那一支是 `threshold: -1.0`：`0.5 >= -1.0` 永真 ⇒ 闸门静默失效）。两个方向
          都是「配置写错一个数字，产物整体反掉」，且都不出声。
        - `threshold: 'x'` / `threshold:`（null）不在加载面出声，而是晚到 `score()`
          里抛 `TypeError: '>=' not supported between instances of 'float' and 'str'`
          ——报错点离笔误隔了一整个流水线。

        `weights` 只判 null：形状与「和为 1」那两条判据的权威住在 `quality.QualityScorer`
        （`len != 3` / `abs(sum - 1) > 0.01` 抛 `QualityError`），在这里再抄一遍就是
        A77 禁止的第二份权威；剩下的洞（`weights: 'abc'` 长度恰好 3、判不过的是
        `sum()` 的 `TypeError`）记在 A124。
        """
        _reject_null_fields("quality", self)
        require_bool("quality.enabled", self.enabled)
        lo, hi = QUALITY_THRESHOLD_RANGE
        require_ratio("quality.threshold", self.threshold, minimum=lo, maximum=hi)


@dataclass
class DedupConfig:
    """去重配置"""
    enabled: bool = True
    threshold: float = 0.9

    def __post_init__(self):
        """运行时判据（A118 / L76）：与 `Deduplicator` 的既有那道界同集合、出声更早。

        这一节改前**有**判据，但只在建对象时：`dedup.threshold=1.7` 得到
        `DedupError: 阈值必须在 0-1 之间`（消息可行动），而 `dedup.threshold='x'`
        绕过它 —— `if threshold < 0 or threshold > 1` 在字符串上直接
        `TypeError: '<' not supported between instances of 'str' and 'int'`，
        同样是「报错点离笔误一整条流水线」。`enabled` 的非布尔写法今天靠真值判断：
        `0` / `'0'` / `'false'` / `[]` 四种写法两种语义（假 / 真 / 真 / 假），其中
        两种与用户写的字面意思相反；`0` 那一档只是**碰巧**对上意图（真值表与 YAML
        拼法的差异由 `validation.require_bool` 的文案统一记录）。

        判据取 `DEDUP_THRESHOLD_RANGE` 的闭区间，与 `Deduplicator.__init__` 的
        `< 0 or > 1` 对**数值**恰好同集合（含两端），所以补这一步不会改变任何一份
        今天能加载的配置；两边判决的等判由测试逐值钉住，不是这里的一句断言。
        那一支判据对 `bool` 与 `NaN` 是漏的（`nan < 0` 与 `nan > 1` 都是 `False` ⇒
        NaN 一路走到 `dist >= threshold` 上恒假，去重整条静默失效；实测
        `Temp/l76q/parity.json`），本轮**只**在配置侧挡住，消费侧那一洞另立 A125。
        """
        _reject_null_fields("dedup", self)
        require_bool("dedup.enabled", self.enabled)
        lo, hi = DEDUP_THRESHOLD_RANGE
        require_ratio("dedup.threshold", self.threshold, minimum=lo, maximum=hi)


@dataclass
class ExportConfig:
    """导出配置"""
    default_format: str = "jsonl"
    formats: list = field(default_factory=lambda: ["jsonl", "llama_factory", "alpaca", "sharegpt", "chatml"])


@dataclass
class ContextConfig:
    """对话上下文配置"""
    enabled: bool = False
    num_turns: int = 3


@dataclass
class VersioningConfig:
    """版本管理配置"""
    enabled: bool = True
    storage_dir: str = "data/versions"
    auto_snapshot: bool = True


@dataclass
class SamplerConfig:
    """主动学习配置"""
    enabled: bool = False
    dimensions: list = field(default_factory=lambda: ["topic", "question_type", "length", "complexity"])


@dataclass
class ExpanderConfig:
    """领域扩展配置"""
    enabled: bool = False
    strategies: list = field(default_factory=lambda: ["similar", "related", "scenario"])


@dataclass
class TrackerConfig:
    """效果追踪配置"""
    enabled: bool = False
    metrics: list = field(default_factory=lambda: ["train_loss", "eval_accuracy", "perplexity"])


@dataclass
class VisualizationConfig:
    """可视化配置"""
    enabled: bool = True
    types: list = field(default_factory=lambda: ["wordcloud", "length_distribution", "topic_cluster", "timeline", "quality_distribution"])


@dataclass
class MultilingualConfig:
    """多语言配置"""
    enabled: bool = False
    default_target_lang: str = "en"
    supported_langs: list = field(default_factory=lambda: ["zh", "en"])
    translate_batch_size: int = 10


@dataclass
class RAGConfig:
    """RAG 训练数据配置"""
    enabled: bool = False
    default_format: str = "llamaindex"
    chunk_size: int = 512
    chunk_overlap: int = 64


@dataclass
class EvaluationConfig:
    """模型评估配置"""
    enabled: bool = False
    metrics: list = field(default_factory=lambda: ["bleu", "rouge_l", "similarity"])
    reference_field: str = "output"


@dataclass
class VectorConfig:
    """向量数据库配置"""
    enabled: bool = False
    backend: str = "faiss"
    dimension: int = 384
    storage_dir: str = "data/vectors"
    collection: str = "default"


@dataclass
class MultimodalConfig:
    """多模态数据配置"""
    enabled: bool = False
    image_extensions: list = field(default_factory=lambda: [".jpg", ".jpeg", ".png", ".bmp", ".webp"])
    audio_extensions: list = field(default_factory=lambda: [".wav", ".mp3", ".flac", ".ogg", ".m4a"])


@dataclass
class BenchmarkConfig:
    """数据质量基准配置"""
    enabled: bool = False
    baseline_file: str = "data/benchmark_baseline.json"
    metrics: list = field(default_factory=lambda: ["pass_rate", "avg_total_score", "diversity", "duplication_rate"])


@dataclass
class ActiveLearningConfig:
    """主动学习循环配置"""
    enabled: bool = False
    strategy: str = "uncertainty"
    batch_size: int = 50
    max_iterations: int = 10


@dataclass
class FrameworkConfig:
    """LLM 框架集成配置"""
    enabled: bool = False
    frameworks: list = field(default_factory=lambda: ["langchain", "llamaindex"])


@dataclass
class WebConfig:
    """Web UI 配置"""
    port: int = 8000
    host: str = "0.0.0.0"
    static_dir: str = "web/dist"
    # 跨源默认**一个也不放行**：随包 UI 与后端同源（axios `baseURL: '/api'`、vite dev
    # 用 proxy），所以 CORS 头只对第三方浏览器客户端有意义。实测 starlette 1.6.0 在
    # `["*"] + credentials=True` 下会把请求方的 Origin **原样回显**并附
    # `access-control-allow-credentials: true`（预检一并放行）⇒ 任意网站都能带凭据读这个
    # API。需要跨源访问请显式列出白名单，见 config.yaml。
    cors_origins: list = field(default_factory=list)
    # 不带凭据：本仓 API 不用 cookie（`set_cookie` / `request.cookies` 全 0 命中），
    # 鉴权走 `X-API-Key` 头 ⇒ `allow_credentials` 对合法用法零收益、纯风险。
    cors_credentials: bool = False
    # REST API 允许访问的目录白名单。客户端传入的文件路径 resolve 后必须落在其中
    # 某个根目录内，否则返回 403。默认 `["data"]`，即只放行数据目录。
    # 相对路径的解析顺序：先在白名单各根目录内找已存在的文件（所以裸文件名可用），
    # 找不到才按**进程工作目录**解释（见 `api.deps.resolve_within_roots`）。
    # 写入必须显式给出白名单内的路径，例如 `data/xxx.json`。
    # 环境变量 AUGMENTOR_DATA_ROOTS（os.pathsep 分隔）优先级更高。
    data_roots: list = field(default_factory=lambda: ["data"])
    # 滑动窗口限流：窗口内单客户端最大请求数。0 表示关闭限流。
    rate_limit_max_requests: int = 300
    # 限流窗口长度（秒）
    rate_limit_window_seconds: float = 60.0
    # 免限流路径（前缀匹配）
    rate_limit_exempt_paths: list = field(
        default_factory=lambda: ["/api/health", "/docs", "/redoc", "/openapi.json"]
    )

    def __post_init__(self):
        """运行时判据（L51 / A80）：`web` 节管的是访问边界，不能只在显式校验时才判。

        L50 把这一节接上了校验器，但校验器只在跑 `validate-config` 时才动，于是
        两侧都还空着的那一半就是症状本身（全部改前实测）：

        - `data_roots: data`（写成标量）被逐字符拆成 `d/a/t/a` 四个根 ⇒ 数据端点
          一律 403，看起来像后端坏了；
        - `cors_origins: "https://api.corp.example"`（同样写成标量）交给 starlette
          后走的是**子串**匹配（1.6.0 与 1.2.1 同形）⇒ `https://api.corp`、
          `https://api` 这些**别的主机**被放行 —— 收紧意图拿到的是放宽结果；
        - `data_roots=[None]` 经 `Path(str(p))` 变成一个名叫 `None` 的白名单根；
        - `rate_limit_window_seconds=NaN` 让窗口永不滚动 ⇒ 超过阈值后**永久 429**，
          且 `retry_after()` 抛 `ValueError: cannot convert float NaN to integer`；
          写成负数则是限流静默关闭。

        区间常量与校验器那一侧同一批（见模块开头），所以「配了不生效」和「两边
        各判一套」这两件事同时被封住。
        """
        _reject_null_fields("web", self)
        lo, hi = PORT_RANGE
        require_count("web.port", self.port, minimum=lo, maximum=hi)
        require_string("web.host", self.host)
        require_string("web.static_dir", self.static_dir)
        require_string_list("web.cors_origins", self.cors_origins)
        if not isinstance(self.cors_credentials, bool):
            raise DataValidationError(
                f"web.cors_credentials 必须是布尔值，当前是 "
                f"{self.cors_credentials!r}（{type(self.cors_credentials).__name__}）"
            )
        require_string_list("web.data_roots", self.data_roots)
        require_count("web.rate_limit_max_requests", self.rate_limit_max_requests,
                      minimum=RATE_LIMIT_MIN_REQUESTS)
        require_seconds("web.rate_limit_window_seconds",
                        self.rate_limit_window_seconds,
                        minimum=RATE_LIMIT_MIN_WINDOW_SECONDS)
        require_string_list("web.rate_limit_exempt_paths", self.rate_limit_exempt_paths)


@dataclass
class LoggingConfig:
    """日志配置（L57 / A97 起真的生效）

    三档默认值不是「想要的样子」而是**今天实际发生的样子**：CLI 进程里 root logger
    一个 handler 都没有，日志走 `logging.lastResort`，那是 WARNING 档 + `%(message)s`
    裸消息落 stderr。实测（Temp `l57/probe1.txt` P0/P1）按这三档装上 handler 之后同一条
    WARNING 的 stderr 逐字节不变 —— 所以「没写 `logging` 节」与「写了三档默认值」
    在两个面上同形。

    `file` 默认空串 = **不落文件**。相对路径按进程工作目录解释，父目录必须已存在。
    """
    level: str = "WARNING"
    file: str = ""
    format: str = "%(message)s"

    def __post_init__(self):
        """运行时判据：三键各有一个人话可执行的错法，且与校验器同一批判据

        - `level: INFORMATION`（看着像拼错的 INFO）⇒ 数值化失败，出声拒收。允许集
          只有 `logging_setup.LOGGING_LEVELS` 一份，`config_validator` 的 `choices`
          规格引的就是它。
        - `format: "%(nope)s"`（构造期合法、发一条才炸）⇒ 由
          `logging_setup.assert_format_renderable` 对着真 record 试渲染一次挡掉；
          同一个串在一个进程里只探一次（判据不省，省重复），理由见该函数 docstring。
        - `file: 1` / `file: null` ⇒ 类型判据。空串是**合法值**（= 不落文件），
          所以这里不能用拒空串的 `require_string`；`None` 由 `_reject_null_fields`
          挡（写了键没给值）。
        """
        _reject_null_fields("logging", self)
        require_string("logging.level", self.level)
        level_number(self.level)
        require_string("logging.format", self.format)
        assert_format_renderable(self.format)
        if isinstance(self.file, bool) or not isinstance(self.file, str):
            raise DataValidationError(
                f"logging.file 必须是字符串（空串 = 不落文件），当前是 "
                f"{self.file!r}（{type(self.file).__name__}）"
            )


@dataclass
class AppConfig:
    """应用配置"""
    models: dict = field(default_factory=dict)
    default_model: str = "ernie"
    augmentation: AugmentationConfig = field(default_factory=AugmentationConfig)
    quality: QualityConfig = field(default_factory=QualityConfig)
    dedup: DedupConfig = field(default_factory=DedupConfig)
    export: ExportConfig = field(default_factory=ExportConfig)
    context: ContextConfig = field(default_factory=ContextConfig)
    versioning: VersioningConfig = field(default_factory=VersioningConfig)
    sampler: SamplerConfig = field(default_factory=SamplerConfig)
    expander: ExpanderConfig = field(default_factory=ExpanderConfig)
    tracker: TrackerConfig = field(default_factory=TrackerConfig)
    visualization: VisualizationConfig = field(default_factory=VisualizationConfig)
    multilingual: MultilingualConfig = field(default_factory=MultilingualConfig)
    rag: RAGConfig = field(default_factory=RAGConfig)
    evaluation: EvaluationConfig = field(default_factory=EvaluationConfig)
    vector: VectorConfig = field(default_factory=VectorConfig)
    multimodal: MultimodalConfig = field(default_factory=MultimodalConfig)
    benchmark: BenchmarkConfig = field(default_factory=BenchmarkConfig)
    active_learning: ActiveLearningConfig = field(default_factory=ActiveLearningConfig)
    frameworks: FrameworkConfig = field(default_factory=FrameworkConfig)
    web: WebConfig = field(default_factory=WebConfig)
    logging: LoggingConfig = field(default_factory=LoggingConfig)


def _resolve_env(value: Any) -> Any:
    """解析环境变量占位符
    
    Args:
        value: 配置值，可能是 ${ENV_VAR} 格式的环境变量占位符
    
    Returns:
        解析后的值
    """
    if isinstance(value, str) and value.startswith('${') and value.endswith('}'):
        env_var = value[2:-1]
        return os.environ.get(env_var, '')
    return value


# 模型条目的键集，**从 dataclass 推导**（A114 / L75）。写死的清单正是本条缺陷的
# 形状：改前 `load_config` 手抄了九个回落值，实测其中四个与 `ModelConfig` 的字段
# 默认不一致（`Temp/l75q/before.json` 的 `a114_drift` 档）。同一套推导的先例是
# `config_validator._warn_unread_model_keys`（它同样用 `fields(ModelConfig)` 而不是
# 抄一份），所以给 `ModelConfig` 加字段时不需要记得改第二处 —— 反过来「条目里出现
# 一个 dataclass 没有的键」由 A76 那一路「写了没人读」负责出声。
MODEL_ENTRY_KEYS = frozenset(f.name for f in fields(ModelConfig))

# 需要解析 `${ENV}` 占位符的三个键。这份清单是 A95 记下的现状（全仓只有模型条目
# 这三处真的解析占位符），本轮只是把它从「三行调用」提成一个具名元组，好让
# `_model_entry` 的循环不再点名。
MODEL_CREDENTIAL_KEYS = ("api_key", "secret_key", "base_url")


def _model_entry(name: str, conf: Dict) -> ModelConfig:
    """把一条模型条目装配成 `ModelConfig`：回落值只有一个权威（A114），消息带条目名（A120）

    三条口径，每条都对应一处实测：

    1. **只传 YAML 里在场的键**，不在场的交给字段默认值 ⇒ 加载侧不再抄第二份默认。
       改前漂移的四个键分两支：`api_key` / `secret_key` / `base_url` 从 `''` 变 `None`
       —— 实测五个后端对两种假值**逐字同判**（`before.json` 的 `credential_shape`
       十例：同一句 `ModelNotConfiguredError`）⇒ 本改对凭证消费方零影响，且 `''`
       从来只是加载侧的私有伪装；`model` 从 `''` 变 `'default'` —— 那一支是真缺陷，
       空串模型名会被原样直发（实测 gemini 的 URL 变成 `.../models/:generateContent`）。
    2. **未知键丢弃**：出声归 `_warn_unread_keys`（A76 / A84），这里不重复判。
    3. **`type` 的存在性判在这里、合法性仍归 `__post_init__`**：`type` 是唯一没有
       默认值的字段，而 `__post_init__` 看不见「键不在场」（它手里只有已绑定的值）——
       这正是 A119 在静态面上的同一个结构洞，两侧各补自己那一层，措辞与静态面的
       `_check_required_fields` 两条判决对齐。

    Args:
        name: 条目名（用户起了什么就是什么），只用于报错文案
        conf: 该条目的原始映射（已由 `_as_mapping` 护过形状）

    Returns:
        构造完成的 `ModelConfig`

    Raises:
        DataValidationError: 条目没写 `type`、`type` 是 null，或某个在场值越界
            （消息里带真实条目名）
    """
    kwargs = {}
    for key, value in conf.items():
        if key not in MODEL_ENTRY_KEYS:
            continue
        if key in MODEL_CREDENTIAL_KEYS:
            value = _resolve_env(value)
        kwargs[key] = value

    where = f"models.{name}"
    if "type" not in kwargs:
        raise DataValidationError(
            f"{where}.type 缺少必填字段（支持的类型: {', '.join(MODEL_TYPES)}）"
        )
    if kwargs["type"] is None:
        raise DataValidationError(
            f"{where}.type 不能是 null（配置里写了这个键却没有给值）"
        )

    try:
        return ModelConfig(**kwargs)
    except DataValidationError as exc:
        # 判据不动，只换文案：把 dataclass 那一层的占位符换成真实条目名。
        # `from None` 而不是 `from exc`：两条消息除条目名外逐字相同，链两层等于让
        # 用户在栈里读两遍同一句话，反而更难定位。
        raise DataValidationError(
            str(exc).replace(MODEL_ENTRY_PLACEHOLDER, where)
        ) from None


def _load_section(raw_config: Dict, key: str, config_class: type) -> Any:
    """加载配置节：**缺哪个键由那个字段的默认值答**（A123）

    改前这里有第四个参数 `defaults`：`load_config` 里那张 20 行映射表把每一节的
    默认值又抄了一遍（68 个键），于是同一个回落值有两个权威 —— 与 L75 刚收掉的
    `conf.get(key, 默认)` 九行是**同族第二格**（A114）。本函数按 `_model_entry`
    已经定下的口径改：只把 YAML 里**在场、且那个字段可传入**（`init`）的键交给构造器，
    其余一律不传。

    为什么这一步现在是安全的（`Temp/l77q/a123_census.json`，**Python 字典面**；
    这份取证在改前跑过一遍（提交态 edb26ddbe），改后又跑一遍 —— 改前那一侧不再是
    内存里的真身，而是从 `git show HEAD` 逐字复刻的旧函数，复刻与 HEAD 源码由 AST
    同式判据守着，不一致脚本当场自杀）：

    - **键集双向差 = 0**：表里的 68 个键与 20 节 dataclass 的 init 字段**恰好相等**
      （`only_in_table` 与 `only_in_fields` 两侧都空，`AppConfig` 的 20 个节字段
      也全部被表覆盖）⇒ 没有「表里有而字段没有」的假键，也没有「字段有而表漏了」
      的静默回落。
    - **值 / 类型漂移 = 0**：68 个共有键逐键比 `==` 与 `type()`，两档都无差异；
      另有 15 个键的表默认是 list/dict 字面量、字段侧用 `default_factory` —— 值相等
      （所以不算漂移），但改后由工厂答 ⇒ 少一份「每次调用新建字面量」的隐式约定
      （`test_config.py:503` 钉过的那一维：把字面量提到模块级就会变成跨实例共享对象，
      删掉表之后这个风险面直接不存在）。
    - **行为等价 120/120**：20 节 × 六档输入（整节不在 / 节写成 `null` / 空映射 /
      一个键在场 / 一个键在场且给 `null` / 带一个不认识的键），改前的复刻函数与本函数
      各跑一遍 ⇒ 落值全等；「键在场且给 `null`」那一档里有 5 节走到运行时判据，两侧
      抛出**逐字相同**的 `DataValidationError: <节>.<键> 不能是 null（配置里写了这个
      键却没有给值）`（比较的是「异常类型 + 消息」拼成的字符串，不是只比类型）。

    Args:
        raw_config: 原始配置字典
        key: 配置节名称
        config_class: 该节对应的 dataclass 类型（默认值的唯一权威）

    Returns:
        配置实例

    Raises:
        ConfigError: 该节写成了标量 / 列表等「键: 值」以外的形状
    """
    if key not in raw_config:
        return config_class()

    conf = raw_config[key]
    # 只写一个节名、下面什么都没有（`logging:`）时 YAML 给的是 `None`，与 A94 里
    # 0 字节文件同形；`logging: app.log` 这种「把节当值写」给的是标量。两种过去都
    # 在 `conf.get` 上抛 `AttributeError: 'NoneType' object has no attribute 'get'`
    # / `'str' object has no attribute 'get'`（实测 Temp `l57/probe1.txt` P4/P5：
    # **每一节**都同形，不止 logging），用户拿到的是无法行动的栈。语义与 A94 一致：
    # 写了节名而什么都没写 = 该节全默认；写成标量则是摆错了形状，明说。
    if conf is None:
        return config_class()
    if not isinstance(conf, dict):
        raise ConfigError(
            f"{key} 必须是「键: 值」的映射，当前是 {conf!r}"
            f"（{type(conf).__name__}）"
        )

    # 取键用 `__dataclass_fields__` 而不是 `fields(cls)` 再套一层 frozenset：这不是
    # 风格偏好，是同进程 A/B 量出来的差额（`Temp/l77q/perf_ab_l77.py` → `perf_ab.json`
    # 的 `runs` 键存着**连续两跑的逐字读数**；20 节 × 三档输入 × 7 轮交替 × 400 次，
    # 三侧产物逐字相同才计入，否则脚本自杀）—— 「空映射」那一档表版 18.81 / 18.39 µs、
    # `fields`+frozenset 版 29.05 / 28.29 µs（**比改前慢 +54.45 % / +53.84 %**，因为每次
    # 调用都新建一个集合）、本写法 12.69 / 12.22 µs（比改前 **−32.53 % / −33.56 %**，
    # 两跑的 20/20 节都是「每轮更快」）。只在「节不在场」那一档两种新写法打平
    # （7.83 对 7.86、8.01 对 8.00 µs，两者都比表版快三成），因为那一支根本不取键；
    # 「只带一个拼错的键」那一档与本写法同形状（表版 18.97 / 18.58 µs，中间版
    # +56.57 % / +55.86 %，本写法 −29.27 % / −30.25 %）。
    # 本模块的既有口径也一致：热点上的 `_reject_null_fields`（`:126`）读的就是这个
    # 属性，而只在导入期跑一次的 `MODEL_ENTRY_KEYS`（`:605`）用 `fields()`。
    #
    # `and names[k].init` 不是防御性冗余，是**「保存 → 重新加载」这一趟的行为面**：
    # `save_config` 用 `dataclasses.asdict()`（见它内部的 `_to_dict`），而 asdict
    # **不看 `init`**，
    # 于是任何一节的 `init=False` 字段都会写进 YAML；加载侧若把它喂回构造器，实测
    # （`Temp/l77q/inject_l77.json` 的 m7 档，改前实现）当场
    # `TypeError: AugmentationConfig.__init__() got an unexpected keyword argument
    # 'l77_probe'` —— 即「产品自己写出的配置文件把自己打崩」。HEAD 那张表只含 init
    # 键，所以旧实现是把该键无声丢掉（另一笔账，见 `test_no_section_has_a_non_init_field`
    # 的棘轮理由）。判据一侧同步：`consumed_section_keys()` 也只认 init 字段，
    # 否则那个键会既没人读、又不在「写了没人读」名单里。
    names = config_class.__dataclass_fields__
    # 未知键在这里丢弃但**不出声**：出声归 `_warn_unread_keys`（A76 / A84），
    # 与 `_model_entry` 的第 2 条口径同式。
    return config_class(**{k: v for k, v in conf.items()
                           if k in names and names[k].init})


def _log_unread_keys(raw_config: Dict) -> None:
    """把「写了没人读」的键在**加载**这一步就说出来（A84）

    诊断面（`validate_config`）早就报这条，产品面却一声不吭：拼错一个键名的
    人拿到的是「配置生效了、值却没变」，只能靠读源码找回拼写。走
    `logging.warning` 而不是 stdout —— 不碰任何命令的输出契约；不进退出码 ——
    与 L53 定下的「WARNING 不判负」同一口径。出厂 `config.yaml` 零命中，
    所以正常一次运行不多一行输出，拼错才出声。

    Args:
        raw_config: 已从 YAML 读出的原始配置字典
    """
    # 反向 import：`config_validator` 顶部要用本模块的区间常量，只有函数内取才不成环。
    from .config_validator import unread_key_messages

    for message in unread_key_messages(raw_config):
        logger.warning("%s", message)


def load_config(config_path: Optional[str] = None) -> AppConfig:
    """加载配置文件
    
    Args:
        config_path: 配置文件路径，为 None 时使用默认配置
    
    Returns:
        AppConfig 实例
    """
    config = AppConfig()
    
    if config_path and os.path.exists(config_path):
        with open(config_path, 'r', encoding='utf-8') as f:
            # 空文件或只含注释时 YAML 返回 `None`，而下面所有 `'x' in raw_config`
            # 都会抛 `TypeError: argument of type 'NoneType' is not iterable`
            # ——实测 CLI 拿到的是这行无法行动的栈信息（A94）。一份「什么都没写」的
            # 配置语义上就是全默认，和本函数不传路径同解；`or {}` 的先例在本模块
            # `save_config` 里已经有了。
            raw_config = yaml.safe_load(f) or {}
        
        # 加载模型配置
        # `models` 走这条特判路径、绕开了 `_load_section` 的形状守卫（A101 当年只
        # 护住走 `_load_section` 的那 20 节）。于是 `models:` 写成标量、或某个模型条目写成标量 /
        # 空 / 列表时，过去会当场崩在 `model_conf.get` 的 `AttributeError`，而校验
        # 面却报 `is_valid=False` —— 同族缺陷两侧不同判（A85，A101 的漏网）。这里补
        # 回与 `_load_section` 逐字同口径的判据：写成非映射抛可行动的 ConfigError；
        # 只写名字没给内容（None）先折成空条目 —— 而空条目（`models.qwen:` 光一个名字）
        # 自 A119 起在 `_model_entry` 的**存在性**判据上当场拒。改前它是折成空 dict、
        # 再靠 `conf.get('type', '')` 回落成 `''`、最后被 `type` 的封闭清单拒掉：
        # 两版都拒，但消息从「不支持: ''」变成「缺少必填字段」，指得更准 ——
        # 一条没有 `type` 的条目永远建不出后端，留着只会让 `pipeline` 把它读成
        # 「后端不可用」（来龙去脉见 `tests/unit/test_model_section_shape_l63.py`）。
        def _as_mapping(value, where):
            if value is None:
                return {}
            if not isinstance(value, dict):
                raise ConfigError(
                    f"{where} 必须是「键: 值」的映射，当前是 {value!r}"
                    f"（{type(value).__name__}）"
                )
            return value

        if 'models' in raw_config:
            models_raw = _as_mapping(raw_config['models'], 'models')
            # 同一族的第三格（A123）：改前这里写的是 `models_raw.get('default', 'ernie')`，
            # 把 `AppConfig.default_model` 的默认值又抄了一遍。实测两侧同值
            # （`AppConfig().default_model` 与字面量 `'ernie'` 逐字相同，
            # `Temp/l77q/default_model.json`）⇒ 改成「键在场才覆盖」与改前等价，
            # 但回落值从此只有字段默认一个权威。
            if 'default' in models_raw:
                config.default_model = models_raw['default']
            for name, model_conf in models_raw.items():
                if name == 'default':
                    continue
                conf = _as_mapping(model_conf, f"models.{name}")
                # 回落值不再抄在这里（A114）：`_model_entry` 只把 YAML 里在场的键
                # 交给 `ModelConfig`，缺哪个键就由该字段的默认值答哪个 —— 改前这
                # 九行手抄默认里有四行与 dataclass 不一致。`${ENV}` 占位符也在那里
                # 解析（只那三条凭证键，A95 记的现状）。
                config.models[name] = _model_entry(name, conf)
        
        # 节 → 类的清单。这里**只列名字与类型，不再抄默认值**（A123）：回落值由每节
        # dataclass 的字段默认唯一决定，与 `_model_entry` 同式（A114 定的口径）。
        # 「有哪些节」仍需要这份显式清单，因为 `models` / `default_model` 走上面那条
        # 特判路径、不吃 `_load_section`；清单与 `AppConfig` 的字段是否两集相等由
        # `tests/unit/test_config.py::TestSectionRegistryMatchesAppConfigFields` 对账。
        config_sections = [
            ('augmentation', AugmentationConfig),
            ('quality', QualityConfig),
            ('dedup', DedupConfig),
            ('export', ExportConfig),
            ('context', ContextConfig),
            ('versioning', VersioningConfig),
            ('sampler', SamplerConfig),
            ('expander', ExpanderConfig),
            ('tracker', TrackerConfig),
            ('visualization', VisualizationConfig),
            ('multilingual', MultilingualConfig),
            ('rag', RAGConfig),
            ('evaluation', EvaluationConfig),
            ('vector', VectorConfig),
            ('multimodal', MultimodalConfig),
            ('benchmark', BenchmarkConfig),
            ('active_learning', ActiveLearningConfig),
            ('frameworks', FrameworkConfig),
            ('web', WebConfig),
            ('logging', LoggingConfig),
        ]
        
        for key, config_class in config_sections:
            setattr(config, key, _load_section(raw_config, key, config_class))
        
        # 装配日志排在「写了没人读」出声**之前**：`logging.level: ERROR` 从此真的
        # 能静音那条 WARNING 通道（A97 与 A84 必须同屏读 —— 有反馈通道还得有旋钮）。
        # 刻意只对「文件里真的出现的 `logging` 节」动手，且只对它写出的键动手，
        # 理由见 `logging_setup` 模块 docstring；SDK 直构 `AppConfig()` 到这里一步
        # 都不发生，所以不传路径 / 不写节 = 两个面一字不变。
        raw_logging = raw_config.get('logging')
        if isinstance(raw_logging, dict):
            apply_logging_config(config.logging, written=set(raw_logging))
        
        _log_unread_keys(raw_config)
    
    return config


def get_model_config(config: AppConfig, model_name: Optional[str] = None) -> ModelConfig:
    """获取模型配置
    
    Args:
        config: 应用配置
        model_name: 模型名称，为 None 时使用默认模型
    
    Returns:
        ModelConfig 实例
    """
    name = model_name or config.default_model
    if name not in config.models:
        raise ConfigError(f"模型 '{name}' 未配置。可用模型: {list(config.models.keys())}")
    return config.models[name]


def save_config(config: AppConfig, config_path: str = "config.yaml") -> None:
    """将配置保存回 YAML 文件（仅持久化非敏感运行时字段；密钥保留占位符）

    有两件事必须保证，否则「保存 → 重新加载」会静默丢配置：

    1. **默认模型要写成 `models.default`**。`load_config` 只从这个键读默认模型
       （`config.default_model = raw_config['models'].get('default', 'ernie')`），
       写成顶层 `default_model` 下次加载时会被忽略并回落到 `ernie`。
    2. **文件里已经存在、而 `AppConfig` 不建模的顶层段落必须原样保留**。
       `ConfigValidator` 会把它们当作配置的一部分，直接整体覆盖等于删掉用户
       手写的段落。

    Args:
        config: 应用配置
        config_path: 配置文件路径
    """
    import dataclasses

    def _to_dict(obj):
        if dataclasses.is_dataclass(obj):
            return {k: _to_dict(v) for k, v in dataclasses.asdict(obj).items()}
        return obj

    data = _to_dict(config)

    # `logging` 节由用户手写，工具不代笔（A97）。装配的生效条件是「文件里真的写了
    # 这节」，而 `save_config` 一旦把三档默认值 dump 出去，下次加载就从「没写」变成
    # 「写了」—— 一条用户没要求的隐藏激活路径：API 侧会把 `basicConfig` 的
    # `%(name)s` 格式覆盖掉。下面保留既有段落的循环会原样带回文件里真有的这节，
    # 所以这个 pop 只挡「无中生有」，不丢用户手写的配置。`POST /api/config` 也不能
    # 改这节（它只认 augmentation/quality/dedup/export/vector/rag/multimodal 七节），
    # 于是这里没有任何会丢的写入路径。
    data.pop("logging", None)

    # 默认模型并入 models.default：这是 load_config 唯一认的键
    models = data.get("models") or {}
    models["default"] = data.pop("default_model", config.default_model)
    data["models"] = models

    # 保留既有文件中 AppConfig 不建模的顶层键
    existing = {}
    if os.path.exists(config_path):
        try:
            with open(config_path, "r", encoding="utf-8") as f:
                existing = yaml.safe_load(f) or {}
        except (OSError, yaml.YAMLError):
            existing = {}
        if not isinstance(existing, dict):
            existing = {}
        for key, value in existing.items():
            if key not in data:
                data[key] = value

    # 密钥不落盘：沿用文件里原有的 `${ENV_VAR}` 占位符。
    # 早前直接 pop 掉整个键，等于把 `api_key: ${BAIDU_API_KEY}` 这行删了——
    # 重新加载时该模型拿不到密钥，增强能力静默失效。而如果文件里写的是明文
    # 密钥，则一律丢弃（只保留环境变量引用，避免把明文写回磁盘）。
    existing_models = existing.get("models") if isinstance(existing.get("models"), dict) else {}
    for name, model_conf in data.get("models", {}).items():
        if name == "default" or not isinstance(model_conf, dict):
            continue
        previous = existing_models.get(name) if isinstance(existing_models.get(name), dict) else {}
        for secret in ("api_key", "secret_key"):
            placeholder = previous.get(secret, "")
            if isinstance(placeholder, str) and placeholder.startswith("${") and placeholder.endswith("}"):
                model_conf[secret] = placeholder
            else:
                model_conf.pop(secret, None)

    with open(config_path, "w", encoding="utf-8") as f:
        yaml.safe_dump(data, f, allow_unicode=True, sort_keys=False)


def apply_section_update(section: Any, updates: Dict[str, Any]) -> list:
    """把一批子键写进某个配置节，并让该节**自己的**运行时判据立刻复查（L73 / A117）。

    为什么必须有这个函数：L45–L72 七轮接上的判据一律住在各 dataclass 的
    `__post_init__` 里，而 `setattr` **不会再次触发它** —— 于是「先构造、后改字段」
    的写法整片绕过判据。全仓这样的写入点只有一处（`api/routes/config.py` 的
    `POST /api/config`，`load_config` 那一处走 `_load_section` 构造、判据照跑），
    它的链条实测三段都在（账本 L72 那条有完整复现口径）：
    `max_retry_wait` 默认 300.0 → `setattr(..., 9999)` 接受 → `save_config` 把
    `augmentation.max_retry_wait: 9999` 原样写进 YAML → 下次 `load_config` 抛
    `DataValidationError`。症状是这一族里最难诊断的一种：**当次请求返回成功、
    进程跑得好好的，服务重启后起不来**，而且那时已经没有一份「能改回来」的配置了。

    三个设计选择：

    1. **复查而不是另立判据**：跑的是该节 `__post_init__` 里那批 `require_*`，
       界仍然只住 `config.py` 一处（A77）。这里不新增第二条口径，也不抄第二份区间。
    2. **没有 `__post_init__` 的节照旧写入**：`export` / `vector` / `rag` /
       `multimodal` 四节至今没有运行时判据（只有校验器那一半），本函数不假装判了
       —— 那四节「运行时零判据」是 A118 余下的账，不在本轮扩面。（`quality` 与
       `dedup` 两节原本也在这份名单里，L76 / A118 给它们接上了判据；名单由
       `tests/integration/test_config_write_path_l73.py` 的精确集合棘轮钉住，
       一节接上一节就会红一次，所以这里不靠记忆维护。）
    3. **要么全落、要么全不落**：批次里任何一条被判负 ⇒ 已写的键逐个回滚到旧值再抛。
       不做回滚就会留下「内存里前几条已生效、磁盘一条都没写」的分叉，而端点的
       契约是 `success` 才代表保存过 —— 分叉正是本轮要修的那一类缺陷。

    Args:
        section: 配置节对象（`AugmentationConfig` / `QualityConfig` 这类 dataclass）
        updates: 要写入的子键映射

    Returns:
        被丢弃的未知键清单（`hasattr` 判不出来的那些），顺序与 `updates` 一致；
        出声与「是否想写 X」的建议仍归调用方，这里是纯判据 + 纯写入

    Raises:
        DataValidationError: 某个新值越界（来自该节的 `require_*`）；抛出时本节
            已回滚到调用前的状态
    """
    post_init = getattr(type(section), "__post_init__", None)
    applied = []
    ignored = []
    try:
        for key, value in updates.items():
            if not hasattr(section, key):
                ignored.append(key)
                continue
            applied.append((key, getattr(section, key)))
            setattr(section, key, value)
            if post_init is not None:
                post_init(section)
    except Exception:
        for key, old in reversed(applied):
            setattr(section, key, old)
        raise
    return ignored
