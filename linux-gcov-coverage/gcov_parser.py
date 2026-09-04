#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""
gcov data parser for kernel branch coverage.

Parses .gcno/.gcda files and gcov tool output to extract
line-level and branch-level coverage information.
"""

import os
import re
import subprocess
import sys
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple


@dataclass
class BranchCoverage:
    """Coverage status for a single branch."""
    line: int
    branch_id: int
    count: int
    taken: bool


@dataclass
class FileCoverage:
    """Coverage data for a single source file."""
    path: str
    lines: Dict[int, int] = field(default_factory=dict)  # line -> exec count
    branches: List[BranchCoverage] = field(default_factory=list)
    functions: Dict[str, int] = field(default_factory=dict)  # func_name -> exec count


class GcovParser:
    """Parse gcov output for kernel coverage data."""

    def __init__(self, build_dir: str, source_dir: str):
        self.build_dir = os.path.abspath(build_dir)
        self.source_dir = os.path.abspath(source_dir)
        self._gcov_available = None

    def _check_gcov(self) -> bool:
        """Check if gcov is available."""
        if self._gcov_available is None:
            try:
                subprocess.run(["gcov", "--version"],
                             capture_output=True, check=True)
                self._gcov_available = True
            except (subprocess.CalledProcessError, FileNotFoundError):
                self._gcov_available = False
        return self._gcov_available

    def parse_gcov_file(self, gcov_path: str) -> FileCoverage:
        """Parse a single .gcov file produced by gcov -b."""
        file_cov = FileCoverage(path="")
        try:
            with open(gcov_path, "r", encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
        except OSError:
            return file_cov

        for line in lines:
            line = line.rstrip("\n")
            # gcov format: "count:line_number:content" or "branch:line_number:taken X"
            if not line or line.startswith("-"):
                continue

            parts = line.split(":", 2)
            if len(parts) < 3:
                continue

            count_str = parts[0].strip()
            line_num_str = parts[1].strip()
            content = parts[2]

            try:
                line_num = int(line_num_str)
            except ValueError:
                continue

            if count_str == "branch":
                # Branch line: "branch  <line>:    branch <id> taken <pct>"
                branch_match = re.search(r'branch\s+(\d+)\s+taken\s+(\d+)%', content)
                if branch_match:
                    branch_id = int(branch_match.group(1))
                    pct = int(branch_match.group(2))
                    file_cov.branches.append(BranchCoverage(
                        line=line_num,
                        branch_id=branch_id,
                        count=1 if pct > 0 else 0,
                        taken=pct > 0,
                    ))
                continue

            try:
                count = int(count_str)
            except ValueError:
                continue

            if count_str == "function":
                func_match = re.search(r'function (\S+) called (\d+)', content)
                if func_match:
                    func_name = func_match.group(1)
                    call_count = int(func_match.group(2))
                    file_cov.functions[func_name] = call_count
                continue

            file_cov.lines[line_num] = count

        return file_cov

    def collect_coverage(self, build_dir: Optional[str] = None) -> Dict[str, FileCoverage]:
        """Collect coverage data from all .gcno files in the build directory."""
        bdir = build_dir or self.build_dir
        coverage: Dict[str, FileCoverage] = {}

        for root, dirs, files in os.walk(bdir):
            dirs.sort()
            for fname in sorted(files):
                if not fname.endswith(".gcno"):
                    continue
                gcov_path = os.path.join(root, fname)
                file_cov = self.parse_gcov_file(gcov_path)
                if file_cov.path or file_cov.lines:
                    # Map build path to source path
                    rel_path = os.path.relpath(gcov_path, bdir)
                    source_path = self._map_build_to_source(rel_path)
                    file_cov.path = source_path
                    coverage[source_path] = file_cov

        return coverage

    def _map_build_path(self, build_path: str) -> str:
        """Map a build-relative path to source-relative path."""
        # Remove .gcno extension
        base = build_path[:-5] if build_path.endswith(".gcno") else build_path
        # Try direct mapping
        if os.path.isfile(os.path.join(self.source_dir, base)):
            return base
        # Try removing build dir prefix
        parts = base.split(os.sep)
        for i, part in enumerate(parts):
            if part in ("obj", "build", "output"):
                rel = os.path.join(*parts[i+1:])
                if os.path.isfile(os.path.join(self.source_dir, rel)):
                    return rel
        return base

    def _map_build_to_source(self, build_path: str) -> str:
        """Map a build-relative .gcno path to source path."""
        # Remove .gcno suffix
        base = build_path[:-5] if build_path.endswith(".gcno") else build_path
        # Try direct
        candidate = os.path.join(self.source_dir, base)
        if os.path.isfile(candidate):
            return base
        # Strip common build prefixes
        for prefix in ("obj/", "build/", "output/", "tools/", "."):
            if base.startswith(prefix):
                rel = base[len(prefix):]
                if os.path.isfile(os.path.join(self.source_dir, rel)):
                    return rel
        return base
