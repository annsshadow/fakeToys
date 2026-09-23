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
    get_pipeline,
    read_json_file,
    resolve_within_roots,
    run_in_thread,
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
    """列出数据文件"""
    loop = asyncio.get_event_loop()

    def scan_files():
        data_dir = Path(".")
        files = []
        for f in data_dir.glob("*.json"):
            if f.name.startswith("train_data"):
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


@router.post(
    "/api/data/upload",
    response_model=DataUploadResponse,
    summary="上传数据文件",
)
async def upload_data(
    file: UploadFile = File(...), _auth: None = Depends(verify_api_key)
):
    """上传数据文件"""
    try:
        content = await file.read()
        items = json.loads(content.decode('utf-8'))

        safe_name = Path(file.filename).name
        save_path = _safe_data_path(safe_name)
        await write_json_file(save_path, items)

        return {"success": True, "path": str(save_path), "count": len(items)}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


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
