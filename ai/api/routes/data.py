"""数据管理 API 路由"""

import asyncio
import json
from pathlib import Path

from fastapi import APIRouter, HTTPException, UploadFile, File

from ..deps import get_pipeline, read_json_file, run_in_thread, write_json_file

router = APIRouter(tags=["data"])


def _safe_data_path(filename: str) -> Path:
    """将文件名规范化为安全路径，防止路径遍历"""
    path = Path(filename)
    # 禁止包含 .. 组件，防止跳出目录
    parts = path.parts
    if ".." in parts:
        raise HTTPException(status_code=400, detail="路径包含非法组件")
    return path.resolve()


@router.get("/api/data/list")
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


@router.get("/api/data/load/{filename}")
async def load_data(filename: str, page: int = 1, page_size: int = 20, search: str = ""):
    """加载数据（分页）"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        items = await read_json_file(file_path)

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


@router.put("/api/data/update/{filename}")
async def update_data_item(filename: str, index: int, item: dict):
    """更新单条数据"""
    file_path = _safe_data_path(filename)
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


@router.delete("/api/data/delete/{filename}")
async def delete_data_item(filename: str, index: int):
    """删除单条数据"""
    file_path = _safe_data_path(filename)
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


@router.post("/api/data/upload")
async def upload_data(file: UploadFile = File(...)):
    """上传数据文件"""
    try:
        content = await file.read()
        items = json.loads(content.decode('utf-8'))

        safe_name = Path(file.filename).name
        save_path = _safe_data_path(safe_name)
        await write_json_file(save_path, items)

        return {"success": True, "path": str(save_path), "count": len(items)}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


# ============ 数据分析 ============

@router.get("/api/analyze/{filename}")
async def analyze_data(filename: str):
    """分析数据集（覆盖度、统计信息、去重报告）"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        p = get_pipeline()
        return await run_in_thread(p.analyze_dataset, str(file_path))
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/api/visualize/{filename}")
async def visualize_data(filename: str):
    """生成数据集可视化图表"""
    file_path = _safe_data_path(filename)
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="文件不存在")

    try:
        p = get_pipeline()
        results = await run_in_thread(p.visualize_dataset, str(file_path))
        return {"charts": results}
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
