# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 51 轮：同步续传把上一轮产物回填进多样性参照窗。

串行分支（`use_parallel=False`）的 `augment_seed` 会把 `existing_generated`
传给 `filter_by_quality` 做多样性判定。改前续传时 `existing_generated` 恒为
空数组 —— 上一轮已生成的 instruction 不在参照窗里，跨运行近重复项通过
质量过滤（白花模型调用并产重复数据）。

判据：先手工造一个「已完成前 2 条」的断点 + 输出文件（模拟上一轮中断），
续传只跑剩余 3 条；`augment_seed` 第一次调用收到的 `existing_generated`
必须包含上一轮 2 条产物的 instruction。
"""

import json
import pathlib

import pytest

import augmentor.pipeline as pl
from augmentor import AugmentorPipeline, load_config

REPO = pathlib.Path(__file__).resolve().parents[2]


@pytest.fixture
def pipeline(tmp_path, monkeypatch):
    config = load_config(str(REPO / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    config.versioning.auto_snapshot = False
    instance = AugmentorPipeline(config)
    instance.version_manager.storage_dir = tmp_path / "versions"
    instance.checkpoint_manager.checkpoint_dir = tmp_path / "checkpoints"
    monkeypatch.chdir(tmp_path)
    return instance


def test_sync_resume_refills_diversity_window(tmp_path, pipeline):
    items = [{"instruction": f"问题{i}？", "input": "", "output": f"回答{i}"}
             for i in range(5)]
    src = tmp_path / "sync_in.json"
    src.write_text(json.dumps(items, ensure_ascii=False), encoding="utf-8")
    out = str(tmp_path / "sync_out.json")

    # 模拟上一轮中断：断点显示前 2 条已完成，输出文件含这 2 条产物。
    task_id = pipeline.build_task_id(str(src))
    ck = pipeline.checkpoint_manager.create_checkpoint(task_id, len(items))
    ck.completed_indices = [0, 1]
    pipeline.checkpoint_manager.save_checkpoint(ck)
    (tmp_path / "sync_out.json").write_text(
        json.dumps([{**items[i], "input": "变体"} for i in (0, 1)], ensure_ascii=False),
        encoding="utf-8",
    )

    seen: list = []

    def fake_augment_seed(self, item, use_quality_check=None, existing_generated=None):
        seen.append((item["instruction"], list(existing_generated or [])))
        return [{**item, "input": "变体"}]

    pl.AugmentorPipeline.augment_seed = fake_augment_seed

    report = pipeline.augment_dataset(
        src, out,
        use_checkpoint=True,
        use_quality_check=True,
        use_dedup=False,
        use_parallel=False,
    )
    assert report["errors"] == [], report["errors"]
    # 续传只跑剩余的 3 条（index 2,3,4）。
    assert [ins for ins, _ in seen] == ["问题2？", "问题3？", "问题4？"], seen
    # 第一次调用（index 2）的参照窗必须包含上一轮 2 条产物的 instruction。
    assert seen[0][1] == ["问题0？", "问题1？"], (
        f"续传轮的多样性参照窗必须包含上一轮成功产物，实际 {seen[0][1]!r}"
    )