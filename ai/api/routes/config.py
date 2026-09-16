"""配置与模型 API 路由"""

from fastapi import APIRouter, HTTPException

from ..deps import get_pipeline

router = APIRouter(tags=["config"])


@router.get("/api/config")
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


@router.get("/api/models")
async def list_models():
    """列出可用模型"""
    try:
        p = get_pipeline()
        models = list(p.config.models.keys())
        return {"models": models, "default": p.config.default_model}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
