# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 26 轮：httpx 必须声明在 requirements-core.txt。

`use_async=True` 时 5 个模型后端（openai/ollama/ernie/gemini/claude）都走
`httpx.AsyncClient`；此前 httpx 只声明在 requirements-dev.txt（测试链），
Docker 默认镜像（core）缺它 → 每次异步调用回退线程池并打 warning，
「开异步」在 core 镜像里等于白开。本测试防止它再被挪回 dev-only。
"""

from pathlib import Path

REQS_CORE = Path(__file__).resolve().parents[2] / "requirements-core.txt"


def test_httpx_is_declared_in_core_requirements():
    text = REQS_CORE.read_text(encoding="utf-8")
    assert "httpx" in text, "requirements-core.txt 必须声明 httpx（异步引擎的运行时依赖）"
