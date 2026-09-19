"""CLI 配置验证命令测试

validate-config 需通过校验器正确报告仓库自带配置为有效，
并对非法配置给出明确的错误清单。
"""

import json
import textwrap
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


class TestValidateConfigCommand:
    def test_repo_config_is_valid(self, tmp_path, monkeypatch):
        """仓库自带 config.yaml 需通过校验（补齐 app 段后）"""
        monkeypatch.chdir(AI_DIR)
        out, err, code = run_cli(["cli", "validate-config"])
        assert code is None
        assert "通过" in out
        assert "错误: 0" in out

    def test_missing_file_reports_failure(self, tmp_path, monkeypatch):
        """配置文件缺失需报失败而非崩溃"""
        monkeypatch.chdir(tmp_path)
        out, err, code = run_cli(
            ["cli", "--config", str(tmp_path / "no_such.yaml"), "validate-config"]
        )
        # validate-config 只读 args.config；缺文件时校验器应报错误而非抛异常
        assert "失败" in out or "错误" in out or code is not None

    def test_invalid_config_lists_errors(self, tmp_path, monkeypatch):
        """缺必填字段的配置需逐条列出错误路径"""
        monkeypatch.chdir(tmp_path)
        bad = tmp_path / "bad.yaml"
        bad.write_text(
            textwrap.dedent(
                """
                models:
                  default: ernie
                """
            ),
            encoding="utf-8",
        )
        out, err, code = run_cli(
            ["cli", "--config", str(bad), "validate-config"]
        )
        assert "失败" in out
        assert "app" in out  # 缺 app 段应被列为错误


class TestGlobalConfigRegression:
    """全局 --config 不得被子命令的同名参数静默覆盖（默认 None 会顶掉全局值）"""

    def test_global_config_respected_before_subcommand(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        valid = tmp_path / "good.yaml"
        valid.write_text(
            "app:\n  name: x\n  version: '1'\nmodels:\n  default: ernie\n",
            encoding="utf-8",
        )
        out, err, code = run_cli(
            ["cli", "--config", str(valid), "validate-config"]
        )
        # 全局 --config 指向的有效文件应被实际校验（而非回退到 CWD 的 config.yaml）
        assert "通过" in out, f"全局 --config 被忽略: {out} {err}"


class TestRepoConfigIntegrity:
    """仓库 config.yaml 与校验器的一致性（防止再次出现自带配置自校验失败）"""

    def test_shipped_config_app_section_present(self):
        """config.yaml 需含校验器要求的 app.name/app.version"""
        text = (AI_DIR / "config.yaml").read_text(encoding="utf-8")
        assert "app:" in text
        assert "name:" in text
        assert "version:" in text
