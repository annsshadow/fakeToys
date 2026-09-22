# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第65轮: rag检索增强"""
import pytest
from augmentor.rag import RAGFormatter


class TestRAGFormatter:
    def test_create(self):
        r = RAGFormatter()
        assert r is not None

    def test_has_format_method(self):
        r = RAGFormatter()
        assert hasattr(r, 'format')
        assert hasattr(r, 'chunk_text')
        assert hasattr(r, 'to_langchain')
        assert hasattr(r, 'to_llamaindex')

    def test_chunk_text(self):
        r = RAGFormatter()
        text = "word " * 100
        chunks = r.chunk_text(text)
        assert isinstance(chunks, list)
        assert len(chunks) > 0

    def test_format_basic(self):
        r = RAGFormatter()
        items = [{"instruction": "q", "output": "a"}]
        result = r.format(items)
        assert result is not None
