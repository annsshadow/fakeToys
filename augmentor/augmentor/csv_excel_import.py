# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CSV/Excel 数据导入模块

功能：支持从 CSV 和 Excel 文件导入数据集，
自动识别格式并标准化为 JSON 数据格式（instruction / output 字段）。

使用示例：
    from augmentor.csv_excel_import import import_dataset
    data = import_dataset("data.csv")
    # 或指定格式
    data = import_dataset("data.xlsx", format="excel")
"""

import logging
from typing import List, Dict, Optional, Union
from pathlib import Path

logger = logging.getLogger(__name__)

try:
    import pandas as pd
    HAS_PANDAS = True
except ImportError:
    HAS_PANDAS = False


def import_from_csv(file_path: Union[str, Path],
                     text_column: str = "instruction",
                     output_column: Optional[str] = "output") -> List[Dict]:
    """从 CSV 文件导入数据
    
    Args:
        file_path: CSV 文件路径
        text_column: 包含指令的列名
        output_column: 包含输出的列名（可选）
    
    Returns:
        标准格式的数据列表
    """
    if not HAS_PANDAS:
        raise ImportError("需要安装 pandas 才能导入 CSV: pip install pandas")
    
    df = pd.read_csv(file_path)
    results = []
    
    for _, row in df.iterrows():
        item = {
            "instruction": str(row.get(text_column, "")),
            "input": "",
            "output": str(row.get(output_column, "")) if output_column else ""
        }
        results.append(item)
    
    logger.info(f"从 CSV 导入 {len(results)} 条数据: {file_path}")
    return results


def import_from_excel(file_path: Union[str, Path],
                       sheet_name: Union[str, int] = 0,
                       text_column: str = "instruction",
                       output_column: Optional[str] = "output") -> List[Dict]:
    """从 Excel 文件导入数据
    
    Args:
        file_path: Excel 文件路径
        sheet_name: 工作表名称或索引
        text_column: 包含指令的列名
        output_column: 包含输出的列名（可选）
    
    Returns:
        标准格式的数据列表
    """
    if not HAS_PANDAS:
        raise ImportError("需要安装 pandas 才能导入 Excel: pip install pandas openpyxl")
    
    df = pd.read_excel(file_path, sheet_name=sheet_name)
    results = []
    
    for _, row in df.iterrows():
        item = {
            "instruction": str(row.get(text_column, "")),
            "input": "",
            "output": str(row.get(output_column, "")) if output_column else ""
        }
        results.append(item)
    
    logger.info(f"从 Excel 导入 {len(results)} 条数据: {file_path}")
    return results


def import_dataset(file_path: Union[str, Path], format: Optional[str] = None) -> List[Dict]:
    """自动识别格式并导入数据集
    
    Args:
        file_path: 文件路径
        format: 指定格式（"csv" 或 "excel"），为 None 时自动识别
    
    Returns:
        标准格式的数据列表
    """
    path = Path(file_path)
    suffix = path.suffix.lower()
    
    if format is None:
        if suffix == ".csv":
            format = "csv"
        elif suffix in (".xlsx", ".xls"):
            format = "excel"
        else:
            raise ValueError(f"无法识别文件格式: {suffix}，请指定 format 参数")
    
    if format == "csv":
        return import_from_csv(path)
    elif format == "excel":
        return import_from_excel(path)
    else:
        raise ValueError(f"不支持的导入格式: {format}")


def export_to_csv(items: List[Dict],
                  file_path: Union[str, Path],
                  columns: Optional[List[str]] = None) -> int:
    """导出数据集到 CSV 文件

    Args:
        items: 数据列表
        file_path: 输出 CSV 路径
        columns: 指定列顺序，缺省为所有字段的并集

    Returns:
        写入的行数
    """
    if not HAS_PANDAS:
        raise ImportError("需要安装 pandas 才能导出 CSV: pip install pandas")

    df = pd.DataFrame(items)
    if columns:
        missing = [c for c in columns if c not in df.columns]
        if missing:
            raise ValueError(f"以下列不存在: {missing}")
        df = df[columns]
    out = Path(file_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    df.to_csv(out, index=False, encoding="utf-8-sig")
    logger.info(f"导出 {len(df)} 条数据到 CSV: {file_path}")
    return len(df)


def export_to_excel(items: List[Dict],
                    file_path: Union[str, Path],
                    sheet_name: str = "data",
                    columns: Optional[List[str]] = None) -> int:
    """导出数据集到 Excel 文件

    Args:
        items: 数据列表
        file_path: 输出 Excel 路径
        sheet_name: 工作表名称
        columns: 指定列顺序

    Returns:
        写入的行数
    """
    if not HAS_PANDAS:
        raise ImportError("需要安装 pandas 才能导出 Excel: pip install pandas openpyxl")

    df = pd.DataFrame(items)
    if columns:
        missing = [c for c in columns if c not in df.columns]
        if missing:
            raise ValueError(f"以下列不存在: {missing}")
        df = df[columns]
    out = Path(file_path)
    out.parent.mkdir(parents=True, exist_ok=True)
    df.to_excel(out, sheet_name=sheet_name, index=False)
    logger.info(f"导出 {len(df)} 条数据到 Excel: {file_path}")
    return len(df)
