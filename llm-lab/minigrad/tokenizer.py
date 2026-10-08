# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""minigrad.tokenizer —— 两个分词器：字符级 与 极简 BPE。

* ``CharTokenizer``：把每个字符当一个 token。零训练、直观，适合入门与小语料。
* ``BPETokenizer``：字节对编码（Byte-Pair Encoding）的教学实现——反复把语料中
  出现频次最高的相邻 token 对合并成一个新 token。GPT 系列用的就是 BPE 的变体。
"""

from __future__ import annotations

from collections import Counter


class CharTokenizer:
    def __init__(self, text):
        chars = sorted(set(text))
        self.stoi = {c: i for i, c in enumerate(chars)}
        self.itos = {i: c for i, c in enumerate(chars)}

    @property
    def vocab_size(self):
        return len(self.stoi)

    def encode(self, text):
        return [self.stoi[c] for c in text]

    def decode(self, ids):
        return "".join(self.itos[i] for i in ids)


class BPETokenizer:
    """最小可用的 BPE：在字节序列上学习合并规则。

    训练流程（见 ``train``）：
      1. 文本编码为 UTF-8 字节，每个字节是一个初始 token（0..255）；
      2. 统计相邻 token 对的频次，把最高频的一对合并成新 id；
      3. 重复，直到词表达到 ``vocab_size``。
    """

    def __init__(self):
        self.merges = {}        # (a, b) -> 新 id
        self.vocab = {i: bytes([i]) for i in range(256)}

    @staticmethod
    def _pair_counts(ids):
        counts = Counter()
        for a, b in zip(ids, ids[1:]):
            counts[(a, b)] += 1
        return counts

    @staticmethod
    def _merge(ids, pair, new_id):
        out, i = [], 0
        while i < len(ids):
            if i < len(ids) - 1 and ids[i] == pair[0] and ids[i + 1] == pair[1]:
                out.append(new_id)
                i += 2
            else:
                out.append(ids[i])
                i += 1
        return out

    def train(self, text, vocab_size=512):
        assert vocab_size >= 256
        ids = list(text.encode("utf-8"))
        for new_id in range(256, vocab_size):
            counts = self._pair_counts(ids)
            if not counts:
                break
            pair = max(counts, key=counts.get)
            ids = self._merge(ids, pair, new_id)
            self.merges[pair] = new_id
            self.vocab[new_id] = self.vocab[pair[0]] + self.vocab[pair[1]]
        return self

    @property
    def vocab_size(self):
        return len(self.vocab)

    def encode(self, text):
        ids = list(text.encode("utf-8"))
        # 按合并先后顺序反复应用规则
        for pair, new_id in self.merges.items():
            ids = self._merge(ids, pair, new_id)
        return ids

    def decode(self, ids):
        data = b"".join(self.vocab[i] for i in ids)
        return data.decode("utf-8", errors="replace")
