# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L71：cli.py 剩余分支（错误参数 exit、就绪提示、告警/问题/错误详情打印）

通过 monkeypatch 底层函数注入「有告警 / 有问题 / 有错误」的返回，
确定性命中各命令的详情打印分支；参数缺失分支走真实 argparse 流程。
"""

import io
import json
import sys
from contextlib import redirect_stdout, redirect_stderr
from pathlib import Path
from types import SimpleNamespace


def _run(argv, cwd=None):
    from cli import main

    old_argv = sys.argv
    old_cwd = None
    if cwd:
        import os

        old_cwd = os.getcwd()
        os.chdir(cwd)
    sys.argv = ["cli"] + argv
    out, err = io.StringIO(), io.StringIO()
    code = None
    try:
        with redirect_stdout(out), redirect_stderr(err):
            try:
                main()
            except SystemExit as e:
                code = e.code
    finally:
        sys.argv = old_argv
        if cwd:
            import os

            os.chdir(old_cwd)
    return out.getvalue(), err.getvalue(), code


class TestBackupMissingArgs:
    def test_restore_requires_name_and_output(self, tmp_path):
        _, err, code = _run(["backup", "--action", "restore"], cwd=tmp_path)
        assert code == 1
        assert "--name" in err and "--output" in err

    def test_delete_requires_name(self, tmp_path):
        _, err, code = _run(["backup", "--action", "delete"], cwd=tmp_path)
        assert code == 1
        assert "--name" in err


class TestAuditReadyMessage:
    def test_ready_dataset_prints_hint(self, tmp_path):
        data = [
            {"instruction": "如何申请退款", "input": "订单号123", "output": "提交工单"},
            {"instruction": "配送多久到", "input": "城市北京", "output": "三日达"},
        ]
        f = tmp_path / "d.json"
        f.write_text(json.dumps(data, ensure_ascii=False), encoding="utf-8")
        out, _, _ = _run(["audit", "--input", str(f)], cwd=tmp_path)
        assert "数据集就绪" in out


class TestVersionControlMissingArgs:
    def test_load_requires_version_and_output(self, tmp_path):
        _, err, code = _run(
            ["version", "--action", "load", "--versions-dir", str(tmp_path / "v")],
            cwd=tmp_path,
        )
        assert code == 1
        assert "--version" in err and "--output" in err

    def test_compare_without_current_version_exits(self, tmp_path):
        """版本目录为空时，`--version-b` 缺省取不到当前版本 → 显式报错"""
        _, err, code = _run(
            [
                "version",
                "--action",
                "compare",
                "--version",
                "v1",
                "--versions-dir",
                str(tmp_path / "empty"),
            ],
            cwd=tmp_path,
        )
        assert code == 1
        assert "--version-b" in err


class TestMonitorAlerts:
    def test_monitor_prints_alerts(self, tmp_path, monkeypatch):
        import augmentor.quality_monitor as qm

        snapshot = SimpleNamespace(
            snapshot_id="s1",
            timestamp="now",
            metrics={"accuracy": 0.4},
            alerts=[SimpleNamespace(message="准确率过低")],
        )
        monkeypatch.setattr(qm, "monitor_quality", lambda items: snapshot)
        data = [{"instruction": "q", "input": "", "output": "a"}]
        f = tmp_path / "d.json"
        f.write_text(json.dumps(data, ensure_ascii=False), encoding="utf-8")
        out, _, _ = _run(["monitor", "--input", str(f)], cwd=tmp_path)
        assert "告警" in out
        assert "准确率过低" in out


class TestDependencyValidateIssues:
    def test_validate_prints_issues(self, tmp_path, monkeypatch):
        import augmentor.dependency as dep_mod

        monkeypatch.setattr(
            dep_mod.DependencyManager,
            "validate_dependencies",
            lambda self: [{"message": "数据集缺失"}],
        )
        out, _, _ = _run(
            ["dependency", "--action", "validate", "--registry-path", str(tmp_path / "reg")],
            cwd=tmp_path,
        )
        assert "发现 1 个问题" in out
        assert "数据集缺失" in out


class TestMigrateErrors:
    def test_migrate_prints_error_details(self, tmp_path, monkeypatch):
        import augmentor.migration as mig

        result = SimpleNamespace(
            migration_id="m1",
            total_items=2,
            migrated_items=1,
            failed_items=1,
            rules_applied=["normalize"],
            errors=[{"index": 1, "error": "字段缺失"}],
        )
        monkeypatch.setattr(mig, "migrate_file", lambda i, o, r: result)
        src = tmp_path / "in.json"
        src.write_text(json.dumps([{"instruction": "q"}], ensure_ascii=False), encoding="utf-8")
        out, _, _ = _run(
            ["migrate", "--input", str(src), "--output", str(tmp_path / "out.json")],
            cwd=tmp_path,
        )
        assert "错误详情" in out
        assert "字段缺失" in out
