"""L286 格二：委托导出面的「空数据集静默零写入 + 照样回成功」——原生 CSV 那一格的孪生

`Exporter.export` 对没有原生实现的格式委托给 `EnhancedExporter`（`export.py` 里那条委托边）。
改前 `EnhancedExporter._export_csv` 与 `_export_tsv` 都以 `if not items: return` 开头：格式化函数
**一个字都不写**，而 `export()` 照旧返回 `{"format", "output_path", "item_count": 0, ...}`，
调用方与 HTTP 面拿到的都是成功 —— 回执里那条 `output_path` 指向一个不存在的文件。

这与 `export.py` 原生 CSV 分支刚按 **A343** 立起的判据自相矛盾：那一格改前是 `if data:` 整段跳过、
改后是「产物存在、读回 0 条、没有表头可抄就不谎造表头」。同一族的两条路（原生 / 委托）不许一套判据
两种命运，而 `csv` 与 `tsv` 都是 `POST /api/data/export` 打得到的格式。

处置：委托面的 `csv` / `tsv` 与原生档同形 —— 文件必建，空集就 0 字节，表头只在有条目时才写。
`json` / `jsonl` 两档本来就写产物（`[]` 与空文件），本轮一并钉住当**回归钉**，防下一格从旁边长出来。

尺子面：
1. 逐格式盘面事实：空集 ⇒ 产物存在 + 读回 0 条 + 不谎造表头。
2. 非空回归钉：`csv` / `tsv` 仍然写表头与逐行数据（判据不许把正常档一起放宽）。
3. 委托边仍然成立：`Exporter.export` 的 `csv` 走原生、`tsv` 走委托，空集两侧同形（都落 0 字节
   文件）；非空两侧**列投影不同**是两条路的既有分工，本轮按实测量在案而不是抹平。
4. 牙齿：把 `_export_csv` 换成改前那一档的忠实复刻，产物必须当场消失。
"""

from __future__ import annotations

import csv
import io
import sys
from pathlib import Path

import pytest

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor.export import Exporter  # noqa: E402
from augmentor.export_enhanced import (  # noqa: E402
    EnhancedExporter,
    ExportFormat,
    ExportOptions,
)

#: 委托面里本轮处置的两档 + 本来就写产物的两档（后者作回归钉，一并逐格式点名）。
DELEGATED_FORMATS = [
    ExportFormat.CSV,
    ExportFormat.TSV,
    ExportFormat.JSON,
    ExportFormat.JSONL,
]

ITEMS = [{"instruction": "i1", "output": "o1"}, {"instruction": "i2", "output": "o2"}]


class TestEmptySetStillProducesAnArtifact:
    @pytest.mark.parametrize("fmt", DELEGATED_FORMATS)
    def test_the_delegated_exporter_writes_a_file_for_an_empty_dataset(self, tmp_path, fmt):
        out = tmp_path / f"empty.{fmt.value}"
        result = EnhancedExporter().export([], str(out), ExportOptions(format=fmt))
        assert out.exists(), f"{fmt.value} 空集没有落盘，回执却给了 {result['output_path']}"
        assert result["item_count"] == 0, result
        text = out.read_text(encoding="utf-8")
        if fmt is ExportFormat.JSON:
            assert text == "[]", repr(text)
        else:
            assert text == "", repr(text)
            assert list(csv.reader(io.StringIO(text))) == [], fmt.value

    def test_the_empty_csv_carries_no_fabricated_header(self, tmp_path):
        """空集就是空集：没有真实字段可抄 ⇒ 不许凭空造一行表头冒充字段契约"""
        out = tmp_path / "empty.csv"
        EnhancedExporter().export([], str(out), ExportOptions(format=ExportFormat.CSV))
        assert out.read_text(encoding="utf-8") == ""

    def test_the_receipt_path_is_the_file_that_exists(self, tmp_path):
        out = tmp_path / "sub" / "empty.tsv"
        result = EnhancedExporter().export([], str(out), ExportOptions(format=ExportFormat.TSV))
        assert Path(result["output_path"]) == out
        assert out.exists()


class TestNonEmptyRowsAreUntouched:
    @pytest.mark.parametrize(
        ("fmt", "delimiter"), [(ExportFormat.CSV, ","), (ExportFormat.TSV, "\t")]
    )
    def test_header_and_rows_are_still_written(self, tmp_path, fmt, delimiter):
        out = tmp_path / f"full.{fmt.value}"
        result = EnhancedExporter().export(ITEMS, str(out), ExportOptions(format=fmt))
        assert result["item_count"] == 2, result
        rows = list(csv.reader(io.StringIO(out.read_text(encoding="utf-8")), delimiter=delimiter))
        assert rows[0] == ["instruction", "output"], rows
        assert rows[1:] == [["i1", "o1"], ["i2", "o2"]], rows


class TestTheDelegationEdgeStaysHonest:
    def test_the_native_exporter_delegates_tsv_and_csv_to_the_same_shape(self, tmp_path):
        """`Exporter.export` 的 `csv` 走原生、`tsv` 走委托，两条路的空集产物必须同形

        这一支钉的是「改一处判据，两条路都得对齐」——只对齐委托面会留下原生档，反过来也一样。
        """
        exporter = Exporter()
        for fmt, name in [("csv", "via_native.csv"), ("tsv", "via_native.tsv")]:
            out = tmp_path / name
            exporter.export([], str(out), fmt)
            assert out.exists(), f"Exporter.export 对 {fmt} 空集仍然只回成功不写文件"
            assert out.read_text(encoding="utf-8") == ""

    def test_both_csv_paths_project_their_own_columns_and_write_a_header(self, tmp_path):
        """非空档两侧都写「表头 + 逐行」，但**列投影本来就不同**（本轮没有把它抹平）

        实测同一份 `ITEMS`：原生 `Exporter.export(..., "csv")` 的表头是
        `instruction,input,output`（`_convert_to_csv` 走字段归一），委托面
        `EnhancedExporter` 的表头是观测到的键 `instruction,output`。两条路的分工写在
        `export.py` 的文件头注释里，本轮只对齐「空集必须落盘」这一格，不改列契约。
        """
        a, b = tmp_path / "native.csv", tmp_path / "delegated.csv"
        Exporter().export(ITEMS, str(a), "csv")
        EnhancedExporter().export(ITEMS, str(b), ExportOptions(format=ExportFormat.CSV))
        native = list(csv.reader(io.StringIO(a.read_text(encoding="utf-8"))))
        delegated = list(csv.reader(io.StringIO(b.read_text(encoding="utf-8"))))
        assert native[0] == ["instruction", "input", "output"], native
        assert delegated[0] == ["instruction", "output"], delegated
        assert len(native) == len(delegated) == 3, (native, delegated)


class TestTheRulerHasTeeth:
    def test_the_pre_fix_early_return_makes_the_artifact_disappear(self, tmp_path, monkeypatch):
        """忠实复刻改前那一档（空集直接 `return`），产物必须当场消失

        这一支证明上面那几组盘面事实不是恒真：把判据摘回去，`csv` 又变成「回执说有、盘上没有」。
        """

        def pre_fix(self, items, output, options):  # 改前的形状：空集不写任何字节
            if not items:
                return
            with open(output, "w", encoding="utf-8", newline="") as f:
                writer = csv.DictWriter(f, fieldnames=["instruction", "output"])
                writer.writeheader()
                writer.writerows(items)

        monkeypatch.setattr(EnhancedExporter, "_export_csv", pre_fix)
        out = tmp_path / "teeth.csv"
        result = EnhancedExporter().export([], str(out), ExportOptions(format=ExportFormat.CSV))
        assert not out.exists(), "注入之后仍然写得出文件 ⇒ 第 1 组是恒真的，这把尺子没有牙齿"
        assert result["output_path"] == str(out), "回执仍然指向那个不存在的文件（改前的缺陷形状）"
