# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""版本管理 API 路由"""

from typing import Optional

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from ..deps import get_pipeline, read_json_file, require_file, run_in_thread, verify_api_key

router = APIRouter(tags=["version"])


class VersionCreateRequest(BaseModel):
    """版本创建请求"""
    label: Optional[str] = None
    description: str = ""


class DiffRequest(BaseModel):
    """版本对比请求"""
    version1: str
    version2: str


@router.get("/api/versions")
async def list_versions():
    """列出所有版本"""
    try:
        p = get_pipeline()
        versions = p.version_manager.list_versions()
        return {
            "versions": [
                {
                    "version_id": v.version_id,
                    "label": v.label,
                    "description": v.description,
                    "created_at": v.created_at,
                    "item_count": v.item_count
                }
                for v in versions
            ]
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/api/versions/create")
async def create_version(
    filename: str,
    request: VersionCreateRequest,
    _auth: None = Depends(verify_api_key),
):
    """创建新版本"""
    file_path = require_file(filename)

    try:
        items = await read_json_file(file_path)

        p = get_pipeline()
        version = await run_in_thread(
            p.version_manager.create_version,
            items,
            request.label,
            request.description
        )

        return {"success": True, "version_id": version.version_id}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/api/versions/diff")
async def diff_versions(request: DiffRequest):
    """对比两个版本"""
    try:
        p = get_pipeline()
        diff = await run_in_thread(
            p.version_manager.diff, request.version1, request.version2
        )
        return {
            "version1": diff.version1,
            "version2": diff.version2,
            "added_count": diff.added_count,
            "removed_count": diff.removed_count,
            "modified_count": diff.modified_count
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/versions/history")
async def version_history(limit: int = 20):
    """获取版本操作历史"""
    try:
        p = get_pipeline()
        return {"history": p.version_manager.get_history(limit=limit)}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/versions/{version_id}")
async def get_version(version_id: str):
    """获取版本信息"""
    try:
        p = get_pipeline()
        version = p.version_manager.get_version_info(version_id)
        return {
            "version_id": version.version_id,
            "label": version.label,
            "description": version.description,
            "created_at": version.created_at,
            "item_count": version.item_count,
            "metadata": version.metadata
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/versions/{version_id}/data")
async def get_version_data(version_id: str):
    """获取版本数据"""
    try:
        p = get_pipeline()
        items = await run_in_thread(p.version_manager.load_version, version_id)
        return {"items": items}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/api/versions/{version_id}/rollback")
async def rollback_version(
    version_id: str, _auth: None = Depends(verify_api_key)
):
    """回滚到指定版本"""
    try:
        p = get_pipeline()
        success = await run_in_thread(p.version_manager.rollback, version_id)
        return {"success": success}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.delete("/api/versions/{version_id}")
async def delete_version(
    version_id: str, _auth: None = Depends(verify_api_key)
):
    """删除版本"""
    try:
        p = get_pipeline()
        await run_in_thread(p.version_manager.delete_version, version_id)
        return {"success": True}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
