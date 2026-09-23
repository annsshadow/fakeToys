# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 版本管理与备份命令集成测试

3.0 起 `version` 只有一套后端（`version_control.DatasetVersionManager`，
独立 `--versions-dir`）：原先并存的 `pipeline.version_manager` 已被合并掉，
因此本文件不再需要「造一份指向临时目录的 config.yaml」那套脚手架。

backup（create/restore/list/delete）未受影响。
"""

import json
import re

import pytest

from cli import main

SAMPLE_ITEMS = [
    {"instruction": "如何申请租房？", "input": "", "output": "登录官网申请"},
    {"instruction": "租房多少钱？", "input": "", "output": "按房型定价"},
    {"instruction": "如何退租押金？", "input": "", "output": "满一年后退还"},
    {"instruction": "可以申请月付吗？", "input": "", "output": "支持月付"},
    {"instruction": "租期最短多久？", "input": "", "output": "一个月起租"},
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
def vdir(tmp_path):
    """版本目录（独立于 cwd，避免污染工作副本）"""
    return str(tmp_path / ".versions")


def create_version(data, versions_dir, description=""):
    """创建版本并返回版本 ID

    Args:
        data: 数据文件路径
        versions_dir: 版本目录
        description: 版本描述

    Returns:
        版本 ID
    """
    out, err, code = run_cli(
        ["cli", "version", "--action", "create", "--input", str(data),
         "--versions-dir", versions_dir, "--description", description]
    )
    assert code is None, err
    match = re.search(r"版本ID: (\S+)", out)
    assert match is not None, f"未找到版本 ID: {out}"
    return match.group(1)


class TestVersionCommand:
    """`version` 命令（单一后端 DatasetVersionManager）"""

    def test_create_list_load_roundtrip(self, tmp_path, monkeypatch, vdir):
        """create → list → load 需能往返恢复原始数据"""
        monkeypatch.chdir(tmp_path)
        data = tmp_path / "ds.json"
        data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")

        version_id = create_version(data, vdir)

        out, _, code = run_cli(
            ["cli", "version", "--action", "list", "--versions-dir", vdir]
        )
        assert code is None
        assert "找到 1 个版本" in out
        assert version_id in out

        restored = tmp_path / "restored.json"
        out, _, code = run_cli(
            ["cli", "version", "--action", "load", "--version", version_id,
             "--output", str(restored), "--versions-dir", vdir]
        )
        assert code is None
        assert json.loads(restored.read_text(encoding="utf-8")) == SAMPLE_ITEMS

    def test_compare_counts_are_internally_consistent(self, tmp_path, monkeypatch, vdir):
        """compare 的三个计数必须自洽

        旧 `diff` 动作已并入 `compare`，这条守住合并后仍能回答「差了多少条」。
        断言用不变式而不是写死数字：only_in_a + in_both == |A|、
        only_in_b + in_both == |B|。写死数字会随 key 字段定义变化而失效。
        """
        monkeypatch.chdir(tmp_path)
        a = tmp_path / "ds_a.json"
        b = tmp_path / "ds_b.json"
        extra = {"instruction": "新数据", "input": "", "output": "新答案"}
        a.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
        b.write_text(
            json.dumps(SAMPLE_ITEMS[:3] + [extra], ensure_ascii=False), encoding="utf-8"
        )

        vid_a = create_version(a, vdir)
        vid_b = create_version(b, vdir)

        out, err, code = run_cli(
            ["cli", "version", "--action", "compare", "--version", vid_a,
             "--version-b", vid_b, "--versions-dir", vdir]
        )
        assert code is None, err

        only_a = int(re.search(r"仅在A中: (\d+)", out).group(1))
        only_b = int(re.search(r"仅在B中: (\d+)", out).group(1))
        both = int(re.search(r"共同数据: (\d+)", out).group(1))

        assert only_a + both == len(SAMPLE_ITEMS)
        assert only_b + both == len(SAMPLE_ITEMS[:3]) + 1
        assert only_b >= 1, "B 多出的那条必须体现在计数里"

    def test_compare_defaults_to_current_version(self, tmp_path, monkeypatch, vdir):
        """`--version-b` 缺省时与当前版本比较（旧 diff 的等价用法）"""
        monkeypatch.chdir(tmp_path)
        a = tmp_path / "ds_a.json"
        b = tmp_path / "ds_b.json"
        a.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
        b.write_text(json.dumps(SAMPLE_ITEMS[:2], ensure_ascii=False), encoding="utf-8")

        vid_a = create_version(a, vdir)
        vid_b = create_version(b, vdir)

        out, err, code = run_cli(
            ["cli", "version", "--action", "compare", "--version", vid_a,
             "--versions-dir", vdir]
        )
        assert code is None, err
        assert vid_a in out and vid_b in out, "比较的应是 vid_a 与当前版本 vid_b"

    def test_compare_requires_version(self, tmp_path, monkeypatch, vdir):
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "version", "--action", "compare", "--versions-dir", vdir]
        )
        assert code == 1
        assert "--version" in err

    def test_rollback_changes_current(self, tmp_path, monkeypatch, vdir):
        """rollback 必须真的改变 current（旧 pipeline 后端才有这个动作）"""
        monkeypatch.chdir(tmp_path)
        data = tmp_path / "ds.json"
        data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
        version_id = create_version(data, vdir)

        out, err, code = run_cli(
            ["cli", "version", "--action", "rollback", "--version", version_id,
             "--versions-dir", vdir]
        )
        assert code is None, err
        assert f"已回滚到 {version_id}" in out

        from augmentor.version_control import DatasetVersionManager

        assert DatasetVersionManager(vdir).get_current_version() == version_id


class TestBackupCommand:
    """backup 命令"""

    def test_backup_lifecycle(self, tmp_path, monkeypatch):
        """create → list → restore → delete 全生命周期"""
        monkeypatch.chdir(tmp_path)
        data = tmp_path / "ds.json"
        data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
        bdir = str(tmp_path / ".backups")

        out, _, code = run_cli(
            [
                "cli", "backup", "--action", "create",
                "--input", str(data), "--name", "my_backup", "--backup-dir", bdir,
            ]
        )
        assert code is None
        assert "备份创建成功" in out

        out2, _, code2 = run_cli(
            ["cli", "backup", "--action", "list", "--backup-dir", bdir]
        )
        assert code2 is None
        assert "my_backup" in out2

        restored = tmp_path / "restored.json"
        out3, _, code3 = run_cli(
            [
                "cli", "backup", "--action", "restore",
                "--name", "my_backup", "--output", str(restored), "--backup-dir", bdir,
            ]
        )
        assert code3 is None
        assert json.loads(restored.read_text(encoding="utf-8")) == SAMPLE_ITEMS

        out4, _, code4 = run_cli(
            ["cli", "backup", "--action", "delete",
             "--name", "my_backup", "--backup-dir", bdir]
        )
        assert code4 is None
        assert "删除成功" in out4

    def test_backup_missing_input_exits_1(self, tmp_path, monkeypatch):
        """create 缺 --input 需以 exit 1 拒绝而非崩溃"""
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "backup", "--action", "create", "--name", "x"]
        )
        assert code == 1
        assert "必需" in err
