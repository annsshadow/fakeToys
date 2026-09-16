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
