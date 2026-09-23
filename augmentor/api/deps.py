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
from typing import Any, Callable, List, Optional, Tuple

from fastapi import Header, HTTPException

from augmentor import AugmentorPipeline, load_config
from augmentor.config import WebConfig

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


def allowed_data_roots() -> List[Path]:
    """解析允许访问的数据目录白名单

    来源优先级：
      1. 环境变量 ``AUGMENTOR_DATA_ROOTS``（``os.pathsep`` 分隔）——便于容器与测试覆盖
      2. ``config.yaml`` 的 ``web.data_roots``
      3. 出厂默认 ``WebConfig.data_roots``（当前为 ``["data"]``）

    Returns:
        已 resolve 的根目录列表
    """
    raw = os.environ.get("AUGMENTOR_DATA_ROOTS")
    if raw:
        candidates = [p.strip() for p in raw.split(os.pathsep) if p.strip()]
    else:
        try:
            candidates = [str(p) for p in load_config("config.yaml").web.data_roots]
        except Exception:  # 配置损坏不应让所有请求 500
            logger.warning("读取 web.data_roots 失败，回退到出厂默认", exc_info=True)
            candidates = []

    if not candidates:
        # 空白名单等于「所有请求都 403」，比放宽更危险，因此两级降级都落到出厂默认：
        # 绝不落到工作目录——那会把出厂默认刚刚收紧掉的范围又悄悄放开。
        candidates = [str(p) for p in WebConfig().data_roots]

    return [Path(c).resolve() for c in candidates]


def _assert_within_roots(path: Path, roots: List[Path]) -> Path:
    """校验路径落在白名单根目录内

    Args:
        path: 已 resolve 的绝对路径
        roots: 已 resolve 的根目录列表（与候选生成共用同一份快照）

    Returns:
        原路径

    Raises:
        HTTPException: 403 路径越界
    """
    for root in roots:
        try:
            path.relative_to(root)
            return path
        except ValueError:
            continue
    raise HTTPException(status_code=403, detail="路径超出允许的数据目录范围")


def _relative_candidates(raw: Path, roots: List[Path]) -> List[Path]:
    """为相对路径生成候选绝对路径，**顺序即优先级**

    白名单根目录在前，进程工作目录在最后兜底。这么排是为收紧出厂默认
    ``["data"]`` 补的必要一环：``/api/data/list`` 返回的 ``name`` 只是文件名，
    前端 8 处都把它原样拼回 ``/api/data/load/{filename}``，而 ``{filename}``
    只匹配单个路径段——若只认 ``data/xxx.json`` 这种写法，整个文件选择器会
    集体 403。候选路径要么在白名单内、要么仍要过 ``_assert_within_roots``，
    因此不放宽可达范围。

    Args:
        raw: 客户端传入的相对路径（已确认不含 ``..``）
        roots: 已 resolve 的白名单根目录

    Returns:
        去重后的候选列表；最后一项恒为工作目录解释
    """
    candidates = []
    for root in roots:
        candidate = (root / raw).resolve()
        if candidate not in candidates:
            candidates.append(candidate)

    fallback = (Path.cwd() / raw).resolve()
    if fallback not in candidates:
        candidates.append(fallback)
    return candidates


def resolve_within_roots(name: str, label: str) -> Path:
    """把客户端传入的路径规范化为白名单内的绝对路径

    相对路径按 ``_relative_candidates`` 的顺序取**第一个存在**的解释，白名单
    根目录优先于工作目录。一个都不存在时（典型是写入新文件）落回工作目录解释，
    由后续的存在性检查 404 或白名单闸 403 ——**不**替调用方凭空挑一个写入目录，
    那种猜测会静默改掉产物落点。要写入请显式传 ``data/xxx.json``。

    ``..`` 组件直接拒绝。``Path.resolve()`` 会展开符号链接，因此指向根目录
    之外的软链接同样被拦下。

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

    # 一次解析、两处共用：候选顺序与越界判定必须基于同一份白名单快照
    roots = allowed_data_roots()
    if raw.is_absolute():
        return _assert_within_roots(raw.resolve(), roots)

    candidates = _relative_candidates(raw, roots)
    chosen = next((c for c in candidates if c.exists()), candidates[-1])
    return _assert_within_roots(chosen, roots)


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


def to_http_error(
    exc: Exception,
    *,
    not_found: str = "文件不存在",
    not_found_types: Tuple[type, ...] = (),
) -> HTTPException:
    """把下游抛出的异常映射成 HTTP 错误

    统一三件事，避免每个 handler 各写一套 ``except`` 分支：

    1. ``FileNotFoundError`` → 404（``not_found`` 可定制文案，如「备份不存在」）；
    2. ``JSONDecodeError`` → 400，且**不转发 json 模块的英文原文**；
    3. 其余 ``ValueError`` → 400（参数或数据不合法），其它 → 500。

    顺序有讲究：``JSONDecodeError`` 是 ``ValueError`` 的子类，必须先判。
    ``HTTPException`` **不**在这里处理 —— 它应当由调用方原样抛出，否则会被
    降级成 500。

    之所以把 400/500 的判据收在一处：此前两条路径是分散的，同一个
    「文件不是合法 JSON」在有的端点是 400、有的端点是 500，且都把
    ``"Expecting property name enclosed in double quotes: line 1 column 3"``
    这种解析器内部措辞原样回给了客户端。

    Args:
        exc: 下游抛出的异常
        not_found: 判定为「资源不存在」时使用的文案
        not_found_types: 额外视为「资源不存在」的异常类型。用于语义明确、
            但不是 ``FileNotFoundError`` 的情况（如 ``BackupError`` 唯一
            的抛出点就是「备份不存在」）。这些类型同样是 ``ValueError``
            子类，因此必须排在 ``ValueError`` 分支之前判定。

    Returns:
        对应的 HTTPException
    """
    if isinstance(exc, FileNotFoundError) or (
        not_found_types and isinstance(exc, not_found_types)
    ):
        return HTTPException(status_code=404, detail=not_found)
    if isinstance(exc, json.JSONDecodeError):
        # 只回传行列号：足够定位，又不泄漏实现细节。
        return HTTPException(
            status_code=400,
            detail=f"数据文件不是合法 JSON（第 {exc.lineno} 行第 {exc.colno} 列）",
        )
    if isinstance(exc, ValueError):
        return HTTPException(status_code=400, detail=str(exc))
    return HTTPException(status_code=500, detail=str(exc))


_JSON_KIND_NAMES = {
    dict: "对象", list: "数组", str: "字符串", int: "数字", float: "数字",
    bool: "布尔值", type(None): "空值",
}


def _json_kind(value) -> str:
    """JSON 值的类型中文名，用于「顶层形态不对」的 400 文案"""
    return _JSON_KIND_NAMES.get(type(value), type(value).__name__)


def read_items(file_path: Path) -> list:
    """同步读取**已 resolve** 的 JSON 数据集文件

    与 ``load_items`` 的分工：``load_items`` 收客户端传入的**名字**（负责解析与
    白名单校验），本函数收**已经 resolve 过的路径**——供「一次校验、多次读取」
    的路由复用（数据集对比要读两个文件，聚合要读任意多个）。

    Args:
        file_path: 已 resolve 的绝对路径

    Returns:
        数据列表

    Raises:
        HTTPException: 400 文件不是合法 JSON；或顶层不是数组 / 数组元素不是对象

    形态校验放在这里而不是各调用点，是因为下游一律按 `item.get(...)` 取字段：
    顶层是对象时拿到的是键（字符串），元素是标量时拿到标量，都会深入到
    `augmentor/` 内部才炸出 `'str' object has no attribute 'keys'` 这类
    AttributeError，变成 500 并把 Python 内部措辞回给客户端。
    唯一例外是 `/api/dataset/validate`——它的职责就是报告畸形数据项，
    因此它不经本函数，自己走 `DatasetValidator.validate_file`。
    """
    try:
        with open(file_path, "r", encoding="utf-8") as f:
            data = json.load(f)
    except json.JSONDecodeError as e:
        raise to_http_error(e) from e

    if not isinstance(data, list):
        raise HTTPException(
            status_code=400,
            detail=f"数据文件的顶层必须是 JSON 数组，当前是{_json_kind(data)}",
        )
    for index, item in enumerate(data):
        if not isinstance(item, dict):
            raise HTTPException(
                status_code=400,
                detail=(
                    f"数据文件的第 {index + 1} 个数据项必须是 JSON 对象，"
                    f"当前是{_json_kind(item)}"
                ),
            )
    return data


async def read_json_file(file_path: Path) -> list:
    """异步读取 JSON 文件

    Args:
        file_path: 文件路径

    Returns:
        数据列表

    Raises:
        HTTPException: 400 文件不是合法 JSON
    """
    loop = asyncio.get_event_loop()
    return await loop.run_in_executor(None, read_items, file_path)


def _sync_write_json(file_path: Path, data: list):
    """同步写入 JSON 文件

    Args:
        file_path: 文件路径
        data: 数据列表
    """
    file_path.parent.mkdir(parents=True, exist_ok=True)
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, separators=(',', ':'))


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

    Raises:
        HTTPException: 400 参数非法 / 文件不是合法 JSON；403 路径越界；404 文件不存在
    """
    return read_items(require_file(filename))


def save_items(filename: str, items: List[dict]):
    """同步保存数据文件（供线程内使用）

    Args:
        filename: 文件路径
        items: 数据列表
    """
    _sync_write_json(resolve_data_path(filename, for_write=True), items)
