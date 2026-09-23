# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""质量评估 API 路由"""

from typing import Any, Dict, List

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from ..deps import load_items, resolve_data_path, run_in_thread, verify_api_key

router = APIRouter(tags=["quality"])


class QualityRequest(BaseModel):
    """质量评估请求"""
    input_file: str
    threshold: float = 0.6


class DedupRequest(BaseModel):
    """去重请求"""
    input_file: str
    threshold: float = 0.9


class QualityEvaluateResponse(BaseModel):
    """质量评分响应

    `effective_weights` 是**重归一化后**的权重（API 拿不到原始种子问题，
    语义维度不参与评分），返回它才能让调用方自己复算总分。
    """
    total_samples: int
    passed_samples: int
    filtered_samples: int
    pass_rate: float
    avg_score: float
    threshold: float
    semantic_evaluated: bool
    effective_weights: List[float]


class DedupResponse(BaseModel):
    """去重响应

    `duplicate_groups` 是**重复组数量**（不是 `DedupResult.duplicate_groups`
    那个 `List[List[int]]`），`kept_indices` 才是保留条目的下标。
    """
    original_count: int
    deduplicated_count: int
    removed_count: int
    duplicate_groups: int
    kept_indices: List[int]


class QualityReportResponse(BaseModel):
    """质量报告响应（与 `QualityReport.to_dict()` 的键一一对应）"""
    total_samples: int
    passed_samples: int
    filtered_samples: int
    pass_rate: float
    score_distribution: Dict[str, float]
    filter_statistics: Dict[str, int]
    metric_summary: Dict[str, Dict[str, float]]
    dedup_summary: Dict[str, Any]
    improvement_suggestions: List[str]
    charts: Dict[str, Any]


class CleanResponse(BaseModel):
    """清洗响应（与 `data.CleanResult` 的字段一一对应）

    注意这里叫 `dropped_count`，与 CLI `clean` 的 `removed_count` 不同名：
    两者由不同的清洗器产生（`data.DataCleaner` 与 `cleaner.DataCleaner`），
    键名差异是既有契约，不在此处统一。
    """
    original_count: int
    cleaned_count: int
    dropped_count: int
    changed_count: int
    language_distribution: Dict[str, int]
    issues: Dict[str, int]


class AnnotateResponse(BaseModel):
    """自动标注响应（与 `AutoAnnotator.generate_report()` 的返回键一致）"""
    total_items: int
    entity_count: int
    avg_entities_per_item: float
    intent_distribution: Dict[str, int]
    sentiment_distribution: Dict[str, int]
    text_key: str


class BenchmarkResponse(BaseModel):
    """质量基准响应（与 `QualityBenchmark.run_benchmark()` 的返回键一致）

    `metrics` 只含配置里声明的指标，`all_metrics` 含全部已计算指标。
    """
    sample_count: int
    metrics: Dict[str, float]
    all_metrics: Dict[str, float]
    threshold: float
    timestamp: str


def _build_scoring_items(items):
    """构造质量评分输入

    API 拿到的是一份已落盘的数据集，条目只有 instruction / output，
    **不存在原始种子问题**，因此无法计算语义相似度。

    早期实现把 instruction 同时填入 original 与 generated，使语义相似度恒为 1.0
    （占 0.3 权重），导致一条答案与问题完全无关的样本也能拿到 0.6 分并通过默认阈值。
    现改为不伪造 original，由调用方以 include_semantic=False 跳过该维度。

    Args:
        items: 原始数据列表

    Returns:
        评分输入列表（仅含 generated 与 output）
    """
    return [
        {
            "generated": item.get("instruction", ""),
            "output": item.get("output", "")
        }
        for item in items
    ]


@router.post(
    "/api/quality/evaluate",
    response_model=QualityEvaluateResponse,
    summary="质量评分",
)
async def evaluate_quality(request: QualityRequest):
    """质量评估"""
    try:
        items = await run_in_thread(load_items, request.input_file)

        def evaluate():
            from augmentor.quality import QualityScorer

            scorer = QualityScorer(threshold=request.threshold)
            # 无原始种子问题 → 跳过语义维度，权重在相关性与多样性上重归一化
            scores = scorer.batch_score(
                _build_scoring_items(items), include_semantic=False
            )
            total_scores = [s.total_score for s in scores]
            passed = sum(1 for s in scores if s.passed)

            return {
                "total_samples": len(items),
                "passed_samples": passed,
                "filtered_samples": len(items) - passed,
                "pass_rate": passed / len(items) if items else 0.0,
                "avg_score": (
                    sum(total_scores) / len(total_scores) if total_scores else 0.0
                ),
                "threshold": request.threshold,
                # 语义维度未参与评分：数据集条目不含原始种子问题
                "semantic_evaluated": False,
                "effective_weights": scorer.effective_weights(False)
            }

        return await run_in_thread(evaluate)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/quality/dedup",
    response_model=DedupResponse,
    summary="智能去重",
)
async def deduplicate(request: DedupRequest):
    """智能去重"""
    try:
        items = await run_in_thread(load_items, request.input_file)

        def dedup():
            from augmentor.dedup import Deduplicator

            deduplicator = Deduplicator(threshold=request.threshold)
            result = deduplicator.deduplicate(items)
            return {
                "original_count": result.original_count,
                "deduplicated_count": result.deduplicated_count,
                "removed_count": result.removed_count,
                "duplicate_groups": len(result.duplicate_groups),
                "kept_indices": result.kept_indices
            }

        return await run_in_thread(dedup)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/quality/report",
    response_model=QualityReportResponse,
    summary="生成质量报告",
)
async def generate_report(request: QualityRequest):
    """生成质量报告"""
    try:
        items = await run_in_thread(load_items, request.input_file)

        def build_report():
            from augmentor.quality import QualityScorer
            from augmentor.dedup import Deduplicator
            from augmentor.report import ReportGenerator

            scorer = QualityScorer(threshold=request.threshold)
            # 同 evaluate：数据集不含原始种子问题，跳过语义维度
            scores = scorer.batch_score(
                _build_scoring_items(items), include_semantic=False
            )
            dedup_summary = Deduplicator().generate_report(items)

            report = ReportGenerator(threshold=request.threshold).generate(
                items, scores, dedup_summary
            )
            return report.to_dict()

        return await run_in_thread(build_report)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/quality/clean",
    response_model=CleanResponse,
    summary="数据清洗",
)
async def clean_data(request: QualityRequest):
    """数据清洗"""
    try:
        items = await run_in_thread(load_items, request.input_file)

        def clean():
            from augmentor.data import DataCleaner

            result = DataCleaner().clean(items)
            return {
                "original_count": result.original_count,
                "cleaned_count": result.cleaned_count,
                "dropped_count": result.dropped_count,
                "changed_count": result.changed_count,
                "language_distribution": result.language_distribution,
                "issues": result.issues
            }

        return await run_in_thread(clean)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/quality/annotate",
    response_model=AnnotateResponse,
    summary="自动标注",
)
async def annotate_data(request: QualityRequest):
    """自动标注"""
    try:
        items = await run_in_thread(load_items, request.input_file)

        def annotate():
            from augmentor.data import AutoAnnotator

            return AutoAnnotator().generate_report(items)

        return await run_in_thread(annotate)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post(
    "/api/quality/benchmark",
    response_model=BenchmarkResponse,
    summary="运行数据质量基准",
)
async def run_benchmark(request: QualityRequest):
    """运行数据质量基准"""
    try:
        items = await run_in_thread(load_items, request.input_file)

        def benchmark():
            from augmentor.benchmark import QualityBenchmark

            bench = QualityBenchmark(threshold=request.threshold)
            return bench.run_benchmark(items)

        return await run_in_thread(benchmark)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


class OutlierRequest(BaseModel):
    """异常值检测请求"""
    input_file: str
    method: str = "zscore"
    threshold: float = 3.0
    field: str = "length"


class ProfilingRequest(BaseModel):
    """数据画像请求"""
    input_file: str
    save: bool = False
    output_path: str = ""


class OutlierResponse(BaseModel):
    """离群点检测响应

    `outlier_count` / `outlier_rate` 是 `OutlierReport` 上的**计算属性**而非字段，
    必须显式列出，否则会被 response_model 静默裁掉。
    """
    total_items: int
    outlier_count: int
    outlier_rate: float
    method: str
    threshold: float
    field: str
    outliers: List[Dict[str, Any]]


class ProfilingResponse(BaseModel):
    """数据画像响应（与 `DataProfiler.profile()` 的返回键一致）"""
    total_items: int
    field_completeness: Dict[str, Any]
    length_stats: Dict[str, Any]
    duplicate_rate: float
    language_distribution: Dict[str, Any]
    top_keywords: List[Any]


@router.post("/api/quality/outliers", response_model=OutlierResponse, summary="长度离群点检测")
async def detect_outliers_endpoint(request: OutlierRequest):
    """检测长度异常样本"""
    try:
        raw_items = await run_in_thread(load_items, request.input_file)

        def detect():
            from augmentor.outlier import OutlierDetector

            detector = OutlierDetector(
                method=request.method,
                threshold=request.threshold,
                field=request.field,
            )
            # 若指定 length 字段，则基于 instruction 长度现场计算
            working_items = detector.attach_length_field(raw_items) \
                if request.field == "length" else raw_items
            report = detector.detect(working_items)
            return report.to_dict()

        return await run_in_thread(detect)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/api/quality/profiling", response_model=ProfilingResponse, summary="数据集画像")
async def profile_dataset(
    request: ProfilingRequest, _auth: None = Depends(verify_api_key)
):
    """生成数据集画像"""
    try:
        items = await run_in_thread(load_items, request.input_file)
        # 落盘路径同样受白名单约束
        output_path = (
            resolve_data_path(request.output_path, for_write=True)
            if request.save and request.output_path else None
        )

        def profile():
            from augmentor.profiling import DataProfiler

            profiler = DataProfiler()
            report = profiler.profile(items)
            if output_path is not None:
                profiler.save_profile(items, str(output_path))
            return report

        return await run_in_thread(profile)
    except FileNotFoundError:
        raise HTTPException(status_code=404, detail="文件不存在")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))
