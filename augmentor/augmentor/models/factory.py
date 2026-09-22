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


def create_model_backend(config: ModelConfig, model_type: Optional[str] = None) -> ModelBackend:
    """创建模型后端实例
    
    Args:
        config: 模型配置
        model_type: 模型类型，为 None 时根据配置自动判断
    
    Returns:
        ModelBackend 实例
    
    Raises:
        ValueError: 不支持的模型类型
    """
    model_type = model_type or config.type
    
    if model_type == "baidu":
        return ERNIEBackend(config)
    elif model_type == "openai":
        return OpenAIBackend(config)
    elif model_type == "ollama":
        return OllamaBackend(config)
    elif model_type == "claude":
        return ClaudeBackend(config)
    elif model_type == "gemini":
        return GeminiBackend(config)
    else:
        raise ValueError(
            f"不支持的模型类型: {model_type}。"
            f"支持的类型: baidu, openai, ollama, claude, gemini"
        )
