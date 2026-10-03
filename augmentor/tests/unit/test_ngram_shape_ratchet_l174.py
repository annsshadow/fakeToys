# -*- coding: utf-8 -*-
"""L174 / B244：ngram 热路径的**相对**性能形状棘轮。

`_search_ngram` 的子串判定实现（查询侧 k 个 gram × 文档子串判定，O(文档数) 次数）
被 L 轮性能工作替换掉「文档侧物化整串 gram 集合」的旧实现后提速约 6 倍
（本仓 docstring 里的实测：单字段 22.2 → 3.2 ms）。本守卫不钉绝对毫秒
（A157 的教训：绝对墙钟预算是环境性假红），而是钉**形状**——同一进程内
A/B 交替量当前实现与朴素物化实现的相对耗时比：形状完好时比值稳定在
0.15–0.3，谁把实现换回朴素形状（比值 → ≈1）或换出更慢的形状（比值 ↑），
当场红。
"""
import time

from augmentor.search_enhanced import EnhancedSearcher

_CORPUS = [
    {"instruction": f"问题编号 {i}: 请解释一下机器学习里的第 {i} 个概念",
     "output": f"答案编号 {i}: 这是一个关于概念 {i} 的解释性段落"}
    for i in range(2000)
]
_QUERY = "概念"

# 朴素形状（被换掉的那个实现）：每条文档物化整串 gram 集合，O(文档长度×gram)
# 个字符串对象。留着它就是为了量「形状」，不是留着它跑生产。
def _naive_gram_scores(items, field, query, n=2):
    lowered = query.lower()
    q_grams = {lowered[i:i + n] for i in range(len(lowered) - n + 1)}
    total = len(q_grams)
    out = {}
    for idx, item in enumerate(items):
        value = item.get(field, "")
        if not isinstance(value, str):
            continue
        v = value.lower()
        doc_grams = {v[i:i + n] for i in range(len(v) - n + 1)}
        hit = sum(1 for g in q_grams if g in doc_grams)
        if hit:
            out[idx] = hit / total
    return out


def _best_of(fn, rounds=3):
    best = float("inf")
    for _ in range(rounds):
        t0 = time.perf_counter()
        fn()
        best = min(best, time.perf_counter() - t0)
    return best


def test_current_shape_is_faster_than_the_materalized_shape():
    """形状棘轮：当前实现必须是朴素物化实现的相当快档（≤ 0.7 倍）。

    系数留了环境余量（实测约 0.15–0.3）——机器再慢也保持「同机 A/B 相对比」
    的口径，不受绝对耗时影响（A157 的教训）。比值爬到 ≈1 意味着有人把实现
    换回了朴素形状（或更糟），本条当场红。
    """
    searcher = EnhancedSearcher(_CORPUS)
    t_current = _best_of(lambda: searcher._search_ngram("instruction", _QUERY, 2))
    t_naive = _best_of(lambda: _naive_gram_scores(_CORPUS, "instruction", _QUERY, 2))
    ratio = t_current / t_naive
    assert ratio <= 0.7, (
        f"ngram 热路径形状回退：当前实现是朴素物化实现的 {ratio:.2f} 倍耗时"
        "（健康档 ≤ 0.7）。若你把 `_search_ngram` 改回了「每条文档物化整串 gram"
        " 集合」的形状，本条会红——打分规模必须跟着**查询**走，不跟着文档走。"
    )


def test_scores_are_unchanged_by_the_shape_guard_corpus():
    """同分护栏：本守卫的 corpus 上两形状逐条同分（与既有全量同分用例不重复——
    这里只用小语料，保证棘轮跑得快，形状一旦回退也不会让全量门禁多付账）。"""
    searcher = EnhancedSearcher(_CORPUS)
    assert searcher._search_ngram("instruction", _QUERY, 2) == _naive_gram_scores(
        _CORPUS, "instruction", _QUERY, 2
    )
