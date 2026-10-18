# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""训练/测试集泄漏检测模块

检测训练集与测试集之间是否存在重复或近似重复样本（数据泄漏），
避免同一问题同时出现在训练与评测两侧导致指标虚高。

设计原则：
- 本地离线实现，不依赖任何服务；近似计数的稠密路线用 numpy（仓内硬依赖）；
- 精确匹配基于归一化签名，近似匹配基于字符三元组 Jaccard 相似度；
- 倒排索引把候选集缩到「共享至少一个三元组」的训练签名，并按可采纳剪枝
  `I >= t*|a|` 进一步收窄；相似度由「共享三元组个数 + 两侧基数」直接算出，
  热路径不做任何集合运算 ⇒ 规模翻倍时增长是线性的，不再等于全量两两比对；
- posting 塌缩档（词表小、子串共享率高 ⇒ 每条测试样本摊平几乎整份索引）按
  :func:`_dense_route_wins` 的代价账改走 numpy 0/1 矩阵乘法，逐条判决与倒排
  **完全相同**（实测见 `tests/unit/test_leakage_dense_route_l240.py`）；
- 输出可审计报告，附泄漏示例与处置建议。
"""

import logging
import re
from collections import Counter
from dataclasses import dataclass, field
from itertools import chain
from typing import Any, Dict, List, Optional, Set, Tuple

import numpy as np

from augmentor.validation import DataValidationError, require_count, require_ratio

logger = logging.getLogger(__name__)

# 归一化用：仅保留字母数字与中文，丢弃空白与标点
_NORM_RE = re.compile(r"[\u4e00-\u9fff0-9a-z]")

#: 稠密路线时间账的单价：一次「摊平 posting 累加」折算多少次 matmul 乘加。
#: 两侧实测（Temp/l240_leakprobe8.py，同一台机器、同一条 24 词模板语料与一条 48 词变体）：
#: 判赢侧最差一档是 48 词 4000×4000 —— 摊平 3.61 亿次 vs 乘加 98.2 亿次，比值 27.2，
#: 而那一档倒排 55.7 s / 稠密 0.72 s（稠密贵 77 倍）；判负侧唯一一档是随机五字母词
#: 4000×4000 —— 摊平 1.06 千万次 vs 乘加 2810 亿次，比值 26,500，那一档倒排 0.52 s /
#: 稠密 0.97 s（倒排赢 1.9 倍）。取 1000 落在两档之间且偏向保守：判赢的最差档离它 37 倍，
#: 判负的那档离它 26 倍，两侧都不会因为比值小幅抖动换队。
_DENSE_ADD_COST = 1000

#: 稠密视图 `vocab × n_train` 的 float32 字节硬上限。时间账管不到「词表宽 *且* posting
#: 也宽」的形状（那种档比值照样判赢，但要物化几十 GB），所以内存单独封顶。
#: 实测两侧：10k×10k 模板档视图 26.7 MB（走稠密，端到端见 L240 账本读数），随机词档
#: 281 MB（走倒排，那一档稠密本来就被时间账判负）。取 64 MiB ⇒ 与单块临时数组
#: （:data:`_DENSE_MAX_CELLS` × 24 B ≈ 24 MB）合起来把一次检测的稠密侧峰值压在
#: 约 90 MB 以内，同时给 10k×10k 那一档留 2.4 倍余量。
_DENSE_MAX_BYTES = 64 * 1024 * 1024

#: 单块最多处理多少条测试签名。块越大 BLAS 越划算，但峰值临时数组正比于 `行 × n_train`，
#: 所以块大小由 :data:`_DENSE_MAX_CELLS` 反推，这里只是上限（小训练集时不必把整份测试集
#: 堵在一个块里，也保证倒排档的摊平记账不失控）。
_DENSE_MAX_ROWS = 1024

#: 单块 matmul 结果的最大格数（行 × n_train）。一格在判决里最多活三次（float32 累加、
#: float64 商、float64 并集），所以峰值临时内存 ≈ 该格数 × 24 B。取 100 万格 ⇒ 约 24 MB，
#: 在 10k×10k 那一档给出 102 行一块。
_DENSE_MAX_CELLS = 1_000_000


def _normalize(text: str) -> str:
    """归一化文本（小写、去空白/标点，保留中英数字）

    Args:
        text: 原始文本

    Returns:
        归一化字符串
    """
    if not isinstance(text, str):
        return ""
    return "".join(_NORM_RE.findall(text.lower()))


def _char_trigrams(normalized: str) -> Set[str]:
    """生成字符三元组集合

    Args:
        normalized: 归一化文本

    Returns:
        三元组集合
    """
    if len(normalized) < 3:
        return {normalized} if normalized else set()
    return {normalized[i:i + 3] for i in range(len(normalized) - 2)}


def _jaccard_from_counts(intersection: int, size_a: int, size_b: int) -> float:
    """由「交集大小 + 两侧基数」算 Jaccard（并集不必真的建出来）

    倒排索引里逐对数出来的共享三元组个数**就是**交集大小，所以并集可以按
    `|a| + |b| - |a∩b|` 算出来，热路径因此一次集合运算都不做。这一支是算式的
    唯一权威，`_jaccard` 只是它的集合版外壳。

    Args:
        intersection: 交集元素个数
        size_a: 集合 A 的基数
        size_b: 集合 B 的基数

    Returns:
        相似度 (0-1)
    """
    union = size_a + size_b - intersection
    return intersection / union if union else 0.0


def _jaccard(a: Set[str], b: Set[str]) -> float:
    """Jaccard 相似度（集合版外壳，两侧已是集合时用）

    Args:
        a: 集合 A
        b: 集合 B

    Returns:
        相似度 (0-1)
    """
    if not a or not b:
        return 0.0
    return _jaccard_from_counts(len(a & b), len(a), len(b))


def _dense_block_rows(n_train: int) -> int:
    """行块大小：只由训练侧条数反推，纯整数运算，不看墙钟（A286 同口径）

    Args:
        n_train: 去重后的训练签名条数

    Returns:
        每块最多处理多少条测试签名（>= 1）
    """
    if n_train <= 0:
        return _DENSE_MAX_ROWS
    return max(1, min(_DENSE_MAX_ROWS, _DENSE_MAX_CELLS // n_train))


def _dense_route_wins(block_rows: int, n_train: int, vocab_size: int,
                      flatten_adds: int) -> bool:
    """这个块的近似计数该走 numpy 稠密矩阵乘法，还是沿用摊平倒排

    两条独立的账，与 `dedup.py` 的选路判据同族：

    1. **时间**——倒排要做 `flatten_adds` 次 Python 级「把 posting 元素喂进计数器」，
       稠密要做 `block_rows × n_train × vocab_size` 次乘加；一次 Python 累加按实测
       折算 :data:`_DENSE_ADD_COST` 次乘加，只有倒排那份更贵才换路。
    2. **内存**——稠密要物化 `n_train × vocab_size` 的 float32 视图，
       超过 :data:`_DENSE_MAX_BYTES` 就不许走（时间账管不到「词表宽且 posting 宽」）。

    任何一侧为 0（没测试样本、没训练签名、没词表、摊平次数为零）都不换路：走倒排那条
    空转即可，而且稠密视图在 `n_train` 或 `vocab` 为 0 时压根选不出 argmax。

    Args:
        block_rows: 本块里非精确匹配的测试签名条数
        n_train: 去重后的训练签名条数
        vocab_size: 三元组词表大小，即稠密视图的行数
        flatten_adds: 倒排方案在本块必须摊平的 posting 元素总数

    Returns:
        True 表示本块走稠密计数
    """
    if block_rows <= 0 or n_train <= 0 or vocab_size <= 0 or flatten_adds <= 0:
        return False
    if n_train * vocab_size * 4 > _DENSE_MAX_BYTES:
        return False
    return flatten_adds * _DENSE_ADD_COST >= block_rows * n_train * vocab_size


def _inverted_flags(rows: List[Set[str]],
                    gram_index: Dict[str, Set[int]],
                    sig_gram_counts: List[int],
                    threshold: float) -> List[bool]:
    """摊平倒排计数：逐条测试签名把自己全部三元组的 posting 串起来数出现次数

    Args:
        rows: 块内非精确匹配测试签名的三元组集合，按测试顺序
        gram_index: 三元组 → 训练签名序号集合
        sig_gram_counts: 训练签名序号 → 该签名三元组个数
        threshold: Jaccard 阈值

    Returns:
        与 rows 同序的布尔列表，True 表示判为近似泄漏
    """
    flags: List[bool] = []
    for grams in rows:
        size_a = len(grams)
        # 剪枝：J = I/U 且 U >= |a| ⇒ J >= t 蕴含 I >= t*|a|。取整用 int()（向零
        # 截断）而不是 ceil：浮点误差只会**少剪**，不会把该比的对误剪掉（L30 平局教训）。
        need = int(threshold * size_a)
        shared = Counter(chain.from_iterable(
            gram_index.get(gram, ()) for gram in grams))
        best = max(
            (_jaccard_from_counts(overlap, size_a, sig_gram_counts[sid])
             for sid, overlap in shared.items() if overlap >= need),
            # 旧版在这里从 best=0.0 起算，所以「阈值 <= 0 且无可比候选」也判 fuzzy；
            # 这条 default 就是那一份口径，不许顺手改掉
            default=0.0,
        )
        flags.append(best >= threshold)
    return flags


def _dense_view(gram_index: Dict[str, Set[int]],
                sig_gram_counts: List[int]) -> Tuple[Dict[str, int], np.ndarray, np.ndarray]:
    """把倒排 posting 铺成 `(词表 × 训练条数)` 的 0/1 float32 视图

    行序 = `gram_index` 的插入序（即训练侧首次出现序），与 :func:`_dense_flags` 里
    查列号用的 `gram_ids` 同源，所以两者必须一起建、一起用。

    Args:
        gram_index: 三元组 → 训练签名序号集合
        sig_gram_counts: 训练签名序号 → 该签名三元组个数

    Returns:
        `(三元组 → 行号, 视图, 每列基数)`，最后一项是 float64 整数值的数组
    """
    gram_ids = {gram: row for row, gram in enumerate(gram_index)}
    mat = np.zeros((len(gram_ids), len(sig_gram_counts)), dtype=np.float32)
    for row, sids in enumerate(gram_index.values()):
        line = mat[row]
        for sid in sids:
            line[sid] = 1.0
    size_b = np.fromiter(sig_gram_counts, dtype=np.float64, count=len(sig_gram_counts))
    return gram_ids, mat, size_b


def _dense_flags(rows: List[Set[str]],
                 gram_ids: Dict[str, int],
                 gram_mat: np.ndarray,
                 size_b: np.ndarray,
                 threshold: float) -> List[bool]:
    """稠密计数：一次 float32 矩阵乘法出整块「共享三元组数」，判决仍回权威算式

    与 :func:`_inverted_flags` **逐条判决相同**——这不是近似，可证的链条是：

    1. `ov` 是整数（0/1 矩阵乘法的值 ≤ min(|a|, |b|) < 2^24 ⇒ float32 精确表示），
       等于该条与每个训练签名的三元组交集基数；
    2. 判据 `best >= t`（`best` 为 max，且 t > 0 由构造器保证）等价于「存在一个候选
       使 J >= t」，所以剪枝剪不剪都不改判决；
    3. argmax 在 float64 下选，两个整数在 float64 里的商与 Python 的 `int / int`
       逐位相同（同为 IEEE 754 正确舍入）⇒ 选出的就是倒排那条算出的同一个 `best`；
    4. 最终 J 由 :func:`_jaccard_from_counts` 现算，稠密这条路不引入第二套相似度算式。

    Args:
        rows: 块内非精确匹配测试签名的三元组集合
        gram_ids: 三元组 → 视图行号
        gram_mat: :func:`_dense_view` 的视图
        size_b: :func:`_dense_view` 的每列基数
        threshold: Jaccard 阈值

    Returns:
        与 rows 同序的布尔列表，True 表示判为近似泄漏
    """
    size_a = np.fromiter((len(gs) for gs in rows), dtype=np.float64, count=len(rows))
    test_mat = np.zeros((len(rows), gram_mat.shape[0]), dtype=np.float32)
    for r, grams in enumerate(rows):
        line = test_mat[r]
        for gram in grams:
            row = gram_ids.get(gram)
            if row is not None:
                line[row] = 1.0
    ov = (test_mat @ gram_mat).astype(np.float64)
    # union 恒 >= max(|a|, |b|) >= 1：签名非空才有三元组进这里，所以不写除零分支
    union = size_a[:, None] + size_b[None, :] - ov
    arg = (ov / union).argmax(axis=1)
    return [
        _jaccard_from_counts(int(ov[r, arg[r]]),
                             int(size_a[r]),
                             int(size_b[arg[r]])) >= threshold
        for r in range(len(rows))
    ]


@dataclass
class LeakageReport:
    """泄漏检测报告"""

    train_size: int
    test_size: int
    exact_leaks: int = 0
    fuzzy_leaks: int = 0
    leaked_examples: List[Dict[str, Any]] = field(default_factory=list)

    @property
    def total_leaks(self) -> int:
        return self.exact_leaks + self.fuzzy_leaks

    @property
    def leak_rate(self) -> float:
        """泄漏率 = 泄漏测试条数 / 测试集总量"""
        if self.test_size == 0:
            return 0.0
        return self.total_leaks / self.test_size

    @property
    def is_clean(self) -> bool:
        return self.total_leaks == 0

    def to_dict(self) -> Dict[str, Any]:
        return {
            "train_size": self.train_size,
            "test_size": self.test_size,
            "exact_leaks": self.exact_leaks,
            "fuzzy_leaks": self.fuzzy_leaks,
            "total_leaks": self.total_leaks,
            "leak_rate": self.leak_rate,
            "is_clean": self.is_clean,
            "leaked_examples": self.leaked_examples,
        }


class LeakageDetector:
    """训练/测试集泄漏检测器"""

    def __init__(self,
                 fields: Optional[List[str]] = None,
                 fuzzy_threshold: float = 0.8,
                 min_examples: int = 10):
        """初始化检测器

        Args:
            fields: 参与比较的字段名，缺省为 instruction
            fuzzy_threshold: 近似匹配的 Jaccard 阈值（>= 判定为泄漏）。可用区间 (0, 1]：
                0 档是「假零」（Jaccard 下界 0 ⇒ 任何 token 相交即判泄漏，干净数据也报
                100% 泄漏），> 1 档「静默关闭」（Jaccard 永不超过 1，永远报不出近似泄漏）。
                两方向都能造出假读数，故构造期即拒（A172，L181；fail-loud 方向）。
            min_examples: 报告中最多保留的泄漏示例条数。只收「非负整数」：0 是合法
                的「报告不带示例」意图（与「没传参数」区分开），负数会让 `[:n]` 切片
                换语义（`[:-2]` = 丢掉最后 2 条、要得越多剩得越多），坏形状（None /
                非整数 / bool）一并在构造期拒掉（计数旋钮族，require_count 先例 L144+）。

        Raises:
            DataValidationError: fuzzy_threshold 为 None / 非数值 / bool / NaN / 越界（(0,1] 之外）；
                min_examples 为 None / 负数 / 非整数 / bool
        """
        # fuzzy_threshold 权威判据住构造器（A77 一条判据一处）：API/CLI 面经 detect_leakage
        # 委托到本构造器，自动吃到同一档；下界取开（0 档是假零家族 L144 同式），上界取闭。
        if fuzzy_threshold is None:
            raise DataValidationError(
                "fuzzy_threshold 不能为 None（必填比例旋钮，显式 None 属坏值非缺省）"
            )
        fuzzy_threshold = require_ratio(
            "fuzzy_threshold", fuzzy_threshold, minimum=0.0, maximum=1.0
        )
        if fuzzy_threshold <= 0:
            raise DataValidationError(
                "fuzzy_threshold 必须大于 0（0 档会把干净数据全判为泄漏，假零家族 L144 同式）"
            )
        self.fields = fields if fields is not None else ["instruction"]
        if not self.fields:
            raise DataValidationError(
                "fields 不能是空清单：零字段作用域下所有样本的签名同为分隔符，"
                "会把「无从比对」报成「无泄漏」（假干净档）"
            )
        self.fuzzy_threshold = fuzzy_threshold
        # min_examples 是计数旋钮（require_count 口径）：0 = 「报告不带示例」的合法意图，
        # 负数档让 `[:n]` 切片换语义（`[:-2]` = 丢掉最后 2 条，要得越多剩得越多）⇒ 拒；
        # 显式 None 是坏值而非缺省（L181 的 fuzzy_threshold None 专判先例）。
        if min_examples is None:
            raise DataValidationError(
                "min_examples 不能为 None（必填计数旋钮，显式 None 属坏值非缺省）"
            )
        self.min_examples = require_count("min_examples", min_examples, minimum=0)

    def _signature(self, item: Dict[str, Any]) -> str:
        parts = [_normalize(str(item.get(f, ""))) for f in self.fields]
        return "||".join(parts)

    def detect(self,
               train_items: List[Dict],
               test_items: List[Dict]) -> LeakageReport:
        """检测训练集与测试集之间的泄漏

        Args:
            train_items: 训练集
            test_items: 测试集

        Returns:
            LeakageReport 实例
        """
        report = LeakageReport(train_size=len(train_items), test_size=len(test_items))
        if not test_items:
            return report

        # 训练侧：签名去重成序号 + 三元组→序号倒排 + 序号→该签名三元组个数
        # （旧版在这里另存一份 `sig -> 三元组集合` 供逐对交并；改成「共享元组数即交集」
        #  之后那整份第二拷贝不再需要，峰值内存随训练侧三元组总量减半）
        sig_ids: Dict[str, int] = {}
        gram_index: Dict[str, Set[int]] = {}
        sig_gram_counts: List[int] = []
        for item in train_items:
            sig = self._signature(item)
            if not sig or sig in sig_ids:
                continue
            sid = len(sig_gram_counts)
            sig_ids[sig] = sid
            grams = _char_trigrams(sig)
            sig_gram_counts.append(len(grams))
            for gram in grams:
                gram_index.setdefault(gram, set()).add(sid)

        threshold = self.fuzzy_threshold
        n_train = len(sig_gram_counts)
        vocab_size = len(gram_index)
        block_rows = _dense_block_rows(n_train)
        # 稠密视图按代价账现建，且只建一次；全程走倒排的档位一个 numpy 数组都不碰
        view: Optional[Tuple[Dict[str, int], np.ndarray, np.ndarray]] = None
        kinds: List[Optional[str]] = []
        for start in range(0, len(test_items), block_rows):
            chunk_kinds: List[Optional[str]] = []
            rows: List[Set[str]] = []
            positions: List[int] = []
            flatten_adds = 0
            for test_item in test_items[start:start + block_rows]:
                sig = self._signature(test_item)
                if not sig:
                    chunk_kinds.append(None)
                    continue
                if sig in sig_ids:
                    chunk_kinds.append("exact")
                    continue
                grams = _char_trigrams(sig)
                chunk_kinds.append(None)
                positions.append(len(chunk_kinds) - 1)
                rows.append(grams)
                # 代价账的输入：倒排那条要把这些 posting 元素逐个摊进计数器，
                # 这里只数长度、不碰集合，所以记账本身不进热路径成本
                for gram in grams:
                    posting = gram_index.get(gram)
                    flatten_adds += len(posting) if posting is not None else 0
            if rows:
                if _dense_route_wins(len(rows), n_train, vocab_size, flatten_adds):
                    if view is None:
                        view = _dense_view(gram_index, sig_gram_counts)
                    flags = _dense_flags(rows, view[0], view[1], view[2], threshold)
                else:
                    flags = _inverted_flags(rows, gram_index, sig_gram_counts, threshold)
                for pos, flag in zip(positions, flags):
                    chunk_kinds[pos] = "fuzzy" if flag else None
            kinds.extend(chunk_kinds)

        for test_item, kind in zip(test_items, kinds):
            if kind == "exact":
                report.exact_leaks += 1
                report.leaked_examples.append({
                    "type": "exact",
                    "test_item": test_item,
                })
            elif kind == "fuzzy":
                report.fuzzy_leaks += 1
                report.leaked_examples.append({
                    "type": "fuzzy",
                    "test_item": test_item,
                })

        report.leaked_examples = report.leaked_examples[:self.min_examples]
        logger.info(
            "泄漏检测完成: 测试集 %d 条中 %d 条泄漏（精确 %d / 近似 %d）",
            report.test_size, report.total_leaks, report.exact_leaks, report.fuzzy_leaks,
        )
        return report


def detect_leakage(train_items: List[Dict],
                   test_items: List[Dict],
                   fields: Optional[List[str]] = None,
                   fuzzy_threshold: float = 0.8) -> LeakageReport:
    """检测数据集泄漏（模块级便捷函数）

    Args:
        train_items: 训练集
        test_items: 测试集
        fields: 参与比较的字段名
        fuzzy_threshold: 近似匹配阈值

    Returns:
        LeakageReport 实例
    """
    detector = LeakageDetector(fields=fields, fuzzy_threshold=fuzzy_threshold)
    return detector.detect(train_items, test_items)
