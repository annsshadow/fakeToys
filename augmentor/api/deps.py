# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""API 共享依赖

集中管理管道单例、文件读写、路径校验与异步执行辅助。
"""

import asyncio
import json
import logging
import os
import secrets
from pathlib import Path
from typing import Any, Callable, List, Optional

from fastapi import Header, HTTPException

from augmentor import AugmentorPipeline, load_config

logger = logging.getLogger(__name__)

# 全局管道实例
_pipeline: Optional[AugmentorPipeline] = None

# 写操作鉴权使用的环境变量名
API_KEY_ENV = "AUGMENTOR_API_KEY"


def api_key_required() -> bool:
    """是否启用了写操作鉴权

    Returns:
        设置了 AUGMENTOR_API_KEY 时为 True
    """
    return bool(os.environ.get(API_KEY_ENV, "").strip())


def verify_api_key(x_api_key: Optional[str] = Header(None, alias="X-API-Key")) -> None:
    """校验写操作的 API Key

    未设置 ``AUGMENTOR_API_KEY`` 时不做校验——保持零配置即可用的既有行为，
    由启动日志给出告警；一旦设置，所有写操作必须携带匹配的 ``X-API-Key``。

    Args:
        x_api_key: 请求头 X-API-Key

    Raises:
        HTTPException: 401 未提供或密钥不匹配
    """
    expected = os.environ.get(API_KEY_ENV, "").strip()
    if not expected:
        return

    if not x_api_key or not secrets.compare_digest(x_api_key, expected):
        raise HTTPException(status_code=401, detail="缺少或无效的 X-API-Key")


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


def _allowed_roots() -> List[Path]:
    """解析允许访问的数据目录白名单

    来源优先级：
      1. 环境变量 ``AUGMENTOR_DATA_ROOTS``（``os.pathsep`` 分隔）——便于容器与测试覆盖
      2. ``config.yaml`` 的 ``web.data_roots``
      3. 默认 ``["."]``，即进程工作目录

    Returns:
        已 resolve 的根目录列表
    """
    raw = os.environ.get("AUGMENTOR_DATA_ROOTS")
    if raw:
        candidates = [p.strip() for p in raw.split(os.pathsep) if p.strip()]
    else:
        try:
            candidates = [str(p) for p in load_config("config.yaml").web.data_roots]
        except Exception:  # 配置损坏不应让所有请求 500，退化为工作目录
            logger.warning("读取 web.data_roots 失败，回退到工作目录", exc_info=True)
            candidates = ["."]

    if not candidates:
        candidates = ["."]

    return [Path(c).resolve() for c in candidates]


def _assert_within_roots(path: Path) -> Path:
    """校验路径落在白名单根目录内

    Args:
        path: 已 resolve 的绝对路径

    Returns:
        原路径

    Raises:
        HTTPException: 403 路径越界
    """
    for root in _allowed_roots():
        try:
            path.relative_to(root)
            return path
        except ValueError:
            continue
    raise HTTPException(status_code=403, detail="路径超出允许的数据目录范围")


def resolve_within_roots(name: str, label: str) -> Path:
    """把客户端传入的路径规范化为白名单内的绝对路径

    相对路径按进程工作目录解析（与既有行为一致）；``..`` 组件直接拒绝。
    ``Path.resolve()`` 会展开符号链接，因此指向根目录之外的软链接同样被拦下。

    Args:
        name: 客户端传入的路径
        label: 出错信息中使用的字段名

    Returns:
        已 resolve 的绝对路径

    Raises:
        HTTPException: 400 参数非法；403 路径越界
    """
    if not isinstance(name, str) or not name.strip():
        raise HTTPException(status_code=400, detail=f"{label} 不能为空")

    raw = Path(name.strip())
    if ".." in raw.parts:
        raise HTTPException(status_code=400, detail="路径包含非法组件")

    candidate = (raw if raw.is_absolute() else Path.cwd() / raw).resolve()
    return _assert_within_roots(candidate)


def resolve_data_path(name: str, *, for_write: bool = False) -> Path:
    """解析并校验数据文件路径

    Args:
        name: 客户端传入的文件路径
        for_write: True 表示用于写入，不要求文件已存在

    Returns:
        已 resolve 的绝对路径

    Raises:
        HTTPException: 400 参数非法；403 路径越界；404 读操作时文件不存在
    """
    candidate = resolve_within_roots(name, "文件路径")

    if not for_write and not candidate.is_file():
        raise HTTPException(status_code=404, detail="文件不存在")

    return candidate


def resolve_data_dir(name: str) -> Path:
    """解析并校验目录路径（读写通用）

    Args:
        name: 客户端传入的目录路径

    Returns:
        已 resolve 的绝对路径

    Raises:
        HTTPException: 400 参数非法；403 路径越界
    """
    return resolve_within_roots(name, "目录路径")


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
    """校验文件存在、位于白名单目录内并返回路径

    Args:
        filename: 文件名或路径

    Returns:
        Path 实例

    Raises:
        HTTPException: 400 参数非法；403 路径越界；404 文件不存在
    """
    return resolve_data_path(filename)


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
    _sync_write_json(resolve_data_path(filename, for_write=True), items)
