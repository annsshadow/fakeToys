# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L80（B2 读边）：Excel 摄取的 **端到端** 契约（CLI + API）与源格式清单防漂

产品码改的是 `converter.convert_file` 的读边（见
`tests/unit/test_converter_excel_input_l80.py` 的理由说明）。本文件守的是它对外那两面：

* CLI `convert --input-format` 的 `choices` 必须与 `converter.INPUT_FORMAT_CHOICES`
  是同一份清单 —— 这是 A77「一份权威只住一处」在源格式这一维的落点，`EXPORT_FORMATS`
  漏 `raw` 那次事故就是两份清单各写各的。
* API `/api/dataset/convert` 的 `source_format` 是自由字符串，**零改动**就吃到这次
  能力；未知后缀与缺 pandas 两条失败路径都必须变成 400（带可行动文案），不能是 500。
* A134（`target_format="jsonl"` 双重序列化）在这里按 CLI 与 API 各钉一遍端到端形态。

真 xlsx 的用例整体 `skipif`（aug 侧解释器无 pandas），但**不**用文件级
`importorskip`：那样会让整个模块在 aug 侧不被 collect，改动两侧收集数的差值口径。
"""

import io
import json
import os
import sys
import tempfile
from contextlib import redirect_stderr, redirect_stdout
from types import SimpleNamespace

import pytest
from fastapi.testclient import TestClient

from cli import main

from augmentor import csv_excel_import as cei
from augmentor.cli.parser import INPUT_FORMATS, build_parser
from augmentor.converter import EXCEL_FORMAT, INPUT_FORMAT_CHOICES


def _excel_io_available() -> bool:
    try:
        import openpyxl  # noqa: F401
        import pandas  # noqa: F401
    except ImportError:
        return False
    return True


requires_excel = pytest.mark.skipif(
    not _excel_io_available(), reason="造真 xlsx 需要 pandas 与 openpyxl"
)

ROWS = [
    {"instruction": "如何申请租房？", "output": "登录官网申请", "category": "租房"},
    {"instruction": "租房多少钱？", "output": "按房型定价", "category": "价格"},
    {"instruction": "如何退租押金？", "output": "满一年后退还", "category": "退租"},
]


def run_cli(argv):
    """跑一次 CLI，返回 (stdout, exit_code, stderr)

    Args:
        argv: 完整参数列表（含程序名）

    Returns:
        (stdout 文本, SystemExit 码或 None, stderr 文本)
    """
    out = io.StringIO()
    err = io.StringIO()
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
def cli_api_env(tmp_path, monkeypatch):
    """隔离 CWD 与数据白名单，备一个 API 客户端 + 一份后缀认不出的文件

    这里**不** import pandas：`TestUnknownSuffixEndToEnd` 与
    `TestMissingPandasEndToEnd` 要在无 pandas 的解释器上同样跑到。
    """
    from api.main import app

    monkeypatch.setenv(
        "AUGMENTOR_DATA_ROOTS",
        os.pathsep.join([str(tmp_path), tempfile.gettempdir()]),
    )
    monkeypatch.delenv("AUGMENTOR_API_KEY", raising=False)
    monkeypatch.chdir(tmp_path)

    bogus = tmp_path / "corpus.bin"
    bogus.write_bytes(b"\x00\x01\x02not-a-dataset")
    # 一份「存在但内容是占位」的 xlsx：缺 pandas 的降级判据排在读文件之前，所以内容
    # 无所谓 —— 用例因此不需要 pandas，在无 pandas 的解释器上也是真跑。
    ghost_xlsx = tmp_path / "ghost.xlsx"
    ghost_xlsx.write_bytes(b"not-a-real-workbook")
    return SimpleNamespace(tmp=tmp_path, bogus=bogus, ghost_xlsx=ghost_xlsx,
                           client=TestClient(app))


@pytest.fixture
def xlsx_file(tmp_path):
    """一份三列三行的真 xlsx（只被 `@requires_excel` 的类请求）"""
    import pandas as pd

    path = tmp_path / "corpus.xlsx"
    pd.DataFrame(ROWS).to_excel(path, index=False)
    return path


# ==================== 清单防漂：CLI choices ↔ 权威常量 ====================

class TestConvertInputFormatSurface:
    """`--input-format` 收的取值必须**就是**转换器认的那份清单

    比「两份字面量相等」更进一步：从 `build_parser()` 造出来的 action 上取实际生效的
    `choices`，于是「常量改了、`add_argument` 忘了引用它」也会被判负。
    """

    @staticmethod
    def _cli_choices():
        import argparse

        for action in build_parser()._actions:
            if isinstance(action, argparse._SubParsersAction):
                convert = action.choices["convert"]
                return next(a.choices for a in convert._actions
                            if "--input-format" in a.option_strings)
        raise AssertionError("convert 子命令上没有 --input-format")

    def test_parser_wires_the_authority_list(self):
        choices = self._cli_choices()
        assert set(choices) == set(INPUT_FORMAT_CHOICES), (
            f"CLI 缺失 {sorted(set(INPUT_FORMAT_CHOICES) - set(choices))}，"
            f"多余 {sorted(set(choices) - set(INPUT_FORMAT_CHOICES))}"
        )

    def test_literal_matches_the_authority(self):
        assert INPUT_FORMATS == INPUT_FORMAT_CHOICES

    @pytest.mark.parametrize("spelling", ["excel", "xlsx", "xls"])
    def test_excel_spellings_parse(self, spelling):
        """只验 argparse 收不收：真读 xlsx 由 `TestExcelConvertCli` 负责

        这里必须**不**跑到 handler —— 判据是「清单收这个值」，让它去读一份假 xlsx
        只会把「参数被拒」和「读盘失败」两件事混成一条断言。
        """
        parsed = build_parser().parse_args(
            ["convert", "--input", "in.json", "--output", "o.json",
             "--input-format", spelling, "--format", "json"]
        )
        assert parsed.input_format == spelling

    def test_unknown_source_name_is_rejected_by_argparse(self, tmp_path):
        src = tmp_path / "in.json"
        src.write_text("[]", encoding="utf-8")
        _, code, _ = run_cli(
            ["cli", "convert", "--input", str(src), "--output", str(tmp_path / "o.json"),
             "--input-format", "parquet", "--format", "json"]
        )
        assert code == 2, f"argparse 应当直接拒收清单外的源格式: code={code}"


# ==================== 不需要 pandas 的两条失败路径（两侧解释器都跑） ====================

class TestUnknownSuffixEndToEnd:
    """未知后缀：CLI 退出码 1 + 可行动文案；API 400 + 同一句文案"""

    def test_cli_reports_the_reason_and_the_way_out(self, cli_api_env):
        out = cli_api_env.tmp / "o.json"
        _, code, err = run_cli(
            ["cli", "convert", "--input", str(cli_api_env.bogus),
             "--output", str(out), "--format", "json"]
        )
        assert code == 1, f"认不出输入格式应当以非 0 退出: code={code}, err={err}"
        assert "无法从扩展名推断输入格式" in err, err
        assert "--input-format" in err, err
        assert not out.exists(), "报错前落了半成品"

    def test_api_returns_400_with_the_same_message(self, cli_api_env):
        response = cli_api_env.client.post(
            "/api/dataset/convert",
            json={"input_file": str(cli_api_env.bogus),
                  "output_file": str(cli_api_env.tmp / "o.json"),
                  "target_format": "json"},
        )
        assert response.status_code == 400, response.text
        detail = response.json()["detail"]
        assert "无法从扩展名推断输入格式" in detail, detail
        assert "corpus.bin" in detail, detail


class TestMissingPandasEndToEnd:
    """缺 pandas 的降级路径要在两侧解释器上都被真跑到"""

    def test_cli_degrades_to_an_actionable_error(self, cli_api_env, monkeypatch):
        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        out = cli_api_env.tmp / "o.json"

        _, code, err = run_cli(
            ["cli", "convert", "--input", str(cli_api_env.ghost_xlsx), "--output", str(out),
             "--format", "json"]
        )
        assert code == 1, f"缺依赖应当以非 0 退出: code={code}, err={err}"
        assert "pip install pandas openpyxl" in err, err
        assert not out.exists()

    def test_api_returns_400_not_a_500_traceback(self, cli_api_env, monkeypatch):
        monkeypatch.setattr(cei, "HAS_PANDAS", False)
        response = cli_api_env.client.post(
            "/api/dataset/convert",
            json={"input_file": str(cli_api_env.ghost_xlsx),
                  "output_file": str(cli_api_env.tmp / "o.json"),
                  "target_format": "json"},
        )
        assert response.status_code == 400, response.text
        assert "pip install pandas openpyxl" in response.json()["detail"]


# ==================== 真 xlsx 端到端 ====================

@requires_excel
class TestExcelConvertCli:
    """`convert` 吃 xlsx：靠扩展名、靠显式声明都能进，产物保留全部列"""

    def test_inferred_from_suffix(self, cli_api_env, xlsx_file):
        out = cli_api_env.tmp / "out.json"
        stdout, code, err = run_cli(
            ["cli", "convert", "--input", str(xlsx_file), "--output", str(out),
             "--format", "json"]
        )
        assert code is None, err
        payload = json.loads(stdout)
        assert payload["source_format"] == EXCEL_FORMAT
        assert payload["input_count"] == len(ROWS)
        assert json.loads(out.read_text(encoding="utf-8")) == ROWS

    @pytest.mark.parametrize("spelling", ["excel", "xlsx", "xls"])
    def test_declared_source_format(self, cli_api_env, xlsx_file, spelling):
        out = cli_api_env.tmp / f"out_{spelling}.json"
        _, code, err = run_cli(
            ["cli", "convert", "--input", str(xlsx_file), "--output", str(out),
             "--input-format", spelling, "--format", "json"]
        )
        assert code is None, err
        assert json.loads(out.read_text(encoding="utf-8")) == ROWS

    def test_export_to_jsonl_is_standard_jsonl(self, cli_api_env, xlsx_file):
        """A134 在 CLI 上的形态：xlsx → jsonl 的每行是一个对象，不是字符串

        修前这里是 `"{\\"instruction\\": ...}"`，标准 JSONL 读法拿到的每条都是字符串。
        """
        out = cli_api_env.tmp / "out.jsonl"
        _, code, err = run_cli(
            ["cli", "convert", "--input", str(xlsx_file), "--output", str(out),
             "--format", "jsonl"]
        )
        assert code is None, err
        lines = [ln for ln in out.read_text(encoding="utf-8").splitlines() if ln.strip()]
        assert len(lines) == len(ROWS)
        assert [json.loads(ln) for ln in lines] == ROWS


@requires_excel
class TestExcelConvertApi:
    """API 侧零改动即获得读边能力：`source_format` 是自由字符串"""

    def test_convert_xlsx_by_inferred_suffix(self, cli_api_env, xlsx_file):
        """源格式靠扩展名推断（请求里不给 `source_format`）

        `target_format` 必须给：`ConvertRequest` 的默认值是 `jsonl`，不给就会把 JSONL
        写进 `.json` 文件里（`_write_file` 对四种容器一律尊重声明的目标格式）。
        """
        out = cli_api_env.tmp / "api_out.json"
        response = cli_api_env.client.post(
            "/api/dataset/convert",
            json={"input_file": str(xlsx_file), "output_file": str(out),
                  "target_format": "json"},
        )
        assert response.status_code == 200, response.text
        payload = response.json()
        assert payload["source_format"] == EXCEL_FORMAT
        assert payload["input_count"] == len(ROWS)
        assert json.loads(out.read_text(encoding="utf-8")) == ROWS

    def test_convert_xlsx_with_declared_source_format(self, cli_api_env, xlsx_file):
        out = cli_api_env.tmp / "api_out.jsonl"
        response = cli_api_env.client.post(
            "/api/dataset/convert",
            json={"input_file": str(xlsx_file), "output_file": str(out),
                  "source_format": "xlsx", "target_format": "jsonl"},
        )
        assert response.status_code == 200, response.text
        assert response.json()["target_format"] == "jsonl"
        lines = [ln for ln in out.read_text(encoding="utf-8").splitlines() if ln.strip()]
        assert [json.loads(ln) for ln in lines] == ROWS
