# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 参数解析器

T1.7 起，同一能力的两种实现合并为**一个子命令 + `--enhanced` 开关**，
旧命令名注册为 argparse alias（`export-enhanced` → `export`），
由 `main()` 归一化并打印弃用提示。合并的 8 对：

    export     ← export-enhanced      analyze  ← analyze-data
    visualize  ← visualize-data       version  ← version-control
    compare    ← compare-enhanced     search   ← search-enhanced
    clean      ← clean-enhanced       stats    ← stats-enhanced

`quality` 与 `quality-report` **未合并**：前者筛数据、后者出报告，是两种能力。
"""

import argparse

# `--enhanced` 的统一说明。合并后它不再表示「更好」，而是表示
# 「走独立实现，绕过 AugmentorPipeline」——旧名里的 enhanced 只是历史拼写。
ENHANCED_HELP = "使用独立实现（等价于旧的 <命令>-enhanced），绕过 pipeline"

# 旧名 → 规范名。`main()` 用它归一化 `args.command` 并置 `args.enhanced = True`。
LEGACY_ALIASES: dict = {
    "export-enhanced": "export",
    "analyze-data": "analyze",
    "visualize-data": "visualize",
    "version-control": "version",
    "compare-enhanced": "compare",
    "search-enhanced": "search",
    "clean-enhanced": "clean",
    "stats-enhanced": "stats",
}


# `export --format`（独立实现）支持的格式。基础实现的 `--formats` 不限，
# 因为它走 `Exporter.NATIVE_FORMATS` 并在内部委托，接受面更宽。
EXPORT_FORMATS = [
    "json", "jsonl", "csv", "tsv", "alpaca", "sharegpt", "chatml",
    "llama_factory", "vicuna", "belle", "openai", "huggingface",
]


def build_parser() -> argparse.ArgumentParser:
    """构建命令行解析器

    Returns:
        ArgumentParser 实例
    """
    parser = argparse.ArgumentParser(description="AI 训练数据增强工具")
    parser.add_argument("--config", type=str, default="config.yaml", help="配置文件路径")

    subparsers = parser.add_subparsers(dest="command", help="子命令")

    # 增强命令
    augment_parser = subparsers.add_parser("augment", help="增强数据集")
    augment_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    augment_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    augment_parser.add_argument("--no-quality", action="store_true", help="禁用质量检查")
    augment_parser.add_argument("--no-dedup", action="store_true", help="禁用去重")
    augment_parser.add_argument("--no-checkpoint", action="store_true", help="禁用断点续传")

    # 导出命令（合并 export-enhanced）
    # 基础实现走 pipeline.export_dataset：--output-dir + --formats（可多选，批量落盘）
    # 独立实现走 export_enhanced.export_dataset：--output + --format（单文件，可截断/打乱）
    export_parser = subparsers.add_parser(
        "export", aliases=["export-enhanced"], help="导出数据集"
    )
    export_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    export_parser.add_argument("--output-dir", type=str, help="输出目录（基础实现必填）")
    export_parser.add_argument("--formats", nargs="+", help="导出格式（基础实现，可多选）")
    export_parser.add_argument("--output", type=str, help="输出文件路径（--enhanced 必填）")
    export_parser.add_argument("--format", type=str, default=None, choices=EXPORT_FORMATS,
                               help="导出格式（--enhanced，默认 json）")
    export_parser.add_argument("--max-items", type=int, help="最大导出数量（--enhanced）")
    export_parser.add_argument("--shuffle", action="store_true", help="随机打乱（--enhanced）")
    export_parser.add_argument("--seed", type=int, default=42, help="随机种子（--enhanced）")
    export_parser.add_argument("--enhanced", action="store_true", help=ENHANCED_HELP)

    # 导出预览命令
    preview_parser = subparsers.add_parser("preview", help="预览导出格式转换结果")
    preview_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    preview_parser.add_argument("--format", type=str, default="jsonl", help="目标格式")
    preview_parser.add_argument("--size", type=int, default=5, help="预览条数")

    # 质量命令
    quality_parser = subparsers.add_parser("quality", help="质量评估、去重与报告")
    quality_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    quality_parser.add_argument("--output", type=str, help="过滤后数据的输出路径")
    quality_parser.add_argument("--report", type=str, help="质量报告输出路径（.md 或 .json）")

    # 清洗命令（合并 clean-enhanced）
    # 基础实现走 data.DataCleaner：--no-url-removal
    # 独立实现走 cleaner.clean_dataset：--rules
    clean_parser = subparsers.add_parser(
        "clean", aliases=["clean-enhanced"], help="数据清洗"
    )
    clean_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    clean_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    clean_parser.add_argument("--no-url-removal", action="store_true", help="保留 URL（基础实现）")
    clean_parser.add_argument("--rules", type=str, nargs="+",
                              default=["remove_empty", "remove_duplicates",
                                       "normalize_whitespace", "trim_whitespace"],
                              help="清洗规则（--enhanced）")
    clean_parser.add_argument("--enhanced", action="store_true", help=ENHANCED_HELP)

    # 标注命令
    annotate_parser = subparsers.add_parser("annotate", help="自动标注")
    annotate_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    annotate_parser.add_argument("--output", type=str, required=True, help="输出文件路径")

    # RAG 格式命令
    rag_parser = subparsers.add_parser("rag", help="转换为 RAG 训练数据格式")
    rag_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    rag_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    rag_parser.add_argument("--format", type=str, default="llamaindex",
                            choices=["llamaindex", "langchain", "custom"], help="目标格式")

    # 基准命令
    benchmark_parser = subparsers.add_parser("benchmark", help="数据质量基准")
    benchmark_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    benchmark_parser.add_argument("--baseline", type=str, help="基准文件路径")
    benchmark_parser.add_argument("--save-baseline", action="store_true", help="将结果保存为基准")
    benchmark_parser.add_argument("--report", type=str, help="报告输出路径")

    # 分析命令（合并 analyze-data）
    analyze_parser = subparsers.add_parser(
        "analyze", aliases=["analyze-data"], help="分析数据集"
    )
    analyze_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    analyze_parser.add_argument("--fields", type=str, nargs="+", default=["instruction", "output"],
                                help="分析字段（--enhanced）")
    analyze_parser.add_argument("--output", type=str, help="分析报告输出路径（--enhanced）")
    analyze_parser.add_argument("--enhanced", action="store_true", help=ENHANCED_HELP)

    # 可视化命令（合并 visualize-data）
    visualize_parser = subparsers.add_parser(
        "visualize", aliases=["visualize-data"], help="可视化数据集"
    )
    visualize_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    visualize_parser.add_argument("--output-dir", type=str, default="visualizations",
                                  help="输出目录（基础实现）")
    visualize_parser.add_argument("--output", type=str, help="输出报告路径（--enhanced）")
    visualize_parser.add_argument("--format", type=str, default="text", choices=["text", "json"],
                                  help="报告格式（--enhanced）")
    visualize_parser.add_argument("--enhanced", action="store_true", help=ENHANCED_HELP)

    # 版本管理命令（合并 version-control）
    # 两者是**两套独立的版本后端**，不是同一实现的强弱版：
    #   基础：pipeline.version_manager（augmentor/versioning.py），版本 ID 用 --version-id
    #   独立：version_control.DatasetVersionManager，版本 ID 用 --version，另有 --versions-dir
    # `--action` 取两者并集；`create`/`list` 两者都有，靠 --enhanced 区分后端。
    version_parser = subparsers.add_parser(
        "version", aliases=["version-control"], help="版本管理"
    )
    version_parser.add_argument("--action", type=str,
                                choices=["list", "create", "diff", "rollback", "history",
                                         "load", "compare"],
                                required=True, help="操作类型")
    version_parser.add_argument("--input", type=str, help="输入文件路径")
    version_parser.add_argument("--version-id", type=str, help="版本 ID（基础实现）")
    version_parser.add_argument("--version-id-2", type=str, help="第二个版本 ID（基础实现 diff）")
    version_parser.add_argument("--version", type=str, help="版本 ID（--enhanced）")
    version_parser.add_argument("--output", type=str, help="输出文件路径（--enhanced）")
    version_parser.add_argument("--description", type=str, default="", help="版本描述（--enhanced）")
    version_parser.add_argument("--versions-dir", type=str, default=".versions",
                                help="版本目录（--enhanced）")
    version_parser.add_argument("--enhanced", action="store_true", help=ENHANCED_HELP)

    # 数据集对比命令（合并 compare-enhanced）
    compare_parser = subparsers.add_parser(
        "compare", aliases=["compare-enhanced"], help="对比两个数据集"
    )
    compare_parser.add_argument("--dataset-a", type=str, required=True, help="数据集A文件路径")
    compare_parser.add_argument("--dataset-b", type=str, required=True, help="数据集B文件路径")
    compare_parser.add_argument("--name-a", type=str, default="Dataset A",
                                help="数据集A的名称（基础实现）")
    compare_parser.add_argument("--name-b", type=str, default="Dataset B",
                                help="数据集B的名称（基础实现）")
    compare_parser.add_argument("--key-fields", type=str, nargs="+", default=["instruction"],
                                help="关键字段（--enhanced）")
    compare_parser.add_argument("--output", type=str, help="对比结果输出路径")
    compare_parser.add_argument("--enhanced", action="store_true", help=ENHANCED_HELP)

    # 流式处理命令
    stream_parser = subparsers.add_parser("stream", help="流式处理大数据集")
    stream_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    stream_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    stream_parser.add_argument("--chunk-size", type=int, default=1000, help="分块大小")
    stream_parser.add_argument("--operation", type=str, required=True,
                               choices=["quality", "dedup", "clean", "export"],
                               help="处理操作")

    # 数据集合并命令
    merge_parser = subparsers.add_parser("merge", help="合并多个数据集")
    merge_parser.add_argument("--inputs", type=str, nargs="+", required=True, help="输入文件路径列表")
    merge_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    merge_parser.add_argument("--no-dedup", action="store_true", help="禁用去重")

    # 数据集采样命令
    sample_parser = subparsers.add_parser("sample", help="采样数据集")
    sample_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    sample_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    sample_parser.add_argument("--size", type=int, help="采样数量")
    sample_parser.add_argument("--ratio", type=float, help="采样比例 (0-1)")
    sample_parser.add_argument("--method", type=str, default="random",
                               choices=["random", "systematic", "stratified"], help="采样方法")
    sample_parser.add_argument("--seed", type=int, help="随机种子")

    # 数据集分割命令
    split_parser = subparsers.add_parser("split", help="分割数据集")
    split_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    split_parser.add_argument("--output-dir", type=str, required=True, help="输出目录")
    split_parser.add_argument("--train-ratio", type=float, default=0.8, help="训练集比例")
    split_parser.add_argument("--val-ratio", type=float, default=0.1, help="验证集比例")
    split_parser.add_argument("--test-ratio", type=float, default=0.1, help="测试集比例")
    split_parser.add_argument("--seed", type=int, help="随机种子")

    # 数据集统计命令（合并 stats-enhanced）
    stats_parser = subparsers.add_parser(
        "stats", aliases=["stats-enhanced"], help="数据集统计信息"
    )
    stats_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    stats_parser.add_argument("--output", type=str, help="统计报告输出路径（--enhanced）")
    stats_parser.add_argument("--fields", type=str, nargs="+", help="统计字段（--enhanced）")
    stats_parser.add_argument("--enhanced", action="store_true", help=ENHANCED_HELP)

    # 数据集验证命令
    validate_parser = subparsers.add_parser("validate", help="验证数据集格式")
    validate_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    validate_parser.add_argument("--preset", type=str, default="basic",
                                 choices=["basic", "strict", "chat"], help="验证规则预设")
    validate_parser.add_argument("--output", type=str, help="验证结果输出路径")

    # 数据集转换命令
    convert_parser = subparsers.add_parser("convert", help="转换数据集格式")
    convert_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    convert_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    convert_parser.add_argument("--format", type=str, required=True,
                                choices=["json", "jsonl", "csv", "alpaca", "sharegpt", 
                                        "chatml", "llama_factory", "vicuna", "belle"],
                                help="目标格式")

    # 数据集搜索命令（合并 search-enhanced）
    # --method 取两者并集：基础支持 exact/contains/ngram，独立实现支持 exact/contains/fuzzy/regex
    search_parser = subparsers.add_parser(
        "search", aliases=["search-enhanced"], help="搜索数据集"
    )
    search_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    search_parser.add_argument("--query", type=str, required=True, help="搜索查询")
    search_parser.add_argument("--field", type=str, nargs="+", default=["instruction", "output"],
                               help="搜索字段")
    search_parser.add_argument("--method", type=str, default="contains",
                               choices=["exact", "contains", "ngram", "fuzzy", "regex"],
                               help="搜索方法")
    search_parser.add_argument("--limit", type=int, default=None, help="返回数量")
    search_parser.add_argument("--offset", type=int, default=0, help="偏移量（--enhanced）")
    search_parser.add_argument("--output", type=str, help="结果输出路径（--enhanced）")
    search_parser.add_argument("--enhanced", action="store_true", help=ENHANCED_HELP)

    # 配置验证命令
    validate_config_parser = subparsers.add_parser("validate-config", help="验证配置文件")
    # SUPPRESS：仅当子命令后显式传 --config 才写入，避免默认 None 覆盖全局 --config 的值
    validate_config_parser.add_argument(
        "--config", type=str, default=argparse.SUPPRESS, help="配置文件路径"
    )

    # 数据分析 / 数据清洗 / 增强导出 / 数据可视化 四个旧命令
    # （analyze-data / clean-enhanced / export-enhanced / visualize-data）
    # 已合并进对应的主命令，旧名注册为 alias，不再单独定义解析器。

    # 质量报告命令
    quality_report_parser = subparsers.add_parser("quality-report", help="生成质量报告")
    quality_report_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    quality_report_parser.add_argument("--output", type=str, help="输出报告路径")
    quality_report_parser.add_argument("--format", type=str, default="json", choices=["json", "markdown"], help="报告格式")
    quality_report_parser.add_argument("--threshold", type=float, default=0.7, help="质量阈值")

    # 备份命令
    backup_parser = subparsers.add_parser("backup", help="数据备份")
    backup_parser.add_argument("--action", type=str, required=True, choices=["create", "restore", "list", "delete"], help="操作类型")
    backup_parser.add_argument("--input", type=str, help="输入文件路径")
    backup_parser.add_argument("--output", type=str, help="输出文件路径")
    backup_parser.add_argument("--name", type=str, help="备份名称")
    backup_parser.add_argument("--backup-dir", type=str, default=".backups", help="备份目录")

    # PII 脱敏命令
    sanitize_parser = subparsers.add_parser("sanitize", help="PII 脱敏")
    sanitize_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    sanitize_parser.add_argument("--output", type=str, required=True, help="脱敏结果输出路径")
    sanitize_parser.add_argument("--fields", type=str, nargs="+",
                                 default=["instruction", "input", "output"], help="脱敏字段")
    sanitize_parser.add_argument("--extra", action="store_true",
                                 help="启用额外模式（信用卡号/URL，误伤面较大）")

    # 泄漏检测命令
    leak_parser = subparsers.add_parser("check-leakage", help="训练/测试集泄漏检测")
    leak_parser.add_argument("--train", type=str, required=True, help="训练集文件路径")
    leak_parser.add_argument("--test", type=str, required=True, help="测试集文件路径")
    leak_parser.add_argument("--fields", type=str, nargs="+", default=["instruction"], help="比较字段")
    leak_parser.add_argument("--fuzzy-threshold", type=float, default=0.8, help="近似匹配阈值")
    leak_parser.add_argument("--output", type=str, help="报告输出路径（.json）")

    # 数据集就绪审计命令
    audit_parser = subparsers.add_parser("audit", help="数据集就绪审计")
    audit_parser.add_argument("--input", type=str, required=True, help="数据集文件路径")
    audit_parser.add_argument("--reference", type=str, help="参考集（如测试集）路径")
    audit_parser.add_argument("--fields", type=str, nargs="+",
                              default=["instruction", "input", "output"], help="统计字段")
    audit_parser.add_argument("--output", type=str, help="报告输出路径（.json）")

    # 依赖诊断命令
    doctor_parser = subparsers.add_parser("doctor", help="运行时依赖诊断")
    doctor_parser.add_argument("--json", action="store_true", help="输出 JSON 报告")

    # 增强搜索 / 增强统计 / 增强比较 / 版本控制四个旧命令
    # （search-enhanced / stats-enhanced / compare-enhanced / version-control）
    # 已合并进对应的主命令，旧名注册为 alias，不再单独定义解析器。

    # 自动化测试命令
    auto_test_parser = subparsers.add_parser("auto-test", help="自动化测试")
    auto_test_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    auto_test_parser.add_argument("--output", type=str, help="输出报告路径")
    auto_test_parser.add_argument("--suite", type=str, help="测试套件名称")

    # 质量监控命令
    monitor_parser = subparsers.add_parser("monitor", help="质量监控")
    monitor_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    monitor_parser.add_argument("--output", type=str, help="输出报告路径")

    # 依赖管理命令
    dependency_parser = subparsers.add_parser("dependency", help="依赖管理")
    dependency_parser.add_argument("--action", type=str, required=True, choices=["register", "list", "graph", "validate"], help="操作类型")
    dependency_parser.add_argument("--input", type=str, help="输入文件路径")
    dependency_parser.add_argument("--name", type=str, help="数据集名称")
    dependency_parser.add_argument("--output", type=str, help="输出报告路径")
    dependency_parser.add_argument("--registry-path", type=str, default=".dependency_registry", help="注册表路径")

    # 迁移命令
    migrate_parser = subparsers.add_parser("migrate", help="数据迁移")
    migrate_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    migrate_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    migrate_parser.add_argument("--rules", type=str, nargs="+", help="迁移规则")

    # 数据画像命令
    profiling_parser = subparsers.add_parser("profile", help="生成数据集画像")
    profiling_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    profiling_parser.add_argument("--output", type=str, help="画像输出路径（.json）")

    # 异常检测命令
    outlier_parser = subparsers.add_parser("outliers", help="检测长度异常样本")
    outlier_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    outlier_parser.add_argument("--method", type=str, default="zscore",
                                choices=["zscore", "iqr", "zscore_one_sided"], help="检测方法")
    outlier_parser.add_argument("--threshold", type=float, default=3.0, help="判定阈值")
    outlier_parser.add_argument("--field", type=str, default="length", help="检测字段")
    outlier_parser.add_argument("--output", type=str, help="结果输出路径（.json）")

    # 特征检测命令
    feature_parser = subparsers.add_parser("features", help="检测数据集特征维度")
    feature_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    feature_parser.add_argument("--output", type=str, help="结果输出路径（.json）")

    # 自动配置推荐命令
    auto_config_parser = subparsers.add_parser("auto-config", help="推荐数据管线参数")
    auto_config_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    auto_config_parser.add_argument("--output", type=str, help="推荐结果输出路径（.json）")

    # 数据聚合命令
    aggregate_parser = subparsers.add_parser("aggregate", help="聚合多个数据集")
    aggregate_parser.add_argument("--inputs", type=str, nargs="+", required=True,
                                   help="源数据集文件路径列表")
    aggregate_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    aggregate_parser.add_argument("--strategy", type=str, default="union",
                                   choices=["union", "intersection", "weighted", "consistent"],
                                   help="聚合策略")
    aggregate_parser.add_argument("--target-size", type=int, default=100, help="加权采样目标量")

    return parser
