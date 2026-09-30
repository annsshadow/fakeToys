# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""构造对象不碰盘：断点/追踪/可视化三族的输出目录改到首次写入时才创建

立案理由（B5 死键分型顺路撞出，与 A168 残留守卫的失明同一形状）：这三族的构造函数各自
`mkdir`，而 `AugmentorPipeline.__init__` 会把它们全部建一遍 —— 于是「只是拿一个管道对象」
就在当前工作目录留下 `checkpoints/`、`experiments/`、`visualizations/` 三个目录，哪怕用户
一次断点都没存、一张图都没出。空目录不进版本库（git 不跟踪空目录），所以它们既不出现在
`git status` 里，又把 A168 残留守卫的「前后差集」抵冲成空 —— 既存目录让守卫看不见自己。
"""

from augmentor.checkpoint import CheckpointManager
from augmentor.tracker import ExperimentTracker
from augmentor.visualizer import DataVisualizer


class TestNoDirectoryOnConstruction:
    """构造函数不该在盘上留下任何东西"""

    def test_checkpoint_manager_creates_nothing(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        CheckpointManager(checkpoint_dir="checkpoints")
        assert not (tmp_path / "checkpoints").exists()

    def test_tracker_creates_nothing(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        ExperimentTracker()
        assert not (tmp_path / "experiments").exists()

    def test_visualizer_creates_nothing(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        DataVisualizer()
        assert not (tmp_path / "visualizations").exists()


class TestDirectoryAppearsOnFirstWrite:
    """真正写东西时目录照样要在 —— 推迟建目录不能把功能写成坏"""

    def test_checkpoint_dir_created_by_first_save(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        manager = CheckpointManager(checkpoint_dir="checkpoints")
        manager.create_checkpoint("task-a", total_items=3)
        assert (tmp_path / "checkpoints" / "task-a_checkpoint.json").exists()

    def test_delta_dir_created_by_first_append(self, tmp_path, monkeypatch):
        """增量走追加而不是原子写：那条路不经 `atomic_write` 的顺手建目录，得自己保证"""
        monkeypatch.chdir(tmp_path)
        manager = CheckpointManager(checkpoint_dir="cp", auto_save_interval=1)
        manager.create_checkpoint("task-b", total_items=2)
        manager.update_progress(0, True, quality_score=0.8)
        assert (tmp_path / "cp" / "task-b_delta.jsonl").exists()

    def test_experiment_dir_created_by_first_save(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        tracker = ExperimentTracker()
        tracker.start_experiment("exp-1", dataset_version="v1", model_name="m")
        assert (tmp_path / "experiments" / "exp-1.json").exists()

    def test_output_path_helper_makes_the_dir(self, tmp_path, monkeypatch):
        """不依赖 matplotlib：判据是「算出图路径」这一步就把目录建出来"""
        monkeypatch.chdir(tmp_path)
        visualizer = DataVisualizer()
        path = visualizer._output_path("wordcloud.png")
        assert path.parts == ("visualizations", "wordcloud.png")
        assert (tmp_path / "visualizations").is_dir()


class TestExplicitLocationNotRewritten:
    """显式传参的落点不被默认值改写：这里去掉的只是「建目录」这一步"""

    def test_relative_explicit_dir_is_used(self, tmp_path, monkeypatch):
        monkeypatch.chdir(tmp_path)
        tracker = ExperimentTracker(storage_dir="nested/deep")
        tracker.start_experiment("exp-x", dataset_version="v", model_name="m")
        assert (tmp_path / "nested" / "deep" / "exp-x.json").exists()

    def test_absolute_dir_is_used_verbatim(self, tmp_path):
        given = tmp_path / "given"
        tracker = ExperimentTracker(storage_dir=str(given))
        tracker.start_experiment("exp-y", dataset_version="v", model_name="m")
        assert (given / "exp-y.json").exists()


def test_pipeline_init_leaves_none_of_the_three_dirs(tmp_path, monkeypatch):
    """管道初始化是这三族的共同入口，判据要落在它身上"""
    from augmentor.config import AppConfig
    from augmentor.pipeline import AugmentorPipeline

    monkeypatch.chdir(tmp_path)
    AugmentorPipeline(AppConfig())
    for name in ("checkpoints", "experiments", "visualizations"):
        assert not (tmp_path / name).exists(), f"管道初始化仍然顺手建了 {name}/"


def test_checkpoint_relative_dir_still_anchors_at_construction(tmp_path, monkeypatch):
    """A 系列的既有口径不能被动摇：相对路径在构造这一刻定基，之后 chdir 不改落点

    本批把「构造时建目录」改成「写时建目录」，那一步不能顺手把定基也搬晚 ——
    否则同一个任务的两半进度会分家到两个目录。
    """
    first = tmp_path / "first"
    first.mkdir()
    monkeypatch.chdir(first)
    manager = CheckpointManager(checkpoint_dir="cp")
    monkeypatch.chdir(tmp_path)
    manager.create_checkpoint("task-c", total_items=1)
    assert (first / "cp" / "task-c_checkpoint.json").exists()
    assert not (tmp_path / "cp").exists()
