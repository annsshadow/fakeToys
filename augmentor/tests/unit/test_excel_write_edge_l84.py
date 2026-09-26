# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L84（A133 写边）：`convert_file` 写得出真 xlsx，且拒写任何「名字与内容不符」的容器

立项事实（L80 记 A133，L84 现量复核）：`csv_excel_import.export_to_excel` 在包里躺了
很久，**产品调用点为零**；同时 `convert --output out.xlsx` 会静默落一份 JSON 字节 ——
`_infer_format` 那张表只认 json/jsonl/csv/tsv，其余一律回落 json，而 `--format` 是必填
参数，走命令行连推断都不经过。

本轮修法（A133 原案是「二选一」，这里取「新增一条写边」那一支的收紧版）：

* 不给 `DataFormat` 加成员、不进 `_converters` 转换图 —— Excel 是**容器**不是 schema，
  图只走到「记录列表」这份规范形，zip 布局交给写边（与 L80 的 `jsonl` 同一口径）。
* 写边表 `OUTPUT_EXTENSION_FORMATS` 与读边表**分家**：读边认 `.xls`（openpyxl 读得了
  BIFF），写边不认（要 xlwt，不在依赖表里）。合并成一张表等于把假容器放回去。
* 二进制容器要求扩展名与声明**互相点名**，三个方向都判，判在 `_write_file` 的门口
  （字节唯一产地），文本那一族不设这条界。

真 xlsx 的用例逐类 `skipif`（aug 侧解释器无 pandas），拒收与缺依赖降级那两族**不需要**
pandas，所以在两侧解释器上都是真跑。
"""

import ast
import json
import zipfile
from pathlib import Path

import pytest

from augmentor import csv_excel_import as cei
from augmentor.converter import (
    EXCEL_FORMAT,
    EXCEL_REJECTED_OUTPUT_SUFFIXES,
    EXCEL_SOURCE_NAMES,
    EXCEL_TARGET_NAMES,
    INPUT_EXTENSION_FORMATS,
    INPUT_FORMAT_CHOICES,
    OUTPUT_EXTENSION_FORMATS,
    OUTPUT_FORMAT_CHOICES,
    DataFormat,
    DatasetConverter,
    csv_fieldnames,
    get_supported_formats,
)
from augmentor.exceptions import DataFormatError, DataLoadError, UnsupportedFormatError


REPO = Path(__file__).resolve().parents[2]
#: 产品码的全部落点（SDK 包 + API + 根目录那个 `cli.py` 入口脚本）。测试与 Temp 不算：
#: 本守卫问的是「有没有入口走到它」，测试自己调用它当然不构成入口。
PRODUCT_PATHS = (REPO / "augmentor", REPO / "api", REPO / "cli.py")
NOISE_DIRS = {"Temp", "__pycache__", "archive", "bak", ".backups"}


def _excel_io_available() -> bool:
    """pandas 与 openpyxl 是否都在（写真 xlsx 需要两者，读回来还需要 openpyxl）"""
    try:
        import openpyxl  # noqa: F401
        import pandas  # noqa: F401
    except ImportError:
        return False
    return True


def product_files():
    for root in PRODUCT_PATHS:
        if root.is_file():
            yield root
        elif root.is_dir():
            for path in sorted(root.rglob("*.py")):
                if not (NOISE_DIRS & set(path.relative_to(REPO).parts)):
                    yield path


def product_call_sites(function_name: str):
    """按**解析后的调用点**数产品码里谁调了 `function_name`（不是按引用字符串筛）

    散文与 docstring 里也会写 `export_to_excel`，用 grep 数会把这些算进去（纪律 (n)
    的同族坑）；AST 只认真正的 call 节点。返回值刻意**不含行号** —— 这份守卫钉的是
    「有哪几处」，带上行号就会因为无关插行而红。
    """
    hits = []
    for path in product_files():
        if path.name == "csv_excel_import.py":  # 定义处本身
            continue
        try:
            tree = ast.parse(path.read_text(encoding="utf-8"))
        except (SyntaxError, UnicodeDecodeError):
            continue
        for node in ast.walk(tree):
            func = node.func if isinstance(node, ast.Call) else None
            name = None
            if isinstance(func, ast.Attribute):
                name = func.attr
            elif isinstance(func, ast.Name):
                name = func.id
            if name == function_name:
                hits.append(path.relative_to(REPO).as_posix())
    return hits


requires_excel = pytest.mark.skipif(
    not _excel_io_available(),
    reason="造/读真 xlsx 需要 pandas 与 openpyxl（aug 侧解释器无 pandas）",
)

ROWS = [
    {"instruction": "如何申请租房？", "output": "登录官网申请", "category": "租房"},
    {"instruction": "租房多少钱？", "output": "按房型定价"},
    {"instruction": "如何退租押金？", "output": "满一年后退还", "category": "退租",
     "note": "只有第三条有第五列"},
]


@pytest.fixture
def converter():
    return DatasetConverter()


@pytest.fixture
def source_json(tmp_path):
    path = tmp_path / "corpus.json"
    path.write_text(json.dumps(ROWS, ensure_ascii=False), encoding="utf-8")
    return path


# ==================== 形状守卫：两张表 + 一份推导清单 ====================

class TestWriteEdgeShape:
    """写边新增的那三份常量各自守住一条界，少守住一条就等于 A133 复发"""

    def test_excel_stays_out_of_the_graph(self, converter):
        """Excel 不是 `DataFormat` 成员、也不进 `_converters`：它是容器，不是节点"""
        assert EXCEL_FORMAT not in {f.value for f in DataFormat}
        assert EXCEL_FORMAT not in get_supported_formats()
        assert ("json", EXCEL_FORMAT) not in converter._converters

    def test_write_table_and_read_table_differ_only_by_xls(self):
        """两张表的差集**恰好**是 `.xls`：多一个少一个都说明有人把它们重新合并了"""
        assert set(INPUT_EXTENSION_FORMATS) - set(OUTPUT_EXTENSION_FORMATS) == {".xls"}
        for suffix, fmt in OUTPUT_EXTENSION_FORMATS.items():
            assert INPUT_EXTENSION_FORMATS[suffix] == fmt

    def test_xlsx_is_the_only_binary_container_on_the_write_side(self):
        assert OUTPUT_EXTENSION_FORMATS[".xlsx"] == EXCEL_FORMAT
        assert EXCEL_REJECTED_OUTPUT_SUFFIXES == (".xls",)
        assert set(EXCEL_TARGET_NAMES) | {"xls"} == set(EXCEL_SOURCE_NAMES)

    def test_target_choices_cover_the_graph_plus_excel(self):
        """目标清单 = 图里的十个 + excel/xlsx，且**不含** xls

        只比集合差还不够：还要证明十项 schema 与读边同源（有人手抄一份就会漏项）。
        """
        graph = set(INPUT_FORMAT_CHOICES) - set(EXCEL_SOURCE_NAMES)
        assert set(OUTPUT_FORMAT_CHOICES) == graph | set(EXCEL_TARGET_NAMES)
        assert "xls" not in OUTPUT_FORMAT_CHOICES
        assert [f for f in INPUT_FORMAT_CHOICES if f not in EXCEL_SOURCE_NAMES] == \
            OUTPUT_FORMAT_CHOICES[:len(graph)]

    def test_export_to_excel_now_has_a_product_caller(self):
        """A133 的原始症状反面：`export_to_excel` 从「零调用者」变成恰好一处

        钉「== 1」而不是「>= 1」：再加调用点要先问该不该复用同一条写边（两处就是两份
        Excel 落盘口径，正是 L80 立项时批评的那种「实现有、入口散」）。
        """
        assert product_call_sites("export_to_excel") == ["augmentor/converter.py"], \
            "Excel 写边的调用点变了：先确认新调用点该不该复用 `_write_excel`"


# ==================== `_infer_format` 逐档 ====================

class TestOutputSuffixInference:
    @pytest.mark.parametrize(("name", "expected"), [
        ("o.json", "json"), ("o.jsonl", "jsonl"), ("o.csv", "csv"),
        ("o.tsv", "tsv"), ("o.xlsx", EXCEL_FORMAT),
    ])
    def test_recognized(self, converter, name, expected):
        assert converter._infer_format(Path(name)) == expected

    @pytest.mark.parametrize("name", ["o.xls", "o.dat", "o"])
    def test_unrecognized_still_falls_back_to_json(self, converter, name):
        """回落没被删（本轮只是多认了 `.xlsx`）；`.xls` 的拒收发生在写门口

        把 `.xls` 的判据也放进推断函数会让「推格式」这件事混进「能不能写」，而
        `convert_file` 在调用方**显式给了**目标格式时根本不推扩展名 —— 那一刀必须
        站在字节产地，否则 `--format json --output o.xls` 这条路照旧写出假容器。
        """
        assert converter._infer_format(Path(name)) == "json"


# ==================== 真落盘（需要 pandas） ====================

@requires_excel
class TestRealWorkbookWritten:
    def test_output_is_a_zip_and_not_json_text(self, converter, source_json, tmp_path):
        out = tmp_path / "corpus.xlsx"
        result = converter.convert_file(source_json, out, target_format="xlsx")
        raw = out.read_bytes()

        assert raw[:4] == b"PK\x03\x04"
        assert zipfile.is_zipfile(out)
        assert raw[:1] not in (b"{", b"[", b'"'), \
            "写出的仍是 JSON 文本（A133 的原始症状：zip 名字下装 JSON）"
        assert result["target_format"] == EXCEL_FORMAT
        assert (result["input_count"], result["output_count"]) == (3, 3)

    def test_sheet_and_header(self, converter, source_json, tmp_path):
        import pandas as pd

        out = tmp_path / "corpus.xlsx"
        converter.convert_file(source_json, out, target_format="excel")

        frame = pd.read_excel(out)
        assert list(frame.columns) == csv_fieldnames(ROWS), \
            "列序必须与 csv 写边同源（首次出现的全量键并集）"
        assert frame["note"].isna().sum() == 2, "异构记录的缺列要落空单元格"

    def test_round_trip_keeps_every_column(self, converter, source_json, tmp_path):
        """xlsx → json 往返：三条记录的四个字段逐个回来（写边没剥列）"""
        book = tmp_path / "book.xlsx"
        back = tmp_path / "back.json"
        converter.convert_file(source_json, book, target_format="excel")
        converter.convert_file(book, back, target_format="json")

        got = json.loads(back.read_text(encoding="utf-8"))
        assert [(r["instruction"], r["output"]) for r in got] == \
            [(r["instruction"], r["output"]) for r in ROWS]
        assert got[0]["category"] == "租房"
        assert got[2]["note"] == "只有第三条有第五列"

    def test_empty_dataset_writes_an_empty_workbook(self, converter, tmp_path):
        src = tmp_path / "empty.json"
        src.write_text("[]", encoding="utf-8")
        out = tmp_path / "empty.xlsx"

        result = converter.convert_file(src, out, target_format="excel")
        assert result["output_count"] == 0
        assert zipfile.is_zipfile(out)

    def test_target_spelling_both_work_and_normalize(self, converter, source_json, tmp_path):
        for spelling in EXCEL_TARGET_NAMES:
            out = tmp_path / f"spell_{spelling}.xlsx"
            result = converter.convert_file(source_json, out, target_format=spelling)
            assert result["target_format"] == EXCEL_FORMAT
            assert zipfile.is_zipfile(out)

    def test_no_extension_still_writes_json_into_a_json_named_file(
            self, converter, source_json, tmp_path):
        """回落那一族的行为没动：认不出的后缀仍写 JSON 字节、仍叫 json

        这条是「收紧不等于扩大」的证人：本轮只把 xlsx 提上来，`.dat` 的既有口径不变。
        """
        out = tmp_path / "mystery.dat"
        result = converter.convert_file(source_json, out)
        assert result["target_format"] == "json"
        assert json.loads(out.read_text(encoding="utf-8")) == ROWS

    def test_nested_values_match_the_csv_write_edge_literal(
            self, converter, tmp_path):
        """多轮记录的 `history` 在两个写边里落成同一段字面量（都不是崩溃）"""
        import pandas as pd

        rows = [{"instruction": "q1", "output": "a1",
                 "history": [{"role": "user", "content": "早一问"},
                             {"role": "assistant", "content": "早一答"}]}]
        src = tmp_path / "multi.json"
        src.write_text(json.dumps(rows, ensure_ascii=False), encoding="utf-8")
        book, table = tmp_path / "multi.xlsx", tmp_path / "multi.csv"
        converter.convert_file(src, book, target_format="excel")
        converter.convert_file(src, table, target_format="csv")

        cell = str(pd.read_excel(book).to_dict(orient="records")[0]["history"])
        assert cell in table.read_text(encoding="utf-8-sig"), \
            "openpyxl 对嵌套值同样 str() 化，两条写边不许一条崩一条写"


# ==================== 拒收矩阵（不需要 pandas） ====================

class TestFalseContainerRefused:
    """三个方向都要响亮拒收，且**一个字节都不落**"""

    CASES = [
        # (说明, convert_file 的 target_format, 输出名, 报错文案必须点到的那一向)
        ("目标由扩展名推断", None, "out.xls", "只产 .xlsx"),
        ("显式目标 json", "json", "out2.xls", "只产 .xlsx"),
        ("显式目标 excel", "excel", "out3.xls", "只产 .xlsx"),
        ("扩展名是 xlsx 但目标是 json", "json", "mix.xlsx", "的扩展名要求 Excel"),
        ("目标是 excel 但扩展名是 json", "excel", "mix.json", "扩展名不是 .xlsx"),
        ("目标写成 xls 这种拼法", "xls", "named.xlsx", "不支持的目标格式"),
    ]

    @pytest.mark.parametrize(("label", "target", "name", "phrase"), CASES)
    def test_refused_without_touching_the_disk(self, converter, source_json, tmp_path,
                                               label, target, name, phrase):
        """三向拒收要**各报各的那一向**，不只是「抛同一个异常类」

        第 4 个字段是注入分判（`Temp/l84q/injection_out.txt` 的 m4）逼出来的：把
        「扩展名要 excel 而声明不要」那一向摘掉后，控制流会落到另一向的 `raise` 上，
        异常类不变、只有文案换人 —— 光比类型的矩阵于是照绿，把方向搞混这件事在单元面
        上就成了不可见。文案点到的那一句才是这一行真正的判据。
        """
        out = tmp_path / name
        with pytest.raises(UnsupportedFormatError) as exc:
            converter.convert_file(source_json, out, target_format=target)
        assert phrase in str(exc.value), f"{label}：文案没点到这一向 → {exc.value}"
        assert not out.exists(), f"{label}：报错前已经落了文件"

    def test_xls_message_names_the_way_out(self, converter, source_json, tmp_path):
        with pytest.raises(UnsupportedFormatError) as exc:
            converter.convert_file(source_json, tmp_path / "a.xls")
        text = str(exc.value)
        assert "xlwt" in text and ".xlsx" in text, text
        assert ", ".join(OUTPUT_FORMAT_CHOICES) in text, \
            "文案里的清单要从常量生成，否则加一个目标格式就漂一份"

    def test_mismatch_message_offers_both_fixes(self, converter, source_json, tmp_path):
        with pytest.raises(UnsupportedFormatError) as exc:
            converter.convert_file(source_json, tmp_path / "b.xlsx",
                                   target_format="csv")
        text = str(exc.value)
        assert "的扩展名要求 Excel" in text, \
            f"这一行钉的是「扩展名要 excel 而声明不要」那一向，报成另一向就算漂了: {text}"
        assert "csv" in text and "xlsx" in text

    def test_refusal_happens_before_the_parent_directory_is_made(
            self, converter, source_json, tmp_path):
        """拒绝一次不该留下目录：判据站在 `mkdir` 之前

        反过来（先建目录再拒）会让「试错式调用」在数据白名单里留下一堆空目录。
        """
        out = tmp_path / "deep" / "nested" / "c.xls"
        with pytest.raises(UnsupportedFormatError):
            converter.convert_file(source_json, out)
        assert not out.parent.exists(), "拒收前就 mkdir 了"

    def test_missing_pandas_degrades_to_data_load_error(
            self, converter, source_json, tmp_path, monkeypatch):
        """缺依赖的报错停在「装依赖」，且不需要本机真没装 pandas 也能验

        `monkeypatch` 让两侧解释器跑的是同一条断言（aug 侧本来就没 pandas，标记为
        跳过反而会让这条能力面在它那里从不被验证）。
        """
        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        out = tmp_path / "d.xlsx"
        with pytest.raises(DataLoadError) as exc:
            converter.convert_file(source_json, out, target_format="excel")
        assert "pip install pandas openpyxl" in str(exc.value)
        assert not out.exists()

    def test_non_object_records_are_refused_before_the_dependency_gate(
            self, converter, tmp_path, monkeypatch):
        """数据错要先报数据，不能被「装依赖」抢话

        aug 侧解释器（无 pandas）第一次跑两侧时就是这一格红的：它拿到的是
        `DataLoadError`「pip install pandas openpyxl」，而装完依赖那条标量行照样写不进
        表头。判据顺序因此是契约，不是实现细节 ⇒ 用 `monkeypatch` 把 pandas 门关掉，
        让两侧解释器跑同一条断言（与 `test_missing_pandas_degrades_to_data_load_error`
        同一手法，也顺手钉住「先判内容、后判依赖」那个顺序）。
        """
        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        bad = tmp_path / "bad.json"
        bad.write_text(json.dumps([{"instruction": "q"}, ["not", "a", "dict"]]),
                       encoding="utf-8")
        with pytest.raises(DataFormatError) as exc:
            converter.convert_file(bad, tmp_path / "e.xlsx", target_format="excel")
        assert "第 2 条记录必须是 JSON 对象" in str(exc.value), str(exc.value)


# ==================== 读边没被本轮撞坏 ====================

class TestReadEdgeStillIntact:
    def test_xls_input_is_still_recognized(self, converter):
        """`.xls` 只是写不出来，读它仍然合法：本轮没顺手改掉读边

        这条与 `test_read_table_and_write_table_differ_only_by_xls` 是一对：前者管
        「判定」，后者管「两张表的差集」。只留后者的话，有人把 `.xls` 从读边表里删掉
        也照样达标（两张表差集变成两个元素才会红，而那时写边表也会被牵动）。
        """
        assert converter._resolve_input_format(Path("legacy.xls")) == EXCEL_FORMAT
        assert converter._resolve_input_format(Path("legacy.xlsx")) == EXCEL_FORMAT
        assert INPUT_EXTENSION_FORMATS[".xls"] == EXCEL_FORMAT
