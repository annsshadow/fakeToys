# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

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
        """缺必填字段的配置需逐条列出错误路径

        缺的必须是**真正必填**的字段。`models.default` 是 `load_config` 读取默认
        模型的唯一来源，缺了它会静默回落到 `ernie`，所以它是必填项。
        （`app` / `app.name` / `app.version` 已降级为可选：`AppConfig` 没有对应
        字段，也没有任何代码读它们，把它们设为必填会让 `save_config` 写出的配置
        必然「不合法」。）
        """
        monkeypatch.chdir(tmp_path)
        bad = tmp_path / "bad.yaml"
        bad.write_text(
            textwrap.dedent(
                """
                app:
                  name: ai-augmentor
                  version: '1.0'
                models: {}
                """
            ),
            encoding="utf-8",
        )
        out, err, code = run_cli(
            ["cli", "--config", str(bad), "validate-config"]
        )
        assert "失败" in out
        assert "models.default" in out  # 缺 models.default 应被列为错误


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


class TestValidateConfigVerdictIsMachineReadable:
    """A89：`validate-config` 打出的判决必须同时体现在退出码上

    这个命令除了 stdout 没有别的机器可读出口，退出码就是脚本 / CI 唯一能读的判决位。
    口径与 `health-gate` 一致：**只在判负时** `sys.exit(1)`，通过时不抛 SystemExit
    ——上面 `test_repo_config_is_valid` 的 `assert code is None` 就是这条契约。
    """

    def test_reported_errors_exit_one(self, tmp_path, monkeypatch):
        """报 `错误: N`（N>0）的配置不得 exit 0，否则拿它当门禁会永远绿"""
        monkeypatch.chdir(tmp_path)
        bad = tmp_path / "bad.yaml"
        bad.write_text(
            "app:\n  name: x\n  version: '1'\nmodels: {}\n", encoding="utf-8"
        )
        out, err, code = run_cli(["cli", "--config", str(bad), "validate-config"])
        assert "失败" in out
        assert code == 1, f"判决是失败却退出 {code!r}"

    def test_warnings_only_stay_clean(self, tmp_path, monkeypatch):
        """只有 WARNING（含 L52 的「写了没人读」）时不判负"""
        monkeypatch.chdir(tmp_path)
        warn = tmp_path / "warn.yaml"
        warn.write_text(
            "app:\n  name: x\n  version: '1'\nmodels:\n  default: ernie\n"
            "augmenation:\n  max_retries: 3\n",
            encoding="utf-8",
        )
        out, err, code = run_cli(["cli", "--config", str(warn), "validate-config"])
        assert "配置验证结果: 通过" in out
        assert "错误: 0" in out
        assert "augmenation" in out  # 本轮注入的那个假节名必须自己出声
        assert code is None, "多余键只该出声，不该挡红"

    def test_loader_rejection_reports_error_instead_of_traceback(self, tmp_path, monkeypatch):
        """配置坏到 `load_config` 拒收时也得走「错误: …」+ exit 1，而不是裸 traceback"""
        monkeypatch.chdir(tmp_path)
        hostile = tmp_path / "hostile.yaml"
        hostile.write_text(
            "app:\n  name: x\n  version: '1'\nmodels:\n  default: ernie\n"
            "web:\n  port: 99999\n",
            encoding="utf-8",
        )
        out, err, code = run_cli(["cli", "--config", str(hostile), "validate-config"])
        assert code == 1
        assert err.startswith("错误: ")
        assert "Traceback" not in err
        assert out == ""


class TestRepoConfigIntegrity:
    """仓库 config.yaml 与校验器的一致性（防止再次出现自带配置自校验失败）"""

    def test_shipped_config_app_section_present(self):
        """config.yaml 需含校验器要求的 app.name/app.version"""
        text = (AI_DIR / "config.yaml").read_text(encoding="utf-8")
        assert "app:" in text
        assert "name:" in text
        assert "version:" in text
