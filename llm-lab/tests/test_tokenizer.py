# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""分词器往返与 BPE 合并行为测试。"""

from minigrad.tokenizer import BPETokenizer, CharTokenizer


def test_char_roundtrip():
    text = "hello, 世界! 123\n"
    tok = CharTokenizer(text)
    assert tok.decode(tok.encode(text)) == text
    assert tok.vocab_size == len(set(text))


def test_bpe_roundtrip_ascii_and_unicode():
    tok = BPETokenizer().train("the cat sat on the mat. the cat ran.", vocab_size=300)
    for s in ["the cat", "世界你好", "a mat"]:
        assert tok.decode(tok.encode(s)) == s


def test_bpe_merges_shrink_sequence():
    """高频子串被合并后，编码长度应短于纯字节长度。"""
    text = "ababababab ababab abab" * 5
    tok = BPETokenizer().train(text, vocab_size=270)
    assert tok.merges, "应至少学到一条合并规则"
    assert len(tok.encode(text)) < len(text.encode("utf-8"))
