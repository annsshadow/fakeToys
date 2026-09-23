# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集就绪审计 API 路由"""

from typing import List, Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import load_items, run_in_thread
from augmentor.audit import AuditReport, DatasetAuditor

router = APIRouter(tags=["audit"])


class AuditRequest(BaseModel):
    """审计请求"""
    input_file: str
    reference_file: Optional[str] = None
    fields: Optional[List[str]] = None


# 直接用 dataclass 作 response_model：FastAPI 能为其生成 OpenAPI schema，
# 且字段集自动跟随 dataclass，不存在「模型漏字段导致响应被静默裁剪」的风险。
# AuditReport 没有计算属性（to_dict() 的键与字段一一对应），因此可以直接用。
@router.post("/api/audit", response_model=AuditReport, summary="数据集就绪审计")
async def audit_dataset_endpoint(request: AuditRequest):
    """对数据集做就绪审计，返回组合信号与总体判定"""
    try:
        items = load_items(request.input_file)
        reference = load_items(request.reference_file) if request.reference_file else None

        def run():
            auditor = DatasetAuditor(fields=request.fields)
            report = auditor.audit(items, reference)
            return report.to_dict()

        return await run_in_thread(run)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
