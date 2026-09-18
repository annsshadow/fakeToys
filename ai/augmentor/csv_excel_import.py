"""CSV/Excel 数据导入模块 - 功能完善

支持从 CSV 和 Excel 文件导入数据集，并自动转换为标准 JSON 格式。
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
