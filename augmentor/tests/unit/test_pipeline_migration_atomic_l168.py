# -*- coding: utf-8 -*-
"""L168 / B238：管线交付口与迁移落盘收原子写（A182 逐面收路线第十面）。

- `AugmentorPipeline` 的 `process()` / 异步变体两条最终输出边是**产品核心交付
  面**：整个管线的产物数据集经 `output_file` 落盘（API 链落数据根白名单、
  `list_data_files` 扫描读者在写窗口内拿半份）——L163/L164 同判据必收。
- `migration.migrate_file` 的迁移输出边：`/api/system/migrate` 端点路径经
  `resolve_data_path(for_write=True)` 白名单 → 同判据必收。

同轮豁免归档（读侧行为对账后全部有据）：`benchmark.save_baseline` 读侧
`load_baseline` 裸抛响亮（可恢复）；`comparison.save_comparison` /
`quality_report` / `visualize_enhanced` 报告面（CLI/API 一次性写）；`faiss.persist`
meta 边读侧裸抛响亮；`data_ops` 两边 CLI 报告面（L166 口径）。

**记档待裁（超出 l99 收边类型）**：`tracker` 实验记录是「写侧 except 吞成
log + 读侧 except 吞成 None」的 checkpoint 症状完全体——修它要同时改读/写
两侧异常语义（fail-loud 行为变更），单独轮裁；`quality_trend` 的写边有检疫
交互（`_corrupt_unquarantined` 跳过保存），同族复合形状一并待裁。
"""
import json
import os
import pathlib

import pytest

from augmentor import migration as mig_mod
from augmentor.migration import migrate_file

_MIG = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "migration.py"
_PIPE = pathlib.Path(__file__).resolve().parents[2] / "augmentor" / "pipeline.py"


def _source(tmp_path):
    src = tmp_path / "in.json"
    src.write_text(
        json.dumps([{"instruction": "q", "output": "a"}], ensure_ascii=False),
        encoding="utf-8",
    )
    return str(src)


def test_migration_write_failure_leaves_previous_output_intact(tmp_path, monkeypatch):
    """迁移写中断：既有输出保持原样——旧写法在第一行就截断交付物。"""
    out = tmp_path / "mig.json"
    migrate_file(_source(tmp_path), str(out), [])
    before = out.read_bytes()

    def boom(*args, **kwargs):
        raise RuntimeError("写中断")

    monkeypatch.setattr(mig_mod, "atomic_write_json", boom)
    with pytest.raises(RuntimeError):
        migrate_file(_source(tmp_path), str(out), [])
    monkeypatch.undo()
    assert out.read_bytes() == before
    assert not [p for p in tmp_path.iterdir() if ".tmp-" in p.name]


def test_migration_round_trip(tmp_path):
    out = tmp_path / "mig.json"
    migrate_file(_source(tmp_path), str(out), [])
    assert json.loads(out.read_text(encoding="utf-8"))


def test_pipeline_edges_use_the_atomic_throat_and_only_there():
    """静态面：管线两条最终输出边都走原子喉管，且不再有裸写对。"""
    src = _PIPE.read_text(encoding="utf-8")
    assert src.count("atomic_write_json(output_file, all_results)") == 2
    assert "with open(output_file, 'w'" not in src


def test_the_migration_edge_is_no_longer_an_open_w_plus_dump_pair():
    """静态面：迁移输出边不再命中 l99 棘轮形状。"""
    src = _MIG.read_text(encoding="utf-8")
    assert "json.dump(migrated_items" not in src
    assert "atomic_write_json(output, migrated_items)" in src
