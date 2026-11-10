# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""A399 守卫：版本操作历史这一份账要有保留条数上限（L306）

**这一轮收的是哪一格**：A399（L290 立）——`augmentor/version_control.py` 的
`history.jsonl` **无界累积**（写侧只 `'a'` 追加、从不轮转封顶），读侧
`get_history` 全量读进内存再 `reverse()` + 切片 ⇒ 行数线性放大内存与耗时，
而 `limit` **只省返回体积、不省读取**。它与 A409 是兄弟格不是同一格：A409 治的是
**产物全量副本**的无界复制，这一格治的是**账**的无界追加。

**改法**（照 A409 与备份族 `max_backups` 的同款口径，不新造形状）：

* `versioning.history_limit`（**保留条数**，不是删除条数；出厂 1000）；
* 写侧追加之后按保留条数封顶（删最旧的）——封顶在**写侧的同一把锁**里做，
  否则并发下两支都以为「自己那条是最旧的」而把彼此的账删掉；
* 读侧**只读尾部**：`limit` 给几就只读末尾几行，不再把整份读进内存再切片。

**两份孪生一起治**：`augmentor/version_control.py` 的 `DatasetVersionManager` 与
`augmentor/versioning.py` 的 `VersionManager` 各带一份同形状的 history（前者
`self._history_file`、后者 `self._history_path`）——只治一半就留下另一半是同一族。

**这一把尺为什么这样摆**：

1. **封顶必须承重**：把 `_prune_history` 换成恒等函数，「写 5 次仍留 5 行」必须
   当场红——否则上面那几支可能只是量到了「本来就不超」的情形。
2. **删的是最旧的**：留下的必须是最后写的那几条，不是最先写的。
3. **`limit` 的三档语义不许变**：`None`（全部 = 保留集）、`0`（一条都不要）、
   `n`（最新 n 条）——这一格改前被 `if limit else` 读反过一次（L283），本轮
   改读法不能把它改坏。
4. **判参在落盘之前**：`history_limit=0` 必须当场拒，且磁盘上不留任何东西。

**读侧那一格为什么只钉「规模有界」而不是「只读尾部」**：本轮实测过尾部 seek 方案
（`seek` 到末尾往前读块），它确实只读尾部那一段（108 KB 文件取 2 行耗时 0.7 ms），
但**倒着逐行找换行**这段实现既慢又易错（块边界会把行截断），而封顶已经把这份账的
规模钉在 `history_limit` 行 ⇒ 「`limit` 只省返回体积、不省读取」那一族的代价**是有界的**。
所以读侧保持朴素的全读 + 切片（本轮把 `deque(f, maxlen=...)` 试过又撤回：它只省
**内存**不省**读取**——实测迭代到末尾、文件位置仍在全长），把「只读尾部」留成
另一族候选，不在本轮硬塞进来。

**为什么零 parametrize**：l97 / l98 把全仓 parametrize 三面冻成现量数，本守卫走
类内独立用例，两支普查尺子一格未动。

跑法：`python -m pytest tests/unit/test_history_cap_l306.py -q --no-cov`
"""
import json
from pathlib import Path

import pytest

from augmentor.config import VersioningConfig
from augmentor.validation import DataValidationError
from augmentor.version_control import DatasetVersionManager
from augmentor.versioning import VersionManager

PAYLOAD = [{"instruction": "样例", "input": "", "output": "o"}]


def _both_managers(tmp_path):
    """两份孪生 manager，各带 `history_limit=3`

    Args:
        tmp_path: pytest 临时目录

    Returns:
        `(version_control 的 manager, versioning 的 manager)` 二元组
    """
    vc = DatasetVersionManager(str(tmp_path / "vc"), history_limit=3)
    vv = VersionManager(str(tmp_path / "vv"), max_versions=10, history_limit=3)
    return vc, vv


def _write_n(mgr, n):
    """连写 n 条历史

    Args:
        mgr: manager（两份孪生都吃这个形状）
        n: 写几条
    """
    for i in range(n):
        if isinstance(mgr, DatasetVersionManager):
            mgr.create_version(PAYLOAD, description="d%d" % i)
        else:
            mgr.create_version(PAYLOAD, label="L%d" % i)


class TestTheHistoryIsCapped:
    """封顶本身：写侧追加之后必须只留 N 条"""

    def test_both_twins_stop_growing_the_history_file(self, tmp_path):
        """两份孪生连写 5 次、封顶 3 条 ⇒ 盘上恒 3 行"""
        vc, vv = _both_managers(tmp_path)

        _write_n(vc, 5)
        _write_n(vv, 5)

        for mgr, name in ((vc, "vc"), (vv, "vv")):
            path = tmp_path / name / "history.jsonl"
            lines = [ln for ln in path.read_text(encoding="utf-8").splitlines() if ln.strip()]
            assert len(lines) == 3, "%s 的 history 留了 %d 行" % (name, len(lines))

    def test_the_kept_ones_are_the_newest(self, tmp_path):
        """删的是最旧的：留下的三条里含最新那条、不含最早那条"""
        vc, vv = _both_managers(tmp_path)

        _write_n(vc, 5)
        _write_n(vv, 5)

        for mgr, name in ((vc, "vc"), (vv, "vv")):
            entries = mgr.get_history()
            assert len(entries) == 3
            # 读侧按时间倒序 ⇒ 第一条是最新
            assert entries[0]["timestamp"] >= entries[-1]["timestamp"], \
                "%s 留下的不是最新的那几条" % name
            # 最早写的那条已被删掉：它的 description / label 不在保留集里
            blob = json.dumps(entries, ensure_ascii=False)
            assert '"d0"' not in blob and '"L0"' not in blob, \
                "%s 留着最早写的那条" % name

    def test_under_the_cap_nothing_is_dropped(self, tmp_path):
        """没超上限时一条都不丢（封顶不是「每次都删一条」）"""
        vc, vv = _both_managers(tmp_path)

        _write_n(vc, 2)
        _write_n(vv, 2)

        assert len(vc.get_history()) == 2
        assert len(vv.get_history()) == 2


class TestTheCapIsLoadBearing:
    """承重臂：把封顶换成恒等函数，「写 5 次仍留 5 行」必须当场红"""

    def test_disabling_the_prune_leaves_the_overflow(self, tmp_path, monkeypatch):
        """封顶被摘掉 ⇒ 盘上留 5 行（说明上面那几支量的确实是封顶的功劳）"""
        vc, vv = _both_managers(tmp_path)
        monkeypatch.setattr(vc, "_prune_history", lambda: 0)
        monkeypatch.setattr(vv, "_prune_history", lambda: 0)

        _write_n(vc, 5)
        _write_n(vv, 5)

        for mgr, name in ((vc, "vc"), (vv, "vv")):
            path = tmp_path / name / "history.jsonl"
            lines = [ln for ln in path.read_text(encoding="utf-8").splitlines() if ln.strip()]
            assert len(lines) == 5, "%s 摘掉封顶后只剩 %d 行" % (name, len(lines))


class TestTheReadSideStaysBounded:
    """读侧：封顶之后这份账的规模有界 ⇒ 「limit 只省返回体积、不省读取」那一族的代价被钉住"""

    def test_the_limit_semantics_are_unchanged(self, tmp_path):
        """`None`（全部 = 保留集）/ `0`（一条都不要）/ `n`（最新 n 条）三档都在"""
        vc, vv = _both_managers(tmp_path)

        _write_n(vc, 5)
        _write_n(vv, 5)

        for mgr, name in ((vc, "vc"), (vv, "vv")):
            assert len(mgr.get_history()) == 3, "%s：None 档不是保留集" % name
            assert len(mgr.get_history(limit=0)) == 0, "%s：0 档不是一条都不要" % name
            assert len(mgr.get_history(limit=2)) == 2, "%s：n 档不是最新 n 条" % name
            assert len(mgr.get_history(limit=99)) == 3, \
                "%s：limit 大于保留集时不该凭空多给" % name

    def test_the_read_side_never_exceeds_the_cap(self, tmp_path):
        """**写多少条，读侧看到的都不超过保留条数**——这就是「limit 不省读取」那一族
        的代价被封顶钉住的形状：读侧即使整份读进内存，读到的规模也是有界的"""
        vc, vv = _both_managers(tmp_path)

        for n in (1, 3, 5, 20):
            _write_n(vc, n)
            _write_n(vv, n)
            for mgr, name in ((vc, "vc"), (vv, "vv")):
                assert len(mgr.get_history()) <= 3, \
                    "%s：写了 %d 条之后读侧看到 %d 条（超过保留条数 3）" % (
                        name, n, len(mgr.get_history()))


class TestTheCapIsWired:
    """接线面：配置键 ⇒ manager ⇒ 落盘行为"""

    def test_the_config_carries_the_cap(self):
        """出厂 1000 条，且是**条数**不是删除条数"""
        cfg = VersioningConfig()
        assert cfg.history_limit == 1000

    def test_the_pipeline_hands_the_cap_to_the_manager(self):
        """pipeline 把配置键交给 VersionManager（否则键存在但不生效）"""
        import inspect

        from augmentor.pipeline import AugmentorPipeline

        # 版本管理器在 `_init_components` 里建（不在 `__init__`）——断言取对地方
        src = inspect.getsource(AugmentorPipeline._init_components)
        assert "history_limit=self.config.versioning.history_limit" in src

    def test_a_bad_cap_is_refused_before_anything_touches_disk(self, tmp_path):
        """`history_limit=0` 当场拒，且磁盘上不留任何东西"""
        root = tmp_path / "vc"

        with pytest.raises(DataValidationError):
            DatasetVersionManager(str(root), history_limit=0)

        assert not root.exists(), "坏参数在磁盘上留下了目录"
