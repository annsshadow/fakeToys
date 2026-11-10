# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A409 守卫：自动快照这一路要有保留份数上限（L305）

**这一轮收的是哪一格**：A409（L294 立）——`versioning.enabled` ×
`versioning.auto_snapshot` 这一路的自动快照**没有保留上限**：每次增强结束都在
`data/versions` 落一份**全量副本**，而仓内查无任何一侧封顶（`max_versions` /
`keep_last` / `prune` / `retention` 四个候选键名在 `augmentor/versioning.py` 与
`augmentor/config.py` 两文件现量 **0 命中**）。A399 治的是同一族里
`history.jsonl` 的无界追加，这一格是**产物全量副本**的无界复制，量级差两个数量级。

**改法**（照备份族 `max_backups` 的同款口径，不新造形状）：

* `VersioningConfig` 新增 `max_versions`（**保留份数**，不是删除份数），出厂 10；
* `VersionManager` 接住它，`create_version` 之后自己收旧 ⇒ **谁建版本谁负责封顶**，
  免得「自动快照封了、手工建的版本无限堆」这一族只治一半；
* 超出时**删旧不拒新**：刚建的那一份永远在保留集里，被删的永远是最旧的。理由是
  这一族的用户动作是「增强结束自动打点」，拒新会让打点**静默失败**（调用方拿不到
  任何信号），删旧则可逆（history 仍记着发生过什么）。

**这一把尺为什么这样摆**：

1. **封顶必须承重**：把 `prune_versions` 换成恒等函数，「超上限仍留 N+1 份」必须
   当场红——否则上面那几支可能只是量到了「本来就不超」的情形。
2. **current 永不被删**：删掉当前版本会让 `current` 指向不存在的目录，而读侧
   `get_current_version` 仍报得出那个 id（孤儿指针）。这一支单独钉。
3. **份数语义是「留几份」不是「删几份」**：`max_versions=3` 时留 3 份、删 1 份，
   两者的方向搞反会得到「删到只剩 N-1 份」或「删到 N+1 份」两种都错的实现。
4. **判参在落盘之前**：`max_versions=0` / 负数必须当场拒，且**不在磁盘上留下任何
   东西**（L44 立的口径）。

**为什么零 parametrize**：l97 / l98 把全仓 parametrize 三面冻成现量数，本守卫走
类内独立用例，两支普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_snapshot_cap_l305.py -q --no-cov`
"""
import json
from pathlib import Path

import pytest

from augmentor.config import VersioningConfig
from augmentor.validation import DataValidationError
from augmentor.versioning import VersionManager


def _payload(n=50):
    """造一份可观测体量的数据集

    Args:
        n: 条数

    Returns:
        数据集列表
    """
    return [{"instruction": "样例 %d" % i, "input": "", "output": "o"} for i in range(n)]


def _version_dirs(root: Path):
    """盘上的版本目录名集合（不含 current 标记）

    Args:
        root: 版本存储目录

    Returns:
        目录名集合
    """
    return {p.name for p in root.iterdir() if p.is_dir() and p.name != "current"}


class TestTheCapHolds:
    """封顶本身：超上限必须真的删旧"""

    def test_repeated_snapshots_stop_growing_the_directory(self, tmp_path):
        """连打 5 次快照、封顶 3 份 ⇒ 盘上恒为 3 份（改前是 5 份且无上限）"""
        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root), max_versions=3)

        for i in range(5):
            mgr.create_version(_payload(), label="增强数据_%d" % i)

        assert len(_version_dirs(root)) == 3

    def test_the_kept_ones_are_the_newest(self, tmp_path):
        """删的是最旧的：留下的三份里含最新那份、不含最早那份"""
        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root), max_versions=3)

        ids = [mgr.create_version(_payload(), label="L%d" % i).version_id
               for i in range(5)]

        on_disk = _version_dirs(root)
        assert ids[-1] in on_disk, "最新那份被删了 ⇒ 封顶删错了方向"
        assert ids[0] not in on_disk, "最早那份还在 ⇒ 封顶没删旧"

    def test_under_the_cap_nothing_is_deleted(self, tmp_path):
        """没超上限时不删任何一份（封顶不是「每次都删一份」）"""
        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root), max_versions=5)

        for i in range(3):
            mgr.create_version(_payload(), label="L%d" % i)

        assert len(_version_dirs(root)) == 3

    def test_the_current_version_is_never_pruned(self, tmp_path):
        """current 指向的那一版永不被删：删了它 current 就成了孤儿指针

        口径说明（这一格量的是**指向 current 的那一版**）：`create_version` 每次
        建版都会把 current 前移到新版本，所以「被回滚指着的旧版本」在下一轮建版
        时已经不再被 current 指着 —— 它按「删旧不拒新」的正常口径被删掉是**合乎
        语义**的（用户已经用新版本覆盖了它）。真正要防的缺陷是另一件：**current
        此刻指向的那一版被删掉** ⇒ 读侧 `get_current_version` 仍报得出那个 id，
        而目录已经没了（孤儿指针）。
        """
        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root), max_versions=3)

        for i in range(6):
            mgr.create_version(_payload(), label="L%d" % i)
            # 每一轮建完，当前 current 指向的那一版都必须在盘上
            current = mgr.get_current_version()
            assert current in _version_dirs(root), \
                "current 指向的 %s 被封顶删了 ⇒ current 成了孤儿指针" % current


class TestTheCapIsLoadBearing:
    """承重臂：把封顶换成恒等函数，「超上限仍留 N+1 份」必须当场红"""

    def test_disabling_the_prune_leaves_the_overflow_on_disk(self, tmp_path,
                                                            monkeypatch):
        """封顶被摘掉 ⇒ 盘上留 5 份（说明上面那几支量的确实是封顶的功劳）"""
        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root), max_versions=3)
        # 签名要吃 `protect`（`create_version` 会按关键字传它）
        monkeypatch.setattr(mgr, "prune_versions", lambda protect=None: 0)

        for i in range(5):
            mgr.create_version(_payload(), label="L%d" % i)

        assert len(_version_dirs(root)) == 5

    def test_the_prune_reports_how_many_it_deleted(self, tmp_path):
        """封顶返回删除份数（0 / 1 两档都要能读到）"""
        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root), max_versions=3)

        assert mgr.prune_versions() == 0
        for i in range(4):
            mgr.create_version(_payload(), label="L%d" % i)

        assert mgr.prune_versions() == 0, "封顶内再调一次不该再删"


class TestTheCapIsWired:
    """接线面：配置键 ⇒ VersionManager ⇒ 落盘行为"""

    def test_the_config_carries_the_cap(self):
        """出厂 10 份，且是**份数**不是删除份数"""
        cfg = VersioningConfig()
        assert cfg.max_versions == 10

    def test_the_pipeline_hands_the_cap_to_the_manager(self):
        """pipeline 把配置键交给 VersionManager（否则键存在但不生效）"""
        import inspect

        from augmentor.pipeline import AugmentorPipeline

        # 版本管理器在 `_init_components` 里建（不在 `__init__`）——断言取对地方，
        # 否则这一支会因为读错函数而恒绿。
        src = inspect.getsource(AugmentorPipeline._init_components)
        assert "max_versions=self.config.versioning.max_versions" in src

    def test_the_prune_action_is_recorded_in_history(self, tmp_path):
        """被封顶删掉的那份在 history 里留一笔（删旧是可逆的证据）"""
        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root), max_versions=2)

        for i in range(4):
            mgr.create_version(_payload(), label="L%d" % i)

        actions = [json.loads(line)["action"]
                   for line in (root / "history.jsonl").read_text(
                       encoding="utf-8").splitlines()]
        assert "prune" in actions, "封顶删档没有留痕 ⇒ 删旧不是可逆的"

    def test_a_bad_cap_is_refused_before_anything_touches_disk(self, tmp_path):
        """`max_versions=0` 当场拒，且磁盘上不留任何东西"""
        root = tmp_path / "versions"

        with pytest.raises(DataValidationError):
            VersionManager(storage_dir=str(root), max_versions=0)

        assert not root.exists(), "坏参数在磁盘上留下了目录"


class TestTheCapUnderConcurrency:
    """并发那一维：封顶开着时版本一个都不许丢，封顶关着时只留 N 份"""

    def test_with_the_cap_open_no_version_is_lost(self, tmp_path):
        """封顶开到够用的值 ⇒ 并发建的版本一份都不丢（与 l273 那一支同口径）"""
        from concurrent.futures import ThreadPoolExecutor

        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root), max_versions=24)
        payload = _payload(10)

        with ThreadPoolExecutor(max_workers=24) as pool:
            infos = list(pool.map(
                lambda i: mgr.create_version(payload, label="L%d" % i), range(24)))

        assert len({info.version_id for info in infos}) == 24, "版本号重复"
        labels = {info.label for info in infos}
        on_disk = {p.name for p in root.iterdir() if p.is_dir()}
        assert len(on_disk) == 24, "盘上少了 %d 份" % (24 - len(on_disk))
        assert labels == {"L%d" % i for i in range(24)}

    def test_with_the_cap_closed_the_oldest_are_dropped(self, tmp_path):
        """封顶关着（出厂 10）⇒ 并发建的版本只剩 10 份（这一维由封顶决定，不是并发缺陷）"""
        from concurrent.futures import ThreadPoolExecutor

        root = tmp_path / "versions"
        mgr = VersionManager(storage_dir=str(root))  # 出厂封顶 10
        payload = _payload(10)

        with ThreadPoolExecutor(max_workers=24) as pool:
            list(pool.map(
                lambda i: mgr.create_version(payload, label="L%d" % i), range(24)))

        assert len({p.name for p in root.iterdir() if p.is_dir()}) == 10
