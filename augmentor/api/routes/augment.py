# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据增强 API 路由"""

import logging
import threading
from collections import OrderedDict
from typing import Any, Dict, List, Optional

from fastapi import APIRouter, BackgroundTasks, Depends, HTTPException
from pydantic import BaseModel

from ..deps import raise_internal_error, raise_boundary_error, get_pipeline, resolve_data_path, verify_api_key
from ..schemas import MessageResponse

logger = logging.getLogger(__name__)

router = APIRouter(tags=["augment"])

# 增强报告缓存（第 35 轮）：后台任务的返回值此前被整体丢弃 ——
# errors 失败明细（条目下标/异常类型/预览）只存在于被丢弃的字典里，
# UI 只能看到 failed_items 计数。这里按 task_id 存进程内报告，
# `GET /api/augment/report/{task_id}` 可取。单进程部署适用；
# 多 worker 部署需要落盘，见端点 docstring 的边界说明。
#
# 第 73 轮：缓存加容量上限（LRU 逐出最旧）。报告含逐条错误明细，长期运行的
# 服务每跑一个任务就多占一份，此前**无上限**——任务数就是内存增长量。
_MAX_REPORTS = 200
_REPORTS: "OrderedDict[str, Dict]" = OrderedDict()
_REPORTS_LOCK = threading.Lock()


def _remember_report(report: Dict) -> None:
    """保存一份任务报告；超出上限时逐出最久未访问的（LRU）

    Args:
        report: 含 `task_id` 的增强报告
    """
    task_id = report["task_id"]
    with _REPORTS_LOCK:
        _REPORTS[task_id] = report
        _REPORTS.move_to_end(task_id)
        while len(_REPORTS) > _MAX_REPORTS:
            _REPORTS.popitem(last=False)


def _get_report(task_id: str) -> Optional[Dict]:
    """取一份报告并把该条移到 LRU 队尾（读也算一次访问）

    Args:
        task_id: 任务 ID

    Returns:
        报告字典；不存在时 None
    """
    with _REPORTS_LOCK:
        report = _REPORTS.get(task_id)
        if report is not None:
            _REPORTS.move_to_end(task_id)
        return report



class CheckpointListResponse(BaseModel):
    """断点任务 ID 列表"""
    checkpoints: List[str]


class AugmentReportResponse(BaseModel):
    """增强任务报告（第 35 轮端点的完整响应契约）。

    键与 `augment_dataset` 返回的报告一致：失败明细在 `errors`，
    `progress` / `memory_monitor` 是运行期诊断。
    """
    task_id: str
    input_file: str
    output_file: str
    input_count: int
    output_count: int
    quality_check: bool
    dedup: bool
    parallel: bool
    vector_index: Optional[Any] = None
    errors: List[Dict]
    progress: Optional[Dict[str, Any]] = None
    memory_monitor: Dict[str, Any]


class AugmentStartResponse(MessageResponse):
    """启动增强任务的回执：在 `success` / `message` 之外**带上任务身份**

    A408：任务身份的权威是 `AugmentorPipeline.build_task_id`（按输入文件的
    **解析后**路径取 md5 前 12 位），而客户端**算不出**那个 id（它只知道自己
    送的文件名，不知道服务端解析后的路径）⇒ 客户端要问「**我这次**任务的进度」
    就必须由服务端把那个 id 递回来。回执里没有它，客户端只能去读「当下谁的
    断点在跑」那种环境态读法。

    `task_id` 声明成可选：它是**新增**的一格，老客户端不读它 ⇒ 形状变化是
    加法而不是改法。
    """
    task_id: Optional[str] = None


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
    response_model=AugmentStartResponse,
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
            report = p.augment_dataset(
                str(input_path),
                str(output_path),
                use_checkpoint=request.use_checkpoint,
                use_quality_check=request.use_quality,
                use_dedup=request.use_dedup,
                use_async=request.use_async
            )
            # 第 35 轮：报告（含 errors 失败明细）按 task_id 存下，
            # 前端轮询到终态后能拉取「哪条失败、为什么」。
            try:
                _remember_report(report)
            except Exception:
                # 报告存储失败不应影响任务本身已完成的输出。
                logger.exception("保存增强报告失败")

        background_tasks.add_task(run_augment)

        # A408：把**这次运行**的任务身份递回客户端。id 的算法与
        # `augment_dataset` 内部用的是同一个（`build_task_id` 按解析后的输入
        # 路径取 md5 前 12 位）⇒ 客户端拿它去问「我这次任务的进度」能问到同一份。
        # 走**类**上的静态方法而不是 `p.build_task_id(...)`：那个 id 是路径的纯
        # 函数，与实例无关 ⇒ 注入的替身管道（测试里常见）也能给出同一个 id。
        from augmentor.pipeline import AugmentorPipeline
        task_id = AugmentorPipeline.build_task_id(str(input_path))
        return {"success": True, "message": "增强任务已启动", "task_id": task_id}
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)


@router.get(
    "/api/augment/report/{task_id}",
    response_model=AugmentReportResponse,
    summary="获取增强任务报告（含失败明细）",
)
async def get_augment_report(task_id: str, _auth: None = Depends(verify_api_key)):
    """第 35 轮：读取一次增强任务的报告。

    报告含 `errors` 失败明细（条目下标/异常类型/预览），后台任务完成时
    按 task_id 缓存。边界：单进程部署适用 —— 多 worker 时报告落在
    处理该任务的 worker 内存里，其它 worker 读不到（后续可改落盘）。
    """
    report = _get_report(task_id)
    if report is None:
        raise HTTPException(status_code=404, detail="报告不存在（任务未完成或由其它 worker 处理）")
    return report


@router.delete(
    "/api/augment/checkpoints/{task_id}",
    response_model=MessageResponse,
    summary="删除断点（从头重跑）",
)
async def delete_checkpoint(task_id: str, _auth: None = Depends(verify_api_key)):
    """第 56 轮：删除一个断点。

    文档承诺「如需从头重跑，先调用 delete_checkpoint」，但此前 API/UI 都
    做不到——只能登服务器删文件。删除后同一输入文件的下一次运行从零开始。
    """
    try:
        p = get_pipeline()
        p.checkpoint_manager.delete_checkpoint(task_id)
        return {"success": True, "message": "断点已删除"}
    except Exception as e:
        # A280 / L314：坏任务 id（`require_within_dir` 那一档：越界 / 冒号盘符）
        # ⇒ 400 + 那句域判决；其余照旧 500 + 固定文案。
        raise_boundary_error(e)


@router.get(
    "/api/augment/progress",
    response_model=AugmentProgressResponse,
    response_model_exclude_none=True,
    summary="获取增强进度",
)
async def get_augment_progress(task_id: Optional[str] = None):
    """获取增强进度

    **A408：可以点名任务。**

    不带 `task_id` 时是**环境态读法**（当下 `_current_checkpoint` 指谁就读谁）
    —— 那份回执里的 `task_id` 可能属于**最后加载断点的那一次运行**，而不是
    调用方问的那一次（`get_progress` 的 docstring 早就把这个口径写明了，
    但 HTTP 面没给「点名」这条路）。

    带上 `task_id` 时读的是**那一份**断点的进度，且是**只读**的：不消费增量
    （不把正在跑的增量吃掉）、不动环境态（不把别的调用方的指针搬走）。
    那个任务没有断点时回 **404**（「问的那个任务不存在」与「当下没有任务在跑」
    是两件事，不该用同一句话回答）。
    """
    try:
        p = get_pipeline()
        if task_id is None:
            # 环境态读法（既有行为，一格未动）
            return p.checkpoint_manager.get_progress()
        ck = p.checkpoint_manager.peek_checkpoint(task_id)
        if ck is None:
            raise HTTPException(status_code=404, detail=f"任务 {task_id} 没有断点")
        return p.checkpoint_manager.get_progress(ck)
    except HTTPException:
        raise
    except Exception as e:
        # A280 / L314：坏任务 id（`require_within_dir` 那一档）⇒ 400 + 那句域判决；
        # 其余照旧 500 + 固定文案。
        raise_boundary_error(e)


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
