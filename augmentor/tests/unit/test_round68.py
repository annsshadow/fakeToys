# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第68轮: dependency依赖管理"""
from augmentor.dependency import DependencyManager


class TestDependencyManager:
    """注册表目录的落点

    `DependencyManager()` 在**构造时**就对 `registry_path` 执行 `mkdir`，默认值
    `.dependency_registry` 是相对路径。早先这两条冒烟用例直接跑在仓库工作目录里，
    于是每轮全量测试都在版本树中留下一处注册表目录。这里显式切到临时目录，并把
    「默认值跟着工作目录走」这件事本身断言下来——API 侧的默认值刻意不这么做，
    见 `api.deps.default_registry_dir`。
    """

    def test_default_registry_lands_in_working_directory(self, tmp_path, monkeypatch):
        """缺省构造：目录建在工作目录下，且建出来的是空注册表"""
        monkeypatch.chdir(tmp_path)

        manager = DependencyManager()

        assert (tmp_path / ".dependency_registry").is_dir()
        assert manager.list_datasets() == []

    def test_explicit_registry_path_is_used_verbatim(self, tmp_path):
        """显式路径不受工作目录影响"""
        registry = tmp_path / "somewhere" / "else"

        DependencyManager(str(registry))

        assert registry.is_dir()

    def test_dependency_graph_shape(self, tmp_path):
        """空注册表也要返回完整的图结构，客户端不必防 KeyError"""
        graph = DependencyManager(str(tmp_path / "registry")).get_dependency_graph()

        assert {"nodes", "edges"} <= set(graph)
