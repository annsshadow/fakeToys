# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L120–L121（A205）：逐支可达性判定——第二批可达假支清零 + 结构性不可达记档

逐条对应 L100 终态偏支清单（本轮判定结果写在每条用例/类 docstring 里）：

**可达、补测（本文件）**
- export.py 173->180        chatml 里 `system` 显式为空串 ⇒ 系统消息被跳过
- export.py 277->286        CSV 且 items 为空 ⇒ `if data:` 假支，不落文件
- pipeline.py 369->352      并行 + 用断点 + 某条处理失败 ⇒ `if use_checkpoint` 假支（update_progress(idx, False)）
- version_control.py 321->326  删的是「当前版本」且还有别的版本 ⇒ 回退到列表尾
- version_control.py 329->335  删的是「当前版本」且它是最后一条 ⇒ 当前版本置 None
- version_control.py 321->326  删的是「当前版本」且版本目录已被外部移走 ⇒ `if version_dir.exists()` 假支
- version_control.py 329->335  删的是「当前版本」且它是最后一条 ⇒ 当前版本置 None
- versioning.py 256->253    list_versions 遇到「目录在、metadata.json 缺」的版本目录 ⇒ 跳过

**判定为结构性不可达、记档不测（不许用绕过入口的构造硬凑）**
- quality.py 222->230      `_calculate_diversity` 入口有 `if not existing_texts: return 1.0`
                           （:181），而 `diversity_sample_size` 判据 `require_count(minimum=1)`
                           ⇒ 走到 :219 时切片 `existing_texts[:n]`（n≥1、existing_texts 非空）
                           恒非空 ⇒ `if sample_texts:` 的假支不可达，属冗余防御。
- search_enhanced.py 221->exit  `_ensure_indexes` 的锁内二次检查：公共路径 :218 已判
                           `_index_ready` 为 False 才进锁，而锁内无任何让别的线程置位
                           的代码路径 ⇒ 内层 `if not self._index_ready:` 恒真，假支不可达
                           （双检锁的防御式写法，保留不改）。
- model_manager.py 23->26    双检锁内层 `if cls._instance is None:` 的假支要线程竞态才
                           触发，单线程测试里构造不出来 ⇒ 记档。
- visualizer.py 46->exit     `if self._wordcloud is None:` 的假支（已加载过再进）由懒加载
                           语义保证第二次必为真支的补集，需同一实例两次调用才见 ⇒ 记档
                           为本轮不做（下一轮随懒加载族一起补）。
"""

import json
import shutil

import pytest


# ---------------------------------------------------------------------------
# 可达：export / pipeline / version_control / versioning
# ---------------------------------------------------------------------------

class TestExportChatmlEmptySystem:
    def test_empty_system_skips_system_message(self):
        from augmentor.export import Exporter

        exporter = Exporter()
        items = [
            # system 缺省 ⇒ 注入默认系统提示（真支）
            {"instruction": "问1", "output": "答1"},
            # system 显式空串 ⇒ `if system:` 假支，不注入系统消息
            {"instruction": "问2", "output": "答2", "system": ""},
        ]
        records = exporter._convert_to_chatml(items)
        assert records[0]["messages"][0]["role"] == "system"
        assert records[1]["messages"][0]["role"] != "system"
        assert all(m["role"] != "system" for m in records[1]["messages"])


class TestExportCsvEmptyItems:
    def test_csv_export_of_empty_items_writes_no_file(self, tmp_path):
        from augmentor.export import Exporter

        output = tmp_path / "empty.csv"
        used = Exporter().export([], str(output), "csv")
        # `if data:` 假支：没有数据就不建 CSV 文件（表头都写不出）
        assert used == "csv"
        assert not output.exists()


class TestPipelineVariantFiltering:
    """pipeline.py 195->194 可达（混入缺 instruction 键的 dict 与非 dict 变体 ⇒ 两个假支）

    顺带记档 :352 的 `if use_checkpoint:`（成功侧）与 :369（失败侧）：
    失败侧要求 `success=False`，而 `_process_single_item` 对模型异常是「catch 后
    放 (idx, True, [])」（:245 的 except 恒 True），公共路径构造不出 success=False ⇒
    记档为结构性不可达（需 monkeypatch 注入，本轮不做）。
    """

    def test_malformed_variants_filtered_out(self, tmp_path):
        from augmentor.pipeline import AugmentorPipeline

        input_file = tmp_path / "in.json"
        input_file.write_text(
            json.dumps([{"instruction": "能成功", "input": "", "output": "好"}], ensure_ascii=False),
            encoding="utf-8",
        )
        output_file = tmp_path / "out.json"

        pipeline = AugmentorPipeline()

        class _FakeBackend:
            def generate(self, prompt):
                # 混入「缺 instruction 键的 dict」与「裸字符串」⇒ :195 的
                # `isinstance(v, dict) and "instruction" in v` 两个假支同批命中
                return '[{"instruction": "改写1"}, {"foo": 1}, "裸串"]'

        pipeline.model_backend = _FakeBackend()

        pipeline.augment_dataset(
            str(input_file),
            str(output_file),
            use_checkpoint=False,
            use_quality_check=False,
            use_dedup=False,
            use_parallel=False,
        )

        # 只有带 instruction 键的变体被保留，缺键 dict 与裸串被过滤
        written = json.loads(output_file.read_text(encoding="utf-8"))
        assert written == [{"instruction": "改写1", "input": "", "output": "好"}]


class TestVersionControlDeleteCurrentVersion:
    def _make_manager(self, tmp_path):
        from augmentor.version_control import DatasetVersionManager

        manager = DatasetVersionManager(str(tmp_path / "v"))
        data = [{"instruction": "问", "input": "", "output": "答"}]
        first = manager.create_version(data, "a")
        second = manager.create_version(data, "b")
        return manager, first, second

    def test_delete_current_with_other_versions_falls_back_to_last(self, tmp_path):
        manager, first, second = self._make_manager(tmp_path)
        # 让要删的成为当前版本
        manager.set_current_version(first.version_id)
        assert manager.delete_version(first.version_id) is True
        # 当前版本被删、还有别的版本 ⇒ 回退到列表尾（:321->326 命中）
        assert manager.get_current_version() == second.version_id
        assert [v["version_id"] for v in manager.list_versions()] == [second.version_id]

    def test_delete_current_last_version_resets_current_to_none(self, tmp_path):
        manager, first, second = self._make_manager(tmp_path)
        manager.set_current_version(second.version_id)
        # 先删掉另一个（此时 second 不是当前 ⇒ 不碰当前版本分支），把 second 变成唯一
        manager.delete_version(first.version_id)
        # 现在当前版本 = 最后一条 second，删它 ⇒ current_version 归 None（:329->335 命中）
        manager.set_current_version(second.version_id)
        assert manager.delete_version(second.version_id) is True
        assert manager.get_current_version() is None
        assert manager.list_versions() == []

    def test_delete_current_version_with_missing_dir(self, tmp_path):
        manager, first, second = self._make_manager(tmp_path)
        manager.set_current_version(first.version_id)
        # 外部把版本目录挪走：索引里还有、盘上没有了
        shutil.rmtree(manager._versions_dir / first.version_id)
        assert manager.delete_version(first.version_id) is True
        # 目录不存在 ⇒ `if version_dir.exists()` 假支（:321->326），索引照常清
        assert [v["version_id"] for v in manager.list_versions()] == [second.version_id]


class TestVersioningListVersionsMissingMetadata:
    def test_version_dir_without_metadata_is_skipped(self, tmp_path):
        from augmentor.versioning import VersionManager

        manager = VersionManager(str(tmp_path / "versions"))
        items = [{"instruction": "问", "output": "答"}]
        first = manager.create_version(items, "A")
        second = manager.create_version(items, "B")
        # 把 A 的 metadata.json 抹掉、目录留着 ⇒ list_versions 跳过它（:256 假支）
        (manager.storage_dir / first.version_id / "metadata.json").unlink()
        listed = manager.list_versions()
        assert [v.version_id for v in listed] == [second.version_id]
