"""L206 守卫 v5（最终版）：跑串行支路做行为断言，并行支路用源码级判据。

为什么不跑并行：并行支路的 `_process_single_item` 既要干活又要往 `result_queue`
put，stub 它就得连 queue 语义一起仿，那就是在测自己的桩而不是测产品（账本 §2.7
「同源预言机是假测试」的同款）。并行支路的明细写入因此改用**源码级**判据：
两条支路写进的字典形状必须一致、且都不能再惰性首建。
"""
import inspect
import json

import pytest

from augmentor.config import AppConfig
from augmentor.pipeline import AugmentorPipeline

ITEMS = [{"instruction": f"q{i}", "input": "", "output": f"a{i}"} for i in range(4)]


@pytest.fixture
def files(tmp_path):
    src = tmp_path / "in.json"
    src.write_text(json.dumps(ITEMS, ensure_ascii=False), encoding="utf-8")
    return str(src), str(tmp_path / "out.json")


@pytest.fixture
def pipeline(monkeypatch, tmp_path):
    monkeypatch.chdir(tmp_path)
    real = AugmentorPipeline(config=AppConfig())
    monkeypatch.setattr(real.checkpoint_manager, "get_progress",
                        lambda *a, **k: None)
    return real


def _boom_at(pipeline, monkeypatch, boom_indices):
    """只让 instruction 形如 `q<下标>` 的条目抛异常（不依赖对象同一性）"""
    real_seed = pipeline.augment_seed

    def fake_seed(seed_item, **kwargs):
        text = str(seed_item.get("instruction", ""))
        idx = int(text[1:]) if text[:1] == "q" and text[1:].isdigit() else -1
        if idx in boom_indices:
            raise RuntimeError(f"boom {idx}")
        return real_seed(seed_item, use_quality_check=False,
                         existing_generated=[])

    monkeypatch.setattr(pipeline, "augment_seed", fake_seed)


def _run(pipeline, files):
    src, out = files
    # 固定走串行：见模块 docstring
    return pipeline.augment_dataset(src, out, use_checkpoint=False,
                                    use_parallel=False)


class TestErrorsAreReported:
    def test_errors_key_exists_and_is_empty_on_success(self, pipeline, monkeypatch,
                                                       files):
        monkeypatch.setattr(pipeline, "augment_seed", lambda seed, **k: [])
        report = _run(pipeline, files)
        assert "errors" in report, "报告里没有 errors 键 ⇒ 明细仍然没有出口"
        assert report["errors"] == []

    def test_failed_item_shows_up_with_context(self, pipeline, monkeypatch, files):
        _boom_at(pipeline, monkeypatch, {1})
        errors = _run(pipeline, files)["errors"]
        assert len(errors) == 1, errors
        assert errors[0]["index"] == 1
        assert errors[0]["error_type"] == "RuntimeError"
        assert "boom 1" in errors[0]["message"]
        assert errors[0]["item_preview"], "失败明细缺条目预览 ⇒ 无法定位是哪条数据"

    def test_errors_is_a_copy_not_the_live_list(self, pipeline, monkeypatch, files):
        monkeypatch.setattr(pipeline, "augment_seed", lambda seed, **k: [])
        report = _run(pipeline, files)
        report["errors"].append({"index": 999})
        assert pipeline._process_errors == [], (
            "报告返回的是内部 list 本身 ⇒ 调用方一改就污染管道状态"
        )


class TestErrorsDoNotLeakAcrossRuns:
    def test_second_run_starts_clean(self, pipeline, monkeypatch, files):
        _boom_at(pipeline, monkeypatch, {0})
        first = _run(pipeline, files)
        assert len(first["errors"]) == 1

        monkeypatch.setattr(pipeline, "augment_seed", lambda seed, **k: [])
        second = _run(pipeline, files)
        assert second["errors"] == [], (
            "第二批混进了第一批的错误 ⇒ _process_errors 跨调用累积未清"
        )

    def test_internal_list_is_reset_too(self, pipeline, monkeypatch, files):
        """L275 重钉：一次运行**不往共享实例属性上留**明细

        改前这一支断言的是「跑完一批，`pipeline._process_errors` 里正好是本批那 1 笔，
        第二批之后清零」——那时它就是累加器。L275 把累加器换成 `augment_dataset`
        的局部清单之后，实例属性只剩「不传 `errors=` 的直接调用方」这一个默认出口，
        所以一次 `augment_dataset` 跑完这一格必须始终是空：非空就说明明细又写回共享态，
        并发跑两批时两批会互相看见对方的错（实测 139 笔全属对方）。
        两批各出各的明细这件事由第一批那 1 笔与第二批那 0 笔继续钉住，
        所以本支只换落点判据，不放宽任何一格。
        """
        _boom_at(pipeline, monkeypatch, {0})
        first = _run(pipeline, files)
        assert len(first["errors"]) == 1
        assert pipeline._process_errors == [], (
            "本批明细写到了共享实例属性上 ⇒ 并发两批互相污染"
        )
        # 第二批不再炸：boom 换掉后才看得出「清零」不是「没新增」
        monkeypatch.setattr(pipeline, "augment_seed", lambda seed, **k: [])
        _run(pipeline, files)
        assert pipeline._process_errors == []


class TestExistingKeysAreUnchanged:
    def test_report_key_set(self, pipeline, monkeypatch, files):
        monkeypatch.setattr(pipeline, "augment_seed", lambda seed, **k: [])
        report = _run(pipeline, files)
        expected = {
            "task_id", "input_file", "output_file", "input_count",
            "output_count", "quality_check", "dedup", "parallel",
            "errors", "progress", "memory_monitor",
            # L216 新增：`vector.enabled` 打开时的建库回执（关掉是 null）。
            # 这一格是「键集只认逐条签字的新增」，不是「永远不许变」——
            # 新增没在这里签过字的键照样红。
            "vector_index",
        }
        assert set(report) == expected, (
            f"报告键集变了：少 {expected - set(report)}、"
            f"多 {set(report) - expected}"
        )


class TestBothBranchesRecordTheSameShape:
    """并行支路的行为由源码级判据覆盖（理由见模块 docstring）"""

    def test_process_errors_exists_after_construction(self):
        assert "_process_errors" in inspect.getsource(AugmentorPipeline.__init__), (
            "__init__ 里没有 _process_errors ⇒ 又回到惰性首建"
        )

    def test_no_lazy_hasattr_creation_left(self):
        source = inspect.getsource(AugmentorPipeline._process_single_item)
        assert "hasattr(self, '_process_errors')" not in source, (
            "并行支路还在惰性首建 ⇒ 并发下重复建 list 的老坑有复发空间"
        )

    def test_both_branches_write_the_same_shape(self):
        parallel = inspect.getsource(AugmentorPipeline._process_single_item)
        serial = inspect.getsource(AugmentorPipeline.augment_dataset)
        for field in ('"index"', '"error_type"', '"message"', '"item_preview"'):
            assert field in parallel, f"并行支路缺 {field}"
            assert field in serial, f"串行支路缺 {field}"

    def test_serial_branch_writes_the_run_local_list(self):
        """串行支路的落点是**本批局部清单**（L275），不再是共享实例属性

        L206 时这一支钉的是「串行 append 必须持 `self._lock`」，理由是它和并行支路的
        工作线程写同一个共享 list。L275 把写面换成本运行自己的清单后，锁的理由消失了
        （一次运行只走串行或并行其中一支，串行这一支只有调用方一个线程），要守的
        变成另一件事：**别写回共享态**。写成 `self._process_errors` 的话，
        并发两批的明细会互相看见——所以这里的反向判据和持锁判据等重，不是放宽。
        """
        serial = inspect.getsource(AugmentorPipeline.augment_dataset)
        i = serial.find("处理失败")
        assert i > 0, "找不到串行支路的 except 块"
        tail = serial[i:i + 600]
        assert "errors.append({" in tail, (
            "串行支路没写进本批清单 ⇒ 报告里的 errors 与明细来源分家"
        )
        assert "self._process_errors.append" not in tail, (
            "串行支路又往共享实例属性 append ⇒ 并发两批互相污染（L275）"
        )

    def test_parallel_branch_appends_under_lock(self):
        """并行支路的 append 仍须**嵌在** `with self._lock` 块里

        同一批的多个工作线程写的是同一份本批清单，所以 L206 那把锁的理由在这里不消失。
        判据写成「缩进层级」而不是「前面 200 字符里出现过」：后者把 append 行脱出
        with 块也判绿，那是恒真的一半形状。
        """
        lines = inspect.getsource(
            AugmentorPipeline._process_single_item).splitlines()
        app = [k for k, ln in enumerate(lines) if ".append(error_info)" in ln]
        assert len(app) == 1, f"并行支路的 append 应当恰好一处，实测 {len(app)}"
        k = app[0]
        indent = len(lines[k]) - len(lines[k].lstrip())
        lock = [i for i in range(k, -1, -1)
                if lines[i].lstrip().startswith("with self._lock")]
        assert lock, "并行支路的 append 不持锁 ⇒ 同批多线程写同一份清单会丢记录"
        i = lock[0]
        assert indent > (len(lines[i]) - len(lines[i].lstrip())), (
            "append 在 with 块**之外**：持锁那一格和写那一格不同一支"
        )
