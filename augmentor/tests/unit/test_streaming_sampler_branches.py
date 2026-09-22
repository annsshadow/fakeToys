"""streaming / sampler 剩余分支补测

streaming：内存监控快照分支（块 >100 条）、StreamWriter.close() 的
JSON 收尾；sampler：问题类型 what 分支、长度推荐三分支、
sklearn 可用时的加载成功路径、空输入推荐。
"""

import json
import sys
import types
from pathlib import Path

import pytest


def _write_jsonl(path, items):
    with open(path, "w", encoding="utf-8") as f:
        for item in items:
            f.write(json.dumps(item, ensure_ascii=False) + "\n")


class TestStreamMemoryMonitor:
    def test_snapshot_taken_for_large_chunk(self, tmp_path):
        """单块超过 100 条时应触发内存快照（覆盖 take_snapshot 分支）"""
        from augmentor.streaming import StreamAugmentor

        items = [{"instruction": f"问题{i}", "input": "", "output": "答"} for i in range(150)]
        src = tmp_path / "in.jsonl"
        _write_jsonl(src, items)
        out = tmp_path / "out.jsonl"

        augmentor = StreamAugmentor(
            input_file=str(src),
            output_file=str(out),
            processor=lambda chunk: chunk,
            chunk_size=150,
        )
        report = augmentor.augment()
        assert report["total_input"] == 150
        assert report["total_output"] == 150


class TestStreamWriterClose:
    def test_json_close_writes_terminator(self, tmp_path):
        """显式 close() 在 JSON 模式下需写收尾括号（非上下文管理器路径）"""
        from augmentor.streaming import StreamWriter

        target = tmp_path / "close.json"
        writer = StreamWriter(str(target), mode="w", format="json")
        writer.__enter__()
        writer.write_chunk([{"a": 1}, {"b": 2}])
        writer.close()

        assert writer._file is None
        data = json.loads(target.read_text(encoding="utf-8"))
        assert data == [{"a": 1}, {"b": 2}]

    def test_write_chunk_without_open_raises(self, tmp_path):
        from augmentor.streaming import StreamWriter

        writer = StreamWriter(str(tmp_path / "x.json"))
        with pytest.raises(RuntimeError, match="未打开"):
            writer.write_chunk([{"a": 1}])


class TestSamplerQuestionType:
    def test_what_type(self):
        from augmentor.sampler import ActiveSampler

        sampler = ActiveSampler()
        assert sampler._analyze_question_type("什么是人工智能") == "what"
        assert sampler._analyze_question_type("X 是什么") == "what"

    def test_how_type(self):
        from augmentor.sampler import ActiveSampler

        sampler = ActiveSampler()
        assert sampler._analyze_question_type("如何退租") == "how"

    def test_why_type(self):
        from augmentor.sampler import ActiveSampler

        sampler = ActiveSampler()
        assert sampler._analyze_question_type("为什么被拒") == "why"

    def test_can_type(self):
        from augmentor.sampler import ActiveSampler

        sampler = ActiveSampler()
        assert sampler._analyze_question_type("可以月付吗") == "can"

    def test_how_many_type(self):
        from augmentor.sampler import ActiveSampler

        sampler = ActiveSampler()
        assert sampler._analyze_question_type("多少钱") == "how_many"

    def test_other_type(self):
        from augmentor.sampler import ActiveSampler

        sampler = ActiveSampler()
        assert sampler._analyze_question_type("申请流程") == "other"


class TestSamplerLengthSeeds:
    @staticmethod
    def _items(length_plan):
        """按 (长度类别, 数量) 计划构造 items"""
        sample_by_class = {
            "short": "短问题",            # len < 10
            "medium": "这是一段中等长度的问题示例",  # 10 <= len < 30
            "long": "这是一段很长的问题" * 4,        # len >= 30
        }
        items = []
        for cls, count in length_plan:
            for _ in range(count):
                items.append(
                    {"instruction": sample_by_class[cls], "input": "", "output": "答"}
                )
        return items

    def test_short_length_seed(self):
        """short 占比 <0.1 时推荐 short seed"""
        from augmentor.sampler import ActiveSampler

        items = self._items([("short", 1), ("medium", 10)])
        result = ActiveSampler().recommend_seeds(items)
        short_seeds = [s for s in result.recommended_seeds if len(s["instruction"]) < 10]
        assert short_seeds

    def test_medium_length_seed(self):
        from augmentor.sampler import ActiveSampler

        items = self._items([("medium", 1), ("long", 10)])
        result = ActiveSampler().recommend_seeds(items)
        medium_seeds = [
            s for s in result.recommended_seeds if 10 <= len(s["instruction"]) < 30
        ]
        assert medium_seeds

    def test_long_length_seed(self):
        from augmentor.sampler import ActiveSampler

        items = self._items([("long", 1), ("short", 10)])
        result = ActiveSampler().recommend_seeds(items)
        long_seeds = [s for s in result.recommended_seeds if len(s["instruction"]) >= 30]
        assert long_seeds

    def test_empty_input_recommendation(self):
        from augmentor.sampler import ActiveSampler

        result = ActiveSampler().recommend_seeds([])
        assert result.recommended_seeds == []
        assert any("数据为空" in r for r in result.recommendations)


class TestSamplerSklearnPath:
    def test_load_model_success_when_sklearn_present(self, monkeypatch):
        """注入假 sklearn 模块需走加载成功路径（_use_sklearn=True）"""
        from augmentor.sampler import ActiveSampler

        class _Tfidf:
            def __init__(self, **kwargs):
                pass

        class _KMeans:
            def __init__(self, **kwargs):
                pass

        sklearn_mod = types.ModuleType("sklearn")
        feature_mod = types.ModuleType("sklearn.feature_extraction")
        text_mod = types.ModuleType("sklearn.feature_extraction.text")
        text_mod.TfidfVectorizer = _Tfidf
        cluster_mod = types.ModuleType("sklearn.cluster")
        cluster_mod.KMeans = _KMeans
        feature_mod.text = text_mod

        monkeypatch.setitem(sys.modules, "sklearn", sklearn_mod)
        monkeypatch.setitem(sys.modules, "sklearn.feature_extraction", feature_mod)
        monkeypatch.setitem(sys.modules, "sklearn.feature_extraction.text", text_mod)
        monkeypatch.setitem(sys.modules, "sklearn.cluster", cluster_mod)

        sampler = ActiveSampler()
        sampler._load_model()
        assert sampler._use_sklearn is True

    def test_load_model_missing_sklearn_falls_back(self, monkeypatch):
        """无 sklearn 时保持 _use_sklearn=False 且可继续分析"""
        from augmentor.sampler import ActiveSampler

        for mod in (
            "sklearn",
            "sklearn.feature_extraction",
            "sklearn.feature_extraction.text",
            "sklearn.cluster",
        ):
            monkeypatch.setitem(sys.modules, mod, None)  # 触发 ImportError

        sampler = ActiveSampler()
        sampler._load_model()
        assert sampler._use_sklearn is False
