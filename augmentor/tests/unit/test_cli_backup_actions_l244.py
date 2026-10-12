# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L244：`backup --action` 补上 `verify` / `cleanup` 两档，三面（SDK / HTTP / CLI）同源钉死

**收 A329**（原立 L243，走修法 ①）。改前实测：`build_parser()` 里 backup 的 `choices`
现量是 `{create, restore, list, delete}` 四档，而 SDK 有六只入口、HTTP 面自 L243 起也有
六员 ⇒ 「清理旧备份」这一族里最该无人值守的那一半只有页面能点，脚本与 cron 够不着。

**三面同源判据为什么写成双射而不是清单**：清单（「CLI 有这六档」）只防得住「CLI 少一档」，
防不住「CLI 多一档而没人接」与「HTTP 加了第 7 员而 CLI 不知情」——后两条正是 A329 这一族
的成因。所以本文件把三个方向各自钉死：

1. CLI 的 `choices` 与 `run_backup` 源码里的分支集合**两个方向相等**（AST/文本双查）；
2. `choices` 与本文件的 `ACTION_TO_ENDPOINT` 键集相等，而该表值集与 OpenAPI 里以
   `/api/system/backups` 开头的**全部**操作集相等 ⇒ 契约面多一条端点、这张表少一格，当场红；
3. 每档调用的 SDK 入口与对应端点路由调用的那只**同名**（读两侧源码的 import 名单）。

**退出码口径**（`augmentor/cli/verdict.py` 三种形状，L53 立 / L55 接线）：`verify` 是校验形
命令 ⇒ 恒判负，查到损坏就是 `1`，且 stderr **没有**「错误:」那一行（有「错误:」的 1 是崩溃，
两者必须能分）；索引里没有匹配项 ⇒ 打印「没有找到备份」并退出 `0`，与 HTTP 面「幽灵 id 回空
列表」同档（既不判坏也不报错）；`cleanup` 的坏份数由 `require_count` 抛 `ValueError`，经
`cli.py` 的收尾翻译成「错误: …」+ `1`，且**判参先于建目录**（L34 口径④，实测断言目录不在场）；
不认识的 `--action` 由 argparse 给 `2`。

**注入三臂**（零写入通道，全部打在导入后的模块对象上）见
`TestInjectionArmsKeepTheGuardHonest`：摘 SDK 入口 / 摘判决 / 摘清理，各自指名要红的那一支。

跑法：`python -m pytest tests/unit/test_cli_backup_actions_l244.py -q --no-cov`
"""

import inspect
import io
import json
import re
import sys
from contextlib import redirect_stderr, redirect_stdout

import pytest

from cli import main

from augmentor.cli.commands import COMMANDS
from augmentor.cli.commands.version import run_backup
from augmentor.cli.parser import build_parser

ACTIONS = {"create", "restore", "list", "delete", "verify", "cleanup"}

#: CLI 那一档 ↔ HTTP 面那一只端点。一一对应，谁多谁少都红
ACTION_TO_ENDPOINT = {
    "create": ("POST", "/api/system/backups"),
    "restore": ("POST", "/api/system/backups/{backup_id}/restore"),
    "list": ("GET", "/api/system/backups"),
    "delete": ("DELETE", "/api/system/backups/{backup_id}"),
    "verify": ("GET", "/api/system/backups/verify"),
    "cleanup": ("POST", "/api/system/backups/cleanup"),
}

#: CLI 那一档 ↔ 两侧共同调用的 SDK 入口名（`augmentor/backup.py` 的模块级函数）
ACTION_TO_SDK = {
    "create": "create_backup",
    "restore": "restore_backup",
    "list": "list_backups",
    "delete": "delete_backup",
    "verify": "verify_backup_integrity",
    "cleanup": "clean_old_backups",
}

BACKUP_PREFIX = "/api/system/backups"


def _run_cli(argv):
    """以给定参数跑 `main()`，回 (stdout, stderr, 退出码或 None)

    Args:
        argv: 完整参数列表（含程序名）

    Returns:
        (stdout, stderr, exit_code)
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


def _backup_subparser():
    """从 `build_parser()` 里取 backup 的子解析器（不写死它是第几个 `_action`）"""
    for action in build_parser()._actions:
        choices = getattr(action, "choices", None)
        if isinstance(choices, dict) and "backup" in choices:
            return choices["backup"]
    raise AssertionError("build_parser() 里没有 backup 子命令 ⇒ 本文件的推导失效")


def _cli_choices():
    """CLI 的 `--action` choices 现量集合"""
    for arg_action in _backup_subparser()._actions:
        if arg_action.dest == "action":
            return set(arg_action.choices)
    raise AssertionError("backup 子命令没有 --action ⇒ 本文件的推导失效")


def _cli_option_dests():
    """backup 子命令的全部选项 dest（钉 `--max-backups` 在场）"""
    return {a.dest for a in _backup_subparser()._actions}


def _handler_branches():
    """`run_backup` 源码里的 `args.action == "…"` 分支名集合"""
    return set(re.findall(r'args\.action == "([a-z]+)"', inspect.getsource(run_backup)))


def _imported_names(func):
    """函数体里 `from augmentor.backup import …` 导入的名字集合（AST 免读行号）"""
    import ast

    tree = ast.parse(inspect.getsource(func))
    names = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.ImportFrom) and node.module == "augmentor.backup":
            names |= {alias.name for alias in node.names}
    return names


def _backup_endpoint_operations():
    """OpenAPI 里备份一族的全部 (方法, 路径)"""
    from api.main import app

    paths = app.openapi()["paths"]
    return {
        (method.upper(), path)
        for path, spec in paths.items()
        if path.startswith(BACKUP_PREFIX)
        for method in spec
    }


def _subcommand_help(name):
    """子命令在 `--help` 清单里那一行的说明（与 l55 那把尺子同一派生法，不写死位置）"""
    for action in build_parser()._actions:
        choices = getattr(action, "choices", None)
        if isinstance(choices, dict) and name in choices:
            return {sa.dest: sa.help for sa in action._get_subactions()}[name]
    raise AssertionError(f"build_parser() 里没有 {name} 子命令 ⇒ 本文件的推导失效")


def _sandbox(tmp_path, monkeypatch):
    """临时工作目录 + 一份 1 条的源文件，回 (源文件路径, 备份目录)"""
    monkeypatch.chdir(tmp_path)
    src = tmp_path / "p.json"
    src.write_text(
        json.dumps([{"instruction": "q", "input": "", "output": "a"}]), encoding="utf-8"
    )
    return src, tmp_path / "bk"


def _create(src, backup_dir, name):
    out, err, code = _run_cli([
        "cli", "backup", "--action", "create", "--input", str(src),
        "--name", name, "--backup-dir", str(backup_dir),
    ])
    assert code is None, err
    return out


class TestThreeFacesAgree:
    """三个方向各自钉死：少一档、多一档、档与端点错配，都当场红"""

    def test_cli_choices_are_exactly_the_six(self):
        assert _cli_choices() == ACTIONS

    def test_choices_and_handler_branches_agree_both_ways(self):
        """解析器加了档而 handler 没接（跑起来落空分支）⇔ 红；反之也红"""
        assert _handler_branches() == ACTIONS, (
            f"分支缺: {sorted(ACTIONS - _handler_branches())}；"
            f"分支多（choices 里认不了）: {sorted(_handler_branches() - ACTIONS)}"
        )

    def test_cli_actions_and_the_endpoint_map_have_the_same_keys(self):
        assert set(ACTION_TO_ENDPOINT) == _cli_choices()

    def test_the_map_is_a_bijection_onto_every_backup_endpoint_in_the_contract(self):
        """双射：这张表的值集与契约面备份一族的全部操作集**精确相等**

        契约面多出第 7 员（比如日后给 verify 加一只按 id 的变体）而这张表没跟上 ⇒ 红；
        表里指向一条不存在的端点 ⇒ 同样红。
        """
        operations = _backup_endpoint_operations()
        assert len(set(ACTION_TO_ENDPOINT.values())) == len(ACTION_TO_ENDPOINT), "表里两条档指同一只端点"
        assert set(ACTION_TO_ENDPOINT.values()) == operations, (
            f"契约面有而 CLI 没有: {sorted(operations - set(ACTION_TO_ENDPOINT.values()))}；"
            f"CLI 有而契约面没有: {sorted(set(ACTION_TO_ENDPOINT.values()) - operations)}"
        )

    def test_each_action_calls_the_same_sdk_entry_as_its_endpoint(self):
        """CLI 分支 import 的名单与端点路由 import 的名单都覆盖同一组六只函数"""
        from api.routes import system_ops

        cli_names = _imported_names(run_backup)
        route_names = set()
        for func in vars(system_ops).values():
            if callable(func) and getattr(func, "__name__", "").endswith("_endpoint"):
                route_names |= _imported_names(func)
        assert cli_names >= set(ACTION_TO_SDK.values())
        assert route_names >= set(ACTION_TO_SDK.values())
        assert COMMANDS["backup"] is run_backup

    def test_max_backups_default_is_the_same_number_on_all_three_faces(self):
        """`--max-backups` 的 10 == HTTP 请求模型的 10 == SDK 形参的 10"""
        import inspect as _inspect

        from api.routes.system_ops import BackupCleanupRequest
        from augmentor.backup import clean_old_backups

        assert _cli_option_dests() >= {"max_backups"}
        default = next(
            a for a in _backup_subparser()._actions if a.dest == "max_backups"
        ).default
        assert default == 10
        assert BackupCleanupRequest.model_fields["max_backups"].default == 10
        assert _inspect.signature(clean_old_backups).parameters[
            "max_backups"
        ].default == 10

    def test_the_subcommand_help_declares_the_exit_code(self):
        """`verdict.py` 的口径：接了退出码却没在 `--help` 里说 = 文档撒谎（l55 那把尺子同族）"""
        assert "退出码" in _subcommand_help("backup")


class TestCliVerifyVerdict:
    """`--action verify`：完好 ⇒ 0、坏 ⇒ 1 且不带「错误:」、幽灵 ⇒ 0"""

    def test_all_good_backups_exit_zero(self, tmp_path, monkeypatch):
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")

        out, err, code = _run_cli(["cli", "backup", "--action", "verify", "--backup-dir", str(bd)])

        assert code is None, f"{out}{err}"
        lines = out.splitlines()
        assert len(lines) == 2, out
        assert lines[0].startswith("  - a: 完好（1 条，") and lines[0].endswith("）"), lines[0]
        assert lines[1] == "1 个备份全部完好"
        assert "错误:" not in err

    def test_a_corrupt_backup_is_verdict_one_not_a_crash(self, tmp_path, monkeypatch):
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")
        (bd / "a.json").write_text(
            json.dumps([{"instruction": "被改坏的内容"}]), encoding="utf-8"
        )

        out, err, code = _run_cli(["cli", "backup", "--action", "verify", "--backup-dir", str(bd)])

        assert code == 1, f"{out}{err}"
        assert "校验和不匹配" in out and "1 个不正常: a" in out
        # 判决与崩溃必须分得开（verdict.py 口径）：崩溃的 1 前面有「错误:」
        assert "错误:" not in err, "拿崩溃冒充判决 ⇒ 退出码口径又糊了"

    def test_a_zero_byte_file_is_reported_as_lost_and_still_fails(self, tmp_path, monkeypatch):
        """与 HTTP 面同档：零字节算「文件丢了」，不是「对不上」，两者在输出里分列"""
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")
        (bd / "a.json").write_text("", encoding="utf-8")

        out, _err, code = _run_cli(["cli", "backup", "--action", "verify", "--backup-dir", str(bd)])

        assert code == 1
        assert "文件丢失或为空" in out and "校验和不匹配" not in out

    def test_ghost_name_exits_zero_with_no_backup_found(self, tmp_path, monkeypatch):
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")

        out, _err, code = _run_cli([
            "cli", "backup", "--action", "verify", "--name", "ghost", "--backup-dir", str(bd),
        ])

        assert code is None
        assert out == "没有找到备份（索引里没有 ghost）\n"

    def test_a_missing_dir_stays_absent_and_reports_no_backup(self, tmp_path, monkeypatch):
        """L249 收 A332：CLI 的只读档不再把备份目录建出来，与 HTTP 面同源

        改前实测：CLI 这一侧没有路由那道 `if not resolved.exists()` 挡板，而
        `DatasetBackup` 的构造无条件 mkdir 并落一份 index.json ⇒ 同一次「查一下有没有
        坏的」在 CLI 与 HTTP 两面留下**不同的磁盘状态**。修法不是给 CLI 加挡板，而是把
        副作用从构造里摘掉（A307 那一族同批受益），所以三面现在都由库里同源。
        """
        bd = tmp_path / "nodir"
        monkeypatch.chdir(tmp_path)

        out, _err, code = _run_cli(["cli", "backup", "--action", "verify", "--backup-dir", str(bd)])

        assert code is None
        assert out == "没有找到备份\n"
        assert not bd.exists(), "只读的 verify 把备份目录建出来了 ⇒ A332 又犯"
        assert list(tmp_path.iterdir()) == [], "只读的一趟在别处留下了东西"


class TestCliCleanup:
    """`--action cleanup`：按 `--max-backups` 留最新的，坏份数判参先于副作用"""

    def test_keeping_one_deletes_the_older_rows(self, tmp_path, monkeypatch):
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")
        _create(src, bd, "b")

        out, err, code = _run_cli([
            "cli", "backup", "--action", "cleanup", "--max-backups", "1", "--backup-dir", str(bd),
        ])

        assert code is None, f"{out}{err}"
        assert out == "清理完成: 删除 1 个，保留最新 1 个\n"
        rows = json.loads((bd / "index.json").read_text(encoding="utf-8"))["backups"]
        assert [r["backup_id"] for r in rows] == ["b"]

    def test_the_server_default_keeps_everything_when_there_are_fewer(self, tmp_path, monkeypatch):
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")

        out, _err, code = _run_cli(["cli", "backup", "--action", "cleanup", "--backup-dir", str(bd)])

        assert code is None
        assert out == "清理完成: 删除 0 个，保留最新 10 个\n"

    @pytest.mark.parametrize("bad,value", [("zero", "0"), ("negative", "-1")])
    def test_bad_counts_exit_one_before_touching_the_disk(self, tmp_path, monkeypatch, bad, value):
        """`require_count` 的判决经 `cli.py` 收尾成「错误: …」+ 1，且目录**不在场**（L34 口径④）"""
        bd = tmp_path / "bk"
        monkeypatch.chdir(tmp_path)

        _out, err, code = _run_cli([
            "cli", "backup", "--action", "cleanup", "--max-backups", value,
            "--backup-dir", str(bd),
        ])

        assert code == 1
        assert err.strip() == f"错误: max_backups 必须是不小于 1 的整数，当前是 {int(value)}"
        assert not bd.exists(), "坏参数在磁盘上留下了东西 ⇒ 判参没排在构造之前"

    def test_an_unknown_action_is_argparse_two_not_a_crash(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)

        _out, _err, code = _run_cli(["cli", "backup", "--action", "nope"])

        assert code == 2


class TestInjectionArmsKeepTheGuardHonest:
    """三臂注入：把产品那一侧摘掉，本文件的对应断言必须当场失效（零写入通道）"""

    def test_arm_stubbing_the_sdk_entry_empties_the_verify_reading(self, tmp_path, monkeypatch):
        """臂 1：`verify_backup_integrity` 换成恒回空 ⇒ verify 只说「没有找到备份」且退 0

        证明 `TestCliVerifyVerdict` 量的是那次真哈希，而不是把索引字段凑成一句话。
        """
        import augmentor.backup as bk

        monkeypatch.setattr(bk, "verify_backup_integrity", lambda *a, **k: {})
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")

        out, _err, code = _run_cli(["cli", "backup", "--action", "verify", "--backup-dir", str(bd)])

        assert code is None
        assert out == "没有找到备份\n"

    def test_arm_blinding_the_verdict_turns_the_bad_backup_back_into_zero(
            self, tmp_path, monkeypatch):
        """臂 2：`verdict_exit` 换成 no-op ⇒ 坏备份从 1 变 0，而文案照旧说「不正常」

        这一臂是「退出码由判决产生」的正面证据：摘掉之后**输出内容一字未变**（还是那句
        `1 个不正常: a`），只有判决没了 ⇒ 证明上一类的 `code == 1` 不是崩溃、不是解析失败。
        """
        import augmentor.cli.commands.version as ver

        monkeypatch.setattr(ver, "verdict_exit", lambda passed, enforce=True: None)
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")
        (bd / "a.json").write_text(json.dumps([{"instruction": "坏"}]), encoding="utf-8")

        out, err, code = _run_cli(["cli", "backup", "--action", "verify", "--backup-dir", str(bd)])

        assert code is None, f"{out}{err}"
        assert "1 个不正常: a" in out and "错误:" not in err

    def test_arm_noop_cleanup_reports_zero_deleted_while_the_files_stay(self, tmp_path, monkeypatch):
        """臂 3：`clean_old_backups` 换成恒回 0 ⇒ 明明有两份却说「删除 0 个」，索引一字未动

        证明 `test_keeping_one_deletes_the_older_rows` 删的是 SDK 那一只手，不是分支里
        顺手写的别的逻辑。
        """
        import augmentor.backup as bk

        monkeypatch.setattr(bk, "clean_old_backups", lambda *a, **k: 0)
        src, bd = _sandbox(tmp_path, monkeypatch)
        _create(src, bd, "a")
        _create(src, bd, "b")

        out, _err, code = _run_cli([
            "cli", "backup", "--action", "cleanup", "--max-backups", "1", "--backup-dir", str(bd),
        ])

        assert code is None
        assert out == "清理完成: 删除 0 个，保留最新 1 个\n"
        rows = json.loads((bd / "index.json").read_text(encoding="utf-8"))["backups"]
        assert [r["backup_id"] for r in rows] == ["a", "b"]
