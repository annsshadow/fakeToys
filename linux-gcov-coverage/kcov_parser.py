#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""
kcov data parser for kernel branch coverage.

Parses kcov debugfs output and raw coverage data
to extract per-PC coverage information.
"""

import os
import struct
import sys
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Set, Tuple


@dataclass
class KcovCoverage:
    """Coverage data from kcov."""
    pc_set: Set[int] = field(default_factory=set)
    cmp_set: Set[Tuple[int, int]] = field(default_factory=set)  # (pc, operand)


class KcovParser:
    """Parse kcov coverage data from debugfs."""

    def __init__(self, debugfs_path: str = "/sys/kernel/debug/kcov"):
        self.debugfs_path = debugfs_path
        self._kcov_available = None

    def _check_kcov(self) -> bool:
        """Check if kcov is available."""
        if self._kcov_available is None:
            kcov_dev = os.path.join(self.debugfs_path, "kcov")
            self._kcov_available = os.path.exists(kcov_dev)
        return self._kcov_available

    def read_coverage(self, pid: int) -> KcovCoverage:
        """Read coverage data for a specific PID."""
        if not self._check_kcov():
            return KcovCoverage()

        coverage = KcovCoverage()
        try:
            kcov_path = os.path.join(self.debugfs_path, "kcov")
            with open(kcov_path, "rb") as f:
                # kcov ioctl interface requires specific sequence
                # This is a simplified reader for the mmap'd buffer approach
                data = f.read()
                if len(data) >= 8:
                    # Each entry is a 64-bit PC address
                    num_entries = len(data) // 8
                    for i in range(num_entries):
                        pc = struct.unpack_from("Q", data, i * 8)[0]
                        if pc != 0:
                            coverage.pc_set.add(pc)
        except (OSError, struct.error):
            pass

        return coverage

    def collect_from_trace(self, trace_file: str) -> KcovCoverage:
        """Collect coverage from a kcov trace file."""
        coverage = KcovCoverage()
        try:
            with open(trace_file, "rb") as f:
                data = f.read()
            num_entries = len(data) // 8
            for i in range(num_entries):
                pc = struct.unpack_from("Q", data, i * 8)[0]
                if pc != 0:
                    coverage.pc_set.add(pc)
        except (OSError, struct.error, FileNotFoundError):
            pass
        return coverage

    def get_covered_branches(self, coverage: KcovCoverage,
                            branch_map: Dict[int, List[int]]) -> List[Tuple[int, int]]:
        """
        Map kcov PC coverage to branch coverage.

        branch_map maps branch IDs to (source_pc, target_pc) pairs.
        Returns list of covered (branch_id, taken) tuples.
        """
        covered_branches = []
        for branch_id, (src_pc, tgt_pc) in branch_map.items():
            src_hit = src_pc in coverage.pc_set
            tgt_hit = tgt_pc in coverage.pc_set
            if src_hit:
                covered_branches.append((branch_id, 1 if tgt_hit else 0))
        return covered_branches
