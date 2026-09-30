# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""Excel **写**边的独立模块（只依赖 openpyxl）

为什么单独一份而不是留在 `csv_excel_import.py` 里：那个模块在 import 时就 `import pandas`，
而 `df.to_excel()` 对写一张表只做三件事 —— 把记录列表拼成矩形、按列序落单元格、把值交给
openpyxl。前两件各一次线性遍历就够，第三件本来就是 openpyxl 干的。只要实现还住在那个模块里，
写 xlsx 就要先付一次 pandas 的导入税（现量：干净进程里 `import pandas` **371.8 ms**、
`import openpyxl` **242.2 ms**，`C:/Python314/python.exe`，min-of-3）。

所以这里的判据是：**写边不许把 pandas 拉进进程**。这条不是风格，是可测的毫秒 ——
L85 终态 A/B（同一 CLI 入口、同一份 6902 条语料、各自 min-of-5，两轮独立跑）：
写 xlsx 单程 **1290.8 → 1031.8 ms** 与 **1146.9 → 789.0 ms**，即省 **259.0 ~ 357.9 ms**
（1.25× ~ 1.45×；每条 166~187 µs 降到 114~150 µs）。省下的量与 `import pandas` 那
371.8 ms 对得上，剩下的 xlsx/csv 差距（残差 342~438 ms）是 openpyxl 自己的 242 ms 导入
与逐格写盘，属于「xlsx 是 zip 容器」的固有代价，不在本轮射程内。守卫在
`tests/unit/test_excel_write_native_l85.py`：干净子进程里跑完 convert 之后断言
`'pandas' not in sys.modules`。

读边仍然走 pandas（`converter._read_excel` → `csv_excel_import`），本模块**不**负责：
`pd.read_excel` 的剥列/类型推断口径与写边不同，改写它要另立一轮。
"""

import logging
import math
from collections.abc import Mapping
from pathlib import Path
from typing import Any, Dict, List, Optional, Union

from .exceptions import DataValidationError

logger = logging.getLogger(__name__)

try:
    from openpyxl import Workbook
    HAS_OPENPYXL = True
except ImportError:
    HAS_OPENPYXL = False


def excel_columns(items: List[Dict], columns: Optional[List[str]]) -> List[Any]:
    """定列：`columns` 缺省时按**首次出现顺序**取键并集，给了就按它并校验存在性

    列序规则必须与 `pd.DataFrame(list_of_dicts)` 逐字相同，否则同一份数据在「pandas 路」
    与「openpyxl 路」之间会给出表头顺序不同的两张表（L84 定下的口径：csv 写边与 xlsx
    写边必须是同一张表头）。pandas 的做法就是按行扫描、把没见过的新键追加到尾部，
    所以这里同样只用一次遍历。

    非对象记录在**这里**拦，判在任何写盘动作之前：pandas 路对 `[{"a":1}, 5]` 这种
    ragged 输入抛的是 `TypeError: 'int' object is not iterable`（L85 golden `c24/c25`
    现量），对全标量输入则**静默产出一张列名叫 `0` 的表**（`c13`）。两者都不是本函数
    声明的 `List[Dict]`，改成点名记录序号的 `DataValidationError` 是与
    `converter._require_object_records` 同一把刀，不是新发明。
    """
    seen: List[Any] = []
    known = set()
    for pos, rec in enumerate(items, 1):
        if not isinstance(rec, Mapping):
            raise DataValidationError(
                f"第 {pos} 条记录必须是 JSON 对象，得到 {type(rec).__name__}"
            )
        for key in rec:
            if key not in known:
                known.add(key)
                seen.append(key)
    if not columns:
        return seen
    missing = [c for c in columns if c not in known]
    if missing:
        raise DataValidationError(f"以下列不存在: {missing}")
    return list(columns)


def excel_cell(value: Any) -> Any:
    """值 → 单元格，逐条对齐 pandas 路的落地形状（L85 golden 那 30 档现量）

    - `None` / `NaN` → 空格；
    - `±inf` → 字面量 `"inf"` / `"-inf"`（pandas 把非有限浮点转成字符串交给 openpyxl，
      因为 xlsx 的数字类型装不下它）；
    - 列表 / 字典 / 元组 / 集合 / bytes → `str()` 化（`"['a', 'b']"`、`"{1, 2}"`、
      `"b'xy'"`）—— 与 csv 写边对同样输入给的字面量一致，这条是 L84 定下的契约；
    - 空串**不**特殊处理：pandas 路里它是 openpyxl 写的空单元格，读回来同样是 `None`；
    - 前导 `=` 的字符串两条路都被 openpyxl 当公式存（golden `c07`：读回 `None`），
      本函数**不**悄悄改写它 —— 那是另一个契约，L85 只在账上记它没变。
    - 超过 32767 字符的串也不在这里截：截是 openpyxl 干的（两条路读回都是 32767，
      L85 现量），本函数只负责不撒谎。报警在 `export_table_to_excel` 那一头。
    """
    if value is None:
        return None
    if isinstance(value, bool):
        return value
    if isinstance(value, float):
        if math.isnan(value):
            return None
        if not math.isfinite(value):
            return str(value)
        return value
    if isinstance(value, (list, dict, tuple, set, frozenset, bytes, bytearray)):
        return str(value)
    return value


def export_table_to_excel(items: List[Dict],
                          file_path: Union[str, Path],
                          sheet_name: str = "data",
                          columns: Optional[List[str]] = None) -> int:
    """把对象记录列表写成一张 `.xlsx` 表，返回写入的行数

    Args:
        items: 数据列表（每条必须是一个对象记录）
        file_path: 输出 Excel 路径
        sheet_name: 工作表名称
        columns: 指定列顺序，缺省为按首次出现顺序的键并集

    为什么不走 `write_only=True` 之外更强的方案（比如自己拼 zip + XML）：那一档能再省
    掉 openpyxl 的 242 ms 导入税，但要自己维护 xlsx 的 schema、共享字符串表与
    32767 字符截断规则 —— 那不是优化，是把一个格式规范搬进本仓。等真有大表诉求再说。

    **依赖门排在内容门之后**，与 `converter._write_excel` 同一条顺序契约：数据里有标量行
    这件事跟本机装没装 openpyxl 无关，先报「装依赖」会把一个改数据就能修好的错误说成
    改依赖才能修好（aug 侧解释器就是那种环境）。L85 定这条时两侧实测：顺序没对上时，
    本模块那 8 格内容判据在装了 openpyxl 的一侧全绿、在没装的一侧全红 —— 即它们在
    最需要它们的那一侧从不被验证。
    """
    cols = excel_columns(items, columns)
    if not HAS_OPENPYXL:
        raise ImportError("需要安装 openpyxl 才能导出 Excel: pip install openpyxl")

    out = Path(file_path)
    out.parent.mkdir(parents=True, exist_ok=True)

    wb = Workbook(write_only=True)
    try:
        ws = wb.create_sheet(title=sheet_name)
        # 一列都没有时，pandas 路连表头都不写，逐行也不写（golden `c26/c27`：返回行数
        # 1/2 而读回是空表）。这里必须一样：没有列就没有行。
        truncated = 0
        if cols:
            ws.append(cols)
            for rec in items:
                row = [excel_cell(rec.get(c)) for c in cols]
                truncated += sum(1 for v in row if isinstance(v, str) and len(v) > 32767)
                ws.append(row)
        wb.save(out)
    finally:
        wb.close()

    if truncated:
        # openpyxl 在 write_only 模式下把超长串**静默**截到 32767（非 write_only 那条
        # 会 UserWarning）。装依赖换了实现就不出声，等于把一次数据删改藏起来 ⇒ 补一帧
        # warning，与 pandas 路的响度对齐。
        logger.warning(
            f"导出 Excel 时有 {truncated} 个单元格超过 32767 字符，已截断写入: {file_path}"
        )

    logger.info(f"导出 {len(items)} 条数据到 Excel: {file_path}")
    return len(items)
