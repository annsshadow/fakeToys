# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CSV/Excel 导出功能测试（csv_excel_import 模块的导出侧）

覆盖 export_to_csv / export_to_excel 的正常导出、列选择、
缺列报错、目录自动创建与 pandas 缺失降级。
"""

import json

import pytest

augmentor_ce = pytest.importorskip("pandas")

from augmentor.csv_excel_import import export_to_csv, export_to_excel

ITEMS = [
    {"instruction": "如何申请？", "input": "", "output": "登录官网"},
    {"instruction": "多少钱？", "input": "", "output": "按月计"},
]


class TestExportCsv:
    def test_roundtrip(self, tmp_path):
        out = tmp_path / "out.csv"
        written = export_to_csv(ITEMS, out)
        assert written == 2
        assert out.exists()
        import pandas as pd

        df = pd.read_csv(out, encoding="utf-8-sig")
        assert list(df["instruction"]) == ["如何申请？", "多少钱？"]
        assert "output" in df.columns

    def test_column_selection(self, tmp_path):
        out = tmp_path / "cols.csv"
        export_to_csv(ITEMS, out, columns=["instruction", "output"])
        import pandas as pd

        df = pd.read_csv(out, encoding="utf-8-sig")
        assert list(df.columns) == ["instruction", "output"]

    def test_missing_column_raises(self, tmp_path):
        out = tmp_path / "bad.csv"
        with pytest.raises(ValueError, match="不存在"):
            export_to_csv(ITEMS, out, columns=["no_such_col"])

    def test_creates_parent_dir(self, tmp_path):
        out = tmp_path / "deep" / "dir" / "x.csv"
        export_to_csv(ITEMS, out)
        assert out.exists()

    def test_empty_items(self, tmp_path):
        out = tmp_path / "empty.csv"
        written = export_to_csv([], out)
        assert written == 0


class TestExportExcel:
    def test_roundtrip(self, tmp_path):
        out = tmp_path / "out.xlsx"
        written = export_to_excel(ITEMS, out, sheet_name="sheet1")
        assert written == 2
        import pandas as pd

        df = pd.read_excel(out, sheet_name="sheet1")
        assert len(df) == 2
        assert "instruction" in df.columns

    def test_column_selection(self, tmp_path):
        out = tmp_path / "cols.xlsx"
        export_to_excel(ITEMS, out, columns=["instruction"])
        import pandas as pd

        df = pd.read_excel(out)
        assert list(df.columns) == ["instruction"]


class TestPandasMissing:
    def test_export_csv_no_pandas_raises(self, tmp_path, monkeypatch):
        import augmentor.csv_excel_import as cei

        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        with pytest.raises(ImportError, match="pandas"):
            export_to_csv(ITEMS, tmp_path / "x.csv")

    def test_export_excel_no_pandas_raises(self, tmp_path, monkeypatch):
        import augmentor.csv_excel_import as cei

        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        with pytest.raises(ImportError, match="pandas"):
            export_to_excel(ITEMS, tmp_path / "x.xlsx")
