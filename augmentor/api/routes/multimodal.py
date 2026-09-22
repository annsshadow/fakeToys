"""多模态数据处理 API 路由"""

from typing import Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import resolve_data_dir, resolve_within_roots, run_in_thread

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


@router.post("/api/multimodal/scan")
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


@router.get("/api/multimodal/formats")
async def supported_formats():
    """列出支持的多模态格式"""
    from augmentor.data import ImageProcessor, AudioProcessor

    return {
        "image_extensions": ImageProcessor().supported_extensions,
        "audio_extensions": AudioProcessor().supported_extensions
    }
