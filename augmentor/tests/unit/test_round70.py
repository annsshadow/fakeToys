# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第70轮: feature_detect特征检测"""
import pytest
from augmentor.feature_detect import FeatureDetector


class TestFeatureDetector:
    def test_create(self):
        f = FeatureDetector()
        assert f is not None

    def test_has_core_methods(self):
        f = FeatureDetector()
        assert hasattr(f, 'detect')
        assert hasattr(f, 'detect_field_features')
        assert hasattr(f, 'detect_intent_distribution')
        assert hasattr(f, 'build_feature_matrix')

    def test_detect_basic(self):
        f = FeatureDetector()
        items = [{"instruction": "test question", "output": "test answer"}]
        result = f.detect(items)
        assert result is not None
