"""ExperimentTracker 单元测试

覆盖实验生命周期、指标记录、对比与报告生成。
"""

import json

from augmentor.tracker import ExperimentResult, ExperimentTracker


def make_tracker(tmp_path) -> ExperimentTracker:
    return ExperimentTracker(storage_dir=str(tmp_path / "experiments"))


class TestExperimentLifecycle:
    def test_start_experiment_creates_file_and_state(self, tmp_path):
        tracker = make_tracker(tmp_path)
        experiment = tracker.start_experiment("exp-1", "v1", "ernie")

        assert experiment.experiment_id == "exp-1"
        assert experiment.dataset_version == "v1"
        assert experiment.model_name == "ernie"
        assert experiment.end_time == ""
        assert experiment.metrics == {}
        assert experiment.metadata == {}
        assert (tmp_path / "experiments" / "exp-1.json").exists()
        assert tracker._current_experiment is experiment

    def test_start_experiment_keeps_metadata(self, tmp_path):
        tracker = make_tracker(tmp_path)
        experiment = tracker.start_experiment(
            "exp-1", "v1", "ernie", metadata={"lr": 0.01}
        )
        assert experiment.metadata == {"lr": 0.01}

    def test_log_metric_appends_values(self, tmp_path):
        tracker = make_tracker(tmp_path)
        tracker.start_experiment("exp-1", "v1", "ernie")
        tracker.log_metric("loss", 0.5)
        tracker.log_metric("loss", 0.3)
        tracker.log_metric("accuracy", 0.8)

        assert tracker._current_experiment.metrics == {
            "loss": [0.5, 0.3],
            "accuracy": [0.8],
        }

    def test_log_metric_persists_to_disk(self, tmp_path):
        tracker = make_tracker(tmp_path)
        tracker.start_experiment("exp-1", "v1", "ernie")
        tracker.log_metric("loss", 0.5)

        with open(tmp_path / "experiments" / "exp-1.json", encoding="utf-8") as f:
            saved = json.load(f)
        assert saved["metrics"] == {"loss": [0.5]}

    def test_log_metric_without_experiment_is_noop(self, tmp_path):
        tracker = make_tracker(tmp_path)
        tracker.log_metric("loss", 0.5)  # 不应抛异常
        assert tracker._current_experiment is None

    def test_end_experiment_records_final_metrics(self, tmp_path):
        tracker = make_tracker(tmp_path)
        tracker.start_experiment("exp-1", "v1", "ernie")
        tracker.log_metric("loss", 0.5)
        tracker.log_metric("loss", 0.3)
        tracker.end_experiment()

        assert tracker._current_experiment is None
        loaded = tracker.load_experiment("exp-1")
        assert loaded.end_time != ""
        assert loaded.final_metrics == {"loss": 0.3}

    def test_end_experiment_without_experiment_is_noop(self, tmp_path):
        tracker = make_tracker(tmp_path)
        tracker.end_experiment()  # 不应抛异常
        assert tracker._current_experiment is None


class TestLoadAndList:
    def test_load_missing_experiment_returns_none(self, tmp_path):
        tracker = make_tracker(tmp_path)
        assert tracker.load_experiment("not-exist") is None

    def test_load_corrupted_experiment_returns_none(self, tmp_path):
        tracker = make_tracker(tmp_path)
        (tmp_path / "experiments" / "broken.json").write_text("{not json", encoding="utf-8")
        assert tracker.load_experiment("broken") is None

    def test_list_experiments_sorted_by_start_time_desc(self, tmp_path):
        tracker = make_tracker(tmp_path)
        first = tracker.start_experiment("exp-old", "v1", "ernie")
        first.start_time = "2020-01-01T00:00:00"
        tracker._save_experiment(first)
        tracker.start_experiment("exp-new", "v2", "gpt")

        experiments = tracker.list_experiments()
        assert [e.experiment_id for e in experiments] == ["exp-new", "exp-old"]

    def test_list_experiments_skips_corrupted_files(self, tmp_path):
        tracker = make_tracker(tmp_path)
        tracker.start_experiment("exp-1", "v1", "ernie")
        (tmp_path / "experiments" / "broken.json").write_text("{not json", encoding="utf-8")

        assert [e.experiment_id for e in tracker.list_experiments()] == ["exp-1"]

    def test_save_experiment_handles_write_failure(self, tmp_path, monkeypatch):
        """保存实验文件失败时不应崩溃"""
        tracker = make_tracker(tmp_path)
        tracker.start_experiment("exp-1", "v1", "ernie")
        
        def fail_write(*args, **kwargs):
            raise OSError("Permission denied")
        
        monkeypatch.setattr("builtins.open", fail_write)
        tracker._save_experiment(tracker._current_experiment)
        # 不应抛异常


class TestCompareExperiments:
    def test_returns_error_when_nothing_found(self, tmp_path):
        tracker = make_tracker(tmp_path)
        assert tracker.compare_experiments(["nope"]) == {"error": "没有找到实验"}

    def test_compares_metrics_across_experiments(self, tmp_path):
        tracker = make_tracker(tmp_path)

        tracker.start_experiment("exp-1", "v1", "ernie")
        tracker.log_metric("loss", 0.5)
        tracker.log_metric("bleu", 0.2)
        tracker.end_experiment()

        tracker.start_experiment("exp-2", "v2", "gpt")
        tracker.log_metric("loss", 0.3)
        tracker.end_experiment()

        result = tracker.compare_experiments(["exp-1", "exp-2"])

        assert result["experiments"] == ["exp-1", "exp-2"]
        assert result["dataset_versions"] == ["v1", "v2"]
        assert result["model_names"] == ["ernie", "gpt"]
        assert result["metric_comparison"]["loss"] == {"exp-1": 0.5, "exp-2": 0.3}
        # exp-2 没有记录 bleu，应显式为 None 而非被丢弃
        assert result["metric_comparison"]["bleu"] == {"exp-1": 0.2, "exp-2": None}

    def test_ignores_missing_experiment_ids(self, tmp_path):
        tracker = make_tracker(tmp_path)
        tracker.start_experiment("exp-1", "v1", "ernie")
        tracker.end_experiment()

        result = tracker.compare_experiments(["exp-1", "missing"])
        assert result["experiments"] == ["exp-1"]

    def test_identical_metrics(self, tmp_path):
        """所有实验指标完全重叠时应正确处理"""
        tracker = make_tracker(tmp_path)
        for i in range(3):
            tracker.start_experiment(f"exp-{i}", "v1", "model")
            tracker.log_metric("loss", 0.5)
            tracker.end_experiment()
        result = tracker.compare_experiments(["exp-0", "exp-1", "exp-2"])
        assert result["metric_comparison"]["loss"] == {
            "exp-0": 0.5, "exp-1": 0.5, "exp-2": 0.5
        }


class TestGenerateReport:
    def test_missing_experiment_reports_error(self, tmp_path):
        tracker = make_tracker(tmp_path)
        assert tracker.generate_report("nope") == {"error": "实验不存在: nope"}

    def test_report_contains_metrics_and_duration(self, tmp_path):
        tracker = make_tracker(tmp_path)
        tracker.start_experiment("exp-1", "v1", "ernie", metadata={"tag": "baseline"})
        tracker.log_metric("loss", 0.5)
        tracker.end_experiment()

        report = tracker.generate_report("exp-1")

        assert report["experiment_id"] == "exp-1"
        assert report["dataset_version"] == "v1"
        assert report["model_name"] == "ernie"
        assert report["metrics_history"] == {"loss": [0.5]}
        assert report["final_metrics"] == {"loss": 0.5}
        assert report["metadata"] == {"tag": "baseline"}
        assert report["duration"].endswith("秒")


class TestCalculateDuration:
    def _experiment(self, start, end):
        return ExperimentResult(
            experiment_id="exp",
            dataset_version="v1",
            model_name="ernie",
            start_time=start,
            end_time=end,
            metrics={},
            final_metrics={},
            metadata={},
        )

    def test_running_experiment(self, tmp_path):
        tracker = make_tracker(tmp_path)
        experiment = self._experiment("2026-01-01T00:00:00", "")
        assert tracker._calculate_duration(experiment) == "进行中"

    def test_seconds_format(self, tmp_path):
        tracker = make_tracker(tmp_path)
        experiment = self._experiment("2026-01-01T00:00:00", "2026-01-01T00:00:30")
        assert tracker._calculate_duration(experiment) == "30.0秒"

    def test_minutes_format(self, tmp_path):
        tracker = make_tracker(tmp_path)
        experiment = self._experiment("2026-01-01T00:00:00", "2026-01-01T00:02:00")
        assert tracker._calculate_duration(experiment) == "2.0分钟"

    def test_hours_format(self, tmp_path):
        tracker = make_tracker(tmp_path)
        experiment = self._experiment("2026-01-01T00:00:00", "2026-01-01T02:00:00")
        assert tracker._calculate_duration(experiment) == "2.0小时"

    def test_invalid_timestamp_returns_unknown(self, tmp_path):
        tracker = make_tracker(tmp_path)
        experiment = self._experiment("not-a-date", "also-bad")
        assert tracker._calculate_duration(experiment) == "未知"
