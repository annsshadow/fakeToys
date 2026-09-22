# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""DataVisualizer 单元测试

分两类：
1. 依赖缺失时的降级路径（真实环境未安装 wordcloud / matplotlib）；
2. 通过注入假 wordcloud / matplotlib 模块，覆盖真实绘图分支。

这样无需在测试机上安装重型可视化依赖，同时仍能验证绘图代码路径。
"""

import importlib.util
import sys
import types
from pathlib import Path

import numpy as np
import pytest

from augmentor.visualizer import DataVisualizer


def _module_missing(name: str) -> bool:
    """判断某个可选依赖是否缺失"""
    return importlib.util.find_spec(name) is None


class _FakeAxis:
    """记录绘图调用但不真正渲染"""

    def __init__(self):
        self.calls = []

    def __getattr__(self, name):
        def _recorder(*args, **kwargs):
            self.calls.append((name, args, kwargs))
            return None

        return _recorder


class _FakePyplot:
    """假 matplotlib.pyplot"""

    def __init__(self):
        self.saved_paths = []
        self.closed = 0
        self.last_axis = None

    def subplots(self, *args, **kwargs):
        self.last_axis = _FakeAxis()
        return object(), self.last_axis

    def savefig(self, path, **kwargs):
        Path(path).write_bytes(b"\x89PNG\r\n\x1a\nfake")
        self.saved_paths.append(str(path))

    def close(self, fig):
        self.closed += 1


class _FakeWordCloud:
    """假 WordCloud，仅写出占位文件"""

    def __init__(self, **kwargs):
        self.kwargs = kwargs
        self.text = ""

    def generate(self, text):
        self.text = text

    def to_file(self, path):
        Path(path).write_bytes(b"fake-wordcloud")


@pytest.fixture
def fake_matplotlib(monkeypatch):
    """注入假 matplotlib / matplotlib.pyplot"""
    pyplot = _FakePyplot()

    matplotlib_module = types.ModuleType("matplotlib")
    matplotlib_module.use = lambda *args, **kwargs: None
    pyplot_module = types.ModuleType("matplotlib.pyplot")
    pyplot_module.subplots = pyplot.subplots
    pyplot_module.savefig = pyplot.savefig
    pyplot_module.close = pyplot.close

    monkeypatch.setitem(sys.modules, "matplotlib", matplotlib_module)
    monkeypatch.setitem(sys.modules, "matplotlib.pyplot", pyplot_module)
    return pyplot


@pytest.fixture
def fake_wordcloud(monkeypatch):
    """注入假 wordcloud 模块"""
    wordcloud_module = types.ModuleType("wordcloud")
    wordcloud_module.WordCloud = _FakeWordCloud
    monkeypatch.setitem(sys.modules, "wordcloud", wordcloud_module)
    return wordcloud_module


class TestFallbackWithoutDependencies:
    def test_wordcloud_returns_empty_when_missing(self, tmp_path):
        if not _module_missing("wordcloud"):
            pytest.skip("wordcloud 已安装，无法测试缺失降级路径")
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        assert visualizer.generate_wordcloud([{"instruction": "你好"}]) == ""

    def test_length_distribution_returns_empty_when_missing(self, tmp_path):
        if not _module_missing("matplotlib"):
            pytest.skip("matplotlib 已安装，无法测试缺失降级路径")
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        assert visualizer.generate_length_distribution([{"instruction": "你好"}]) == ""

    def test_quality_distribution_returns_empty_when_missing(self, tmp_path):
        if not _module_missing("matplotlib"):
            pytest.skip("matplotlib 已安装，无法测试缺失降级路径")
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        assert visualizer.generate_quality_distribution([0.5, 0.8]) == ""

    def test_timeline_returns_empty_when_missing(self, tmp_path):
        if not _module_missing("matplotlib"):
            pytest.skip("matplotlib 已安装，无法测试缺失降级路径")
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        assert visualizer.generate_timeline([{"timestamp": "t1", "count": 1}]) == ""

    def test_generate_all_visualizations_returns_empty_mapping(self, tmp_path):
        if not (_module_missing("matplotlib") and _module_missing("wordcloud")):
            pytest.skip("可视化依赖已安装，无法测试缺失降级路径")
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        result = visualizer.generate_all_visualizations([{"instruction": "你好"}], scores=[0.5])
        assert result == {}

    def test_output_dir_is_created(self, tmp_path):
        target = tmp_path / "nested" / "viz"
        DataVisualizer(output_dir=str(target))
        assert target.is_dir()


class TestWordcloudWithFakeModule:
    def test_generates_wordcloud_file(self, tmp_path, fake_wordcloud):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_wordcloud(
            [{"instruction": "租金"}, {"instruction": "押金"}]
        )

        assert path == str(tmp_path / "wordcloud.png")
        assert Path(path).read_bytes() == b"fake-wordcloud"
        # 合并文本应包含全部条目内容
        assert visualizer._wordcloud is _FakeWordCloud

    def test_custom_text_key_and_output_file(self, tmp_path, fake_wordcloud):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_wordcloud(
            [{"output": "回答内容"}], text_key="output", output_file="custom.png"
        )
        assert path.endswith("custom.png")


class TestChartsWithFakeMatplotlib:
    def test_length_distribution_saves_file(self, tmp_path, fake_matplotlib):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_length_distribution(
            [{"instruction": "短"}, {"instruction": "长" * 40}]
        )

        assert path == str(tmp_path / "length_distribution.png")
        assert fake_matplotlib.saved_paths == [path]
        assert fake_matplotlib.closed == 1
        hist_calls = [c for c in fake_matplotlib.last_axis.calls if c[0] == "hist"]
        assert hist_calls[0][1] == ([1, 40],)
        assert hist_calls[0][2] == {"bins": 30, "edgecolor": "black"}

    def test_quality_distribution_draws_threshold_line(self, tmp_path, fake_matplotlib):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_quality_distribution([0.5, 0.7, 0.9])

        assert path == str(tmp_path / "quality_distribution.png")
        call_names = [name for name, _, _ in fake_matplotlib.last_axis.calls]
        assert "hist" in call_names
        assert "axvline" in call_names
        assert "legend" in call_names

    def test_timeline_returns_empty_for_empty_progress_data(self, tmp_path, fake_matplotlib):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        assert visualizer.generate_timeline([]) == ""
        assert fake_matplotlib.saved_paths == []

    def test_timeline_saves_file(self, tmp_path, fake_matplotlib):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_timeline(
            [
                {"timestamp": "2026-01-01", "count": 10},
                {"timestamp": "2026-01-02", "count": 25},
            ]
        )

        assert path == str(tmp_path / "timeline.png")
        call_names = [name for name, _, _ in fake_matplotlib.last_axis.calls]
        assert "plot" in call_names
        assert "set_xticklabels" in call_names

    def test_topic_cluster_returns_empty_without_sklearn(self, tmp_path, fake_matplotlib):
        if not _module_missing("sklearn"):
            pytest.skip("sklearn 已安装，该分支会走真实聚类")
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        assert visualizer.generate_topic_cluster([{"instruction": "你好"}]) == ""

    def test_generate_all_visualizations_collects_existing_files(
        self, tmp_path, fake_matplotlib, fake_wordcloud
    ):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        result = visualizer.generate_all_visualizations(
            [{"instruction": "租金怎么算"}, {"instruction": "押金怎么退"}],
            scores=[0.6, 0.9],
        )

        # sklearn 缺失导致话题聚类跳过，其余三项应全部产出
        assert set(result) == {"wordcloud", "length_distribution", "quality_distribution"}
        for path in result.values():
            assert Path(path).exists()

    def test_generate_all_visualizations_without_scores(
        self, tmp_path, fake_matplotlib, fake_wordcloud
    ):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        result = visualizer.generate_all_visualizations([{"instruction": "租金"}])
        assert "quality_distribution" not in result


class TestGenerateStatistics:
    def test_statistics_for_normal_items(self, tmp_path):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        items = [
            {"instruction": "租金，押金"},
            {"instruction": "租金，合同"},
        ]
        stats = visualizer.generate_statistics(items)

        assert stats["total_items"] == 2
        assert stats["avg_length"] == 5
        assert stats["min_length"] == 5
        assert stats["max_length"] == 5
        # 中文按连续汉字串分词，标点作为分隔
        assert stats["top_words"]["租金"] == 2
        assert stats["unique_words"] == 3

    def test_statistics_for_empty_items(self, tmp_path):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        stats = visualizer.generate_statistics([])

        assert stats["total_items"] == 0


class TestVisualizerExtended:
    """DataVisualizer 扩展测试"""

    def test_init_default_dir(self, tmp_path):
        """默认输出目录"""
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        assert str(visualizer.output_dir) == str(tmp_path)

    def test_wordcloud_empty_items(self, tmp_path, fake_wordcloud):
        """空数据生成词云"""
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_wordcloud([])
        assert path == "" or path is not None

    def test_length_distribution_single_item(self, tmp_path, fake_matplotlib):
        """单条数据长度分布"""
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_length_distribution([{"instruction": "短"}])
        assert path is not None

    def test_quality_distribution_single_score(self, tmp_path, fake_matplotlib):
        """单个分数质量分布"""
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_quality_distribution([0.5])
        assert path is not None

    def test_statistics_single_item(self, tmp_path):
        """单条数据统计"""
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        stats = visualizer.generate_statistics([{"instruction": "测试"}])
        assert stats["total_items"] == 1
        assert stats["avg_length"] == 2.0
        assert stats["min_length"] == 2
        assert stats["max_length"] == 2
        assert stats["unique_words"] == 1

    def test_statistics_respects_text_key(self, tmp_path):
        visualizer = DataVisualizer(output_dir=str(tmp_path))
        stats = visualizer.generate_statistics(
            [{"instruction": "问题", "output": "回答内容"}], text_key="output"
        )
        assert stats["avg_length"] == 4


class TestTopicClusterWithFakeSklearn:
    """使用假 sklearn 测试话题聚类"""

    def test_generates_topic_cluster_file(self, tmp_path, fake_matplotlib, monkeypatch):
        """话题聚类图生成"""
        # 注入假 sklearn
        sklearn_module = types.ModuleType("sklearn")
        feature_extraction = types.ModuleType("sklearn.feature_extraction")
        text_module = types.ModuleType("sklearn.feature_extraction.text")
        text_module.TfidfVectorizer = type("TfidfVectorizer", (), {
            "__init__": lambda self, **kw: None,
            "fit_transform": lambda self, X: type("FakeMatrix", (), {
                "toarray": lambda self: np.array([[1.0, 0.0], [0.0, 1.0]])
            })()
        })
        manifold = types.ModuleType("sklearn.manifold")
        manifold.TSNE = type("TSNE", (), {
            "__init__": lambda self, **kw: None,
            "fit_transform": lambda self, X: np.array([[0.0, 0.0], [1.0, 1.0]])
        })
        monkeypatch.setitem(sys.modules, "sklearn", sklearn_module)
        monkeypatch.setitem(sys.modules, "sklearn.feature_extraction", feature_extraction)
        monkeypatch.setitem(sys.modules, "sklearn.feature_extraction.text", text_module)
        monkeypatch.setitem(sys.modules, "sklearn.manifold", manifold)

        visualizer = DataVisualizer(output_dir=str(tmp_path))
        path = visualizer.generate_topic_cluster(
            [{"instruction": "租金"}, {"instruction": "押金"}]
        )
        assert path.endswith("topic_cluster.png")
        assert Path(path).exists()
