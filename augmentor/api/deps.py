"""API 共享依赖

集中管理管道单例、文件读写与异步执行辅助。
"""

import asyncio
import json
import logging
from pathlib import Path
from typing import Any, Callable, List, Optional

from fastapi import HTTPException

from augmentor import AugmentorPipeline, load_config

logger = logging.getLogger(__name__)

# 全局管道实例
_pipeline: Optional[AugmentorPipeline] = None


def get_pipeline() -> AugmentorPipeline:
    """获取管道实例（惰性初始化）

    Returns:
        AugmentorPipeline 实例
    """
    global _pipeline
    if _pipeline is None:
        config = load_config("config.yaml")
        _pipeline = AugmentorPipeline(config)
    return _pipeline


def reset_pipeline():
    """重置管道实例（主要供测试使用）"""
    global _pipeline
    _pipeline = None


def _sync_read_json(file_path: Path) -> list:
    """同步读取 JSON 文件

    Args:
        file_path: 文件路径

    Returns:
        数据列表
    """
    with open(file_path, 'r', encoding='utf-8') as f:
        return json.load(f)


def _sync_write_json(file_path: Path, data: list):
    """同步写入 JSON 文件

    Args:
        file_path: 文件路径
        data: 数据列表
    """
    file_path.parent.mkdir(parents=True, exist_ok=True)
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, separators=(',', ':'))


async def read_json_file(file_path: Path) -> list:
    """异步读取 JSON 文件

    Args:
        file_path: 文件路径

    Returns:
        数据列表
    """
    loop = asyncio.get_event_loop()
    return await loop.run_in_executor(None, _sync_read_json, file_path)


async def write_json_file(file_path: Path, data: list):
    """异步写入 JSON 文件

    Args:
        file_path: 文件路径
        data: 数据列表
    """
    loop = asyncio.get_event_loop()
    await loop.run_in_executor(None, _sync_write_json, file_path, data)


async def run_in_thread(func: Callable[..., Any], *args, **kwargs) -> Any:
    """在线程池中执行阻塞函数

    Args:
        func: 待执行函数
        *args: 位置参数
        **kwargs: 关键字参数

    Returns:
        函数返回值
    """
    loop = asyncio.get_event_loop()
    return await loop.run_in_executor(None, lambda: func(*args, **kwargs))


def require_file(filename: str) -> Path:
    """校验文件存在并返回路径

    Args:
        filename: 文件名或路径

    Returns:
        Path 实例

    Raises:
        HTTPException: 文件不存在
    """
    file_path = Path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")
    return file_path


def load_items(filename: str) -> List[dict]:
    """同步加载数据文件（供线程内使用）

    Args:
        filename: 文件路径

    Returns:
        数据列表
    """
    return _sync_read_json(require_file(filename))


def save_items(filename: str, items: List[dict]):
    """同步保存数据文件（供线程内使用）

    Args:
        filename: 文件路径
        items: 数据列表
    """
    _sync_write_json(Path(filename), items)
