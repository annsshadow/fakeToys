# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""CLI `security` 组命令（由 cli.py 拆出，逻辑未改）"""

from ..io import _dump_json, _load_items, _print, _save_items


# ============ PII 脱敏 ============
def run_sanitize(args, config):
    """`sanitize` 子命令"""
    from augmentor.privacy import PiiSanitizer, DEFAULT_PATTERNS, EXTRA_PATTERNS

    items = _load_items(args.input)
    patterns = dict(DEFAULT_PATTERNS)
    if args.extra:
        patterns.update(EXTRA_PATTERNS)
    sanitizer = PiiSanitizer(fields=args.fields, patterns=patterns)
    sanitized, report = sanitizer.sanitize_dataset(items)
    _save_items(sanitized, args.output)
    _print(report.to_dict())
    print(f"已脱敏 {report.touched_items}/{report.total_items} 条，"
          f"命中 {report.total_matches} 处，输出到 {args.output}")


# ============ 泄漏检测 ============
def run_check_leakage(args, config):
    """`check-leakage` 子命令"""
    from augmentor.leakage import detect_leakage

    train_items = _load_items(args.train)
    test_items = _load_items(args.test)
    report = detect_leakage(
        train_items,
        test_items,
        fields=args.fields,
        fuzzy_threshold=args.fuzzy_threshold,
    )
    _print(report.to_dict())
    if report.is_clean:
        print("未检测到训练/测试集泄漏")
    else:
        print(f"检测到 {report.total_leaks} 条泄漏（精确 {report.exact_leaks} / "
              f"近似 {report.fuzzy_leaks}），泄漏率 {report.leak_rate:.2%}")
    if args.output:
        _dump_json(report.to_dict(), args.output)
        print(f"泄漏报告已保存到 {args.output}")


# ============ 数据集就绪审计 ============
def run_audit(args, config):
    """`audit` 子命令"""
    from augmentor.audit import DatasetAuditor

    items = _load_items(args.input)
    reference = _load_items(args.reference) if args.reference else None
    auditor = DatasetAuditor(fields=args.fields)
    report = auditor.audit(items, reference)
    _print(report.to_dict())
    if report.ready:
        print("数据集就绪，可进入增强/交付流程")
    else:
        print(f"数据集未就绪（{len(report.findings)} 项发现）：")
        for finding in report.findings:
            print(f"  - {finding}")
    if args.output:
        _dump_json(report.to_dict(), args.output)
        print(f"审计报告已保存到 {args.output}")
