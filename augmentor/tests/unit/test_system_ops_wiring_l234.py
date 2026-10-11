# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L234（A279 第九批 / A313）：系统那一族五条接进页面 —— 清单同源、能力有差、口径如实

**这一轮接的五条**：`POST /api/system/monitor` 与 `POST /api/system/auto-test` 进系统状态页
第五张卡（都只读、都不挂门禁），`POST /api/system/migrate` 与 `POST /api/system/stream` 进
第六张卡（都写盘、都挂门禁），`POST /api/system/validate-config` 进配置管理页新卡。
消费面因此从 67 涨到 **72**、缺口从 8 收到 **3**（那三个数由 l224 那把尺子钉，本文件不另钉
一份）；l233 的「挂门禁但没接线」那一格同步从 3 收到 **1**（只剩 translate）。

**接线时量出来的一处产品缺陷，本轮当场收（A313）**：`stream` 的 `operation` 在两端不是同一份
清单 —— CLI 的 `--operation` choices 写的是 `quality / dedup / clean / export`，而端点认的是
`quality / dedup / clean / none`。两头各错一半：`export` 被 argparse 收下、落到 handler 的
`else` 分支，而那个分支只是 `lambda items: items`（**原样搬运**，名字承诺的导出从来没发生过），
连既有测试都把这句谎话钉成了事实（`test_stream_export_passthrough` 的 docstring 逐字写着
「export 操作为直通处理」）；反过来 HTTP 侧合法的 `none` 在 CLI 里被 argparse 直接拒（退出码 2）。
修法是把 CLI 那份字面改成与端点同一份，并由本文件 `TestTheClosedListsAreOnePerFace` 做三向
对账。**没有新插 Python 常数**：把权威搬进 `augmentor/streaming.py` 会在 `WRITER_FORMATS`
之后插行，平移账本 A147 那条活引用 `augmentor/streaming.py:274`（`read_chunks`）；仓内既有
惯例（l226 / l175 / l189）也是「CLI 留字面、守卫对账」，故沿用。

五面判据，各对应一种真实失效形状：

1. **清单同源**（`TestTheClosedListsAreOnePerFace`）：`operation` 三向逐项相等（页面下拉 /
   CLI choices / 端点实收），`output_format` 与迁移规则各自对齐 Python 权威
   （`WRITER_FORMATS` / `BUILTIN_MIGRATION_RULE_IDS`）。这一面正是 A313 那个缺陷的棘轮。
2. **能力真有差**（`TestTheFourOperationsActuallyDiffer`）：四档不是四个标签，是四种不同结果
   —— quality 丢 3 条、dedup 丢 2 条、clean **一条不丢但改写内容**（实测把全角标点换成半角）、
   none 原样。接线只让指标涨而能力不涨时，这一面红。
3. **哨兵是躲判决的**（`TestTheSentinelsDodgeVerdicts`）：服务层那六处 `|| undefined` 逐个
   有对应的 400 —— 空串 `operation` / 空串 `suite` / 空串 `path` / `chunk_size` 0 与 -1 /
   `parquet`，全是 **400 且 detail 是字符串**（前端 `apiErrorDetail` 的主支）。删掉任一处
   哨兵，页面就会把「服务端默认」那一档换成一句判决。
4. **口径如实**（`TestTheHonestProse`）：页面上写着那几条量出来的脾气 ——`processed` 恒等于
   读入条数（不是「被改动过」）、规则留空等于三条全用（不是「不迁移」）、metrics 可以是空
   对象、告警 0 不等于质量合格、配置校验不通过也回 200、行号那一格恒为空、**合法也可能带着
   警告**。最后一条是接线途中量出来的：配置卡那句 success 文案原写「按服务端那套规则没查出
   问题」，而最小合法配置回 `is_valid: true` 且带 1 条警告、出厂那份带 5 条 ⇒ 文案已改。
5. **真链路形状**（`TestTheRealLinkShapes`）：回执键集、五条测试的 id 与顺序、空数据集那一档、
   以及**三只只读端点在门禁升起时仍然 200**（它们是 POST，l233 那条「没有 GET 被门禁」管不到）。

跑法：`python -m pytest tests/unit/test_system_ops_wiring_l234.py -q --no-cov`
"""

import io
import json
import pathlib
import re
from contextlib import redirect_stderr

import pytest

from augmentor.cli.parser import build_parser
from augmentor.migration import BUILTIN_MIGRATION_RULE_IDS
from augmentor.streaming import WRITER_FORMATS
from tests.unit.test_wired_dropdown_choices_l226 import needs_web_src, strip_comments

ROOT = pathlib.Path(__file__).resolve().parents[2]
SRC = ROOT / "web" / "src"
API_TS = SRC / "services" / "api.ts"
SYSTEM_TSX = SRC / "pages" / "System.tsx"
SETTINGS_TSX = SRC / "pages" / "Settings.tsx"

#: 端点认的四档（A313 之后 CLI 与页面也是这四档，顺序也相同）
STREAM_OPERATIONS = ["quality", "dedup", "clean", "none"]
#: 改前 CLI 独有的那一档：端点是 400，CLI 现在也是退出码 2
RETIRED = "export"

#: 夹具：5 条，含一组完全重复、一条过短、一条带全角标点与 HTML 标签
#: 这一份的读数是新守卫「能力真有差」那一面的分母，改它就要重量四档
ITEMS = [
    {"instruction": "问题 0 请回答得详细一些，至少二十个字以上。", "input": "",
     "output": "这是第 0 条答案，内容足够长，用于通过质量检查。"},
    {"instruction": "问题 0 请回答得详细一些，至少二十个字以上。", "input": "",
     "output": "这是第 0 条答案，内容足够长，用于通过质量检查。"},
    {"instruction": "短", "input": "", "output": "短"},
    {"instruction": "  问题 3 带首尾空白  ", "input": "",
     "output": "  <p>这是第 3 条答案，带一点 HTML 标签与首尾空白。</p>  "},
    {"instruction": "问题 4 请回答得详细一些，至少二十个字以上。", "input": "",
     "output": "这是第 4 条答案，内容足够长，用于通过质量检查。"},
]

#: 四档在这份夹具上的实测写出条数（仓外探针 `l234_counts.py`，L234 跑完即删）
OUTPUT_BY_OPERATION = {"none": 5, "quality": 2, "dedup": 3, "clean": 5}

#: 内置 default 套件的五条测试，id 与顺序都实测固定
FIVE_TESTS = ["format_check", "field_check", "quality_check", "diversity_check", "duplicate_check"]

#: `POST /api/system/validate-config` 显式路径为空串时那一句（400）
EMPTY_PATH_DETAIL = "文件路径 不能为空"

#: 一份**最小**的合法配置：只要 `models.default` 在就 is_valid 为 true（探针量到）
MINIMAL_VALID_YAML = "models:\n  default: ernie\n"

#: 那份合法配置回来时**仍带着的一条警告**（逐字来自探针）——「合法」不等于「没有问题清单」
MINIMAL_VALID_WARNING = "模型 'ernie' 未配置。可用模型: []"


@pytest.fixture
def http(tmp_path, monkeypatch):
    """白名单根指向临时目录：这一族有四支真写盘，产物不许落进仓库

    `reset_pipeline()` 前后各调一次：`api/deps.py` 把管道缓存在模块级 `_pipeline` 上，
    而 stream 那一支的 quality 阈值取自 `get_pipeline().config` ⇒ 不重置就会把别的测试
    留下的配置（连同阈值）漏进这一份夹具的读数里。
    """
    from api.deps import reset_pipeline

    root = tmp_path / "ds"
    root.mkdir()
    (root / "mixed.json").write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    (root / "empty.json").write_text("[]", encoding="utf-8")
    (root / "minimal.yaml").write_text(MINIMAL_VALID_YAML, encoding="utf-8")
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(root))
    monkeypatch.setenv("AUGMENTOR_API_KEY", "")
    monkeypatch.chdir(tmp_path)
    reset_pipeline()
    from fastapi.testclient import TestClient

    from api.main import app

    yield TestClient(app), root, monkeypatch
    reset_pipeline()


def _stream(client, operation=None, **extra):
    body = {"input_file": "mixed.json", "output_file": f"out_{extra.get('tag', 'x')}.jsonl"}
    body.pop("tag", None)
    if operation is not None:
        body["operation"] = operation
    body.update({k: v for k, v in extra.items() if k != "tag"})
    return client.post("/api/system/stream", json=body)


def cli_stream_choices():
    """从 parser 上真读 `stream --operation` 的 choices（不抄字面）"""
    parser = build_parser()
    for action in parser._subparsers._group_actions:  # noqa: SLF001 —— argparse 没有公开 accessor
        for name, sub in getattr(action, "choices", {}).items():
            if name != "stream":
                continue
            for sub_action in sub._actions:  # noqa: SLF001
                if "--operation" in sub_action.option_strings:
                    return list(sub_action.choices)
    raise AssertionError("parser 里找不到 stream --operation ⇒ 判据要跟着换形状")


def _code(path: pathlib.Path) -> str:
    assert path.is_file(), f"{path.name} 不在场 ⇒ 本文件的判据全部失效"
    return strip_comments(path.read_text(encoding="utf-8"))


#: 与 l226 那把同形，但**认空串那一档**（`'([^']*)'` 而不是 `'([^']+)'`）
_VALUE_RE = re.compile(r"value:\s*'([^']*)'")
_LABEL_RE = re.compile(r"label:\s*'([^']+)'")


def page_options(path: pathlib.Path, const_name: str) -> list:
    """抠出 `[{ value: 'x', label: '…' }, …]` 的 `(value, label)` 对，保持原序

    **不复用 l226 的 `options_from`**：那把的正则要求 value 至少一个字符，而这一族有两个
    下拉（`operation` 与 `output_format`）以**空串**当「页面不送、由服务端取默认」的哨兵
    （空串直送服务端是 400，见第 3 面）⇒ 复用会在「values 与 labels 数目不等」上判假红：
    首跑实测 STREAM_FORMAT_OPTIONS 量到 2 个 value 对 3 个 label。那把尺子的范围与它的禁令
    面同族（它管的两个下拉没有哨兵档），所以不去改它，这里另写一份。
    """
    block = re.search(rf"const {const_name} = \[(.*?)\]", strip_comments(
        path.read_text(encoding="utf-8")), re.S)
    assert block, f"{path.name} 里没有 {const_name} 常量 ⇒ 判据要跟着换形状"
    values = _VALUE_RE.findall(block.group(1))
    labels = _LABEL_RE.findall(block.group(1))
    assert len(values) == len(labels), f"{const_name} 里有项没写 label: {block.group(1)}"
    return list(zip(values, labels))


class TestTheClosedListsAreOnePerFace:
    """第 1 面：A313 的棘轮 —— 同一份清单，三个面逐项相等（连顺序一起）"""

    @needs_web_src
    def test_the_page_dropdown_is_those_four_in_that_order(self):
        values = [v for v, _ in page_options(SYSTEM_TSX, "STREAM_OPERATION_OPTIONS")]
        assert values == [""] + STREAM_OPERATIONS, (
            f"页面下拉现量 {values}；空串那一档是「不送、由服务端取默认」的哨兵，必须留在首位")

    def test_the_cli_choices_are_the_same_four_in_the_same_order(self):
        assert cli_stream_choices() == STREAM_OPERATIONS, cli_stream_choices()

    def test_the_endpoint_takes_exactly_those_four(self, http):
        client, _, _ = http
        for operation in STREAM_OPERATIONS:
            r = _stream(client, operation, tag=operation)
            assert r.status_code == 200, f"{operation} 不被端点认下：{r.status_code} {r.text[:120]}"

    def test_the_retired_export_is_rejected_by_both_ends(self, http):
        """A313 的判决面：改前它在 CLI 是 200（静默原样搬运）、在 HTTP 是 400 —— 现在两头都拒"""
        client, _, _ = http
        r = _stream(client, RETIRED, tag=RETIRED)
        assert r.status_code == 400
        detail = r.json()["detail"]
        assert isinstance(detail, str), f"detail 不是字符串 ⇒ 前端 apiErrorDetail 的主支接不住：{detail}"
        assert RETIRED in detail and "quality / dedup / clean / none" in detail, detail

        parser = build_parser()
        err = io.StringIO()
        with redirect_stderr(err):
            with pytest.raises(SystemExit) as exc:
                parser.parse_args(["stream", "--input", "a.json", "--output", "b.json",
                                   "--operation", RETIRED])
        assert exc.value.code == 2, exc.value.code

    def test_the_cli_now_accepts_none(self):
        """反过来那一半：HTTP 侧一直合法的 `none`，改前在 CLI 是退出码 2"""
        ns = build_parser().parse_args(
            ["stream", "--input", "a.json", "--output", "b.json", "--operation", "none"])
        assert ns.operation == "none"

    @needs_web_src
    def test_the_format_dropdown_matches_the_writer_formats_authority(self):
        values = [v for v, _ in page_options(SYSTEM_TSX, "STREAM_FORMAT_OPTIONS") if v]
        assert values == list(WRITER_FORMATS), (
            f"页面 {values} 对 Python 权威 {list(WRITER_FORMATS)}（augmentor/streaming.py）")

    @needs_web_src
    def test_the_rule_dropdown_matches_the_builtin_rule_ids(self):
        values = [v for v, _ in page_options(SYSTEM_TSX, "MIGRATION_RULE_OPTIONS")]
        assert values == list(BUILTIN_MIGRATION_RULE_IDS), (
            f"页面 {values} 对 Python 权威 {list(BUILTIN_MIGRATION_RULE_IDS)}（augmentor/migration.py）")

    @needs_web_src
    def test_no_fake_suite_dropdown(self):
        """`suite` 只有一个合法值（default），所以页面**不给**下拉 —— 只有一项的选择器是噪音

        而空串是 400、null 与省略才走默认 ⇒ 那一格连自由输入都不能给。判据钉两件事：
        没有那个 aria-label，且调用处不送第二个实参。
        """
        page = _code(SYSTEM_TSX)
        assert 'aria-label="测试套件"' not in page
        assert "runDatasetAutoTest(qaFile)" in page, "自测那一支的调用形状变了 ⇒ 判据要跟着换"


class TestTheFourOperationsActuallyDiffer:
    """第 2 面：接线不能只让指标涨。四档必须真给出四种不同结果"""

    def test_quality_and_dedup_drop_different_counts(self, http):
        client, _, _ = http
        seen = {}
        for operation in ("none", "quality", "dedup", "clean"):
            r = _stream(client, operation, tag=operation)
            seen[operation] = r.json()["total_output"]
        assert seen == OUTPUT_BY_OPERATION, (
            f"四档的写出条数漂了（夹具没动的话就是处置逻辑变了）：实得 {seen}，定标 {OUTPUT_BY_OPERATION}")
        assert len(set(seen.values())) >= 3, f"四档塌成同一种结果 ⇒ 下拉是装饰：{seen}"

    def test_clean_drops_nothing_but_rewrites_content(self, http):
        """clean 的取舍在**内容**上不在条数上：5→5，但产物与输入不是同一份字节

        这一格最容易接线接错 —— 只看条数会把它报成「什么都没做」（none 也是 5→5）。
        实测它把全角逗号与句号换成半角、剥掉首尾空白，所以判据同时钉「条数相同」与「字节不同」。
        还有一处实测的**非对称**：改写只落在 `instruction` 上，`output` 一字未动（第 4 条里
        那些 HTML 标签与首尾空白原样留着）。页面上那句口径就是照这一格写的，所以两半都钉住。
        """
        client, root, _ = http
        r = _stream(client, "clean", tag="clean")
        assert r.json()["total_output"] == 5
        produced = (root / "out_clean.jsonl").read_text(encoding="utf-8")
        source = json.dumps(ITEMS, ensure_ascii=False)
        assert produced.strip() != source.strip(), "clean 一份字节都没改 ⇒ 那一档等于 none"
        rows = [json.loads(line) for line in produced.splitlines()]
        first = rows[0]
        assert "，" not in first["instruction"] and "。" not in first["instruction"], (
            f"实测 clean 会把全角标点换成半角，现在没换：{first['instruction']}")
        assert rows[3]["instruction"] == "问题 3 带首尾空白", rows[3]["instruction"]
        assert [row["output"] for row in rows] == [item["output"] for item in ITEMS], (
            "clean 连 output 一起改了 ⇒ 页面上「改写只落在 instruction」那句口径成了假话")

    def test_processed_counts_what_was_read_not_what_changed(self, http):
        """四档的 `processed` 全都等于读入条数 ⇒ 页面上不能拿它当取舍读数"""
        client, _, _ = http
        for operation in STREAM_OPERATIONS:
            body = _stream(client, operation, tag=operation).json()
            assert body["processed"] == body["total_input"] == len(ITEMS), (operation, body)

    def test_migrate_renames_the_fields_and_reports_all_three_rules(self, http):
        """默认那三条规则是**破坏性**的：instruction → question、output → answer"""
        client, root, _ = http
        r = client.post("/api/system/migrate",
                        json={"input_file": "mixed.json", "output_file": "mig.json"})
        assert r.status_code == 200, r.text[:200]
        body = r.json()
        assert body["rules_applied"] == list(BUILTIN_MIGRATION_RULE_IDS)
        assert body["total_items"] == body["migrated_items"] == len(ITEMS)
        assert body["failed_items"] == 0 and body["errors"] == []
        first = json.loads((root / "mig.json").read_text(encoding="utf-8"))[0]
        assert "question" in first and "answer" in first, first
        assert "instruction" not in first and "output" not in first, first

    def test_empty_rules_means_all_three_not_nothing(self, http):
        """`rules: []` 与省略**等价**（三条全用）⇒ 页面上不能有「不迁移」那一档"""
        client, _, _ = http
        omitted = client.post("/api/system/migrate", json={
            "input_file": "mixed.json", "output_file": "mig_a.json"}).json()
        empty = client.post("/api/system/migrate", json={
            "input_file": "mixed.json", "output_file": "mig_b.json", "rules": []}).json()
        assert omitted["rules_applied"] == empty["rules_applied"] == list(BUILTIN_MIGRATION_RULE_IDS)

    def test_an_unknown_rule_is_400_and_is_judged_before_the_file(self, http):
        """规则名写错时连输入文件存不存在都不看（判据顺序实测）"""
        client, _, _ = http
        r = client.post("/api/system/migrate", json={
            "input_file": "no_such_file.json", "output_file": "mig_c.json", "rules": ["nope"]})
        assert r.status_code == 400, r.status_code
        assert "未知的迁移规则: nope" in r.json()["detail"]


class TestTheSentinelsDodgeVerdicts:
    """第 3 面：服务层那几处 `|| undefined` 逐个有对应的 400 —— 不是形状卫生，是躲判决"""

    def test_an_empty_operation_is_400_not_a_default(self, http):
        client, _, _ = http
        r = _stream(client, "", tag="empty_op")
        assert r.status_code == 400
        assert "不支持的 operation" in r.json()["detail"]

    def test_an_empty_suite_is_400_not_a_default(self, http):
        """服务端判的是 `suite is not None`，空串算「给了一个不认识的名字」"""
        client, _, _ = http
        r = client.post("/api/system/auto-test", json={"input_file": "mixed.json", "suite": ""})
        assert r.status_code == 400
        assert "未知的测试套件" in r.json()["detail"]

    def test_a_null_suite_is_the_default(self, http):
        """对照组：null 与省略才走默认 ⇒ 哨兵必须丢成 undefined 而不是空串"""
        client, _, _ = http
        for body in ({"input_file": "mixed.json"},
                     {"input_file": "mixed.json", "suite": None}):
            r = client.post("/api/system/auto-test", json=body)
            assert r.status_code == 200, r.text[:120]
            assert r.json()["name"] == "default"

    def test_an_empty_config_path_is_400(self, http):
        client, _, _ = http
        r = client.post("/api/system/validate-config", json={"path": ""})
        assert r.status_code == 400
        assert r.json()["detail"] == EMPTY_PATH_DETAIL

    def test_zero_and_negative_chunk_size_are_400(self, http):
        client, _, _ = http
        for size in (0, -1):
            r = _stream(client, "none", tag=f"cs{size}", chunk_size=size)
            assert r.status_code == 400, (size, r.text[:120])
            assert "chunk_size 必须是不小于 1 的整数" in r.json()["detail"]

    def test_parquet_is_400_with_a_string_detail(self, http):
        client, _, _ = http
        r = _stream(client, "none", tag="pq", output_format="parquet")
        assert r.status_code == 400
        detail = r.json()["detail"]
        assert isinstance(detail, str) and "json / jsonl" in detail, detail

    @needs_web_src
    def test_the_service_layer_drops_every_empty_sentinel(self):
        """源码面：六处哨兵一处都不能少（少一处，页面那一档就变成一句 400）"""
        code = _code(API_TS)
        for shape in (
            "suite: opts.suite || undefined",
            "rules: opts.rules && opts.rules.length > 0 ? opts.rules : undefined",
            "operation: opts.operation || undefined",
            "chunk_size: opts.chunkSize && opts.chunkSize > 0 ? opts.chunkSize : undefined",
            "output_format: opts.outputFormat || undefined",
            "path: opts.path || undefined",
        ):
            assert shape in code, f"服务层少了这一处哨兵：{shape}"


class TestTheHonestProse:
    """第 4 面：口径。这几句都是量出来的脾气，页面上不写就等于让用户自己撞"""

    @needs_web_src
    def test_the_system_page_says_processed_is_not_a_drop_count(self):
        page = SYSTEM_TSX.read_text(encoding="utf-8")
        assert "processed" in page and "都等于读入条数" in page

    @needs_web_src
    def test_the_system_page_says_empty_rules_means_all_three(self):
        page = SYSTEM_TSX.read_text(encoding="utf-8")
        assert "留空等于三条全用" in page and "不是「不迁移」" in page

    @needs_web_src
    def test_the_system_page_says_clean_rewrites_content(self):
        """本轮新量到的一格：clean 一条不丢，改写发生在内容里"""
        page = SYSTEM_TSX.read_text(encoding="utf-8")
        assert "一条不丢" in page and "改写内容" in page

    @needs_web_src
    def test_the_system_page_says_metrics_can_be_an_empty_object(self):
        page = SYSTEM_TSX.read_text(encoding="utf-8")
        assert "空对象" in page and "不是 0" not in page.split("空对象")[0][-40:]

    @needs_web_src
    def test_the_system_page_says_zero_alerts_is_not_a_pass(self):
        page = SYSTEM_TSX.read_text(encoding="utf-8")
        assert "不是「质量合格」" in page

    @needs_web_src
    def test_the_settings_page_says_a_failed_validation_is_still_200(self):
        page = SETTINGS_TSX.read_text(encoding="utf-8")
        assert "校验不通过也回 HTTP 200" in page and "is_valid" in page

    @needs_web_src
    def test_the_settings_page_says_the_line_number_is_always_empty(self):
        page = SETTINGS_TSX.read_text(encoding="utf-8")
        assert "恒为空" in page and "跳到出错行" in page

    @needs_web_src
    def test_the_settings_page_says_valid_can_still_carry_warnings(self):
        """接线时量出来的第二句假话：合法那份也带着警告（最小合法配置 1 条、出厂那份 5 条）"""
        page = SETTINGS_TSX.read_text(encoding="utf-8")
        assert "不等于没有问题清单" in page
        assert "按服务端那套规则没查出问题" not in page, "那句已被实测证伪，不许留在页面上"


class TestTheRealLinkShapes:
    """第 5 面：回执形状与门禁面，逐个量（不推）"""

    def test_monitor_snapshot_has_exactly_those_four_keys(self, http):
        client, _, _ = http
        body = client.post("/api/system/monitor", json={"input_file": "mixed.json"}).json()
        assert set(body) == {"snapshot_id", "timestamp", "metrics", "alerts"}
        assert set(body["metrics"]) == {"completeness", "diversity", "consistency"}
        assert body["alerts"] == [], "这份夹具不该触阈值；触了说明阈值或夹具漂了"

    def test_an_empty_dataset_gives_an_empty_metrics_object(self, http):
        """空数组那一档：metrics 是**空对象**，不是三个 0 ⇒ 页面按「键在不在」渲染"""
        client, _, _ = http
        body = client.post("/api/system/monitor", json={"input_file": "empty.json"}).json()
        assert body["metrics"] == {}, body["metrics"]

    def test_auto_test_is_five_named_tests_in_that_order(self, http):
        client, _, _ = http
        body = client.post("/api/system/auto-test", json={"input_file": "mixed.json"}).json()
        assert body["total_tests"] == 5
        assert [t["test_id"] for t in body["results"]] == FIVE_TESTS
        assert body["passed_tests"] + body["failed_tests"] == body["total_tests"]
        assert body["pass_rate"] == pytest.approx(body["passed_tests"] / body["total_tests"])
        assert set(body["results"][0]) == {
            "test_id", "test_name", "passed", "message", "execution_time_ms", "timestamp"}

    def test_a_failed_validation_is_still_http_200(self, http):
        """这一支的失败**进不了 catch**：判决只在 body 上"""
        client, root, _ = http
        (root / "broken.yaml").write_text("models:\n  default: 123\nweb:\n  data_roots: 3\n",
                                          encoding="utf-8")
        r = client.post("/api/system/validate-config", json={"path": "broken.yaml"})
        assert r.status_code == 200
        body = r.json()
        assert body["is_valid"] is False
        assert body["summary"]["errors"] == len(body["errors"]) >= 1
        assert all(issue["line"] is None for issue in body["errors"]), body["errors"]

    def test_a_valid_config_can_still_carry_warnings(self, http):
        """is_valid 为 true 的那一档**不是**「一条问题都没有」

        页面上那句 success 文案原先写着「按服务端那套规则没查出问题」，接线时量出来是假的：
        这份最小合法配置回 `is_valid: true`、`errors: []`，却带着 1 条 warning（`models.default`
        指的那个名字没有对应配置）；出厂那份 config.yaml 同样 true 且带 5 条「环境变量未设置」。
        文案已改成「合法也不等于没有问题清单」，这一支就是它的证据。
        """
        client, _, _ = http
        r = client.post("/api/system/validate-config", json={"path": "minimal.yaml"})
        assert r.status_code == 200, r.text[:200]
        body = r.json()
        assert body["is_valid"] is True and body["errors"] == []
        assert body["summary"] == {"errors": 0, "warnings": 1, "info": 0}, body["summary"]
        assert [w["message"] for w in body["warnings"]] == [MINIMAL_VALID_WARNING]
        assert body["warnings"][0]["path"] == "models.default"
        assert body["warnings"][0]["line"] is None

    def test_the_services_own_config_is_also_200_when_missing(self, http):
        """省略 path 那一档：临时 cwd 下没有 config.yaml，仍是 200 + 不合法，不是 404"""
        client, _, _ = http
        r = client.post("/api/system/validate-config", json={})
        assert r.status_code == 200
        body = r.json()
        assert body["is_valid"] is False
        assert any("文件不存在" in e["message"] for e in body["errors"]), body["errors"]

    def test_migrate_and_stream_are_gated_while_the_three_reads_are_not(self, http):
        """门禁升起时：两支写盘的 401，三只只读的（都是 POST）仍然 200

        l233 那条「24 条门禁里没有一条 GET」管不到这一格 —— 这三只只读端点是 POST，
        所以「只读 ⇒ 不被门禁」在这里不成立，得逐条量。
        """
        client, _, monkeypatch = http
        monkeypatch.setenv("AUGMENTOR_API_KEY", "s3cret")
        from api.deps import API_KEY_ENV

        assert client.post("/api/system/migrate", json={
            "input_file": "mixed.json", "output_file": "g1.json"}).status_code == 401
        assert _stream(client, "none", tag="g2").status_code == 401
        assert client.post("/api/system/monitor",
                           json={"input_file": "mixed.json"}).status_code == 200
        assert client.post("/api/system/auto-test",
                           json={"input_file": "mixed.json"}).status_code == 200
        assert client.post("/api/system/validate-config", json={}).status_code == 200
        assert API_KEY_ENV == "AUGMENTOR_API_KEY"
