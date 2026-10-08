# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""泄漏检测 API 路由"""

from typing import Any, Dict, List, Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel, Field

from ..deps import raise_internal_error, load_items, run_in_thread
from augmentor.leakage import LeakageReport, detect_leakage

router = APIRouter(tags=["leakage"])


class LeakageRequest(BaseModel):
    """泄漏检测请求"""
    train_file: str
    test_file: str
    fields: Optional[List[str]] = None
    # A172（L181）：fuzzy_threshold 可用区间 (0, 1]（0 档假零报 100% 泄漏、>1 静默关闭），
    # 边界 422 拒在请求面，与 SDK 构造器判据（augmentor/leakage.py，权威）同档。
    fuzzy_threshold: float = Field(0.8, gt=0, le=1)


class LeakageResponse(BaseModel):
    """泄漏检测响应

    字段必须与 `LeakageReport.to_dict()` 的键完全一致。其中
    `total_leaks` / `leak_rate` / `is_clean` 是 dataclass 上的**计算属性**，
    不是字段——直接拿 `LeakageReport` 当 response_model 会把这三个键静默丢掉。
    """
    train_size: int
    test_size: int
    exact_leaks: int
    fuzzy_leaks: int
    total_leaks: int
    leak_rate: float
    is_clean: bool
    leaked_examples: List[Dict[str, Any]]


@router.post("/api/leakage/check", response_model=LeakageResponse, summary="训练/测试集泄漏检测")
async def check_leakage(request: LeakageRequest):
    """检测训练/测试集之间的数据泄漏"""
    try:
        train_items = await run_in_thread(load_items, request.train_file)
        test_items = await run_in_thread(load_items, request.test_file)

        def run():
            report = detect_leakage(
                train_items,
                test_items,
                fields=request.fields,
                fuzzy_threshold=request.fuzzy_threshold,
            )
            return report.to_dict()

        return await run_in_thread(run)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise_internal_error(e)
