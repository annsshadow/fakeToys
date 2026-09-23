# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""多模态数据处理 API 路由"""

from typing import Any, Dict, List, Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import resolve_data_dir, resolve_within_roots, run_in_thread

router = APIRouter(tags=["multimodal"])


class MultimodalRecordResponse(BaseModel):
    """单条多模态记录（与 `MultimodalRecord.to_dict()` 的键一一对应）

    `image` / `audio` 对缺失或损坏的媒体是 **None**，这里刻意不启用
    `exclude_none`：契约要求这两个键始终存在，值为 null 才是「该模态没有内容」
    与「该模态解析失败」之外的正确表达。
    """
    text: str
    image: Optional[Dict[str, Any]]
    audio: Optional[Dict[str, Any]]
    modalities: List[str]
    valid: bool
    errors: List[str]


class MultimodalScanResponse(BaseModel):
    """目录扫描结果：汇总报告 + 逐条记录

    前 6 个键来自 `MultimodalProcessor.generate_report()`，`records` 由路由补上。
    """
    total_records: int
    valid_records: int
    invalid_records: int
    modality_counts: Dict[str, int]
    error_count: int
    errors: List[str]
    records: List[MultimodalRecordResponse]


class MultimodalFormatsResponse(BaseModel):
    """支持的媒体扩展名"""
    image_extensions: List[str]
    audio_extensions: List[str]


class MultimodalRequest(BaseModel):
    """多模态处理请求"""
    text: str = ""
    image: Optional[str] = None
    audio: Optional[str] = None


class ScanRequest(BaseModel):
    """目录扫描请求"""
    directory: str


@router.post(
    "/api/multimodal/process",
    response_model=MultimodalRecordResponse,
    summary="处理单条多模态数据",
)
async def process_multimodal(request: MultimodalRequest):
    """处理单条多模态数据"""
    try:
        # 媒体文件路径同样受白名单约束，避免借多模态处理读取任意文件。
        # 这里只做边界校验、不要求文件存在——MultimodalProcessor 的既有约定是
        # 对缺失/损坏的媒体文件优雅降级（在结果里记录 errors），而不是直接报错。
        payload = request.model_dump()
        for field in ("image", "audio"):
            if payload.get(field):
                payload[field] = str(
                    resolve_within_roots(payload[field], "媒体文件路径")
                )

        def run():
            from augmentor.data import MultimodalProcessor

            processor = MultimodalProcessor()
            record = processor.fuse_modalities(payload)
            return record.to_dict()

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/multimodal/scan",
    response_model=MultimodalScanResponse,
    summary="扫描目录并处理多模态文件",
)
async def scan_directory(request: ScanRequest):
    """扫描目录并处理多模态文件"""
    try:
        directory = resolve_data_dir(request.directory)

        def run():
            from augmentor.data import MultimodalProcessor

            processor = MultimodalProcessor()
            records = processor.process_directory(str(directory))
            report = processor.generate_report(records)
            report["records"] = [r.to_dict() for r in records]
            return report

        return await run_in_thread(run)
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get(
    "/api/multimodal/formats",
    response_model=MultimodalFormatsResponse,
    summary="列出支持的多模态格式",
)
async def supported_formats():
    """列出支持的多模态格式"""
    from augmentor.data import ImageProcessor, AudioProcessor

    return {
        "image_extensions": ImageProcessor().supported_extensions,
        "audio_extensions": AudioProcessor().supported_extensions
    }
