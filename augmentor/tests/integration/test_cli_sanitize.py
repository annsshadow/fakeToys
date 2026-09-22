"""CLI sanitize 命令集成测试

PII 脱敏命令端到端：输入 → 脱敏落盘 + 报告，含 --extra 与 --fields。
"""

import json
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent


def run_cli(argv):
    """以给定参数运行 main()，返回 (stdout, stderr, exit_code)

    Args:
        argv: 完整参数列表（含程序名）

    Returns:
        (stdout, stderr, SystemExit 码或 None)
    """
    import io
    import sys
    from contextlib import redirect_stderr, redirect_stdout

    old_argv = sys.argv
    sys.argv = argv
    out, err = io.StringIO(), io.StringIO()
    exit_code = None
    try:
        with redirect_stdout(out), redirect_stderr(err):
            main()
    except SystemExit as exc:
        exit_code = exc.code
    finally:
        sys.argv = old_argv
    return out.getvalue(), err.getvalue(), exit_code


@pytest.fixture
def sanitize_context(tmp_path, monkeypatch):
    """切到 ai 目录并写入含 PII 的数据集

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (数据文件, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    items = [
        {"instruction": "联系 zhang@example.com 或 13800138000", "input": "", "output": "可"},
        {"instruction": "身份证 11010119900307123X", "input": "", "output": "好"},
        {"instruction": "干净问题", "input": "", "output": "干净回答"},
    ]
    data = tmp_path / "pii.json"
    data.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    return data, tmp_path


class TestSanitizeCommand:
    def test_sanitize_writes_output_and_report(self, sanitize_context):
        data, tmp = sanitize_context
        out_file = tmp / "sanitized.json"
        out, err, code = run_cli(
            ["cli", "sanitize", "--input", str(data), "--output", str(out_file)]
        )
        assert code is None, err
        cleaned = json.loads(out_file.read_text(encoding="utf-8"))
        joined = json.dumps(cleaned, ensure_ascii=False)
        assert "zhang@example.com" not in joined
        assert "13800138000" not in joined
        assert "11010119900307123X" not in joined
        # stdout 前半段是可解析的报告 JSON
        report = json.loads(out[: out.rindex("}") + 1])
        assert report["total_items"] == 3
        assert report["touched_items"] == 2

    def test_sanitize_extra_patterns(self, sanitize_context):
        data, tmp = sanitize_context
        extra = tmp / "pii_extra.json"
        extra.write_text(
            json.dumps(
                [{"instruction": "卡号 4111 1111 1111 1111", "input": "", "output": ""}],
                ensure_ascii=False,
            ),
            encoding="utf-8",
        )
        out_file = tmp / "sanitized_extra.json"
        out, err, code = run_cli(
            [
                "cli", "sanitize",
                "--input", str(extra), "--output", str(out_file), "--extra",
            ]
        )
        assert code is None, err
        cleaned = json.loads(out_file.read_text(encoding="utf-8"))
        assert "4111" not in cleaned[0]["instruction"].replace("[CREDIT", "")

    def test_sanitize_custom_fields(self, sanitize_context):
        data, tmp = sanitize_context
        custom = tmp / "pii_fields.json"
        custom.write_text(
            json.dumps([{"question": "邮箱 a@b.com", "answer": "电话 13800138000"}],
                      ensure_ascii=False),
            encoding="utf-8",
        )
        out_file = tmp / "sanitized_fields.json"
        out, err, code = run_cli(
            [
                "cli", "sanitize",
                "--input", str(custom), "--output", str(out_file),
                "--fields", "question", "answer",
            ]
        )
        assert code is None, err
        cleaned = json.loads(out_file.read_text(encoding="utf-8"))
        assert "[EMAIL]" in cleaned[0]["question"]
        assert "[PHONE]" in cleaned[0]["answer"]

    def test_sanitize_missing_input_exits_1(self, sanitize_context):
        data, tmp = sanitize_context
        out, err, code = run_cli(
            [
                "cli", "sanitize",
                "--input", str(tmp / "nope.json"), "--output", str(tmp / "o.json"),
            ]
        )
        assert code == 1
        assert "错误" in err
