"""质量评估 API 路由"""

from fastapi import APIRouter, HTTPException
from pydantic import BaseModel

from ..deps import load_items, run_in_thread

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

    Args:
        items: 原始数据列表

    Returns:
        评分输入列表
    """
    return [
        {
            "original": item.get("instruction", ""),
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
            scores = scorer.batch_score(_build_scoring_items(items))
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
                "threshold": request.threshold
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
            scores = scorer.batch_score(_build_scoring_items(items))
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
