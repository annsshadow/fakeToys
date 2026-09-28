# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""L119（A205）：空/不可解析输入触发的可达假支

逐条对应 L100 终态偏支：
- report.py 115->113        scores 为空 ⇒ 该指标 values 为空 ⇒ 不写进 summary
- json_extract.py 136->143  代码围栏里是不可解析 JSON ⇒ 两次尝试都失败 ⇒ 落到平衡括号扫描
- json_extract.py 150->143  平衡括号截出的片段也不可解析 ⇒ 内层尝试耗尽 ⇒ 回到下一个开括号
"""


class TestReportEmptyScores:
    def test_metric_summary_of_empty_scores_is_empty(self):
        from augmentor.report import ReportGenerator

        # scores 为空 ⇒ 每个指标的 values 都是空列表 ⇒ `if values:` 假支 ⇒ summary 空
        summary = ReportGenerator()._compute_metric_summary([])
        assert summary == {}


class TestJsonExtractUnparseable:
    def test_fenced_invalid_json_falls_through_to_balanced_scan(self):
        from augmentor.json_extract import extract_json

        # 围栏里是彻底不可解析的内容，平衡括号扫描截出的 `{...}` 同样解析失败
        # ⇒ 走完围栏假支(136->143)与平衡假支(150->143)后，返回 ok=False/method=none
        result = extract_json("```json\n{ 这不是 :: 合法 JSON @@@ }\n```")
        assert result.ok is False
        assert result.method == "none"

    def test_valid_fence_still_parses(self):
        from augmentor.json_extract import extract_json

        # 对照：合法围栏内容仍走真支、正常解析
        result = extract_json('```json\n{"instruction": "问", "output": "答"}\n```')
        assert result.ok is True
        assert result.value["instruction"] == "问"
