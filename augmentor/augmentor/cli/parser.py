# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI 参数解析器

## 3.0：`--enhanced` 双轨已合并

T1.7 曾把同一能力的两种实现合并为「一个子命令 + `--enhanced` 开关」，但那只是
把两条实现轨道收进一个命令名，**两条轨道本身仍在**：同一个命令有两套代码路径、
两套结果类型、两套测试，用户还得先知道该加不加 `--enhanced`。3.0 起彻底合并：

* 每个命令只保留**一套实现**（下表的"实现"列）；
* 落败侧独有的能力**并进**幸存实现（下表"并入"列），不做能力删除；
* `--enhanced` 开关、`ENHANCED_HELP` 常量一并移除。传它会由 argparse 直接
  拒绝（退出码 2），比静默忽略更安全。

合并结果：

| 命令 | 实现 | 并入的独有能力 |
| --- | --- | --- |
| `export` | `Exporter`（原生 6 种 + 委托 `EnhancedExporter`，覆盖 `ExportFormat` 全量成员） | 单文件模式、`--max-items` / `--shuffle` / `--seed` |
| `clean` | `cleaner.DatasetCleaner`（规则式） | 旧 `DataCleaner` 的噪声清除（`remove_urls` / `remove_html_tags` / `remove_control_chars` 三条规则），旧 `--no-url-removal` 由「`--rules` 里去掉 `remove_urls`」表达 |
| `analyze` | `analytics.DatasetAnalyzer`（洞察 + 建议 + 三个分数） | 旧 `pipeline.analyze_dataset` 的 `coverage_analysis` / `statistics` / `dedup_report` 三段 |
| `stats` | `statistics.DatasetStatisticsCalculator`（逐字段） | — |
| `visualize` | `visualize_enhanced`（文本/ JSON 报告） | 图表落盘（`--output-dir`） |
| `compare` | `compare_enhanced`（重叠度 + 字段级指标 + 建议） | 旧 `comparison.DatasetComparator` 的质量 / 长度 / 词汇对比与获胜方结论，`--name-a` / `--name-b` |
| `search` | `search_enhanced.EnhancedSearcher` | `ngram` 方法（已移植进 `EnhancedSearcher`） |
| `version` | `version_control.DatasetVersionManager`（不依赖 pipeline） | `rollback`（映射到 `set_current_version`）、`history`（新增操作日志）、`delete` |

`quality` 与 `quality-report` **未合并**：前者筛数据、后者出报告，是两种能力。

## 输出约定

`clean` / `analyze` / `stats` / `search` / `compare` 等报告型命令的 **stdout 保持
机器可读**：`clean` / `analyze` / `stats` 走 `_print`（JSON），`search` 为
「两行摘要 + 条目 JSON 列表」，`compare` 为 markdown 摘要——这些都是合并前
「基础实现」的既有契约，合并不得改变，否则管道下游会静默解析失败。
`--output` 只负责把同一份内容落盘，提示语写 stderr，不污染 stdout。
"""

import argparse

# `export` 支持的格式。`Exporter` 原生实现其中 6 种，其余委托给
# `EnhancedExporter`，因此这个列表就是全量支持面。
#
# 必须与 `Exporter().get_supported_formats()`（即 `ExportFormat` 枚举去掉别名）
# 一致：`raw` 曾被漏在这里，导致 API 与 SDK 都能导、CLI 却报「无效选择」，
# 而注释还声称自己是全量支持面。由
# `tests/integration/test_cli_merged_commands.py::TestExportFormatSurface` 守住。
EXPORT_FORMATS = [
    "json", "jsonl", "csv", "tsv", "alpaca", "sharegpt", "chatml",
    "llama_factory", "vicuna", "belle", "openai", "huggingface", "raw",
]

# `clean --rules` 可选的规则名，与 `cleaner.DatasetCleaner._default_rules` 的键
# 一一对应。在这里显式列出（而不是留空）是为了让拼错的规则名由 argparse 拒绝，
# 而不是被 `cleaner.clean` 静默忽略——后者会让人以为清洗跑过了。
CLEAN_RULES = [
    "remove_empty", "remove_duplicates",
    "remove_urls", "remove_html_tags", "remove_control_chars",
    "normalize_whitespace", "remove_special_chars", "trim_whitespace",
    "remove_long_texts", "remove_short_texts", "normalize_punctuation",
]

# `clean` 的默认规则集。顺序即执行顺序，噪声清除必须在空白归一化之前。
# 这个集合刻意对齐合并前「基础实现」（`data.cleaner.DataCleaner`）的默认行为：
# 去空 / 去重 / 去 URL / 去 HTML 标签 / 去控制字符 / 空白归一化 / 标点归一化 /
# 去首尾空白。`remove_special_chars`（会削掉 `:` `/` 从而破坏 URL）与
# 长/短文本过滤（会直接丢数据）不在默认集里，需要时显式传 `--rules`。
CLEAN_DEFAULT_RULES = [
    "remove_empty", "remove_duplicates",
    "remove_urls", "remove_html_tags", "remove_control_chars",
    "normalize_whitespace", "normalize_punctuation", "trim_whitespace",
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

    # 导出命令
    # 两种输出模式由参数决定（原先由 --enhanced 决定）：
    #   --output + --format      导出单个文件
    #   --output-dir + --formats 每个格式导出一个文件
    export_parser = subparsers.add_parser("export", help="导出数据集")
    export_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    export_parser.add_argument("--output", type=str, help="输出文件路径（单文件模式）")
    export_parser.add_argument("--format", type=str, default=None, choices=EXPORT_FORMATS,
                               help="导出格式（单文件模式，默认 json）")
    export_parser.add_argument("--output-dir", type=str, help="输出目录（批量模式）")
    export_parser.add_argument("--formats", type=str, nargs="+", choices=EXPORT_FORMATS,
                               help="导出格式（批量模式，可多选）")
    export_parser.add_argument("--max-items", type=int, help="最大导出数量")
    export_parser.add_argument("--shuffle", action="store_true", help="导出前随机打乱")
    export_parser.add_argument("--seed", type=int, default=42,
                               help="随机种子（配合 --shuffle）")

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

    # 清洗命令
    # 规则式清洗：旧基础实现的 `--no-url-removal` 已被 `--rules` 覆盖——
    # 不启用 `remove_special_chars` 就等于保留 URL，且粒度更细。
    clean_parser = subparsers.add_parser("clean", help="数据清洗")
    clean_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    clean_parser.add_argument("--output", type=str, required=True, help="输出文件路径")
    clean_parser.add_argument("--rules", type=str, nargs="+", choices=CLEAN_RULES,
                              default=list(CLEAN_DEFAULT_RULES),
                              help="清洗规则（顺序即执行顺序；去掉 remove_urls 即等价于旧的 --no-url-removal）")
    clean_parser.add_argument("--fields", type=str, nargs="+",
                              default=["instruction", "input", "output"],
                              help="参与清洗的字段")

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

    # 健康度门禁命令
    # 与 `benchmark` / `quality` 的区别：那两个只**产出**指标，这一条把指标
    # 组合成「放行 / 拦下」的判定，并以退出码表达（拦下 = 1），可直接当 CI 关卡。
    health_gate_parser = subparsers.add_parser(
        "health-gate", help="数据集健康度评分 + 质量门禁（门禁不通过时退出码为 1）"
    )
    health_gate_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    health_gate_parser.add_argument("--text-field", type=str, default="instruction",
                                    help="计算重复率使用的文本字段")
    health_gate_parser.add_argument("--weights", type=float, nargs=4,
                                    metavar=("COMPLETENESS", "DIVERSITY", "BALANCE", "COVERAGE"),
                                    help="健康分权重（4 个，之和须为 1.0；缺省各 0.25）")
    health_gate_parser.add_argument("--pass-rate", type=float,
                                    help="质量通过率，取自 `quality` 的 pass_rate；"
                                         "不传则该规则不参与判定并出现在 skipped_rules 里")
    health_gate_parser.add_argument("--pass-rate-min", type=float, default=0.6,
                                    help="pass_rate 规则的最低阈值")
    health_gate_parser.add_argument("--duplicate-rate-max", type=float, default=0.3,
                                    help="duplicate_rate 规则的最高阈值")
    health_gate_parser.add_argument("--completeness-min", type=float, default=0.8,
                                    help="completeness 规则的最低阈值（warning 级）")
    health_gate_parser.add_argument("--block-on-warning", action="store_true",
                                    help="warning 级规则失败也阻断")

    # 分析命令
    analyze_parser = subparsers.add_parser("analyze", help="分析数据集")
    analyze_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    analyze_parser.add_argument("--fields", type=str, nargs="+",
                                default=["instruction", "output"], help="分析字段")
    analyze_parser.add_argument("--output", type=str, help="分析报告输出路径（.json）")

    # 可视化命令
    # 两种输出模式由参数决定（原先由 --enhanced 决定）：
    #   --output-dir         生成图表文件（词云/长度分布/质量分布…）
    #   --output + --format  生成文本或 JSON 报告
    visualize_parser = subparsers.add_parser("visualize", help="可视化数据集")
    visualize_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    visualize_parser.add_argument("--output-dir", type=str,
                                  help="图表输出目录（图表模式，默认 visualizations）")
    visualize_parser.add_argument("--output", type=str, help="报告输出路径（报告模式）")
    visualize_parser.add_argument("--format", type=str, default="text",
                                  choices=["text", "json"], help="报告格式（报告模式）")

    # 版本管理命令
    # 单一后端：`version_control.DatasetVersionManager`。原先还有一套走
    # pipeline 的 `versioning.VersionManager`，两套的版本 ID 规则与存储目录都不同，
    # 用户必须靠 `--enhanced` 才知道自己在跟哪一套打交道。
    #   * `diff` 并入 `compare`（后者给出 only_in_a / only_in_b / in_both 统计）
    #   * `rollback` 由 `set_current_version` 实现
    #   * `history` 是**操作日志**（谁在何时 create / rollback），与 `list`
    #     列出的版本清单项不是同一份数据，两者都保留
    version_parser = subparsers.add_parser("version", help="版本管理")
    version_parser.add_argument("--action", type=str,
                                choices=["list", "create", "load", "compare", "rollback",
                                         "delete", "history"],
                                required=True, help="操作类型")
    version_parser.add_argument("--input", type=str, help="输入文件路径（create）")
    version_parser.add_argument("--version", type=str, help="版本 ID（版本A）")
    version_parser.add_argument("--version-b", type=str,
                                help="版本 ID（版本B；compare 缺省时取当前版本）")
    version_parser.add_argument("--output", type=str, help="输出文件路径（load）")
    version_parser.add_argument("--description", type=str, default="",
                                help="版本描述（create）")
    version_parser.add_argument("--versions-dir", type=str, default=".versions",
                                help="版本目录")

    # 数据集对比命令
    compare_parser = subparsers.add_parser("compare", help="对比两个数据集")
    compare_parser.add_argument("--dataset-a", type=str, required=True, help="数据集A文件路径")
    compare_parser.add_argument("--dataset-b", type=str, required=True, help="数据集B文件路径")
    compare_parser.add_argument("--name-a", type=str,
                                help="数据集A的展示名（缺省取文件名 stem）")
    compare_parser.add_argument("--name-b", type=str,
                                help="数据集B的展示名（缺省取文件名 stem）")
    compare_parser.add_argument("--key-fields", type=str, nargs="+", default=["instruction"],
                                help="判定「同一条数据」的关键字段")
    compare_parser.add_argument("--output", type=str, help="对比结果输出路径（.json）")

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

    # 数据集统计命令
    stats_parser = subparsers.add_parser("stats", help="数据集统计信息")
    stats_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    stats_parser.add_argument("--output", type=str, help="统计报告输出路径（.json）")
    stats_parser.add_argument("--fields", type=str, nargs="+", help="统计字段（默认全部）")

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
                                choices=["json", "jsonl", "csv", "tsv", "alpaca", "sharegpt",
                                        "chatml", "llama_factory", "vicuna", "belle"],
                                help="目标格式")
    # 反向边（alpaca/sharegpt/chatml/... → json）：这些格式落盘也是 `.json`，
    # 扩展名推不出来，只能显式声明；不给就按扩展名当 json 原样读。
    convert_parser.add_argument("--input-format", type=str, default=None,
                                choices=["json", "jsonl", "csv", "tsv", "alpaca", "sharegpt",
                                         "chatml", "llama_factory", "vicuna", "belle"],
                                help="源格式（默认从输入文件扩展名推断）")

    # 数据集搜索命令
    # `--method` 取全部方法：exact / contains / ngram / fuzzy / regex。
    # `ngram` 原先只存在于 indexer 实现，已移植进 `EnhancedSearcher`。
    search_parser = subparsers.add_parser("search", help="搜索数据集")
    search_parser.add_argument("--input", type=str, required=True, help="输入文件路径")
    search_parser.add_argument("--query", type=str, required=True, help="搜索查询")
    search_parser.add_argument("--field", type=str, nargs="+",
                               default=["instruction", "output"], help="搜索字段")
    search_parser.add_argument("--method", type=str, default="contains",
                               choices=["exact", "contains", "ngram", "fuzzy", "regex"],
                               help="搜索方法")
    search_parser.add_argument("--limit", type=int, default=None, help="返回数量")
    search_parser.add_argument("--offset", type=int, default=0, help="偏移量")
    search_parser.add_argument("--fuzzy-threshold", type=float, default=0.6,
                               help="fuzzy 的相似度门槛 (0, 1]，越大越严格；1.0 即要求整串出现")
    search_parser.add_argument("--ngram-n", type=int, default=2,
                               help="ngram 的 gram 长度（>=1），越大越严格")
    search_parser.add_argument("--output", type=str, help="结果输出路径（.json）")

    # 配置验证命令
    validate_config_parser = subparsers.add_parser("validate-config", help="验证配置文件")
    # SUPPRESS：仅当子命令后显式传 --config 才写入，避免默认 None 覆盖全局 --config 的值
    validate_config_parser.add_argument(
        "--config", type=str, default=argparse.SUPPRESS, help="配置文件路径"
    )

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
