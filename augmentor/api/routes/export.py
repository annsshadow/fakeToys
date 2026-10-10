# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据导出 API 路由"""

from pathlib import Path
from typing import Any, Dict, List, Optional

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from ..deps import (
    raise_internal_error,

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
    """格式预览请求

    `format` 省略时读 `config.export.default_format`（L214）——请求参数优先、
    配置兜底，与其余消费点同一权威序。
    """
    input_file: str
    format: Optional[str] = None
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
        # formats 两级（A138② / L214）：请求点名 > config.export.formats > 导出器
        # 的「全部格式」旧行为。空列表按「没配」处理，与批导出同一口径。
        formats = request.formats or p.config.export.formats or None
        results = await run_in_thread(
            p.export_dataset, str(input_path), str(output_dir), formats
        )
        return {"success": True, "files": results}
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


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

            p = get_pipeline()
            # formats 与 default_format 都是「请求参数 > config.export」两级（A138② / L214）；
            # 都没有时才落 Exporter 的内置默认。
            formats = request.formats or p.config.export.formats or None
            exporter = Exporter(default_format=p.config.export.default_format)
            return exporter.export_batch(
                datasets, str(output_dir), formats
            )

        results = await run_in_thread(run)
        return {"success": True, "results": results}
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


@router.post(
    "/api/export/preview",
    response_model=PreviewResponse,
    summary="预览导出格式转换结果",
)
async def preview_export(request: PreviewRequest):
    """预览导出格式转换结果"""
    try:
        items = await run_in_thread(load_items, request.input_file)
        # format 省略 ⇒ config.export.default_format（① 档读者在预览面的延伸）⇒
        # Exporter 内置默认；PreviewGenerator 只实现了六族原生格式，配置里若写了
        # 别的合法格式名（csv/openai…），这一脚会得到它自己的 400 而不是静默换格式。
        fmt = request.format or get_pipeline().config.export.default_format

        def run():
            from augmentor.preview import PreviewGenerator

            return PreviewGenerator(
                preview_size=request.size
            ).preview(items, fmt).to_dict()

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


@router.get(
    "/api/export/formats",
    response_model=ExportFormatsResponse,
    summary="列出支持的导出格式",
)
async def list_export_formats():
    """列出支持的导出格式

    这是**能力清单**（导出器认得的格式全集），不是策略面：`config.export.formats`
    管的是「调用方没点名时要导哪几族」（见上面两个导出端点），拿它裁剪能力清单
    会把 csv/openai 这些合法格式从客户端视野里抹掉（L214 探针实测：默认配置只列
    五族，而能力全集有 13 族）。
    """
    try:
        from augmentor.export import Exporter

        return {"formats": Exporter().get_supported_formats()}
    except Exception as e:
        raise_internal_error(e)
