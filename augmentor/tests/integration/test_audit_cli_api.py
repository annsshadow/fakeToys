"""audit CLI 命令与 /api/audit 路由测试

CLI：就绪/未就绪提示、--reference 泄漏、--output 落盘；
API：审计往返、缺文件 404。
"""

import json
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent

DIRTY = [
    {"instruction": "联系 zhang@example.com", "input": "", "output": "可"},
    {"instruction": "如何申请？", "input": "", "output": "A"},
    {"instruction": "如何申请？", "input": "", "output": "B"},
]
REFERENCE = [
    {"instruction": "如何申请？", "input": "", "output": "C"},
    {"instruction": "无关问题", "input": "", "output": "D"},
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
def audit_context(tmp_path, monkeypatch):
    """切到 ai 目录并写入脏数据集与参考集

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (脏数据文件, 参考集文件, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    dirty = tmp_path / "dirty.json"
    ref = tmp_path / "ref.json"
    dirty.write_text(json.dumps(DIRTY, ensure_ascii=False), encoding="utf-8")
    ref.write_text(json.dumps(REFERENCE, ensure_ascii=False), encoding="utf-8")
    return dirty, ref, tmp_path


class TestAuditCLI:
    def test_dirty_dataset_not_ready(self, audit_context):
        dirty, _, _ = audit_context
        out, err, code = run_cli(
            ["cli", "audit", "--input", str(dirty)]
        )
        assert code is None, err
        report = json.loads(out[: out.rindex("}") + 1])
        assert report["ready"] is False
        assert report["pii_matches"] == 1
        assert "未就绪" in out

    def test_reference_triggers_leakage(self, audit_context):
        dirty, ref, _ = audit_context
        out, err, code = run_cli(
            ["cli", "audit", "--input", str(dirty), "--reference", str(ref)]
        )
        assert code is None, err
        report = json.loads(out[: out.rindex("}") + 1])
        assert report["leak_count"] == 1

    def test_saves_report_file(self, audit_context):
        dirty, _, tmp = audit_context
        out_file = tmp / "audit.json"
        out, err, code = run_cli(
            ["cli", "audit", "--input", str(dirty), "--output", str(out_file)]
        )
        assert code is None, err
        saved = json.loads(out_file.read_text(encoding="utf-8"))
        assert "ready" in saved

    def test_missing_input_exits_1(self, audit_context, tmp_path):
        _, _, tmp = audit_context
        out, err, code = run_cli(
            ["cli", "audit", "--input", str(tmp / "nope.json")]
        )
        assert code == 1
        assert "错误" in err


class TestAuditAPI:
    def _client(self):
        from fastapi.testclient import TestClient

        from api.main import app

        return TestClient(app)

    def test_audit_endpoint(self, audit_context, monkeypatch):
        _, ref, tmp = audit_context
        monkeypatch.chdir(tmp)
        dirty = tmp / "dirty.json"
        ref2 = tmp / "ref.json"
        dirty.write_text(json.dumps(DIRTY, ensure_ascii=False), encoding="utf-8")
        ref2.write_text(json.dumps(REFERENCE, ensure_ascii=False), encoding="utf-8")

        client = self._client()
        response = client.post(
            "/api/audit",
            json={"input_file": "dirty.json", "reference_file": "ref.json"},
        )
        assert response.status_code == 200
        payload = response.json()
        assert payload["ready"] is False
        assert payload["leak_count"] == 1

    def test_audit_missing_file_404(self, audit_context, monkeypatch):
        _, _, tmp = audit_context
        monkeypatch.chdir(tmp)
        client = self._client()
        response = client.post(
            "/api/audit", json={"input_file": "nope.json"}
        )
        assert response.status_code == 404
