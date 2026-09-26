# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据管理 API 路由"""

import asyncio
import json
from pathlib import Path
from typing import Any, Dict, List

from fastapi import APIRouter, Depends, HTTPException, UploadFile, File
from pydantic import BaseModel

from ..deps import (
    allowed_data_roots,
    assert_dataset_shape,
    get_pipeline,
    max_upload_bytes,
    read_json_file,
    resolve_within_roots,
    run_in_thread,
    to_http_error,
    verify_api_key,
    write_json_file,
)
from ..schemas import SuccessResponse

router = APIRouter(tags=["data"])


class DataFileInfo(BaseModel):
    """单个数据文件条目"""
    name: str
    path: str
    size: int


class DataFileListResponse(BaseModel):
    """数据文件列表"""
    files: List[DataFileInfo]


class DataLoadResponse(BaseModel):
    """分页读取结果

    `total` 是**过滤后**的总条数（`search` 生效时小于文件总条数），
    客户端据此翻页。
    """
    total: int
    page: int
    page_size: int
    items: List[Dict[str, Any]]


class DataUploadResponse(BaseModel):
    """上传结果：落盘路径与实际写入条数"""
    success: bool
    path: str
    count: int


class DatasetAnalysisResponse(BaseModel):
    """数据集分析结果（与 `Pipeline.analyze_dataset()` 的返回键一致）"""
    coverage_analysis: Dict[str, Any]
    statistics: Dict[str, Any]
    dedup_report: Dict[str, Any]


class VisualizeResponse(BaseModel):
    """可视化结果：图表名到文件路径的映射"""
    charts: Dict[str, str]


class DemoItem(BaseModel):
    """演示数据条目"""
    instruction: str
    input: str
    output: str


def _safe_data_path(filename: str) -> Path:
    """将文件名规范化为安全路径

    委托给 deps 中的统一白名单校验：拒绝 `..` 组件与绝对路径逃逸，
    并把结果限制在 `web.data_roots` 之内。是否要求文件存在由调用方判断。

    Args:
        filename: 客户端传入的文件名或路径

    Returns:
        已 resolve 的绝对路径

    Raises:
        HTTPException: 400 参数非法；403 路径越界
    """
    return resolve_within_roots(filename, "文件路径")


@router.get("/api/data/list", response_model=DataFileListResponse, summary="列出数据文件")
async def list_data_files():
    """列出白名单根目录下的 `train_data*.json`

    扫描范围**必须**与 `allowed_data_roots` 同源：这里曾写死 `Path(".")`，
    于是出厂默认收紧到 `["data"]` 之后，列表会把工作目录里的文件端给用户，而用户一点
    就是 403——列表能给的，路由必须能读。

    `name` 是裸文件名而 `{filename}` 只匹配单个路径段，所以它可加载的前提是
    `resolve_within_roots` 会在白名单根目录内查找相对路径（见 `deps` 的说明）。
    """
    loop = asyncio.get_event_loop()

    def scan_files():
        files = []
        seen = set()
        # 白名单里的目录可能不存在（例如 `data/` 还没建）。Python 3.13 的
        # `Path.glob` 对此返回空而不是抛错，因此无需 `is_dir()` 预检——加了
        # 反而是一条永远不会走到的分支。
        for root in allowed_data_roots():
            for f in sorted(root.glob("*.json")):
                if not f.name.startswith("train_data"):
                    continue
                resolved = f.resolve()
                if resolved in seen:  # 多个根目录互相嵌套时去重
                    continue
                seen.add(resolved)
                files.append({
                    "name": f.name,
                    "path": str(f),
                    "size": f.stat().st_size
                })
        return files

    files = await loop.run_in_executor(None, scan_files)
    return {"files": files}


@router.get(
    "/api/data/load/{filename}",
    response_model=DataLoadResponse,
    summary="分页加载数据",
)
async def load_data(filename: str, page: int = 1, page_size: int = 20, search: str = ""):
    """加载数据（分页）"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        items = await read_json_file(file_path)

        if search:
            search_lower = search.lower()
            items = [
                item for item in items
                if search_lower in item.get("instruction", "").lower()
                or search_lower in item.get("output", "").lower()
            ]

        total = len(items)
        start = (page - 1) * page_size
        end = start + page_size

        return {
            "total": total,
            "page": page,
            "page_size": page_size,
            "items": items[start:end]
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.put(
    "/api/data/update/{filename}",
    response_model=SuccessResponse,
    summary="更新单条数据",
)
async def update_data_item(
    filename: str,
    index: int,
    item: dict,
    _auth: None = Depends(verify_api_key),
):
    """更新单条数据"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        items = await read_json_file(file_path)

        if index < 0 or index >= len(items):
            raise HTTPException(status_code=400, detail="索引越界")

        items[index] = item
        await write_json_file(file_path, items)

        return {"success": True}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.delete(
    "/api/data/delete/{filename}",
    response_model=SuccessResponse,
    summary="删除单条数据",
)
async def delete_data_item(
    filename: str, index: int, _auth: None = Depends(verify_api_key)
):
    """删除单条数据"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        items = await read_json_file(file_path)

        if index < 0 or index >= len(items):
            raise HTTPException(status_code=400, detail="索引越界")

        items.pop(index)
        await write_json_file(file_path, items)

        return {"success": True}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


async def _read_upload_body(file: UploadFile, limit: int) -> bytes:
    """读上传体，但**先在 O(1) 的尺寸上判上限**；越界当场 413，且先于任何落盘动作

    为什么先判尺寸而不是分块攒：分块攒那份 `bytearray` 在**没越界**时要付实打实的
    代价（实测同一个 64.1 MiB 上传，攒块比一次 `read()` 多 64.0 MiB 峰值、慢 5.9 倍，
    因为 bytearray 扩容要留旧缓冲区）。而 Starlette 1.2.1 的 `UploadFile.write()`
    里就有 `self.size += len(data)`，解析完的 `size` 是**服务端自己按实际写入字节
    累加出来的**（不是客户端报的 Content-Length），于是「上限」可以在读之前一次判掉：
    越界的一个字节都不搬进内存，未越界的照旧一次读完。

    两道判据不是重复：`declared` 那道负责内存，事后那道负责形状（`size` 为 `None`
    的构造方 —— 不走解析器的直接构造 —— 只能读完了才知道有多大）。

    **如实记下管不到的那一半**：Starlette 的 multipart 解析在进入本函数之前就把文件
    部分写进 `SpooledTemporaryFile(max_size=1 MiB)`，超过就溢到临时目录；它那 1 MiB 的
    `max_part_size` 只判**非文件**表单字段（`_current_part.file is None` 那一支），
    文件部分直接绕开。所以这条上限管的是「服务端为一次上传分配多少内存、要不要把
    内容写进数据目录」，**不管**「客户端能不能把大 body 灌到磁盘临时文件上」——
    那一层在 ASGI/反代的 `client_max_body_size`，不在本仓代码里（另立一笔）。

    Args:
        file: 上传的文件部分
        limit: 允许的最大字节数（来自 `deps.max_upload_bytes()`）

    Returns:
        不超过 `limit` 的完整字节

    Raises:
        HTTPException: 413 越界（文案含上限与实际/申报字节数，不回显内容）
    """
    declared = getattr(file, "size", None)
    if declared is not None and declared > limit:
        raise HTTPException(
            status_code=413,
            detail=(
                f"上传内容 {declared} 字节，超过 web.max_upload_bytes={limit}"
                "（未读取、未落盘）"
            ),
        )

    content = await file.read()
    if len(content) > limit:
        raise HTTPException(
            status_code=413,
            detail=(
                f"上传内容 {len(content)} 字节，超过 web.max_upload_bytes={limit}"
                "（未落盘）"
            ),
        )
    return content


@router.post(
    "/api/data/upload",
    response_model=DataUploadResponse,
    summary="上传数据文件",
)
async def upload_data(
    file: UploadFile = File(...), _auth: None = Depends(verify_api_key)
):
    """上传数据文件

    落点：`file.filename` 按**白名单内的相对路径**用，不再剥成裸文件名。改前剥的
    那一刀把 `resolve_within_roots` 对外承诺的写法（「要写入请显式传 `data/xxx.json`」）
    在这条路由上变成了拿不到的出口——客户端唯一能成功的姿势是先知道目标已存在。
    `..` 仍是 400，越界的绝对路径仍是 403，白名单闸一字未松。

    读体上限见 `_read_upload_body`；形态判据与读侧共用 `assert_dataset_shape`，
    所以「上传成功但那份文件读端点自己拒绝」已经不可能发生。

    **下面那句 `json.loads` 刻意留在事件循环里**（L87 尺子，`Temp/l87q/stall_parse_l87.py`）：
    AST 普查把它记成「async 处理器里的阻塞调用」，本仓对这一类的既有处置是
    `deps.run_in_thread`，但实测搬过去省的不到一成 —— 11.6 MiB 档最差跳距
    `inline` 比噪声地板高 11.0 ms、`executor` 高 11.8 ms（净收益 **−0.8 ms**）；
    64.1 MiB 档分别是 +88.1 ms 与 +78.1 ms（净收益 10.0 ms）。原因在 GIL：`json` 的
    C 扫描器整个 `loads()` 期间持锁，线程池治的是**让出 GIL 的等待**（磁盘 I/O、
    `sleep`），治不了解放锁的纯 CPU 工作。尺子的正负对照同批实测（`time.sleep(0.20)`：
    循环里 215.9 ms vs 线程里 20.0 ms ≈ 地板 19.0 ms），所以这两组差是真读数不是瞎读。
    真正的封顶是上面那条字节上限：停顿与 body 尺寸同阶，把尺寸交给配置就同时把
    最坏停顿交给了配置。
    """
    try:
        content = await _read_upload_body(file, max_upload_bytes())
        items = json.loads(content.decode("utf-8"))
    except HTTPException:
        raise
    except UnicodeDecodeError as e:
        raise HTTPException(status_code=400, detail="上传内容不是合法的 UTF-8 文本") from e
    except Exception as e:
        raise to_http_error(e) from e

    assert_dataset_shape(items, "上传内容")

    try:
        save_path = _safe_data_path(file.filename)
        await write_json_file(save_path, items)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e

    return {"success": True, "path": str(save_path), "count": len(items)}


# ============ 数据分析 ============

@router.get(
    "/api/analyze/{filename}",
    response_model=DatasetAnalysisResponse,
    summary="分析数据集",
)
async def analyze_data(filename: str):
    """分析数据集（覆盖度、统计信息、去重报告）"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        p = get_pipeline()
        return await run_in_thread(p.analyze_dataset, str(file_path))
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get(
    "/api/visualize/{filename}",
    response_model=VisualizeResponse,
    summary="生成可视化图表",
)
async def visualize_data(filename: str):
    """生成数据集可视化图表"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        p = get_pipeline()
        results = await run_in_thread(p.visualize_dataset, str(file_path))
        return {"charts": results}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


# ============ 演示数据 ============

_DEMO_DATA = [
    {
        "instruction": "什么是人工智能？",
        "input": "",
        "output": "人工智能（AI）是计算机科学的一个分支，致力于创建能够执行通常需要人类智能的任务的系统，如学习、推理、问题解决、感知和语言理解。"
    },
    {
        "instruction": "解释一下机器学习的基本概念",
        "input": "",
        "output": "机器学习是人工智能的子领域，它使计算机能够从数据中学习并做出决策或预测，而无需显式编程。主要类型包括监督学习、无监督学习和强化学习。"
    },
    {
        "instruction": "深度学习与传统机器学习有什么区别？",
        "input": "",
        "output": "深度学习使用多层神经网络自动提取特征，适合处理大量数据；传统机器学习通常需要人工特征工程，在小数据集上可能更高效。"
    },
    {
        "instruction": "什么是自然语言处理（NLP）？",
        "input": "",
        "output": "自然语言处理是AI的一个分支，专注于让计算机理解、解释和生成人类语言。应用包括机器翻译、情感分析、聊天机器人和文本摘要。"
    },
    {
        "instruction": "请介绍一下计算机视觉的基本任务",
        "input": "",
        "output": "计算机视觉的基本任务包括图像分类、目标检测、语义分割、实例分割和图像生成。这些技术应用于自动驾驶、医学影像、安防监控等领域。"
    }
]


@router.get(
    "/api/demo/data",
    response_model=List[DemoItem],
    summary="内置演示数据集",
)
async def get_demo_data():
    """获取内置演示数据集，开箱即用

    返回的是**裸数组**（不是 `{"data": [...]}` 包装），前端可直接当数据集喂给
    其它端点。这里刻意返回列表而不是 `JSONResponse`：后者会绕过
    response_model，让上面声明的契约退化成装饰。
    """
    return _DEMO_DATA
