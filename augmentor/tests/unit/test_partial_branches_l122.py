# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L122（A205）：偏支逐支审计——backup / version_control / indexer / rag / export

**可达、补测（本文件）**
- backup.py 219->223         删除备份时备份文件已被外部移走 ⇒ `if backup_path.exists():` 假支
- backup.py 331->330        自动清理时索引里出现**重复 backup_id** ⇒ 第一条删掉后
                             第二条找不到 ⇒ `if manager.delete_backup(...):` 假支
- version_control.py 245->244  `get_version` 找列表尾的版本 / 找不存在的 id ⇒
                             `if version_data["version_id"] == version_id:` 假支扫过
- version_control.py 382->384  同一实例第二次比较**不同版本对**（缓存键不同、不进 :354
                             缓存命中早退）⇒ `if not hasattr(self, '_compare_cache'):` 假支
- indexer.py 168->166       n-gram 索引字段上某条是非字符串值 ⇒
                             `if isinstance(value, str):` 假支（该条不入索引）

**判定为结构性不可达、记档不测**
- rag.py 61->68            分块循环 `for start in range(0, len(text), step)` 的回边
                             （range 自然耗尽）：任一轮 `start` 恒满足
                             `start + chunk_size >= len(text) - step + chunk_size =
                             len(text) + overlap > len(text)`（`overlap ≥ 0` 时
                             `len - step ≤ start`）⇒ :66 的 `break` 总先于回边触发，
                             回边不可达（冗余循环设计，保留不改）。
- rag.py 63->65            `if chunk:` 假支要 `start >= len(text)` 才出空切片，而
                             range 的 `start` 恒 `< len(text)` ⇒ 判据恒真，假支不可达
                             （防御式空切片检查，与 quality.py 222->230 一族）。
- export.py 277->286       `elif export_format == ExportFormat.CSV:` 假支（穿透到 :286）
                             要求一个**既不在 :240 NATIVE_FORMATS 委托判据里、又穿过前
                             五个 elif** 的格式值；而 ExportFormat 恰为六成员封闭枚举、
                             六个全被上方分支覆盖 ⇒ 假支不可达（穷举 elif 链的尾巴，
                             与 L120 的「:285 注释」同构）。
"""

import json
import pathlib

from augmentor.backup import DatasetBackup, clean_old_backups
from augmentor.indexer import DatasetIndexer
from augmentor.version_control import DatasetVersionManager


# ---------------------------------------------------------------------------
# backup.py
# ---------------------------------------------------------------------------

class TestBackupDeleteMissingFile:
    """backup.py 219->223：备份文件外部缺失 ⇒ 跳 unlink、索引照常清、仍返回成功"""

    def test_delete_backup_with_file_gone_cleans_index(self, tmp_path):
        source = tmp_path / "s.json"
        source.write_text("[1]", encoding="utf-8")
        manager = DatasetBackup(str(tmp_path / "bk"))
        info = manager.backup(str(source))
        # 外部把备份文件挪走（索引里还在）
        pathlib.Path(info.backup_path).unlink()
        assert manager.delete_backup(info.backup_id) is True
        assert manager.list_backups() == []


class TestCleanOldBackupsDuplicateIndexId:
    """backup.py 331->330：索引重复 backup_id ⇒ 第二条删除返回 False、计数不增

    构造是**索引文件**上的真实损坏形态（同一 id 两条目），走公共入口
    `clean_old_backups`；不是调私有方法传不可能的参数。
    """

    def test_duplicate_id_counts_only_one_deletion(self, tmp_path):
        d = tmp_path / "bk2"
        d.mkdir()
        payload = d / "a.dat"
        payload.write_text("x", encoding="utf-8")
        entries = []
        for i, ts in enumerate(("20260101_000000", "20260102_000000", "20260103_000000")):
            bid = "dup1" if i < 2 else "keep"
            entries.append({
                "backup_id": bid,
                "timestamp": ts,
                "source_path": str(payload),
                "backup_path": str(payload),
                "output_path": str(payload),
                "item_count": 1,
            })
        (d / "index.json").write_text(json.dumps({"backups": entries}), encoding="utf-8")

        deleted = clean_old_backups(str(d), max_backups=1)
        # 最旧两条同 id：第一条删掉全部同 id 条目（True），第二条找不到（False）
        assert deleted == 1
        remaining = DatasetBackup(str(d)).list_backups()
        assert [b["backup_id"] for b in remaining] == ["keep"]


# ---------------------------------------------------------------------------
# version_control.py
# ---------------------------------------------------------------------------

class TestVersionControlGetVersionScan:
    """version_control.py 245->244：`get_version` 扫过不匹配项 / 扫完无果返回 None"""

    def _make(self, tmp_path):
        manager = DatasetVersionManager(str(tmp_path / "v"))
        data = [{"instruction": "问", "input": "", "output": "答"}]
        first = manager.create_version(data, "a")
        second = manager.create_version(data, "b")
        return manager, first, second

    def test_get_version_scans_past_mismatch(self, tmp_path):
        manager, first, second = self._make(tmp_path)
        # 找列表尾：:245 先假（first 不匹配）再真（second 匹配）
        found = manager.get_version(second.version_id)
        assert found is not None and found.version_id == second.version_id

    def test_get_version_absent_returns_none(self, tmp_path):
        manager, *_ = self._make(tmp_path)
        # 全部扫完仍不匹配 ⇒ for 耗尽出循环（:245->244 回边）⇒ None
        assert manager.get_version("no-such-version") is None


class TestVersionControlCompareCache:
    """version_control.py 382->384：第二对不同版本对 ⇒ `_compare_cache` 已存在，跳初始化"""

    def test_second_compare_with_other_pair_skips_cache_init(self, tmp_path):
        manager = DatasetVersionManager(str(tmp_path / "v"))
        a = manager.create_version(
            [{"instruction": "Q1", "output": "A1"}, {"instruction": "Q2", "output": "A2"}], "a")
        b = manager.create_version(
            [{"instruction": "Q2", "output": "A2"}, {"instruction": "Q3", "output": "A3"}], "b")

        r1 = manager.compare_versions(a.version_id, b.version_id)
        # 反过来比（缓存键不同 ⇒ 不走 :354 早退）：:382 假支（属性已存在）
        r2 = manager.compare_versions(b.version_id, a.version_id)
        assert r1["stats"] == {"only_in_a_count": 1, "only_in_b_count": 1, "in_both_count": 1}
        assert r2["stats"] == {"only_in_a_count": 1, "only_in_b_count": 1, "in_both_count": 1}
        assert r1["version_a"] == a.version_id and r2["version_a"] == b.version_id


# ---------------------------------------------------------------------------
# indexer.py
# ---------------------------------------------------------------------------

class TestIndexerNgramSkipsNonString:
    """indexer.py 168->166：索引字段上的非字符串值被 `if isinstance(value, str):` 挡出索引"""

    def test_non_string_value_not_indexed_but_others_are(self):
        items = [
            {"instruction": "房屋价格", "output": "好"},
            {"instruction": 42, "output": "好"},
            {"instruction": "房屋面积", "output": "好"},
        ]
        indexer = DatasetIndexer(items)
        # 命中「房屋」的两条在索引里，int 值那条不在（守卫写反/删掉时它要么 crash 要么误入）
        assert indexer.search_ngram("instruction", "房屋", 2) == [0, 2]
