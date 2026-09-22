"""CLI 共用 I/O helper（由 cli.py 拆出）"""

import json
from pathlib import Path


def _load_items(path: str):
    """加载 JSON 数据文件

    Args:
        path: 文件路径

    Returns:
        数据列表
    """
    with open(path, 'r', encoding='utf-8') as f:
        return json.load(f)


def _save_items(items, path: str):
    """保存数据到 JSON 文件

    Args:
        path: 文件路径
    """
    output_path = Path(path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(items, f, ensure_ascii=False, indent=2)


def _dump_json(data, path: str):
    """将任意可 JSON 序列化对象写入文件

    Args:
        data: 可 JSON 序列化对象
        path: 输出文件路径
    """
    output_path = Path(path)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=2, default=str)


def _print(data):
    """打印 JSON 结果

    Args:
        data: 任意可序列化对象
    """
    print(json.dumps(data, ensure_ascii=False, indent=2, default=str))
