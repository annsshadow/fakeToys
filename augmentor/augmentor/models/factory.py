# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""模型后端工厂"""

from typing import Optional
from .base import ModelBackend
from .ernie import ERNIEBackend
from .openai_model import OpenAIBackend
from .ollama import OllamaBackend
from .claude import ClaudeBackend
from .gemini import GeminiBackend
from ..config import ModelConfig
from ..exceptions import ConfigError


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
    model_type = model_type or config.type

    # 请求超时的两档在此合一：模型条目显式写了就用它，没写（None）才落到全局档。
    # 判「是 None」而不是判真假 —— `or` 会把用户显式写的 0 读成「没写」，
    # 而 0 正是 `requests` 当场抛 ValueError 的那个值（L33 的同一条禁令）。
    request_timeout = (
        config.request_timeout
        if config.request_timeout is not None else default_request_timeout
    )

    # 这些参数对所有后端语义一致，集中构造后透传，避免在 5 个分支里各写一遍
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

    if model_type == "baidu":
        return ERNIEBackend(config, **backend_kwargs)
    elif model_type == "openai":
        return OpenAIBackend(config, **backend_kwargs)
    elif model_type == "ollama":
        return OllamaBackend(config, **backend_kwargs)
    elif model_type == "claude":
        return ClaudeBackend(config, **backend_kwargs)
    elif model_type == "gemini":
        return GeminiBackend(config, **backend_kwargs)
    else:
        raise ConfigError(
            f"不支持的模型类型: {model_type}。"
            f"支持的类型: baidu, openai, ollama, claude, gemini"
        )
