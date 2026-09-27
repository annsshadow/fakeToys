# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L100 守卫：把 A185 那笔「靠副作用活着的免费覆盖」换成真判据

**病理**（L99 批⑤ 实测，账本 A185）：L99 批① 把断点保存从裸 `open(w)` 换成原子替换之后，
`augmentor/checkpoint.py` 的缺数从 0 涨到 2、偏支从 85 涨到 87。二分取证给出的因果是：
L98 那两条缺数**从来没有人主动测过** —— `tests/unit/test_pipeline_branches.py` 的 fixture 在
构造之后把 `checkpoint_dir` 改到一个从未创建的目录，于是旧版 `_save_checkpoint`（裸 open）抛
`FileNotFoundError` 被吞，随后 `_save_delta` 撞同一个缺失目录 ⇒ 两处 `except` 各被**副作用**
白捡一次覆盖。批① 之后写盘顺手建了目录，那条 `except` 再没人路过 ⇒ 缺数现形。

⇒ 真正的问题不是「覆盖率掉了 0.03」，而是**三件本来就该有判据的事一直只有巧合**：
① `_save_delta` 追加失败时那一批不许被当成已保存（否则下次崩溃就少一批）；
② 恢复路径（`load_checkpoint` 的合并收尾）必须看写盘判决再删增量 —— L99 只给手动保存那一路
   补了判据，**恢复这一路更危险**：它发生在进程刚重启、内存里什么都没有的时候；
③ 相对 `checkpoint_dir` 的落点到底以哪一刻的当前目录为准。

三件都在本文件里落成测，外加 `_replace_with_retry` 的「瞬时被拒、随后成功」那一档（旧写法
把最后一次替换藏在 `if attempt == 末次: raise` 里，循环出口结构上不可达，于是那条偏支永远红；
改写成「最后一次放在循环外」之后，同一档变成可测的真实路径）。
"""

import json
import logging

from augmentor import checkpoint as checkpoint_module
from augmentor.checkpoint import CheckpointManager

LOGGER = "augmentor.checkpoint"


class _Writer:
    """按开关决定「写成功」还是「抛 OSError」的 `atomic_write_json` 替身"""

    def __init__(self, real):
        self.real = real
        self.fail = False
        self.calls = 0

    def __call__(self, file_path, data):
        self.calls += 1
        if self.fail:
            raise OSError("盘满（测试注入）")
        return self.real(file_path, data)


def _seed(tmp_path, completed=(1, 2, 3)):
    """造一份**已经落盘**的断点：主文件里有 completed，不含任何增量"""
    mgr = CheckpointManager(checkpoint_dir=str(tmp_path / "ck"))
    cp = mgr.create_checkpoint("t1", total_items=100)
    cp.completed_indices = list(completed)
    cp.processed_items = len(completed)
    assert mgr._save_checkpoint(cp) is True
    return mgr, cp


class TestDeltaAppendFailureKeepsTheBatch:
    """A185 处方①：增量追加失败时，那一批留在内存里而不是被清空"""

    def test_failed_append_leaves_pending_set_alone_and_says_so(self, tmp_path, caplog):
        mgr, _cp = _seed(tmp_path)
        delta_path = mgr._get_delta_path("t1")
        delta_path.mkdir(parents=True)  # 路径上躺着一个目录 ⇒ open('a') 必失败
        mgr._pending_completed = {7}
        mgr._pending_failed = {8}

        with caplog.at_level(logging.WARNING, logger=LOGGER):
            mgr._save_delta()

        assert mgr._pending_completed == {7}, "写失败却清了待保存集合 ⇒ 那一批凭空消失"
        assert mgr._pending_failed == {8}
        assert any("保存增量失败" in r.message for r in caplog.records), caplog.text
        assert delta_path.is_dir(), "追加失败却把挡路的目录清了 ⇒ 写侧动到了不该动的东西"

    def test_the_withheld_batch_is_written_by_the_next_successful_save(self, tmp_path):
        """反空转档：证清「延迟落盘」而不是「丢失」——解除障碍后下一次保存把同一批补上"""
        mgr, _cp = _seed(tmp_path)
        delta_path = mgr._get_delta_path("t1")
        delta_path.mkdir(parents=True)
        mgr._pending_completed = {7}
        mgr._save_delta()
        assert mgr._pending_completed == {7}

        delta_path.rmdir()
        mgr._save_delta()
        assert mgr._pending_completed == set(), "补写成功后才该清账"
        lines = [json.loads(ln) for ln in
                 delta_path.read_text(encoding="utf-8").splitlines()]
        assert len(lines) == 1 and lines[0]["completed"] == [7]


class TestRecoveryMergeLooksAtTheWriteVerdict:
    """A185 处方②：`load_checkpoint` 的合并收尾必须「先写成、再删增量」"""

    def _with_delta(self, mgr, index):
        """往盘上落一行增量（不经 `save_checkpoint`，它会把增量并掉再删）"""
        path = mgr._get_delta_path("t1")
        with open(path, "a", encoding="utf-8") as f:
            f.write(json.dumps({"completed": [index], "failed": [],
                                "quality_scores": {}}) + "\n")

    def test_failed_merge_keeps_the_delta_and_reports_error(self, tmp_path, caplog, monkeypatch):
        mgr, _cp = _seed(tmp_path)
        self._with_delta(mgr, 7)
        writer = _Writer(checkpoint_module.atomic_write_json)
        writer.fail = True
        monkeypatch.setattr(checkpoint_module, "atomic_write_json", writer)

        fresh = CheckpointManager(checkpoint_dir=str(tmp_path / "ck"))
        with caplog.at_level(logging.WARNING, logger=LOGGER):
            back = fresh.load_checkpoint("t1")

        assert writer.calls >= 1, "没路过写盘这一步 ⇒ 本档的断言全是零读数"
        assert back is not None and back.completed_indices == [1, 2, 3, 7]
        assert mgr._get_delta_path("t1").exists(), "主文件没写成却删了增量 ⇒ 那一批永久丢失"
        assert any("保存断点失败" in r.message for r in caplog.records), caplog.text

    def test_recovery_after_a_failed_merge_is_idempotent(self, tmp_path, monkeypatch):
        """没删成的增量下一次恢复必须**同样**而不是重复累加 —— 留着它的前提条件"""
        mgr, _cp = _seed(tmp_path)
        self._with_delta(mgr, 7)
        writer = _Writer(checkpoint_module.atomic_write_json)
        writer.fail = True
        monkeypatch.setattr(checkpoint_module, "atomic_write_json", writer)
        first = CheckpointManager(checkpoint_dir=str(tmp_path / "ck")).load_checkpoint("t1")
        second = CheckpointManager(checkpoint_dir=str(tmp_path / "ck")).load_checkpoint("t1")
        assert first.completed_indices == second.completed_indices == [1, 2, 3, 7]
        assert first.processed_items == second.processed_items == 4

        writer.fail = False  # 盘腾出来了：这一次合并该把增量吃掉
        third = CheckpointManager(checkpoint_dir=str(tmp_path / "ck")).load_checkpoint("t1")
        assert third.completed_indices == [1, 2, 3, 7], "补写成之后进度不该再变"
        assert not mgr._get_delta_path("t1").exists(), "合并成功却没删增量 ⇒ 下次恢复重复累加"


class TestRelativeCheckpointDirIsBasedAtConstruction:
    """A185 处方③：相对落点在构造那一刻定基，之后换当前目录不搬走断点"""

    def test_relative_dir_does_not_follow_a_later_chdir(self, tmp_path, monkeypatch):
        base = tmp_path / "A"
        base.mkdir()
        monkeypatch.chdir(base)
        mgr = CheckpointManager(checkpoint_dir="ck")
        assert mgr.checkpoint_dir == base / "ck"

        other = tmp_path / "B"
        other.mkdir()
        monkeypatch.chdir(other)
        cp = mgr.create_checkpoint("t1", total_items=10)
        cp.completed_indices = [1]
        assert mgr._save_checkpoint(cp) is True
        assert (base / "ck" / "t1_checkpoint.json").exists(), "断点没落在构造时那一侧"
        assert not (other / "ck").exists(), "写盘时顺带在新目录建了分家的断点目录"

    def test_absolute_dir_is_kept_byte_for_byte(self, tmp_path):
        """绝对路径不做二次解析：`resolve()` 会展开符号链接与 8.3 短名，测试目录不该被它改写"""
        target = tmp_path / "abs"
        assert CheckpointManager(checkpoint_dir=str(target)).checkpoint_dir == target
