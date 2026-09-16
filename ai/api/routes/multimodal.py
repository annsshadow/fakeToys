"""多模态数据处理 API 路由"""

from typing import Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import run_in_thread

router = APIRouter(tags=["multimodal"])


class MultimodalRequest(BaseModel):
    """多模态处理请求"""
    text: str = ""
    image: Optional[str] = None
    audio: Optional[str] = None


class ScanRequest(BaseModel):
    """目录扫描请求"""
    directory: str


@router.post("/api/multimodal/process")
async def process_multimodal(request: MultimodalRequest):
    """处理单条多模态数据"""
    try:
        def run():
            from augmentor.data import MultimodalProcessor

            processor = MultimodalProcessor()
            record = processor.fuse_modalities(request.model_dump())
            return record.to_dict()

        return await run_in_thread(run)
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/api/multimodal/scan")
async def scan_directory(request: ScanRequest):
    """扫描目录并处理多模态文件"""
    try:
        def run():
            from augmentor.data import MultimodalProcessor

            processor = MultimodalProcessor()
            records = processor.process_directory(request.directory)
            report = processor.generate_report(records)
            report["records"] = [r.to_dict() for r in records]
            return report

        return await run_in_thread(run)
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/multimodal/formats")
async def supported_formats():
    """列出支持的多模态格式"""
    from augmentor.data import ImageProcessor, AudioProcessor

    return {
        "image_extensions": ImageProcessor().supported_extensions,
        "audio_extensions": AudioProcessor().supported_extensions
    }
