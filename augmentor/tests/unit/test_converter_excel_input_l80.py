# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L80（B2 读边）：`convert_file` 的**输入**格式判定与 Excel 摄取

改动前的三条实测（`Temp/l80q/probe_b2.py`、`Temp/l80q/smoke_read.py`）：

* `convert --input x.xlsx` 走的是 `_infer_format`（那张表只认 json/jsonl/csv/tsv），
  认不出就回落 `json`，于是二进制 xlsx 崩在解码器里 ——
  `UnicodeDecodeError: 'utf-8' codec can't decode byte 0xc7 in position 15`。报错停在
  「第 15 个字节不是 UTF-8」，与真实原因「没认出格式」无关。
* 包内 `csv_excel_import.import_from_excel` 在 `convert_file` 上**可达性为零**：没有任何
  入口能走到它，`--input-format` 的 choices 里也没有 excel。
* 因此「Excel 摄取」这条能力在 SDK 公开面（`augmentors`）上等于不存在。

本文件钉四件事：读边认得出 Excel（三种拼法、可覆盖后缀）、认不出**必须报错**且文案
可行动、缺依赖时的报错停在「装依赖」而不是 traceback 且**不落半成品**，以及同轮探针
顺带查出的 **A134**（`target_format="jsonl"` 双重序列化，见
`TestJsonlSerializedOnce` 的 docstring —— 它正是 Excel 导出 jsonl 时会踩到的那条边）。

`.xlsx` 作为**输出**仍不是 Excel（写边没有 Excel 输出器，A133），这条边界也钉在这里，
免得下一轮把它当成已完成。
"""

import json
from pathlib import Path

import pytest

from augmentor import csv_excel_import as cei
from augmentor.converter import (
    EXCEL_FORMAT,
    EXCEL_SOURCE_NAMES,
    INPUT_EXTENSION_FORMATS,
    INPUT_FORMAT_CHOICES,
    DataFormat,
    DatasetConverter,
    excel_cell,
    get_supported_formats,
)
from augmentor.exceptions import DataLoadError, UnsupportedFormatError


def _excel_io_available() -> bool:
    """pandas 与 openpyxl 是否都在（写一份真 xlsx 需要两者）"""
    try:
        import openpyxl  # noqa: F401
        import pandas  # noqa: F401
    except ImportError:
        return False
    return True


EXCEL_IO = _excel_io_available()
requires_excel = pytest.mark.skipif(
    not EXCEL_IO, reason="造/读真 xlsx 需要 pandas 与 openpyxl（aug 侧解释器无 pandas）"
)

ROWS = [
    {"instruction": "如何申请租房？", "output": "登录官网申请", "category": "租房"},
    {"instruction": "租房多少钱？", "output": "按房型定价", "category": "价格"},
    {"instruction": "如何退租押金？", "output": "满一年后退还", "category": "退租"},
]


@pytest.fixture
def converter():
    return DatasetConverter()


@pytest.fixture
def xlsx_file(tmp_path):
    """一张三列三行的真表，外加一列空值与一列日期（缺失值的 JSON 安全性是本轮判据）"""
    import pandas as pd

    frame = pd.DataFrame(
        {
            "instruction": [r["instruction"] for r in ROWS] + [None],
            "output": [r["output"] for r in ROWS] + ["补齐值"],
            "category": [r["category"] for r in ROWS] + [None],
            "score": [1.5, 2.0, float("nan"), 4.0],
            "due": pd.to_datetime(["2026-01-02", "2026-02-03", pd.NaT, "2026-04-05"]),
        }
    )
    path = tmp_path / "corpus.xlsx"
    frame.to_excel(path, index=False)
    return path


# ==================== 读边判定：认得出 ====================

class TestInputFormatResolution:
    """`_resolve_input_format`：认得出的一律认，认不出的一律报"""

    @pytest.mark.parametrize(
        ("suffix", "expected"),
        [(".json", "json"), (".JSON", "json"), (".jsonl", "jsonl"),
         (".csv", "csv"), (".tsv", "tsv"), (".xlsx", EXCEL_FORMAT),
         (".XLSX", EXCEL_FORMAT), (".xls", EXCEL_FORMAT)],
    )
    def test_suffix_table(self, converter, suffix, expected):
        """扩展名表逐条钉住，且大小写不敏感"""
        assert converter._resolve_input_format(Path(f"dir/data{suffix}")) == expected

    @pytest.mark.parametrize(("fmt", "payload", "expected"), [
        ("json", '[{"instruction": "问", "output": "答"}]',
         [{"instruction": "问", "output": "答"}]),
        ("jsonl", '{"instruction": "问", "output": "答"}\n',
         [{"instruction": "问", "output": "答"}]),
        ("csv", "instruction,output\n问,答\n",
         [{"instruction": "问", "output": "答"}]),
        ("tsv", "instruction\toutput\n问\t答\n",
         [{"instruction": "问", "output": "答"}]),
    ])
    def test_every_declared_text_format_really_reads(self, converter, tmp_path,
                                                     fmt, payload, expected):
        """表里每个**文本**格式都得真跑得通读边

        只比字符串表的话，往表里加一条 `.parquet -> parquet` 也能过 —— 而 `_read_file`
        没有那个分支，它会落到 schema 嗅探里把 parquet 当文本读。所以这里逐条落一份
        最小文件、跑一次 `_read_file`。`excel` 那一格由 `TestExcelReadEdge` 覆盖
        （它在无 pandas 的环境里整体跳过，不能混进来）。
        """
        assert INPUT_EXTENSION_FORMATS[f".{fmt}"] == fmt
        path = tmp_path / f"corpus.{fmt}"
        path.write_text(payload, encoding="utf-8")
        assert converter._read_file(path, fmt) == expected


class TestUnknownExtensionRejected:
    """认不出扩展名 ⇒ 报错，而不是静默回落 json"""

    def test_raises_unsupported_format(self, converter, tmp_path):
        bogus = tmp_path / "corpus.bin"
        bogus.write_bytes(b"\x00\x01binary")
        out = tmp_path / "out.json"

        with pytest.raises(UnsupportedFormatError):
            converter.convert_file(bogus, out)
        assert not out.exists(), "报错前就写出了半成品"

    def test_message_names_the_file_and_the_way_out(self, converter, tmp_path):
        bogus = tmp_path / "corpus.bin"
        bogus.write_text("x", encoding="utf-8")

        with pytest.raises(UnsupportedFormatError) as excinfo:
            converter.convert_file(bogus, tmp_path / "out.json")
        message = str(excinfo.value)

        assert "corpus.bin" in message, message
        assert "--input-format" in message, message

    def test_message_lists_are_generated_from_the_constants(self, converter, tmp_path):
        """报错里的两份清单必须由常量生成，而不是抄了一份字面量

        判据是「常量动一下、文案跟着动」：这里往常量里临时加一项，若文案是硬编码的，
        断言就会失败。
        """
        bogus = tmp_path / "corpus.bin"
        bogus.write_text("x", encoding="utf-8")

        from augmentor import converter as converter_module

        extra_ext = ".parquet"
        extra_name = "parquet"
        converter_module.INPUT_EXTENSION_FORMATS[extra_ext] = "parquet"
        converter_module.INPUT_FORMAT_CHOICES.append(extra_name)
        try:
            with pytest.raises(UnsupportedFormatError) as excinfo:
                converter.convert_file(bogus, tmp_path / "out.json")
            message = str(excinfo.value)
            assert extra_ext in message, message
            assert extra_name in message, message
        finally:
            converter_module.INPUT_EXTENSION_FORMATS.pop(extra_ext, None)
            converter_module.INPUT_FORMAT_CHOICES.remove(extra_name)

        with pytest.raises(UnsupportedFormatError) as excinfo:
            converter.convert_file(bogus, tmp_path / "out.json")
        assert extra_ext not in str(excinfo.value), "还原常量后清单没跟着缩回去"

    def test_no_extension_at_all_is_rejected(self, converter, tmp_path):
        path = tmp_path / "README"
        path.write_text("x", encoding="utf-8")
        with pytest.raises(UnsupportedFormatError):
            converter.convert_file(path, tmp_path / "out.json")


# ==================== Excel 读边：真表 ====================

@requires_excel
class TestExcelReadEdge:
    """.xlsx / .xls 输入 → 整张表、全部列、缺失值可安全进 JSON"""

    def test_extension_inference_reads_xlsx(self, converter, xlsx_file, tmp_path):
        out = tmp_path / "out.json"
        result = converter.convert_file(xlsx_file, out, target_format="json")

        assert result["source_format"] == EXCEL_FORMAT
        assert result["input_count"] == 4
        records = json.loads(out.read_text(encoding="utf-8"))
        assert len(records) == 4

    def test_all_columns_survive(self, converter, xlsx_file, tmp_path):
        """不复用 `import_from_excel` 的理由：它只留两列，转换器剥列就是丢数据"""
        out = tmp_path / "out.json"
        converter.convert_file(xlsx_file, out, target_format="json")

        records = json.loads(out.read_text(encoding="utf-8"))
        assert set(records[0]) == {"instruction", "output", "category", "score", "due"}
        assert records[0]["instruction"] == "如何申请租房？"
        assert records[0]["category"] == "租房"

    def test_missing_cells_become_empty_and_output_is_valid_json(self, converter, xlsx_file, tmp_path):
        """NaN/NaT 不许写成 `NaN` 字面量（那是**非法** JSON），口径与 csv 读边一致"""
        out = tmp_path / "out.json"
        converter.convert_file(xlsx_file, out, target_format="json")

        text = out.read_text(encoding="utf-8")
        assert "NaN" not in text and "NaT" not in text, text
        records = json.loads(text)
        assert records[3]["instruction"] == ""
        assert records[2]["score"] == ""
        assert records[3]["category"] == ""

    def test_numeric_and_datetime_cells_keep_their_content(self, converter, xlsx_file, tmp_path):
        out = tmp_path / "out.json"
        converter.convert_file(xlsx_file, out, target_format="json")

        records = json.loads(out.read_text(encoding="utf-8"))
        assert records[0]["score"] == 1.5
        assert str(records[0]["due"]).startswith("2026-01-02")

    @pytest.mark.parametrize("spelling", EXCEL_SOURCE_NAMES)
    def test_three_spellings_are_the_same_edge(self, converter, xlsx_file, tmp_path, spelling):
        out = tmp_path / f"out_{spelling}.json"
        result = converter.convert_file(xlsx_file, out, source_format=spelling,
                                        target_format="json")
        assert result["source_format"] == EXCEL_FORMAT
        assert result["input_count"] == 4

    def test_declared_source_format_overrides_a_lying_suffix(self, converter, xlsx_file, tmp_path):
        """内容是 xlsx、后缀是 `.dat`：显式声明就该读得出来（CLI 的 `--input-format` 之路）"""
        import shutil

        disguised = tmp_path / "disguised.dat"
        shutil.copy(xlsx_file, disguised)
        out = tmp_path / "out.json"

        result = converter.convert_file(disguised, out, source_format="xlsx",
                                        target_format="json")
        assert result["input_count"] == 4

    def test_routes_through_the_csv_family_of_the_graph(self, converter, xlsx_file, tmp_path):
        """Excel 不是 `DataFormat` 成员、也不该多出一条边：它借 csv 那一族走图"""
        assert EXCEL_FORMAT not in {f.value for f in DataFormat}
        assert EXCEL_FORMAT not in get_supported_formats()
        assert ("excel", "json") not in converter._converters

        out = tmp_path / "out.jsonl"
        result = converter.convert_file(xlsx_file, out, target_format="jsonl")
        assert result["target_format"] == "jsonl"
        lines = [ln for ln in out.read_text(encoding="utf-8").splitlines() if ln.strip()]
        assert len(lines) == 4
        assert json.loads(lines[0])["instruction"] == "如何申请租房？"


class TestExcelCellShape:
    """`excel_cell` 的标量优先顺序（`pd.NA != pd.NA` 会当场 TypeError）"""

    def test_python_missing_values(self):
        assert excel_cell(None) == ""
        assert excel_cell(float("nan")) == ""

    def test_python_scalars_pass_through_untouched(self):
        assert excel_cell(True) is True
        assert excel_cell(0) == 0
        assert excel_cell("文本") == "文本"
        assert excel_cell(1.25) == 1.25

    @requires_excel
    def test_pandas_missing_markers_and_timestamps(self):
        import datetime

        import pandas as pd

        # `pd.NA != pd.NA` 返回 NA 本身，对它取真值才抛 TypeError ⇒ 走 except 分支
        assert excel_cell(pd.NA) == ""
        assert excel_cell(pd.NaT) == ""
        assert excel_cell(float("nan")) == ""
        assert str(excel_cell(pd.Timestamp("2026-01-02"))).startswith("2026-01-02")
        assert excel_cell(datetime.date(2026, 1, 2)) == "2026-01-02"


class TestMissingPandasDegradesLoudly:
    """缺 pandas ⇒ `DataLoadError`（带安装指引），不是 ImportError traceback"""

    def test_error_precedes_any_file_access(self, converter, tmp_path, monkeypatch):
        """依赖判据必须排在**读文件**之前：路径根本不存在也先报「装 pandas」"""
        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        out = tmp_path / "out.json"

        with pytest.raises(DataLoadError, match="pandas") as excinfo:
            converter.convert_file(tmp_path / "ghost.xlsx", out, target_format="json")

        assert "openpyxl" in str(excinfo.value)
        assert not out.exists(), "降级路径写出了半成品"


# ==================== A134：`.jsonl` 只序列化一次 ====================

RECORDS = [{"instruction": "甲", "output": "A"}, {"instruction": "乙", "output": "B"}]


def _write_source(tmp_path, fmt, records):
    """把同一批记录按 `json` / `jsonl` / `csv` 三种容器各落一份"""
    import csv as csv_module

    path = tmp_path / f"corpus.{fmt}"
    if fmt == "json":
        path.write_text(json.dumps(records, ensure_ascii=False), encoding="utf-8")
    elif fmt == "jsonl":
        path.write_text("\n".join(json.dumps(r, ensure_ascii=False) for r in records) + "\n",
                        encoding="utf-8")
    else:
        with open(path, "w", encoding="utf-8", newline="") as handle:
            writer = csv_module.DictWriter(handle, fieldnames=["instruction", "output"])
            writer.writeheader()
            writer.writerows(records)
    return path


class TestJsonlSerializedOnce:
    """`convert_file(..., target_format="jsonl")` 的每一行必须是一个 JSON **对象**

    同轮探针实测的修前形态：图里 `("json", "jsonl")` 那条边交出来的已经是字符串行
    （`_json_to_jsonl` 做了 `json.dumps`），`_write_file` 的 jsonl 分支又 dumps 一次
    ⇒ 落盘 `"{\\"instruction\\": ...}"`。自家读边恰好能双重解包（`_jsonl_to_json`
    对 str 也 loads），所以缺陷一直只在跨工具互操作时露头：按标准 JSONL 读的外部程序
    拿到的每条记录都是字符串，取字段当场 `TypeError`。
    """

    @pytest.mark.parametrize("fmt", ["json", "jsonl", "csv"])
    def test_every_text_source_writes_object_lines(self, converter, tmp_path, fmt):
        src = _write_source(tmp_path, fmt, RECORDS)
        out = tmp_path / f"to_{fmt}.jsonl"

        result = converter.convert_file(src, out, source_format=fmt, target_format="jsonl")
        assert result["target_format"] == "jsonl"
        lines = [ln for ln in out.read_text(encoding="utf-8").splitlines() if ln.strip()]
        assert len(lines) == 2
        assert [json.loads(ln) for ln in lines] == RECORDS

    def test_string_records_stay_valid_jsonl(self, converter, tmp_path):
        """passthrough 的字符串数据集：每行是 JSON 字符串字面量，不再多套一层引号"""
        src = tmp_path / "strs.json"
        src.write_text(json.dumps(["甲", "乙"], ensure_ascii=False), encoding="utf-8")
        out = tmp_path / "strs.jsonl"

        converter.convert_file(src, out, target_format="jsonl")
        lines = [ln for ln in out.read_text(encoding="utf-8").splitlines() if ln.strip()]
        assert [json.loads(ln) for ln in lines] == ["甲", "乙"]

    def test_legacy_double_encoded_file_still_reads(self, converter, tmp_path):
        """读侧兼容：修前写出的双重编码文件照样读得回来，读边没有跟着改"""
        legacy = tmp_path / "legacy.jsonl"
        legacy.write_text('"{\\"instruction\\": \\"甲\\", \\"output\\": \\"A\\"}"\n',
                          encoding="utf-8")
        out = tmp_path / "legacy.json"

        result = converter.convert_file(legacy, out, target_format="json")
        assert result["output_count"] == 1
        assert json.loads(out.read_text(encoding="utf-8")) == [
            {"instruction": "甲", "output": "A"}]

    def test_in_memory_edge_shape_is_unchanged(self, converter):
        """只改写边：`convert(..., "jsonl")` 的内存形态仍是字符串行，公开契约不动"""
        lines = converter.convert(RECORDS, "json", "jsonl")
        assert lines == [json.dumps(r, ensure_ascii=False) for r in RECORDS]
        assert all(isinstance(ln, str) for ln in lines)

    def test_jsonl_round_trip_through_files(self, converter, tmp_path):
        """写出来的一份 jsonl 再读回来必须等于原记录（不靠双重解包侥幸对上）"""
        src = _write_source(tmp_path, "json", RECORDS)
        mid = tmp_path / "mid.jsonl"
        back = tmp_path / "back.json"

        converter.convert_file(src, mid, target_format="jsonl")
        converter.convert_file(mid, back, target_format="json")
        assert json.loads(back.read_text(encoding="utf-8")) == RECORDS


# ==================== 写边边界：L80 立项、L84 关闭 ====================
class TestWriteEdgeStillFallsBack:
    """`_infer_format` 只管输出；`.xlsx` 从 L84 起是 Excel，`.xls` 仍回落 json

    这一族用例在 L80 是「已知缺口钉在原地」的形状（`.xlsx` 期望 `json`，注释里写明
    等 A133 补上写边时它会以「期望 json、得到 excel」的形式要求改它）。L84 就是那一轮，
    所以期望值改了、用例留着：回落口径本身（认不出的后缀仍是 json）没变，变的只有
    `.xlsx` 这一格，而 `.xls` 恰好是它反面的证人。
    """

    @pytest.mark.parametrize("name", ["out.xls", "out.dat", "out"])
    def test_unrecognized_output_suffix_is_json(self, converter, name):
        assert converter._infer_format(Path(name)) == "json"

    @pytest.mark.parametrize(("name", "expected"),
                             [("out.json", "json"), ("out.jsonl", "jsonl"),
                              ("out.csv", "csv"), ("out.tsv", "tsv"),
                              ("out.xlsx", EXCEL_FORMAT)])
    def test_recognized_output_suffix(self, converter, name, expected):
        assert converter._infer_format(Path(name)) == expected

    def test_read_edge_and_write_edge_are_two_different_tables(self, converter):
        """`.xlsx` 两边都是 Excel，但 `.xls` 只有读边认 —— 两张表不许被合并成一张

        合并的代价是单向新增的：把写边并进来会让 `.xls` 输出被「认下来」，于是
        `--output x.xls` 自称写了 Excel、实际写下 JSON 或 zip 字节（A133 的原始症状）；
        把读边并进来则会让未知后缀静默回落 json（L80 那一轮修掉的缺陷）。
        """
        assert converter._resolve_input_format(Path("o.xlsx")) == EXCEL_FORMAT
        assert converter._resolve_input_format(Path("o.xls")) == EXCEL_FORMAT
        assert converter._infer_format(Path("o.xlsx")) == EXCEL_FORMAT
        assert converter._infer_format(Path("o.xls")) == "json"
