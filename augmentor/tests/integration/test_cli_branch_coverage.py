# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 剩余分支补测（L10）

覆盖此前未触达的分支：version rollback、stream clean/直通操作、
backup 空列表/删除不存在、version 缺参与比较、dependency
空列表/依赖图、benchmark 空基准跳过对比。

3.0 起 `--enhanced` 已移除，原先测 `--enhanced` 路径的用例改为测合并后的
单一实现；pipeline 版 version 后端的用例改为守「不得再写 pipeline 存储目录」。
"""

import json
import re
from pathlib import Path

import pytest

from cli import main

AI_DIR = Path(__file__).resolve().parent.parent.parent

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
def branch_context(tmp_path, monkeypatch):
    """切到 ai 目录并写入数据集与空基准

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (数据集文件, 空基准文件, 临时目录)
    """
    monkeypatch.chdir(AI_DIR)
    data = tmp_path / "branch.json"
    data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    empty_baseline = tmp_path / "empty_baseline.json"
    empty_baseline.write_text("{}", encoding="utf-8")
    return data, empty_baseline, tmp_path


class TestVersionRollbackMoved:
    """原 `TestVersionRollback`（pipeline 版 VersionManager 的 rollback）已移除。

    3.0 把 `version` 的两套后端合并为 `version_control.DatasetVersionManager`，
    pipeline 版的 `--version-id` / `config.versioning.storage_dir` 已不存在。
    等价行为由 `tests/integration/test_cli_merged_commands.py` 的
    `TestVersionCommand::test_rollback_sets_current_version` 覆盖
    （它除了退出码与输出，还断言 `get_current_version()` 真的被改了）。
    """

    def test_only_versions_dir_is_written(self, tmp_path, monkeypatch):
        """只能写 `--versions-dir`，不得再往 pipeline 的 storage_dir 写

        用一个「空 cwd + 显式 --versions-dir」的行为信号来判断后端：
        pipeline 版会把版本写进 `config.versioning.storage_dir`，
        `DatasetVersionManager` 只认 `--versions-dir`。
        """
        monkeypatch.chdir(tmp_path)
        data = tmp_path / "a.json"
        data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
        vdir = tmp_path / "vd"

        out, err, code = run_cli(
            ["cli", "version", "--action", "create",
             "--input", str(data), "--versions-dir", str(vdir)]
        )

        assert code is None, err
        assert vdir.is_dir(), "--versions-dir 未被使用"
        assert not (tmp_path / "data" / "versions").exists(), (
            "pipeline 版后端仍在写 config.versioning.storage_dir"
        )


class TestStreamOtherOperations:
    def test_stream_clean_operation(self, branch_context):
        data, _, tmp = branch_context
        out_file = tmp / "stream_clean.json"
        out, err, code = run_cli(
            [
                "cli", "stream", "--input", str(data),
                "--output", str(out_file), "--operation", "clean",
            ]
        )
        assert code is None, err
        report = json.loads(out)
        assert report["total_input"] == 5

    def test_stream_export_passthrough(self, branch_context):
        """export 操作为直通处理，输出条数 == 输入条数"""
        data, _, tmp = branch_context
        out_file = tmp / "stream_pass.json"
        out, err, code = run_cli(
            [
                "cli", "stream", "--input", str(data),
                "--output", str(out_file), "--operation", "export",
            ]
        )
        assert code is None, err
        report = json.loads(out)
        assert report["total_output"] == 5


class TestBackupBranches:
    def test_backup_list_empty(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "backup", "--action", "list", "--backup-dir", str(tmp_path / "bk")]
        )
        assert code is None, err
        assert "没有找到备份" in out

    def test_backup_delete_missing(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            [
                "cli", "backup", "--action", "delete", "--name", "ghost",
                "--backup-dir", str(tmp_path / "bk"),
            ]
        )
        assert code is None, err
        assert "删除失败" in out


class TestVersionControlBranches:
    def test_create_missing_input_exits_1(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            [
                "cli", "version", "--action", "create",
                "--versions-dir", str(tmp_path / "vc"),
            ]
        )
        assert code == 1
        assert "必需" in err

    def test_list_empty(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            [
                "cli", "version", "--action", "list",
                "--versions-dir", str(tmp_path / "vc"),
            ]
        )
        assert code is None, err
        assert "没有找到版本" in out

    def test_compare_requires_version(self, tmp_path, monkeypatch):
        """compare 不再需要 --output（旧实现的必填项其实从未被使用）"""
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            [
                "cli", "version", "--action", "compare",
                "--versions-dir", str(tmp_path / "vc"),
            ]
        )
        assert code == 1
        assert "--version" in err

    def test_compare_full_flow(self, tmp_path, monkeypatch):
        """两版本 + current 指向新版后，compare 旧版需报只在B中的条目"""
        monkeypatch.chdir(tmp_path)
        data = tmp_path / "ds.json"
        data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
        data2 = tmp_path / "ds2.json"
        data2.write_text(json.dumps(SAMPLE_ITEMS[:2], ensure_ascii=False), encoding="utf-8")
        vdir = str(tmp_path / "vc")

        out1, _, c1 = run_cli(
            ["cli", "version", "--action", "create",
             "--input", str(data), "--versions-dir", vdir]
        )
        assert c1 is None
        vid1 = re.search(r"版本ID: (\S+)", out1).group(1)
        out2, _, c2 = run_cli(
            ["cli", "version", "--action", "create",
             "--input", str(data2), "--versions-dir", vdir]
        )
        assert c2 is None
        # current = vid2；比较 current vs vid1：vid2 比 vid1 少 3 条
        out3, err3, c3 = run_cli(
            [
                "cli", "version", "--action", "compare",
                "--version", vid1, "--versions-dir", vdir,
            ]
        )
        assert c3 is None, err3
        assert "仅在B中" in out3


class TestDependencyBranches:
    def test_dependency_list_empty(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "dependency", "--action", "list", "--registry-path", str(tmp_path / "reg")]
        )
        assert code is None, err
        assert "没有注册的数据集" in out

    def test_dependency_graph_with_save(self, branch_context):
        data, _, tmp = branch_context
        reg = tmp / "reg"
        run_cli(["cli", "dependency", "--action", "register",
                 "--input", str(data), "--name", "ds1", "--registry-path", str(reg)])
        graph_file = tmp / "graph.json"
        out, err, code = run_cli(
            [
                "cli", "dependency", "--action", "graph",
                "--registry-path", str(reg), "--output", str(graph_file),
            ]
        )
        assert code is None, err
        assert "节点数: 1" in out
        graph = json.loads(graph_file.read_text(encoding="utf-8"))
        assert len(graph["nodes"]) == 1


class TestBenchmarkSkipBranch:
    def test_empty_baseline_reports_skip(self, branch_context):
        """存在的空基准需走跳过对比分支并在 stderr 说明原因"""
        data, empty_baseline, _ = branch_context
        out, err, code = run_cli(
            [
                "cli", "benchmark", "--input", str(data),
                "--baseline", str(empty_baseline),
            ]
        )
        assert code is None
        assert "跳过基准对比" in err
        parsed = json.loads(out)
        assert "comparisons" not in parsed


class TestMergedCommandArgValidation:
    """合并后残留的参数校验分支

    合并把「两个实现各自的必填参数」压进一个解析器，argparse 表达不了
    「取决于另一个 flag」的必填，于是校验下移到 handler。这几条守住那些
    分支——它们平时不会被走到，一旦回归就是「静默不落盘 / 静默用了别的后端」。
    """

    def test_export_without_any_output_target_exits_1(self, branch_context):
        """既不给 --output 也不给 --output-dir 需报错，而不是静默不落盘"""
        data, _, _ = branch_context
        out, err, code = run_cli(["cli", "export", "--input", str(data)])
        assert code == 1
        assert "--output-dir" in err
        assert "--output" in err, "错误信息要同时指出两种输出模式的写法"

    def test_export_batch_without_formats_exits_1(self, branch_context):
        """--output-dir 缺 --formats 需报错（否则会静默只导默认格式）"""
        data, _, tmp = branch_context
        out, err, code = run_cli(
            ["cli", "export", "--input", str(data), "--output-dir", str(tmp / "b")]
        )
        assert code == 1
        assert "--formats" in err

    def test_version_diff_action_is_gone(self, tmp_path, monkeypatch):
        """`diff` 已并入 `compare`，必须由 argparse 拒绝（退出码 2）

        若只是从 choices 里删掉而没删 handler 分支，就会变成「静默无输出」。
        """
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "version", "--action", "diff", "--versions-dir", str(tmp_path / "v")]
        )
        assert code == 2
        assert "diff" in err

    def test_version_load_without_args_exits_1(self, tmp_path, monkeypatch):
        """`load` 现在是规范动作，缺 --version / --output 时需显式报错"""
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "version", "--action", "load", "--versions-dir", str(tmp_path / "v")]
        )
        assert code == 1
        assert "--version" in err

    def test_version_rollback_without_version_exits_1(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "version", "--action", "rollback", "--versions-dir", str(tmp_path / "v")]
        )
        assert code == 1
        assert "--version" in err


class TestUnimplementedSubcommandGuard:
    def test_missing_dispatch_entry_fails_loud(self, tmp_path, monkeypatch):
        """分发表漏登记时必须报错退出，而不是静默什么都不做

        T1.6 拆包后 `main()` 用 `COMMANDS.get(...)` 查表；如果只是 `.get()` 后
        直接调用，漏登记会变成 `None is not callable` 或静默跳过。这条守住那个
        显式 `if handler is None` 分支。
        """
        monkeypatch.chdir(tmp_path)
        # `cli.py` 是 `from augmentor.cli import COMMANDS`，名字在导入时已绑定，
        # 所以要打在 `cli` 模块自己的命名空间上。
        monkeypatch.setattr("cli.COMMANDS", {}, raising=True)
        out, err, code = run_cli(
            ["cli", "augment", "--input", "x.json", "--output", "y.json"]
        )
        assert code == 1
        assert "未实现的子命令" in err
