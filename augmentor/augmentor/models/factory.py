# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""模型后端工厂"""

from typing import Dict, Optional
from .base import ModelBackend
from .ernie import ERNIEBackend
from .openai_model import OpenAIBackend
from .ollama import OllamaBackend
from .claude import ClaudeBackend
from .gemini import GeminiBackend
from ..config import MODEL_TYPES, ModelConfig
from ..exceptions import ConfigError

# 类型名 → 后端类（A115 / L74）。改前是五支 `if/elif` 串，每支把类型名抄两遍
# （一支判、一句文案），于是「合法取值清单」在运行时与错误文案里各存一份、
# 与静态规格表又是第三份。换成表之后文案从 `MODEL_TYPES` 生成，由
# `tests/unit/test_model_type_choices_l74.py` 钉住「表的键集 == `MODEL_TYPES`」，
# 三处塌成一处定义 + 一个断言。
MODEL_BACKENDS: Dict[str, type] = {
    "baidu": ERNIEBackend,
    "openai": OpenAIBackend,
    "ollama": OllamaBackend,
    "claude": ClaudeBackend,
    "gemini": GeminiBackend,
}


def create_model_backend(
    config: ModelConfig,
    model_type: Optional[str] = None,
    response_cache_dir: Optional[str] = None,
    response_cache_ttl: Optional[float] = None,
    response_cache_max_bytes: Optional[int] = None,
    default_attempts: Optional[int] = None,
    default_retry_delay: Optional[float] = None,
    default_max_retry_wait: Optional[float] = None,
    default_retry_jitter: Optional[float] = None,
    default_request_timeout: Optional[float] = None,
) -> ModelBackend:
    """创建模型后端实例

    Args:
        config: 模型配置
        model_type: 模型类型，为 None 时根据配置自动判断
        response_cache_dir: 磁盘响应缓存目录。**默认 None 即不启用**——
            启用意味着把 prompt 与模型响应写入磁盘，属于需要显式选择的行为
        response_cache_ttl: 磁盘缓存生存时间（秒），None 表示不按时间过期
        response_cache_max_bytes: 磁盘缓存容量上限（字节），
            None 时用 ModelBackend.DEFAULT_RESPONSE_CACHE_MAX_BYTES
        default_attempts: 后端的重试默认档位（总尝试次数），
            对应配置文件 `augmentation.max_retries`
        default_retry_delay: 后端的退避基数默认值（秒），
            对应配置文件 `augmentation.retry_delay`
        default_max_retry_wait: 后端「服务端 `Retry-After` 那一支」的等待上限（秒），
            对应配置文件 `augmentation.max_retry_wait`；只能夹小不能放大
        default_retry_jitter: 后端的退避抖动比例（0-1），
            对应配置文件 `augmentation.retry_jitter`
        default_request_timeout: 后端单次请求超时（秒），
            对应配置文件 `augmentation.request_timeout`；
            `config.request_timeout`（模型条目里的 `models.<名字>.request_timeout`）
            存在时**优先于本参数** —— 全局档 + 每模型覆盖，与重试两旋钮的口径一致

    Returns:
        ModelBackend 实例

    Raises:
        ConfigError: 不支持的模型类型
        DataValidationError: default_attempts / default_retry_delay /
            default_max_retry_wait / default_retry_jitter / default_request_timeout
            越界
    """
    # 与下面 `request_timeout` 同一口径：哨兵是 `None`，不是「任何假值」。
    # 改前 `model_type or config.type` 会把调用方显式写的 `''` / `0` 读成「没传」，
    # 于是它不报错，而是**静默换成配置里那个类型**去建后端 —— 症状是「我明明传了
    # 空类型，怎么建出来一个 baidu」。判 `is None` 之后这类值才进得下面的清单判据，
    # 与本函数 Args 所述「为 None 时根据配置自动判断」一致。（A115 / L74 的副作用面：
    # 清单要生效，先得让假值进得来。）
    model_type = model_type if model_type is not None else config.type

    # 请求超时的两档在此合一：模型条目显式写了就用它，没写（None）才落到全局档。
    # 判「是 None」而不是判真假 —— `or` 会把用户显式写的 0 读成「没写」，
    # 而 0 正是 `requests` 当场抛 ValueError 的那个值（L33 的同一条禁令）。
    request_timeout = (
        config.request_timeout
        if config.request_timeout is not None else default_request_timeout
    )

    # 这些参数对所有后端语义一致，集中构造后一次透传，不在按类型分发的分支里重复
    backend_kwargs = {
        "response_cache_dir": response_cache_dir,
        "response_cache_ttl": response_cache_ttl,
        "response_cache_max_bytes": response_cache_max_bytes,
        "default_attempts": default_attempts,
        "default_retry_delay": default_retry_delay,
        "default_max_retry_wait": default_max_retry_wait,
        "default_retry_jitter": default_retry_jitter,
        "default_request_timeout": request_timeout,
    }

    # `isinstance` 那一层是保 parity 而不是防御：改前的 `==` 链对不可哈希的入参
    # （有人把 `model_type` 传成 list/dict）只会一路判假、落到 `ConfigError`；
    # 裸 `dict.get` 则会先炸一个 `TypeError: unhashable type`，把「配置里类型写错」
    # 的症状换成栈跟踪。
    backend_cls = (
        MODEL_BACKENDS.get(model_type) if isinstance(model_type, str) else None
    )
    if backend_cls is None:
        raise ConfigError(
            f"不支持的模型类型: {model_type}。"
            f"支持的类型: {', '.join(MODEL_TYPES)}"
        )
    return backend_cls(config, **backend_kwargs)
