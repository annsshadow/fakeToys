# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L91：上传面的表格进料（A132 ③）+ 裸文件名落点（A160）+ Excel 读边的错误形状（A161）

改前实测（`Temp/l91q/upload_census.py`，py314 / 默认 `web.data_roots=["data"]`）：

| 进料 | 改前 | 改后 |
| --- | --- | --- |
| `data.csv`（真表格） | 400「数据文件不是合法 JSON（第 1 行第 1 列）」 | 200，落成 `data/data.json`，2 条 |
| `data.tsv` | 同上 400 | 200 |
| `data.jsonl`（两行） | 400「…第 2 行第 1 列」 | 200 |
| `data.xlsx` | 400「上传内容不是合法的 UTF-8 文本」 | 200 |
| `data.json` / `data.txt` / `noext` | **403「路径超出允许的数据目录范围」** | 200，落进白名单根目录 |

最后那一行是本轮撞到的第二件事：L87 把落点改成「按白名单内的相对路径用」之后，
浏览器的 `<input type=file>`（只给裸文件名，Chromium/Firefox 都剥路径）上传**新**文件
必 403 —— 也就是本仓自带的那套 UI 从改起到 L91 之前一次都没能把数据传上去。这条记
A160，本文件把它判回来。

判据分三层，各钉一件事：

1. `read_records`（转换器侧）：字节 + 声明格式 → 规范记录，与路径入口 `_read_file`
   **共用同一套解析**，不另写一份 csv/jsonl 判据；
2. `_upload_source_format` / `_table_input_lies`（路由侧判决）：显式声明优先于扩展名、
   认不出扩展名**不改**既有 JSON 那条路，以及「整份内容其实是 JSON 却按表格解析」的
   交叉检查 —— 实测缩进过的 JSON 数组改名 `.csv` 会被 `DictReader` 吃成 **251 条**键为
   `'['` 的记录，单行版则是 **0 条**，两者都能穿过 `assert_dataset_shape`；
3. 端点面：真实 multipart 走真实中间件链，断言落盘字节、落点名（表格产物一律
   `.json`，不做假容器）与拒绝时**不留半成品**。

Excel 那格另有一条独立判据（A161）：损坏 xlsx 在 pandas 侧抛 `zipfile.BadZipFile` /
`pandas.errors.OptionError`，两者都**不是** `ValueError` 子类 ⇒ 不翻译就是 500。

跑法：`python -m pytest tests/unit/test_upload_table_formats_l91.py -q --no-cov`
"""

import io
import json
import os
import pathlib
import re

import pytest
from fastapi import HTTPException

from augmentor import csv_excel_import as cei
from augmentor.converter import (
    DatasetConverter,
    INPUT_EXTENSION_FORMATS,
    UnsupportedFormatError,
    read_records,
)
from augmentor.exceptions import DataFormatError, DataLoadError
from api.routes.data import (
    _table_input_lies,
    _upload_landing_path,
    _upload_source_format,
)

ROWS = [{"instruction": "问1", "input": "", "output": "答1"},
        {"instruction": "问2", "input": "", "output": "答2"}]
CSV_BYTES = "instruction,input,output\n问1,,答1\n问2,,答2\n".encode("utf-8")
TSV_BYTES = CSV_BYTES.replace(b",", b"\t")
JSONL_BYTES = b"\n".join(json.dumps(r, ensure_ascii=False).encode("utf-8") for r in ROWS)
JSON_BYTES = json.dumps(ROWS, ensure_ascii=False).encode("utf-8")
PRETTY_JSON_BYTES = json.dumps(ROWS, ensure_ascii=False, indent=2).encode("utf-8")

needs_pandas = pytest.mark.skipif(not cei.HAS_PANDAS, reason="读 xlsx 需要 pandas 与 openpyxl")


@pytest.fixture
def excel_bytes(tmp_path):
    """一份真 xlsx 的字节：由自家写边产出，不用第二份夹具格式"""
    src = tmp_path / "src.json"
    src.write_bytes(JSON_BYTES)
    out = tmp_path / "sample.xlsx"
    DatasetConverter().convert_file(str(src), str(out),
                                    source_format="json", target_format="excel")
    return out.read_bytes()


@pytest.fixture
def api_env(tmp_path, monkeypatch):
    """把白名单收到 `tmp_path/ds`，工作目录留在 `tmp_path` ⇒ 裸名落点可判

    `tests/conftest.py` 那份 autouse 把白名单放宽成「工作目录 + 系统临时目录」，
    于是裸文件名在两种实现下都会成功，A160 这一格就测不出来。这里把白名单收窄到
    一个**子目录**（环境变量按 `(原值, 工作目录)` 缓存，换了值自然重解析，不需要
    手动清缓存），使「裸名能不能上传」重新成为一个有答案的问题。
    """
    root = tmp_path / "ds"
    root.mkdir()
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.chdir(tmp_path)
    from fastapi.testclient import TestClient
    from api.main import app

    return TestClient(app), root, tmp_path


def post_upload(http, filename, content, input_format=None):
    files = {"file": (filename, io.BytesIO(content), "application/octet-stream")}
    data = {} if input_format is None else {"input_format": input_format}
    return http.post("/api/data/upload", files=files, data=data)


class TestReadRecordsInMemory:
    """SDK 面：`read_records` 与路径入口同一套解析，且报错停在真因上"""

    @pytest.mark.parametrize("content,source,want", [
        (CSV_BYTES, "csv", ROWS),
        (TSV_BYTES, "tsv", ROWS),
        (JSONL_BYTES, "jsonl", ROWS),
    ])
    def test_three_text_containers_land_on_the_same_records(self, content, source, want):
        assert read_records(content, source, "x") == want

    def test_alpaca_schema_is_canonicalised_not_passed_through(self):
        """alpaca 的 JSONL 进料出来就是 `instruction/input/output`，与 CLI 同口径"""
        assert read_records(JSONL_BYTES, "alpaca", "x.jsonl") == ROWS

    def test_json_source_keeps_the_path_entry_no_shape_judgement(self):
        """`json` 源不做顶层判决：形状归调用方（`assert_dataset_shape`）

        这是与 `_read_file` 的 json 分支**同一条口径**，不是漏判：单条对象在路径入口
        也是原样交出去，由消费方决定接不接受。
        """
        assert read_records(b'{"instruction": "a"}', "json", "x") == {"instruction": "a"}

    def test_unknown_format_names_the_whole_read_side_list(self):
        with pytest.raises(UnsupportedFormatError, match="认不出的源格式: xml"):
            read_records(CSV_BYTES, "xml", "x.csv")

    def test_corrupt_excel_is_a_data_format_error_not_a_zip_error(self, monkeypatch):
        """A161 的**翻译本身**：pandas 抛的 `BadZipFile` 不是 `ValueError` 子类

        这一档用替身把「pandas 说了什么」钉住，因此在没有 pandas 的解释器（aug 侧
        3.13.14）上同样跑得到 —— 本轮初版直接拿真字节调 pandas，在 aug 侧成了假红：
        缺依赖时 `_read_excel_source` 在碰字节之前就回 `DataLoadError`，翻译分支根本没进。
        真凭据（真 pandas 确实抛非 ValueError）由下面那条 `@needs_pandas` 档钉。
        """
        import zipfile

        class Boom:
            @staticmethod
            def read_excel(source):
                raise zipfile.BadZipFile("File is not a zip file")

        monkeypatch.setattr(cei, "HAS_PANDAS", True)
        # `pd` 这个名字只在 pandas 装得上时才存在（`csv_excel_import` 的 try 里赋值），
        # 所以 aug 侧必须 `raising=False` —— 这恰恰是本档要能在无 pandas 环境跑的原因。
        monkeypatch.setattr(cei, "pd", Boom, raising=False)
        with pytest.raises(DataFormatError, match="坏文件.xlsx 读不出 Excel 工作表") as got:
            read_records(b"whatever", "xlsx", "坏文件.xlsx")
        assert isinstance(got.value, ValueError)

    @needs_pandas
    def test_a_real_corrupt_xlsx_reaches_that_same_translation(self):
        """真 pandas 侧：`PK\x03\x04` + 零字节确实抛非 ValueError，落进同一档 `DataFormatError`"""
        with pytest.raises(DataFormatError) as got:
            read_records(b"PK\x03\x04" + b"\x00" * 40, "xlsx", "坏文件.xlsx")
        assert got.value.__cause__ is not None

    @needs_pandas
    @pytest.mark.parametrize("spelling", ["excel", "xlsx", "xls"])
    def test_the_three_excel_spellings_are_one_format(self, excel_bytes, spelling):
        assert len(read_records(excel_bytes, spelling, "x.xlsx")) == 2

    @needs_pandas
    def test_excel_reads_from_bytes_without_a_temporary_file(self, excel_bytes):
        """内存入口的真凭据：没有第二次磁盘写，pandas 按内容而不是文件名选引擎"""
        records = read_records(excel_bytes, "excel", "")
        assert [r["instruction"] for r in records] == ["问1", "问2"]
        assert [r["output"] for r in records] == ["答1", "答2"]

    def test_missing_pandas_is_an_actionable_load_error(self, monkeypatch):
        """没有 pandas 的环境（aug 侧解释器）必须拿到「装什么」而不是 traceback"""
        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        with pytest.raises(DataLoadError, match="pip install pandas openpyxl"):
            read_records(b"whatever", "xlsx", "x.xlsx")


class TestSourceFormatDecision:
    """路由侧的格式判决：声明 > 扩展名，认不出**不改**既有那条 JSON 路"""

    @pytest.mark.parametrize("filename,want", [
        ("a.csv", "csv"), ("a.tsv", "tsv"), ("a.jsonl", "jsonl"),
        ("a.xlsx", "excel"), ("a.xls", "excel"),
        ("a.json", None), ("a.txt", None), ("a", None), ("", None),
    ])
    def test_extension_only(self, filename, want):
        assert _upload_source_format(filename, "") is want

    def test_explicit_declaration_beats_the_extension(self):
        assert _upload_source_format("a.json", "csv") == "csv"

    def test_declaring_json_keeps_the_one_existing_code_path(self):
        """声明 `json` 与不传是**同一条代码**，不留第二份 JSON 判据"""
        assert _upload_source_format("a.csv", "json") is None

    def test_bogus_declaration_fails_loud_with_the_choices(self):
        with pytest.raises(HTTPException) as got:
            _upload_source_format("a.csv", "xml")
        assert got.value.status_code == 400
        assert "alpaca" in got.value.detail and "jsonl" in got.value.detail


class TestLyingExtensionCrossCheck:
    """交叉检查：改前的「响亮拒绝」不能换成改后的「静默接受」"""

    @pytest.mark.parametrize("content,source,want", [
        (PRETTY_JSON_BYTES, "csv", True),
        (JSON_BYTES, "csv", True),
        (PRETTY_JSON_BYTES, "jsonl", True),
        (JSONL_BYTES, "csv", False),
        (JSONL_BYTES, "jsonl", False),
        (b'{"instruction": "a"}', "jsonl", False),
    ])
    def test_the_six_shapes(self, content, source, want):
        assert _table_input_lies(content, source) is want

    def test_binary_input_never_enters_the_text_sniff(self):
        """Excel 不参与嗅探：对它做 `json.loads` 只会把解码错混进判决"""
        assert _table_input_lies(b"PK\x03\x04binary", "excel") is False

    def test_undecodable_bytes_are_not_a_lie(self):
        assert _table_input_lies(b"\xff\xfe\x00csv\xff", "csv") is False


class TestLandingPath:
    """落点三件事：裸名进根目录、显式路径照旧 honored、表格产物换 `.json` 名字"""

    def test_bare_filename_lands_in_the_root_not_the_working_directory(self, api_env):
        _, root, cwd = api_env
        path = _upload_landing_path("l91bare.json", None)
        assert path == (root / "l91bare.json").resolve()
        assert not (cwd / "l91bare.json").exists()

    def test_a_name_with_a_directory_component_is_not_treated_as_bare(self, api_env):
        """带目录的写法**不**走本轮那条补根前缀的路：仍由 `resolve_within_roots` 解释

        这间取证房里白名单只有 `tmp/ds`、工作目录是 `tmp`，所以 `sub/x.json` 按既有
        口径落到 `tmp/sub/x.json` ⇒ 403。断言的是「本轮没有改动那一支」，不是「子目录
        写法坏了」：默认部署（`web.data_roots=["data"]`、cwd 在项目根）下
        `data/sub/x.json` 仍然成功，L87 挣来的那一格由
        `tests/unit/test_upload_ceiling_l87.py::TestUploadPathIsHonoured` 继续钉住。
        """
        with pytest.raises(HTTPException) as got:
            _upload_landing_path("sub/x.json", None)
        assert got.value.status_code == 403

    @pytest.mark.parametrize("filename,source,want", [
        ("t.xlsx", "excel", "t.json"), ("t.csv", "csv", "t.json"),
        ("t.json", "csv", "t.json"), ("noext", "tsv", "noext.json"),
    ])
    def test_table_products_never_keep_a_foreign_container_name(self, api_env,
                                                                 filename, source, want):
        """假容器判据与写边同源：JSON 字节不写进 `.xlsx` / `.csv` 的名字里"""
        _, root, _ = api_env
        assert _upload_landing_path(filename, source).name == want

    def test_json_input_keeps_its_name_untouched(self, api_env):
        """只有「源是表格」才改名；`data.txt` 装 JSON 仍是 `data.txt`（改前语义）"""
        _, root, _ = api_env
        assert _upload_landing_path("data.txt", None) == (root / "data.txt").resolve()

    def test_parent_escape_is_still_rejected(self, api_env):
        """`..` 那一支一字未动（裸名补根前缀发生在进 `resolve_within_roots` 之前）"""
        with pytest.raises(HTTPException) as got:
            _upload_landing_path("../etc/passwd.json", None)
        assert got.value.status_code == 400


class TestUploadEndpointTables:
    """端点面：真实 multipart、真实中间件链，成功看落盘字节、失败看不留半成品"""

    def test_csv_upload_becomes_a_json_dataset(self, api_env):
        http, root, _ = api_env
        response = post_upload(http, "train.csv", CSV_BYTES)
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["count"] == 2
        assert payload["path"].endswith("train.json")
        assert json.loads((root / "train.json").read_text(encoding="utf-8")) == ROWS

    def test_jsonl_upload_no_longer_needs_a_wrapper_array(self, api_env):
        http, root, _ = api_env
        response = post_upload(http, "train.jsonl", JSONL_BYTES)
        assert response.status_code == 200, response.text
        assert json.loads((root / "train.json").read_text(encoding="utf-8")) == ROWS

    @needs_pandas
    def test_xlsx_upload_reads_the_whole_sheet(self, api_env, excel_bytes):
        http, root, _ = api_env
        response = post_upload(http, "sheet.xlsx", excel_bytes)
        assert response.status_code == 200, response.text
        saved = json.loads((root / "sheet.json").read_text(encoding="utf-8"))
        assert [r["instruction"] for r in saved] == ["问1", "问2"]

    def test_declared_format_wins_over_an_unrecognised_extension(self, api_env):
        http, root, _ = api_env
        response = post_upload(http, "mystery.dat", CSV_BYTES, input_format="csv")
        assert response.status_code == 200, response.text
        assert (root / "mystery.json").is_file()

    def test_corrupt_xlsx_uploads_as_400_not_500(self, api_env):
        """A161 的产品面：**两个解释器都判**，只判状态码与磁盘

        无 pandas 时 `_read_excel_source` 在碰字节之前就先报「装什么」（`DataLoadError`，
        同为 `ValueError` 子类 ⇒ 仍 **400**），诊断不如 pandas 侧精确，但「把客户端错误
        说成服务端故障」和「落一份半成品」两件事在两侧都不发生 —— 那才是本条要守的。
        文案随环境变，所以这条不与文案耦合：翻译后的文案由替身档
        `test_corrupt_excel_is_a_data_format_error_not_a_zip_error`（两侧都跑）钉住。
        """
        http, root, _ = api_env
        response = post_upload(http, "broken.xlsx", b"PK\x03\x04" + b"\x00" * 40)
        assert response.status_code == 400, response.text
        assert list(root.glob("*")) == []

    @pytest.mark.parametrize("filename,content,want", [
        ("lie.csv", PRETTY_JSON_BYTES, "整份是一份合法 JSON"),
        ("lie.jsonl", PRETTY_JSON_BYTES, "整份是一份合法 JSON"),
        ("header-only.csv", b"instruction,input,output\n", "0 条记录"),
    ])
    def test_the_two_new_rejections_leave_no_file(self, api_env, filename, content, want):
        http, root, _ = api_env
        response = post_upload(http, filename, content)
        assert response.status_code == 400, response.text
        assert want in response.json()["detail"]
        assert list(root.glob("*")) == []

    def test_bogus_input_format_is_rejected_before_any_body_read(self, api_env):
        http, root, _ = api_env
        response = post_upload(http, "x.csv", CSV_BYTES, input_format="xml")
        assert response.status_code == 400
        assert "读边认得的格式" in response.json()["detail"]
        assert list(root.glob("*")) == []

    @pytest.mark.parametrize("filename,content", [
        ("ok.json", JSON_BYTES), ("noext", JSON_BYTES), ("plain.txt", JSON_BYTES),
    ])
    def test_the_existing_json_face_is_unchanged(self, api_env, filename, content):
        """JSON 那条路一字未改：包括「无扩展名」与 `.txt`，也包括名字不被换成 `.json`"""
        http, root, _ = api_env
        response = post_upload(http, filename, content)
        assert response.status_code == 200, response.text
        assert json.loads((root / filename).read_text(encoding="utf-8")) == ROWS

    def test_empty_json_array_is_still_a_legal_dataset(self, api_env):
        """`[]` 是「零条数据」不是「形态不对」（L87 口径），本轮**没有**把它判死

        表格那侧的「0 条」判据与此不冲突：那一条只在「字节非空却一条也没读出来」时
        才响，量的是解析有没有吃到东西，不是数据集有没有内容。
        """
        http, root, _ = api_env
        response = post_upload(http, "empty.json", b"[]")
        assert response.status_code == 200, response.text
        assert response.json()["count"] == 0
        assert json.loads((root / "empty.json").read_text(encoding="utf-8")) == []

    def test_parent_escape_still_400_and_absolute_escape_still_403(self, api_env):
        """白名单闸一字未松（L87 的两条回归）

        写法用 `as_posix()`：带反斜杠的 multipart `filename` 在解析时会被引号串的
        转义规则改掉（实测 `"C:\tmp-l91.json"` 到服务端成了 `C:%09mp-l91.json`，
        即 `\t` 被吃成一个 TAB，变成一个**裸文件名**而不再是绝对路径）。那种形状
        在两种平台上都还落在白名单里，测不到「越界绝对路径 403」这一格，故正斜杠写。
        """
        http, root, _ = api_env
        outside = pathlib.Path(os.path.abspath(os.sep)) / "l91-outside.json"
        assert post_upload(http, "../evil.json", JSON_BYTES).status_code == 400
        assert post_upload(http, outside.as_posix(), JSON_BYTES).status_code == 403
        assert list(root.glob("*")) == []


#: 上传控件所在页面（`web/` 是仓库内的前端源码，发行包里可能只有构建产物）
PAGE_TSX = pathlib.Path(__file__).resolve().parents[2] / "web" / "src" / "pages" / "DataManagement.tsx"
ACCEPT_RE = re.compile(r'accept="([^"]*)"')

needs_web_src = pytest.mark.skipif(
    not PAGE_TSX.is_file(), reason="前端源码不在场（只带构建产物的安装没有 web/）"
)


@needs_web_src
class TestUploadAcceptFace:
    """产品面可达性：后端读边认的扩展名，UI 的选文件框必须**选得到**

    这一格是本轮实测里最容易漏的那一层：端点做到「认 `.csv`」只是能力的一半，
    `DataManagement` 原先写 `accept=".json"`，浏览器直接把表格文件灰掉 ⇒ 用户看不见
    新能力（改前实测该属性只有 `.json` 一项）。

    判据**双向**，因为漂移会朝两个方向发生：
    - 读边有、accept 没有 ⇒ 服务端收得到而 UI 选不到（本轮改前的形状）；
    - accept 有、读边没有 ⇒ 用户选得到而服务端必 400，等于骗人。

    权威在这里是**直接 import 的常数** `INPUT_EXTENSION_FORMATS`，不是在 TS 里正则抠
    Python。本档曾在 vitest 侧写跨语言守卫，实测那边 `tsc --noEmit` 过不去（web 的
    `@types` 里没有 node，`node:fs` 无声明），为一条断言去动依赖表不值当，故搬到
    Python 侧；这也让守卫每轮全量都跑得到，而不是只在 `npm test` 时跑。

    **局限如实说**：文本扫描不证明 antd 的 `Upload` 把 `accept` 透传给了原生 input，
    也不证明选中的文件真能读通（那是上面端点档的事）。
    """

    @staticmethod
    def _accepted() -> list:
        match = ACCEPT_RE.search(PAGE_TSX.read_text(encoding="utf-8"))
        assert match, "DataManagement 的 Upload 没有 accept 属性 ⇒ 判据要跟着换形状"
        return [item.strip() for item in match.group(1).split(",") if item.strip()]

    def test_the_accept_list_is_scanned_and_not_vacuous(self):
        """扫描本身要有效：只抠到一项就说明正则在空转"""
        accepted = self._accepted()
        assert len(accepted) >= 2, accepted
        assert ".json" in accepted, "JSON 那档恒要在场：既有客户端不能因扩围而选不到"

    def test_every_extension_the_read_side_accepts_can_be_picked(self):
        accepted = self._accepted()
        missing = [ext for ext in INPUT_EXTENSION_FORMATS if ext not in accepted]
        assert not missing, f"读边认这些扩展名，但上传控件选不到: {missing}"

    def test_nothing_is_offered_that_the_read_side_cannot_parse(self):
        accepted = self._accepted()
        extra = [ext for ext in accepted if ext not in INPUT_EXTENSION_FORMATS]
        assert not extra, f"上传控件放开这些扩展名，但读边认不出（选了必 400）: {extra}"
