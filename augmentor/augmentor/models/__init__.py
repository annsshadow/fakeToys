# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""模型后端模块"""

from .base import ModelBackend, extract_json_array
from .factory import create_model_backend
from .ernie import ERNIEBackend
from .openai_model import OpenAIBackend
from .ollama import OllamaBackend
from .claude import ClaudeBackend
from .gemini import GeminiBackend

__all__ = [
    "ModelBackend",
    "extract_json_array",
    "create_model_backend",
    "ERNIEBackend",
    "OpenAIBackend",
    "OllamaBackend",
    "ClaudeBackend",
    "GeminiBackend"
]
