# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L326：版本 id 的申领必须是**原子**的（`mkdir(exist_ok=False)` 当票）

**为什么有这一支**：L326 那一轮修的是 `AI platform CI` 上一道**长期红**的门禁，
其中一格是并发建版本：24 支并发在 CI 的 2 vCPU 跑轮上**撞出同一枚 id**
⇒ 盘上只剩 23 份（后建的那一份覆盖先建的那一份）⇒ 封顶那一支于是看到 22 份在册、
按「受保护的一版 + 最新 9 份」删到 **11** 份（闸门要 10）。

**修法**：`augmentor/versioning.py` 的 `_generate_version_id` 不再用「先看存不存在、
再改一个后缀」那个 **check-then-act**（它依赖一次 `exists()` 读到的形状，而那个窗口
在 `mkdir` 之前），改成**把 `mkdir` 本身当申领票**（`exist_ok=False` ⇒ 撞上就抛
`FileExistsError`）⇒ 「这枚 id 归我了」是原子的。

**这一支把「慢机器上才会撞」改成「必然撞」**：把 `datetime.now()` 钉成同一微秒 ⇒
两次申领必然落在同一个基名上 ⇒ 改前那两枚 id 相同、盘上只剩一份；改后两枚不同、
两份都在。

**零 `parametrize`**（仓里「新守卫一律零 parametrize」的惯例 ⇒ l97 / l98 两把
参数化普查尺的读数一格未动）。
"""

import datetime
from pathlib import Path

from augmentor.versioning import VersionManager


class _FrozenClock:
    """把 `augmentor.versioning` 用的 `datetime.now()` 钉在**同一微秒**上"""

    def __init__(self, stamp: str):
        self.stamp = stamp

    def now(self):
        return datetime.datetime.strptime(self.stamp, "%Y-%m-%d %H:%M:%S.%f")


def _pin_clock(monkeypatch, stamp: str = "2026-11-13 00:00:00.000001"):
    import augmentor.versioning as versioning

    monkeypatch.setattr(versioning, "datetime", _FrozenClock(stamp))


class TestTheClaimIsAtomic:
    def test_two_claims_in_the_same_microsecond_get_two_ids(self, tmp_path, monkeypatch):
        """同一微秒里申领两次 ⇒ **两枚 id 不同**、**两份都在盘上**"""
        _pin_clock(monkeypatch)
        manager = VersionManager(storage_dir=str(tmp_path))

        first = manager._generate_version_id()
        second = manager._generate_version_id()

        assert first != second, (first, second)
        assert (manager.storage_dir / first).is_dir(), first
        assert (manager.storage_dir / second).is_dir(), second

    def test_the_ticket_is_issued_before_the_caller_uses_the_id(self, tmp_path, monkeypatch):
        """**票已经发出去了**：申领返回时目录就在盘上 ⇒ 调用方拿到的 id 一定是自己的"""
        _pin_clock(monkeypatch)
        manager = VersionManager(storage_dir=str(tmp_path))

        vid = manager._generate_version_id()

        assert (manager.storage_dir / vid).is_dir(), vid

    def test_the_collision_path_takes_the_suffix(self, tmp_path, monkeypatch):
        """撞上时走**后缀**那一支（不是覆盖，也不是报错）⇒ 那一枚仍归自己"""
        _pin_clock(monkeypatch)
        manager = VersionManager(storage_dir=str(tmp_path))

        first = manager._generate_version_id()
        # 手工占住「基名 + _1」那一格 ⇒ 下一枚必须再退一格
        (manager.storage_dir / f"{first}_1").mkdir(parents=True, exist_ok=True)
        second = manager._generate_version_id()

        assert second == f"{first}_2", second
        assert (manager.storage_dir / second).is_dir(), second


class TestTheOldShapeIsWhatBroke:
    """反向臂：换回旧的 check-then-act 形状 ⇒ 同一微秒里两枚 id 撞成一枚

    这一支证明上面那三格是**承重**的，而不是「碰巧绿」：旧形状在本测试的同一微秒里
    必然撞车（`exists()` 读到的形状里，第二个线程还没看见第一个建目录）⇒ 盘上只剩一份。
    """

    def test_the_old_check_then_act_shape_collides_in_the_same_microsecond(self, tmp_path):
        root = Path(tmp_path)
        stamp = "2026-11-13 00:00:00.000001"
        when = datetime.datetime.strptime(stamp, "%Y-%m-%d %H:%M:%S.%f")

        def old_generate_version_id():
            """改前的形状：微秒时间戳 + 存在性去重（**不建目录** ⇒ 那个窗口真实存在）"""
            timestamp = when.strftime("%Y%m%d_%H%M%S_%f")
            version_id = f"v_{timestamp}"
            suffix = 1
            while (root / version_id).exists():
                version_id = f"v_{timestamp}_{suffix}"
                suffix += 1
            return version_id

        first = old_generate_version_id()
        second = old_generate_version_id()

        # 旧形状：两枚**相同**（第二个还没看见第一个建目录 ⇒ 那个存在性检查读到的形状
        # 是「不存在」）⇒ 后建的那一份会覆盖先建的那一份 ⇒ 盘上少一份。
        assert first == second, (first, second)
        assert not (root / first).exists(), "旧形状不建目录 ⇒ 这就是「后建的覆盖先建的」那个窗口"
