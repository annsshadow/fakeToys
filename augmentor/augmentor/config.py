# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置管理模块"""

import logging
import os
import yaml
from dataclasses import dataclass, field
from typing import Any, Dict, Optional
from pathlib import Path
from .exceptions import ConfigError, DataValidationError
from .logging_setup import (apply_logging_config, assert_format_renderable,
                            level_number)
from .retry import MAX_RETRY_AFTER
from .validation import (require_count, require_ratio, require_seconds,
                         require_string, require_string_list)

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
        """模型条目的四个数值键逐个判（L71 / A74 那一根 + L72 / A113 采样三键）。

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
        """
        _reject_null_fields("models.<名字>", self,
                            ("temperature", "top_p", "max_output_tokens"))
        lo, hi = TEMPERATURE_RANGE
        require_ratio("models.<名字>.temperature", self.temperature,
                      minimum=lo, maximum=hi)
        lo, hi = TOP_P_RANGE
        require_ratio("models.<名字>.top_p", self.top_p, minimum=lo, maximum=hi)
        require_count("models.<名字>.max_output_tokens", self.max_output_tokens,
                      minimum=MAX_OUTPUT_TOKENS_MIN)
        lo, hi = REQUEST_TIMEOUT_RANGE
        require_seconds("models.<名字>.request_timeout", self.request_timeout,
                        minimum=lo, maximum=hi)


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


@dataclass
class DedupConfig:
    """去重配置"""
    enabled: bool = True
    threshold: float = 0.9


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


def _load_section(raw_config: Dict, key: str, config_class: type, defaults: Dict) -> Any:
    """加载配置节
    
    Args:
        raw_config: 原始配置字典
        key: 配置节名称
        config_class: 配置类
        defaults: 默认值字典
    
    Returns:
        配置实例
    """
    if key not in raw_config:
        return config_class(**defaults)

    conf = raw_config[key]
    # 只写一个节名、下面什么都没有（`logging:`）时 YAML 给的是 `None`，与 A94 里
    # 0 字节文件同形；`logging: app.log` 这种「把节当值写」给的是标量。两种过去都
    # 在 `conf.get` 上抛 `AttributeError: 'NoneType' object has no attribute 'get'`
    # / `'str' object has no attribute 'get'`（实测 Temp `l57/probe1.txt` P4/P5：
    # **每一节**都同形，不止 logging），用户拿到的是无法行动的栈。语义与 A94 一致：
    # 写了节名而什么都没写 = 该节全默认；写成标量则是摆错了形状，明说。
    if conf is None:
        return config_class(**defaults)
    if not isinstance(conf, dict):
        raise ConfigError(
            f"{key} 必须是「键: 值」的映射，当前是 {conf!r}"
            f"（{type(conf).__name__}）"
        )

    kwargs = {}
    for param_name, default_value in defaults.items():
        kwargs[param_name] = conf.get(param_name, default_value)
    
    return config_class(**kwargs)


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
        
        # 解析环境变量
        def resolve_env(value):
            return _resolve_env(value)
        
        # 加载模型配置
        # `models` 走这条特判路径、绕开了 `_load_section` 的形状守卫（A101 当年只
        # 护住映射表那 20 节）。于是 `models:` 写成标量、或某个模型条目写成标量 /
        # 空 / 列表时，过去会当场崩在 `model_conf.get` 的 `AttributeError`，而校验
        # 面却报 `is_valid=False` —— 同族缺陷两侧不同判（A85，A101 的漏网）。这里补
        # 回与 `_load_section` 逐字同口径的判据：只写名字没给内容（None）= 全默认；
        # 写成非映射则抛可行动的 ConfigError。
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
            config.default_model = models_raw.get('default', 'ernie')
            for name, model_conf in models_raw.items():
                if name == 'default':
                    continue
                conf = _as_mapping(model_conf, f"models.{name}")
                config.models[name] = ModelConfig(
                    type=conf.get('type', ''),
                    api_key=resolve_env(conf.get('api_key', '')),
                    secret_key=resolve_env(conf.get('secret_key', '')),
                    base_url=resolve_env(conf.get('base_url', '')),
                    model=conf.get('model', ''),
                    temperature=conf.get('temperature', 0.99),
                    top_p=conf.get('top_p', 0.95),
                    max_output_tokens=conf.get('max_output_tokens', 2048),
                    # 键写了没给值时 YAML 给 None：对 `request_timeout` 这是合法读法
                    # （= 不覆盖全局档），对采样三键则不是，所以 `temperature` /
                    # `top_p` / `max_output_tokens` 三行**不**能照抄这一读法 —— 它们
                    # 走的是带默认值的 `conf.get(key, 默认)`，写了键没给值时拿到的是
                    # None 而不是默认值，坏值与 null 都在下面构造 `ModelConfig` 时
                    # 当场判（`__post_init__`，L71 / A74 + L72 / A113）。
                    request_timeout=conf.get('request_timeout')
                )
        
        # 使用映射表加载其他配置（减少重复代码）
        config_sections = [
            ('augmentation', AugmentationConfig, {
                'variants_per_seed': 5, 'num_threads': 40, 'auto_save_interval': 10,
                'max_retries': 3, 'retry_delay': 1.0,
                'max_retry_wait': 300.0, 'retry_jitter': 0.0,
                'request_timeout': 120.0
            }),
            ('quality', QualityConfig, {
                'enabled': True, 'threshold': 0.6, 'weights': [0.3, 0.4, 0.3]
            }),
            ('dedup', DedupConfig, {'enabled': True, 'threshold': 0.9}),
            ('export', ExportConfig, {
                'default_format': 'jsonl',
                'formats': ['jsonl', 'llama_factory', 'alpaca', 'sharegpt', 'chatml']
            }),
            ('context', ContextConfig, {'enabled': False, 'num_turns': 3}),
            ('versioning', VersioningConfig, {
                'enabled': True, 'storage_dir': 'data/versions', 'auto_snapshot': True
            }),
            ('sampler', SamplerConfig, {
                'enabled': False,
                'dimensions': ['topic', 'question_type', 'length', 'complexity']
            }),
            ('expander', ExpanderConfig, {
                'enabled': False, 'strategies': ['similar', 'related', 'scenario']
            }),
            ('tracker', TrackerConfig, {
                'enabled': False,
                'metrics': ['train_loss', 'eval_accuracy', 'perplexity']
            }),
            ('visualization', VisualizationConfig, {
                'enabled': True,
                'types': ['wordcloud', 'length_distribution', 'topic_cluster', 'timeline', 'quality_distribution']
            }),
            ('multilingual', MultilingualConfig, {
                'enabled': False, 'default_target_lang': 'en',
                'supported_langs': ['zh', 'en'], 'translate_batch_size': 10
            }),
            ('rag', RAGConfig, {
                'enabled': False, 'default_format': 'llamaindex',
                'chunk_size': 512, 'chunk_overlap': 64
            }),
            ('evaluation', EvaluationConfig, {
                'enabled': False, 'metrics': ['bleu', 'rouge_l', 'similarity'],
                'reference_field': 'output'
            }),
            ('vector', VectorConfig, {
                'enabled': False, 'backend': 'faiss', 'dimension': 384,
                'storage_dir': 'data/vectors', 'collection': 'default'
            }),
            ('multimodal', MultimodalConfig, {
                'enabled': False,
                'image_extensions': ['.jpg', '.jpeg', '.png', '.bmp', '.webp'],
                'audio_extensions': ['.wav', '.mp3', '.flac', '.ogg', '.m4a']
            }),
            ('benchmark', BenchmarkConfig, {
                'enabled': False, 'baseline_file': 'data/benchmark_baseline.json',
                'metrics': ['pass_rate', 'avg_total_score', 'diversity', 'duplication_rate']
            }),
            ('active_learning', ActiveLearningConfig, {
                'enabled': False, 'strategy': 'uncertainty',
                'batch_size': 50, 'max_iterations': 10
            }),
            ('frameworks', FrameworkConfig, {
                'enabled': False, 'frameworks': ['langchain', 'llamaindex']
            }),
            ('web', WebConfig, {
                'port': 8000, 'host': '0.0.0.0', 'static_dir': 'web/dist',
                'cors_origins': [], 'cors_credentials': False,
                'data_roots': ['data'],
                'rate_limit_max_requests': 300,
                'rate_limit_window_seconds': 60.0,
                'rate_limit_exempt_paths': [
                    '/api/health', '/docs', '/redoc', '/openapi.json'
                ],
            }),
            ('logging', LoggingConfig, {
                'level': 'WARNING', 'file': '',
                'format': '%(message)s'
            }),
        ]
        
        for key, config_class, defaults in config_sections:
            setattr(config, key, _load_section(raw_config, key, config_class, defaults))
        
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
