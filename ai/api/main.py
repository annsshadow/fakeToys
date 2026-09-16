"""FastAPI 后端入口 - 优化版"""

import os
import json
from typing import List, Optional
from pathlib import Path
import asyncio

from fastapi import FastAPI, HTTPException, UploadFile, File, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel

import sys
sys.path.insert(0, str(Path(__file__).parent.parent))

from augmentor import AugmentorPipeline, load_config
from augmentor.versioning import VersionInfo
from augmentor.checkpoint import CheckpointData

app = FastAPI(title="AI 数据增强工具", version="1.0.0")

# CORS 配置
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# 全局管道实例
pipeline: Optional[AugmentorPipeline] = None


def get_pipeline() -> AugmentorPipeline:
    """获取管道实例"""
    global pipeline
    if pipeline is None:
        config = load_config("config.yaml")
        pipeline = AugmentorPipeline(config)
    return pipeline


async def read_json_file(file_path: Path) -> list:
    """异步读取 JSON 文件"""
    loop = asyncio.get_event_loop()
    return await loop.run_in_executor(None, _sync_read_json, file_path)


def _sync_read_json(file_path: Path) -> list:
    """同步读取 JSON 文件（在线程池中运行）"""
    with open(file_path, 'r', encoding='utf-8') as f:
        return json.load(f)


async def write_json_file(file_path: Path, data: list):
    """异步写入 JSON 文件"""
    loop = asyncio.get_event_loop()
    await loop.run_in_executor(None, _sync_write_json, file_path, data)


def _sync_write_json(file_path: Path, data: list):
    """同步写入 JSON 文件（在线程池中运行）"""
    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, separators=(',', ':'))


# ============ 数据模型 ============

class AugmentRequest(BaseModel):
    input_file: str
    output_file: str
    use_quality: bool = True
    use_dedup: bool = True
    use_checkpoint: bool = True


class ExportRequest(BaseModel):
    input_file: str
    output_dir: str
    formats: Optional[List[str]] = None


class VersionCreateRequest(BaseModel):
    label: Optional[str] = None
    description: str = ""


class DiffRequest(BaseModel):
    version1: str
    version2: str


# ============ 数据管理 API ============

@app.get("/api/data/list")
async def list_data_files():
    """列出数据文件"""
    loop = asyncio.get_event_loop()
    
    def scan_files():
        data_dir = Path(".")
        files = []
        for f in data_dir.glob("*.json"):
            if f.name.startswith("train_data"):
                files.append({
                    "name": f.name,
                    "path": str(f),
                    "size": f.stat().st_size
                })
        return files
    
    files = await loop.run_in_executor(None, scan_files)
    return {"files": files}


@app.get("/api/data/load/{filename}")
async def load_data(filename: str, page: int = 1, page_size: int = 20, search: str = ""):
    """加载数据（分页）"""
    file_path = Path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")
    
    try:
        items = await read_json_file(file_path)
        
        # 搜索过滤
        if search:
            search_lower = search.lower()
            items = [
                item for item in items
                if search_lower in item.get("instruction", "").lower()
                or search_lower in item.get("output", "").lower()
            ]
        
        total = len(items)
        start = (page - 1) * page_size
        end = start + page_size
        
        return {
            "total": total,
            "page": page,
            "page_size": page_size,
            "items": items[start:end]
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.put("/api/data/update/{filename}")
async def update_data_item(filename: str, index: int, item: dict):
    """更新单条数据"""
    file_path = Path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")
    
    try:
        items = await read_json_file(file_path)
        
        if index < 0 or index >= len(items):
            raise HTTPException(status_code=400, detail="索引越界")
        
        items[index] = item
        await write_json_file(file_path, items)
        
        return {"success": True}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.delete("/api/data/delete/{filename}")
async def delete_data_item(filename: str, index: int):
    """删除单条数据"""
    file_path = Path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")
    
    try:
        items = await read_json_file(file_path)
        
        if index < 0 or index >= len(items):
            raise HTTPException(status_code=400, detail="索引越界")
        
        items.pop(index)
        await write_json_file(file_path, items)
        
        return {"success": True}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/api/data/upload")
async def upload_data(file: UploadFile = File(...)):
    """上传数据文件"""
    try:
        content = await file.read()
        items = json.loads(content.decode('utf-8'))
        
        # 保存到当前目录
        save_path = Path(file.filename)
        await write_json_file(save_path, items)
        
        return {"success": True, "path": str(save_path), "count": len(items)}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/api/data/export")
async def export_data(request: ExportRequest):
    """导出数据"""
    try:
        p = get_pipeline()
        loop = asyncio.get_event_loop()
        results = await loop.run_in_executor(
            None, 
            lambda: p.export_dataset(request.input_file, request.output_dir, request.formats)
        )
        return {"success": True, "files": results}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


# ============ 增强监控 API ============

@app.post("/api/augment/start")
async def start_augmentation(request: AugmentRequest, background_tasks: BackgroundTasks):
    """启动增强任务"""
    try:
        p = get_pipeline()
        
        # 在后台运行
        def run_augment():
            p.augment_dataset(
                request.input_file,
                request.output_file,
                use_checkpoint=request.use_checkpoint,
                use_quality_check=request.use_quality,
                use_dedup=request.use_dedup
            )
        
        background_tasks.add_task(run_augment)
        
        return {"success": True, "message": "增强任务已启动"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/api/augment/progress")
async def get_augment_progress():
    """获取增强进度"""
    try:
        p = get_pipeline()
        progress = p.checkpoint_manager.get_progress()
        return progress
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/api/augment/checkpoints")
async def list_checkpoints():
    """列出断点"""
    try:
        p = get_pipeline()
        checkpoints = p.checkpoint_manager.list_checkpoints()
        return {"checkpoints": checkpoints}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


# ============ 数据分析 API ============

@app.get("/api/analyze/{filename}")
async def analyze_data(filename: str):
    """分析数据"""
    file_path = Path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")
    
    try:
        p = get_pipeline()
        loop = asyncio.get_event_loop()
        result = await loop.run_in_executor(
            None,
            lambda: p.analyze_dataset(str(file_path))
        )
        return result
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/api/visualize/{filename}")
async def visualize_data(filename: str):
    """可视化数据"""
    file_path = Path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")
    
    try:
        p = get_pipeline()
        loop = asyncio.get_event_loop()
        results = await loop.run_in_executor(
            None,
            lambda: p.visualize_dataset(str(file_path))
        )
        return {"charts": results}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


# ============ 版本管理 API ============

@app.get("/api/versions")
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


@app.post("/api/versions/create")
async def create_version(filename: str, request: VersionCreateRequest):
    """创建新版本"""
    file_path = Path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")
    
    try:
        items = await read_json_file(file_path)
        
        p = get_pipeline()
        loop = asyncio.get_event_loop()
        version = await loop.run_in_executor(
            None,
            lambda: p.version_manager.create_version(
                items,
                label=request.label,
                description=request.description
            )
        )
        
        return {
            "success": True,
            "version_id": version.version_id
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/api/versions/{version_id}")
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


@app.get("/api/versions/{version_id}/data")
async def get_version_data(version_id: str):
    """获取版本数据"""
    try:
        p = get_pipeline()
        loop = asyncio.get_event_loop()
        items = await loop.run_in_executor(
            None,
            lambda: p.version_manager.load_version(version_id)
        )
        return {"items": items}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/api/versions/diff")
async def diff_versions(request: DiffRequest):
    """对比两个版本"""
    try:
        p = get_pipeline()
        loop = asyncio.get_event_loop()
        diff = await loop.run_in_executor(
            None,
            lambda: p.version_manager.diff(request.version1, request.version2)
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


@app.post("/api/versions/{version_id}/rollback")
async def rollback_version(version_id: str):
    """回滚到指定版本"""
    try:
        p = get_pipeline()
        loop = asyncio.get_event_loop()
        success = await loop.run_in_executor(
            None,
            lambda: p.version_manager.rollback(version_id)
        )
        return {"success": success}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.delete("/api/versions/{version_id}")
async def delete_version(version_id: str):
    """删除版本"""
    try:
        p = get_pipeline()
        loop = asyncio.get_event_loop()
        await loop.run_in_executor(
            None,
            lambda: p.version_manager.delete_version(version_id)
        )
        return {"success": True}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


# ============ 配置管理 API ============

@app.get("/api/config")
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
            }
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/api/models")
async def list_models():
    """列出可用模型"""
    try:
        p = get_pipeline()
        models = list(p.config.models.keys())
        return {"models": models, "default": p.config.default_model}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


# ============ 健康检查 ============

@app.get("/api/health")
async def health_check():
    """健康检查"""
    return {"status": "ok", "version": "1.0.0"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
