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
from .exceptions import DataValidationError, UnsupportedFormatError

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
    
    df = pd.read_excel(safe_sheet_source(file_path, Path(file_path).name), sheet_name=sheet_name)
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
            raise UnsupportedFormatError(f"无法识别文件格式: {suffix}，请指定 format 参数")
    
    if format == "csv":
        return import_from_csv(path)
    elif format == "excel":
        return import_from_excel(path)
    else:
        raise UnsupportedFormatError(f"不支持的导入格式: {format}")


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
            raise DataValidationError(f"以下列不存在: {missing}")
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

    **L85 起本函数只是转接口，实现住在 `augmentor/excel_write.py`**，理由是那条路
    不该把 pandas 拉进进程（本模块 import 时就会 `import pandas`，实测一次 371.8 ms）。
    公开签名、返回语义与报错口径都不动，所以直接调用本函数的老代码一行都不用改；
    真正写 xlsx 的产品调用点仍然只有一处（`converter._write_excel`，现在直接指向
    `excel_write`），那一条由 `test_excel_write_edge_l84.py` 的调用点普查守着。
    """
    from .excel_write import export_table_to_excel
    return export_table_to_excel(items, file_path,
                                 sheet_name=sheet_name, columns=columns)


def safe_sheet_source(source: Union[str, Path], name: str) -> Union[str, Path]:
    """zip 容器（xlsx）交给解析器**之前**过一道解压放大判据，返回同一个可继续读的对象

    住在这一只文件是因为两条 Excel 读边都引得到它：本模块的 `import_from_excel` 直接
    调，`converter._read_excel_source` 则由本模块 import 进来（那只方法里现成就有
    `from . import csv_excel_import as cei`）⇒ 接上闸门不需要在任何一只文件里新增 import
    行，也因此不平移 `converter.py` 里高于本函数的历史行号引用。判据本体不住这里，
    三条阈值与判决住在 `augmentor/validation.py`（A77 一份权威只住一处），这里只做
    「读容器的账」这一件机械事。

    **为什么第二臂要真解压一遍数字节**：只查中央目录声明值那一臂挡不住谎报档。实测把一
    份 51.4 MiB 工作簿在中央目录里的声明解压后尺寸改成 1000 字节（CRC 与压缩尺寸保持
    真值），声明侧读数是「总共 2.4 KiB、比率 0.00」⇒ 只看声明值的闸门当场放它过去；
    本仓这一版 Python 的 `zipfile` 在 `read()` 收尾时以 `Bad CRC-32` 拒绝了那份谎报档
    （pandas 侧同一形），但那是解释器实现的副产品，不是本判据给的保证 ⇒ 展开量必须由
    自己数出来的实际字节兜住，不能指望下游报错。

    计数走 1 MiB 分块并**边读边比上限、超了就早停**，所以炸弹在第一兆之后就被掐住，
    闸门自己不会变成第二处内存开销。成本实测（本轮现量）：合法档案上这一遍计数占
    `pandas.read_excel` 自身耗时的 1%（51.4 MiB 的档案 0.05 s 对 8.67 s）。

    认不出 zip 结构的（老 `.xls` 是 BIFF8、不是 zip，损坏的表头同理）一律**放行**，
    让原来的路径去说「读不出工作表」：`.xls` 没有容器压缩，展开量就等于文件本身的大小，
    已经被上传上限管住，不在这一族的判据面里；而在闸门里把它拒掉会改变既有报错口径。

    Args:
        source: 路径或已打开的二进制对象（`BytesIO` 与 `ZipFile` 同意的都行）
        name: 只进报错文案

    Returns:
        进入时的同一个对象；文件对象已被 `seek(0)` 回卷，可以原样交给解析器
    """
    import zipfile

    from .validation import MAX_SHEET_EXPANDED_BYTES, sheet_expansion_verdict
    try:
        container = zipfile.ZipFile(source)
    except (zipfile.BadZipFile, EOFError, OSError, ValueError):
        return source
    # 容器由 with 关，不写裸 `finally: container.close()`：那一格属于 A378 那一族（清理步
    # 顶替真正的失败），而这句紧跟的 `source.seek(0)` 也会被一起跳过。
    with container:
        infos = container.infolist()
        compressed = sum(info.compress_size for info in infos)
        sheet_expansion_verdict(
            compressed, sum(info.file_size for info in infos), name, "中央目录声明")
        expanded = 0
        for info in infos:
            with container.open(info) as stream:
                for chunk in iter(lambda: stream.read(1024 * 1024), b""):
                    expanded += len(chunk)
                    if expanded > MAX_SHEET_EXPANDED_BYTES:
                        break
            if expanded > MAX_SHEET_EXPANDED_BYTES:
                break
        sheet_expansion_verdict(compressed, expanded, name, "实际字节流")
    if hasattr(source, "seek"):
        source.seek(0)
    return source
