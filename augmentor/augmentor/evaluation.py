# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""模型评估模块

提供生成质量评估指标（BLEU / ROUGE-L / 相似度）与模型对比能力。
"""

import logging
import math
import re
from collections import Counter
from dataclasses import dataclass, field
from typing import List, Dict, Any, Optional, Callable
from .exceptions import DataValidationError

logger = logging.getLogger(__name__)

SUPPORTED_METRICS = ["bleu", "rouge_l", "similarity"]

CJK_PATTERN = re.compile(r"[\u4e00-\u9fff]")
LATIN_WORD_PATTERN = re.compile(r"[A-Za-z0-9]+")


def tokenize(text: str) -> List[str]:
    """分词

    中文按字切分，英文数字按词切分，保证中英混合场景可用。

    Args:
        text: 文本

    Returns:
        词元列表
    """
    if not isinstance(text, str) or not text:
        return []

    tokens: List[str] = []
    buffer: List[str] = []

    for char in text:
        if CJK_PATTERN.match(char):
            if buffer:
                tokens.extend(LATIN_WORD_PATTERN.findall("".join(buffer)))
                buffer = []
            tokens.append(char)
        elif char.isalnum():
            buffer.append(char)
        else:
            if buffer:
                tokens.extend(LATIN_WORD_PATTERN.findall("".join(buffer)))
                buffer = []

    if buffer:
        tokens.extend(LATIN_WORD_PATTERN.findall("".join(buffer)))

    return tokens


def _ngrams(tokens: List[str], n: int) -> Counter:
    """生成 n-gram 计数

    Args:
        tokens: 词元列表
        n: n-gram 大小

    Returns:
        n-gram 计数器
    """
    return Counter(tuple(tokens[i:i + n]) for i in range(len(tokens) - n + 1))


def compute_bleu(generated: str, reference: str, max_n: int = 4) -> float:
    """计算句子级 BLEU

    使用 1~max_n 元精度、裁剪计数、简短惩罚，并对零计数做平滑。

    Args:
        generated: 生成文本
        reference: 参考文本
        max_n: 最大 n-gram 阶数

    Returns:
        BLEU 分数 (0-1)
    """
    gen_tokens = tokenize(generated)
    ref_tokens = tokenize(reference)

    if not gen_tokens or not ref_tokens:
        return 0.0

    log_precision_sum = 0.0
    effective_n = 0

    for n in range(1, max_n + 1):
        gen_ngrams = _ngrams(gen_tokens, n)
        ref_ngrams = _ngrams(ref_tokens, n)

        if not gen_ngrams:
            break

        clipped = sum(min(count, ref_ngrams[gram]) for gram, count in gen_ngrams.items())
        total = sum(gen_ngrams.values())

        # 完全无重叠时直接判 0，避免平滑项产出无意义的极小值
        if clipped == 0:
            return 0.0

        # 平滑：极小概率避免 log(0)
        precision = (clipped + 1e-9) / total
        log_precision_sum += math.log(precision)
        effective_n += 1

    if effective_n == 0:
        return 0.0

    geo_mean = math.exp(log_precision_sum / effective_n)

    # 简短惩罚
    if len(gen_tokens) >= len(ref_tokens):
        brevity_penalty = 1.0
    else:
        brevity_penalty = math.exp(1 - len(ref_tokens) / len(gen_tokens))

    return float(max(0.0, min(1.0, brevity_penalty * geo_mean)))


def _lcs_length(a: List[str], b: List[str]) -> int:
    """计算最长公共子序列长度

    Args:
        a: 序列 A
        b: 序列 B

    Returns:
        LCS 长度
    """
    if not a or not b:
        return 0

    previous = [0] * (len(b) + 1)
    for token_a in a:
        current = [0]
        for j, token_b in enumerate(b):
            if token_a == token_b:
                current.append(previous[j] + 1)
            else:
                current.append(max(previous[j + 1], current[j]))
        previous = current

    return previous[-1]


def compute_rouge_l(generated: str, reference: str) -> float:
    """计算 ROUGE-L（基于 LCS 的 F1）

    Args:
        generated: 生成文本
        reference: 参考文本

    Returns:
        ROUGE-L 分数 (0-1)
    """
    gen_tokens = tokenize(generated)
    ref_tokens = tokenize(reference)

    if not gen_tokens or not ref_tokens:
        return 0.0

    lcs = _lcs_length(gen_tokens, ref_tokens)
    if lcs == 0:
        return 0.0

    precision = lcs / len(gen_tokens)
    recall = lcs / len(ref_tokens)
    # lcs > 0 时 precision 与 recall 均严格为正，分母不会为 0
    f1 = 2 * precision * recall / (precision + recall)
    return float(max(0.0, min(1.0, f1)))


def compute_similarity(generated: str, reference: str) -> float:
    """计算词元级 Jaccard 相似度

    Args:
        generated: 生成文本
        reference: 参考文本

    Returns:
        相似度 (0-1)
    """
    gen_tokens = set(tokenize(generated))
    ref_tokens = set(tokenize(reference))

    if not gen_tokens or not ref_tokens:
        return 0.0

    intersection = len(gen_tokens & ref_tokens)
    union = len(gen_tokens | ref_tokens)
    return float(intersection / union) if union else 0.0


METRIC_FUNCTIONS: Dict[str, Callable[[str, str], float]] = {
    "bleu": compute_bleu,
    "rouge_l": compute_rouge_l,
    "similarity": compute_similarity
}


@dataclass
class EvaluationResult:
    """评估结果"""
    metrics: Dict[str, float] = field(default_factory=dict)
    sample_count: int = 0
    details: List[Dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        """转换为字典

        Returns:
            评估结果字典
        """
        return {
            "metrics": self.metrics,
            "sample_count": self.sample_count,
            "details": self.details
        }


class ModelEvaluator:
    """模型评估器"""

    def __init__(self,
                 metrics: Optional[List[str]] = None,
                 model_factory: Optional[Callable[[str], Any]] = None):
        """初始化评估器

        Args:
            metrics: 评估指标列表
            model_factory: 模型名到后端实例的工厂函数，用于模型对比
        """
        self.metrics = metrics or ["bleu", "rouge_l", "similarity"]

        unsupported = [m for m in self.metrics if m not in METRIC_FUNCTIONS]
        if unsupported:
            raise DataValidationError(
                f"不支持的评估指标: {unsupported}。支持: {SUPPORTED_METRICS}"
            )

        self.model_factory = model_factory

    def evaluate_quality(self, generated: str, reference: str) -> float:
        """评估单条生成文本的质量（各指标均值）

        Args:
            generated: 生成文本
            reference: 参考文本

        Returns:
            综合质量分 (0-1)
        """
        scores = [
            METRIC_FUNCTIONS[name](generated, reference)
            for name in self.metrics
        ]
        return sum(scores) / len(scores) if scores else 0.0

    def evaluate_pair(self, generated: str, reference: str) -> Dict[str, float]:
        """计算单条样本的全部指标

        Args:
            generated: 生成文本
            reference: 参考文本

        Returns:
            指标名到分数的映射
        """
        return {
            name: METRIC_FUNCTIONS[name](generated, reference)
            for name in self.metrics
        }

    def evaluate_batch(self,
                       generated: List[str],
                       references: List[str]) -> EvaluationResult:
        """批量评估

        Args:
            generated: 生成文本列表
            references: 参考文本列表

        Returns:
            EvaluationResult 实例

        Raises:
            DataValidationError: 两个列表长度不一致
        """
        if len(generated) != len(references):
            raise DataValidationError("generated 与 references 长度不一致")

        if not generated:
            return EvaluationResult(metrics={m: 0.0 for m in self.metrics})

        details = []
        totals = {m: 0.0 for m in self.metrics}

        for index, (gen, ref) in enumerate(zip(generated, references)):
            pair_scores = self.evaluate_pair(gen, ref)
            for name, score in pair_scores.items():
                totals[name] += score
            details.append({
                "index": index,
                "generated": gen[:100],
                "reference": ref[:100],
                "scores": pair_scores
            })

        averages = {name: total / len(generated) for name, total in totals.items()}

        return EvaluationResult(
            metrics=averages,
            sample_count=len(generated),
            details=details
        )

    def compare_outputs(self,
                        outputs_a: List[str],
                        outputs_b: List[str],
                        references: List[str]) -> Dict[str, Any]:
        """对比两组输出的质量

        Args:
            outputs_a: 模型 A 的输出
            outputs_b: 模型 B 的输出
            references: 参考文本

        Returns:
            对比结果
        """
        result_a = self.evaluate_batch(outputs_a, references)
        result_b = self.evaluate_batch(outputs_b, references)

        wins = {"a": 0, "b": 0, "tie": 0}
        for index, reference in enumerate(references):
            score_a = self.evaluate_quality(outputs_a[index], reference)
            score_b = self.evaluate_quality(outputs_b[index], reference)
            if abs(score_a - score_b) < 1e-9:
                wins["tie"] += 1
            elif score_a > score_b:
                wins["a"] += 1
            else:
                wins["b"] += 1

        return {
            "model_a": result_a.to_dict(),
            "model_b": result_b.to_dict(),
            "win_counts": wins,
            "winner": "a" if wins["a"] > wins["b"] else (
                "b" if wins["b"] > wins["a"] else "tie"
            )
        }

    def compare_models(self,
                       model_a: Any,
                       model_b: Any,
                       prompts: List[str],
                       references: Optional[List[str]] = None) -> Dict[str, Any]:
        """对比两个模型后端

        Args:
            model_a: 模型 A 的后端实例或名称
            model_b: 模型 B 的后端实例或名称
            prompts: 测试提示列表
            references: 参考文本列表，为 None 时用空字符串占位（仅做生成对比）

        Returns:
            对比结果
        """
        backend_a = self._resolve_backend(model_a)
        backend_b = self._resolve_backend(model_b)

        outputs_a = [backend_a.generate(p) for p in prompts]
        outputs_b = [backend_b.generate(p) for p in prompts]
        refs = references or [""] * len(prompts)

        if references:
            return self.compare_outputs(outputs_a, outputs_b, refs)

        return {
            "model_a_outputs": outputs_a,
            "model_b_outputs": outputs_b,
            "reference_provided": False
        }

    def _resolve_backend(self, model: Any) -> Any:
        """解析模型后端

        Args:
            model: 后端实例或模型名称

        Returns:
            模型后端实例

        Raises:
            DataValidationError: 名称为字符串但未提供工厂函数
        """
        if not isinstance(model, str):
            return model

        if self.model_factory is None:
            raise DataValidationError(
                f"模型 '{model}' 为字符串时需要通过 model_factory 解析"
            )

        return self.model_factory(model)
