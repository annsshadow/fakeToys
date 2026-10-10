# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""多模态数据处理 API 路由"""

from typing import Any, Dict, List, Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import (
    get_pipeline,
    raise_internal_error,
    resolve_data_dir,
    resolve_within_roots,
    run_in_thread,
)

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
    """多模态处理请求

    **这里刻意没有 `image_extensions` / `audio_extensions`**（L214 量过之后撤掉的）：
    `fuse_modalities` 走 `process_image` / `process_audio`，两者按**文件内容**解析、
    不查 `supported_extensions`——那道门只住在 `is_supported` / `process_directory`，
    也就是扫描端点。给一条举不出行为差的请求加字段，等于亲手制造下一格 A138 型幽灵面。
    """
    text: str = ""
    image: Optional[str] = None
    audio: Optional[str] = None


class ScanRequest(BaseModel):
    """目录扫描请求

    两只扩展名键省略时读 `config.multimodal`（L214，A138 ③ 两键在此获得正牌读者），
    请求参数优先。识别面在扫描端点是**有行为差的**：同名 stem 的配对与 `total_records`
    都跟着它变。
    """
    directory: str
    image_extensions: Optional[List[str]] = None
    audio_extensions: Optional[List[str]] = None


def _resolve_extensions(image_extensions, audio_extensions):
    """两级合流：请求参数 > config.multimodal > 处理器内置默认（L214）

    扩展名的「`.` 开头」语义界住在这一层而不是配置校验里（A138 留的口径：
    `Path.suffix` 带的就是点）——这里统一做小写与补点归一，配置里写 `jpg`
    与 `.jpg` 等价。`enabled` **不在这里判**：它是管道阶段的开关，
    住在 `AugmentorPipeline`（与 quality/dedup 同族），端点是调用方**点名要**
    的独立能力，拿它拒绝一次显式调用不属于「消费配置」而是改契约。
    """
    cfg = get_pipeline().config.multimodal

    def merge(request_value, config_value):
        if request_value:
            return list(request_value)
        if config_value:
            return list(config_value)
        return None

    return (
        _normalize_extensions(merge(image_extensions, cfg.image_extensions)),
        _normalize_extensions(merge(audio_extensions, cfg.audio_extensions)),
    )


def _normalize_extensions(exts):
    """小写 + 补点归一；None 透传（= 让处理器用自己的内置默认）"""
    if exts is None:
        return None
    return [e if str(e).startswith(".") else f".{e}"
            for e in (str(x).lower() for x in exts)]


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
        raise_internal_error(e)


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

            image_exts, audio_exts = _resolve_extensions(
                request.image_extensions, request.audio_extensions
            )
            processor = MultimodalProcessor(image_exts, audio_exts)
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
        raise_internal_error(e)


@router.get(
    "/api/multimodal/formats",
    response_model=MultimodalFormatsResponse,
    summary="列出支持的多模态格式",
)
async def supported_formats():
    """列出支持的多模态格式（GET 面无请求参数，读面即配置本身，L214）"""
    from augmentor.data import ImageProcessor, AudioProcessor

    cfg = get_pipeline().config.multimodal
    image = ImageProcessor(_normalize_extensions(cfg.image_extensions) or None)
    audio = AudioProcessor(_normalize_extensions(cfg.audio_extensions) or None)
    return {
        "image_extensions": image.supported_extensions,
        "audio_extensions": audio.supported_extensions
    }
