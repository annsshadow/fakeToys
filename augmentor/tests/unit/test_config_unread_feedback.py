# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A84：把「写了没人读」从诊断面接到产品面；A94：空/只注释的配置文件的档位

L52 把这条警告做进了 `validate_config`，可拼错键的人跑的是 `stats` / `generate` /
API 而不是 `validate-config`。L54 探针实测当时的形状：`stats --config typo.yaml`
**exit=0、stdout 正常、stderr 全空**，而那份配置的 `variants_per_seed` 仍是默认 5
—— 症状是「配置改了、值没变，且没人告诉我为什么」，唯一的找回拼写的路是读源码。
本轮把同一条判据接到 `load_config`（走 `logging.warning`，不碰 stdout、不碰退出码），
并补上 A94：0 字节或只含注释的 YAML 解析出 `None`，加载时抛
`TypeError: argument of type 'NoneType' is not iterable`。

A94 的用例**刻意不断言那句 TypeError 原文**：同一件事在两个解释器里文案不同
（`... is not iterable` / `... is not a container or iterable`，实测分别在
aug 3.13 与 C:\\Python314 上取到），断言原文就是把用例绑死在解释器版本上。
"""

import logging
import os
import subprocess
import sys
from pathlib import Path

import pytest

from augmentor.config import load_config
from augmentor.config_validator import (ConfigValidator,
                                        unread_key_messages,
                                        validate_config)

MARKER = ConfigValidator.UNREAD_MARKER

REPO = Path(__file__).resolve().parents[2]

#: 拼错一节 + 拼错一键：两条判据各命中一次
TYPO_YAML = (
    "models:\n"
    "  default: ernie\n"
    "  ernie:\n"
    "    type: baidu\n"
    "    model: x\n"
    "    temperatue: 0.5\n"
    "augmentation:\n"
    "  variant_per_seed: 999\n"
)

TYPO_DICT = {
    "models": {"default": "ernie",
               "ernie": {"type": "baidu", "model": "x", "temperatue": 0.5}},
    "augmentation": {"variant_per_seed": 999},
}


def _write(tmp_path, name, text):
    path = tmp_path / name
    path.write_text(text, encoding="utf-8", newline="\n")
    return str(path)


def _unread_records(caplog):
    return [r.getMessage() for r in caplog.records if MARKER in r.getMessage()]


class TestLoadConfigSaysUnreadKeys:
    """产品面出声：加载这一步就要把「没人读」说出来，而不是等诊断命令"""

    def test_typo_key_is_said_at_load_time(self, tmp_path, caplog):
        with caplog.at_level(logging.WARNING, logger="augmentor.config"):
            load_config(_write(tmp_path, "typo.yaml", TYPO_YAML))

        messages = _unread_records(caplog)
        assert any("augmentation.variant_per_seed" in m for m in messages), (
            f"拼错的键没在加载时出声，用户仍要靠读源码找回拼写：{messages}")
        assert any("是否想写 variants_per_seed" in m for m in messages), (
            "出声却不给候选拼写，等于把纠错成本还给用户")

    def test_it_is_only_a_warning_loaded_values_stay_default(self, tmp_path, caplog):
        """出声不改行为：`variant_per_seed: 999` 依旧不生效（999 也越界，改了才是双标）"""
        with caplog.at_level(logging.WARNING, logger="augmentor.config"):
            config = load_config(_write(tmp_path, "typo.yaml", TYPO_YAML))

        assert config.augmentation.variants_per_seed == 5
        assert config.web.port == 8000

    def test_shipped_config_emits_nothing(self, caplog):
        """出厂配置的代价必须是零：正常一次运行不多一行输出

        这条同时是「仓库自带配置没有拼错的键」的常驻守卫 —— 判据本身
        （`test_config_validator.py`）也报，但那要有人主动跑 validate-config 才看得到。
        """
        with caplog.at_level(logging.WARNING, logger="augmentor.config"):
            load_config(str(REPO / "config.yaml"))

        assert _unread_records(caplog) == []

    def test_correct_spelling_emits_nothing(self, tmp_path, caplog):
        with caplog.at_level(logging.WARNING, logger="augmentor.config"):
            config = load_config(
                _write(tmp_path, "good.yaml", "augmentation:\n  variants_per_seed: 7\n"))

        assert config.augmentation.variants_per_seed == 7
        assert _unread_records(caplog) == []


class TestTheTwoChannelsShareOneSource:
    """A77 的同族风险：同一条判据在两条通道各抄一遍，一边改了另一边不知道"""

    def test_validate_and_load_agree_on_entries(self):
        """诊断面条目数 == 产品面条目数 > 0：同源而不是抄写"""
        from_validator = [w.message for w in validate_config(TYPO_DICT).warnings
                          if MARKER in w.message]
        from_loader = unread_key_messages(TYPO_DICT)

        assert from_loader == from_validator != []

    def test_env_reference_findings_stay_out_of_the_load_noise(self, tmp_path, caplog):
        """加载时只出「没人读」这一件事，环境变量引用那一路不进产品面

        「环境变量未设置」的条数随本机 shell 而变（L52 的纪律）：把它接进每次加载，
        就等于让同一份配置在不同机器上输出不同行数。
        """
        path = _write(tmp_path, "env.yaml",
                      "models:\n  default: ernie\n  ernie:\n"
                      "    type: baidu\n    api_key: ${NOT_A_REAL_ENV_VAR_L54}\n")
        raw = {"models": {"default": "ernie",
                          "ernie": {"type": "baidu",
                                    "api_key": "${NOT_A_REAL_ENV_VAR_L54}"}}}
        assert any("环境变量未设置" in w.message
                   for w in validate_config(raw).warnings), "对照组：诊断面必须有这条"

        with caplog.at_level(logging.WARNING, logger="augmentor.config"):
            load_config(path)

        assert "环境变量未设置" not in caplog.text


class TestEmptyConfigFileYieldsDefaults:
    """A94：「什么都没写」与「路径不存在」必须是同一档位 —— 全默认，而不是崩"""

    @pytest.mark.parametrize("text", ["", "# 只有注释，没有任何键\n", "\n\n"])
    def test_no_content_yields_defaults(self, tmp_path, text):
        config = load_config(_write(tmp_path, "empty.yaml", text))

        assert config.augmentation.variants_per_seed == 5
        assert config.web.port == 8000

    def test_no_content_emits_no_unread_noise(self, tmp_path, caplog):
        """空文件没写任何键 ⇒ 也没什么可报的"""
        with caplog.at_level(logging.WARNING, logger="augmentor.config"):
            load_config(_write(tmp_path, "empty.yaml", ""))

        assert _unread_records(caplog) == []

    def test_missing_file_still_yields_defaults(self, tmp_path):
        """与空文件相邻的那一档不许被本轮改动带走"""
        assert load_config(str(tmp_path / "not_there.yaml")).augmentation.variants_per_seed == 5


class TestProductSurfaceReachesUser:
    """CLI 侧的可达性与契约：走 stderr、不动 stdout、不进退出码"""

    def _run(self, argv):
        from cli import main

        import io
        from contextlib import redirect_stderr, redirect_stdout

        old_argv = sys.argv
        sys.argv = ["cli"] + argv
        out, err = io.StringIO(), io.StringIO()
        code = None
        try:
            with redirect_stdout(out), redirect_stderr(err):
                main()
        except SystemExit as exc:
            code = exc.code
        finally:
            sys.argv = old_argv
        return out.getvalue(), err.getvalue(), code

    def test_stats_warns_without_touching_stdout_or_exit_code(self, tmp_path, caplog):
        """`stats` 必须真走到那条出声路径（不是只有 validate-config 才行）

        断言走 caplog 而不是 `err`：pytest 的日志插件在根挂了自己的 handler，
        进程内 `logging` 的默认 stderr 落地不会发生 —— 也就是说「文案进 stderr」
        不是本命令代码的契约，真正的 stdout 契约由下面两行守住。
        """
        import json

        data = tmp_path / "d.json"
        data.write_text('[{"instruction": "a", "output": "b"}]', encoding="utf-8")

        with caplog.at_level(logging.WARNING, logger="augmentor.config"):
            out, err, code = self._run(
                ["--config", _write(tmp_path, "typo.yaml", TYPO_YAML),
                 "stats", "--input", str(data)])

        assert any("augmentation.variant_per_seed" in m for m in _unread_records(caplog)), (
            "产品命令没走到加载出声：反馈仍锁在诊断命令里")
        assert code is None, "WARNING 不判负（L53 口径），拼错键不许挡红"
        assert json.loads(out)["total_items"] == 1, "stdout 是命令的输出契约，一行不许多"

    def test_stderr_carries_the_warning_in_a_clean_process(self, tmp_path):
        """用户看到的这一面：未配置 logging 时文案确实落到 stderr

        子进程是必须的：进程内 pytest 的日志 handler 会接管输出（见上一个用例），
        只有干净进程才能验到「默认落地 = stderr」这条用户可见形状。
        """
        data = tmp_path / "d.json"
        data.write_text('[{"instruction": "a", "output": "b"}]', encoding="utf-8")
        proc = subprocess.run(
            [sys.executable, str(REPO / "cli.py"),
             "--config", _write(tmp_path, "typo.yaml", TYPO_YAML),
             "stats", "--input", str(data)],
            capture_output=True, text=True, encoding="utf-8", errors="replace",
            cwd=str(REPO), env=dict(os.environ, PYTHONIOENCODING="utf-8"),
        )

        assert proc.returncode == 0, proc.stderr
        assert MARKER in proc.stderr, f"干净进程里 stderr 没有该文案：{proc.stderr!r}"
        assert MARKER not in proc.stdout
        assert '"total_items": 1' in proc.stdout
