# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A106：`apply_logging_config` 的幂等从「无条件重装」改成「签名命中即返回缓存」

改前每次 `load_config` 都无条件 `_detach_installed` 再重装配，其中包括关掉并**重开
一次日志文件**。`api/deps.py` 的 `/api/config` 是每请求读盘，于是长跑 API 每个请求都
付这笔钱（实测约 103 µs，Temp `l65/out.txt`），且撤与装之间 root 短暂无 handler，并发
请求可能正好落在那一瞬丢日志行。

L65 给装配加了一份 (写出的键 + 三值 + root.handlers 拓扑) 签名（`_APPLIED`）：全一致
就直接返回缓存摘要，不撤不装。本文件锁死这份缓存的**边界**，因为「命中即省事」的另一
面是「命中错了就漏装/漏校验」——所以每一条都是双向的：

1. 命中真的省（不重开文件、不换 handler 对象、返回的是拷贝），
2. 该破的一定要破（换 file / 换 level / 有外部 handler 挂进 root ⇒ 拓扑变），
3. 破完之后**判据一条不省**（坏 level 在 miss 路径上照样抛 `DataValidationError`）。

`a97` 那两条 `test_repeated_identical_load_hits_cache_and_probes_nothing` /
`test_changed_format_breaks_assembly_cache` 是从产品面（经 `load_config`）证的；这里
用 SDK 直构 + 计数器从单元面证同一件事，两边合起来缓存才既有牙又不越界。
"""

import contextlib
import io
import logging

import pytest

from augmentor import logging_setup
from augmentor.config import LoggingConfig
from augmentor.exceptions import DataValidationError
from augmentor.logging_setup import apply_logging_config


@contextlib.contextmanager
def _empty_root():
    """把 root 清成「CLI 进程今天的形状」（一条 handler 都没有）后执行体内做事

    pytest 在每条用例的 call 阶段挂着自己的 `LogCaptureHandler`，夹具(setup)里清空没用
    （清空后它又加回来）。本模块「装不装控制台 handler」只看 `root.handlers` 是否为空，
    所以凡是依赖「本模块确实装出了自己的 handler」的用例，必须在测试体内用
    `with _empty_root():` 清，测的才是 CLI 那一支而不是 API 那一支。
    """
    root = logging.getLogger()
    saved = root.handlers[:]
    root.handlers[:] = []
    try:
        yield root
    finally:
        for handler in logging_setup._INSTALLED[:]:
            if handler in root.handlers:
                root.removeHandler(handler)
            handler.close()
        logging_setup._INSTALLED.clear()
        root.handlers[:] = saved


@pytest.fixture(autouse=True)
def isolate_assembly_state():
    """把 root 的 handler/级别与本模块的两份全局状态隔离在单条用例内

    本模块缓存了 `_INSTALLED`（装过的 handler）和 `_APPLIED`（上次的签名+摘要）。
    不清 `_APPLIED` 就会跨用例漏签名：上一条留下的命中态让下一条「本该完整装配」的
    调用假命中，测出来的绿是假的。`_empty_root()` 只清 handler、不碰签名，所以两条
    路都要在这里兜住。
    """
    root = logging.getLogger()
    saved_handlers, saved_level = root.handlers[:], root.level
    for handler in logging_setup._INSTALLED[:]:
        handler.close()
    logging_setup._INSTALLED.clear()
    logging_setup._APPLIED = None
    root.handlers[:] = []
    root.setLevel(saved_level)
    yield
    for handler in logging_setup._INSTALLED[:]:
        if handler in root.handlers:
            root.removeHandler(handler)
        handler.close()
    logging_setup._INSTALLED.clear()
    logging_setup._APPLIED = None
    root.handlers[:] = saved_handlers
    root.setLevel(saved_level)


class TestCacheHitSkipsWork:
    """签名命中 ⇒ 什么都不做，直接回缓存摘要"""

    def test_identical_apply_reopens_the_log_file_only_once(self, tmp_path, monkeypatch):
        """命中不许重开文件：这正是本轮要省掉的那笔每请求开销"""
        calls = []
        real = logging_setup._open_file_handler

        def counting(path):
            calls.append(path)
            return real(path)

        monkeypatch.setattr(logging_setup, "_open_file_handler", counting)
        cfg = LoggingConfig(level="WARNING", file=str(tmp_path / "a.log"),
                            format="%(message)s")
        apply_logging_config(cfg)
        assert len(calls) == 1, "首次完整装配应开一次文件"
        for _ in range(10):
            apply_logging_config(cfg)
        assert len(calls) == 1, "A106：签名命中却把日志文件重开了"

    def test_cache_hit_returns_a_fresh_copy_not_the_stored_dict(self, tmp_path):
        """缓存摘要必须是拷贝：调用方改了返回值，不能污染下一次命中"""
        cfg = LoggingConfig(level="INFO", file="", format="%(message)s")
        with _empty_root():
            first = apply_logging_config(cfg)
            expected = dict(first)
            first["handlers"] = 999  # 污染本轮返回值
            second = apply_logging_config(cfg)  # 命中：回存起来的干净拷贝
            assert second is not first, "命中返回了同一个对象 = 没拷贝"
            assert second["handlers"] != 999, "调用方的改动漏进了缓存 = 存的是原始 dict 不是拷贝"
            assert second == expected

    def test_cache_hit_keeps_the_same_handler_objects(self, tmp_path):
        """命中不许换装：handler 对象身份必须原样，否则就等于偷偷撤装重装"""
        cfg = LoggingConfig(level="WARNING", file=str(tmp_path / "b.log"),
                            format="%(message)s")
        apply_logging_config(cfg)
        installed = list(logging_setup._INSTALLED)
        assert installed, "前置：首次装配应装出至少一条 handler"
        apply_logging_config(cfg)
        assert logging_setup._INSTALLED == installed, "命中却换了 handler 对象 = 撤装没被跳过"


class TestCacheBreaksWhenItShould:
    """任一变（三值 / 拓扑）都必须打破签名、回到完整装配"""

    def test_changed_file_reopens_and_swaps_the_handler(self, tmp_path, monkeypatch):
        calls = []
        real = logging_setup._open_file_handler

        def counting(path):
            calls.append(path)
            return real(path)

        monkeypatch.setattr(logging_setup, "_open_file_handler", counting)
        cfg = LoggingConfig(level="WARNING", file=str(tmp_path / "a.log"),
                            format="%(message)s")
        apply_logging_config(cfg)
        old = list(logging_setup._INSTALLED)
        cfg.file = str(tmp_path / "b.log")
        apply_logging_config(cfg)
        assert len(calls) == 2, "换 file 必须打破缓存、开新文件"
        assert logging_setup._INSTALLED != old, "换 file 后旧的文件 handler 必须被撤掉"

    def test_changed_level_reapplies_the_root_level(self):
        """换 level 若仍命中缓存，返回的还是旧的 INFO —— 这一条就是「缓存不能恒命中」
        在级别维度上的反证。"""
        cfg = LoggingConfig(level="INFO", file="", format="%(message)s")
        first = apply_logging_config(cfg)
        assert first["level"] == logging.INFO
        cfg.level = "DEBUG"
        second = apply_logging_config(cfg)
        assert second["level"] == logging.DEBUG, "换 level 后摘要仍是旧级别 = 缓存恒命中"
        assert logging.getLogger().level == logging.DEBUG

    def test_externally_added_handler_breaks_the_signature(self, tmp_path):
        """签名带 root.handlers 拓扑：外部挂进一条 handler 后，本模块上次的装配态已不
        成立，必须走完整装配（撤掉自己那条旧的），而不是命中一个已经过期的签名。"""
        cfg = LoggingConfig(level="WARNING", file="", format="%(message)s")
        with _empty_root():
            apply_logging_config(cfg)
            first_installed = list(logging_setup._INSTALLED)
            assert first_installed, "前置：空 root 下本模块此时应装出一条控制台 handler"
            extra = logging.StreamHandler(io.StringIO())
            logging.getLogger().addHandler(extra)  # root 拓扑变了
            apply_logging_config(cfg)
            assert first_installed[0] not in logging.getLogger().handlers, \
                "拓扑变了却仍命中缓存 = 旧的 self-installed handler 没被撤"


class TestCacheDoesNotSkipValidation:
    """破缓存之后的完整装配，判据一条都不能被省"""

    def test_bad_level_still_raises_after_a_warm_good_cache(self):
        """先暖一份合法签名，再把 level 改成坏值：签名必 miss（三值之一变了），
        走完整装配时 `level_number` 照样出声拒收 —— 证明快路不会把校验也一起缓存掉。"""
        cfg = LoggingConfig(level="INFO", file="", format="%(message)s")
        apply_logging_config(cfg)
        cfg.level = "BOGUS"  # 直接赋值绕过 __post_init__，测的是 apply 侧的再校验
        with pytest.raises(DataValidationError):
            apply_logging_config(cfg)
