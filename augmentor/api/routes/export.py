# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据导出 API 路由"""

from pathlib import Path
from typing import Any, Dict, List, Optional

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from ..deps import (
    get_pipeline,
    load_items,
    resolve_data_path,
    resolve_data_dir,
    run_in_thread,
    verify_api_key,
)

router = APIRouter(tags=["export"])


class ExportResponse(BaseModel):
    """单数据集导出结果：格式名到输出文件路径的映射"""
    success: bool
    files: Dict[str, str]


class BatchExportResponse(BaseModel):
    """批量导出结果：数据集名 → (格式名 → 输出文件路径)"""
    success: bool
    results: Dict[str, Dict[str, str]]


class PreviewResponse(BaseModel):
    """格式预览结果（与 `ExportPreview.to_dict()` 的键一一对应）"""
    format: str
    original_data: List[Dict[str, Any]]
    converted_data: List[Dict[str, Any]]
    format_info: Dict[str, Any]
    warnings: List[str]


class ExportFormatsResponse(BaseModel):
    """支持的导出格式"""
    formats: List[str]


class ExportRequest(BaseModel):
    """导出请求"""
    input_file: str
    output_dir: str
    formats: Optional[List[str]] = None


class BatchExportRequest(BaseModel):
    """批量导出请求"""
    datasets: Dict[str, str]
    output_dir: str
    formats: Optional[List[str]] = None


class PreviewRequest(BaseModel):
    """格式预览请求"""
    input_file: str
    format: str = "jsonl"
    size: int = 5


@router.post(
    "/api/data/export",
    response_model=ExportResponse,
    summary="导出数据",
)
async def export_data(request: ExportRequest, _auth: None = Depends(verify_api_key)):
    """导出数据"""
    try:
        # 读写路径都必须落在 web.data_roots 白名单内
        input_path = resolve_data_path(request.input_file)
        output_dir = resolve_data_dir(request.output_dir)

        p = get_pipeline()
        results = await run_in_thread(
            p.export_dataset, str(input_path), str(output_dir), request.formats
        )
        return {"success": True, "files": results}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/export/batch",
    response_model=BatchExportResponse,
    summary="批量导出多个数据集",
)
async def batch_export(request: BatchExportRequest, _auth: None = Depends(verify_api_key)):
    """批量导出多个数据集"""
    try:
        output_dir = resolve_data_dir(request.output_dir)

        def run():
            from augmentor.export import Exporter

            datasets = {}
            for name, path in request.datasets.items():
                datasets[name] = load_items(path)

            exporter = Exporter()
            return exporter.export_batch(
                datasets, str(output_dir), request.formats
            )

        results = await run_in_thread(run)
        return {"success": True, "results": results}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/export/preview",
    response_model=PreviewResponse,
    summary="预览导出格式转换结果",
)
async def preview_export(request: PreviewRequest):
    """预览导出格式转换结果"""
    try:
        items = await run_in_thread(load_items, request.input_file)

        def run():
            from augmentor.preview import PreviewGenerator

            return PreviewGenerator(
                preview_size=request.size
            ).preview(items, request.format).to_dict()

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get(
    "/api/export/formats",
    response_model=ExportFormatsResponse,
    summary="列出支持的导出格式",
)
async def list_export_formats():
    """列出支持的导出格式"""
    try:
        from augmentor.export import Exporter

        return {"formats": Exporter().get_supported_formats()}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
