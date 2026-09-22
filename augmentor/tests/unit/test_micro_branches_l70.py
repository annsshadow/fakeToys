# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""微型分支收尾 L70：语言检测重构回归 + profiling 空输入直调

锁定 data/cleaner 与 multilingual 语言检测消除死尾段后的可达词表
（zh / en / mixed / unknown），并覆盖 DataProfiler 私有方法对空
输入的直接调用分支。
"""

from augmentor.data.cleaner import DataCleaner
from augmentor.multilingual import MultilingualSupport
from augmentor.profiling import DataProfiler


class TestLanguageVocabularyAfterRefactor:
    def test_data_cleaner_detect_language(self):
        detector = DataCleaner()
        assert detector.detect_language("你好世界") == "zh"
        assert detector.detect_language("hello world") == "en"
        # 边界：cjk 恰好 0.8 → zh；混合 0.4/0.6 → mixed
        assert detector.detect_language("中中中中 英英英英") == "zh"
        assert detector.detect_language("你好ab") == "mixed"
        assert detector.detect_language("12345") == "unknown"
        assert detector.detect_language("") == "unknown"
        assert detector.detect_language(None) == "unknown"

    def test_multilingual_detect_language(self):
        support = MultilingualSupport()
        assert support.detect_language("你好世界") == "zh"
        assert support.detect_language("hello world") == "en"
        assert support.detect_language("你好ab") == "mixed"
        assert support.detect_language("12345") == "unknown"
        assert support.detect_language("") == "unknown"
        assert support.detect_language(None) == "unknown"


class TestProfilerEmptyInputs:
    def test_length_stats_empty(self):
        profiler = DataProfiler()
        assert profiler._length_stats([]) == {
            "min": 0, "max": 0, "avg": 0.0, "median": 0.0
        }

    def test_duplicate_rate_empty(self):
        profiler = DataProfiler()
        assert profiler._duplicate_rate([]) == 0.0
