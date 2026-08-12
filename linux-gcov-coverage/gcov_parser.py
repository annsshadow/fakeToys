#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""
gcov data parser for kernel branch coverage.

Parses .gcov files produced by ``gcov -b`` (line-level and branch-level
coverage information) into FileCoverage objects.
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
            except (subprocess.CalledProcessError, FileNotFoundError, OSError):
                self._gcov_available = False
        return self._gcov_available

    def parse_gcov_file(self, gcov_path: str) -> FileCoverage:
        """Parse a single .gcov file produced by `gcov -b`."""
        file_cov = FileCoverage(path="")
        try:
            with open(gcov_path, "r", encoding="utf-8", errors="replace") as f:
                lines = f.readlines()
        except OSError:
            return file_cov

        current_line = 0  # tracks the source line a following branch belongs to
        for raw in lines:
            line = raw.rstrip("\n")
            stripped = line.strip()

            # Branch line: "branch <id> taken <pct>%" or "branch <id> never executed"
            if stripped.startswith("branch "):
                m = re.match(r"branch\s+(\d+)\s+(taken\s+(\d+)%|never executed)", stripped)
                if m:
                    bid = int(m.group(1))
                    if m.group(3):
                        pct = int(m.group(3).rstrip("%"))
                    else:
                        pct = 0
                    file_cov.branches.append(BranchCoverage(
                        line=current_line,
                        branch_id=bid,
                        count=1 if pct > 0 else 0,
                        taken=pct > 0,
                    ))
                continue

            # Function summary line: "function <name> called <n> returned <pct>%"
            if stripped.startswith("function "):
                m = re.search(r"function\s+(\S+)\s+called\s+(\d+)", stripped)
                if m:
                    file_cov.functions[m.group(1)] = int(m.group(2))
                continue

            # Source line: "<count>:<lineno>:<content>"
            parts = line.split(":", 2)
            if len(parts) < 3:
                continue
            count_str = parts[0].strip()
            try:
                line_num = int(parts[1].strip())
            except ValueError:
                continue
            content = parts[2]
            current_line = line_num

            # Capture the real source path declared by gcov.
            if count_str == "-" and content.strip().startswith("Source:"):
                file_cov.path = content.strip()[7:].strip()
                continue
            if count_str == "-":
                # Non-executable line (braces, comments, preprocessor).
                continue

            try:
                count = int(count_str)
            except ValueError:
                # "#####" / "=====" -> uncovered executable line.
                count = 0
            file_cov.lines[line_num] = count

        return file_cov

    def _ensure_gcov_source_symlinks(self) -> None:
        """Make gcov resolve out-of-tree source paths to the real source tree.

        gcov resolves the source path stored in each ``.gcno`` *relative to the
        ``.gcno``'s own directory*, not the current working directory. For an
        out-of-tree build (``O=build_dir``) the ``.gcno`` lives at
        ``build_dir/<A>/<B>/.../file.gcno`` while the source is at
        ``source_dir/<A>/<B>/.../file.c``. gcov therefore looks for
        ``build_dir/<A>/<A>/<B>/.../file.c`` — the first path component is
        *doubled*.

        Rather than copy the whole (huge) source tree, create lightweight
        cross-symlinks ``build_dir/<A>/<B> -> source_dir/<B>`` for every
        top-level build dir ``<A>`` and every top-level source dir ``<B>``.
        Then ``build_dir/<A>/<A>/...`` follows ``build_dir/<A>/<A>`` (symlink)
        into ``source_dir/<A>/...`` and gcov finds the real sources. Only a few
        hundred symlinks, so it is fast even on a slow (Windows) bind mount.
        """
        if not os.path.isdir(self.build_dir) or not os.path.isdir(self.source_dir):
            return
        bdirs = [
            d for d in os.listdir(self.build_dir)
            if os.path.isdir(os.path.join(self.build_dir, d))
            and not d.startswith(".")
        ]
        sdirs = [
            d for d in os.listdir(self.source_dir)
            if os.path.isdir(os.path.join(self.source_dir, d))
            and not d.startswith(".")
            and d != os.path.basename(self.build_dir)
        ]
        for a in bdirs:
            for b in sdirs:
                dst = os.path.join(self.build_dir, a, b)
                if os.path.lexists(dst):
                    continue
                try:
                    os.symlink(os.path.join(self.source_dir, b), dst)
                except OSError:
                    pass

    def _generate_gcov(self, bdir: str) -> None:
        """Run `gcov -b` on every .gcno to emit .gcov text files.

        Requires the matching .gcda (runtime data) to be present next to the
        .gcno files (copied from the running instrumented kernel's debugfs).
        """
        if not self._check_gcov():
            print("WARNING: gcov not available; cannot generate .gcov files.",
                  file=sys.stderr)
            return
        # Out-of-tree builds store source paths that gcov cannot resolve
        # without help; create the cross-symlink forest once.
        self._ensure_gcov_source_symlinks()
        for root, dirs, files in os.walk(bdir):
            dirs.sort()
            gcno_files = [f for f in files if f.endswith(".gcno")]
            if not gcno_files:
                continue
            for f in sorted(gcno_files):
                gcno_path = os.path.join(root, f)
                # Only files that actually have runtime data can yield coverage.
                gcda_path = os.path.splitext(gcno_path)[0] + ".gcda"
                if not os.path.exists(gcda_path):
                    continue
                try:
                    subprocess.run(
                        ["gcov", "-b", "-o", root, gcno_path],
                        cwd=root, capture_output=True, text=True,
                        check=False, timeout=120,
                    )
                except (subprocess.SubprocessError, OSError):
                    pass

    def collect_coverage(self, build_dir: Optional[str] = None) -> Dict[str, FileCoverage]:
        """Collect coverage data from .gcov files in the build directory."""
        bdir = os.path.abspath(build_dir or self.build_dir)
        self._generate_gcov(bdir)

        coverage: Dict[str, FileCoverage] = {}
        for root, dirs, files in os.walk(bdir):
            dirs.sort()
            for fname in sorted(files):
                if not fname.endswith(".gcov"):
                    continue
                gcov_path = os.path.join(root, fname)
                file_cov = self.parse_gcov_file(gcov_path)
                if not (file_cov.lines or file_cov.branches or file_cov.functions):
                    continue
                source_path = self._normalize_source(file_cov.path, bdir, gcov_path)
                file_cov.path = source_path
                coverage[source_path] = file_cov

        return coverage

    def _normalize_source(self, declared: str, bdir: str, gcov_path: str) -> str:
        """Map a gcov-declared Source path to a repo-relative source path."""
        if not declared:
            # Fall back to the build-relative location of the .gcov file.
            rel = os.path.relpath(gcov_path, bdir)
            return rel[:-5] if rel.endswith(".gcov") else rel

        candidate = declared.replace("\\", "/")
        # Absolute path that includes the source dir -> make relative.
        if os.path.isabs(candidate) and candidate.startswith(self.source_dir):
            candidate = os.path.relpath(candidate, self.source_dir)
        # Source file might be referenced without extension inside build tree.
        if not os.path.isfile(os.path.join(self.source_dir, candidate)):
            base, _ = os.path.splitext(candidate)
            for ext in (".c", ".h", ".S", ".rs"):
                if os.path.isfile(os.path.join(self.source_dir, base + ext)):
                    candidate = base + ext
                    break
        return candidate
