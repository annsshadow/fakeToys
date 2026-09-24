# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""智能去重模块 - 优化版"""

import logging
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
import numpy as np
from .model_manager import model_manager
from .validation import require_count
from .exceptions import DedupError

logger = logging.getLogger(__name__)

# 单个相似度块的单元数上限（float32）。用于把峰值内存钉在常量级：
# _MAX_BLOCK_CELLS × 4B = 16MB。无论数据集多大，任一时刻只持有这么多。
# 这是去重不再随 n 平方增长内存的关键，改动前请先读 _row_block_size 的说明。
_MAX_BLOCK_CELLS = 4_000_000

# 一次「稀疏散射累加」相当于多少次「稠密乘加」。取的是实测区间的**下沿**：
# 真实 6902 条数据上稠密 BLAS 跑到 86 G MAC/s、倒排累加 29.5 M add/s，比值
# 2900；同一台机器换小词表能到 250 G MAC/s，比值随之升到 8600。往下沿取
# 意味着只在倒排**明显**更省时才换路——判错方向的代价是不对称的：稠密那条
# 永远是安全的老行为，倒排那条在退化语料上会慢两个数量级（见
# _inverted_blocks_win 的说明）。它只影响选哪条路，不影响任何数值。
_INVERTED_ADD_COST = 3000

# 一次「pair 级稀疏散射累加」在峰值内存上的实测单价（字节）。倒排那条不物化
# n×vocab 矩阵，但每个块要把 pair 级临时数组（行展开、列展开、权重、bincount
# 的 float64 输出）摊在峰值上，所以它的峰值正比于**最大那个块的累加次数**
# ≈ `pair_adds × block_rows / doc_count`，而不是正比于矩阵。同一台机器、随机
# 60 字符语料（词表恒为 3844）逐档量到的峰值/该估算：n=1200 → 54.2/608k=93.5 B、
# 2400 → 93.5、3600 → 94.5、4800 → 95.0、6000 → 96.9、7200 → 98.8，真实 6902 条
# （词表 23033）→ 85.2 B。取上沿 96 是为了**高估**倒排的开销、让判据偏保守。
# 这条线不能省：实测同一批语料上「稠密矩阵是否超过某个固定 MiB 数」是个坏代理——
# n=4800/V=3844 时矩阵 70.4 MiB 已越过任何 64 MiB 量级的固定线，而实测峰值
# 倒排 **171.3 MiB 反而贵于**稠密的 85.7 MiB（且慢 7%）。
_INVERTED_PAIR_BYTES = 96

# 内存账的安全裕度（百分数）：单价是拟合值，块间分布也不是完全均匀，所以要求
# 稠密那份**明显**更贵才换路。取 125%：真实 6902 条上 606 MiB vs 估算 176 MiB
# ×1.25 = 220 MiB，判赢的余量还有 2.8 倍；而上述 n=4800 档差得更远。
_INVERTED_MEM_MARGIN = 125



@dataclass(frozen=True)
class _InvertedNgrams:
    """字符二元组的倒排视图：同词条连续、值已按整篇 L2 范数归一化

    这是 :class:`Deduplicator` 的内部结构，只服务一件事——不物化 n×vocab
    稠密矩阵也能算出相似度块。字段全是一次性构造好的数组：

    - ``terms``/``docs``/``values``：形状 (nnz,) 的三元组，按 ``terms`` 升序
      连续排列（``docs`` 由 ``(文档, 词条)`` 唯一键展开而来，因此每篇至多
      出现一次），``values`` 是「该词条在该篇的出现次数 ÷ 该篇全部词条的
      L2 范数」。
    - ``vocab_size``：词条数，即稠密实现里那个矩阵的列数。
    - ``doc_count``：文档数。
    - ``pair_adds``：``Σ df(df+1)/2``，即倒排方案必须做的散射累加次数
      ——选路判据的代价预言机。
    """
    terms: np.ndarray
    docs: np.ndarray
    values: np.ndarray
    vocab_size: int
    doc_count: int
    pair_adds: int


@dataclass
class DedupResult:
    """去重结果"""
    original_count: int  # 原始数量
    deduplicated_count: int  # 去重后数量
    removed_count: int  # 移除数量
    duplicate_groups: List[List[int]]  # 重复组（索引列表）
    kept_indices: List[int]  # 保留的索引


class Deduplicator:
    """智能去重器 - 优化版"""
    
    def __init__(self, threshold: float = 0.9, use_faiss: bool = False):
        """初始化去重器
        
        Args:
            threshold: 相似度阈值，高于此值被视为重复
            use_faiss: 是否使用 FAISS 加速（需要安装 faiss-cpu）
        """
        if threshold < 0 or threshold > 1:
            raise DedupError("阈值必须在 0-1 之间")
        
        self.threshold = threshold
        self.use_faiss = use_faiss
        self._model = None
        self._faiss_index = None
    
    def _load_model(self):
        """延迟加载模型（使用共享实例）"""
        if self._model is None:
            self._model = model_manager.get_sentence_model()
    
    def _ngram_similarity(self, text1: str, text2: str, n: int = 2) -> float:
        """基于 n-gram 的相似度计算（fallback 方法）
        
        Args:
            text1: 文本 1
            text2: 文本 2
            n: n-gram 大小
        
        Returns:
            相似度分数 (0-1)
        """
        def get_ngrams(text: str) -> set:
            return set(text[i:i+n] for i in range(len(text) - n + 1))
        
        ngrams1 = get_ngrams(text1)
        ngrams2 = get_ngrams(text2)
        
        if not ngrams1 or not ngrams2:
            return 0.0
        
        intersection = len(ngrams1 & ngrams2)
        union = len(ngrams1 | ngrams2)
        
        return intersection / union if union > 0 else 0.0
    
    def _batch_encode(self, texts: List[str]) -> np.ndarray:
        """批量编码文本
        
        Args:
            texts: 文本列表
        
        Returns:
            embeddings 矩阵
        """
        if self._model == "fallback":
            # fallback: 使用 n-gram 向量化
            return self._fallback_encode(texts)
        
        return self._model.encode(texts, show_progress_bar=False, batch_size=64)
    
    def _fallback_encode(self, texts: List[str]) -> np.ndarray:
        """fallback 编码方法
        
        Args:
            texts: 文本列表
        
        Returns:
            向量矩阵
        """
        # 使用字符 n-gram 的 TF 向量
        vocab = {}
        for text in texts:
            for i in range(len(text) - 1):
                ngram = text[i:i+2]
                if ngram not in vocab:
                    vocab[ngram] = len(vocab)

        # dtype 必须是 float32：这是**稠密** n×vocab 矩阵，也是这条路径上最大的
        # 一笔分配（真实 6902 条数据的 vocab 有 23033 项 → float64 要 1.27 GB，
        # float32 只要 0.64 GB）。调用方写的 `np.asarray(..., dtype=np.float32)`
        # 在 dtype 不符时是整份再复制一遍，所以 float64 还额外多出 0.64 GB。
        # TF 值是「某二元组在该文档里出现几次」，个位数量级，float32 在 2^24
        # 以内精确表示整数，计数不会有任何损失。
        vectors = np.zeros((len(texts), len(vocab)), dtype=np.float32)
        for i, text in enumerate(texts):
            for j in range(len(text) - 1):
                ngram = text[j:j+2]
                if ngram in vocab:
                    vectors[i, vocab[ngram]] += 1

        # L2 归一化，全程原地：`np.linalg.norm` 内部要先算平方和并**另起一份
        # 同尺寸临时矩阵**，`vectors / norms` 又是一份——那两步在 0.64 GB 量级
        # 的矩阵上等于凭空多养两份。einsum 直接把行平方和压成一个向量。
        norms = self._row_norms(vectors)
        norms[norms == 0] = 1
        np.divide(vectors, norms[:, None], out=vectors)

        return vectors

    def _ngram_entries(self, texts: List[str]) -> Tuple[np.ndarray, np.ndarray, np.ndarray, int]:
        """一遍扫描字符二元组，产出稀疏 TF 三元组 (文档号, 词条号, 出现次数)

        词表定义与 :meth:`_fallback_encode` 逐字相同（`text[j:j+2]`，长度 < 2
        的文本不产词条；同一篇里重复的二元组计多次），所以两条路算出的向量
        可以逐元素对照。词条按首次出现顺序编号，而**编号本身不参与语义**——
        相似度是点积，对维度的任意排列不变。

        Returns:
            (docs, terms, counts, vocab_size)，三者等长、按 (文档, 词条) 升序
        """
        vocab: Dict[str, int] = {}
        docs: List[int] = []
        terms: List[int] = []
        for i, text in enumerate(texts):
            for j in range(len(text) - 1):
                ngram = text[j:j + 2]
                column = vocab.get(ngram)
                if column is None:
                    column = vocab[ngram] = len(vocab)
                docs.append(i)
                terms.append(column)

        vocab_size = len(vocab)
        if vocab_size == 0:
            # 全部文本短于 2 个字符：没有任何词条，稠密矩阵会是 n×0
            empty = np.zeros(0, dtype=np.int64)
            return empty, empty.copy(), np.zeros(0, dtype=np.float32), 0

        doc_arr = np.asarray(docs, dtype=np.int64)
        term_arr = np.asarray(terms, dtype=np.int64)
        # 「某词条在某篇出现几次」= 按 (文档, 词条) 复合键数 occurrences。
        # 复合键用 doc × vocab_size + term 编码，真实 6902 条数据 nnz 只有
        # 19.4 万，一次 np.unique 就够。对照：:meth:`_fallback_encode` 用两遍
        # Python 循环、第二遍逐元素 `vectors[i, c] += 1`，同一份数据实测 330 ms，
        # 而这一遍扫描连同后面的归一化与排序只要 73 ms。
        uniq, inverse = np.unique(doc_arr * vocab_size + term_arr, return_inverse=True)
        counts = np.bincount(inverse).astype(np.float32)
        return uniq // vocab_size, uniq % vocab_size, counts, vocab_size

    def _inverted_view(self, texts: List[str]) -> _InvertedNgrams:
        """把 :meth:`_ngram_entries` 折成倒排视图，值已按整篇 L2 范数归一化

        范数在**全部**词条上求，包括只在该篇出现一次的——「只出现在一篇里的
        词条对非对角没有贡献」不等于「可以不算进范数」，稠密实现把它算进去了。
        这是倒排块与稠密块逐元素相等的前提。
        """
        doc_count = len(texts)
        docs, terms, counts, vocab_size = self._ngram_entries(texts)

        # 行范数：bincount 的平方和走 float64 累加再开方，最后降回 float32——
        # 与稠密路径的 einsum(float32) 差在最后一位，量级 1e-7。
        squares = np.bincount(docs, weights=counts.astype(np.float64) ** 2,
                              minlength=doc_count)
        norms = np.sqrt(squares).astype(np.float32)
        norms[norms == 0] = 1
        values = counts / norms[docs]

        # searchsorted 取同词条的一段 postings 要求按词条有序；稳定排序让同词条
        # 内部保持 (文档, 词条) 升序，块内累加的下标顺序因此可复现。
        order = np.argsort(terms, kind="stable")
        df = np.bincount(terms, minlength=vocab_size)
        return _InvertedNgrams(
            terms=terms[order],
            docs=docs[order],
            values=values[order],
            vocab_size=vocab_size,
            doc_count=doc_count,
            pair_adds=int((df * (df + 1) // 2).sum()),
        )

    @staticmethod
    def _inverted_blocks_win(doc_count: int, vocab_size: int, pair_adds: int,
                             block_rows: int) -> bool:
        """选相似度块的生产者：倒排累加还是稠密 BLAS

        两条独立的账，都要倒向倒排才换路：

        1. **时间**——稠密那条要做 `(n²/2) × vocab` 次乘加，倒排那条要做
           `pair_adds` 次散射累加，一次散射累加按实测下折算
           :data:`_INVERTED_ADD_COST` 次乘加。
        2. **内存**——倒排把「物化 n×vocab 矩阵」换成「每个块的 pair 级临时数组」，
           两边分别按 `n×vocab×4B` 与 :data:`_INVERTED_PAIR_BYTES` × 最大块的累加
           次数估算，再留 :data:`_INVERTED_MEM_MARGIN`% 的裕度。

        判据不看编码器就不成立（语义模型的向量处处非零，倒排的累加次数会等于
        稠密乘加数、而单次实测贵约三个数量级），只看其中一条也不成立。
        实测同样 6902 条、同一台机器：
        真实语料（vocab 23033，pair_adds 2191 万）倒排 969 ms / 149.1 MiB
        vs 稠密 10484 ms / 621.8 MiB（min-of-3，逐组结果完全相同）；
        完全重复的语料（vocab 9，pair_adds 2.14 亿）倒排 **6.23 s vs 稠密 0.09 s**；
        随机 60 字符（vocab 恒 3844）从 n=1200 到 7200 逐档量下来，倒排峰值
        **一直是稠密的 1.4～2.4 倍**（54.2 vs 22.2、171.3 vs 85.7、175.9 vs 120.9 MiB），
        n≥6000 才在时间上反过来自身省——这些档两条账都判负。

        Args:
            doc_count: 文档数
            vocab_size: 词条数，即稠密实现里那个矩阵的列数
            pair_adds: 倒排方案必须做的散射累加次数
            block_rows: 实际要用的行块大小——内存账按「最大那个块」估

        Returns:
            True 表示走倒排
        """
        dense_madds = doc_count * doc_count // 2 * vocab_size
        if pair_adds * _INVERTED_ADD_COST >= dense_madds:
            return False
        # 走到这里说明 dense_madds > pair_adds×单价 ≥ 0，即 doc_count ≥ 2、可除。
        dense_bytes = doc_count * vocab_size * 4
        inverted_bytes = pair_adds * block_rows * _INVERTED_PAIR_BYTES // doc_count
        return dense_bytes * 100 > inverted_bytes * _INVERTED_MEM_MARGIN

    @staticmethod
    def _row_norms(matrix: np.ndarray) -> np.ndarray:
        """每行的 L2 范数，形状 (n,)，**不**产生 n×vocab 的临时矩阵

        `np.linalg.norm(matrix, axis=1)` 语义相同但会另起一份同尺寸矩阵做平方和；
        在稠密编码矩阵（n×vocab，真实数据 0.64 GB 起）上那是纯浪费。
        einsum 的逐行点积不物化中间矩阵，且 float32 累加结果与原实现逐元素相同。
        """
        return np.sqrt(np.einsum("ij,ij->i", matrix, matrix))
    
    @staticmethod
    def _normalize(embeddings: np.ndarray) -> np.ndarray:
        """按行做 L2 归一化（零向量保持零向量，避免除零）

        返回**新矩阵**；就地版本见 :meth:`_normalize_inplace`——在 n×vocab 的
        稠密编码矩阵上，多养一份同尺寸矩阵是 0.64 GB 量级的浪费。
        """
        norms = Deduplicator._row_norms(embeddings)[:, None]
        norms[norms == 0] = 1
        return embeddings / norms

    @staticmethod
    def _normalize_inplace(embeddings: np.ndarray) -> None:
        """就地按行 L2 归一化（零向量保持零向量，避免除零）

        只可用于**调用方自己拥有**的矩阵：它不返回新数组，直接改写传入的矩阵。
        """
        norms = Deduplicator._row_norms(embeddings)[:, None]
        norms[norms == 0] = 1
        np.divide(embeddings, norms, out=embeddings)

    def _row_block_size(self, n: int, requested: int) -> int:
        """把行块大小压到「单块单元数 ≤ _MAX_BLOCK_CELLS」

        这是峰值内存不随 n 平方增长的关键：无论 n 多大，任一时刻持有的
        相似度块最多 `_MAX_BLOCK_CELLS` 个 float32。

        Args:
            n: 数据总量
            requested: 期望的行块大小

        Returns:
            实际可用的行块大小（至少为 1）
        """
        if n <= 0:
            return 1
        return max(1, min(requested, _MAX_BLOCK_CELLS // n))

    def _iter_similarity_row_blocks(self, normalized: np.ndarray, block_rows: int):
        """按行分块产出 `(行起点, 行终点, 相似度块)`

        与 :meth:`_compute_similarity_matrix_chunked` 的本质区别：这里
        **从不构造完整的 n×n 矩阵**，每次只产出一个 `(block_rows × (n-start))`
        的块，调用方消费完该块后即可被回收。

        列从 `start` 起算，而不是从 0：贪心归组只在 `j > i` 的方向上消费
        （见 :meth:`_greedy_group_from_row` 的 `row[i + 1 - start:]`），而块内
        最小的行号就是 `start`，所以列 < start 的那些结果**按定义没有读者**。
        以前每次都算满 `block_rows × n`，等于把整个下三角白算一遍——实测
        全量 6902 条真实数据时，相似度这一步占 31.2 s 里的 30.0 s，
        其中约一半 flops 落在这段被丢弃的列上。

        Args:
            normalized: 已 L2 归一化的向量矩阵
            block_rows: 每个块包含的行数

        Yields:
            (start, end, block)，block 形状为 (end-start, n-start)，
            其列下标 c 对应全局下标 c + start
        """
        n = len(normalized)
        for start in range(0, n, block_rows):
            end = min(start + block_rows, n)
            block = np.dot(normalized[start:end], normalized[start:].T)
            yield start, end, block
            # 交付后立刻松开生成器自身的引用：否则恢复执行时会先算好下一块、
            # 再覆盖这个变量，导致新旧两块同时存活（峰值凭空翻倍）。
            block = None

    def _iter_inverted_similarity_row_blocks(self, inverted: _InvertedNgrams, block_rows: int):
        """按行分块产出 `(行起点, 行终点, 相似度块)`——倒排累加版

        产出与 :meth:`_iter_similarity_row_blocks` **同形状同语义**：块是
        `(end-start) × (n-start)` 的 float32，列下标 c 对应全局 c+start。
        归组那一段因此不需要知道向量到底是稠密的还是倒排的。

        区别只在代价：这里从不构造 n×vocab 矩阵。真实 6902 条数据的那份矩阵
        是 606 MB，而其中非零元素只有 19.4 万个（0.12%）——BLAS 不知道这一点，
        每个相似度单元都要走完 23033 维，于是这一步 99% 的乘法都在乘零。
        倒排反过来只走「两个文档共享的词条」：对每个词条，把它在块内的条目
        与它在全文档里的 postings 做一次外积散射，总次数就是 `pair_adds`。

        累加顺序与 BLAS 不同（这里按词条升序、BLAS 分块并行），所以块内数值
        与稠密实现差在最后一位：真实 6902 条逐块对照最大绝对差 9.5e-7，
        阈值 0.9 下**判定不同的单元数为 0**。

        Args:
            inverted: :meth:`_inverted_view` 的产物
            block_rows: 每个块包含的行数
        """
        n = inverted.doc_count
        all_terms = inverted.terms
        all_docs = inverted.docs
        all_values = inverted.values

        for start in range(0, n, block_rows):
            end = min(start + block_rows, n)
            width = n - start
            # 列只覆盖 [start, n)：先按文档号切出「尾段」，它仍然按词条有序，
            # 于是同词条的一段 postings 可以用两次 searchsorted 直接夹出来。
            tail = all_docs >= start
            terms, docs, values = all_terms[tail], all_docs[tail], all_values[tail]
            head = docs < end
            entry_terms = terms[head]
            entry_rows = docs[head] - start
            entry_values = values[head]

            run_lo = np.searchsorted(terms, entry_terms, "left")
            run_hi = np.searchsorted(terms, entry_terms, "right")
            run = run_hi - run_lo
            # 展开成 pair 级数组：第 k 条 entry 摊开成 run[k] 个 (行, 列, 权重)。
            # run 全为 0 时这些数组都是空的，bincount 靠 minlength 兜出全零块。
            offsets = np.arange(int(run.sum())) - np.repeat(np.cumsum(run) - run, run)
            picked = np.repeat(run_lo, run) + offsets
            weights = np.repeat(entry_values, run)
            weights *= values[picked]
            flat = np.repeat(entry_rows, run) * width + (docs[picked] - start)
            block = np.bincount(flat, weights=weights,
                                minlength=(end - start) * width).astype(np.float32)
            yield start, end, block.reshape(end - start, width)
            # 同稠密版：块和 pair 级的几条大临时数组都要在算下一块前松手
            block = None
            weights = None
            flat = None
            picked = None

    def _similarity_blocks(self, texts: List[str], chunk_size: int):
        """产出相似度块——两条实现在此二选一

        选路只依据 :meth:`_inverted_blocks_win` 的代价预言机，**两条路算的是
        同一批归一化 TF 向量的点积**，分组语义相同。判据不看编码器就不成立：
        真实语义模型的向量每个维度都非零，倒排的累加次数会和稠密乘加数相等，
        而单次散射实测贵约三个数量级——所以只有 fallback 编码器才进入倒排候选。
        `_batch_encode` 仍然是稠密那条的唯一入口——它是测试注入向量的接缝。

        Args:
            texts: 待去重的文本列表（非空）
            chunk_size: 期望的行块大小
        """
        block_rows = self._row_block_size(len(texts), chunk_size)

        if self._model == "fallback":
            inverted = self._inverted_view(texts)
            if self._inverted_blocks_win(inverted.doc_count, inverted.vocab_size,
                                         inverted.pair_adds, block_rows):
                yield from self._iter_inverted_similarity_row_blocks(inverted, block_rows)
                return
            # 判负就立刻松手：稠密那条要独吞 n×vocab 大矩阵（真实数据 0.64 GB），
            # 别让只为选路而建的 postings 陪它一起活。
            inverted = None

        # 批量编码。dtype 已是 float32 时 np.asarray 不复制，所以这份矩阵归
        # 本生成器所有；就地归一化省掉的正是「再养一份 n×vocab」——真实 6902 条
        # 数据的稠密编码矩阵有 0.64 GB，复制一次的峰值直接翻倍。
        embeddings = np.asarray(self._batch_encode(texts), dtype=np.float32)
        self._normalize_inplace(embeddings)
        yield from self._iter_similarity_row_blocks(embeddings, block_rows)

    def _compute_similarity_matrix_chunked(self, embeddings: np.ndarray, chunk_size: int = 1000) -> np.ndarray:
        """分块计算**完整**相似度矩阵

        .. warning::
           本方法按定义必须返回 n×n 矩阵，因此内存占用是 **O(n²)**——
           分块只降低了临时峰值，并不能避免最终那个大矩阵的分配。
           n=50000 时该矩阵约 10GB，会直接 OOM。

           生产路径（去重分组）已改用 :meth:`_similarity_blocks`——它在
           :meth:`_iter_similarity_row_blocks` 与倒排累加之间按代价选一条，
           两条都不构造完整矩阵。本方法仅保留给小数据集的一次性检视与测试使用。

        Args:
            embeddings: 向量矩阵
            chunk_size: 分块大小

        Returns:
            相似度矩阵（n×n）
        """
        n = len(embeddings)
        normalized = self._normalize(embeddings)

        # 分块计算相似度
        similarity_matrix = np.zeros((n, n), dtype=np.float32)

        for i in range(0, n, chunk_size):
            end_i = min(i + chunk_size, n)
            chunk_i = normalized[i:end_i]

            for j in range(i, n, chunk_size):
                end_j = min(j + chunk_size, n)
                chunk_j = normalized[j:end_j]

                # 计算块间相似度
                block_sim = np.dot(chunk_i, chunk_j.T)

                # 填充对称矩阵
                similarity_matrix[i:end_i, j:end_j] = block_sim
                if i != j:
                    similarity_matrix[j:end_j, i:end_i] = block_sim.T

        return similarity_matrix

    def _find_duplicate_groups_chunked(self, texts: List[str], chunk_size: int = 1000) -> List[List[int]]:
        """按行分块查找重复组（内存占用与 n 成线性，而非平方）

        分组语义与逐行贪心完全一致：按索引升序扫描，每个尚未归组的 i 成为
        新组的代表，把其后所有「与 i 相似度达标、且尚未归组」的 j 并入该组。
        相似度块由 :meth:`_similarity_blocks` 供给（稠密 BLAS 或倒排累加，
        按代价二选一），本方法只消费块、不关心向量长什么样。

        Args:
            texts: 文本列表
            chunk_size: 期望的行块大小（会被 `_MAX_BLOCK_CELLS` 进一步压缩）

        Returns:
            重复组列表
        """
        n = len(texts)

        if n == 0:
            return []

        # 查找重复组
        visited = [False] * n
        duplicate_groups: List[List[int]] = []

        for start, end, block in self._similarity_blocks(texts, chunk_size):
            for offset, i in enumerate(range(start, end)):
                if visited[i]:
                    continue

                group = self._greedy_group_from_row(block, offset, i, start, visited)
                if len(group) > 1:
                    duplicate_groups.append(group)

            # 显式释放本块。注意归组逻辑必须封装在方法里：若在此处直接写
            # `row = block[offset]`，row 作为**视图**会在 del block 之后继续
            # 持有整个块，导致生成器算下一块时新旧两块同时存活（峰值翻倍）。
            del block

        return duplicate_groups

    def _greedy_group_from_row(self,
                               block: np.ndarray,
                               offset: int,
                               i: int,
                               start: int,
                               visited: List[bool]) -> List[int]:
        """以 i 为组代表，把其后「相似度达标且尚未归组」的 j 并入同一组

        贪心语义与逐行扫描一致：只在 i 之后找，且已归组的 j 不再被认领。

        Args:
            block: 当前相似度块，形状 (block_rows, n - start)，列下标 c 对应全局 c + start
            offset: i 在 block 中的行下标
            i: 全局行索引，作为组代表
            start: 本块的起始全局行号（用于把块内列下标换回全局下标）
            visited: 归组标记（原地修改）

        Returns:
            组内索引列表（含代表 i 本身）
        """
        group = [i]
        visited[i] = True

        # 先在 C 层筛出相似度达标的候选，再在 Python 层做归组。
        # 若写成 `for j in range(i+1, n)` 逐元素比较，会把 O(n²) 次比较
        # 全部压到解释器里，大数据集下慢到不可用。
        # 块只覆盖列 [start, n)，故全局列 j 在块内是 j - start；i >= start，
        # 切片起点 `i + 1 - start` 恒 >= 1（跳过自己及自己左侧的列）。
        row = block[offset]
        candidates = np.flatnonzero(row[i + 1 - start:] >= self.threshold) + i + 1
        for j in candidates:
            j = int(j)
            if not visited[j]:
                group.append(j)
                visited[j] = True

        return group

    def _row_pairs_above(self, block: np.ndarray, offset: int, i: int,
                         floor: float):
        """取一行里「与 i 达标」的 `(全局 j, 相似度)`，只看 `j > i` 的方向

        返回的是**新建数组**（fancy indexing 会复制），所以调用方留着它们
        也不会把整个块钉在内存里——同 :meth:`_greedy_group_from_row` 的顾虑：
        块切片是**视图**，视图活着块就回收不掉，生成器算下一块时新旧两块并存。

        Args:
            block: 当前相似度块，形状 (block_rows, n - start)，列下标 c 对应全局 c + start
            offset: i 在 block 中的行下标
            i: 全局行索引
            floor: 收录门槛（相似度下界）

        Returns:
            (j 数组, 相似度数组)；本行无达标项时为 (None, None)
        """
        row = block[offset, offset + 1:]
        hits = np.flatnonzero(row >= floor)
        if hits.size == 0:
            return None, None
        return hits + i + 1, row[hits]

    def _find_similar_pairs_chunked(self, texts: List[str], top_k: int,
                                    chunk_size: int = 1000) -> List[Tuple[int, int, float]]:
        """在**全部**文档的两两相似度里取前 top_k（峰值不随达标对数增长）

        相似度块由 :meth:`_similarity_blocks` 供给，与归组那条路共用同一个
        生产者、同一份代价选路（稠密 BLAS 或倒排累加）。这里从不构造 n×n，
        也不构造 n×vocab 的编码矩阵（判倒排赢时）；候选对按行消费，每行结束
        就把保留集裁回 top_k，所以它的大小至多 `top_k + 单行达标数`（≤ top_k+n），
        **不随总达标对数增长**——「整档两两都达标」的全重复语料在「先收完再排」
        的写法下要养 O(n²) 个候选，在这里只养这么多。

        Args:
            texts: 文本列表（长度 ≥ 2）
            top_k: 返回的最相似对数量
            chunk_size: 期望的行块大小（会被 `_MAX_BLOCK_CELLS` 进一步压缩）

        Returns:
            `[(i, j, similarity), ...]`，相似度降序；同分时按 `(i, j)` 升序
        """
        floor = self.threshold * 0.8

        kept_i = np.empty(0, dtype=np.int64)
        kept_j = np.empty(0, dtype=np.int64)
        kept_s = np.empty(0, dtype=np.float32)

        for start, end, block in self._similarity_blocks(texts, chunk_size):
            for offset in range(end - start):
                i = start + offset
                js, sims = self._row_pairs_above(block, offset, i, floor)
                if js is None:
                    continue
                kept_i = np.concatenate((kept_i, np.full(js.size, i, dtype=np.int64)))
                kept_j = np.concatenate((kept_j, js))
                kept_s = np.concatenate((kept_s, sims))
                if kept_s.size > top_k:
                    order = self._pair_rank(kept_i, kept_j, kept_s)[:top_k]
                    kept_i, kept_j, kept_s = kept_i[order], kept_j[order], kept_s[order]
            # 显式松开本块：上面三个 kept_* 都是新建数组，不是视图，所以这里
            # 一 del 整个块就能回收。
            del block

        if kept_s.size == 0:
            return []
        order = self._pair_rank(kept_i, kept_j, kept_s)
        return [(int(i), int(j), float(s))
                for i, j, s in zip(kept_i[order], kept_j[order], kept_s[order])]

    @staticmethod
    def _pair_rank(pair_i: np.ndarray, pair_j: np.ndarray,
                   pair_s: np.ndarray) -> np.ndarray:
        """候选对的名次下标：相似度降序，同分按 `(i, j)` 升序

        同分档的次序不是可有可无的细节：旧实现把候选按 `(i, j)` 生成后用
        **稳定** `sort(key=相似度)`，于是并列时就是 `(i, j)` 升序。倒排/稠密两条
        生产者的 float32 末位差异会在「数学上恰好相等」的配对上造成并列
        （见 A36），名次规则必须与生产者无关，否则同一份数据换个内部实现
        就会换一批 top_k。
        """
        return np.lexsort((pair_j, pair_i, -pair_s))

    def _find_duplicates_faiss(self, texts: List[str], embeddings: np.ndarray) -> List[List[int]]:
        """使用 FAISS 加速查找重复
        
        Args:
            texts: 文本列表
            embeddings: 向量矩阵
        
        Returns:
            重复组列表
        """
        try:
            import faiss
            
            # 创建 FAISS 索引
            dimension = embeddings.shape[1]
            index = faiss.IndexFlatIP(dimension)  # 内积（余弦相似度，假设已归一化）
            
            # 归一化
            normalized = self._normalize(embeddings)
            
            index.add(normalized.astype('float32'))
            
            # 搜索相似项
            n = len(texts)
            visited = [False] * n
            duplicate_groups = []
            
            for i in range(n):
                if visited[i]:
                    continue
                
                # 搜索最近邻
                query = normalized[i:i+1].astype('float32')
                k = min(n, 100)  # 限制搜索数量
                distances, indices = index.search(query, k)
                
                group = [i]
                visited[i] = True
                
                for j, dist in zip(indices[0], distances[0]):
                    if j == i or visited[j]:
                        continue
                    
                    if dist >= self.threshold:
                        group.append(j)
                        visited[j] = True
                
                if len(group) > 1:
                    duplicate_groups.append(group)
            
            return duplicate_groups
            
        except ImportError:
            raise ImportError("FAISS 未安装")
    
    def deduplicate(self, 
                   items: List[Dict],
                   text_key: str = "instruction") -> DedupResult:
        """执行去重
        
        Args:
            items: 数据列表
            text_key: 用于去重的文本字段名
        
        Returns:
            DedupResult 实例
        """
        self._load_model()
        
        original_count = len(items)
        
        if original_count == 0:
            return DedupResult(
                original_count=0,
                deduplicated_count=0,
                removed_count=0,
                duplicate_groups=[],
                kept_indices=[]
            )
        
        # 提取文本
        texts = [item.get(text_key, "") for item in items]
        
        # 查找重复组：按行分块流式计算，峰值内存与 n 成线性
        duplicate_groups = self._find_duplicate_groups_chunked(texts)
        
        # 确定保留的索引
        removed_indices = set()
        for group in duplicate_groups:
            # 保留第一个，移除其余
            for idx in group[1:]:
                removed_indices.add(idx)
        
        kept_indices = [i for i in range(original_count) if i not in removed_indices]
        
        return DedupResult(
            original_count=original_count,
            deduplicated_count=len(kept_indices),
            removed_count=len(removed_indices),
            duplicate_groups=duplicate_groups,
            kept_indices=kept_indices
        )
    
    def deduplicate_and_filter(self,
                              items: List[Dict],
                              text_key: str = "instruction") -> List[Dict]:
        """执行去重并返回过滤后的数据
        
        Args:
            items: 数据列表
            text_key: 用于去重的文本字段名
        
        Returns:
            去重后的数据列表
        """
        result = self.deduplicate(items, text_key)
        return [items[i] for i in result.kept_indices]
    
    def generate_report(self,
                       items: List[Dict],
                       text_key: str = "instruction") -> Dict:
        """生成去重报告
        
        Args:
            items: 数据列表
            text_key: 用于去重的文本字段名
        
        Returns:
            去重报告字典
        """
        result = self.deduplicate(items, text_key)
        
        return {
            "original_count": result.original_count,
            "deduplicated_count": result.deduplicated_count,
            "removed_count": result.removed_count,
            "removal_rate": result.removed_count / result.original_count if result.original_count > 0 else 0,
            "duplicate_groups_count": len(result.duplicate_groups),
            "avg_group_size": np.mean([len(g) for g in result.duplicate_groups]) if result.duplicate_groups else 0,
            "threshold": self.threshold
        }
    
    def find_similar_pairs(self,
                          items: List[Dict],
                          text_key: str = "instruction",
                          top_k: int = 10) -> List[Tuple[int, int, float]]:
        """查找最相似的文本对（在**全部**条目之间比较）

        收录门槛是 `threshold × 0.8`——刻意比去重阈值松一档，让调用方看得见
        「接近但没到重复」的配对；这个 0.8 是既有口径，改动会让 top_k 内容
        整体漂移（见 A39）。名次规则：相似度降序，**同分时按 `(i, j)` 升序**。

        Args:
            items: 数据列表
            text_key: 用于比较的文本字段名
            top_k: 返回前 k 个最相似对，不小于 0 的整数。判据在
                `augmentor.validation.require_count`：负数会以「裁掉保留集末尾
                |top_k| 个」的身份穿过剪枝逻辑，把有解的请求答成空列表

        Returns:
            相似对列表 [(idx1, idx2, similarity), ...]
        """
        require_count("top_k", top_k)

        self._load_model()

        texts = [item.get(text_key, "") for item in items]

        if len(texts) < 2:
            return []

        return self._find_similar_pairs_chunked(texts, top_k)
