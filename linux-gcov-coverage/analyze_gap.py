#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""
分析「范围内（核心子系统）」覆盖率与 95% 目标的缺口。

读取 baseline_report_scoped.json（由 baseline_measurement.py 产出），
输出：
  1. 范围内整体行/分支覆盖率（95% 目标度量口径）。
  2. 达到整体 95% 分支覆盖率还需覆盖多少分支。
  3. 各子系统按「未覆盖分支数」排序 —— 即 Phase 2（syzkaller/KCOV）的最大杠杆。

用法：
  python3 analyze_gap.py [--report tools/testing/coverage/baseline/baseline_report_scoped.json] [--target 95]
"""
import argparse
import json
import os
import sys


def main() -> int:
    parser = argparse.ArgumentParser(description="核心子系统 95% 覆盖率缺口分析")
    parser.add_argument(
        "--report",
        default="baseline/baseline_report_scoped.json",
        help="baseline_report_scoped.json 路径（相对本工具链目录，或传绝对路径）",
    )
    parser.add_argument("--target", type=float, default=95.0,
                        help="目标分支覆盖率（默认 95%%）")
    args = parser.parse_args()

    if not os.path.isfile(args.report):
        print(f"ERROR: 找不到报告 {args.report}（请先运行 baseline_measurement.py）",
              file=sys.stderr)
        return 1

    with open(args.report, "r", encoding="utf-8") as f:
        report = json.load(f)

    subsystems = report.get("subsystems", [])
    total_branches = report.get("total_branches", 0)
    covered_branches = report.get("covered_branches", 0)
    total_lines = report.get("total_lines", 0)
    covered_lines = report.get("covered_lines", 0)

    branch_pct = covered_branches / total_branches * 100.0 if total_branches else 0.0
    line_pct = covered_lines / total_lines * 100.0 if total_lines else 0.0
    target = args.target

    # 达到整体目标覆盖率所需覆盖的分支数
    need_total = int(total_branches * target / 100.0)
    need_more = max(0, need_total - covered_branches)

    print("=" * 72)
    print(f"范围内（核心子系统）覆盖率  目标分支 = {target:.0f}%")
    print("=" * 72)
    print(f"  分支: {covered_branches}/{total_branches}  ({branch_pct:.2f}%)")
    print(f"  行  : {covered_lines}/{total_lines}  ({line_pct:.2f}%)")
    print(f"  距 {target:.0f}% 分支目标还需覆盖: {need_more} 条分支")
    print()

    # 按未覆盖分支数排序（最大缺口=最大杠杆）
    rows = []
    for s in subsystems:
        tb = s.get("total_branches", 0)
        cb = s.get("covered_branches", 0)
        if tb == 0:
            continue
        uncovered = tb - cb
        cur_pct = cb / tb * 100.0
        # 该子系统单独达到 target 还需覆盖多少
        need_in_ss = max(0, int(tb * target / 100.0) - cb)
        rows.append((s.get("subsystem", "?"), tb, cb, uncovered, cur_pct, need_in_ss))

    rows.sort(key=lambda r: r[3], reverse=True)

    print(f"{'子系统':<14}{'总分支':>10}{'已覆盖':>10}{'未覆盖':>10}{'当前%':>9}{f'{target:.0f}%还需':>11}")
    print("-" * 72)
    for name, tb, cb, unc, cur, need in rows:
        print(f"{name:<14}{tb:>10}{cb:>10}{unc:>10}{cur:>8.1f}%{need:>11}")
    print("-" * 72)
    print("说明：Phase 2（syzkaller/KCOV）应优先瞄准「未覆盖分支数」最大的子系统。")
    print("注意：部分核心代码（如 mm/page_alloc 紧急路径、调度器 idle）在单引导")
    print("场景下结构不可达，95% 仍为极具挑战的上界而非保证值。")
    return 0


if __name__ == "__main__":
    sys.exit(main())
