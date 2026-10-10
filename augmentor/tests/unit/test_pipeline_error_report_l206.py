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
        _boom_at(pipeline, monkeypatch, {0})
        _run(pipeline, files)
        assert len(pipeline._process_errors) == 1
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

    def test_serial_branch_records_under_lock(self):
        """串行支路 append 必须持锁——它和工作线程跑在同一个实例上"""
        serial = inspect.getsource(AugmentorPipeline.augment_dataset)
        i = serial.find("处理失败")
        assert i > 0, "找不到串行支路的 except 块"
        tail = serial[i:i + 600]
        assert "with self._lock" in tail, (
            "串行支路的 append 没持锁 ⇒ 与并行支路并发时可能丢记录"
        )
