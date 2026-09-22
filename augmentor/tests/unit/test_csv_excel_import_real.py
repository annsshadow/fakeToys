# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CSV/Excel 导入真实数据驱动测试

在 pandas/openpyxl 可用时验证实际导入行为；
另通过打桩 HAS_PANDAS=False 覆盖 ImportError 降级分支。
"""

import pytest

augmentor_csv = pytest.importorskip("pandas")

from augmentor import csv_excel_import as cei
from augmentor.csv_excel_import import import_dataset, import_from_csv, import_from_excel

CSV_ROWS = [
    {"instruction": "如何申请租房？", "output": "登录官网申请"},
    {"instruction": "租房多少钱？", "output": "按房型定价"},
    {"instruction": "如何退租押金？", "output": "满一年后退还"},
]


@pytest.fixture
def csv_file(tmp_path):
    import pandas as pd

    path = tmp_path / "sample.csv"
    pd.DataFrame(CSV_ROWS).to_csv(path, index=False, encoding="utf-8")
    return str(path)


@pytest.fixture
def excel_file(tmp_path):
    import pandas as pd

    path = tmp_path / "sample.xlsx"
    pd.DataFrame(CSV_ROWS).to_excel(path, index=False)
    return str(path)


class TestImportFromCsv:
    def test_import_maps_columns(self, csv_file):
        items = import_from_csv(csv_file)
        assert len(items) == 3
        assert items[0]["instruction"] == "如何申请租房？"
        assert items[0]["output"] == "登录官网申请"
        assert items[0]["input"] == ""

    def test_custom_columns(self, tmp_path):
        import pandas as pd

        path = tmp_path / "custom.csv"
        pd.DataFrame([{"q": "问题", "a": "回答"}]).to_csv(path, index=False, encoding="utf-8")
        items = import_from_csv(str(path), text_column="q", output_column="a")
        assert items[0]["instruction"] == "问题"
        assert items[0]["output"] == "回答"

    def test_missing_column_defaults_empty(self, tmp_path):
        import pandas as pd

        path = tmp_path / "sparse.csv"
        pd.DataFrame([{"instruction": "只有问题"}]).to_csv(path, index=False, encoding="utf-8")
        items = import_from_csv(str(path))
        # 缺 output 列时应落为 "nan" 或空（pandas 缺列返回 NaN）
        assert items[0]["instruction"] == "只有问题"

    def test_no_pandas_raises_import_error(self, csv_file, monkeypatch):
        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        with pytest.raises(ImportError, match="pandas"):
            import_from_csv(csv_file)


class TestImportFromExcel:
    def test_import_maps_columns(self, excel_file):
        items = import_from_excel(excel_file)
        assert len(items) == 3
        assert items[0]["instruction"] == "如何申请租房？"
        assert items[0]["output"] == "登录官网申请"

    def test_named_sheet(self, tmp_path):
        import pandas as pd

        path = tmp_path / "sheets.xlsx"
        with pd.ExcelWriter(path) as writer:
            pd.DataFrame(CSV_ROWS).to_excel(writer, sheet_name="first", index=False)
            pd.DataFrame([{"instruction": "另一表", "output": "答案"}]).to_excel(
                writer, sheet_name="second", index=False
            )
        items = import_from_excel(str(path), sheet_name="second")
        assert len(items) == 1
        assert items[0]["instruction"] == "另一表"

    def test_no_pandas_raises_import_error(self, excel_file, monkeypatch):
        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        with pytest.raises(ImportError, match="pandas"):
            import_from_excel(excel_file)


class TestImportDatasetAutoDetect:
    def test_auto_detect_csv(self, csv_file):
        items = import_dataset(csv_file)
        assert len(items) == 3

    def test_auto_detect_excel(self, excel_file):
        items = import_dataset(excel_file)
        assert len(items) == 3

    def test_explicit_format_overrides_suffix(self, csv_file, tmp_path):
        # .dat 扩展名无法自动识别，但显式 format="csv" 仍可导入
        renamed = tmp_path / "renamed.dat"
        import shutil
        shutil.copy(csv_file, renamed)
        items = import_dataset(str(renamed), format="csv")
        assert len(items) == 3

    def test_unknown_suffix_requires_format(self, tmp_path):
        bogus = tmp_path / "data.dat"
        bogus.write_text("x", encoding="utf-8")
        with pytest.raises(ValueError, match="无法识别"):
            import_dataset(str(bogus))

    def test_unknown_format_rejected(self, csv_file):
        with pytest.raises(ValueError, match="不支持"):
            import_dataset(csv_file, format="parquet")
