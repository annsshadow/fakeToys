# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""版本管理 API 路由"""

from typing import Any, Dict, List, Optional

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from augmentor.validation import require_count
from ..deps import raise_boundary_error, get_pipeline, read_json_file, require_file, run_in_thread, verify_api_key
from augmentor.exceptions import VersionError
from ..schemas import SuccessResponse

router = APIRouter(tags=["version"])


class VersionInfo(BaseModel):
    """版本摘要

    `label` 允许为 null（创建时未指定），因此声明为可选且**不**启用
    `exclude_none`——键必须始终存在。
    """
    version_id: str
    label: Optional[str]
    description: str
    created_at: str
    item_count: int


class VersionDetailResponse(VersionInfo):
    """版本详情：在摘要基础上多一份 `metadata`"""
    metadata: Dict[str, Any]


class VersionListResponse(BaseModel):
    """版本列表"""
    versions: List[VersionInfo]


class VersionCreateResponse(BaseModel):
    """版本创建结果"""
    success: bool
    version_id: str


class VersionDiffResponse(BaseModel):
    """版本对比结果

    `added_count` / `removed_count` / `modified_count` 以 version1 为基准。
    """
    version1: str
    version2: str
    added_count: int
    removed_count: int
    modified_count: int


class VersionHistoryResponse(BaseModel):
    """版本操作历史（由版本目录下的操作日志 JSONL 支撑）"""
    history: List[Dict[str, Any]]


class VersionDataResponse(BaseModel):
    """某个版本的数据快照。

    第 52 轮：支持分页。不带 page_size（默认 0）时返回全量（向后兼容
    旧客户端）；带 page_size>0 时返回该页切片。total 恒为全量条数。
    """
    items: List[Dict[str, Any]]
    total: int = 0
    page: int = 1
    page_size: int = 0


class VersionCreateRequest(BaseModel):
    """版本创建请求"""
    label: Optional[str] = None
    description: str = ""


class DiffRequest(BaseModel):
    """版本对比请求"""
    version1: str
    version2: str


@router.get("/api/versions", response_model=VersionListResponse, summary="列出所有版本")
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
        raise_boundary_error(e)


@router.post(
    "/api/versions/create",
    response_model=VersionCreateResponse,
    summary="创建新版本",
)
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
        raise_boundary_error(e)


@router.post(
    "/api/versions/diff",
    response_model=VersionDiffResponse,
    summary="对比两个版本",
)
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
        raise_boundary_error(
            e, not_found="版本不存在", not_found_types=(VersionError,)
        )


@router.get(
    "/api/versions/history",
    response_model=VersionHistoryResponse,
    summary="获取版本操作历史",
)
async def version_history(limit: int = 20):
    """获取版本操作历史"""
    try:
        p = get_pipeline()
        return {"history": p.version_manager.get_history(limit=limit)}
    except Exception as e:
        raise_boundary_error(e)


@router.get(
    "/api/versions/{version_id}",
    response_model=VersionDetailResponse,
    summary="获取版本信息",
)
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
        raise_boundary_error(
            e, not_found="版本不存在", not_found_types=(VersionError,)
        )


@router.get(
    "/api/versions/{version_id}/data",
    response_model=VersionDataResponse,
    summary="获取版本数据",
)
async def get_version_data(version_id: str, page: int = 1, page_size: int = 0):
    """获取版本数据（第 52 轮起支持分页）。

    `page_size` 默认 0 = 返回全量（兼容旧客户端）；>0 时返回该页切片。
    `page_size` 上限 1000：与 /api/data/load 同口径，防止一次序列化整份
    超大版本快照打爆服务端内存。
    """
    try:
        page = require_count("page", page, minimum=1)
        page_size = require_count("page_size", page_size, minimum=0, maximum=1000)
        p = get_pipeline()
        items = await run_in_thread(p.version_manager.load_version, version_id)
        total = len(items)
        if page_size > 0:
            start = (page - 1) * page_size
            items = items[start : start + page_size]
        return {"items": items, "total": total, "page": page, "page_size": page_size}
    except Exception as e:
        raise_boundary_error(
            e, not_found="版本不存在", not_found_types=(VersionError,)
        )


@router.post(
    "/api/versions/{version_id}/rollback",
    response_model=SuccessResponse,
    summary="回滚到指定版本",
)
async def rollback_version(
    version_id: str, _auth: None = Depends(verify_api_key)
):
    """回滚到指定版本"""
    try:
        p = get_pipeline()
        success = await run_in_thread(p.version_manager.rollback, version_id)
        return {"success": success}
    except Exception as e:
        raise_boundary_error(e)


@router.delete(
    "/api/versions/{version_id}",
    response_model=SuccessResponse,
    summary="删除版本",
)
async def delete_version(
    version_id: str, _auth: None = Depends(verify_api_key)
):
    """删除版本"""
    try:
        p = get_pipeline()
        await run_in_thread(p.version_manager.delete_version, version_id)
        return {"success": True}
    except Exception as e:
        raise_boundary_error(e)
