"""质量评估 API 路由"""

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


@router.post("/api/quality/evaluate")
async def evaluate_quality(request: QualityRequest):
    """质量评估"""
    try:
        items = load_items(request.input_file)

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


@router.post("/api/quality/dedup")
async def deduplicate(request: DedupRequest):
    """智能去重"""
    try:
        items = load_items(request.input_file)

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


@router.post("/api/quality/report")
async def generate_report(request: QualityRequest):
    """生成质量报告"""
    try:
        items = load_items(request.input_file)

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


@router.post("/api/quality/clean")
async def clean_data(request: QualityRequest):
    """数据清洗"""
    try:
        items = load_items(request.input_file)

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


@router.post("/api/quality/annotate")
async def annotate_data(request: QualityRequest):
    """自动标注"""
    try:
        items = load_items(request.input_file)

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


@router.post("/api/quality/benchmark")
async def run_benchmark(request: QualityRequest):
    """运行数据质量基准"""
    try:
        items = load_items(request.input_file)

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


@router.post("/api/quality/outliers")
async def detect_outliers_endpoint(request: OutlierRequest):
    """检测长度异常样本"""
    try:
        raw_items = load_items(request.input_file)

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


@router.post("/api/quality/profiling")
async def profile_dataset(
    request: ProfilingRequest, _auth: None = Depends(verify_api_key)
):
    """生成数据集画像"""
    try:
        items = load_items(request.input_file)
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
