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

    Returns:
        ModelBackend 实例

    Raises:
        ConfigError: 不支持的模型类型
    """
    model_type = model_type or config.type

    # 三个缓存参数对所有后端语义一致，集中构造后透传，避免在 5 个分支里各写一遍
    cache_kwargs = {
        "response_cache_dir": response_cache_dir,
        "response_cache_ttl": response_cache_ttl,
        "response_cache_max_bytes": response_cache_max_bytes,
    }

    if model_type == "baidu":
        return ERNIEBackend(config, **cache_kwargs)
    elif model_type == "openai":
        return OpenAIBackend(config, **cache_kwargs)
    elif model_type == "ollama":
        return OllamaBackend(config, **cache_kwargs)
    elif model_type == "claude":
        return ClaudeBackend(config, **cache_kwargs)
    elif model_type == "gemini":
        return GeminiBackend(config, **cache_kwargs)
    else:
        raise ConfigError(
            f"不支持的模型类型: {model_type}。"
            f"支持的类型: baidu, openai, ollama, claude, gemini"
        )
