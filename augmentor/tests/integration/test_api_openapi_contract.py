# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""OpenAPI 契约测试

三类守门：

1. **元数据完整**：`/docs` 与 `/openapi.json` 必须带能力说明、分组描述、
   联系与许可信息，且版本号只有一个来源（`augmentor.__version__`）。
   此前 FastAPI 只设了 title/version，11 个 tag 在 Swagger UI 里是无标题的裸列表。

2. **`response_model` 不得静默裁剪字段**：FastAPI 会按模型字段过滤返回值——
   模型少写一个键，该字段就从响应里消失，**且不报错**。本项目的报告类普遍用
   `@property` 暴露计算字段（`LeakageReport.total_leaks`、`OutlierReport.outlier_count`
   等），直接拿 dataclass 当 response_model 恰好会踩中这个坑。

   注意这里的预言机选择：断言必须拿**领域契约**（handler 实际返回的
   `to_dict()` 键集合）与响应比对，而**不能**拿 `response_model` 的字段集比对。
   后者与被测对象同源——模型漏声明字段时响应和字段集一起变小，断言恒真。

3. **没有 response_model 的端点必须为零**：未声明契约的端点，在 OpenAPI 里
   响应 schema 是空对象，生成客户端只能拿到 `any`。本模块用 `CALLS` 的键集
   与「探测到的已声明端点集」互相约束，任何一侧漏掉都会失败。
"""

import dataclasses
import json
from pathlib import Path
from typing import Any, Callable, FrozenSet, Mapping

import pytest

AI_DIR = Path(__file__).resolve().parent.parent.parent

SAMPLE_ITEMS = [
    {"instruction": "如何申请租房？", "input": "", "output": "登录官网申请"},
    {"instruction": "租房多少钱？", "input": "", "output": "按房型定价"},
    {"instruction": "联系 zhang@example.com", "input": "", "output": "电话 13800138000"},
]


def _keys(*names: str) -> FrozenSet[str]:
    """把键名列表包成 frozenset（只为可读性）"""
    return frozenset(names)


@dataclasses.dataclass(frozen=True)
class _Case:
    """一个端点的契约用例

    Attributes:
        request: 真实调用（接收 TestClient）
        keys: **领域契约**要求的响应键——即 handler 实际返回的键，
            对各报告类就是其 `to_dict()` 的键集合（含计算属性）。
            当 `is_list` 为真时，指的是**数组元素**的键。
        nested: 需要递归校验的子对象，形如 `{响应键: 该子对象的契约键}`
        is_list: 200 响应是裸数组（如 `/api/demo/data`）而非对象
    """

    request: Callable[[Any], Any]
    keys: FrozenSet[str]
    nested: Mapping[str, FrozenSet[str]] = dataclasses.field(default_factory=dict)
    is_list: bool = False


# ============ 版本类端点的前置动作 ============
#
# 版本端点的 URL 里带 version_id，必须先建出来。这里刻意不复用 `_Case` 之外的
# fixture：契约用例只有一个 `request(client)` 入口，前置动作写在函数里，
# 失败时断言信息直接落在该端点的用例上。


def _create_version(client, filename: str, label: str = "") -> str:
    """建一个版本并返回其 ID"""
    response = client.post(
        "/api/versions/create",
        params={"filename": filename},
        json={"label": label} if label else {},
    )
    assert response.status_code == 200, response.text
    return response.json()["version_id"]


def _fresh_version(client, filename: str = "data.json") -> str:
    """建一个版本并让它成为**当前**版本"""
    return _create_version(client, filename)


def _stale_version(client) -> str:
    """建两个版本，返回那个**已不是当前版本**的 ID

    `delete_version` 拒绝删除当前版本（返回 500），所以要删得先让位。
    """
    stale = _create_version(client, "data.json")
    _create_version(client, "train.json", label="second")
    return stale


def _validate_written_config(client):
    """先让服务写一份 config.yaml，再校验它

    校验的必须是「服务自己写出的配置」，而不是仓库里那份手写的——后者在测试
    的临时 CWD 里并不存在。这也正是这条用例的价值：它把「保存」与「校验」
    串成闭环，任何一侧与另一侧不一致都会失败。
    """
    written = client.post("/api/config", json={})
    assert written.status_code == 200, written.text
    return client.post("/api/system/validate-config", json={"path": "config.yaml"})


def _create_backup(client, name: str = "snap") -> str:
    """建一个备份并返回其 ID"""
    response = client.post(
        "/api/system/backups", json={"input_file": "data.json", "name": name}
    )
    assert response.status_code == 200, response.text
    return response.json()["backup_id"]


def _restore_backup(client):
    """建一个备份再恢复它（备份 ID 只能现场产生）"""
    backup_id = _create_backup(client)
    return client.post(
        f"/api/system/backups/{backup_id}/restore", json={"output_file": "restored.json"}
    )


def _delete_backup(client):
    """建一个备份再删掉它"""
    backup_id = _create_backup(client)
    return client.delete(f"/api/system/backups/{backup_id}")


# ============ 端点 → 契约用例 ============
#
# 每声明一个 response_model，就必须在这里补一条用例；由
# `test_all_modelled_endpoints_have_a_no_drop_case` 强制对齐。
CALLS = {
    # ---------- status ----------
    ("GET", "/api/health"): _Case(
        request=lambda c: c.get("/api/health"),
        keys=_keys("status", "version"),
    ),
    ("GET", "/api/status"): _Case(
        request=lambda c: c.get("/api/status"),
        keys=_keys("status", "version", "model_default", "model_available", "dependencies"),
    ),
    # ---------- privacy ----------
    ("GET", "/api/privacy/patterns"): _Case(
        request=lambda c: c.get("/api/privacy/patterns"),
        keys=_keys("default", "extra"),
    ),
    ("POST", "/api/privacy/sanitize"): _Case(
        request=lambda c: c.post(
            "/api/privacy/sanitize", json={"input_file": "data.json"}
        ),
        keys=_keys("items", "report"),
        # report 是嵌套对象：`SanitizeReport.total_matches` 是计算属性，
        # 漏声明时顶层键一个不少，只有递归下去才看得见。
        nested={"report": _keys("total_items", "touched_items", "matches", "total_matches")},
    ),
    # ---------- leakage / audit ----------
    ("POST", "/api/leakage/check"): _Case(
        request=lambda c: c.post(
            "/api/leakage/check",
            json={"train_file": "train.json", "test_file": "test.json"},
        ),
        # 与 `LeakageReport.to_dict()` 的键一一对应（后三个是计算属性）
        keys=_keys(
            "train_size", "test_size", "exact_leaks", "fuzzy_leaks",
            "total_leaks", "leak_rate", "is_clean", "leaked_examples",
        ),
    ),
    ("POST", "/api/audit"): _Case(
        request=lambda c: c.post("/api/audit", json={"input_file": "data.json"}),
        # 与 `AuditReport.to_dict()` 的键一一对应（该 dataclass 的字段与
        # to_dict 键恰好一致，因此可直接当 response_model）
        keys=_keys(
            "total_items", "pii_matches", "duplicate_count", "duplicate_rate",
            "empty_field_rate", "leak_count", "leak_rate", "findings", "ready",
        ),
    ),
    # ---------- quality ----------
    ("POST", "/api/quality/evaluate"): _Case(
        request=lambda c: c.post(
            "/api/quality/evaluate", json={"input_file": "data.json"}
        ),
        keys=_keys(
            "total_samples", "passed_samples", "filtered_samples", "pass_rate",
            "avg_score", "threshold", "semantic_evaluated", "effective_weights",
        ),
    ),
    ("POST", "/api/quality/dedup"): _Case(
        request=lambda c: c.post("/api/quality/dedup", json={"input_file": "data.json"}),
        # 注意 `duplicate_groups` 在这里是**组数**（int），不是
        # `DedupResult.duplicate_groups` 那个 List[List[int]]
        keys=_keys(
            "original_count", "deduplicated_count", "removed_count",
            "duplicate_groups", "kept_indices",
        ),
    ),
    ("POST", "/api/quality/report"): _Case(
        request=lambda c: c.post("/api/quality/report", json={"input_file": "data.json"}),
        # 与 `QualityReport.to_dict()` 的键一一对应
        keys=_keys(
            "total_samples", "passed_samples", "filtered_samples", "pass_rate",
            "score_distribution", "filter_statistics", "metric_summary",
            "dedup_summary", "improvement_suggestions", "charts",
        ),
    ),
    ("POST", "/api/quality/clean"): _Case(
        request=lambda c: c.post("/api/quality/clean", json={"input_file": "data.json"}),
        # 与 `data.CleanResult` 的字段一一对应。注意这里是 `dropped_count`，
        # 与 CLI `clean` 的 `removed_count` 不同名——两者出自不同的清洗器。
        keys=_keys(
            "original_count", "cleaned_count", "dropped_count", "changed_count",
            "language_distribution", "issues",
        ),
    ),
    ("POST", "/api/quality/annotate"): _Case(
        request=lambda c: c.post("/api/quality/annotate", json={"input_file": "data.json"}),
        # 与 `AutoAnnotator.generate_report()` 的返回键一一对应
        keys=_keys(
            "total_items", "entity_count", "avg_entities_per_item",
            "intent_distribution", "sentiment_distribution", "text_key",
        ),
    ),
    ("POST", "/api/quality/benchmark"): _Case(
        request=lambda c: c.post("/api/quality/benchmark", json={"input_file": "data.json"}),
        # 与 `QualityBenchmark.run_benchmark()` 的返回键一一对应
        keys=_keys("sample_count", "metrics", "all_metrics", "threshold", "timestamp"),
    ),
    ("POST", "/api/quality/outliers"): _Case(
        request=lambda c: c.post(
            "/api/quality/outliers", json={"input_file": "data.json"}
        ),
        # 与 `OutlierReport.to_dict()` 的键一一对应（outlier_count/rate 是计算属性）
        keys=_keys(
            "total_items", "outlier_count", "outlier_rate", "method",
            "threshold", "field", "outliers",
        ),
    ),
    ("POST", "/api/quality/profiling"): _Case(
        request=lambda c: c.post(
            "/api/quality/profiling", json={"input_file": "data.json"}
        ),
        # 与 `DataProfiler.profile()` 的返回键一致
        keys=_keys(
            "total_items", "field_completeness", "length_stats",
            "duplicate_rate", "language_distribution", "top_keywords",
        ),
    ),
    ("POST", "/api/quality/health-gate"): _Case(
        request=lambda c: c.post(
            "/api/quality/health-gate", json={"input_file": "data.json"}
        ),
        keys=_keys("health", "gate", "skipped_rules"),
        nested={
            # 与 `DatasetHealthScore.score()` / `GateReport.to_dict()` 一一对应
            "health": _keys(
                "health_score", "level", "metrics", "weights", "total_samples",
            ),
            "gate": _keys(
                "verdict", "passed", "failed_rules", "warned_rules", "metrics",
            ),
        },
    ),
    # ---------- config ----------
    ("GET", "/api/config"): _Case(
        request=lambda c: c.get("/api/config"),
        keys=_keys(
            "default_model", "augmentation", "quality", "dedup",
            "export", "vector", "rag", "multimodal",
        ),
    ),
    ("POST", "/api/config"): _Case(
        request=lambda c: c.post("/api/config", json={}),
        keys=_keys("success", "message", "ignored_keys"),
    ),
    ("GET", "/api/models"): _Case(
        request=lambda c: c.get("/api/models"),
        keys=_keys("models", "default"),
    ),
    # ---------- data ----------
    ("GET", "/api/data/list"): _Case(
        request=lambda c: c.get("/api/data/list"),
        keys=_keys("files"),
    ),
    ("GET", "/api/data/load/{filename}"): _Case(
        request=lambda c: c.get("/api/data/load/data.json"),
        keys=_keys("total", "page", "page_size", "items"),
    ),
    ("PUT", "/api/data/update/{filename}"): _Case(
        request=lambda c: c.put(
            "/api/data/update/data.json",
            params={"index": 0},
            json={"instruction": "改后的问题", "input": "", "output": "改后的答案"},
        ),
        keys=_keys("success"),
    ),
    ("DELETE", "/api/data/delete/{filename}"): _Case(
        request=lambda c: c.delete("/api/data/delete/data.json", params={"index": 0}),
        keys=_keys("success"),
    ),
    ("POST", "/api/data/upload"): _Case(
        request=lambda c: c.post(
            "/api/data/upload",
            files={
                "file": (
                    "train_data_upload.json",
                    json.dumps(SAMPLE_ITEMS, ensure_ascii=False),
                    "application/json",
                )
            },
        ),
        keys=_keys("success", "path", "count"),
    ),
    ("GET", "/api/analyze/{filename}"): _Case(
        request=lambda c: c.get("/api/analyze/data.json"),
        # 与 `Pipeline.analyze_dataset()` 的返回键一致
        keys=_keys("coverage_analysis", "statistics", "dedup_report"),
    ),
    ("GET", "/api/visualize/{filename}"): _Case(
        request=lambda c: c.get("/api/visualize/data.json"),
        keys=_keys("charts"),
    ),
    ("GET", "/api/demo/data"): _Case(
        request=lambda c: c.get("/api/demo/data"),
        # 裸数组响应：校验的是**元素**的键
        keys=_keys("instruction", "input", "output"),
        is_list=True,
    ),
    # ---------- export ----------
    ("POST", "/api/data/export"): _Case(
        request=lambda c: c.post(
            "/api/data/export",
            json={"input_file": "data.json", "output_dir": "exported"},
        ),
        keys=_keys("success", "files"),
    ),
    ("POST", "/api/export/batch"): _Case(
        request=lambda c: c.post(
            "/api/export/batch",
            json={"datasets": {"demo": "data.json"}, "output_dir": "batch_exported"},
        ),
        keys=_keys("success", "results"),
    ),
    ("POST", "/api/export/preview"): _Case(
        request=lambda c: c.post("/api/export/preview", json={"input_file": "data.json"}),
        # 与 `ExportPreview.to_dict()` 的键一一对应
        keys=_keys("format", "original_data", "converted_data", "format_info", "warnings"),
    ),
    ("GET", "/api/export/formats"): _Case(
        request=lambda c: c.get("/api/export/formats"),
        keys=_keys("formats"),
    ),
    # ---------- multimodal ----------
    ("POST", "/api/multimodal/process"): _Case(
        request=lambda c: c.post("/api/multimodal/process", json={"text": "一段文本"}),
        # 与 `MultimodalRecord.to_dict()` 的键一一对应。image/audio 为 null
        # 也必须出现——这里刻意不启用 exclude_none。
        keys=_keys("text", "image", "audio", "modalities", "valid", "errors"),
    ),
    ("POST", "/api/multimodal/scan"): _Case(
        request=lambda c: c.post("/api/multimodal/scan", json={"directory": "."}),
        # 前 6 个键来自 `MultimodalProcessor.generate_report()`，records 由路由补上
        keys=_keys(
            "total_records", "valid_records", "invalid_records",
            "modality_counts", "error_count", "errors", "records",
        ),
    ),
    ("GET", "/api/multimodal/formats"): _Case(
        request=lambda c: c.get("/api/multimodal/formats"),
        keys=_keys("image_extensions", "audio_extensions"),
    ),
    # ---------- version ----------
    ("GET", "/api/versions"): _Case(
        request=lambda c: c.get("/api/versions"),
        keys=_keys("versions"),
    ),
    ("POST", "/api/versions/create"): _Case(
        request=lambda c: c.post(
            "/api/versions/create", params={"filename": "data.json"}, json={}
        ),
        keys=_keys("success", "version_id"),
    ),
    ("POST", "/api/versions/diff"): _Case(
        request=lambda c: c.post(
            "/api/versions/diff",
            json={
                "version1": _create_version(c, "data.json"),
                "version2": _create_version(c, "train.json", label="second"),
            },
        ),
        keys=_keys("version1", "version2", "added_count", "removed_count", "modified_count"),
    ),
    ("GET", "/api/versions/history"): _Case(
        request=lambda c: c.get("/api/versions/history"),
        keys=_keys("history"),
    ),
    ("GET", "/api/versions/{version_id}"): _Case(
        request=lambda c: c.get(f"/api/versions/{_fresh_version(c)}"),
        # `metadata` 只出现在详情里，列表项没有
        keys=_keys("version_id", "label", "description", "created_at", "item_count", "metadata"),
    ),
    ("GET", "/api/versions/{version_id}/data"): _Case(
        request=lambda c: c.get(f"/api/versions/{_fresh_version(c)}/data"),
        keys=_keys("items"),
    ),
    ("POST", "/api/versions/{version_id}/rollback"): _Case(
        request=lambda c: c.post(f"/api/versions/{_fresh_version(c)}/rollback"),
        keys=_keys("success"),
    ),
    ("DELETE", "/api/versions/{version_id}"): _Case(
        # 当前版本不允许删除，所以先造一个「已被顶替」的版本
        request=lambda c: c.delete(f"/api/versions/{_stale_version(c)}"),
        keys=_keys("success"),
    ),
    # ---------- augment ----------
    ("POST", "/api/augment/start"): _Case(
        request=lambda c: c.post(
            "/api/augment/start",
            json={"input_file": "data.json", "output_file": "augmented.json"},
        ),
        keys=_keys("success", "message"),
    ),
    ("GET", "/api/augment/progress"): _Case(
        request=lambda c: c.get("/api/augment/progress"),
        # 没有进行中的任务时只有 `status`；其余 10 个字段只在任务运行中出现，
        # 因此这里无法用一条用例覆盖它们（response_model 仍把它们写进文档）。
        keys=_keys("status"),
    ),
    ("GET", "/api/augment/checkpoints"): _Case(
        request=lambda c: c.get("/api/augment/checkpoints"),
        keys=_keys("checkpoints"),
    ),
    # ---------- dataset（数据集工具箱）----------
    ("POST", "/api/dataset/stats"): _Case(
        request=lambda c: c.post("/api/dataset/stats", json={"input_file": "data.json"}),
        # `summary` 是渲染好的多行文本，不是结构化数据
        keys=_keys(
            "dataset_name", "total_items", "summary",
            "field_statistics", "content_statistics", "quality_metrics",
        ),
    ),
    ("POST", "/api/dataset/validate"): _Case(
        request=lambda c: c.post(
            "/api/dataset/validate", json={"input_file": "data.json"}
        ),
        keys=_keys(
            "is_valid", "total_items", "valid_items", "error_count",
            "warning_count", "issues",
        ),
    ),
    ("POST", "/api/dataset/search"): _Case(
        request=lambda c: c.post(
            "/api/dataset/search", json={"input_file": "data.json", "query": "租房"}
        ),
        # 后四键是**生效的**松紧旋钮与过滤口径：默认方法 contains 谁都不消费、也没传
        # filters，所以响应里全是 null，但键必须在——少一键就是 `SearchResponse`
        # 漏声明、FastAPI 静默裁剪（`test_search_echoes_the_filters_it_applied` 钉值）。
        keys=_keys(
            "query", "method", "total_matches", "query_time_ms", "items", "highlights",
            "fuzzy_threshold", "ngram_n", "applied_filters", "matches_before_filters",
        ),
    ),
    ("POST", "/api/dataset/compare"): _Case(
        request=lambda c: c.post(
            "/api/dataset/compare",
            json={"dataset_a": "data.json", "dataset_b": "train.json"},
        ),
        # 两种互补的对比结论必须同时在
        keys=_keys("quality_comparison", "overlap_comparison"),
    ),
    ("POST", "/api/dataset/features"): _Case(
        request=lambda c: c.post("/api/dataset/features", json={"input_file": "data.json"}),
        keys=_keys("total_items", "field_features", "sparse_fields", "intent_distribution"),
    ),
    ("POST", "/api/dataset/auto-config"): _Case(
        request=lambda c: c.post(
            "/api/dataset/auto-config", json={"input_file": "data.json"}
        ),
        keys=_keys(
            "quality_threshold", "dedup_threshold", "recommended_sample_size",
            "reasoning", "source_stats",
        ),
    ),
    ("POST", "/api/dataset/impact"): _Case(
        request=lambda c: c.post(
            "/api/dataset/impact",
            json={"before_file": "data.json", "after_file": "train.json"},
        ),
        keys=_keys("before", "after", "gains", "beneficial"),
        # before/after 是 `AugmentationMetrics.to_dict()`（含 `extra`），gains 是
        # 四项增益。它们是 `Dict[str, Any]`，顶层模型只会保证「有这个键」，
        # 里面漏一项都不会在顶层显形——所以必须递归钉。
        nested={
            "before": _keys(
                "total_items", "unique_instructions", "avg_length",
                "length_std", "duplicate_rate", "extra",
            ),
            "after": _keys(
                "total_items", "unique_instructions", "avg_length",
                "length_std", "duplicate_rate", "extra",
            ),
            "gains": _keys(
                "scale_gain", "diversity_gain", "dedup_gain", "length_spread_gain"
            ),
        },
    ),
    ("POST", "/api/dataset/evaluate"): _Case(
        request=lambda c: c.post(
            "/api/dataset/evaluate",
            json={
                "generated_file": "data.json",
                "reference_file": "train.json",
            },
        ),
        keys=_keys("metrics", "sample_count", "details"),
    ),
    ("POST", "/api/dataset/convert"): _Case(
        request=lambda c: c.post(
            "/api/dataset/convert",
            json={
                "input_file": "data.json",
                "output_file": "converted.jsonl",
                "target_format": "jsonl",
            },
        ),
        keys=_keys(
            "input_file", "output_file", "source_format", "target_format",
            "input_count", "output_count",
        ),
    ),
    ("POST", "/api/dataset/merge"): _Case(
        request=lambda c: c.post(
            "/api/dataset/merge",
            json={"inputs": ["data.json", "train.json"], "output_file": "merged.json"},
        ),
        keys=_keys(
            "input_files", "output_file", "total_input", "total_output", "removed_duplicates"
        ),
    ),
    ("POST", "/api/dataset/sample"): _Case(
        request=lambda c: c.post(
            "/api/dataset/sample",
            json={"input_file": "data.json", "output_file": "sampled.json", "size": 2},
        ),
        keys=_keys("input_file", "output_file", "input_count", "output_count"),
    ),
    ("POST", "/api/dataset/split"): _Case(
        request=lambda c: c.post(
            "/api/dataset/split",
            json={"input_file": "data.json", "output_dir": "split_out"},
        ),
        keys=_keys("input_file", "output_dir", "splits"),
    ),
    ("POST", "/api/dataset/aggregate"): _Case(
        request=lambda c: c.post(
            "/api/dataset/aggregate",
            json={"datasets": {"a": "data.json", "b": "train.json"}, "output_file": "agg.json"},
        ),
        keys=_keys(
            "output_file", "aggregated_count", "source_counts",
            "removed_duplicates", "conflicts",
        ),
    ),
    ("POST", "/api/dataset/rag"): _Case(
        request=lambda c: c.post(
            "/api/dataset/rag",
            json={"input_file": "data.json", "output_file": "rag.jsonl"},
        ),
        keys=_keys("output_file", "format", "input_count", "record_count"),
    ),
    # ---------- system（运维 / 治理）----------
    ("GET", "/api/system/dependencies"): _Case(
        request=lambda c: c.get("/api/system/dependencies"),
        keys=_keys(
            "installed", "missing", "degraded_features",
            "all_required_present", "available_count",
        ),
    ),
    ("POST", "/api/system/validate-config"): _Case(
        request=_validate_written_config,
        keys=_keys("is_valid", "summary", "errors", "warnings", "info"),
    ),
    ("POST", "/api/system/monitor"): _Case(
        request=lambda c: c.post("/api/system/monitor", json={"input_file": "data.json"}),
        keys=_keys("snapshot_id", "timestamp", "metrics", "alerts"),
    ),
    ("POST", "/api/system/auto-test"): _Case(
        request=lambda c: c.post("/api/system/auto-test", json={"input_file": "data.json"}),
        # total_tests / passed_tests / failed_tests / pass_rate 在 TestSuite 上是
        # 计算属性，漏声明会被静默裁掉
        keys=_keys(
            "name", "description", "total_tests", "passed_tests",
            "failed_tests", "pass_rate", "results",
        ),
    ),
    ("POST", "/api/system/migrate"): _Case(
        request=lambda c: c.post(
            "/api/system/migrate",
            json={"input_file": "data.json", "output_file": "migrated.json"},
        ),
        keys=_keys(
            "migration_id", "source_path", "target_path", "total_items",
            "migrated_items", "failed_items", "rules_applied", "errors", "timestamp",
        ),
    ),
    ("POST", "/api/system/stream"): _Case(
        request=lambda c: c.post(
            "/api/system/stream",
            json={
                "input_file": "data.json",
                "output_file": "streamed.jsonl",
                "operation": "clean",
            },
        ),
        keys=_keys("total_input", "total_output", "processed"),
    ),
    ("GET", "/api/system/dependency/datasets"): _Case(
        request=lambda c: c.get("/api/system/dependency/datasets"),
        keys=_keys("datasets"),
    ),
    ("POST", "/api/system/dependency/datasets"): _Case(
        request=lambda c: c.post(
            "/api/system/dependency/datasets",
            json={"name": "demo", "input_file": "data.json"},
        ),
        keys=_keys(
            "dataset_id", "name", "description", "path", "item_count",
            "created_at", "updated_at", "tags", "metadata",
        ),
    ),
    ("GET", "/api/system/dependency/graph"): _Case(
        request=lambda c: c.get("/api/system/dependency/graph"),
        keys=_keys("nodes", "edges", "issues"),
    ),
    ("GET", "/api/system/backups"): _Case(
        request=lambda c: c.get("/api/system/backups"),
        keys=_keys("backups"),
    ),
    ("POST", "/api/system/backups"): _Case(
        request=lambda c: c.post(
            "/api/system/backups", json={"input_file": "data.json", "name": "snap"}
        ),
        keys=_keys(
            "backup_id", "timestamp", "source_path", "backup_path",
            "item_count", "file_size", "checksum",
        ),
    ),
    ("POST", "/api/system/backups/{backup_id}/restore"): _Case(
        request=_restore_backup,
        keys=_keys("backup_id", "output_path", "item_count"),
    ),
    ("DELETE", "/api/system/backups/{backup_id}"): _Case(
        request=_delete_backup,
        keys=_keys("success"),
    ),
}


def _app():
    from api.main import app

    return app


def _response_model_name(operation: dict):
    """取 200 响应所引用的组件名（未声明 response_model 时返回 None）

    两种形态都要识别：
    - 对象响应：`$ref` 在顶层
    - 裸数组响应（如 `/api/demo/data`）：`$ref` 在 `items` 里

    只看顶层 `$ref` 会把数组型端点当成「未声明契约」，让它悄悄溜过守门。
    """
    schema = (
        operation.get("responses", {})
        .get("200", {})
        .get("content", {})
        .get("application/json", {})
        .get("schema", {})
    )
    ref = schema.get("$ref") or schema.get("items", {}).get("$ref")
    if not ref:
        return None
    return ref.rsplit("/", 1)[-1]


def _modelled_routes(schema: dict) -> set:
    """返回声明了 response_model 的端点集合 {(METHOD, path)}

    这里刻意**不遍历 `app.routes`**：新版 FastAPI 会把 `include_router()` 的结果
    包成 `_IncludedRouter` 塞进 `app.routes`，这些包装对象自身没有
    `path` / `methods` / `response_model`。直接遍历顶层会得出「整个应用只有
    `/api/health` 声明了 response_model」的假象——本模块曾因此整段空转，
    所有断言都在一个只有 health 的世界里通过。OpenAPI schema 是公开接口，
    不受这类内部实现变动影响。
    """
    out = set()
    for path, operations in schema["paths"].items():
        for method, op in operations.items():
            if method.upper() in {"HEAD", "OPTIONS", "PARAMETERS"}:
                continue
            if _response_model_name(op) is not None:
                out.add((method.upper(), path))
    return out


def _response_properties(schema: dict, method: str, path: str) -> set:
    """从 OpenAPI schema 取某端点 200 响应的属性名集合"""
    operation = schema["paths"][path][method.lower()]
    model_name = _response_model_name(operation)
    assert model_name, (
        f"{method} {path} 的 200 响应没有 JSON schema（$ref）——"
        "通常意味着漏了 response_model，响应结构对客户端不可见。"
    )
    return set(schema["components"]["schemas"][model_name].get("properties", {}))


@pytest.fixture
def api_env(tmp_path, monkeypatch):
    """隔离 CWD、写入测试数据集，并**钉住**管道实例

    为什么必须钉住管道：`api.deps._pipeline` 是模块级单例，别的测试模块会用
    `monkeypatch.setattr` 临时换掉它。本模块若不自己钉，拿到的可能是上一个
    测试留下的实例，其版本库目录指向已消失的 tmp 目录，版本类端点的断言就会
    随机失败。这里显式构造一个目录落在本用例 `tmp_path` 内的实例。

    同时把 `augment_dataset` 换成空实现：`/api/augment/start` 会把增强任务
    丢进 `BackgroundTasks`，而 TestClient 在响应返回后**同步执行**它，真实实现
    会去调模型后端。契约测试关心的是 HTTP 形状而非增强算法，故在边界处切断。

    Args:
        tmp_path: pytest 临时目录
        monkeypatch: pytest fixture

    Returns:
        (TestClient, 临时目录)
    """
    from fastapi.testclient import TestClient

    import api.deps as deps
    from api.main import app
    from augmentor import AugmentorPipeline, load_config
    from augmentor.checkpoint import CheckpointManager
    from augmentor.versioning import VersionManager

    config = load_config(str(AI_DIR / "config.yaml"))
    config.versioning.storage_dir = str(tmp_path / "versions")
    pipeline = AugmentorPipeline(config)
    pipeline.version_manager = VersionManager(storage_dir=str(tmp_path / "versions"))
    pipeline.checkpoint_manager = CheckpointManager(
        checkpoint_dir=str(tmp_path / "checkpoints")
    )
    pipeline.augment_dataset = lambda *args, **kwargs: {}
    monkeypatch.setattr(deps, "_pipeline", pipeline)

    monkeypatch.chdir(tmp_path)
    # 白名单也收到临时目录：三个依赖端点不传 registry_path 时，默认注册表目录
    # 跟着白名单首个根走（`api.deps.default_registry_dir`），这样契约测试不会在
    # 仓库工作目录里留下 `.dependency_registry/`。
    monkeypatch.setenv("AUGMENTOR_DATA_ROOTS", str(tmp_path))
    payload = json.dumps(SAMPLE_ITEMS, ensure_ascii=False)
    (tmp_path / "data.json").write_text(payload, encoding="utf-8")
    (tmp_path / "train.json").write_text(payload, encoding="utf-8")
    (tmp_path / "test.json").write_text(
        json.dumps(SAMPLE_ITEMS[:2], ensure_ascii=False), encoding="utf-8"
    )
    return TestClient(app), tmp_path


@pytest.fixture(scope="module")
def schema():
    """OpenAPI schema（模块级缓存：生成一次给所有元数据用例复用）"""
    return _app().openapi()


class TestOpenApiMetadata:
    """`/openapi.json` 的元数据完整性"""

    def test_version_single_source(self, schema):
        """版本号只有一个来源：augmentor.__version__"""
        import augmentor

        assert schema["info"]["version"] == augmentor.__version__

    def test_description_documents_capabilities_and_auth(self, schema):
        """能力分组与鉴权语义必须写进描述，否则 /docs 只是一份裸端点表"""
        desc = schema["info"].get("description") or ""
        assert len(desc) > 300, "description 过短，起不到说明作用"
        assert "鉴权" in desc
        assert "AUGMENTOR_API_KEY" in desc
        assert "429" in desc, "限流行为未写入文档"

    def test_summary_contact_license_present(self, schema):
        info = schema["info"]
        assert info.get("summary"), "缺少 summary"
        assert info.get("contact", {}).get("name"), "缺少 contact"
        assert info["license"]["name"] == "AGPL-3.0-or-later"

    def test_every_used_tag_is_declared(self, schema):
        """端点用到的 tag 必须在 openapi_tags 里有描述，否则 Swagger UI 出现裸分组"""
        declared = {t["name"] for t in schema.get("tags") or []}
        used = set()
        for operations in schema["paths"].values():
            for op in operations.values():
                if isinstance(op, dict):
                    used.update(op.get("tags") or [])
        assert used, "没有任何端点声明 tag"
        assert used <= declared, f"未在 openapi_tags 中声明的 tag: {sorted(used - declared)}"

    def test_health_version_is_not_hardcoded(self):
        """健康检查的版本号必须来自包，不得硬编码（曾硬编码为 2.0.0）"""
        import augmentor
        from fastapi.testclient import TestClient

        response = TestClient(_app()).get("/api/health")
        assert response.status_code == 200
        assert response.json()["version"] == augmentor.__version__


class TestResponseModelDoesNotDropFields:
    """`response_model` 不得静默裁剪响应字段"""

    @pytest.mark.parametrize("route_key", sorted(CALLS))
    def test_response_matches_domain_contract(self, api_env, route_key):
        """行为层：客户端实际收到的键必须与领域契约完全一致

        少了键 → 被 response_model 静默裁掉（调用方拿不到数据）；
        多了键 → 契约未登记（要么补契约，要么说明泄漏了内部字段）。
        """
        client, _ = api_env
        case = CALLS[route_key]

        response = case.request(client)
        assert response.status_code == 200, response.text

        payload = response.json()
        if case.is_list:
            assert isinstance(payload, list) and payload, (
                f"{route_key} 声明为数组响应，但拿到 {type(payload).__name__} 或空数组——"
                "空数组无法校验元素契约。"
            )
            payload = payload[0]
        assert isinstance(payload, dict), f"{route_key} 的响应不是 JSON 对象"

        missing = sorted(set(case.keys) - set(payload))
        extra = sorted(set(payload) - set(case.keys))
        assert not missing and not extra, (
            f"{route_key} 的响应键与领域契约不一致。\n"
            f"  缺少: {missing}\n"
            f"  多出: {extra}\n"
            "缺少通常意味着 response_model 漏声明了字段——注意 dataclass 上的 "
            "@property 不是字段，直接拿它当 response_model 会被静默裁掉。"
        )

        for key, expected in case.nested.items():
            assert key in payload, f"{route_key} 的响应缺少子对象 {key}"
            actual = set(payload[key])
            assert actual == set(expected), (
                f"{route_key} 的 {key} 子对象键与契约不一致。\n"
                f"  缺少: {sorted(set(expected) - actual)}\n"
                f"  多出: {sorted(actual - set(expected))}"
            )

    @pytest.mark.parametrize("route_key", sorted(CALLS))
    def test_openapi_advertises_domain_contract(self, schema, route_key):
        """文档层：`/openapi.json` 必须声明契约的全部键

        /docs 与生成客户端用的是这套 schema；它少了键，前端就永远拿不到该字段。
        """
        method, path = route_key
        props = _response_properties(schema, method, path)
        missing = sorted(set(CALLS[route_key].keys) - props)
        assert not missing, (
            f"{route_key} 的 OpenAPI 响应 schema 未声明 {missing}——"
            "response_model 漏了这些字段，文档与生成客户端都会缺字段。"
        )

    def test_all_modelled_endpoints_have_a_no_drop_case(self, schema):
        """新增 response_model 时必须同步补「不裁字段」用例

        没有这条，新端点可以带着一个漏字段的模型合入而无人察觉。
        """
        modelled = _modelled_routes(schema)

        # 先保证「发现机制」本身有效：若探测失效（返回空集或只返回
        # /api/health），下面的差集断言会**空转通过**，整个模块的守门能力
        # 归零。这里显式把这种情况变成失败。
        assert len(modelled) >= len(CALLS), (
            f"只发现 {len(modelled)} 个声明了 response_model 的端点，"
            f"少于 CALLS 登记的 {len(CALLS)} 个——探测机制很可能失效了，"
            "此时本模块所有断言都在空转。"
        )

        missing = sorted(modelled - set(CALLS))
        assert not missing, (
            f"以下端点声明了 response_model 但缺少「不裁字段」验证: {missing}。"
            "请在 CALLS 中补一条真实调用与领域契约键。"
        )

    def test_every_endpoint_declares_a_response_model(self, schema):
        """反向守门：不允许存在「没有契约」的端点

        未声明 response_model 的端点，其 200 响应在 OpenAPI 里是空 schema，
        生成客户端只能把它当 `any`，前端也拿不到字段提示。

        `CALLS` 覆盖了全部已声明端点（上一条用例保证），这条用例则保证
        **已声明端点就是全部端点**——两条合起来把「端点集」钉死，新端点
        无论漏了模型还是漏了用例都会失败。
        """
        unmodelled = sorted(_all_routes(schema) - _modelled_routes(schema))
        assert not unmodelled, (
            f"以下端点没有声明 response_model: {unmodelled}。\n"
            "请在对应路由上补 response_model（注意 dataclass 的 @property 计算字段"
            "必须显式列出），再到 CALLS 补一条真实调用与领域契约键。"
        )


def _all_routes(schema: dict) -> set:
    """OpenAPI 里登记的全部端点 {(METHOD, path)}"""
    out = set()
    for path, operations in schema["paths"].items():
        for method in operations:
            if method.upper() in {"HEAD", "OPTIONS", "PARAMETERS"}:
                continue
            out.add((method.upper(), path))
    return out
