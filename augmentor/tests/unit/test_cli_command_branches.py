# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L73：cli.py 各命令的未完整走通分支（--field length / --output 落盘 / 完整流）

覆盖 outliers 的长度字段附加、validate 结果落盘、backup 删除完整流、
version --enhanced 比较、dependency 图/校验等此前未走通的方向。
"""

import io
import json
import sys
from contextlib import redirect_stdout, redirect_stderr


def _run(argv, cwd=None):
    from cli import main

    import os

    old_argv = sys.argv
    old_cwd = None
    if cwd:
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


def _write(path, data):
    path.write_text(json.dumps(data, ensure_ascii=False), encoding="utf-8")
    return str(path)


def _dataset(n=3):
    return [
        {"instruction": f"问题 {i}", "input": "in", "output": f"答案 {i}"}
        for i in range(n)
    ]


class TestOutliersLengthField:
    def test_outliers_with_length_field(self, tmp_path):
        # 一条超长 instruction，触发长度异常并验证 attach_length_field 分支
        data = _dataset(4)
        data[0]["instruction"] = "长" * 500
        f = _write(tmp_path / "d.json", data)
        out, _, _ = _run(["outliers", "--input", f, "--field", "length"], cwd=tmp_path)
        assert "异常" in out or "outlier" in out.lower()


class TestValidateOutput:
    def test_validate_saves_to_file(self, tmp_path):
        f = _write(tmp_path / "d.json", _dataset())
        out_path = str(tmp_path / "val.json")
        out, _, _ = _run(
            ["validate", "--input", f, "--output", out_path], cwd=tmp_path
        )
        assert "验证结果" in out
        assert (tmp_path / "val.json").exists()


class TestBackupDeleteFlow:
    def test_backup_delete_existing(self, tmp_path):
        f = _write(tmp_path / "d.json", _dataset())
        # 先创建备份
        out, _, _ = _run(
            [
                "backup",
                "--action",
                "create",
                "--input",
                f,
                "--name",
                "bk1",
                "--backup-dir",
                str(tmp_path / "bk"),
            ],
            cwd=tmp_path,
        )
        assert "备份创建成功" in out
        # 再删除该备份
        out2, _, _ = _run(
            [
                "backup",
                "--action",
                "delete",
                "--name",
                "bk1",
                "--backup-dir",
                str(tmp_path / "bk"),
            ],
            cwd=tmp_path,
        )
        assert "删除成功" in out2


class TestVersionControlCompare:
    def test_compare_without_current_version_exits(self, tmp_path):
        """版本目录为空时，`--version-b` 缺省取不到当前版本 → 显式报错"""
        _, err, code = _run(
            [
                "version",
                "--action",
                "compare",
                "--version",
                "v9",
                "--versions-dir",
                str(tmp_path / "vdir"),
            ],
            cwd=tmp_path,
        )
        assert code == 1
        assert "--version-b" in err


class TestDependencyGraphAndValidate:
    def test_dependency_graph_with_output(self, tmp_path):
        reg = str(tmp_path / "reg")
        f = _write(tmp_path / "d.json", _dataset())
        # 注册数据集以填充注册表
        _run(
            [
                "dependency",
                "--action",
                "register",
                "--input",
                f,
                "--name",
                "ds1",
                "--registry-path",
                reg,
            ],
            cwd=tmp_path,
        )
        out, _, _ = _run(
            [
                "dependency",
                "--action",
                "graph",
                "--registry-path",
                reg,
                "--output",
                str(tmp_path / "g.json"),
            ],
            cwd=tmp_path,
        )
        assert "节点数" in out
        assert (tmp_path / "g.json").exists()

    def test_dependency_validate(self, tmp_path):
        reg = str(tmp_path / "reg")
        out, _, _ = _run(
            ["dependency", "--action", "validate", "--registry-path", reg],
            cwd=tmp_path,
        )
        assert "验证" in out or "通过" in out or "问题" in out
