# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""服务状态 API 路由

综合报告健康、版本、模型后端可用性与可选依赖诊断。
"""

import logging

from fastapi import APIRouter
from pydantic import BaseModel
from typing import Optional

from ..deps import get_pipeline
from augmentor import __version__
from augmentor.diagnostics import check_dependencies

logger = logging.getLogger(__name__)
router = APIRouter(tags=["status"])


class StatusResponse(BaseModel):
    """状态响应结构

    `model_retry_count` / `model_retry_wait_seconds`（L66 后端重试统计的 API 出口）
    与 `model_generate_seconds`（2026-10-24：真实 API 调用累计耗时）放在这里而非
    `/api/health`：health 被容器探针依赖、明令「不加会阻塞或可能失败的
    字段」，而 status 本就在 try 里读 `p.model_backend`。管道缺失时几键为 `None`
    （区分「没有后端」与「后端重试过 0 次」）。
    """
    status: str
    version: str
    model_default: str
    model_available: bool
    model_retry_count: Optional[int]
    model_retry_wait_seconds: Optional[float]
    model_generate_seconds: Optional[float]
    dependencies: dict


class HealthResponse(BaseModel):
    """健康检查响应结构

    只含存活信号与版本号：容器探针与前端启动检查依赖它，
    因此不要往里加会阻塞或可能失败的字段。
    """
    status: str
    version: str


@router.get("/api/status", response_model=StatusResponse, summary="服务综合状态")
async def get_status():
    """获取服务综合状态"""
    model_retry_count = None
    model_retry_wait_seconds = None
    model_generate_seconds = None
    try:
        p = get_pipeline()
        default_model = p.config.default_model
        model_available = p.model_backend is not None
        if p.model_backend is not None:
            model_retry_count = p.model_backend.retry_count
            model_retry_wait_seconds = p.model_backend.retry_wait_seconds
            model_generate_seconds = p.model_backend.total_generate_seconds
    except Exception as e:
        # 原文只落服务端日志、不进响应体，与 `deps` 的 500 支同一口径；从前这一格连日志都不留
        logger.warning(f"读取管道状态失败，按模型后端不可用回执: {e}")
        default_model = "unknown"
        model_available = False

    deps = check_dependencies()
    return StatusResponse(
        status="ok",
        version=__version__,
        model_default=default_model,
        model_available=model_available,
        model_retry_count=model_retry_count,
        model_retry_wait_seconds=model_retry_wait_seconds,
        model_generate_seconds=model_generate_seconds,
        dependencies=deps.to_dict(),
    ).model_dump()
