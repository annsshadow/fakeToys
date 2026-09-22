# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据增强 API 路由"""

from fastapi import APIRouter, BackgroundTasks, Depends, HTTPException
from pydantic import BaseModel

from ..deps import get_pipeline, resolve_data_path, verify_api_key

router = APIRouter(tags=["augment"])


class AugmentRequest(BaseModel):
    """增强请求"""
    input_file: str
    output_file: str
    use_quality: bool = True
    use_dedup: bool = True
    use_checkpoint: bool = True


@router.post("/api/augment/start")
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


@router.get("/api/augment/progress")
async def get_augment_progress():
    """获取增强进度"""
    try:
        p = get_pipeline()
        return p.checkpoint_manager.get_progress()
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/augment/checkpoints")
async def list_checkpoints():
    """列出断点"""
    try:
        p = get_pipeline()
        checkpoints = p.checkpoint_manager.list_checkpoints()
        return {"checkpoints": checkpoints}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
