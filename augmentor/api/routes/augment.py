# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据增强 API 路由"""

from typing import List, Optional

from fastapi import APIRouter, BackgroundTasks, Depends, HTTPException
from pydantic import BaseModel

from ..deps import raise_internal_error, get_pipeline, resolve_data_path, verify_api_key
from ..schemas import MessageResponse

router = APIRouter(tags=["augment"])


class CheckpointListResponse(BaseModel):
    """断点任务 ID 列表"""
    checkpoints: List[str]


class AugmentProgressResponse(BaseModel):
    """增强进度

    `CheckpointManager.get_progress()` 有**两种形态**，而且两形的键集不同：
    没有进行中的任务时只返回 `{"status": "no_checkpoint"}`（**只有这一键**），
    有任务时返回 11 个任务字段而**不含 `status`**（实测 `checkpoint.py` 的
    两条 `return`，也实测 HTTP 响应）。路由启用 `response_model_exclude_none=True`
    ⇒ 两形各自精确：空态回一个键，任务态回 11 个键。

    因此 `status` 只能是**可选**：从前把它声明成必填 `str`，而任务态的字典里
    压根没有这个键 ⇒ 一旦 `_current_checkpoint` 在场（任务运行中、以及任务结束后
    ——`get_progress` 读的正是那一份），FastAPI 的响应模型校验当场失败，
    `GET /api/augment/progress` 回 **500**，前端每 2 秒轮询一次每次吃一次 500，
    进度卡从此看不见任何真进度（`web/src/pages/Augmentation.tsx` 的 `catch {}`
    又把这句 500 咽成「暂无进行中的任务」）。前端本来就按「`task_id` 在不在」
    判别两形，TS 侧 `AugmentProgress` 也是这么建模的，错的只有这份契约。

    代价：任务进行中若 `avg_quality_score` 恰为 None，该键会被排除，
    即它不保证「键永远存在」。这是为了让空态契约保持不变而做的取舍。
    """
    status: Optional[str] = None
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
    """增强请求

    `use_quality` / `use_dedup` 是 `Optional[bool]`（L215）：字段缺席**不等于**关掉，
    而是「调用方没点」，由 `config.quality.enabled` / `config.dedup.enabled` 定；
    显式给 `true`/`false` 才是点。出厂两键都是 `true` ⇒ 省略方拿到的结果与改前一致，
    随包前端（`web/src/services/api.ts` 两只字段总是显式送）也完全不变。
    """
    input_file: str
    output_file: str
    use_quality: Optional[bool] = None
    use_dedup: Optional[bool] = None
    use_checkpoint: bool = True
    # 并行档是否走异步引擎（AU-P1）。默认 False = 原线程池路径。
    #
    # 这里**不**做"后端有没有原生异步"的判断：那是 pipeline 的职责，它在
    # 后端不支持时自动回退并打 warning。端点若在这里先判一遍，就成了第二份
    # 权威，两边一旦不一致就是"接口说开了异步、实际走线程池"。
    use_async: bool = False


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
            # L215：这里**不做**任何默认值判断，缺席就是 `None` 原样传下去，
            # 由 `augment_dataset` 那一层的权威序（调用参数 > 配置 > 内置默认）解析。
            # 端点若在这里补成 True，配置臂就永远拿不到控制权——那正是要避免的
            # 「键写在请求模型里、读者却只有代码默认值」的幽灵面。
            p.augment_dataset(
                str(input_path),
                str(output_path),
                use_checkpoint=request.use_checkpoint,
                use_quality_check=request.use_quality,
                use_dedup=request.use_dedup,
                use_async=request.use_async
            )

        background_tasks.add_task(run_augment)

        return {"success": True, "message": "增强任务已启动"}
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


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
        raise_internal_error(e)


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
        raise_internal_error(e)
