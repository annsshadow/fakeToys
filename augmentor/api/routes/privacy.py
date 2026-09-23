# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""PII 脱敏 API 路由"""

from typing import Any, Dict, List, Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import load_items, run_in_thread
from augmentor.privacy import PiiSanitizer, DEFAULT_PATTERNS, EXTRA_PATTERNS

router = APIRouter(tags=["privacy"])


class SanitizeRequest(BaseModel):
    """脱敏请求"""
    input_file: str
    fields: Optional[List[str]] = None
    include_extra: bool = False


class SanitizeReportResponse(BaseModel):
    """脱敏报告

    `total_matches` 是 `SanitizeReport` 上的**计算属性**而非字段，
    必须显式列出，否则会被 response_model 静默裁掉。
    """
    total_items: int
    touched_items: int
    matches: Dict[str, int]
    total_matches: int


class SanitizeResponse(BaseModel):
    """脱敏响应：脱敏后的数据 + 报告"""
    items: List[Dict[str, Any]]
    report: SanitizeReportResponse


class PiiPatternsResponse(BaseModel):
    """可用 PII 模式清单"""
    default: List[str]
    extra: List[str]


@router.post("/api/privacy/sanitize", response_model=SanitizeResponse, summary="PII 脱敏")
async def sanitize_items(request: SanitizeRequest):
    """对数据文件做 PII 脱敏，返回脱敏后数据与报告"""
    try:
        items = load_items(request.input_file)

        def run():
            patterns = dict(DEFAULT_PATTERNS)
            if request.include_extra:
                patterns.update(EXTRA_PATTERNS)
            sanitizer = PiiSanitizer(
                fields=request.fields,
                patterns=patterns,
            )
            sanitized, report = sanitizer.sanitize_dataset(items)
            return {"items": sanitized, "report": report.to_dict()}

        return await run_in_thread(run)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/privacy/patterns", response_model=PiiPatternsResponse, summary="PII 模式清单")
async def list_pii_patterns():
    """列出当前可用的 PII 脱敏模式"""
    return {
        "default": sorted(DEFAULT_PATTERNS.keys()),
        "extra": sorted(EXTRA_PATTERNS.keys()),
    }
