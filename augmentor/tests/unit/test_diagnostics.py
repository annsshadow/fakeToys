# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""diagnostics 依赖诊断 + doctor CLI 测试

覆盖依赖探测、降级提示、缺依赖断言、摘要行、doctor 命令文本/JSON 输出。
"""

import json

import pytest

from augmentor import (
    DiagnosticsReport,
    check_dependencies,
    require_dependency,
    summary_line,
)
from augmentor.diagnostics import OPTIONAL_DEPENDENCIES


class TestCheckDependencies:
    def test_yaml_installed(self):
        report = check_dependencies()
        assert report.installed.get("yaml") is True
        assert report.all_required_present is True

    def test_report_structure(self):
        report = check_dependencies()
        assert isinstance(report, DiagnosticsReport)
        assert set(report.installed) == set(OPTIONAL_DEPENDENCIES.keys())
        assert report.available_count == sum(report.installed.values())

    def test_missing_fields_populated(self):
        report = check_dependencies()
        # faiss/psutil 等可能缺失，缺失项需同时出现在 missing 与降级提示
        for name in report.missing:
            assert report.installed[name] is False

    def test_report_to_dict(self):
        report = check_dependencies()
        d = report.to_dict()
        assert d["all_required_present"] is True
        assert "missing" in d and "degraded_features" in d

    def test_summary_line_readable(self):
        line = summary_line()
        assert "可选依赖" in line
        assert "/" in line


class TestRequireDependency:
    def test_require_present_ok(self):
        require_dependency("yaml")  # yaml 恒可用

    def test_require_missing_raises(self, monkeypatch):
        import augmentor.diagnostics as diag

        monkeypatch.setitem(diag.OPTIONAL_DEPENDENCIES, "faiss", "nonexistent_module_xyz")
        with pytest.raises(ImportError, match="缺少依赖"):
            require_dependency("faiss")

    def test_require_unknown_name_noop(self):
        require_dependency("not_a_known_dep")  # 未知名不做断言


class TestDoctorCLI:
    def _run_doctor(self, json_mode: bool):
        import io
        import sys
        from contextlib import redirect_stdout, redirect_stderr

        from cli import main

        argv = ["cli", "doctor"] + (["--json"] if json_mode else [])
        old = sys.argv
        sys.argv = argv
        out, err = io.StringIO(), io.StringIO()
        try:
            with redirect_stdout(out), redirect_stderr(err):
                main()
        finally:
            sys.argv = old
        return out.getvalue(), err.getvalue()

    def test_doctor_text_output(self):
        out, err = self._run_doctor(json_mode=False)
        assert "可选依赖" in out

    def test_doctor_json_output(self):
        out, err = self._run_doctor(json_mode=True)
        report = json.loads(out)
        assert "installed" in report
        assert "missing" in report
        assert report["all_required_present"] is True
