# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""AU-P1 第二批：`augment_dataset` 的异步引擎（`use_async=True`）。

# 为什么必须有「与同步档同结果」这条判据

异步分支重写的是**结果归并与断点推进**这一段最容易分叉的逻辑。判据不能是
「异步档跑完了」—— 那在结果错位、变体丢失、断点漏推时同样成立。所以主判据是
**同一份输入跑两档，输出逐字节相同**，其余用例只钉异步特有的三件事：
并发真的重叠、单条失败不拖垮整批、没有原生异步的后端会回退而不是假装异步。
"""

import asyncio
import json
import logging

import pytest

from augmentor.config import AppConfig
from augmentor.pipeline import AugmentorPipeline

ITEMS = [{"instruction": f"q{i}", "input": "", "output": f"a{i}"} for i in range(6)]


def _instruction_of(prompt: str) -> str:
    """从提示词里取回 instruction（提示词以 `原JSON对象如下：` 结尾接 JSON）"""
    tail = prompt.split("原JSON对象如下：", 1)[1]
    return json.loads(tail)[0]["instruction"]


class NativeAsyncBackend:
    """有原生异步实现（覆写 `_acall_api`）的替身。"""

    def __init__(self, per_seed=2, fail_for=(), delay=0.0):
        self.per_seed = per_seed
        self.fail_for = set(fail_for)
        self.delay = delay
        self.calls = 0
        self.in_flight = 0
        self.max_in_flight = 0
        self._lock = None  # 事件循环单线程，不需要真锁

    async def _acall_api(self, prompt: str) -> str:
        self.calls += 1
        self.in_flight += 1
        self.max_in_flight = max(self.max_in_flight, self.in_flight)
        try:
            if self.delay:
                await asyncio.sleep(self.delay)
            text = _instruction_of(prompt)
            if text in self.fail_for:
                raise RuntimeError(f"boom {text}")
            variants = [{"instruction": f"{text}-v{k}"} for k in range(self.per_seed)]
            return json.dumps(variants, ensure_ascii=False)
        finally:
            self.in_flight -= 1

    async def agenerate(self, prompt: str, **_kwargs) -> str:
        return await self._acall_api(prompt)

    def generate(self, prompt: str, **_kwargs) -> str:
        """同步档用这一支做对照。语义与 `_acall_api` 一字不差。"""
        self.calls += 1
        text = _instruction_of(prompt)
        if text in self.fail_for:
            raise RuntimeError(f"boom {text}")
        variants = [{"instruction": f"{text}-v{k}"} for k in range(self.per_seed)]
        return json.dumps(variants, ensure_ascii=False)


class SyncOnlyBackend:
    """只有同步实现的替身（没有 `_acall_api`）⇒ 应回退线程池。"""

    def __init__(self, per_seed=2):
        self.per_seed = per_seed
        self.calls = 0

    def generate(self, prompt: str, **_kwargs) -> str:
        self.calls += 1
        text = _instruction_of(prompt)
        return json.dumps(
            [{"instruction": f"{text}-v{k}"} for k in range(self.per_seed)],
            ensure_ascii=False,
        )


@pytest.fixture
def files(tmp_path):
    src = tmp_path / "in.json"
    src.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return str(src), tmp_path


def _pipeline(monkeypatch, tmp_path) -> AugmentorPipeline:
    monkeypatch.chdir(tmp_path)
    return AugmentorPipeline(config=AppConfig())


def _run(pipe, files, backend, *, use_async, name):
    src, tmp = files
    pipe.model_backend = backend
    out = str(tmp / name)
    return pipe.augment_dataset(
        src, out,
        use_checkpoint=False,
        use_quality_check=False,
        use_dedup=False,
        use_parallel=True,
        use_async=use_async,
    ), out


def test_async_matches_sync_as_a_set(monkeypatch, tmp_path, files):
    """两档的**内容**必须逐条相同；**顺序**刻意不同（见下）。

    ⚠️ 这是本文件最重要的一格，也是实测抓出来的真实差异：

    - 线程档用 `as_completed` ⇒ 输出顺序是**完成序**（哪条先跑完先落），
      同一次输入跑两遍就可能不同；
    - 异步档用 `gather` ⇒ 输出顺序是**提交序**（下标序），可复现。

    顺序对数据集本身没有语义（变体之间无序），但它是可观察行为，所以这里
    明写判据而不是让"逐字节相同"悄悄变成假绿：内容按多重集比，顺序另行钉死。
    """
    pipe = _pipeline(monkeypatch, tmp_path)

    report_sync, out_sync = _run(pipe, files, NativeAsyncBackend(), use_async=False,
                                 name="out_sync.json")
    report_async, out_async = _run(pipe, files, NativeAsyncBackend(), use_async=True,
                                   name="out_async.json")

    assert report_sync["output_count"] == report_async["output_count"]
    with open(out_sync, encoding="utf-8") as f:
        sync_data = json.load(f)
    with open(out_async, encoding="utf-8") as f:
        async_data = json.load(f)

    key = lambda d: json.dumps(d, ensure_ascii=False, sort_keys=True)
    assert sorted(map(key, async_data)) == sorted(map(key, sync_data)), \
        "两档的内容必须逐条相同（顺序另判）"
    assert report_async["errors"] == []


def test_async_output_order_is_submission_order(monkeypatch, tmp_path, files):
    """异步档的顺序必须是**提交序**，且两次运行完全一致（可复现）。

    线程档做不到这一点：`as_completed` 的产出顺序取决于谁先返回。这不是缺陷，
    但它是"开了异步之后输出文件长什么样"的一部分，必须钉住。
    """
    pipe = _pipeline(monkeypatch, tmp_path)

    _, out1 = _run(pipe, files, NativeAsyncBackend(delay=0.0), use_async=True,
                   name="o1.json")
    _, out2 = _run(pipe, files, NativeAsyncBackend(delay=0.0), use_async=True,
                   name="o2.json")

    with open(out1, encoding="utf-8") as f:
        d1 = json.load(f)
    with open(out2, encoding="utf-8") as f:
        d2 = json.load(f)
    assert d1 == d2, "同样的输入跑两遍，异步档的输出必须逐字节一致"
    assert [x["instruction"] for x in d1] == [
        f"q{i}-v{k}" for i in range(len(ITEMS)) for k in range(2)
    ], "异步档应按提交序输出"


def test_async_actually_overlaps(monkeypatch, tmp_path, files):
    pipe = _pipeline(monkeypatch, tmp_path)
    backend = NativeAsyncBackend(delay=0.02)

    _run(pipe, files, backend, use_async=True, name="out.json")

    # 6 条、每条睡 20ms：串行要 120ms 且 max_in_flight 恒为 1。
    assert backend.calls == len(ITEMS)
    assert backend.max_in_flight > 1, "并发没有真的重叠，等于退回串行"
    assert backend.max_in_flight <= len(ITEMS)


def test_one_failed_item_does_not_kill_the_batch(monkeypatch, tmp_path, files):
    pipe = _pipeline(monkeypatch, tmp_path)
    backend = NativeAsyncBackend(fail_for={"q2"})

    report, out = _run(pipe, files, backend, use_async=True, name="out.json")

    with open(out, encoding="utf-8") as f:
        data = json.load(f)
    generated = {item["instruction"] for item in data}
    assert "q2-v0" not in generated and "q2-v1" not in generated
    for i in (0, 1, 3, 4, 5):
        assert f"q{i}-v0" in generated, f"q{i} 的变体应仍在结果里"
    # `_agenerate_variants` 把模型侧异常吞成空列表（与同步版同口径），
    # 所以这一条不进 errors —— 这里钉住「不因为一条失败就整批失败」。
    assert report["errors"] == []


def test_falls_back_to_threads_without_native_async(monkeypatch, tmp_path, files,
                                                    caplog):
    pipe = _pipeline(monkeypatch, tmp_path)
    backend = SyncOnlyBackend()

    with caplog.at_level(logging.WARNING):
        report, out = _run(pipe, files, backend, use_async=True, name="out.json")

    assert backend.calls == len(ITEMS), "回退后必须真的把每条都跑了"
    with open(out, encoding="utf-8") as f:
        assert len(json.load(f)) == len(ITEMS) * 2
    assert any("回退线程池" in rec.message for rec in caplog.records), \
        "回退必须出声：静默回退会让「我开了异步」变成一句无法验证的话"


def test_async_advances_checkpoint_for_every_item(monkeypatch, tmp_path, files):
    pipe = _pipeline(monkeypatch, tmp_path)
    src, tmp = files
    pipe.model_backend = NativeAsyncBackend()

    report = pipe.augment_dataset(
        src, str(tmp / "out.json"),
        use_checkpoint=True,
        use_quality_check=False,
        use_dedup=False,
        use_parallel=True,
        use_async=True,
    )

    progress = report["progress"]
    assert progress is not None
    assert progress["processed_items"] == len(ITEMS), \
        "异步档的断点推进必须与同步档一样逐条落（少推会让续传重跑已完成条目）"
