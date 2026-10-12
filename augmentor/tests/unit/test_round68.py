# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第68轮: dependency依赖管理"""
from augmentor.dependency import DependencyManager


class TestDependencyManager:
    """注册表目录的落点

    `DependencyManager()` 自 L249（A307）起**构造时不碰磁盘**，目录延后到第一次真写才建，
    默认值 `.dependency_registry` 仍是相对路径。早先这两条冒烟用例直接跑在仓库工作目录里，
    于是每轮全量测试都在版本树中留下一处注册表目录。这里显式切到临时目录，并把
    「落点跟着工作目录走」这件事本身断言下来——API 侧的默认值刻意不这么做，
    见 `api.deps.default_registry_dir`。
    """

    def test_default_registry_lands_in_working_directory(self, tmp_path, monkeypatch):
        """缺省构造不落盘（A307 / L249），真写时才在工作目录下建注册表"""
        monkeypatch.chdir(tmp_path)

        manager = DependencyManager()

        assert not (tmp_path / ".dependency_registry").exists(), "构造把目录建出来了 ⇒ A307 又犯"
        assert manager.list_datasets() == []
        manager.register_dataset("d", str(tmp_path / "p.json"), 1)
        assert (tmp_path / ".dependency_registry" / "datasets.json").is_file()

    def test_explicit_registry_path_is_used_verbatim(self, tmp_path):
        """显式路径不受工作目录影响：构造不碰磁盘，写之后落在那条路径上"""
        registry = tmp_path / "somewhere" / "else"

        manager = DependencyManager(str(registry))
        assert not registry.exists()
        manager.register_dataset("d", str(tmp_path / "p.json"), 1)

        assert registry.is_dir()

    def test_dependency_graph_shape(self, tmp_path):
        """空注册表也要返回完整的图结构，客户端不必防 KeyError"""
        graph = DependencyManager(str(tmp_path / "registry")).get_dependency_graph()

        assert {"nodes", "edges"} <= set(graph)
