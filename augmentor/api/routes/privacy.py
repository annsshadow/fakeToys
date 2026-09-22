"""PII 脱敏 API 路由"""

from typing import List, Optional

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import load_items, run_in_thread
from augmentor.privacy import PiiSanitizer, DEFAULT_PATTERNS, EXTRA_PATTERNS, SanitizeReport

router = APIRouter(tags=["privacy"])


class SanitizeRequest(BaseModel):
    """脱敏请求"""
    input_file: str
    fields: Optional[List[str]] = None
    include_extra: bool = False


@router.post("/api/privacy/sanitize")
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


@router.get("/api/privacy/patterns")
async def list_pii_patterns():
    """列出当前可用的 PII 脱敏模式"""
    return {
        "default": sorted(DEFAULT_PATTERNS.keys()),
        "extra": sorted(EXTRA_PATTERNS.keys()),
    }
