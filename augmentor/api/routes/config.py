# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""配置与模型 API 路由"""

from typing import Any, Dict, List

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from ..deps import get_pipeline, verify_api_key
from ..schemas import MessageResponse

router = APIRouter(tags=["config"])


class ConfigResponse(BaseModel):
    """当前生效的配置

    只暴露各分区的**关键项**，不是 `AppConfig` 的完整镜像——密钥等敏感字段
    不会出现在响应里。各分区用 `Dict` 承载，因此新增配置项不需要改这个模型。
    """
    default_model: str
    augmentation: Dict[str, Any]
    quality: Dict[str, Any]
    dedup: Dict[str, Any]
    export: Dict[str, Any]
    vector: Dict[str, Any]
    rag: Dict[str, Any]
    multimodal: Dict[str, Any]


class ModelsResponse(BaseModel):
    """可用模型清单"""
    models: List[str]
    default: str


class ConfigUpdateRequest(BaseModel):
    """配置更新请求"""
    default_model: str | None = None
    augmentation: dict | None = None
    quality: dict | None = None
    dedup: dict | None = None
    export: dict | None = None
    vector: dict | None = None
    rag: dict | None = None
    multimodal: dict | None = None


@router.get("/api/config", response_model=ConfigResponse, summary="获取配置")
async def get_config():
    """获取配置"""
    try:
        p = get_pipeline()
        return {
            "default_model": p.config.default_model,
            "augmentation": {
                "variants_per_seed": p.config.augmentation.variants_per_seed,
                "num_threads": p.config.augmentation.num_threads
            },
            "quality": {
                "enabled": p.config.quality.enabled,
                "threshold": p.config.quality.threshold
            },
            "dedup": {
                "enabled": p.config.dedup.enabled,
                "threshold": p.config.dedup.threshold
            },
            "export": {
                "default_format": p.config.export.default_format,
                "formats": p.config.export.formats
            },
            "vector": {
                "enabled": p.config.vector.enabled,
                "backend": p.config.vector.backend
            },
            "rag": {
                "enabled": p.config.rag.enabled,
                "default_format": p.config.rag.default_format
            },
            "multimodal": {
                "enabled": p.config.multimodal.enabled
            }
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/config",
    response_model=MessageResponse,
    summary="更新配置",
)
async def update_config(
    request: ConfigUpdateRequest, _auth: None = Depends(verify_api_key)
):
    """更新配置并持久化到 config.yaml"""
    try:
        from augmentor.config import save_config
        
        p = get_pipeline()
        updates = request.model_dump(exclude_none=True)
        
        if "default_model" in updates:
            p.config.default_model = updates["default_model"]
        
        for section in ["augmentation", "quality", "dedup", "export", "vector", "rag", "multimodal"]:
            if section in updates:
                section_config = getattr(p.config, section)
                for key, value in updates[section].items():
                    if hasattr(section_config, key):
                        setattr(section_config, key, value)
        
        save_config(p.config, "config.yaml")
        return {"success": True, "message": "配置已保存，部分配置需要重启服务生效"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/models", response_model=ModelsResponse, summary="列出可用模型")
async def list_models():
    """列出可用模型"""
    try:
        p = get_pipeline()
        models = list(p.config.models.keys())
        return {"models": models, "default": p.config.default_model}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
