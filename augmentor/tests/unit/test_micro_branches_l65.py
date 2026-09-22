# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""微型分支收尾 L65：visualizer/visualize_enhanced/export_enhanced 残留分支

- visualizer: 话题聚类结果写入 results + output_dir 方法
- visualize_enhanced: 500+ 长度桶、json 报告落盘
- export_enhanced: 不支持格式报错、空数据集 csv/tsv 提前返回
"""

from pathlib import Path

import pytest

from augmentor.export_enhanced import (
    EnhancedExporter,
    ExportFormat,
    ExportOptions,
)
from augmentor.visualizer import DataVisualizer
from augmentor.visualize_enhanced import visualize_dataset


class TestVisualizerAll:
    def test_topic_cluster_result_included(self, monkeypatch):
        visualizer = DataVisualizer()
        monkeypatch.setattr(
            visualizer, "generate_topic_cluster",
            lambda items, text_key: "visualizations/topic.png",
        )
        results = visualizer.generate_all_visualizations(
            [{"instruction": "如何申请？"}]
        )
        assert results.get("topic_cluster") == "visualizations/topic.png"

    def test_output_dir_attribute(self, tmp_path, monkeypatch):
        # 实例属性 output_dir 是 Path（构造函数中创建目录）
        monkeypatch.chdir(tmp_path)
        viz = DataVisualizer(output_dir=str(tmp_path / "viz"))
        assert viz.output_dir == tmp_path / "viz"
        assert (tmp_path / "viz").is_dir()


class TestVisualizeEnhanced:
    def test_length_bucket_500_plus(self):
        report = visualize_dataset([{"instruction": "长" * 600}])
        assert "500+" in report

    def test_json_report_saved_to_file(self, tmp_path):
        out = tmp_path / "report.json"
        report = visualize_dataset(
            [{"instruction": "q", "output": "a"}],
            output_path=str(out),
            format="json",
        )
        assert out.exists()
        # json 报告是聚合统计，不含原始文本
        assert '"total_items": 1' in report


class TestExportEnhancedBranches:
    def test_unsupported_format_raises(self, tmp_path):
        options = ExportOptions(format="bogus_format")  # 绕过类型约束构造非法值
        with pytest.raises(ValueError, match="不支持的格式"):
            EnhancedExporter().export(
                [{"a": 1}], str(tmp_path / "o.json"), options
            )

    def test_empty_items_csv_early_return(self, tmp_path):
        result = EnhancedExporter().export(
            [], str(tmp_path / "a.csv"), ExportOptions(format=ExportFormat.CSV)
        )
        assert result["item_count"] == 0
        assert not (tmp_path / "a.csv").exists()

    def test_empty_items_tsv_early_return(self, tmp_path):
        result = EnhancedExporter().export(
            [], str(tmp_path / "b.tsv"), ExportOptions(format=ExportFormat.TSV)
        )
        assert result["item_count"] == 0
        assert not (tmp_path / "b.tsv").exists()
