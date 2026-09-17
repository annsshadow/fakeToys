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


# ============ 演示数据 ============

_DEMO_DATA = [
    {
        "instruction": "什么是人工智能？",
        "input": "",
        "output": "人工智能（AI）是计算机科学的一个分支，致力于创建能够执行通常需要人类智能的任务的系统，如学习、推理、问题解决、感知和语言理解。"
    },
    {
        "instruction": "解释一下机器学习的基本概念",
        "input": "",
        "output": "机器学习是人工智能的子领域，它使计算机能够从数据中学习并做出决策或预测，而无需显式编程。主要类型包括监督学习、无监督学习和强化学习。"
    },
    {
        "instruction": "深度学习与传统机器学习有什么区别？",
        "input": "",
        "output": "深度学习使用多层神经网络自动提取特征，适合处理大量数据；传统机器学习通常需要人工特征工程，在小数据集上可能更高效。"
    },
    {
        "instruction": "什么是自然语言处理（NLP）？",
        "input": "",
        "output": "自然语言处理是AI的一个分支，专注于让计算机理解、解释和生成人类语言。应用包括机器翻译、情感分析、聊天机器人和文本摘要。"
    },
    {
        "instruction": "请介绍一下计算机视觉的基本任务",
        "input": "",
        "output": "计算机视觉的基本任务包括图像分类、目标检测、语义分割、实例分割和图像生成。这些技术应用于自动驾驶、医学影像、安防监控等领域。"
    }
]


@router.get("/api/demo/data")
async def get_demo_data():
    """获取内置演示数据集，开箱即用"""
    from fastapi.responses import JSONResponse
    return JSONResponse(_DEMO_DATA)
