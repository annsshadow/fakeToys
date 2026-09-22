# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""check-leakage CLI 与 leakage API 路由测试

CLI：训练/测试集文件 → 泄漏报告（含 --output 落盘、无泄漏提示）；
API：/api/leakage/check 往返与缺文件 404。
"""

import json
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent

TRAIN = [
    {"instruction": "如何申请租房？", "input": "", "output": "登录官网"},
    {"instruction": "租房多少钱？", "input": "", "output": "按月计"},
    {"instruction": "押金怎么退？", "input": "", "output": "满一年退"},
]
# 第 1 条与训练集精确重复，第 2 条近似，第 3 条干净
TEST = [
    {"instruction": "如何申请租房？", "input": "", "output": "登录官网"},
    {"instruction": "租房 多少钱？", "input": "", "output": "按月计"},
    {"instruction": "天气怎么样？", "input": "", "output": "晴"},
]


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
def leakage_context(tmp_path, monkeypatch):
    """切到 ai 目录并写入 train/test 数据集

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (train 文件, test 文件, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    train = tmp_path / "train.json"
    test = tmp_path / "test.json"
    train.write_text(json.dumps(TRAIN, ensure_ascii=False), encoding="utf-8")
    test.write_text(json.dumps(TEST, ensure_ascii=False), encoding="utf-8")
    return train, test, tmp_path


class TestCheckLeakageCLI:
    def test_reports_leaks(self, leakage_context):
        train, test, _ = leakage_context
        out, err, code = run_cli(
            ["cli", "check-leakage", "--train", str(train), "--test", str(test)]
        )
        assert code is None, err
        report = json.loads(out[: out.rindex("}") + 1])
        assert report["total_leaks"] >= 1
        assert report["train_size"] == 3
        assert report["test_size"] == 3

    def test_clean_datasets_report_no_leak(self, leakage_context, tmp_path, monkeypatch):
        train, _, _ = leakage_context
        clean_test = tmp_path / "clean_test.json"
        clean_test.write_text(
            json.dumps([{"instruction": "完全无关的问题", "input": "", "output": "答"}],
                      ensure_ascii=False),
            encoding="utf-8",
        )
        out, err, code = run_cli(
            ["cli", "check-leakage", "--train", str(train), "--test", str(clean_test)]
        )
        assert code is None, err
        assert "未检测到" in out

    def test_saves_report_file(self, leakage_context):
        train, test, tmp = leakage_context
        out_file = tmp / "leak.json"
        out, err, code = run_cli(
            [
                "cli", "check-leakage",
                "--train", str(train), "--test", str(test),
                "--output", str(out_file),
            ]
        )
        assert code is None, err
        saved = json.loads(out_file.read_text(encoding="utf-8"))
        assert saved["total_leaks"] >= 1

    def test_missing_test_file_exits_1(self, leakage_context, tmp_path):
        train, _, _ = leakage_context
        out, err, code = run_cli(
            ["cli", "check-leakage", "--train", str(train), "--test", str(tmp_path / "nope.json")]
        )
        assert code == 1
        assert "错误" in err


class TestLeakageAPI:
    def _client(self):
        from fastapi.testclient import TestClient

        from api.main import app

        return TestClient(app)

    def test_check_endpoint(self, leakage_context, monkeypatch):
        train, test, tmp = leakage_context
        monkeypatch.chdir(tmp)  # 让 load_items 相对路径落在临时目录
        train2 = tmp / "train.json"
        test2 = tmp / "test.json"
        train2.write_text(json.dumps(TRAIN, ensure_ascii=False), encoding="utf-8")
        test2.write_text(json.dumps(TEST, ensure_ascii=False), encoding="utf-8")

        client = self._client()
        response = client.post(
            "/api/leakage/check",
            json={"train_file": "train.json", "test_file": "test.json"},
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["total_leaks"] >= 1
        assert payload["is_clean"] is False

    def test_check_missing_file_404(self, leakage_context, monkeypatch):
        _, _, tmp = leakage_context
        monkeypatch.chdir(tmp)
        client = self._client()
        response = client.post(
            "/api/leakage/check",
            json={"train_file": "nope.json", "test_file": "test.json"},
        )
        assert response.status_code == 404
