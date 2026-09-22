# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第89轮: streaming流式处理"""
import pytest
from augmentor.streaming import StreamReader, StreamWriter, StreamProcessor, create_stream_processor


class TestStreaming:
    def test_create_stream_processor(self):
        def process_fn(item):
            return item
        p = create_stream_processor(process_fn)
        assert p is not None

    def test_has_core_classes(self):
        assert StreamReader is not None
        assert StreamWriter is not None
        assert StreamProcessor is not None
