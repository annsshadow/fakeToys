# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L297 / A391②：CSV / TSV 的表头也不许长成公式 —— 危险列名在写出前**拒收**

A389（L287）把**单元格值**按各格式原生手段挡住了，列名那一格当时留在账上：给表头加
前导单引号会改变读回侧用来配行的键，一处转义要两处同步。本轮按用户 2026-11-09 对
A391 的拍板② 收口：**不改写、直接拒收**。命中的判据与值面共用 `SHEET_FORMULA_LEADERS`
（`is_sheet_formula_text`），处置相反——`DataValidationError` 点名危险列名，并建议改
列名或改用 xlsx / jsonl。

现量（L297 前的仓外探针）：`export_enhanced` 与 `converter` 的 CSV 表头把
`=cmd|' /C calc'!A0` **原样成文**；`Exporter` 原生 CSV 先归一成
`instruction/input/output`，不受任意列名影响；xlsx 表头是 `t="inlineStr"`、零 `<f>`，
本来就有类型位。给表头加 `'` 前缀后 `csv.DictReader` 读回的键变成 `'=cmd...`，正是
拍板② 拒绝静默改写的理由。

尺子面：

1. 判据本体：六档前导逐档拒收、每个危险列名都进文案、普通列名原序原值放行。
2. 五条写边真跑：converter csv/tsv、export_enhanced csv/tsv、pandas `to_csv` 各自
   抛 `DataValidationError`，且产物里一个危险表头字节都没落。
3. 负面对照：xlsx 仍接受同一列名并强制文本类型位；json / jsonl 一字未动。
4. 接线普查：产品树每一处 `csv.DictWriter(fieldnames=...)` 都必须过权威；产品树唯一
   一处 `DataFrame.to_csv` 必须在 `mkdir` / 写盘之前调用权威。
5. 牙齿：把权威换成恒等 ⇒ 危险表头当场回到磁盘。

跑法：`python -m pytest tests/unit/test_sheet_header_injection_l297.py -q --no-cov`
"""

from __future__ import annotations

import ast
import csv
import json
import sys
import zipfile
from pathlib import Path
from typing import Dict, List, Tuple

import pytest

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor import excel_write, validation  # noqa: E402
from augmentor.converter import convert_file  # noqa: E402
from augmentor.csv_excel_import import export_to_csv  # noqa: E402
from augmentor.exceptions import DataValidationError  # noqa: E402
from augmentor.export import Exporter  # noqa: E402
from augmentor.export_enhanced import EnhancedExporter, ExportFormat, ExportOptions  # noqa: E402
from augmentor.validation import (  # noqa: E402
    SHEET_FORMULA_LEADERS,
    require_sheet_safe_fieldnames,
)

#: 探针与 A389 守卫同形的危险列名
FORGED = "=cmd|' /C calc'!A0"

PRODUCT_ROOTS = ("augmentor", "api")


def _header(path: Path, delimiter: str = ",") -> List[str]:
    """读回表头那一行"""
    with open(path, encoding="utf-8", newline="") as f:
        return next(csv.reader(f, delimiter=delimiter))


def _records(key: str = FORGED) -> List[Dict[str, str]]:
    return [{key: "ok", "instruction": "普通指令"}]


def _converted(records: List[Dict], out: Path, fmt: str) -> None:
    src = out.with_suffix(".json")
    src.write_text(json.dumps(records, ensure_ascii=False), encoding="utf-8")
    convert_file(str(src), str(out), target_format=fmt)


def _enhanced(records: List[Dict], out: Path, fmt: ExportFormat) -> None:
    EnhancedExporter().export(records, str(out), ExportOptions(format=fmt))


def _sheet_xml(path: Path) -> str:
    """xlsx 容器里首个工作表的 XML（判 `<f>` 只能看字节，读回侧会替它说谎）"""
    with zipfile.ZipFile(path) as archive:
        names = sorted(n for n in archive.namelist() if n.startswith("xl/worksheets/sheet"))
        return archive.read(names[0]).decode("utf-8")


def _product_files() -> List[Path]:
    return sorted(p for root in PRODUCT_ROOTS for p in (REPO / root).rglob("*.py"))


def _dictwriter_sites(tree: ast.AST, label: str) -> List[Tuple[str, int, bool]]:
    """每一处 `csv.DictWriter(...)`：`(文件, 行号, fieldnames 是否过权威)`"""
    hits = []
    for node in ast.walk(tree):
        if not (isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)):
            continue
        if node.func.attr != "DictWriter":
            continue
        wrapped = False
        for keyword in node.keywords:
            if keyword.arg != "fieldnames":
                continue
            value = keyword.value
            if not isinstance(value, ast.Call):
                continue
            name = getattr(value.func, "id", None) or getattr(value.func, "attr", None)
            wrapped = name == "require_sheet_safe_fieldnames"
        hits.append((label, node.lineno, wrapped))
    return hits


def _to_csv_sites(tree: ast.AST, label: str) -> List[Tuple[str, str, int, List[int]]]:
    """每一处 `DataFrame.to_csv` 所在函数：`(文件, 函数名, 行号, 权威调用行号列表)`"""
    hits = []
    for func in ast.walk(tree):
        if not isinstance(func, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        calls = [n for n in ast.walk(func) if isinstance(n, ast.Call)]
        csv_calls = [
            n for n in calls
            if isinstance(n.func, ast.Attribute) and n.func.attr == "to_csv"
        ]
        if not csv_calls:
            continue
        guards = [
            n.lineno for n in calls
            if (getattr(n.func, "id", None) or getattr(n.func, "attr", None))
            == "require_sheet_safe_fieldnames"
        ]
        for call in csv_calls:
            hits.append((label, func.name, call.lineno, guards))
    return hits


class TestTheAuthorityItself:
    @pytest.mark.parametrize("leader", SHEET_FORMULA_LEADERS, ids=repr)
    def test_each_leader_is_refused(self, leader):
        """六档前导逐档拒收：判据不是只认 `=`"""
        name = leader + "cmd"
        with pytest.raises(DataValidationError) as excinfo:
            require_sheet_safe_fieldnames(["ok", name], "csv")
        message = str(excinfo.value)
        assert repr(name) in message, message
        assert "csv" in message, message

    def test_every_forged_name_is_named(self):
        """一条报错里把所有危险列名都点出来，而不是只报第一条"""
        with pytest.raises(DataValidationError) as excinfo:
            require_sheet_safe_fieldnames(["=a", "ok", "+b", "-c"], "tsv")
        message = str(excinfo.value)
        for name in ("=a", "+b", "-c"):
            assert repr(name) in message, message
        assert "'ok'" not in message, message

    def test_normal_names_pass_through_in_order(self):
        """普通列名原序原值放行；返回值可直接交给 `csv.DictWriter`"""
        names = ["instruction", "input", "output"]
        assert require_sheet_safe_fieldnames(iter(names), "csv") == names

    def test_the_error_stays_a_value_error_for_the_api(self):
        """`DataValidationError` 是 `ValueError` 子类 ⇒ `to_http_error` 的 ValueError 支回 400"""
        with pytest.raises(DataValidationError) as excinfo:
            require_sheet_safe_fieldnames([FORGED], "csv")
        assert isinstance(excinfo.value, ValueError)
        assert excinfo.value.__class__.__name__ == "DataValidationError"

    def test_the_api_answers_400_with_the_forged_names(self):
        """对外契约：导出端点拿到这条异常时回 400，文案里点名危险列名"""
        from api.deps import to_http_error

        with pytest.raises(DataValidationError) as excinfo:
            require_sheet_safe_fieldnames([FORGED], "csv")
        error = to_http_error(excinfo.value)
        assert error.status_code == 400
        assert FORGED in error.detail


class TestEveryCsvWriteEdgeRefuses:
    @pytest.mark.parametrize("fmt", ["csv", "tsv"])
    def test_converter_refuses_before_the_header(self, tmp_path, fmt):
        out = tmp_path / f"conv.{fmt}"
        with pytest.raises(DataValidationError) as excinfo:
            _converted(_records(), out, fmt)
        assert FORGED in str(excinfo.value)
        # 文件可能已被 open() 建出来，但危险表头一个字节都不许进
        if out.exists():
            assert FORGED not in out.read_text(encoding="utf-8", errors="replace")

    @pytest.mark.parametrize(
        "fmt,shape",
        [(ExportFormat.CSV, "csv"), (ExportFormat.TSV, "tsv")],
        ids=["csv", "tsv"],
    )
    def test_enhanced_exporter_refuses(self, tmp_path, fmt, shape):
        out = tmp_path / f"enhanced.{shape}"
        with pytest.raises(DataValidationError) as excinfo:
            _enhanced(_records(), out, fmt)
        assert FORGED in str(excinfo.value)
        if out.exists():
            assert FORGED not in out.read_text(encoding="utf-8", errors="replace")

    @pytest.mark.parametrize("columns", [None, [FORGED]], ids=["all-columns", "named-column"])
    def test_pandas_edge_refuses(self, tmp_path, columns):
        out = tmp_path / "pandas.csv"
        with pytest.raises(DataValidationError) as excinfo:
            export_to_csv(_records(), out, columns=columns)
        assert FORGED in str(excinfo.value)
        # pandas 那条边判在 mkdir 之前：连空文件都不留
        assert not out.exists(), out

    def test_native_exporter_normalises_before_the_header(self, tmp_path):
        """`Exporter` 原生 CSV 的表头是固定三键：任意列名到不了表头，独立于拒收面"""
        out = tmp_path / "native.csv"
        Exporter().export(_records(), str(out), "csv")
        assert _header(out) == ["instruction", "input", "output"]
        assert FORGED not in _header(out)


class TestTheScopeStaysNarrow:
    @pytest.mark.parametrize("fmt", ["json", "jsonl"])
    def test_containers_without_a_formula_interpreter_are_untouched(self, tmp_path, fmt):
        """json / jsonl 没有公式解释器：危险列名照常保留，判据不许扩到容器之外"""
        out = tmp_path / f"face.{fmt}"
        _converted(_records(), out, fmt)
        assert FORGED in out.read_text(encoding="utf-8")

    @pytest.mark.skipif(not excel_write.HAS_OPENPYXL, reason="本机没有 openpyxl")
    def test_xlsx_keeps_the_column_as_text(self, tmp_path):
        """xlsx 有真正的类型位：同一列名照收，但格子钉成文本、零 `<f>`"""
        out = tmp_path / "kept.xlsx"
        excel_write.export_table_to_excel(_records(), str(out), columns=[FORGED])
        assert "<f>" not in _sheet_xml(out)

        from openpyxl import load_workbook

        book = load_workbook(out)
        try:
            head = book.active["A1"]
            assert (head.value, head.data_type) == (FORGED, "s")
        finally:
            book.close()


class TestTheWiringIsClosed:
    def test_every_dictwriter_fieldnames_passes_the_authority(self):
        """产品树每一处 CSV/TSV 写边都过权威；摘掉任一处，这里点名"""
        hits = [
            hit
            for path in _product_files()
            for hit in _dictwriter_sites(ast.parse(path.read_text(encoding="utf-8")),
                                         path.relative_to(REPO).as_posix())
        ]
        assert hits, "扫描器一支都没读到 ⇒ 这是数不到，不是干净"
        files = sorted({where.split(":")[0] if ":" in where else where for where, _, _ in hits})
        assert files == [
            "augmentor/converter.py",
            "augmentor/export.py",
            "augmentor/export_enhanced.py",
        ], hits
        assert len(hits) == 5, hits
        for where, line, wrapped in hits:
            assert wrapped, f"{where}:{line} 的 fieldnames 没过权威"

    def test_the_pandas_edge_is_guarded_before_the_write(self):
        """产品树唯一一处 `to_csv`：权威必须在 `to_csv` 之前调用"""
        hits = [
            hit
            for path in _product_files()
            for hit in _to_csv_sites(ast.parse(path.read_text(encoding="utf-8")),
                                     path.relative_to(REPO).as_posix())
        ]
        assert len(hits) == 1, hits
        where, func, line, guards = hits[0]
        assert (where, func) == ("augmentor/csv_excel_import.py", "export_to_csv"), hits
        assert guards, f"{where}:{line} 的 to_csv 没过权威"
        assert min(guards) < line, f"{where}:{line} 的权威调用在写盘之后：{guards}"

    def test_the_dictwriter_scanner_has_teeth(self):
        forged = "def f(f, rows):\n    w = csv.DictWriter(f, fieldnames=rows[0].keys())\n"
        assert _dictwriter_sites(ast.parse(forged), "synthetic.py") == [
            ("synthetic.py", 2, False)
        ]
        guarded = (
            "def f(f, rows):\n"
            "    w = csv.DictWriter(f, fieldnames=require_sheet_safe_fieldnames(rows[0].keys(), 'csv'))\n"
        )
        assert _dictwriter_sites(ast.parse(guarded), "synthetic.py") == [
            ("synthetic.py", 2, True)
        ]

    def test_the_pandas_scanner_has_teeth(self):
        forged = "def f(df, out):\n    df.to_csv(out, index=False)\n"
        assert _to_csv_sites(ast.parse(forged), "synthetic.py") == [
            ("synthetic.py", "f", 2, [])
        ]
        guarded = (
            "def f(df, out):\n"
            "    require_sheet_safe_fieldnames(df.columns, 'csv')\n"
            "    df.to_csv(out, index=False)\n"
        )
        assert _to_csv_sites(ast.parse(guarded), "synthetic.py") == [
            ("synthetic.py", "f", 3, [2])
        ]


class TestTheRulerHasTeeth:
    def test_identity_helper_lets_the_forged_header_reach_disk(self, tmp_path, monkeypatch):
        """把权威换成恒等 ⇒ 危险表头当场回到磁盘，证明写边那几次调用是承重的"""
        monkeypatch.setattr(validation, "require_sheet_safe_fieldnames",
                            lambda names, container: list(names))
        out = tmp_path / "teeth.csv"
        _enhanced(_records(), out, ExportFormat.CSV)
        assert _header(out) == [FORGED, "instruction"], _header(out)
