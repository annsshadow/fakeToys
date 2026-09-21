"""数据导出 API 路由"""

from pathlib import Path
from typing import Dict, List, Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import get_pipeline, load_items, run_in_thread

router = APIRouter(tags=["export"])


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


@router.post("/api/data/export")
async def export_data(request: ExportRequest):
    """导出数据"""
    try:
        p = get_pipeline()
        results = await run_in_thread(
            p.export_dataset, request.input_file, request.output_dir, request.formats
        )
        return {"success": True, "files": results}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/api/export/batch")
async def batch_export(request: BatchExportRequest):
    """批量导出多个数据集"""
    try:
        def run():
            from augmentor.export import Exporter

            datasets = {}
            for name, path in request.datasets.items():
                datasets[name] = load_items(path)

            exporter = Exporter()
            return exporter.export_batch(
                datasets, request.output_dir, request.formats
            )

        results = await run_in_thread(run)
        return {"success": True, "results": results}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/api/export/preview")
async def preview_export(request: PreviewRequest):
    """预览导出格式转换结果"""
    try:
        items = load_items(request.input_file)

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


@router.get("/api/export/formats")
async def list_export_formats():
    """列出支持的导出格式"""
    try:
        from augmentor.export import Exporter

        return {"formats": Exporter().get_supported_formats()}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
