# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""AU-Q2：断点与配置校验面的 except Exception 收窄成真实失败族

被收窄的七处（augmentor/checkpoint.py 六处 + augmentor/config_validator.py 一处）
原先一律 except Exception，把「盘满、坏 JSON、明文密钥被丢」和「编程错误」压成同一
个答案：写失败返回 False、读失败返回没有断点、校验失败报成用户配置问题。后者正是
checkpoint.py 自己 docstring 点名的那一格 —— 一次坏读取被翻译成「这个任务没有断点」，
已完成的整段工作凭空消失，而真正的缺陷（AttributeError 之类）连一行日志都没有。

收窄口径：只收 try 块真实能产生的失败族（OSError / UnicodeDecodeError /
JSONDecodeError / TypeError / ValueError / KeyError），其余异常上抛。既有用例
覆盖真实失败族的行为不变（OSError 注入、坏 JSON、权限被拒都还有测试盯着）；
本文件补的是另一半：**别族异常必须逃逸**，即「吞掉」这个行为真的有边界。

本文件注释里不写反引号路径引用：它自己也在注释普查的扫描面上（A184 口径）。
"""

import json
import logging
import sys
from pathlib import Path

import pytest

REPO = Path(__file__).resolve().parents[2]
if str(REPO) not in sys.path:
    sys.path.insert(0, str(REPO))

from augmentor import checkpoint as checkpoint_module  # noqa: E402
from augmentor.checkpoint import CheckpointManager  # noqa: E402
from augmentor.config_validator import ConfigValidator  # noqa: E402

LOGGER = "augmentor.checkpoint"


def _manager(tmp_path):
    return CheckpointManager(checkpoint_dir=str(tmp_path / "ck"))


def _seeded_manager(tmp_path):
    """造一份已经落盘的断点（completed=[1, 2, 3]）"""
    mgr = _manager(tmp_path)
    cp = mgr.create_checkpoint("t1", total_items=100)
    cp.completed_indices = [1, 2, 3]
    cp.processed_items = 3
    assert mgr._save_checkpoint(cp) is True
    return mgr, cp


# ============ _save_checkpoint：写失败只收 OSError/TypeError/ValueError ============


class TestSaveCheckpointNarrowing:
    def test_oserror_still_means_write_failed(self, tmp_path, monkeypatch):
        """真实失败族不变：OSError 仍然翻译成 False + 一条 error 日志"""
        mgr, cp = _seeded_manager(tmp_path)

        def refuse(*_a, **_k):
            raise OSError("盘满（测试注入）")

        monkeypatch.setattr(checkpoint_module, "atomic_write_json", refuse)
        assert mgr._save_checkpoint(cp) is False

    def test_programming_error_escapes_instead_of_false(self, tmp_path, monkeypatch):
        """别族异常必须逃逸：RuntimeError 不再被翻译成「写失败」的假判决"""
        mgr, cp = _seeded_manager(tmp_path)

        def boom(*_a, **_k):
            raise RuntimeError("simulated programming error")

        monkeypatch.setattr(checkpoint_module, "atomic_write_json", boom)
        with pytest.raises(RuntimeError):
            mgr._save_checkpoint(cp)


# ============ _save_delta：追加失败只收 OSError/TypeError/ValueError ============


class TestSaveDeltaNarrowing:
    def test_oserror_keeps_the_batch_and_logs(self, tmp_path, caplog):
        """真实失败族不变：open 撞 OSError 时待保存集合不清、有 error 日志"""
        mgr, _cp = _seeded_manager(tmp_path)
        delta_path = mgr._get_delta_path("t1")
        delta_path.mkdir(parents=True)  # 路径上躺目录 ⇒ open('a') 必失败
        mgr._pending_completed = {7}

        with caplog.at_level(logging.WARNING, logger=LOGGER):
            mgr._save_delta()

        assert mgr._pending_completed == {7}
        assert any("保存增量失败" in r.message for r in caplog.records), caplog.text

    def test_programming_error_escapes_instead_of_silent_skip(self, tmp_path, monkeypatch):
        """别族异常必须逃逸：open 抛 AttributeError 时不许假装「这一批没写成」"""
        mgr, _cp = _seeded_manager(tmp_path)
        mgr._pending_completed = {7}
        real_open = open

        def boom(file, *args, **kwargs):
            if len(args) >= 1 and "a" in str(args[0]) or kwargs.get("mode") == "a":
                raise AttributeError("simulated programming error")
            return real_open(file, *args, **kwargs)

        monkeypatch.setattr("builtins.open", boom)
        with pytest.raises(AttributeError):
            mgr._save_delta()


# ============ _read_delta：读不出/坏行只收 OSError/UnicodeDecodeError/JSONDecodeError ============


def _write_delta(tmp_path, text):
    mgr = _manager(tmp_path)
    path = mgr._get_delta_path("t1")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")
    return mgr, path


class TestReadDeltaNarrowing:
    def test_unreadable_file_still_rebuilds(self, tmp_path, monkeypatch):
        """真实失败族不变：读不出（OSError）时返回空增量 + warning"""
        mgr, path = _write_delta(tmp_path, '{"completed": [1], "failed": [], "quality_scores": {}}\n')

        def refuse(*_a, **_k):
            raise PermissionError("denied")

        monkeypatch.setattr(Path, "read_text", refuse)
        assert mgr._read_delta(path) == {"completed": [], "failed": [], "quality_scores": {}}

    def test_bad_json_lines_are_still_skipped(self, tmp_path):
        """真实失败族不变：坏行（JSONDecodeError）照旧跳过、不跳整份"""
        mgr, path = _write_delta(
            tmp_path,
            '{"completed": [1], "failed": [], "quality_scores": {}}\n'
            "{truncated\n",
        )
        delta = mgr._read_delta(path)
        assert delta["completed"] == [1]

    def test_programming_error_in_read_escapes(self, tmp_path, monkeypatch):
        """别族异常必须逃逸：read_text 抛 AttributeError 不再降级成空增量"""
        mgr, path = _write_delta(tmp_path, "{}\n")

        def boom(*_a, **_k):
            raise AttributeError("simulated programming error")

        monkeypatch.setattr(Path, "read_text", boom)
        with pytest.raises(AttributeError):
            mgr._read_delta(path)

    def test_programming_error_in_whole_parse_escapes(self, tmp_path, monkeypatch):
        """整份解析只认 JSONDecodeError：别族异常不再假装「不是一个批次」"""
        mgr, path = _write_delta(tmp_path, '{"completed": [1], "failed": [], "quality_scores": {}}\n')
        real_loads = json.loads

        def boom(*_a, **_k):
            raise AttributeError("simulated programming error")

        monkeypatch.setattr(json, "loads", boom)
        with pytest.raises(AttributeError):
            mgr._read_delta(path)
        monkeypatch.setattr(json, "loads", real_loads)  # 还回去，后面断方用真解析

    def test_programming_error_in_line_parse_escapes(self, tmp_path, monkeypatch):
        """逐行解析只认 JSONDecodeError：别族异常不再借 continue 咽掉"""
        mgr, path = _write_delta(
            tmp_path,
            '{"completed": [1], "failed": [], "quality_scores": {}}\n'
            "BOOM\n",
        )
        real_loads = json.loads

        def picky(text, *args, **kwargs):
            if text.strip() == "BOOM":
                raise AttributeError("simulated programming error")
            return real_loads(text, *args, **kwargs)

        monkeypatch.setattr(json, "loads", picky)
        with pytest.raises(AttributeError):
            mgr._read_delta(path)


# ============ load_checkpoint：坏断点只收数据/IO 失败族 ============


class TestLoadCheckpointNarrowing:
    def test_corrupt_file_still_means_no_checkpoint(self, tmp_path):
        """真实失败族不变：坏 JSON 仍然翻译成 None + error 日志"""
        mgr = _manager(tmp_path)
        (tmp_path / "ck").mkdir(parents=True, exist_ok=True)
        (tmp_path / "ck" / "broken_checkpoint.json").write_text("{not json", encoding="utf-8")
        assert mgr.load_checkpoint("broken") is None

    def test_programming_error_escapes_instead_of_no_checkpoint(self, tmp_path, monkeypatch):
        """别族异常必须逃逸：不再把编程错误翻译成「这个任务没有断点」"""
        mgr, _cp = _seeded_manager(tmp_path)

        def boom(*_a, **_k):
            raise RuntimeError("simulated programming error")

        monkeypatch.setattr(json, "load", boom)
        with pytest.raises(RuntimeError):
            mgr.load_checkpoint("t1")


# ============ config_validator.validate_file：读取失败只收 OSError/UnicodeDecodeError ============


class TestConfigValidatorReadNarrowing:
    def test_permission_error_still_reported_as_read_error(self, tmp_path, monkeypatch):
        """真实失败族不变：权限被拒（OSError）仍然报「读取文件错误」"""
        import builtins

        p = tmp_path / "cfg.yaml"
        p.write_text("app: {name: x, version: '1.0'}\nmodels: {default: ernie}\n",
                     encoding="utf-8")
        real_open = builtins.open

        def failing_open(file, *args, **kwargs):
            if str(file) == str(p):
                raise PermissionError("access denied")
            return real_open(file, *args, **kwargs)

        monkeypatch.setattr(builtins, "open", failing_open)
        result = ConfigValidator().validate_file(str(p))
        assert result.is_valid is False
        assert result.errors[0].path == "file"
        assert "读取文件错误" in result.errors[0].message

    def test_programming_error_escapes_instead_of_validation_error(self, tmp_path, monkeypatch):
        """别族异常必须逃逸：不再把编程错误压成一条用户配置问题的校验结论"""
        import builtins

        p = tmp_path / "cfg.yaml"
        p.write_text("app: {name: x, version: '1.0'}\nmodels: {default: ernie}\n",
                     encoding="utf-8")
        real_open = builtins.open

        def failing_open(file, *args, **kwargs):
            if str(file) == str(p):
                raise AttributeError("simulated programming error")
            return real_open(file, *args, **kwargs)

        monkeypatch.setattr(builtins, "open", failing_open)
        with pytest.raises(AttributeError):
            ConfigValidator().validate_file(str(p))
