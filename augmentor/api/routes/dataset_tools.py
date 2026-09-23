# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""数据集工具 API 路由

补齐 CLI 里与「数据集本身的读 / 写 / 变换」相关的独有能力：统计、验证、
格式转换、合并、采样、分割、搜索、对比、特征检测、聚合、自动配置推荐、
RAG 格式转换。此前这些能力只能通过命令行使用。

**分组约定**（与 CLI 的语义一致，不是随手定的）：

* **只读分析类**（stats / validate / search / compare / features /
  auto-config）把完整结果直接返回——结果本身就是调用方要的东西；
* **写盘变换类**（convert / merge / sample / split / aggregate / rag）
  必须给输出路径，响应只回传「写到哪、写了多少」。产物规模与输入同量级，
  塞进 HTTP 响应既慢又容易被客户端或网关截断。
  这 6 条**挂 `verify_api_key`**（与 `/api/data/export` 同一策略）：它们会往
  白名单内的路径写文件，属于写操作；只读分析类不挂，保持零配置可用。

所有路径都经 `deps.resolve_data_path` / `deps.resolve_data_dir` 做白名单校验，
与其它路由一致；越界返回 403，参数非法返回 400。

**异步约定**：这些路由是 `async def`，所以读文件一律 `await read_json_file(path)`
（`read_items` 的异步外壳），重活一律 `await run_in_thread(...)`。直接在函数体里
调同步版本 = 让整个服务在那几十毫秒内无法响应任何其他请求，见 A4 / A13。
"""

import json
from pathlib import Path
from typing import Any, Dict, List, Optional

from fastapi import APIRouter, Depends, HTTPException
from pydantic import BaseModel

from ..deps import (
    read_json_file,
    resolve_data_dir,
    resolve_data_path,
    run_in_thread,
    to_http_error,
    verify_api_key,
)

router = APIRouter(tags=["dataset"])

# ============ 枚举白名单 ============
#
# 这些取值与 CLI 的 `choices=` 一一对应（parser.py）。必须在这里显式校验，
# 因为下游库对未知值**静默降级**：`DatasetValidator` 回退 basic、
# `EnhancedSearcher` 回退 contains。静默降级比报错危险得多——调用方会以为
# 「strict 校验通过了」「fuzzy 搜不到」，而实际用的是另一个规则/方法。
#
# 状态码用 400 而非 422：与本项目既有约定一致（`/api/quality/outliers` 的
# 未知 method 也是 400，见 tests/integration/test_api.py）。
VALIDATION_PRESETS = ("basic", "strict", "chat")
SEARCH_METHODS = ("exact", "contains", "ngram", "fuzzy", "regex")


# ============ 请求模型 ============

class DatasetFileRequest(BaseModel):
    """只针对一个数据集文件的请求"""
    input_file: str


class StatsRequest(BaseModel):
    """统计请求"""
    input_file: str
    fields: Optional[List[str]] = None


class ValidateRequest(BaseModel):
    """数据集验证请求

    `preset` 默认 `basic`，与 CLI 的 `--preset` 默认值一致。
    （此前这里写的是 `"default"` —— 那个名字**根本不存在**于
    `DatasetValidator.PRESET_RULES`，于是默认请求也会静默回退到 basic。）
    """
    input_file: str
    preset: str = "basic"


class ConvertRequest(BaseModel):
    """格式转换请求"""
    input_file: str
    output_file: str
    target_format: str = "jsonl"


class MergeRequest(BaseModel):
    """合并请求"""
    inputs: List[str]
    output_file: str
    deduplicate: bool = True


class SampleRequest(BaseModel):
    """采样请求

    `size` 与 `ratio` 二选一；都给时以 `size` 为准（`SampleConfig` 的既有语义）。
    """
    input_file: str
    output_file: str
    method: str = "random"
    size: Optional[int] = None
    ratio: Optional[float] = None
    seed: Optional[int] = None


class SplitRequest(BaseModel):
    """分割请求

    三个比例之和必须为 1，否则 `SplitConfig` 会拒绝。
    """
    input_file: str
    output_dir: str
    train_ratio: float = 0.8
    val_ratio: float = 0.1
    test_ratio: float = 0.1
    seed: Optional[int] = None


class SearchRequest(BaseModel):
    """搜索请求

    `fields` 为 None 时搜索全部字段；`method` 支持
    exact / contains / ngram / fuzzy / regex。
    """
    input_file: str
    query: str
    fields: Optional[List[str]] = None
    method: str = "contains"
    limit: int = 100
    offset: int = 0


class CompareRequest(BaseModel):
    """数据集对比请求

    展示名缺省取文件名 stem。
    """
    dataset_a: str
    dataset_b: str
    name_a: Optional[str] = None
    name_b: Optional[str] = None
    key_fields: Optional[List[str]] = None


class AggregateRequest(BaseModel):
    """聚合请求

    `datasets` 是「来源名 → 数据集文件」的映射；`strategy` 支持
    union / intersection / weighted / consistent。
    """
    datasets: Dict[str, str]
    output_file: str
    strategy: str = "union"
    target_size: Optional[int] = None


class RagRequest(BaseModel):
    """RAG 格式转换请求"""
    input_file: str
    output_file: str
    format: str = "langchain"


# ============ 响应模型 ============

class StatisticsResponse(BaseModel):
    """数据集统计（与 `DatasetStatistics.to_dict()` 的键一一对应）

    `summary` 是**渲染好的多行文本**而非结构化数据，`field_statistics` 才是
    逐字段的结构化结果——两者是「给人看」与「给程序用」的两份表达。
    """
    dataset_name: str
    total_items: int
    summary: str
    field_statistics: Dict[str, Any]
    content_statistics: Dict[str, Any]
    quality_metrics: Dict[str, float]


class ValidationResponse(BaseModel):
    """数据集验证结果（与 `ValidationResult.to_dict()` 的键一一对应）"""
    is_valid: bool
    total_items: int
    valid_items: int
    error_count: int
    warning_count: int
    issues: List[Dict[str, Any]]


class ConvertResponse(BaseModel):
    """格式转换结果（与 `converter.convert_file()` 的返回键一一对应）"""
    input_file: str
    output_file: str
    source_format: str
    target_format: str
    input_count: int
    output_count: int


class MergeResponse(BaseModel):
    """合并结果（与 `DatasetOperations.merge_files()` 的返回键一一对应）"""
    input_files: int
    output_file: str
    total_input: int
    total_output: int
    removed_duplicates: int


class SampleResponse(BaseModel):
    """采样结果（与 `DatasetOperations.sample_file()` 的返回键一一对应）"""
    input_file: str
    output_file: str
    input_count: int
    output_count: int


class SplitPart(BaseModel):
    """分割产物的一支"""
    file: str
    count: int


class SplitResponse(BaseModel):
    """分割结果（与 `DatasetOperations.split_file()` 的返回键一一对应）"""
    input_file: str
    output_dir: str
    splits: Dict[str, SplitPart]


class SearchResponse(BaseModel):
    """搜索结果（与 `SearchResult.to_dict()` 的键一一对应）"""
    query: str
    method: str
    total_matches: int
    query_time_ms: float
    items: List[Dict[str, Any]]
    highlights: List[Dict[str, Any]]


class CompareResponse(BaseModel):
    """数据集对比结果：两种互补的结论

    `quality_comparison` 来自 `comparison.DatasetComparator`（质量 / 通过率 /
    长度 / 词汇量与获胜方），`overlap_comparison` 来自
    `compare_enhanced.compare_datasets_enhanced`（重叠度 / 字段级差异 /
    改进建议）。任一单独返回都会丢一半结论，因此两者都在。
    """
    quality_comparison: Dict[str, Any]
    overlap_comparison: Dict[str, Any]


class FieldFeature(BaseModel):
    """单个字段的特征（与 `FeatureInfo.to_dict()` 的键一一对应）"""
    name: str
    feature_type: str
    coverage: float
    unique_ratio: float
    description: str


class FeaturesResponse(BaseModel):
    """特征检测结果（与 `FeatureDetector.detect()` 的返回键一一对应）"""
    total_items: int
    field_features: List[FieldFeature]
    sparse_fields: List[str]
    intent_distribution: Dict[str, int]


class AggregateResponse(BaseModel):
    """聚合结果：产物路径与汇总计数"""
    output_file: str
    aggregated_count: int
    source_counts: Dict[str, int]
    removed_duplicates: int
    conflicts: List[Dict[str, Any]]


class AutoConfigResponse(BaseModel):
    """自动配置推荐（与 `AutoConfigRecommendation.to_dict()` 的键一一对应）"""
    quality_threshold: float
    dedup_threshold: float
    recommended_sample_size: int
    reasoning: List[str]
    source_stats: Dict[str, Any]


class RagResponse(BaseModel):
    """RAG 格式转换结果：产物路径与条数"""
    output_file: str
    format: str
    input_count: int
    record_count: int


def _dump(items: Any, path: Path) -> None:
    """写入 JSON 数据集文件"""
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(items, f, ensure_ascii=False, indent=2)


# ============ 只读分析 ============

@router.post("/api/dataset/stats", response_model=StatisticsResponse, summary="数据集统计")
async def dataset_stats(request: StatsRequest):
    """逐字段统计（填充率 / 长度 / 唯一值）与整体质量指标"""
    try:
        from augmentor.statistics import calculate_statistics

        path = resolve_data_path(request.input_file)
        items = await read_json_file(path)
        stats = await run_in_thread(
            calculate_statistics, items, Path(path).stem, request.fields
        )
        return stats.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/validate", response_model=ValidationResponse, summary="数据集验证")
async def dataset_validate(request: ValidateRequest):
    """按预设规则验证数据集，返回全部问题清单"""
    if request.preset not in VALIDATION_PRESETS:
        raise HTTPException(
            status_code=400,
            detail=(
                f"未知的验证预设: {request.preset}"
                f"（可选 {' / '.join(VALIDATION_PRESETS)}）"
            ),
        )
    try:
        from augmentor.validation import DatasetValidator

        path = resolve_data_path(request.input_file)
        result = await run_in_thread(
            lambda: DatasetValidator(preset=request.preset).validate_file(str(path))
        )
        return result.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/search", response_model=SearchResponse, summary="数据集搜索")
async def dataset_search(request: SearchRequest):
    """按 exact / contains / ngram / fuzzy / regex 搜索数据集条目"""
    if request.method not in SEARCH_METHODS:
        raise HTTPException(
            status_code=400,
            detail=(
                f"未知的搜索方法: {request.method}"
                f"（可选 {' / '.join(SEARCH_METHODS)}）"
            ),
        )
    try:
        from augmentor.search_enhanced import search_dataset

        path = resolve_data_path(request.input_file)
        items = await read_json_file(path)
        result = await run_in_thread(
            search_dataset,
            items,
            request.query,
            request.fields,
            request.method,
            request.limit,
            request.offset,
        )
        return result.to_dict()
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/compare", response_model=CompareResponse, summary="数据集对比")
async def dataset_compare(request: CompareRequest):
    """同时给出质量向与重叠度两种互补的对比结论"""
    try:
        from augmentor.comparison import DatasetComparator
        from augmentor.compare_enhanced import compare_datasets_enhanced

        path_a = resolve_data_path(request.dataset_a)
        path_b = resolve_data_path(request.dataset_b)
        items_a = await read_json_file(path_a)
        items_b = await read_json_file(path_b)

        name_a = request.name_a or Path(path_a).stem
        name_b = request.name_b or Path(path_b).stem

        def run():
            quality = DatasetComparator().compare(items_a, items_b, name_a, name_b)
            overlap = compare_datasets_enhanced(
                items_a, items_b, name_a, name_b, request.key_fields
            )
            return {
                "quality_comparison": quality.to_dict(),
                "overlap_comparison": overlap.to_dict(),
            }

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/features", response_model=FeaturesResponse, summary="字段特征检测")
async def dataset_features(request: DatasetFileRequest):
    """检测字段覆盖度、稀疏字段与意图分布"""
    try:
        from augmentor.feature_detect import FeatureDetector

        path = resolve_data_path(request.input_file)
        items = await read_json_file(path)
        return await run_in_thread(lambda: FeatureDetector().detect(items))
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post(
    "/api/dataset/auto-config",
    response_model=AutoConfigResponse,
    summary="自动配置推荐",
)
async def dataset_auto_config(request: DatasetFileRequest):
    """基于数据集画像推荐质量阈值、去重阈值与采样量"""
    try:
        from augmentor.auto_config import AutoConfig
        from augmentor.profiling import DataProfiler

        path = resolve_data_path(request.input_file)
        items = await read_json_file(path)

        def run():
            profile = DataProfiler().profile(items)
            recommendation = AutoConfig().recommend(profile, len(items))
            return recommendation.to_dict()

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


# ============ 写盘变换 ============

@router.post("/api/dataset/convert", response_model=ConvertResponse,
             summary="格式转换", dependencies=[Depends(verify_api_key)])
async def dataset_convert(request: ConvertRequest):
    """把数据集转换为另一种格式并落盘"""
    try:
        from augmentor.converter import convert_file

        input_path = resolve_data_path(request.input_file)
        output_path = resolve_data_path(request.output_file, for_write=True)
        return await run_in_thread(
            convert_file, str(input_path), str(output_path), request.target_format
        )
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/merge", response_model=MergeResponse, summary="合并数据集",
             dependencies=[Depends(verify_api_key)])
async def dataset_merge(request: MergeRequest):
    """把多个数据集合并为一份，可选去重"""
    try:
        from augmentor.dataset_ops import DatasetOperations, MergeConfig

        if not request.inputs:
            raise HTTPException(status_code=400, detail="inputs 不能为空")

        input_paths = [str(resolve_data_path(name)) for name in request.inputs]
        output_path = resolve_data_path(request.output_file, for_write=True)

        def run():
            ops = DatasetOperations()
            return ops.merge_files(
                input_paths,
                str(output_path),
                MergeConfig(deduplicate=request.deduplicate),
            )

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/sample", response_model=SampleResponse, summary="数据集采样",
             dependencies=[Depends(verify_api_key)])
async def dataset_sample(request: SampleRequest):
    """按数量或比例采样并落盘"""
    try:
        from augmentor.dataset_ops import DatasetOperations, SampleConfig

        input_path = resolve_data_path(request.input_file)
        output_path = resolve_data_path(request.output_file, for_write=True)

        def run():
            ops = DatasetOperations()
            return ops.sample_file(
                str(input_path),
                str(output_path),
                SampleConfig(
                    method=request.method,
                    size=request.size,
                    ratio=request.ratio,
                    seed=request.seed,
                ),
            )

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/split", response_model=SplitResponse, summary="分割数据集",
             dependencies=[Depends(verify_api_key)])
async def dataset_split(request: SplitRequest):
    """按比例把数据集切成 train / val / test 三份"""
    try:
        from augmentor.dataset_ops import DatasetOperations, SplitConfig

        input_path = resolve_data_path(request.input_file)
        output_dir = resolve_data_dir(request.output_dir)

        def run():
            ops = DatasetOperations()
            return ops.split_file(
                str(input_path),
                str(output_dir),
                SplitConfig(
                    ratios=(request.train_ratio, request.val_ratio, request.test_ratio),
                    seed=request.seed,
                ),
            )

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/aggregate", response_model=AggregateResponse,
             summary="多源聚合", dependencies=[Depends(verify_api_key)])
async def dataset_aggregate(request: AggregateRequest):
    """按 union / intersection / weighted / consistent 策略聚合多个数据源"""
    try:
        from augmentor.aggregator import DataAggregator

        if not request.datasets:
            raise HTTPException(status_code=400, detail="datasets 不能为空")

        # 逐个 await 而不是循环里同步 read：保持原有的「按 datasets 顺序解析、
        # 第一个坏文件先报错」语义，同时不再占着事件循环。
        datasets: Dict[str, list] = {}
        for name, source in request.datasets.items():
            datasets[name] = await read_json_file(resolve_data_path(source))
        output_path = resolve_data_path(request.output_file, for_write=True)

        def run():
            result = DataAggregator().aggregate(
                datasets, request.strategy, target_size=request.target_size
            )
            _dump(result.aggregated, output_path)
            return {
                "output_file": str(output_path),
                "aggregated_count": result.total_count,
                "source_counts": result.source_counts,
                "removed_duplicates": result.removed_duplicates,
                "conflicts": result.conflicts,
            }

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e


@router.post("/api/dataset/rag", response_model=RagResponse, summary="转换为 RAG 格式",
             dependencies=[Depends(verify_api_key)])
async def dataset_rag(request: RagRequest):
    """把数据集转换为 RAG 检索记录（langchain / llama_index 等）并落盘"""
    try:
        from augmentor.rag import RAGFormatter

        input_path = resolve_data_path(request.input_file)
        output_path = resolve_data_path(request.output_file, for_write=True)
        items = await read_json_file(input_path)

        def run():
            from ..deps import get_pipeline

            rag_config = get_pipeline().config.rag
            records = RAGFormatter(
                chunk_size=rag_config.chunk_size,
                chunk_overlap=rag_config.chunk_overlap,
            ).format(items, request.format)
            _dump(records, output_path)
            return {
                "output_file": str(output_path),
                "format": request.format,
                "input_count": len(items),
                "record_count": len(records),
            }

        return await run_in_thread(run)
    except HTTPException:
        raise
    except Exception as e:
        raise to_http_error(e) from e

