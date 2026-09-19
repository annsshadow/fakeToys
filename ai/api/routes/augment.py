"""数据增强 API 路由"""

from fastapi import APIRouter, BackgroundTasks, HTTPException
from pydantic import BaseModel

from ..deps import get_pipeline

router = APIRouter(tags=["augment"])


class AugmentRequest(BaseModel):
    """增强请求"""
    input_file: str
    output_file: str
    use_quality: bool = True
    use_dedup: bool = True
    use_checkpoint: bool = True


@router.post("/api/augment/start")
async def start_augmentation(request: AugmentRequest, background_tasks: BackgroundTasks):
    """启动增强任务"""
    try:
        p = get_pipeline()

        def run_augment():
            p.augment_dataset(
                request.input_file,
                request.output_file,
                use_checkpoint=request.use_checkpoint,
                use_quality_check=request.use_quality,
                use_dedup=request.use_dedup
            )

        background_tasks.add_task(run_augment)

        return {"success": True, "message": "增强任务已启动"}
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
