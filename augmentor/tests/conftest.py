# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""pytest 全局配置

确保测试能直接 import augmentor / api，并提供共享测试数据。
"""

import os
import sys
from pathlib import Path

import pytest

AI_DIR = Path(__file__).resolve().parent.parent
if str(AI_DIR) not in sys.path:
    sys.path.insert(0, str(AI_DIR))

# 模型后端在构造阶段即校验密钥存在，这里提供占位值避免真实调用
os.environ.setdefault("BAIDU_API_KEY", "test-api-key")
os.environ.setdefault("BAIDU_SECRET_KEY", "test-secret-key")
os.environ.setdefault("OPENAI_API_KEY", "test-openai-key")


@pytest.fixture(autouse=True)
def force_fallback_model(monkeypatch):
    """强制模型管理器使用 fallback，避免测试期下载模型

    Args:
        monkeypatch: pytest 内置 fixture
    """
    from augmentor.model_manager import model_manager

    monkeypatch.setattr(model_manager, "_sentence_model", "fallback", raising=False)
    yield


@pytest.fixture(autouse=True)
def allow_temp_data_roots(monkeypatch):
    """把系统临时目录并入 API 的路径白名单

    `api.deps` 默认只允许 `web.data_roots`（进程工作目录）内的路径，
    越界一律 403。而测试大量使用 `tmp_path`（位于系统临时目录下）传递
    绝对路径，因此这里显式放宽白名单，使测试无需改动即可覆盖到白名单
    逻辑之外的分支。

    生产环境不受影响：那里不会设置 `AUGMENTOR_DATA_ROOTS`，
    白名单仍由 `config.yaml` 的 `web.data_roots` 决定。

    Args:
        monkeypatch: pytest 内置 fixture
    """
    import tempfile

    roots = [os.getcwd(), tempfile.gettempdir()]
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", os.pathsep.join(roots))
    yield


@pytest.fixture(autouse=True)
def reset_rate_limiter():
    """逐用例重置限流计数

    TestClient 的 `request.client.host` 恒为 "testclient"，整个测试会话共享
    同一个限流键。若不重置，累积请求数迟早会突破窗口上限并让后续用例拿到 429。
    重置后每个用例从零开始，限流逻辑本身仍由专门的用例覆盖。

    Yields:
        None
    """
    from api.main import rate_limiter

    rate_limiter.reset()
    yield
    rate_limiter.reset()


@pytest.fixture
def sample_items():
    """基础样本数据

    Returns:
        数据列表
    """
    return [
        {
            "instruction": "如何申请入住安居乐寓？",
            "input": "",
            "output": "您可以通过官方 App 提交申请，上传身份证明后等待审核。"
        },
        {
            "instruction": "房租多少钱一个月？",
            "input": "",
            "output": "租金根据房型和地段不同，具体可在 App 内查看实时报价。"
        },
        {
            "instruction": "如何申请入住安居乐寓？",
            "input": "",
            "output": "您可以通过官方 App 提交入住申请。"
        },
        {
            "instruction": "退租流程是什么？",
            "input": "",
            "output": "退租需提前 30 天在 App 内提交申请，并完成房屋验收。"
        },
        {
            "instruction": "为什么我的申请被拒绝了？",
            "input": "",
            "output": "常见原因是资料不完整或信用评估未通过，可补充材料后重新提交。"
        }
    ]


@pytest.fixture
def tmp_data_file(tmp_path, sample_items):
    """写入临时数据文件

    Returns:
        (文件路径字符串, 数据列表)
    """
    import json

    path = tmp_path / "train_data_test.json"
    with open(path, 'w', encoding='utf-8') as f:
        json.dump(sample_items, f, ensure_ascii=False)
    return str(path), sample_items
