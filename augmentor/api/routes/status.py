# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""服务状态 API 路由

综合报告健康、版本、模型后端可用性与可选依赖诊断。
"""

from fastapi import APIRouter
from pydantic import BaseModel

from ..deps import get_pipeline
from augmentor import __version__
from augmentor.diagnostics import check_dependencies

router = APIRouter(tags=["status"])


class StatusResponse(BaseModel):
    """状态响应结构"""
    status: str
    version: str
    model_default: str
    model_available: bool
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
    try:
        p = get_pipeline()
        default_model = p.config.default_model
        model_available = p.model_backend is not None
    except Exception:
        default_model = "unknown"
        model_available = False

    deps = check_dependencies()
    return StatusResponse(
        status="ok",
        version=__version__,
        model_default=default_model,
        model_available=model_available,
        dependencies=deps.to_dict(),
    ).model_dump()
