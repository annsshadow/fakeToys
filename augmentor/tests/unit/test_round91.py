# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第91轮: pipeline流水线"""
import pytest


class TestPipelineIntegration:
    def test_pipeline_import(self):
        from augmentor.pipeline import AugmentorPipeline
        assert AugmentorPipeline is not None

    def test_pipeline_class_exists(self):
        from augmentor.pipeline import AugmentorPipeline
        assert hasattr(AugmentorPipeline, 'augment_dataset')
        assert hasattr(AugmentorPipeline, 'augment_async')
        assert hasattr(AugmentorPipeline, 'export_dataset')
        assert hasattr(AugmentorPipeline, 'visualize_dataset')
