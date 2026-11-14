# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""L342 / A357：定位引用的「内容审计」从一次性探针升成每轮固定动作。

l79 的 `code_but_no_name_match` 档原先只说「落点是代码但不是符号定义行」，
不判那一行到底有没有被点名的构造 —— 三处历史引用早已平移出去而两把尺全绿。
本轮把该档每一格的 **被引行原文** 逐格钉成名册：行号平移、插行、内容改写
都会让原文对不上而当场红；新增一格漂移同样红。名册随文档变长的成本按题 7 接受。

零 parametrize（l97 / l98 把全仓 parametrize 三面钉成现量数）。
"""

import pathlib

from tests.unit.test_doc_line_refs_l79 import (
    DOCS, LINE_REF, ROOT, audit, build_index, name_bucket, source_dirs,
)

MEASURED_CONTENT_CELLS = 121

#: (文档名, 去引号的引用, 被引行原文前 70 字) —— 逐格点名，不是按档数总量
CONTENT_ROSTER = frozenset({
    ('ARCHITECTURE.md', 'api/main.py:119-120', ']'),
    ('ARCHITECTURE.md', 'api/main.py:42-45', 'vector,'),
    ('ARCHITECTURE.md', 'api/routes/system_ops.py:324', 'issue["message"] = message'),
    ('ARCHITECTURE.md', 'augmentor/cli/commands/data_ops.py:88', '_print(result)'),
    ('ARCHITECTURE.md', 'augmentor/cli/commands/quality.py:205', 'completeness_min=args.completeness_min,'),
    ('ARCHITECTURE.md', 'augmentor/cli/commands/quality.py:210', 'verdict_exit(result["gate"]["passed"])'),
    ('ARCHITECTURE.md', 'augmentor/cli/commands/quality.py:76', 'print(f"总体评分: {report.overall_score:.2f}")'),
    ('ARCHITECTURE.md', 'augmentor/config.py:21-28', 'from .export_enhanced import ExportFormat'),
    ('ARCHITECTURE.md', 'augmentor/config.py:244', 'require_seconds(f"{MODEL_ENTRY_PLACEHOLDER}.request_timeout",'),
    ('ARCHITECTURE.md', 'augmentor/config.py:293-297', 'minimum=lo, maximum=hi)'),
    ('ARCHITECTURE.md', 'auto_test.py:402-403', 'f"通过: {suite.passed_tests}",'),
    ('ARCHITECTURE.md', 'cli/commands/ops.py:166', 'verdict_exit(result.failed_items == 0, enforce=args.gate)'),
    ('ARCHITECTURE.md', 'cli/commands/ops.py:31', 'verdict_exit(report.all_required_present, enforce=args.gate)'),
    ('ARCHITECTURE.md', 'cli/commands/ops.py:54', 'verdict_exit(suite.failed_tests == 0, enforce=args.gate)'),
    ('ARCHITECTURE.md', 'cli/commands/quality.py:205', 'completeness_min=args.completeness_min,'),
    ('ARCHITECTURE.md', 'cli/commands/quality.py:96', 'verdict_exit(report.overall_passed, enforce=args.gate)'),
    ('ARCHITECTURE.md', 'cli/commands/security.py:52', 'verdict_exit(report.is_clean, enforce=args.gate)'),
    ('ARCHITECTURE.md', 'cli/commands/security.py:76', 'verdict_exit(report.ready, enforce=args.gate)'),
    ('ARCHITECTURE.md', 'config_validator.py:449-467', ''),
    ('ARCHITECTURE.md', 'context.py:21', 'max_workers: int = 5):'),
    ('ARCHITECTURE.md', 'data_ops.py:78', 'verdict_exit(result.is_valid)'),
    ('ARCHITECTURE.md', 'dataset_ops.py:242', '"truncated_by_max_items": truncated_by_max_items'),
    ('ARCHITECTURE.md', 'logging_setup.py:132', 'Windows 上不 `close()` 就锁着文件，测试里的 `tmp_path` 清理会当场失败。'),
    ('ARCHITECTURE.md', 'models/base.py:309', '`temperature: 1.0` 是**同一份配置**（`models.<名字>.temperature` 的界'),
    ('ARCHITECTURE.md', 'multilingual.py:43', 'max_workers: int = 4):'),
    ('ARCHITECTURE.md', 'ops.py:134', 'verdict_exit(not issues)'),
    ('ARCHITECTURE.md', 'test_chromadb_backend.py:110', 'pytest.skip("chromadb 已安装，无法测试缺失降级路径")'),
    ('ARCHITECTURE.md', 'test_vector.py:56', 'pytest.skip("chromadb 已安装，跳过依赖缺失场景")'),
    ('ARCHITECTURE.md', 'validation.py:437-442', '函数入参设计的（「没传参数」由各调用点回落默认值），而配置对象的字段没有'),
    ('OPTIMIZATION_LOOP.md', 'active_learning.py:98', 'self._scorer = QualityScorer(threshold=self.threshold)'),
    ('OPTIMIZATION_LOOP.md', 'api/main.py:117-123', '{"name": "status", "description": "服务综合状态与依赖诊断"},'),
    ('OPTIMIZATION_LOOP.md', 'api/main.py:119-120', ']'),
    ('OPTIMIZATION_LOOP.md', 'api/main.py:42', 'vector,'),
    ('OPTIMIZATION_LOOP.md', 'api/main.py:42-45', 'vector,'),
    ('OPTIMIZATION_LOOP.md', 'api/routes/dataset_tools.py:641-642', '"beneficial": evaluator.is_beneficial('),
    ('OPTIMIZATION_LOOP.md', 'api/routes/dataset_tools.py:769', 'return await run_in_thread('),
    ('OPTIMIZATION_LOOP.md', 'api/routes/quality.py:146', 'from augmentor.quality import QualityScorer'),
    ('OPTIMIZATION_LOOP.md', 'api/routes/system_ops.py:324', 'issue["message"] = message'),
    ('OPTIMIZATION_LOOP.md', 'api/routes/system_ops.py:445', 'try:'),
    ('OPTIMIZATION_LOOP.md', 'api/routes/system_ops.py:544', '"/api/system/dependency/datasets",'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/cli/commands/data_ops.py:44', 'from augmentor.dataset_ops import DatasetOperations, SplitConfig'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/cli/commands/data_ops.py:88', '_print(result)'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/cli/commands/quality.py:205', 'completeness_min=args.completeness_min,'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/cli/commands/quality.py:76', 'print(f"总体评分: {report.overall_score:.2f}")'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/cli/parser.py:275-281', 'compare_parser.add_argument("--key-fields", type=str, nargs="+", defau'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/config.py:21-28', 'from .export_enhanced import ExportFormat'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/config.py:809', '`file` 默认空串 = **不落文件**。相对路径按进程工作目录解释，父目录必须已存在。'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/config_validator.py:138-143', '# 重试两旋钮自 L45 起真的被消费（经 create_model_backend 透传给模型后端），'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/dedup.py:104-116', 'self._model = None'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/dedup.py:242-290', '"""'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/dedup.py:830', '"""'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/indexer.py:213-242', 'query: 查询字符串'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/logging_setup.py:122-124', '（Temp `l57/rec.txt` NONCE-L57-REC：单条构造 2.2~2.3 µs）。'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/models/base.py:139-149', 'default_max_retry_wait 不在 0-300 秒内，'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/models/base.py:284-292', '1. **必须覆盖所有影响输出的输入**——provider / model / 采样参数 / prompt。'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/models/base.py:304', '@staticmethod'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/models/claude.py:78', '"top_p": self.config.top_p,'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/models/factory.py:83-94', '# 判「是 None」而不是判真假 —— `or` 会把用户显式写的 0 读成「没写」，'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/pipeline.py:100', 'self.model_backend = create_model_backend('),
    ('OPTIMIZATION_LOOP.md', 'augmentor/pipeline.py:213-218', '"instruction": v["instruction"],'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/quality.py:175-179', 'Args:'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/retry.py:233', '"第 %d 次调用失败（%s），%.2fs 后重试",'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/schema.py:78', 'return str(value)'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/search_enhanced.py:129-140', '字段名不是字符串，或过滤值的形状与该算子的要求不符'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/search_enhanced.py:160-166', 'if operator not in FILTER_OPERATORS:'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/search_enhanced.py:385-391', '# 不是参数校验，两者不能混（见 `normalize_filters` 的说明）。'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/streaming.py:328', 'Yields:'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/tracker.py:114', '"""'),
    ('OPTIMIZATION_LOOP.md', 'augmentor/validation.py:217-222', 'minimum: 下界（含），默认 0.0'),
    ('OPTIMIZATION_LOOP.md', 'auto_test.py:402-403', 'f"通过: {suite.passed_tests}",'),
    ('OPTIMIZATION_LOOP.md', 'backup.py:96', 'atomic_write_json(self._index_file, self._index)'),
    ('OPTIMIZATION_LOOP.md', 'benchmark.py:69', 'self._scorer = QualityScorer(threshold=self.threshold)'),
    ('OPTIMIZATION_LOOP.md', 'claude.py:78', '"top_p": self.config.top_p,'),
    ('OPTIMIZATION_LOOP.md', 'cli.py:46', 'sys.exit(1)'),
    ('OPTIMIZATION_LOOP.md', 'cli.py:46-53', 'sys.exit(1)'),
    ('OPTIMIZATION_LOOP.md', 'cli.py:49-51', 'if handler is None:'),
    ('OPTIMIZATION_LOOP.md', 'cli/commands/data_ops.py:40-50', ''),
    ('OPTIMIZATION_LOOP.md', 'cli/commands/pipeline.py:33', 'if operation == "quality":'),
    ('OPTIMIZATION_LOOP.md', 'cli/commands/quality.py:205', 'completeness_min=args.completeness_min,'),
    ('OPTIMIZATION_LOOP.md', 'comparison.py:109', '"""'),
    ('OPTIMIZATION_LOOP.md', 'context.py:112-114', ''),
    ('OPTIMIZATION_LOOP.md', 'converter.py:203-207', 'if not isinstance(record, dict):'),
    ('OPTIMIZATION_LOOP.md', 'converter.py:211-213', 'f"第 {index + 1} 条 {format_name} 记录缺少 "'),
    ('OPTIMIZATION_LOOP.md', 'converter.py:212-213', 'f"`{_CONVERSATION_KEY[format_name]}` 数组"'),
    ('OPTIMIZATION_LOOP.md', 'converter.py:35-45', 'seen.add(key)'),
    ('OPTIMIZATION_LOOP.md', 'data/image.py:158', "with open(path, 'rb') as f:"),
    ('OPTIMIZATION_LOOP.md', 'data_ops.py:76-78', ''),
    ('OPTIMIZATION_LOOP.md', 'data_ops.py:78', 'verdict_exit(result.is_valid)'),
    ('OPTIMIZATION_LOOP.md', 'data_ops.py:88', '_print(result)'),
    ('OPTIMIZATION_LOOP.md', 'dataset_tools.py:576', '"""'),
    ('OPTIMIZATION_LOOP.md', 'factory.py:94', '"response_cache_max_bytes": response_cache_max_bytes,'),
    ('OPTIMIZATION_LOOP.md', 'gemini.py:79', '},'),
    ('OPTIMIZATION_LOOP.md', 'gemini.py:80', '}'),
    ('OPTIMIZATION_LOOP.md', 'indexer.py:432', 'self._items = items'),
    ('OPTIMIZATION_LOOP.md', 'logging_setup.py:226', 'if keys and not root.handlers:'),
    ('OPTIMIZATION_LOOP.md', 'logging_setup.py:95', '`Formatter("%(nope)s")` 构造期不报错，到发第一条日志才抛，而 `Handler.emit`'),
    ('OPTIMIZATION_LOOP.md', 'main.py:150', 'license_info={"name": "AGPL-3.0-or-later"},'),
    ('OPTIMIZATION_LOOP.md', 'models/base.py:284-292', '1. **必须覆盖所有影响输出的输入**——provider / model / 采样参数 / prompt。'),
    ('OPTIMIZATION_LOOP.md', 'models/base.py:309', '`temperature: 1.0` 是**同一份配置**（`models.<名字>.temperature` 的界'),
    ('OPTIMIZATION_LOOP.md', 'ollama.py:71', '"options": {'),
    ('OPTIMIZATION_LOOP.md', 'openai_model.py:73', '"model": self.config.model,'),
    ('OPTIMIZATION_LOOP.md', 'ops.py:134', 'verdict_exit(not issues)'),
    ('OPTIMIZATION_LOOP.md', 'ops.py:31', 'verdict_exit(report.all_required_present, enforce=args.gate)'),
    ('OPTIMIZATION_LOOP.md', 'parser.py:275-281', 'compare_parser.add_argument("--key-fields", type=str, nargs="+", defau'),
    ('OPTIMIZATION_LOOP.md', 'parser.py:440', 'audit_parser.add_argument("--input", type=str, required=True, help="数据'),
    ('OPTIMIZATION_LOOP.md', 'sampler.py:296-299', 'raise DataValidationError('),
    ('OPTIMIZATION_LOOP.md', 'security.py:52', 'verdict_exit(report.is_clean, enforce=args.gate)'),
    ('OPTIMIZATION_LOOP.md', 'system_ops.py:295', 'from augmentor.diagnostics import check_dependencies'),
    ('OPTIMIZATION_LOOP.md', 'test_api_dataset_system_tools.py:1344', 'L41 才接上，而这个端点以前只发比例与种子 —— 于是 L41 修好的东西对 HTTP 调用方仍然'),
    ('OPTIMIZATION_LOOP.md', 'test_backup_branches.py:92', 'assert clean_old_backups(bdir, max_backups=10) == 0'),
    ('OPTIMIZATION_LOOP.md', 'test_config_gates_l82.py:569', '@pytest.mark.parametrize("section", sorted(SECTIONS))'),
    ('OPTIMIZATION_LOOP.md', 'test_model_backend.py:133', '不是性能（实测差异在噪声内，见 docs/augmentor/ARCHITECTURE.md §3.20）。'),
    ('OPTIMIZATION_LOOP.md', 'test_retry.py:455', '那句话说的是这一支管不到的事。口径见 docs/augmentor/ARCHITECTURE.md §3.22。'),
    ('OPTIMIZATION_LOOP.md', 'test_versioning.py:478', '(broken_dir / "metadata.json").write_text("not-json", encoding="utf-8"'),
    ('OPTIMIZATION_LOOP.md', 'tests/integration/test_cli_validate_config.py:53', 'assert code is None'),
    ('OPTIMIZATION_LOOP.md', 'tests/unit/test_config.py:362', '"""'),
    ('OPTIMIZATION_LOOP.md', 'tests/unit/test_config_gates_l82.py:569', '@pytest.mark.parametrize("section", sorted(SECTIONS))'),
    ('OPTIMIZATION_LOOP.md', 'tests/unit/test_versioning.py:478', '(broken_dir / "metadata.json").write_text("not-json", encoding="utf-8"'),
    ('OPTIMIZATION_LOOP.md', 'validation.py:437-442', '函数入参设计的（「没传参数」由各调用点回落默认值），而配置对象的字段没有'),
    ('OPTIMIZATION_LOOP.md', 'versioning.py:234', 'with path_lock(self.storage_dir):'),
    ('OPTIMIZATION_LOOP.md', 'versioning.py:310', 'Returns:'),
})


def _live_cells():
    index = build_index()
    src = source_dirs(index)
    out = set()
    for doc in DOCS:
        text = (ROOT / doc).read_text(encoding="utf-8")
        _counts, buckets = audit(text, index)
        for ref in buckets.get("code_but_no_name_match", []):
            m = LINE_REF.search(ref)
            if not m:
                continue
            bucket, rel = name_bucket(m.group(1), index, src)
            if bucket is not None:
                continue
            body = (ROOT / rel).read_text(encoding="utf-8").split("\n")
            n = int(m.group(2))
            content = body[n - 1].strip()[:70] if 1 <= n <= len(body) else "OUT_OF_RANGE"
            out.add((pathlib.Path(doc).name, ref.strip(chr(96)), content))
    return out


def test_every_drift_cell_has_its_content_pinned():
    """双向精确相等：内容平移 / 新增漂移 / 名册漏格都红"""
    got = _live_cells()
    assert got == CONTENT_ROSTER, (
        "新冒出来的", sorted(got - CONTENT_ROSTER),
        "消失的", sorted(CONTENT_ROSTER - got))


def test_the_roster_scale_is_pinned():
    assert len(CONTENT_ROSTER) == MEASURED_CONTENT_CELLS
    assert len(_live_cells()) == MEASURED_CONTENT_CELLS


def test_a_shifted_line_number_changes_the_pinned_content():
    """注入臂：把一处引用的行号 +1 ⇒ 被引行原文变了，名册当场对不上"""
    index = build_index()
    src = source_dirs(index)
    target = next(r for r in sorted(CONTENT_ROSTER)
                  if ":" in r[1] and r[1].rpartition(":")[2].isdigit())
    head, _, tail = target[1].rpartition(":")
    bucket, rel = name_bucket(head, index, src)
    assert bucket is None, head
    body = (ROOT / rel).read_text(encoding="utf-8").split("\n")
    nxt = body[int(tail)].strip()[:70]
    # 相邻两行内容不同 ⇒ 行号平移一格就会让名册里的原文对不上
    assert nxt != target[2], (target, nxt)
    assert (target[0], head + ":" + str(int(tail) + 1), nxt) not in CONTENT_ROSTER

