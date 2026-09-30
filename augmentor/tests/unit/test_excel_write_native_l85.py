# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L85（写边摘掉 pandas）：`augmentor/excel_write.py` 的等价面与导入守卫

立项事实（L85 现量，`C:/Python314/python.exe`，min-of-7 / min-of-3）：同一份 6902 条
语料，CLI 单程 `convert --output out.xlsx` 比 `out.csv` 多花 **906.5 ms**，而干净进程里
`import pandas` 单次 **371.8 ms**、`import openpyxl` **242.2 ms**。写 xlsx 这件事本身
只有 openpyxl 在干活，pandas 只是把记录列表拼成矩形 —— 那是一趟线性遍历。

所以本轮把写边搬进只依赖 openpyxl 的独立模块，并立三族判据：

1. **导入守卫（子进程）**：走产品转换器写出 xlsx 的那条路**不许把 pandas 拉进进程**。
   这条只能在干净子进程里验 —— 测试进程早就被别的用例 import 过 pandas 了，
   在本进程查 `sys.modules` 必然假绿。
2. **单元格等价**：与 pandas 路逐格对照，覆盖 `None`/`NaN`/`±inf`/布尔/容器/bytes/
   空串/超长串/前导 `=`。判据取「读回来的值」而不是「zip 里的 XML」：write_only 模式
   对整行为空的尾部格子不落 `<c>` 元素，那是 openpyxl 的结构差异，Excel 与
   `pd.read_excel` 都看不出区别（L85 现量：消费侧读回 50/50 SAME）。
3. **列序与报错**：表头必须是「按首次出现顺序的键并集」（与 `pd.DataFrame(list_of_dicts)`
   同口径，且与 csv 写边共用同一把尺子），非对象记录要点名序号。
4. **依赖探测那两行自己也要被跑到**：`except ImportError: HAS_OPENPYXL = False` 在装了
   openpyxl 的一侧从不执行 ⇒ 覆盖缺数 +2 且**没有任何用例为它负责**。monkeypatch 那个
   标志位只测了「代码读它」，没测「真缺依赖时它会被设成 False」，所以这里用 reload 把
   那条分支本身走一遍（同族先例是 `test_micro_branches_l61.py` 对 `HAS_PANDAS` 的做法）。

全部落盘走 `tmp_path` 而不是 `mkdtemp`：写边一旦拦在依赖门之前就**不**落文件，而拦不住
时留下的每一份都得由 pytest 自己清掉（本轮改掉的两条老用例正是靠相对路径把 xlsx 写进了
真 CWD）。
"""

import ast
import subprocess
import sys
import zipfile
from pathlib import Path

import pytest

from augmentor import excel_write
from augmentor.csv_excel_import import export_to_excel
from augmentor.exceptions import DataValidationError

REPO = Path(__file__).resolve().parents[2]


def _installed(name: str) -> bool:
    import importlib.util
    return importlib.util.find_spec(name) is not None


requires_openpyxl = pytest.mark.skipif(
    not excel_write.HAS_OPENPYXL, reason="写 xlsx 需要 openpyxl（aug 侧解释器没有）"
)
requires_both = pytest.mark.skipif(
    not (_installed("pandas") and excel_write.HAS_OPENPYXL),
    reason="与 pandas 路对照需要 pandas 与 openpyxl 都在"
)


def pandas_edge(items, path, sheet_name="data", columns=None):
    """老路（本轮之前产品唯一写法）：pandas 拼框再 `to_excel`"""
    import pandas as pd

    frame = pd.DataFrame(items)
    if columns:
        frame = frame[columns]
    frame.to_excel(path, sheet_name=sheet_name, index=False)


def cells_of(path):
    """读回首个工作表的全部格子（含表头），按行返回"""
    from openpyxl import load_workbook

    book = load_workbook(path)
    try:
        return [list(row) for row in book.worksheets[0].iter_rows(values_only=True)]
    finally:
        book.close()


def both_edges(tmp_path, items, columns=None):
    """两条路各写一份到 `tmp_path`，返回 (新格, 老格, 新路边数)"""
    new, old = tmp_path / "new.xlsx", tmp_path / "old.xlsx"
    written = excel_write.export_table_to_excel(items, str(new), columns=columns)
    pandas_edge(items, str(old), columns=columns)
    return cells_of(new), cells_of(old), written


# ==================== 1. 导入守卫（子进程，两侧解释器都真跑） ====================

class TestWriteEdgeStaysPandasFree:
    def test_importing_the_module_pulls_no_pandas(self):
        """`import augmentor.excel_write` 本身不许顺手把 pandas 拽进来

        这条不需要 openpyxl，所以在两侧解释器上都是真跑（aug 侧恰恰是那个「装了
        也用不上 pandas」的部署形态）。`cwd=REPO` 让 `-c` 的 `sys.path[0]` 就是仓根。
        """
        code = (
            "import sys; import augmentor.excel_write as ew; "
            "print('PANDAS=' + str('pandas' in sys.modules)); "
            "print('FLAG=' + str(ew.HAS_OPENPYXL))"
        )
        result = subprocess.run(
            [sys.executable, "-c", code], capture_output=True, text=True,
            encoding="utf-8", errors="replace", cwd=str(REPO),
        )
        assert result.returncode == 0, result.stderr
        assert "PANDAS=False" in result.stdout, result.stdout + result.stderr

    @requires_openpyxl
    def test_converting_to_xlsx_never_imports_pandas(self, tmp_path):
        """产品路径上写出一张 xlsx 之后，进程里仍然不该有 pandas

        这是本轮全部收益的**唯一直接判据**：省下的那 900 毫秒不是靠读代码看出来
        省到的，而是靠「进程里根本没有它」证明的。同时钉住退出码 0 与 stderr 无声
        —— write_only 工作簿若在收尾时漏掉临时文件，Windows 会在 atexit 打一帧
        traceback 出来，那会被 CLI 的 stderr 原样带给用户。
        """
        source = tmp_path / "corpus.json"
        source.write_text(
            '[{"instruction": "q1", "output": "a1"}, '
            '{"instruction": "q2", "output": "a2"}]',
            encoding="utf-8",
        )
        out = tmp_path / "guard.xlsx"
        code = (
            "import sys; from augmentor.converter import DatasetConverter; "
            "DatasetConverter().convert_file(r'%s', r'%s', target_format='excel'); "
            "print('PANDAS=' + str('pandas' in sys.modules)); "
            "print('OPENPYXL=' + str('openpyxl' in sys.modules))"
            % (source, out)
        )
        result = subprocess.run(
            [sys.executable, "-c", code], capture_output=True, text=True,
            encoding="utf-8", errors="replace", cwd=str(REPO),
        )
        assert result.returncode == 0, result.stderr
        assert "PANDAS=False" in result.stdout, \
            f"写边又把 pandas 拉进进程了（本轮省下的 906.5 ms 全在这一格）: {result.stdout}"
        assert "OPENPYXL=True" in result.stdout
        assert "Traceback" not in result.stderr, result.stderr
        assert zipfile.is_zipfile(out)

    def test_the_pandas_route_is_gone_from_the_write_edge(self):
        """源码级守卫：写边模块里不许出现 pandas，而转接口所在模块仍然需要它

        子进程守卫查的是「这台机器上跑出来没进进程」，这一条查的是「有没有人换条路
        又把它请回来」—— 比如给 `excel_write` 顶部补一句 `import pandas as pd`。
        """
        tree = ast.parse((REPO / "augmentor/excel_write.py").read_text(encoding="utf-8"))
        imported = {
            node.module or "" for node in ast.walk(tree)
            if isinstance(node, ast.ImportFrom)
        } | {
            alias.name for node in ast.walk(tree) if isinstance(node, ast.Import)
            for alias in node.names
        }
        assert not any(name.split(".")[0] == "pandas" for name in imported), \
            f"写边又 import 了 pandas: {sorted(imported)}"

        shim = (REPO / "augmentor/csv_excel_import.py").read_text(encoding="utf-8")
        assert "import pandas" in shim, "读边仍然需要 pandas，转接口不该把它删了"


# ==================== 2. 单元格等价 ====================

@requires_both
class TestCellsMatchPandas:
    @pytest.mark.parametrize("value", [
        None, float("nan"), float("inf"), float("-inf"), True, False,
        0, 1, -3, 2.5, "", "普通文本", "带,逗号 和\t制表",
        ["a", "b"], {"k": "v"}, (1, 2), {1, 2}, b"xy",
        "=SUM(A1:A2)", "x" * 32768,
    ], ids=[
        "None", "NaN", "inf", "-inf", "True", "False", "int0", "int1", "neg",
        "float", "empty-str", "cjk", "punct", "list", "dict", "tuple", "set",
        "bytes", "formula", "overlong",
    ])
    def test_single_value_lands_the_same(self, tmp_path, value):
        new, old, written = both_edges(tmp_path, [{"a": 1, "b": value}])
        assert written == 1
        assert new == old, f"值 {value!r} 在两条路上落成了不同的格子"

    def test_ragged_rows_keep_the_same_header_and_holes(self, tmp_path):
        """异构记录：列序与「谁缺哪格」必须逐格一致（这是 csv 写边的同一把尺子）"""
        items = [
            {"instruction": "q1", "output": "a1", "category": "c1"},
            {"output": "a2"},
            {"instruction": "q3", "note": "n3", "category": "c3"},
        ]
        new, old, written = both_edges(tmp_path, items)
        assert written == 3
        assert new[0] == old[0] == ["instruction", "output", "category", "note"]
        assert new == old

    def test_sheet_name_is_honored(self, tmp_path):
        out = tmp_path / "s.xlsx"
        excel_write.export_table_to_excel([{"a": 1}], str(out), sheet_name="清单")
        from openpyxl import load_workbook

        book = load_workbook(out)
        try:
            assert book.sheetnames == ["清单"]
        finally:
            book.close()

    def test_the_public_shim_produces_the_same_table(self, tmp_path):
        """`csv_excel_import.export_to_excel` 与直连新写边必须给同一张表

        转接口若悄悄改了列序或表头，老调用方（含本仓测试）就会拿到一份不同的东西。
        """
        via_shim, via_direct = tmp_path / "shim.xlsx", tmp_path / "direct.xlsx"
        rows = [{"a": 1, "b": "x"}, {"a": 2, "c": None}]
        assert export_to_excel(rows, str(via_shim)) == \
            excel_write.export_table_to_excel(rows, str(via_direct)) == 2
        assert cells_of(via_shim) == cells_of(via_direct)

    def test_illegal_control_character_raises_the_same_as_pandas(self, tmp_path):
        """控制字符：两条路都由 openpyxl 拒收，报错类型必须一致（不是本轮改的契约）"""
        from openpyxl.utils.exceptions import IllegalCharacterError

        items = [{"a": "bad\x01char"}]
        with pytest.raises(IllegalCharacterError):
            excel_write.export_table_to_excel(items, str(tmp_path / "new.xlsx"))
        with pytest.raises(IllegalCharacterError):
            pandas_edge(items, str(tmp_path / "old.xlsx"))


# ==================== 3. 列序、报错与空表 ====================

class TestContentJudgedBeforeTheDependencyGate:
    """内容判据两侧解释器都要真跑：它们不许被「本机没装 openpyxl」挡在后面"""

    def test_missing_column_names_the_columns(self, tmp_path):
        out = tmp_path / "o.xlsx"
        with pytest.raises(DataValidationError, match=r"以下列不存在: \['nope'\]"):
            excel_write.export_table_to_excel([{"a": 1}], str(out), columns=["nope"])
        assert not out.exists(), "报错前落了半成品"

    @pytest.mark.parametrize(("items", "pos"), [
        ([{"a": 1}, 5], 2),
        ([{"a": 1}, ["b"]], 2),
        ([None], 1),
    ])
    def test_non_object_record_is_named_by_index(self, tmp_path, items, pos):
        """标量/列表记录：报错点名第几条（pandas 路给的是 `TypeError` 或静默列名 `0`）

        这是本轮**刻意收紧**的一格，不属于等价面：判据要在任何写盘动作之前，所以
        连输出文件都不该出现。
        """
        out = tmp_path / "o.xlsx"
        with pytest.raises(DataValidationError, match=f"第 {pos} 条记录必须是 JSON 对象"):
            excel_write.export_table_to_excel(items, str(out))
        assert not out.exists(), "报错前落了半成品"

    def test_content_error_outranks_the_missing_dependency(self, tmp_path, monkeypatch):
        """顺序本身是契约：没装 openpyxl 也不许把数据错报成「去装依赖」

        两侧实测过一轮：依赖门在前时，上面那几格内容判据在装了 openpyxl 的一侧全绿、
        在没装的一侧全红 —— 也就是它们恰好在最需要它们的环境里从不被验证。
        """
        monkeypatch.setattr(excel_write, "HAS_OPENPYXL", False)
        out = tmp_path / "o.xlsx"
        with pytest.raises(DataValidationError, match="第 2 条记录必须是 JSON 对象"):
            excel_write.export_table_to_excel([{"a": 1}, "标量"], str(out))
        assert not out.exists()

    def test_valid_data_still_degrades_when_openpyxl_is_absent(self, tmp_path, monkeypatch):
        """内容过了门才轮到依赖门：这时报的才是「装 openpyxl」"""
        monkeypatch.setattr(excel_write, "HAS_OPENPYXL", False)
        out = tmp_path / "o.xlsx"
        with pytest.raises(ImportError, match="pip install openpyxl"):
            excel_write.export_table_to_excel([{"a": 1}], str(out))
        assert not out.exists()

    def test_excel_columns_is_a_pure_function_of_items(self):
        """`excel_columns` 不许写盘：列序判定要能在没有一份文件的情况下被问"""
        assert excel_write.excel_columns([{"b": 1}, {"a": 2, "b": 3}], None) == ["b", "a"]
        assert excel_write.excel_columns([{"b": 1}], ["b"]) == ["b"]


@requires_openpyxl
class TestSheetsAndLoudness:
    """下面几族要**真写出文件**才能判，所以只在装了 openpyxl 的一侧跑"""

    def test_no_columns_writes_no_header_and_no_rows(self, tmp_path):
        """一条键都没有 ⇒ 整张表不写（与 pandas 路同口径，L85 golden `c26/c27`）

        返回的仍是**记录条数**：这一格刻意保持与老签名一致，哪怕表里一行都没有。
        """
        out = tmp_path / "empty.xlsx"
        assert excel_write.export_table_to_excel([{}, {}], str(out)) == 2
        assert cells_of(out) == []

    def test_empty_input_writes_an_empty_sheet(self, tmp_path):
        out = tmp_path / "none.xlsx"
        assert excel_write.export_table_to_excel([], str(out)) == 0
        assert cells_of(out) == []

    def test_parent_directory_is_created(self, tmp_path):
        out = tmp_path / "deep" / "nested" / "o.xlsx"
        excel_write.export_table_to_excel([{"a": 1}], str(out))
        assert out.is_file()

    def test_overlong_cell_warns_once(self, tmp_path, caplog):
        """超长串要出声：两条路读回都是 32767（等价），但响度也得对上

        pandas 路由 openpyxl 的非 write_only 分支发 `UserWarning`；新写边那条分支
        **静默**截断，所以本轮在 `export_table_to_excel` 里补了一帧 logger.warning。
        这一格钉的就是那帧补回来的声音 —— 少了它，「删掉一段用户数据」这件事在两侧
        日志里都看不见。
        """
        with caplog.at_level("WARNING", logger="augmentor.excel_write"):
            excel_write.export_table_to_excel(
                [{"a": "x" * 32768}, {"b": "y" * 40000}], str(tmp_path / "long.xlsx")
            )
        assert "超过 32767 字符" in caplog.text, caplog.text
        assert "2 个单元格" in caplog.text, caplog.text

    def test_normal_cells_stay_quiet(self, tmp_path, caplog):
        """不超长就不许报警：否则 warning 通道会被正常导出刷满"""
        with caplog.at_level("WARNING", logger="augmentor.excel_write"):
            excel_write.export_table_to_excel(
                [{"a": "x" * 32767}, {"b": None}], str(tmp_path / "ok.xlsx")
            )
        assert "超过 32767 字符" not in caplog.text, caplog.text


# ==================== 4. 依赖探测分支本身 ====================

class TestTheDependencyProbeRuns:
    """`HAS_OPENPYXL` 的**产地**（那两行 `except ImportError`）两侧都必须真跑

    为什么不能只靠 monkeypatch 标志位：那种用例测的是「代码读这个位」，而这个位怎么
    被设成 `False` 的那条分支在装了 openpyxl 的一侧（记账解释器）一行都没执行过 ——
    本轮全量覆盖差集量到的 `excel_write.py` 缺数 2 句正是它。同族的 pandas 版本早就
    用 reload 收口了（`test_micro_branches_l61.py`），这里对 openpyxl 做同一件事。
    """

    def test_reloading_without_openpyxl_flips_the_flag_and_restores(self, tmp_path):
        import importlib

        before = excel_write.HAS_OPENPYXL
        real = sys.modules.get("openpyxl")
        out = tmp_path / "never.xlsx"
        try:
            sys.modules["openpyxl"] = None      # 让 `from openpyxl import ...` 抛 ImportError
            degraded = importlib.reload(excel_write)
            assert degraded.HAS_OPENPYXL is False
            with pytest.raises(ImportError, match="pip install openpyxl"):
                degraded.export_table_to_excel([{"a": 1}], str(out))
            assert not out.exists(), "依赖门必须在落盘之前"
        finally:
            if real is None:
                sys.modules.pop("openpyxl", None)
            else:
                sys.modules["openpyxl"] = real
            importlib.reload(excel_write)
        # 复原必须是同一个模块对象（reload 是原地改），否则后面的用例拿到的是影子
        assert excel_write.HAS_OPENPYXL == before
        assert sys.modules["augmentor.excel_write"] is excel_write
