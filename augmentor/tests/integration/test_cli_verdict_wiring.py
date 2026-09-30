# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI「判决 → 退出码」接线守卫（L55，承 L53 §3.28 立的口径）

L53 把口径写进了文档，但文档不是守卫：谁新增一条有判决位的命令却忘了接退出码，
或者接了却没在 `--help` 里说，CI 一声不响。本文件从 `COMMANDS` 和 `build_parser()`
**双向推导**出「声明集合 / 接线集合 / 宣称集合」并两两对账 ⇒ 口径漂移当场变红。

三条形状口径（见 `augmentor/cli/verdict.py`）：`0` = 通过或本命令不判负，
`1` = 判决未通过（只由 `verdict_exit()` 产生），`2` = argparse 用法错误。
"""

import argparse
import inspect
import io
import json
import os
import subprocess
import sys
import types
from contextlib import redirect_stderr, redirect_stdout
from pathlib import Path

import pytest

from cli import main

from augmentor.cli.commands import COMMANDS
from augmentor.cli.parser import build_parser
from augmentor.cli.verdict import verdict_exit

AI_DIR = Path(__file__).resolve().parent.parent.parent

# 质量报告在这份语料上：阈值 1.0 ⇒ 总体状态未通过，阈值 0.01 ⇒ 通过（L55 实测）
QUALITY_ITEMS = [
    {"instruction": "请写一首关于秋天的短诗，并注明作者。",
     "output": "这是一首关于秋天的短诗，作者：某某。这是一首关于秋天的短诗，作者：某某。"},
    {"instruction": "Explain the difference between a list and a tuple in one sentence.",
     "output": "A list is mutable while a tuple is immutable."
               "A list is mutable while a tuple is immutable."},
]

# 带 PII 的单条数据 ⇒ audit 判「未就绪」
PII_ITEMS = [
    {"instruction": "请联系我", "output": "我的手机号是 13800138000，邮箱 foo@example.com"},
]

# 训练/测试集交集非空 ⇒ check-leakage 判「有泄漏」
LEAK_TRAIN = [
    {"instruction": "完全相同的一条训练样本用于制造泄漏", "output": "同样的输出文本同样的输出文本同样的输出文本"},
]
LEAK_TEST = [
    {"instruction": "完全相同的一条训练样本用于制造泄漏", "output": "同样的输出文本同样的输出文本同样的输出文本"},
]

# 空 instruction + 重复 ⇒ auto-test 的「失败: N」非零（L55 实测 5 项检查里失败 3 项）
AUTOTEST_BAD = [
    {"instruction": "", "input": "", "output": ""},
    {"instruction": "同一条", "input": "", "output": "a"},
    {"instruction": "同一条", "input": "", "output": "a"},
]

# L53 就接好退出码的四条校验形命令：它们的接线不许被后来的改动悄悄摘掉
L53_WIRED = {"validate", "validate-config", "dependency", "health-gate"}


def _run_cli(argv):
    """在进程内跑一次 CLI，返回 (stdout, stderr, SystemExit 码或 None)

    Args:
        argv: 完整参数列表（含程序名）

    Returns:
        (stdout 文本, stderr 文本, 退出码或 None)
    """
    old_argv = sys.argv
    sys.argv = argv
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


def _run_gbk(argv):
    """强制子进程 stdio 走 GBK 跑一次 CLI（不依赖本机 locale，Linux 上同样成立）

    Args:
        argv: 子命令及其参数（不含程序名）

    Returns:
        (退出码, 按 GBK 解码的 stdout)
    """
    env = dict(os.environ)
    env["PYTHONIOENCODING"] = "gbk"
    proc = subprocess.run([sys.executable, "cli.py"] + argv, cwd=str(AI_DIR),
                          capture_output=True, env=env)
    return proc.returncode, proc.stdout.decode("gbk", "replace")


def _source(handler):
    """取 handler 源码文本（推导接线集合用）

    Args:
        handler: COMMANDS 里的一个 handler

    Returns:
        源码字符串
    """
    return inspect.getsource(handler)


def _wired_commands(fragment):
    """handler 源码里出现给定片段的命令名集合

    Args:
        fragment: 要检索的源码片段

    Returns:
        命令名集合
    """
    return {name for name, handler in COMMANDS.items() if fragment in _source(handler)}


def _subparsers_action():
    """从 build_parser() 里取出子命令解析器动作（不写死它是第几个 _action）

    Returns:
        argparse 的子命令 action，其 `.choices` 是 name → 子解析器
    """
    for action in build_parser()._actions:
        choices = getattr(action, "choices", None)
        if isinstance(choices, dict) and choices and all(
            isinstance(v, argparse.ArgumentParser) for v in choices.values()
        ):
            return action
    raise AssertionError("build_parser() 里找不到子命令解析器 ⇒ 本文件的推导失效")


def _subcommand_helps():
    """子命令名 → 其在 `--help` 清单里的一行说明

    Returns:
        name → help 文本（未写 help 时为 None）
    """
    return {sa.dest: sa.help for sa in _subparsers_action()._get_subactions()}


def _all_help_strings():
    """CLI 会经 stdout 印出去的全部说明文本，形如 (出处, help)

    Returns:
        [(出处标签, help 文本)]，help 可能为 None
    """
    action = _subparsers_action()
    helps = _subcommand_helps()
    targets = [(name, helps.get(name)) for name in action.choices]
    targets.append(("<root>", _subparsers_action().help))
    for name, sub_parser in action.choices.items():
        for opt in (o for a in sub_parser._actions for o in a.option_strings):
            arg_action = next(a for a in sub_parser._actions if opt in a.option_strings)
            targets.append((f"{name} {opt}", arg_action.help))
    return targets


@pytest.fixture
def corpus(tmp_path):
    """把判决语料写到临时目录

    Args:
        tmp_path: pytest 临时目录

    Returns:
        名称 → 文件路径 的字典
    """
    paths = {}
    for name, payload in (
        ("quality", QUALITY_ITEMS),
        ("pii", PII_ITEMS),
        ("train", LEAK_TRAIN),
        ("test", LEAK_TEST),
        ("autotest_bad", AUTOTEST_BAD),
    ):
        path = tmp_path / f"{name}.json"
        path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")
        paths[name] = path
    return paths


@pytest.fixture(autouse=True)
def _repo_cwd(monkeypatch):
    """让 load_config 找到仓库 config.yaml，与既有 CLI 测试同一前提"""
    monkeypatch.chdir(AI_DIR)


class TestWiringIsDerived:
    """接线与宣称必须双向相等 —— 这是 A91 要的机械守卫"""

    def test_gate_flag_and_enforce_argument_agree(self):
        declared = {
            name for name, sub in _subparsers_action().choices.items()
            if "--gate" in {o for a in sub._actions for o in a.option_strings}
        }
        wired = _wired_commands("enforce=args.gate")
        assert declared, "派生结果为空 ⇒ 守卫失效，不是通过"
        assert declared == wired, (
            f"声明了 --gate 却没接线: {sorted(declared - wired)}；"
            f"接了却没声明（用户无从得知）: {sorted(wired - declared)}"
        )

    def test_wired_command_declares_exit_code_in_help(self):
        wired = _wired_commands("verdict_exit(")
        declared = {name for name, h in _subcommand_helps().items() if h and "退出码" in h}
        assert wired, "派生结果为空 ⇒ 守卫失效，不是通过"
        assert wired == declared, (
            f"接了退出码却不宣称: {sorted(wired - declared)}；"
            f"宣称了却没接线（文档撒谎）: {sorted(declared - wired)}"
        )

    def test_l53_validation_commands_stay_wired(self):
        wired = _wired_commands("verdict_exit(")
        missing = L53_WIRED - wired
        assert not missing, f"L53 立口径的四条命令被摘掉了接线: {sorted(missing)}"

    @pytest.mark.parametrize("passed,enforce,expected", [
        (True, True, None),
        (False, True, 1),
        (True, False, None),
        (False, False, None),
    ])
    def test_verdict_exit_shapes(self, passed, enforce, expected):
        if expected is None:
            verdict_exit(passed, enforce)
        else:
            with pytest.raises(SystemExit) as excinfo:
                verdict_exit(passed, enforce)
            assert excinfo.value.code == expected


class TestGateBehavior:
    """报告形命令：默认恒 0（L53 口径），`--gate` 才把判决翻译成退出码"""

    @pytest.mark.parametrize("threshold,expected", [("1.0", 1), ("0.01", None)])
    def test_quality_report_gate(self, corpus, threshold, expected):
        argv = ["cli", "quality-report", "--input", str(corpus["quality"]),
                "--threshold", threshold, "--gate"]
        out, _, code = _run_cli(argv)
        assert code == expected
        if expected == 1:
            assert "总体状态: 未通过" in out

    def test_quality_report_default_never_fails(self, corpus):
        out, _, code = _run_cli(["cli", "quality-report", "--input", str(corpus["quality"]),
                                 "--threshold", "1.0"])
        assert code is None
        assert "总体状态: 未通过" in out, "报告要出完，只是不落退出码"

    def test_audit_default_reports_but_never_fails(self, corpus):
        out, _, code = _run_cli(["cli", "audit", "--input", str(corpus["pii"])])
        assert code is None
        assert "数据集未就绪" in out, "报告仍要出完，只是不落退出码"

    @pytest.mark.parametrize("input_key,expected", [("pii", 1), ("quality", None)])
    def test_audit_gate(self, corpus, input_key, expected):
        _, _, code = _run_cli(["cli", "audit", "--input", str(corpus[input_key]), "--gate"])
        assert code == expected

    @pytest.mark.parametrize("train_key,test_key,expected", [
        ("train", "test", 1),
        ("quality", "autotest_bad", None),
    ])
    def test_check_leakage_gate_and_default(self, corpus, train_key, test_key, expected):
        argv = ["cli", "check-leakage", "--train", str(corpus[train_key]),
                "--test", str(corpus[test_key])]
        out, _, code = _run_cli(argv + ["--gate"])
        assert code == expected
        if expected == 1:
            assert "泄漏" in out
        _, _, default_code = _run_cli(argv)
        assert default_code is None, "不带 --gate 时报告形命令恒 0"

    def test_auto_test_gate_fails_on_bad_dataset(self, corpus):
        out, _, code = _run_cli(["cli", "auto-test", "--input", str(corpus["autotest_bad"]),
                                 "--gate"])
        assert code == 1
        assert "[失败]" in out

    def test_auto_test_default_never_fails_on_bad_dataset(self, corpus):
        _, _, code = _run_cli(["cli", "auto-test", "--input", str(corpus["autotest_bad"])])
        assert code is None, "「失败: 3」照样只出报告 ⇒ L53 口径不被本次接线推翻"

    def _stub_doctor(self, monkeypatch, all_required_present):
        """桩掉 check_dependencies：必需依赖齐不齐随本机环境而变，不进断言

        Args:
            monkeypatch: pytest fixture
            all_required_present: 桩的判决位
        """
        stub = types.SimpleNamespace(installed={"pandas": not all_required_present},
                                     missing=[] if all_required_present else ["pandas"],
                                     degraded_features=[], available_count=1,
                                     all_required_present=all_required_present)
        monkeypatch.setattr("augmentor.diagnostics.check_dependencies", lambda: stub)

    @pytest.mark.parametrize("required,expected", [(False, 1), (True, None)])
    def test_doctor_gate_follows_required_dependencies(self, monkeypatch, required, expected):
        self._stub_doctor(monkeypatch, required)
        _, err, code = _run_cli(["cli", "doctor", "--gate"])
        assert code == expected, f"判决 {required} 应得退出码 {expected}，实得 {code}；stderr={err[:120]!r}"

    def test_doctor_default_never_fails(self, monkeypatch):
        self._stub_doctor(monkeypatch, False)
        _, _, code = _run_cli(["cli", "doctor"])
        assert code is None

    def test_doctor_gate_uses_real_dependency_probe_not_stub(self, monkeypatch):
        """A100：`doctor --gate` 的非零判决要由真实 SDK 算出，不靠桩

        `_stub_doctor` 直接给了 `all_required_present` 字段，等于把判决位钉死在桩上——
        一旦真实 `DiagnosticsReport.all_required_present` 的属性实现或 `run_doctor` 的
        接线退化，那两条桩用例仍然绿。这里把 `REQUIRED_DEPENDENCIES` 换成一个永不存在的
        模块名，让真实的 `check_dependencies()` 自己算出 `False`（`installed.get(名, False)`
        与本机装了什么无关，故确定性成立），再断言 CLI 落到退出码 1。

        A100 原行写「真子进程用例」，但子进程无法在不新增覆盖旋钮（＝行为变更）的前提下
        被强制判负，且 yaml 在不在本机随环境而变——违反「不靠本机装了什么」。确定性意图
        （真实 SDK 给 False、非零，不靠桩）在进程内即可达成。
        """
        from augmentor.diagnostics import DiagnosticsReport, check_dependencies

        monkeypatch.setattr("augmentor.diagnostics.REQUIRED_DEPENDENCIES",
                            ["definitely_absent_module_for_l69"])
        report = check_dependencies()
        assert isinstance(report, DiagnosticsReport), "必须走真实诊断，不是 SimpleNamespace 桩"
        assert report.all_required_present is False, (
            "真实属性实现须自己算出 False——若这里为 True，说明判决位被别处短路了"
        )

        _, err, code = _run_cli(["cli", "doctor", "--gate"])
        assert code == 1, f"真实诊断为负应得退出码 1，实得 {code}；stderr={err[:120]!r}"

    def _stub_migrate(self, monkeypatch, failed_items):
        """桩掉 migrate_file：造一份「迁移失败 N 条」的判决，不依赖迁移器多宽容

        Args:
            monkeypatch: pytest fixture
            failed_items: 桩的失败条数
        """
        stub = types.SimpleNamespace(migration_id="stub", total_items=2,
                                     migrated_items=2 - failed_items, failed_items=failed_items,
                                     rules_applied=[],
                                     errors=[{"index": 1, "error": "桩"}] if failed_items else [])
        monkeypatch.setattr("augmentor.migration.migrate_file", lambda *a, **k: stub)

    @pytest.mark.parametrize("failed,expected", [(1, 1), (0, None)])
    def test_migrate_gate_tracks_failed_items(self, corpus, monkeypatch, tmp_path, failed, expected):
        self._stub_migrate(monkeypatch, failed)
        _, err, code = _run_cli(["cli", "migrate", "--input", str(corpus["quality"]),
                                 "--output", str(tmp_path / "migrated.json"), "--gate"])
        assert code == expected, f"失败 {failed} 条应得退出码 {expected}，实得 {code}；stderr={err[:120]!r}"

    def test_gate_is_rejected_on_command_without_verdict(self, corpus):
        _, err, code = _run_cli(["cli", "stats", "--input", str(corpus["quality"]), "--gate"])
        assert code == 2, "无判决位的命令不认识 --gate，必须是用法错误（2）不是崩溃（1）"
        assert "unrecognized arguments" in err


class TestGbkPipeIsSafe:
    """CLI 印出去的话必须活得过 GBK 管道 —— L55 实测 quality-report / auto-test 曾崩在这里"""

    def test_every_help_string_is_gbk_encodable(self):
        bad = []
        for label, help_text in _all_help_strings():
            if not help_text:
                continue
            try:
                help_text.encode("gbk")
            except UnicodeEncodeError as exc:
                bad.append(f"{label}: {exc}")
        assert not bad, "help 文本里有 GBK 编不出的字符，`--help` 会在 GBK 管道下崩成 rc=1：\n" + "\n".join(bad)

    @pytest.mark.parametrize("cmd", ["quality-report", "doctor", "check-leakage"])
    def test_help_survives_a_gbk_pipe(self, cmd):
        # 编码崩溃的退出码也是 1 ⇒ 判决与崩溃靠「stdout 有没有内容」分，这条测的就是出得来话
        rc, text = _run_gbk([cmd, "-h"])
        assert rc == 0
        assert text.strip(), f"{cmd} -h 在 GBK 管道下什么都没印出来"

    def test_quality_report_survives_gbk_pipe(self, corpus, tmp_path):
        report = tmp_path / "gbk_quality.json"
        rc, text = _run_gbk(["quality-report", "--input", str(corpus["quality"]),
                          "--threshold", "1.0", "--output", str(report)])
        assert rc == 0, "未接 --gate 时报告形命令恒 0，GBK 管道下也一样"
        assert "总体状态" in text, "整份报告要出得来，不能崩在半张上"
        assert json.loads(report.read_text(encoding="utf-8"))["overall_passed"] is False

    def test_quality_report_gate_still_decides_under_gbk(self, corpus):
        rc, text = _run_gbk(["quality-report", "--input", str(corpus["quality"]),
                          "--threshold", "1.0", "--gate"])
        assert rc == 1
        assert "总体状态" in text, "退出码 1 必须来自判决，不能来自编码崩溃"

    def test_auto_test_survives_gbk_pipe(self, corpus, tmp_path):
        report = tmp_path / "gbk_autotest.json"
        rc, text = _run_gbk(["auto-test", "--input", str(corpus["autotest_bad"]),
                        "--output", str(report)])
        assert rc == 0
        assert "通过:" in text, "改前这条命令在 GBK 管道下 stdout 是 0 字节"
        assert report.exists() and report.stat().st_size > 0
