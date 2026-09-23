# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据增强 API 路由"""

from typing import List, Optional

from fastapi import APIRouter, BackgroundTasks, Depends, HTTPException
from pydantic import BaseModel

from ..deps import get_pipeline, resolve_data_path, verify_api_key
from ..schemas import MessageResponse

router = APIRouter(tags=["augment"])


class CheckpointListResponse(BaseModel):
    """断点任务 ID 列表"""
    checkpoints: List[str]


class AugmentProgressResponse(BaseModel):
    """增强进度

    `CheckpointManager.get_progress()` 有**两种形态**：
    没有进行中的任务时只返回 `{"status": "no_checkpoint"}`，有任务时才补齐
    其余字段。这里把任务字段全部声明为可选，并在路由上启用
    `response_model_exclude_none=True`——于是无任务时响应仍是一个键，
    有任务时字段齐全，两种形态共用同一份 OpenAPI 契约。

    代价：任务进行中若 `avg_quality_score` 恰为 None，该键会被排除，
    即它不保证「键永远存在」。这是为了让空态契约保持不变而做的取舍。
    """
    status: str
    task_id: Optional[str] = None
    total_items: Optional[int] = None
    processed_items: Optional[int] = None
    failed_items: Optional[int] = None
    progress: Optional[float] = None
    progress_percent: Optional[str] = None
    elapsed_time: Optional[str] = None
    remaining_time: Optional[str] = None
    start_time: Optional[str] = None
    last_update: Optional[str] = None
    avg_quality_score: Optional[float] = None


class AugmentRequest(BaseModel):
    """增强请求"""
    input_file: str
    output_file: str
    use_quality: bool = True
    use_dedup: bool = True
    use_checkpoint: bool = True


@router.post(
    "/api/augment/start",
    response_model=MessageResponse,
    summary="启动增强任务",
)
async def start_augmentation(
    request: AugmentRequest,
    background_tasks: BackgroundTasks,
    _auth: None = Depends(verify_api_key),
):
    """启动增强任务"""
    try:
        # 读写路径都必须在白名单内，且校验须在入队前完成，
        # 否则非法路径只会在后台任务里静默失败。
        input_path = resolve_data_path(request.input_file)
        output_path = resolve_data_path(request.output_file, for_write=True)

        p = get_pipeline()

        def run_augment():
            p.augment_dataset(
                str(input_path),
                str(output_path),
                use_checkpoint=request.use_checkpoint,
                use_quality_check=request.use_quality,
                use_dedup=request.use_dedup
            )

        background_tasks.add_task(run_augment)

        return {"success": True, "message": "增强任务已启动"}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get(
    "/api/augment/progress",
    response_model=AugmentProgressResponse,
    response_model_exclude_none=True,
    summary="获取增强进度",
)
async def get_augment_progress():
    """获取增强进度"""
    try:
        p = get_pipeline()
        return p.checkpoint_manager.get_progress()
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get(
    "/api/augment/checkpoints",
    response_model=CheckpointListResponse,
    summary="列出断点",
)
async def list_checkpoints():
    """列出断点"""
    try:
        p = get_pipeline()
        checkpoints = p.checkpoint_manager.list_checkpoints()
        return {"checkpoints": checkpoints}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
