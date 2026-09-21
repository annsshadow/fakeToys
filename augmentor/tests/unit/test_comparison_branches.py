"""comparison 胜者判定/摘要/保存分支测试

覆盖 winner=a/b/tie 三分支、摘要生成、save_comparison 落盘与
模块级 compare_datasets。
"""

import json

import pytest

from augmentor.comparison import DatasetComparator, compare_datasets


def _good():
    return [
        {"instruction": f"详细的问题描述{i}？", "input": "", "output": "专业完整的回答内容" * 2}
        for i in range(4)
    ]


def _bad():
    return [
        {"instruction": "短", "input": "", "output": ""}
        for _ in range(4)
    ]


class TestWinnerBranches:
    def test_winner_a_when_a_higher_quality(self):
        comp = DatasetComparator()
        result = comp.compare(_good(), _bad(), name_a="A", name_b="B")
        assert result.winner == "a"

    def test_winner_b_when_b_higher_quality(self):
        comp = DatasetComparator()
        result = comp.compare(_bad(), _good(), name_a="A", name_b="B")
        assert result.winner == "b"

    def test_tie_when_equal(self):
        comp = DatasetComparator()
        data = _good()
        result = comp.compare(data, [dict(x) for x in data])
        assert result.winner == "tie"


class TestComparisonResult:
    def test_result_fields_and_summary(self):
        comp = DatasetComparator()
        result = comp.compare(_good(), _bad())
        assert result.dataset_a_count == 4
        assert result.dataset_b_count == 4
        # quality_diff 的键取自 quality_a（avg_score / pass_rate / min_score / max_score），
        # 并不包含 "quality_a" 本身。原断言写成 `"quality_a" in result.quality_diff or True`，
        # `or True` 让一个永远为假的判断恒真——等于没测。
        assert set(result.quality_diff) == {
            "avg_score", "pass_rate", "min_score", "max_score"
        }
        # 摘要非空且含双方名称
        assert "Dataset A" in result.summary
        assert "Dataset B" in result.summary

    def test_save_comparison(self, tmp_path):
        comp = DatasetComparator()
        result = comp.compare(_good(), _bad())
        out = tmp_path / "cmp.md"
        comp.save_comparison(result, str(out))
        assert out.exists()
        assert out.stat().st_size > 0


class TestModuleConvenience:
    def test_compare_datasets_files(self, tmp_path):
        fa = tmp_path / "a.json"
        fb = tmp_path / "b.json"
        fa.write_text(json.dumps(_good(), ensure_ascii=False), encoding="utf-8")
        fb.write_text(json.dumps(_bad(), ensure_ascii=False), encoding="utf-8")
        result = compare_datasets(str(fa), str(fb), name_a="A", name_b="B")
        assert result.winner == "a"
        assert result.dataset_a_count == 4

    def test_compare_datasets_missing_file(self, tmp_path):
        fa = tmp_path / "a.json"
        fa.write_text(json.dumps(_good(), ensure_ascii=False), encoding="utf-8")
        with pytest.raises((FileNotFoundError, OSError)):
            compare_datasets(str(fa), str(tmp_path / "nope.json"))
