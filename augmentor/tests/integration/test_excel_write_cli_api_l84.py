# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L84（A133 写边）：Excel **写出**的对外两面（CLI `--format` + API convert）与清单防漂

产品码改的是 `converter._write_file`（见 `tests/unit/test_excel_write_edge_l84.py` 的
理由说明）。本文件守它对外那三面：

* CLI `convert --format` 的 `choices` 必须**就是** `converter.OUTPUT_FORMAT_CHOICES`
  —— 集合与顺序都比（A77 的先例：`EXPORT_FORMATS` 漏 `raw` 那次事故就是两份清单各写
  各的，而 argparse 的 help 文案按顺序印给用户）。
* 它与 `--input-format` 的清单**刻意不互为镜像**，差的恰好是 `xls`。这条差集要双向钉：
  只钉「`xls` 不在目标清单」的话，有人把它从源清单删掉也照样达标。
* 三条失败路径在 CLI 与 API 上都必须**响亮**（rc 1 / 400）且**一个字节都不落**。

真 xlsx 的用例逐类 `skipif`；拒收与缺依赖降级两族不需要真依赖，两侧解释器都真跑。
写边（L85 起）只依赖 openpyxl，读回与造样本仍用 pandas，所以两份门禁不能合并成一个。
"""

import io
import json
import os
import sys
import tempfile
import zipfile
from contextlib import redirect_stderr, redirect_stdout
from types import SimpleNamespace

import pytest
from fastapi.testclient import TestClient

from cli import main

from augmentor import excel_write
from augmentor.cli.parser import CONVERT_TARGET_FORMATS, INPUT_FORMATS, build_parser
from augmentor.converter import EXCEL_FORMAT, OUTPUT_FORMAT_CHOICES


def _excel_io_available() -> bool:
    try:
        import openpyxl  # noqa: F401
        import pandas  # noqa: F401
    except ImportError:
        return False
    return True


requires_excel = pytest.mark.skipif(
    not _excel_io_available(), reason="造/读真 xlsx 需要 pandas 与 openpyxl"
)

ROWS = [
    {"instruction": "如何申请租房？", "output": "登录官网申请", "category": "租房"},
    {"instruction": "租房多少钱？", "output": "按房型定价"},
    {"instruction": "如何退租押金？", "output": "满一年后退还", "category": "退租",
     "note": "只有第三条有第五列"},
]


def run_cli(argv):
    """跑一次 CLI，返回 (stdout, SystemExit 码或 None, stderr)"""
    out, err = io.StringIO(), io.StringIO()
    old_argv = sys.argv
    sys.argv = argv
    code = None
    try:
        with redirect_stdout(out), redirect_stderr(err):
            main()
    except SystemExit as exc:
        code = exc.code
    finally:
        sys.argv = old_argv
    return out.getvalue(), code, err.getvalue()


@pytest.fixture
def env(tmp_path, monkeypatch):
    """隔离 CWD 与数据白名单，备一个 API 客户端 + 一份源 JSON + 一份假 xlsx"""
    from api.main import app

    monkeypatch.setenv(
        "AUGMENTOR_DATA_ROOTS",
        os.pathsep.join([str(tmp_path), tempfile.gettempdir()]),
    )
    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    monkeypatch.chdir(tmp_path)

    source = tmp_path / "corpus.json"
    source.write_text(json.dumps(ROWS, ensure_ascii=False), encoding="utf-8")
    return SimpleNamespace(tmp=tmp_path, source=source, client=TestClient(app))


def post_convert(client, payload):
    return client.post("/api/dataset/convert", json=payload)


# ==================== 清单防漂：CLI choices ↔ 权威常量 ====================

class TestConvertTargetFormatSurface:
    """`--format` 收的取值必须**就是**转换器写得出的那份清单

    比「两份字面量相等」更进一步：从 `build_parser()` 造出来的 action 上取实际生效的
    `choices`，于是「常量改了、`add_argument` 忘了引用它」也会被判负。
    """

    @staticmethod
    def _cli_choices(option="--format"):
        import argparse

        for action in build_parser()._actions:
            if isinstance(action, argparse._SubParsersAction):
                convert = action.choices["convert"]
                return next(a.choices for a in convert._actions
                            if option in a.option_strings)
        raise AssertionError(f"convert 子命令上没有 {option}")

    def test_parser_wires_the_authority_list(self):
        choices = self._cli_choices()
        assert choices == CONVERT_TARGET_FORMATS, (
            f"parser 里的字面量与 `add_argument` 实际生效的 choices 不一致"
        )
        assert list(choices) == list(OUTPUT_FORMAT_CHOICES), (
            f"CLI 缺失 {sorted(set(OUTPUT_FORMAT_CHOICES) - set(choices))}，"
            f"多余 {sorted(set(choices) - set(OUTPUT_FORMAT_CHOICES))}"
        )

    def test_literal_matches_the_authority(self):
        assert CONVERT_TARGET_FORMATS == OUTPUT_FORMAT_CHOICES

    def test_xls_is_input_only_both_directions(self):
        """`xls` 在源清单里、不在目标清单里；两张清单的差集**恰好**只有它

        双向是必要的：只判「目标清单没有 xls」，有人把 xls 从源清单删掉（读边能力倒退）
        也不会红。
        """
        assert set(INPUT_FORMATS) - set(CONVERT_TARGET_FORMATS) == {"xls"}
        assert set(CONVERT_TARGET_FORMATS) - set(INPUT_FORMATS) == set()
        assert "xls" in INPUT_FORMATS and "xls" not in CONVERT_TARGET_FORMATS

    @pytest.mark.parametrize("spelling", ["excel", "xlsx"])
    def test_excel_spellings_parse(self, spelling):
        """只验 argparse 收不收：真写 xlsx 由 `TestExcelWriteCli` 负责

        这里必须**不**跑到 handler —— 判据是「清单收这个值」，让它去写一份文件只会把
        「参数被拒」和「落盘失败」两件事混成一条断言。
        """
        parsed = build_parser().parse_args(
            ["convert", "--input", "in.json", "--output", "o.xlsx",
             "--format", spelling]
        )
        assert parsed.format == spelling

    def test_xls_target_is_rejected_by_argparse(self, tmp_path):
        _, code, err = run_cli(
            ["cli", "convert", "--input", str(tmp_path / "in.json"),
             "--output", str(tmp_path / "o.xlsx"), "--format", "xls"]
        )
        assert code == 2, f"argparse 应当直接拒收写不出来的目标: code={code}"
        assert "invalid choice" in err, err


# ==================== 不需要真依赖的失败路径（两侧解释器都跑） ====================

class TestFalseContainerEndToEnd:
    """名字与内容不符的输出：CLI 退出码 1 + 可行动文案；API 400 + 同一句文案"""

    @pytest.mark.parametrize(("target", "name"), [
        ("json", "cli_mismatch.xlsx"),   # 扩展名要 Excel，声明要 JSON
        ("json", "cli_legacy.xls"),      # 老 BIFF 名字：写不出来
        ("excel", "cli_wrong_name.json"),  # 声明要 Excel，扩展名说自己是 JSON
    ])
    def test_cli_refuses_and_writes_nothing(self, env, target, name):
        out = env.tmp / name
        _, code, err = run_cli(
            ["cli", "convert", "--input", str(env.source),
             "--output", str(out), "--format", target]
        )
        assert code == 1, f"拒收应当以非 0 退出: code={code}, err={err}"
        assert "Excel" in err or ".xlsx" in err, err
        assert not out.exists(), "报错前落了半成品"

    def test_api_returns_400_with_the_same_message(self, env):
        out = env.tmp / "api_mismatch.xlsx"
        response = post_convert(env.client, {
            "input_file": str(env.source), "output_file": str(out),
            "target_format": "json"})
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "api_mismatch.xlsx" in detail, detail
        assert "Excel" in detail, detail
        assert not out.exists()


class TestMissingOpenpyxlWriteEndToEnd:
    """缺写边依赖时的降级路径要在两侧解释器上都被真跑到

    L85 把写边从 pandas 换成只依赖 openpyxl 的 `excel_write`，所以这一族判的是
    `HAS_OPENPYXL` 而不是 `HAS_PANDAS`：**打桩错名字会让用例变成永真的假绿** ——
    打 `cei.HAS_PANDAS` 时写边根本不看那个标志位，降级分支一次也没进过。
    """

    def test_cli_degrades_to_an_actionable_error(self, env, monkeypatch):
        monkeypatch.setattr(excel_write, "HAS_OPENPYXL", False)
        out = env.tmp / "openpyxl_less.xlsx"
        _, code, err = run_cli(
            ["cli", "convert", "--input", str(env.source), "--output", str(out),
             "--format", "excel"]
        )
        assert code == 1, f"缺依赖应当以非 0 退出: code={code}, err={err}"
        assert "pip install openpyxl" in err, err
        assert not out.exists()

    def test_api_returns_400_not_a_500_traceback(self, env, monkeypatch):
        monkeypatch.setattr(excel_write, "HAS_OPENPYXL", False)
        out = env.tmp / "api_openpyxl_less.xlsx"
        response = post_convert(env.client, {
            "input_file": str(env.source), "output_file": str(out),
            "target_format": "xlsx"})
        assert response.status_code == 400, response.text
        assert "pip install openpyxl" in response.json()["detail"]
        assert not out.exists()


# ==================== 真写盘端到端 ====================

@requires_excel
class TestExcelWriteCli:
    def test_writes_a_real_workbook_and_echoes_the_target(self, env):
        out = env.tmp / "cli_out.xlsx"
        stdout, code, err = run_cli(
            ["cli", "convert", "--input", str(env.source), "--output", str(out),
             "--format", "xlsx"]
        )
        assert code is None, err
        payload = json.loads(stdout)
        assert payload["target_format"] == EXCEL_FORMAT
        assert payload["output_count"] == len(ROWS)
        assert zipfile.is_zipfile(out)

    def test_creates_the_parent_directory_it_needed(self, env):
        out = env.tmp / "deep" / "nested" / "cli_out.xlsx"
        _, code, err = run_cli(
            ["cli", "convert", "--input", str(env.source), "--output", str(out),
             "--format", "excel"]
        )
        assert code is None, err
        assert out.is_file()

    def test_csv_to_xlsx_and_back_keep_every_column(self, env):
        """csv → xlsx → csv 双向都不剥列、不改列序（写边复用 csv 那份表头口径）"""
        import csv

        table = env.tmp / "in.csv"
        columns = ["instruction", "output", "note"]
        with open(table, "w", encoding="utf-8-sig", newline="") as f:
            writer = csv.DictWriter(f, fieldnames=columns)
            writer.writeheader()
            writer.writerows([{c: r.get(c, "") for c in columns} for r in ROWS])

        book = env.tmp / "again.xlsx"
        back = env.tmp / "again.csv"
        for src, dst, fmt in ((table, book, "xlsx"), (book, back, "csv")):
            _, code, err = run_cli(
                ["cli", "convert", "--input", str(src), "--output", str(dst),
                 "--format", fmt]
            )
            assert code is None, err

        import pandas as pd

        assert list(pd.read_excel(book).columns) == ["instruction", "output", "note"]
        with open(back, encoding="utf-8-sig", newline="") as f:
            rows = list(csv.DictReader(f))
        assert [r["instruction"] for r in rows] == [r["instruction"] for r in ROWS]
        assert rows[2]["note"] == "只有第三条有第五列"


@requires_excel
class TestExcelWriteApi:
    def test_convert_to_xlsx_by_declared_target(self, env):
        out = env.tmp / "api_out.xlsx"
        response = post_convert(env.client, {
            "input_file": str(env.source), "output_file": str(out),
            "target_format": "excel"})
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["target_format"] == EXCEL_FORMAT
        assert payload["output_count"] == len(ROWS)
        assert zipfile.is_zipfile(out)

    def test_round_trip_through_the_api(self, env):
        """API 写出的 xlsx 再让 API 读回来：三条记录四个字段一个不丢"""
        import pandas as pd

        book = env.tmp / "round.xlsx"
        post = post_convert(env.client, {
            "input_file": str(env.source), "output_file": str(book),
            "target_format": "xlsx"})
        assert post.status_code == 200, post.text

        back = env.tmp / "back.json"
        response = post_convert(env.client, {
            "input_file": str(book), "output_file": str(back),
            "target_format": "json"})
        assert response.status_code == 200, response.text
        assert response.json()["source_format"] == EXCEL_FORMAT
        got = json.loads(back.read_text(encoding="utf-8"))
        assert [g["instruction"] for g in got] == [r["instruction"] for r in ROWS]
        assert got[0]["category"] == "租房"
        assert got[2]["note"] == "只有第三条有第五列"
        assert pd.read_excel(book).shape == (3, 4)
