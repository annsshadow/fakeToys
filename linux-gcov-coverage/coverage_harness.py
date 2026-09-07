#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""
Coverage harness - main entry point for kernel coverage collection.

Orchestrates gcov/kcov configuration, data collection, and report generation.
"""

import argparse
import os
import subprocess
import sys
import time
from typing import Dict, List, Optional

from gcov_parser import GcovParser
from kcov_parser import KcovParser
from report_generator import ReportGenerator


class CoverageHarness:
    """Main coverage collection orchestrator."""

    def __init__(self, source_dir: str, build_dir: str):
        self.source_dir = os.path.abspath(source_dir)
        self.build_dir = os.path.abspath(build_dir)
        self.gcov_parser = GcovParser(build_dir, source_dir)
        self.kcov_parser = KcovParser()
        self.report_gen = ReportGenerator(source_dir)

    def setup_gcov_config(self, config_path: Optional[str] = None) -> bool:
        """Configure kernel for gcov coverage collection."""
        print("Configuring kernel for gcov branch coverage...")

        # Enable required configs
        gcov_configs = [
            "CONFIG_GCOV_KERNEL=y",
            "CONFIG_GCOV_PROFILE_ALL=y",
            "CONFIG_DEBUG_FS=y",
            "CONFIG_DEBUG_INFO=y",
        ]

        if config_path and os.path.isfile(config_path):
            # Merge with existing config
            with open(config_path, "r", encoding="utf-8") as f:
                existing = f.read()
        else:
            existing = ""

        new_config = existing + "\n" + "\n".join(gcov_configs) + "\n"

        if config_path:
            os.makedirs(os.path.dirname(config_path) if os.path.dirname(config_path) else ".", exist_ok=True)
            with open(config_path, "w", encoding="utf-8") as f:
                f.write(new_config)
        else:
            # Merge into .config
            config_file = os.path.join(self.build_dir, ".config")
            with open(config_file, "a", encoding="utf-8") as f:
                f.write("\n" + "\n".join(gcov_configs) + "\n")

        return True

    def setup_kcov_config(self, config_path: Optional[str] = None) -> bool:
        """Configure kernel for kcov coverage collection."""
        print("Configuring kernel for kcov coverage...")

        kcov_configs = [
            "CONFIG_KCOV=y",
            "CONFIG_KCOV_INSTRUMENT_ALL=y",
            "CONFIG_DEBUG_FS=y",
        ]

        config_file = os.path.join(self.build_dir, ".config")
        with open(config_file, "a", encoding="utf-8") as f:
            f.write("\n" + "\n".join(kcov_configs) + "\n")

        return True

    def collect_gcov_data(self) -> Dict[str, object]:
        """Collect gcov coverage data after test run."""
        print("Collecting gcov coverage data...")

        # Mount debugfs if needed
        debugfs_mount = "/sys/kernel/debug"
        if not os.path.ismount(debugfs_mount):
            try:
                subprocess.run(["mount", "-t", "debugfs", "none", debugfs_mount],
                             check=False, capture_output=True)
            except (subprocess.CalledProcessError, FileNotFoundError):
                print("WARNING: Could not mount debugfs", file=sys.stderr)

        # Find .gcda files in build directory
        gcda_files = []
        for root, dirs, files in os.walk(self.build_dir):
            dirs.sort()
            for f in sorted(files):
                if f.endswith(".gcda"):
                    gcda_files.append(os.path.join(root, f))

        print(f"Found {len(gcda_files)} .gcda files")

        # Parse coverage data
        coverage = self.gcov_parser.collect_coverage(self.build_dir)
        return coverage

    def collect_kcov_data(self) -> Dict[str, object]:
        """Collect kcov coverage data."""
        print("Collecting kcov coverage data...")

        kcov_path = "/sys/kernel/debug/kcov"
        if not os.path.exists(kcov_path):
            print("WARNING: kcov not available at " + kcov_path, file=sys.stderr)
            return {}

        # Read coverage from kcov
        coverage_data = self.kcov_parser.read_coverage(0)
        return {"pc_set": coverage_data.pc_set}

    def generate_report(self, coverage: Dict[str, object],
                       output_dir: str, format: str = "lcov") -> str:
        """Generate coverage report in specified format."""
        print(f"Generating {format} report...")

        os.makedirs(output_dir, exist_ok=True)

        if format == "lcov":
            lcov_path = os.path.join(output_dir, "coverage.info")
            self.report_gen.generate_lcov(coverage, lcov_path)
            self.report_gen.print_summary(coverage)
            return lcov_path
        elif format == "html":
            self.report_gen.generate_html(coverage, output_dir)
            self.report_gen.print_summary(coverage)
            return output_dir
        else:
            print(f"Unknown format: {format}", file=sys.stderr)
            return ""

    def run(self, test_command: str, output_dir: str,
            tool: str = "gcov", format: str = "lcov") -> int:
        """Run full coverage collection pipeline."""
        print(f"Starting coverage collection with {tool}...")
        start_time = time.time()

        # Setup
        if tool == "gcov":
            self.setup_gcov_config()
        elif tool == "kcov":
            self.setup_kcov_config()

        # Build kernel
        print("Building kernel with coverage enabled...")
        result = subprocess.run(
            ["make", "-C", self.source_dir, "O=" + self.build_dir, "-j$(nproc)"],
            capture_output=True, shell=False
        )
        if result.returncode != 0:
            print("Build failed:", result.stderr.decode("utf-8", errors="replace"),
                  file=sys.stderr)
            return 1

        # Run tests
        print(f"Running tests: {test_command}")
        test_result = subprocess.run(
            test_command, shell=True, capture_output=True
        )
        if test_result.returncode != 0:
            print("Tests failed:", test_result.stderr.decode("utf-8", errors="replace"),
                  file=sys.stderr)

        # Collect coverage
        if tool == "gcov":
            coverage = self.collect_gcov_data()
        else:
            coverage = self.collect_kcov_data()

        # Generate report
        self.generate_report(coverage, output_dir, format)

        elapsed = time.time() - start_time
        print(f"\nCoverage collection completed in {elapsed:.1f}s")
        return 0


def main():
    parser = argparse.ArgumentParser(
        description="Kernel branch coverage collection harness"
    )
    parser.add_argument("--srcdir", default=".",
                       help="Kernel source directory (default: current directory)")
    parser.add_argument("--builddir", default="build",
                       help="Build directory (default: build)")
    parser.add_argument("--tool", choices=["gcov", "kcov"], default="gcov",
                       help="Coverage tool (default: gcov)")
    parser.add_argument("--format", choices=["lcov", "html"], default="lcov",
                       help="Report format (default: lcov)")
    parser.add_argument("--output", default="coverage_report",
                       help="Output directory (default: coverage_report)")
    parser.add_argument("--test", default="",
                       help="Test command to run (e.g., 'make kselftest')")
    parser.add_argument("--config", default=None,
                       help="Kernel config fragment path")
    parser.add_argument("--collect-only", action="store_true",
                       help="Only collect coverage, skip test run")

    args = parser.parse_args()

    harness = CoverageHarness(args.srcdir, args.builddir)

    if args.collect_only:
        if args.tool == "gcov":
            coverage = harness.collect_gcov_data()
        else:
            coverage = harness.collect_kcov_data()
        harness.generate_report(coverage, args.output, args.format)
    elif args.test:
        sys.exit(harness.run(args.test, args.output, args.tool, args.format))
    else:
        # Just setup config
        if args.tool == "gcov":
            harness.setup_gcov_config(args.config)
        else:
            harness.setup_kcov_config(args.config)
        print("Configuration complete.")


if __name__ == "__main__":
    main()
