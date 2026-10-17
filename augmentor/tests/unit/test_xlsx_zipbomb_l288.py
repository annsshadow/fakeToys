# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L288 / A390：xlsx 容器的解压放大（CWE-409）——两条臂的判据、接线与不被翻译吞掉

改前实测（本轮仓外探针，样本构造与逐格读数记在卷二 L288 块）：Excel 读边除了「压缩后的
上传字节上限」（`web.max_upload_bytes`，默认 256 MiB）之外**没有任何尺寸或比率判据**，
而 zip 容器天然可以把几十 KB 展开成几百 MiB：

- 合法侧现量最高比率 **36.9**（20000 行 × 50 列同字符填满的工作簿，解压后 83.8 MiB），
  本仓写边产出的 3000 行随机文本档案比率 **3.5**；
- 炸弹侧现量最低 **1025.4**（30 个成员、每个 4 MB 同字符，解压后 114.4 MiB）与
  **1026.3**（200 个成员，解压后 1525.9 MiB）。

⇒ 三条阈值按这两侧各留一个数量级定：上限 512 MiB、比率 100、比率判据的地板 32 MiB。
地板不是装饰：合法档案里「同字符填满」那种天然比率就在 1000 以上，没有地板会把一个
4 MiB 的无害工作簿拒掉，而它整个展开完也只占 4 MiB 内存。

判据住 `augmentor/validation.py`（`sheet_expansion_verdict` 与三条常数），**两条臂**跑同
一套判据、读两份数：

1. **中央目录声明**的尺寸——不碰压缩流，诚实声明的炸弹在这里就停下（第 3 组）；
2. **实际解压出来的字节数**——分块计数、边读边比上限、超了就早停，闸门自己因此不会
   变成第二处内存开销。当前解释器上 `zipfile` 自己会按声明尺寸截断并以 `Bad CRC-32`
   掐断谎报档，所以这一臂的「拒绝档」只能用替身打红（第 4 组）；它买的是「不信任声明值」
   这件事本身，成本实测占 `read_excel` 自身耗时的 1%（51.4 MiB 的档案 0.05 s 对 8.67 s）。

接线面（第 5 组）：产品树里两处 `pd.read_excel` 调用点（`converter._read_excel_source`
与 `csv_excel_import.import_from_excel`）的参数位都必须包着 `safe_sheet_source`，**数量
也钉死为 2**——第三条读边长出来而没接闸门，这一组当场点名。

第 6 组钉的是本轮最容易写错的一格：`_read_excel_source` 里那道 `except Exception →
DataFormatError` 翻译（L91 为「把损坏文件说成服务端故障」立的）会把解压放大的判词一起
吞成「文件可能损坏」，所以翻译之前先加一支按类型放行。第 7 组管另一侧：非 zip 容器
（老 `.xls` 是 BIFF8）与读不动的来源一律放行，既有报错口径一字不动。第 8 组逐档摘掉
权威，证明三条阈值各吃掉一档、没有一把是恒真或恒假。
"""

from __future__ import annotations

import ast
import io
import math
import struct
import sys
import zipfile
from pathlib import Path
from typing import Dict, List, Tuple

import pytest

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor import csv_excel_import as cei  # noqa: E402
from augmentor import validation  # noqa: E402
from augmentor.converter import DatasetConverter, read_records  # noqa: E402
from augmentor.exceptions import DataFormatError, DataValidationError  # noqa: E402

MB = 1024 * 1024

#: 本轮现量读数（构造见卷二 L288 块 ①），第 2 组按它们核对阈值两侧的余地
MEASURED = {
    "legit_max_ratio": 36.9,        # 20000 行 × 50 列同字符填满，解压后 83.8 MiB
    "legit_writer_ratio": 3.5,      # 本仓 openpyxl 写边产出、3000 行随机文本
    "bomb_min_ratio": 1025.4,       # 30 员 × 4 MB 同字符，解压后 114.4 MiB
    "legit_max_expanded_mib": 83.8,
    "bomb_min_expanded_mib": 114.4,
}

PRODUCT_ROOTS = ("augmentor", "api")
_NS = ('xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" '
       'xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"')
_CT = ('<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
       '<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">'
       '<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>'
       '<Default Extension="xml" ContentType="application/xml"/>'
       '<Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>'
       '<Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>'
       '</Types>')
_RELS = ('<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
         '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
         '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>'
         '</Relationships>')
_WRELS = ('<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
          '<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
          '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>'
          '</Relationships>')
_SHEET_HEAD = ('<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
               '<worksheet %s><sheetData>' % _NS)
_SHEET_TAIL = "</sheetData></worksheet>"


def zip_bytes(sizes: List[int]) -> bytes:
    """按「每员解压后字节数」造 zip 容器（同字符填满 ⇒ 压缩体极小、比率极高）"""
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as zf:
        for idx, size in enumerate(sizes):
            zf.writestr("xl/worksheets/sheet%d.xml" % idx, "A" * size)
    return buf.getvalue()


def valid_xlsx(rows: int) -> bytes:
    """openpyxl 与 pandas 都读得通的最小合法工作簿：首行是表头，其余各两列

    不用 `openpyxl` 造这份档案是有原因的：实测它的写边把单格截到 32767 字符（申请 64 MiB
    的一格只落出 49 KB 的包），造不出「大成员」那一族形状；而 L287 那批手写的最小 xlsx 缺
    `[Content_Types]` 与 rels，openpyxl 直接报「no valid workbook part」，用它量到的
    「失败即安全」是容器写法污染的假读数。这一版结构合法、成员尺寸可控。
    """
    body = ('<row r="1"><c r="A1" t="inlineStr"><is><t>instruction</t></is></c>'
            '<c r="B1" t="inlineStr"><is><t>output</t></is></c></row>')
    for r in range(2, rows + 1):
        body += ('<row r="%d"><c r="A%d" t="inlineStr"><is><t>x%d</t></is></c>'
                 '<c r="B%d" t="inlineStr"><is><t>y%d</t></is></c></row>' % (r, r, r, r, r))
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as zf:
        zf.writestr("[Content_Types].xml", _CT)
        zf.writestr("_rels/.rels", _RELS)
        zf.writestr("xl/workbook.xml",
                    '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
                    '<workbook %s><sheets><sheet name="s" sheetId="1" r:id="rId1"/>'
                    '</sheets></workbook>' % _NS)
        zf.writestr("xl/_rels/workbook.xml.rels", _WRELS)
        zf.writestr("xl/worksheets/sheet1.xml", _SHEET_HEAD + body + _SHEET_TAIL)
    return buf.getvalue()


def rewrite_central_sizes(data: bytes, file_size: int, compress_size: int) -> bytes:
    """把中央目录里每一员的声明尺寸改成给定值（真字节与 CRC 不动）"""
    eocd = data.rfind(b"PK\x05\x06")
    cd_size, cd_off = struct.unpack_from("<II", data, eocd + 12)
    out = bytearray(data)
    pos, end = cd_off, cd_off + cd_size
    while pos < end:
        name_len, extra_len, comment_len = struct.unpack_from("<HHH", out, pos + 28)
        struct.pack_into("<I", out, pos + 20, compress_size)
        struct.pack_into("<I", out, pos + 24, file_size)
        pos += 46 + name_len + extra_len + comment_len
    return bytes(out)


def declared_sums(data: bytes) -> Tuple[int, int]:
    with zipfile.ZipFile(io.BytesIO(data)) as zf:
        return (sum(i.compress_size for i in zf.infolist()),
                sum(i.file_size for i in zf.infolist()))


class ReadingSpy:
    """产品扫描器：找产品树里每一处 `read_excel(...)`，把整条调用的字面写法带回来"""

    @staticmethod
    def product_files() -> List[Path]:
        return sorted(p for root in PRODUCT_ROOTS for p in (REPO / root).rglob("*.py"))

    @staticmethod
    def calls_in(tree: ast.AST, label: str) -> List[Tuple[str, str]]:
        hits = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute):
                if node.func.attr == "read_excel":
                    hits.append(("%s:%d" % (label, node.lineno), ast.unparse(node)))
        return hits

    @classmethod
    def calls(cls) -> List[Tuple[str, str]]:
        return [
            hit
            for path in cls.product_files()
            for hit in cls.calls_in(
                ast.parse(path.read_text(encoding="utf-8")),
                path.relative_to(REPO).as_posix(),
            )
        ]


class LyingArchive:
    """替身容器：中央目录把声明尺寸报得很小，字节流却一直往外吐

    只为单独打红第二臂而存在。真谎报档在本解释器上先由 `zipfile` 按声明尺寸截断并以
    `Bad CRC-32` 掐断（第 4 组第二条用例量的就是这件事），所以「数出来的实际字节超过
    上限」那一档拒绝在真档案上今天打不到 ⇒ 用替身证明那一臂有牙齿，而不是把它当恒真。
    """

    def __init__(self, stream_total: int, declared: int, compress_size: int):
        self._info = self._Info(declared, compress_size)
        self._stream_total = stream_total
        self.closed = False

    class _Info:
        def __init__(self, declared: int, compress_size: int):
            self.filename = "xl/worksheets/sheet1.xml"
            self.file_size = declared
            self.compress_size = compress_size

    def infolist(self) -> List["LyingArchive._Info"]:
        return [self._info]

    def open(self, info) -> "LyingArchive._Stream":
        return self._Stream(self._stream_total)

    def close(self) -> None:
        self.closed = True

    def __enter__(self) -> "LyingArchive":
        return self

    def __exit__(self, *exc) -> bool:
        self.close()
        return False

    class _Stream:
        def __init__(self, total: int):
            self._left = total

        def __enter__(self) -> "LyingArchive._Stream":
            return self

        def __exit__(self, *exc) -> bool:
            return False

        def read(self, size: int) -> bytes:
            if self._left <= 0:
                return b""
            take = min(size, self._left)
            self._left -= take
            return b"A" * take


# ---------------------------------------------------------------------------
# 第 1 组：合法面——闸门不许挡住读得通的档案，也不许把流的位置弄坏
# ---------------------------------------------------------------------------

def test_legit_workbook_reads_through_every_edge(tmp_path: Path) -> None:
    """三条入口读同一份合法工作簿：接上闸门之后一条记录都不少、内容一字不改"""
    data = valid_xlsx(200)
    path = tmp_path / "good.xlsx"
    path.write_bytes(data)

    for label, rows in [
        ("上传内存入口", read_records(data, "xlsx", "good.xlsx")),
        ("按路径入口", DatasetConverter()._read_excel(path)),
        ("csv_excel_import 入口", cei.import_from_excel(path)),
    ]:
        assert len(rows) == 199, label
    assert read_records(data, "xlsx", "good.xlsx")[0]["instruction"] == "x2"


def test_gate_returns_the_same_object_and_rewinds_it() -> None:
    """闸门返回进入时那个对象，且把位置回卷到 0——解析器接着读的是同一份字节"""
    stream = io.BytesIO(valid_xlsx(50))
    assert cei.safe_sheet_source(stream, "good.xlsx") is stream
    assert stream.read(4) == b"PK\x03\x04"


def test_gate_passes_a_path_source_through_untouched(tmp_path: Path) -> None:
    """路径入口同样过闸门，而它没有 `seek` ⇒ 回卷那一支不执行，对象原样返回"""
    path = tmp_path / "good.xlsx"
    path.write_bytes(valid_xlsx(50))
    assert cei.safe_sheet_source(path, path.name) is path


# ---------------------------------------------------------------------------
# 第 2 组：定标账——阈值两侧各留多少余地，按现量读数核对
# ---------------------------------------------------------------------------

def test_thresholds_keep_measured_headroom_on_both_sides() -> None:
    """合法侧最高比率在上限的一半以内，炸弹侧最低比率在上限的十倍以外"""
    assert MEASURED["legit_max_ratio"] < validation.MAX_SHEET_EXPANSION_RATIO / 2
    assert MEASURED["legit_writer_ratio"] < validation.MAX_SHEET_EXPANSION_RATIO / 2
    assert MEASURED["bomb_min_ratio"] > validation.MAX_SHEET_EXPANSION_RATIO * 10
    assert MEASURED["legit_max_expanded_mib"] * MB < validation.MAX_SHEET_EXPANDED_BYTES
    assert MEASURED["bomb_min_expanded_mib"] * MB > validation.MIN_SHEET_RATIO_JUDGMENT_BYTES


# ---------------------------------------------------------------------------
# 第 3 组：第一臂（中央目录声明），两条子臂各自点名
# ---------------------------------------------------------------------------

def test_honest_bomb_is_rejected_by_its_declared_ratio() -> None:
    """现量炸弹（30 员 × 4 MB，声明解压后 114 MiB、比率 1000 以上）⇒ 比率子臂点名"""
    data = zip_bytes([4_000_000] * 30)
    compressed, expanded = declared_sums(data)
    assert expanded == pytest.approx(120_000_000, rel=0.01)
    assert compressed < MB, "压缩体不到 1 MiB，这正是放大那一族的形状"
    assert expanded / max(compressed, 1) > validation.MAX_SHEET_EXPANSION_RATIO

    with pytest.raises(DataValidationError) as caught:
        cei.safe_sheet_source(io.BytesIO(data), "bomb.xlsx")
    text = str(caught.value)
    assert "中央目录声明" in text and "比率" in text and "bomb.xlsx" in text


def test_honest_bomb_is_rejected_by_its_declared_ceiling() -> None:
    """声明解压后 608 MiB 而压缩体 8 MiB（比率 76，在比率上限之内）⇒ 只有上限子臂抓得到"""
    data = rewrite_central_sizes(zip_bytes([1024] * 8), 76 * MB, 1 * MB)
    compressed, expanded = declared_sums(data)
    assert (compressed, expanded) == (8 * MB, 608 * MB)
    assert expanded / compressed < validation.MAX_SHEET_EXPANSION_RATIO

    with pytest.raises(DataValidationError) as caught:
        cei.safe_sheet_source(io.BytesIO(data), "big.xlsx")
    text = str(caught.value)
    assert "中央目录声明" in text and "超过上限" in text


def test_small_high_ratio_container_is_not_rejected() -> None:
    """地板吃重的正面档：4 MiB、比率 1000 以上，展开完就 4 MiB ⇒ 放行"""
    data = zip_bytes([4 * MB])
    compressed, expanded = declared_sums(data)
    assert expanded / max(compressed, 1) > validation.MAX_SHEET_EXPANSION_RATIO
    assert cei.safe_sheet_source(io.BytesIO(data), "small.xlsx") is not None


# ---------------------------------------------------------------------------
# 第 4 组：第二臂（实际字节流）
# ---------------------------------------------------------------------------

@pytest.mark.parametrize("stream_total,phrase", [
    (40 * MB, "比率"),
    (validation.MAX_SHEET_EXPANDED_BYTES + 5 * MB, "超过上限"),
])
def test_actual_stream_is_counted_not_trusted(
    monkeypatch: pytest.MonkeyPatch, stream_total: int, phrase: str
) -> None:
    """声明说「1000 字节」而字节流一直吐 ⇒ 第二臂数出来，文案点名「实际字节流」"""
    archive = LyingArchive(stream_total, 1000, 100)
    monkeypatch.setattr(zipfile, "ZipFile", lambda source: archive)
    with pytest.raises(DataValidationError) as caught:
        cei.safe_sheet_source(io.BytesIO(b"ignored by the stand-in"), "liar.xlsx")
    text = str(caught.value)
    assert "实际字节流" in text and phrase in text
    assert archive.closed, "闸门没有关掉它打开的容器"


def test_lying_archive_never_reaches_the_parser(monkeypatch: pytest.MonkeyPatch) -> None:
    """真谎报档（改中央目录声明尺寸）在解析器之前就被拦下：`read_excel` 一次都没被调用

    当前解释器的 `zipfile` 自己按声明尺寸截断并以 `Bad CRC-32` 掐断，闸门因此在这一臂
    拿到的是解释器的错；产品路径仍把它收敛成「读不出 Excel 工作表」那一档 400，而不是
    让几百 MiB 的工作表先进内存再报错。两条性质都由这一支钉住。
    """
    data = rewrite_central_sizes(zip_bytes([40 * MB] * 4), 1000, 1024)
    calls: List[str] = []

    def _spy(*args, **kwargs):
        calls.append("called")
        return _empty_frame()

    monkeypatch.setattr(cei.pd, "read_excel", _spy)
    with pytest.raises(DataFormatError) as caught:
        read_records(data, "xlsx", "liar.xlsx")
    assert calls == [], "谎报档在闸门之后还被交给了解析器"
    assert "读不出 Excel 工作表" in str(caught.value)


def _empty_frame():
    class _Frame:
        def to_dict(self, orient: str) -> List[Dict]:
            return []

    return _Frame()


# ---------------------------------------------------------------------------
# 第 5 组：接线普查——判据钉在「Excel 读边」这一族上
# ---------------------------------------------------------------------------

def test_every_excel_read_call_wraps_the_gate() -> None:
    """产品树里每一处 `read_excel(...)` 的参数位都包着 `safe_sheet_source`，现量两处"""
    hits = ReadingSpy.calls()
    assert len(hits) == 2, hits
    for label, call in hits:
        assert "safe_sheet_source" in call, (label, call)


def test_wiring_arm_names_an_unwrapped_call() -> None:
    """牙齿：一条没包闸门的读边喂给同一个扫描器必须被点名，否则第 5 组是恒绿的"""
    forged = ("import pandas as pd\n"
              "def read_it(path):\n"
              "    return pd.read_excel(path)\n")
    hits = ReadingSpy.calls_in(ast.parse(forged), "synthetic.py")
    assert len(hits) == 1 and "safe_sheet_source" not in hits[0][1]


def test_threshold_authority_lives_in_one_file() -> None:
    """三条阈值只在 `augmentor/validation.py` 赋值一次，别处只许共引（A77 那一族）"""
    names = ("MAX_SHEET_EXPANDED_BYTES", "MAX_SHEET_EXPANSION_RATIO",
             "MIN_SHEET_RATIO_JUDGMENT_BYTES")
    offenders = []
    for path in ReadingSpy.product_files():
        if path.name == "validation.py":
            continue
        for node in ast.walk(ast.parse(path.read_text(encoding="utf-8"))):
            if isinstance(node, (ast.Assign, ast.AugAssign)):
                targets = node.targets if isinstance(node, ast.Assign) else [node.target]
                for target in targets:
                    if isinstance(target, ast.Name) and target.id in names:
                        offenders.append(
                            "%s:%d" % (path.relative_to(REPO).as_posix(), node.lineno))
    assert offenders == []


# ---------------------------------------------------------------------------
# 第 6 组：解压放大的判词不许被「文件可能损坏」那道翻译吞掉
# ---------------------------------------------------------------------------

def test_bomb_verdict_is_not_translated_into_a_corrupt_file() -> None:
    """判词原样放行：类型是 `DataValidationError`，文案留着「解压放大」

    L91 给 `_read_excel_source` 加的 `except Exception → DataFormatError` 是必要的
    （损坏 xlsx 在 pandas 侧抛的不是 ValueError 子类），但它无条件吞一切 ⇒ 不加这支
    按类型放行，客户端拿到的是「文件可能损坏」这种把参数缺陷说成故障的判词。
    """
    data = zip_bytes([4_000_000] * 30)
    with pytest.raises(DataValidationError) as caught:
        read_records(data, "xlsx", "bomb.xlsx")
    assert "解压放大" in str(caught.value)
    assert not isinstance(caught.value, DataFormatError)


def test_corrupt_upload_still_gets_the_corrupt_file_wording() -> None:
    """同一道翻译对**真损坏**的档案一字未动：既不是 zip 也不是 xlsx 的字节仍回那句话"""
    with pytest.raises(DataFormatError) as caught:
        read_records(b"this is not a workbook at all", "xlsx", "broken.xlsx")
    assert "读不出 Excel 工作表" in str(caught.value)


# ---------------------------------------------------------------------------
# 第 7 组：闸门对非 zip 容器与读不动的来源一律放行（老 .xls 那一族的口径不变）
# ---------------------------------------------------------------------------

def test_non_zip_container_passes_the_gate(tmp_path: Path) -> None:
    """BIFF8 的 `.xls` 不是 zip：认不出容器就什么也不判，交回原路径"""
    legacy = tmp_path / "old.xls"
    legacy.write_bytes(b"\xd0\xcf\x11\xe0\xa1\xb1\x1a\xe1" + b"0" * 512)
    assert cei.safe_sheet_source(legacy, legacy.name) is legacy
    assert cei.safe_sheet_source(io.BytesIO(b"plain text"), "old.xls") is not None


def test_missing_path_is_not_the_gate_to_report(tmp_path: Path) -> None:
    """不存在的路径不由闸门发言（`OSError` 支放行），报错口径留给既有读边"""
    missing = tmp_path / "nope.xlsx"
    assert cei.safe_sheet_source(missing, missing.name) is missing
    with pytest.raises(DataFormatError) as caught:
        DatasetConverter()._read_excel(missing)
    assert "nope.xlsx" in str(caught.value)


# ---------------------------------------------------------------------------
# 第 8 组：牙齿——三条阈值各吃掉一档，没有一把是恒真或恒假
# ---------------------------------------------------------------------------

@pytest.mark.parametrize("case", ["ratio", "ceiling", "floor"])
def test_each_threshold_is_load_bearing(
    monkeypatch: pytest.MonkeyPatch, case: str
) -> None:
    """同一对读数在三档权威下各判一次：摘掉哪一条，那一档就从「拒」翻成「放行」"""
    if case == "ratio":
        # 114 MiB / 0.1 MiB：比率子臂抓到；把比率换成无穷 ⇒ 同一份档案过闸
        with pytest.raises(DataValidationError):
            validation.sheet_expansion_verdict(100 * 1024, 114 * MB, "b.xlsx", "中央目录声明")
        monkeypatch.setattr(validation, "MAX_SHEET_EXPANSION_RATIO", math.inf)
        validation.sheet_expansion_verdict(100 * 1024, 114 * MB, "b.xlsx", "中央目录声明")
    elif case == "ceiling":
        # 610 MiB / 8 MiB：比率 76 在上限内，只有绝对上限抓到
        with pytest.raises(DataValidationError):
            validation.sheet_expansion_verdict(8 * MB, 610 * MB, "c.xlsx", "中央目录声明")
        monkeypatch.setattr(validation, "MAX_SHEET_EXPANDED_BYTES", math.inf)
        validation.sheet_expansion_verdict(8 * MB, 610 * MB, "c.xlsx", "中央目录声明")
    else:
        # 4 MiB / 4 KiB：比率 1000 但没到地板 ⇒ 本来放行；地板收成 0 就误伤合法小档案
        validation.sheet_expansion_verdict(4 * 1024, 4 * MB, "s.xlsx", "中央目录声明")
        monkeypatch.setattr(validation, "MIN_SHEET_RATIO_JUDGMENT_BYTES", 0)
        with pytest.raises(DataValidationError):
            validation.sheet_expansion_verdict(4 * 1024, 4 * MB, "s.xlsx", "中央目录声明")
