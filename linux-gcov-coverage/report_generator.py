#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""
Coverage report generator.

Generates lcov tracefiles and HTML reports from parsed coverage data.
"""

import os
import sys
from dataclasses import dataclass
from typing import Dict, List, Optional

from gcov_parser import FileCoverage


@dataclass
class CoverageSummary:
    """Summary statistics for a file or directory."""
    total_lines: int = 0
    covered_lines: int = 0
    total_branches: int = 0
    covered_branches: int = 0
    total_functions: int = 0
    covered_functions: int = 0

    @property
    def line_coverage(self) -> float:
        if self.total_lines == 0:
            return 0.0
        return self.covered_lines / self.total_lines * 100.0

    @property
    def branch_coverage(self) -> float:
        if self.total_branches == 0:
            return 0.0
        return self.covered_branches / self.total_branches * 100.0

    @property
    def function_coverage(self) -> float:
        if self.total_functions == 0:
            return 0.0
        return self.covered_functions / self.total_functions * 100.0


class ReportGenerator:
    """Generate coverage reports in lcov and HTML formats."""

    def __init__(self, source_dir: str):
        self.source_dir = os.path.abspath(source_dir)

    def generate_lcov(self, coverage: Dict[str, FileCoverage],
                      output_path: str) -> str:
        """Generate an lcov tracefile from coverage data."""
        lines = ["TN:\n", "SF:__total__\n"]

        for file_path, file_cov in sorted(coverage.items()):
            if not file_cov.lines and not file_cov.branches:
                continue

            abs_path = os.path.join(self.source_dir, file_path)
            lines.append(f"SF:{abs_path}\n")

            # Function data
            for func_name, count in sorted(file_cov.functions.items()):
                lines.append(f"FN:{func_name}\n")
                lines.append(f"FNDA:{count},{func_name}\n")

            # Line data
            for line_num, exec_count in sorted(file_cov.lines.items()):
                lines.append(f"DA:{line_num},{exec_count}\n")

            # Branch data
            for branch in file_cov.branches:
                taken = 1 if branch.taken else 0
                lines.append(f"BRDA:{branch.line},{branch.branch_id},0,{taken}\n")

            lines.append("end_of_record\n")

        # Write summary
        summary = self._compute_summary(coverage)
        lines.append(f"LH:{summary.covered_lines}\n")
        lines.append(f"LF:{summary.total_lines}\n")
        lines.append(f"BRH:{summary.covered_branches}\n")
        lines.append(f"BRF:{summary.total_branches}\n")
        lines.append(f"FNH:{summary.covered_functions}\n")
        lines.append(f"FNF:{summary.total_functions}\n")

        lcov_content = "".join(lines)
        os.makedirs(os.path.dirname(output_path) if os.path.dirname(output_path) else ".", exist_ok=True)
        with open(output_path, "w", encoding="utf-8") as f:
            f.write(lcov_content)

        return lcov_content

    def generate_html(self, coverage: Dict[str, FileCoverage],
                      output_dir: str) -> None:
        """Generate HTML coverage report."""
        os.makedirs(output_dir, exist_ok=True)

        # Index page
        index_lines = [
            "<!DOCTYPE html>",
            "<html>",
            "<head><title>Kernel Coverage Report</title></head>",
            "<body>",
            "<h1>Kernel Branch Coverage Report</h1>",
            "<table border='1'>",
            "<tr><th>File</th><th>Lines</th><th>Branches</th><th>Functions</th></tr>",
        ]

        for file_path, file_cov in sorted(coverage.items()):
            summary = self._file_summary(file_cov)
            safe_name = file_path.replace("/", "_").replace("\\", "_").replace(".", "_")
            index_lines.append(
                f"<tr><td><a href='{safe_name}.html'>{file_path}</a></td>"
                f"<td>{summary.line_coverage:.1f}%</td>"
                f"<td>{summary.branch_coverage:.1f}%</td>"
                f"<td>{summary.function_coverage:.1f}%</td></tr>"
            )

        index_lines.extend([
            "</table>",
            "</body>",
            "</html>",
        ])

        with open(os.path.join(output_dir, "index.html"), "w", encoding="utf-8") as f:
            f.write("\n".join(index_lines))

        # Per-file pages
        for file_path, file_cov in coverage.items():
            safe_name = file_path.replace("/", "_").replace("\\", "_").replace(".", "_")
            self._generate_file_html(file_path, file_cov,
                                    os.path.join(output_dir, f"{safe_name}.html"))

    def _generate_file_html(self, file_path: str, file_cov: FileCoverage,
                           output_path: str) -> None:
        """Generate HTML page for a single file."""
        summary = self._file_summary(file_cov)
        lines = [
            "<!DOCTYPE html>",
            "<html>",
            f"<head><title>{file_path} Coverage</title></head>",
            "<body>",
            f"<h1>{file_path}</h1>",
            "<table border='1'>",
            f"<tr><th>Metric</th><th>Covered</th><th>Total</th><th>%</th></tr>",
            f"<tr><td>Lines</td><td>{summary.covered_lines}</td><td>{summary.total_lines}</td>"
            f"<td>{summary.line_coverage:.1f}%</td></tr>",
            f"<tr><td>Branches</td><td>{summary.covered_branches}</td><td>{summary.total_branches}</td>"
            f"<td>{summary.branch_coverage:.1f}%</td></tr>",
            f"<tr><td>Functions</td><td>{summary.covered_functions}</td><td>{summary.total_functions}</td>"
            f"<td>{summary.function_coverage:.1f}%</td></tr>",
            "</table>",
            "<h2>Branch Details</h2><table border='1'>",
            "<tr><th>Line</th><th>Branch ID</th><th>Taken</th></tr>",
        ]
        for branch in file_cov.branches:
            color = "green" if branch.taken else "red"
            lines.append(
                f"<tr><td>{branch.line}</td><td>{branch.branch_id}</td>"
                f"<td style='color:{color}'>{'yes' if branch.taken else 'no'}</td></tr>"
            )
        lines.extend(["</table>", "</body>", "</html>"])
        with open(output_path, "w", encoding="utf-8") as f:
            f.write("\n".join(lines))

    def _file_summary(self, file_cov: FileCoverage) -> CoverageSummary:
        """Compute summary statistics for a file."""
        summary = CoverageSummary()
        summary.total_lines = len(file_cov.lines)
        summary.covered_lines = sum(1 for c in file_cov.lines.values() if c > 0)
        summary.total_branches = len(file_cov.branches)
        summary.covered_branches = sum(1 for b in file_cov.branches if b.taken)
        summary.total_functions = len(file_cov.functions)
        summary.covered_functions = sum(1 for c in file_cov.functions.values() if c > 0)
        return summary

    def _compute_summary(self, coverage: Dict[str, FileCoverage]) -> CoverageSummary:
        """Compute overall summary across all files."""
        total = CoverageSummary()
        for file_cov in coverage.values():
            s = self._file_summary(file_cov)
            total.total_lines += s.total_lines
            total.covered_lines += s.covered_lines
            total.total_branches += s.total_branches
            total.covered_branches += s.covered_branches
            total.total_functions += s.total_functions
            total.covered_functions += s.covered_functions
        return total

    def print_summary(self, coverage: Dict[str, FileCoverage]) -> None:
        """Print a human-readable coverage summary."""
        summary = self._compute_summary(coverage)
        print(f"\nCoverage Summary:")
        print(f"  Lines:      {summary.covered_lines}/{summary.total_lines} "
              f"({summary.line_coverage:.1f}%)")
        print(f"  Branches:   {summary.covered_branches}/{summary.total_branches} "
              f"({summary.branch_coverage:.1f}%)")
        print(f"  Functions:  {summary.covered_functions}/{summary.total_functions} "
              f"({summary.function_coverage:.1f}%)")
        print()
