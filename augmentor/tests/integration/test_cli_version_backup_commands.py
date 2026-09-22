"""CLI 版本管理与备份命令集成测试

version（管道 VersionManager）/ version-control（DatasetVersionManager）/
backup（create/restore/list/delete）三个命令族，全部重定向到临时目录。
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
def sandbox(tmp_path, monkeypatch):
    """在临时目录准备 config.yaml（版本存储指向临时目录）+ 两个数据集

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (数据文件A, 数据文件B, 配置文件路径, 临时目录)
    """
    import yaml

    monkeypatch.chdir(tmp_path)
    src_cfg = AI_DIR / "config.yaml"
    text = src_cfg.read_text(encoding="utf-8")
    text = text.replace("storage_dir: data/versions",
                        f"storage_dir: {tmp_path / 'versions'}")
    text = text.replace("file: app.log", f"file: {tmp_path / 'app.log'}")
    cfg = tmp_path / "config.yaml"
    cfg.write_text(text, encoding="utf-8")

    a = tmp_path / "ds_a.json"
    b = tmp_path / "ds_b.json"
    a.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
    b.write_text(json.dumps(SAMPLE_ITEMS[:3] + [
        {"instruction": "新数据", "input": "", "output": "新答案"},
    ], ensure_ascii=False), encoding="utf-8")
    return a, b, cfg, tmp_path


class TestVersionPipelineCommand:
    """version 命令（管道内置 VersionManager）"""

    def test_version_create_then_list(self, sandbox):
        """create 需返回版本 ID，list 需列出已创建版本"""
        a, _, cfg, _ = sandbox
        out2, err2, code = run_cli(
            [
                "cli", "--config", str(cfg), "version",
                "--action", "create", "--input", str(a),
            ]
        )
        assert code is None
        m = re.search(r"创建版本: (\S+)", out2)
        assert m is not None, f"未找到版本 ID: {out2} {err2}"
        version_id = m.group(1)

        out3, err3, code3 = run_cli(
            ["cli", "--config", str(cfg), "version", "--action", "list"]
        )
        assert code3 is None
        assert version_id in out3

    def test_version_diff_counts(self, sandbox):
        """两版本 diff 需报告新增/移除计数"""
        a, b, cfg, _ = sandbox
        out2, _, _ = run_cli(
            ["cli", "--config", str(cfg), "version",
             "--action", "create", "--input", str(a)]
        )
        vid1 = re.search(r"创建版本: (\S+)", out2).group(1)
        out3, _, _ = run_cli(
            ["cli", "--config", str(cfg), "version",
             "--action", "create", "--input", str(b)]
        )
        vid2 = re.search(r"创建版本: (\S+)", out3).group(1)

        out4, err4, code = run_cli(
            [
                "cli", "--config", str(cfg), "version",
                "--action", "diff",
                "--version-id", vid1, "--version-id-2", vid2,
            ]
        )
        assert code is None
        m = re.search(r"新增: (\d+), 移除: (\d+)", out4)
        assert m is not None, out4 + err4
        assert int(m.group(2)) >= 1  # B 比 A 多 1 条

    def test_version_history_json(self, sandbox):
        """history 需输出可解析的 JSON 历史"""
        a, _, cfg, _ = sandbox
        run_cli(["cli", "--config", str(cfg), "version",
                 "--action", "create", "--input", str(a)])
        out, _, code = run_cli(
            ["cli", "--config", str(cfg), "version", "--action", "history"]
        )
        assert code is None
        parsed = json.loads(out)
        assert isinstance(parsed, list)
        assert len(parsed) >= 1


class TestVersionControlCommand:
    """version-control 命令（DatasetVersionManager，独立 --versions-dir）"""

    def test_create_list_load_roundtrip(self, tmp_path, monkeypatch):
        """create → list → load 需能往返恢复原始数据"""
        monkeypatch.chdir(tmp_path)
        data = tmp_path / "ds.json"
        data.write_text(json.dumps(SAMPLE_ITEMS, ensure_ascii=False), encoding="utf-8")
        vdir = str(tmp_path / ".versions")

        out, _, code = run_cli(
            [
                "cli", "version", "--enhanced", "--action", "create",
                "--input", str(data), "--versions-dir", vdir,
            ]
        )
        assert code is None
        m = re.search(r"版本ID: (\S+)", out)
        assert m is not None

        out2, _, code2 = run_cli(
            ["cli", "version", "--enhanced", "--action", "list", "--versions-dir", vdir]
        )
        assert code2 is None
        assert "找到 1 个版本" in out2

        restored = tmp_path / "restored.json"
        out3, _, code3 = run_cli(
            [
                "cli", "version", "--enhanced", "--action", "load",
                "--version", m.group(1), "--output", str(restored),
                "--versions-dir", vdir,
            ]
        )
        assert code3 is None
        assert json.loads(restored.read_text(encoding="utf-8")) == SAMPLE_ITEMS

    def test_compare_requires_args(self, tmp_path, monkeypatch):
        """compare 缺参数需以 exit 1 拒绝"""
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "version", "--enhanced", "--action", "compare"]
        )
        assert code == 1
        assert "必需" in err


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
