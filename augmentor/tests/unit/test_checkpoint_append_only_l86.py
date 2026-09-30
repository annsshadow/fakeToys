# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A144 的收口判据：断点增量改成 append-only，且三件兼容责任逐条钉住

L85 的断点尺子量到「每次自动保存 = 整读增量 + extend + 全量写回」，于是第 k 次保存
的代价随 k 线性增长、总代价 **O(N²/interval)**：N=1000 → 16 000 的实测翻倍倍率是
×3.10 / ×3.61 / ×3.93 / ×4.06（16 000 条那一格 **20 179.8 ms**）。本轮写侧改成
每次只追加一行，A144 那一行同时记下三件必须照旧的责任，这一份守卫就按这三件编号：

① 旧格式（整份是**一个** JSON 文档的 `_delta.json`）升级后仍必须读得出来 —— 本仓
   `checkpoints/` 里可能存着真用户的中途进度，破坏续传比慢更糟；
② `load_checkpoint` 合并之后「立即落盘 + 删增量」的语义照旧（重复恢复不得累加）；
③ 崩溃安全：JSONL 尾行可能被截断，恢复侧要**跳过坏行**而不是把整份增量作废
   （旧实现的 `except` 正是「损坏 ⇒ 重建空增量」，那等于丢进度）。

外加一条「写侧确实不再读」的结构判据。纪律 (u) 在这一格里有具体形状：计数器只挂
`builtins.open` 是**看不见** `Path.read_text` 的（pathlib 持的是 `io.open` 的直接引用），
所以那一条同时挂了两把尺子，且自带对照组证明两把都真的在数。
"""

import builtins
import io
import json
import logging
from pathlib import Path

import pytest

from augmentor.checkpoint import CheckpointManager


@pytest.fixture
def io_spy(monkeypatch):
    """同时记录 `open(...)` 的模式与 `Path.read_text(...)` 的调用

    两把都必要：写侧走 `builtins.open`，读侧走 `Path.read_text`，只挂前者会把
    「读了一遍增量」计成 0 次 —— 那正是探针带病时最危险的形状（看起来像清白）。
    """
    calls = []
    real_open = builtins.open

    def spy_open(file, mode="r", *args, **kwargs):
        if isinstance(file, (str, bytes)) or hasattr(file, "__fspath__"):
            calls.append((str(Path(file).resolve()), mode))
        return real_open(file, mode, *args, **kwargs)

    real_read_text = Path.read_text

    def spy_read_text(self, *args, **kwargs):
        calls.append((str(Path(self).resolve()), "read_text"))
        return real_read_text(self, *args, **kwargs)

    # `io.open` 与 `builtins.open` 在 CPython 里是同一个函数对象，但这里一并替换，
    # 免得哪天 pathlib 改成走 `io.open` 时这把尺子悄悄失效。
    monkeypatch.setattr(builtins, "open", spy_open)
    monkeypatch.setattr(io, "open", spy_open)
    monkeypatch.setattr(Path, "read_text", spy_read_text)
    return calls


def _reads_of(calls, path):
    target = str(Path(path).resolve())
    return [c for c in calls if c[0] == target and c[1] != "a"]


def _appends_of(calls, path):
    target = str(Path(path).resolve())
    return [c for c in calls if c[0] == target and c[1] == "a"]


def _lines(path):
    return [ln for ln in Path(path).read_text(encoding="utf-8").splitlines() if ln]


class TestWriteSideIsAppendOnly:
    """结构判据：自动保存只追加，不读不改写整份"""

    def test_twenty_saves_never_read_the_delta_file(self, tmp_path, io_spy):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=2)
        manager.create_checkpoint("t", 40)
        before = len(io_spy)
        for i in range(40):
            manager.update_progress(i, success=True)

        delta = manager._get_delta_path("t")
        window = io_spy[before:]
        assert len(_appends_of(window, delta)) == 20, window
        assert _reads_of(window, delta) == [], window

    def test_control_group_catches_a_read(self, tmp_path, io_spy):
        """对照组：真的读一次增量，两把尺子必须都看得见

        没有这一条，上一格的「读 0 次」无法区分「确实没读」与「尺子看不见」。
        """
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=2)
        manager.create_checkpoint("t", 6)
        for i in range(6):
            manager.update_progress(i, success=True)
        delta = manager._get_delta_path("t")

        before = len(io_spy)
        manager._read_delta(delta)
        assert _reads_of(io_spy[before:], delta) != [], io_spy[before:]

    def test_each_line_is_exactly_one_batch(self, tmp_path):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=3)
        manager.create_checkpoint("t", 9)
        for i in range(9):
            manager.update_progress(i, success=True)

        lines = _lines(manager._get_delta_path("t"))
        assert [json.loads(ln)["completed"] for ln in lines] == [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
        ]

    def test_total_bytes_grow_linearly_with_items(self, tmp_path):
        """超线性的**结构**证据：增量文件字节数与条数成正比（旧实现是平方）

        这条刻意不测墙钟：本机有并行 agent 在同 worktree 写别的目录，小 n 的计时
        不可复现（纪律 (v)）。字节数是确定的，翻倍就是翻倍、平方就是平方。
        """
        def bytes_for(n, interval):
            d = tmp_path / f"ck{n}"
            manager = CheckpointManager(checkpoint_dir=str(d), auto_save_interval=interval)
            manager.create_checkpoint("t", n)
            for i in range(n):
                manager.update_progress(i, success=True, quality_score=0.5)
            return Path(manager._get_delta_path("t")).stat().st_size

        small, big = bytes_for(200, 10), bytes_for(400, 10)
        ratio = big / small
        # 平方会长到 ≈4；线性 ≈2（末行长度只差索引位数，留一点余量）
        assert 1.8 < ratio < 2.3, (small, big, ratio)

    def test_repeated_saves_do_not_reserialize_history(self, tmp_path, io_spy):
        """旧实现每次写回全量 ⇒ 第 k 次写的字节数是 O(k)；新实现每次只写这一批"""
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=1)
        manager.create_checkpoint("t", 5)
        sizes = []
        for i in range(5):
            before = len(io_spy)
            manager.update_progress(i, success=True)
            assert len(_appends_of(io_spy[before:], manager._get_delta_path("t"))) == 1
            sizes.append(len(_lines(manager._get_delta_path("t"))))
        assert sizes == [1, 2, 3, 4, 5]


class TestLegacyFormatStillResumable:
    """责任 ①：升级前留下的 `_delta.json` 必须照样合并"""

    @pytest.mark.parametrize("pretty", [False, True], ids=["compact", "pretty"])
    def test_legacy_single_doc_delta_is_merged(self, tmp_path, pretty):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=100)
        manager.create_checkpoint("t", 10)
        manager.update_progress(0, success=True, quality_score=0.7)
        manager.save_checkpoint()
        # 手动造一份"旧格式"：整份是一个 JSON 文档
        legacy = manager._get_legacy_delta_path("t")
        doc = {"completed": [1, 2], "failed": [3], "quality_scores": {"1": 0.6}}
        legacy.write_text(
            json.dumps(doc, separators=(",", ":")) if not pretty
            else json.dumps(doc, indent=2),
            encoding="utf-8",
        )

        resumed = CheckpointManager(checkpoint_dir=str(tmp_path)).load_checkpoint("t")
        assert sorted(resumed.completed_indices) == [0, 1, 2]
        assert resumed.failed_indices == [3]
        assert resumed.processed_items == 3
        assert resumed.failed_items == 1
        assert resumed.quality_scores == {0: 0.7, 1: 0.6}

    def test_both_formats_merge_together(self, tmp_path):
        """升级的窗口里两种文件可能并存：各算一批，合起来一条不丢"""
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=100)
        manager.create_checkpoint("t", 20)
        manager._get_legacy_delta_path("t").write_text(
            json.dumps({"completed": [1], "failed": [], "quality_scores": {}}),
            encoding="utf-8",
        )
        manager.update_progress(2, success=True)
        manager._save_delta()

        resumed = CheckpointManager(checkpoint_dir=str(tmp_path)).load_checkpoint("t")
        assert sorted(resumed.completed_indices) == [1, 2]
        assert resumed.processed_items == 2

    def test_corrupt_legacy_doc_does_not_void_the_new_file(self, tmp_path):
        """旧那份坏掉时，新那份的进度不能跟着一起丢"""
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=1)
        manager.create_checkpoint("t", 6)
        for i in range(3):
            manager.update_progress(i, success=True)
        manager._get_legacy_delta_path("t").write_text("{ broken", encoding="utf-8")

        resumed = CheckpointManager(checkpoint_dir=str(tmp_path)).load_checkpoint("t")
        assert sorted(resumed.completed_indices) == [0, 1, 2]
        assert resumed.processed_items == 3


class TestResumeSemanticsUnchanged:
    """责任 ②：合并后立刻落盘、删增量，重复恢复不累加"""

    def test_load_persists_and_removes_both_delta_files(self, tmp_path):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=2)
        manager.create_checkpoint("t", 8)
        for i in range(4):
            manager.update_progress(i, success=True)
        manager._get_legacy_delta_path("t").write_text(
            json.dumps({"completed": [], "failed": [], "quality_scores": {}}),
            encoding="utf-8",
        )

        fresh = CheckpointManager(checkpoint_dir=str(tmp_path))
        loaded = fresh.load_checkpoint("t")
        assert loaded.processed_items == 4
        assert not manager._get_delta_path("t").exists()
        assert not manager._get_legacy_delta_path("t").exists()

        data = json.loads(
            (tmp_path / "t_checkpoint.json").read_text(encoding="utf-8")
        )
        assert data["completed_indices"] == [0, 1, 2, 3]
        assert data["processed_items"] == 4

    def test_repeated_load_is_idempotent(self, tmp_path):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=2)
        manager.create_checkpoint("t", 8)
        for i in range(4):
            manager.update_progress(i, success=True)

        first = CheckpointManager(checkpoint_dir=str(tmp_path)).load_checkpoint("t")
        second = CheckpointManager(checkpoint_dir=str(tmp_path)).load_checkpoint("t")
        assert first.processed_items == second.processed_items == 4
        assert first.completed_indices == second.completed_indices

    def test_merge_reproduces_the_old_batch_sequence(self, tmp_path):
        """合出来的 `completed` 与旧实现逐元素同号：每批 sorted、批次按时间拼接"""
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=2)
        manager.create_checkpoint("t", 6)
        for i in range(6):
            manager.update_progress(i, success=True)

        merged = manager._read_delta(manager._get_delta_path("t"))
        assert merged["completed"] == [0, 1, 2, 3, 4, 5]


class TestTruncatedTailKeepsEarlierBatches:
    """责任 ③：尾行截断只丢那一批，不丢整份"""

    def test_partial_last_line_is_skipped_not_fatal(self, tmp_path, caplog):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=2)
        manager.create_checkpoint("t", 8)
        for i in range(4):
            manager.update_progress(i, success=True)
        delta = manager._get_delta_path("t")
        content = delta.read_text(encoding="utf-8")
        delta.write_text(content + '{"completed": [4, 5', encoding="utf-8")

        with caplog.at_level(logging.WARNING, logger="augmentor.checkpoint"):
            merged = manager._read_delta(delta)
        assert merged["completed"] == [0, 1, 2, 3]
        assert any("跳过增量坏行" in r.message for r in caplog.records), caplog.text

        resumed = CheckpointManager(checkpoint_dir=str(tmp_path)).load_checkpoint("t")
        assert resumed.processed_items == 4

    def test_non_object_line_is_skipped(self, tmp_path, caplog):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=100)
        manager.create_checkpoint("t", 4)
        delta = manager._get_delta_path("t")
        delta.write_text(
            '[1, 2]\n{"completed": [0], "failed": [], "quality_scores": {}}\n',
            encoding="utf-8",
        )
        with caplog.at_level(logging.WARNING, logger="augmentor.checkpoint"):
            merged = manager._read_delta(delta)
        assert merged["completed"] == [0]
        assert any("不是 JSON 对象" in r.message for r in caplog.records), caplog.text

    def test_empty_file_returns_empty_without_claiming_progress(self, tmp_path):
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=100)
        manager.create_checkpoint("t", 4)
        delta = manager._get_delta_path("t")
        delta.write_text("", encoding="utf-8")
        assert manager._read_delta(delta) == {
            "completed": [], "failed": [], "quality_scores": {}
        }

    def test_blank_lines_between_records_are_skipped(self, tmp_path):
        """记录之间夹空行不该让合并停手

        人手工编辑过增量、或上一次追加正好落在进程被杀的边界上，都会留下空行；
        「跳过空行」那一支如果没人跑，读侧就只在理想字节序列上成立。
        """
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=100)
        manager.create_checkpoint("t", 4)
        delta = manager._get_delta_path("t")
        delta.write_text(
            "\n\n"
            '{"completed": [0], "failed": [], "quality_scores": {}}\n'
            "\n"
            '{"completed": [1], "failed": [2], "quality_scores": {}}\n'
            "\n",
            encoding="utf-8",
        )
        merged = manager._read_delta(delta)
        assert merged["completed"] == [0, 1]
        assert merged["failed"] == [2]


class TestUnreadableDeltaFile:
    """增量文件**读不出**与**内容坏**是两件事，两条路要各自出声"""

    def test_undecodable_bytes_warn_and_rebuild(self, tmp_path, caplog):
        """整份不是合法 UTF-8 时落在读取那一刀，而不是逐行解析那一刀

        旧实现把「打开 + 解析」包在同一个 try 里，所以这一格在 HEAD 是被
        「内容损坏」那条用例顺带盖住的；拆成两刀之后它成了独立的弧，
        必须自己有一条用例盯着 —— 否则「读不出」会被静默当成「没有增量」。
        """
        manager = CheckpointManager(checkpoint_dir=str(tmp_path), auto_save_interval=100)
        manager.create_checkpoint("t", 4)
        delta = manager._get_delta_path("t")
        delta.write_bytes(b"\xff\xfe\x00{\x00")
        with caplog.at_level(logging.WARNING, logger="augmentor.checkpoint"):
            merged = manager._read_delta(delta)
        assert merged == {"completed": [], "failed": [], "quality_scores": {}}
        assert any("读不出" in r.message for r in caplog.records), caplog.text
