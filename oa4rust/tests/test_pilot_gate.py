#!/usr/bin/env python3
"""Unit tests for the OA4Rust S4 pilot gate."""

from __future__ import annotations

import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from datetime import datetime, timezone
from decimal import Decimal
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "pilot_gate.py"
FIXTURE = ROOT / "tests" / "fixtures" / "pilot_access.fixture"
SPEC = importlib.util.spec_from_file_location("pilot_gate", SCRIPT)
assert SPEC and SPEC.loader
pilot_gate = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = pilot_gate
SPEC.loader.exec_module(pilot_gate)


class PilotGateTests(unittest.TestCase):
    def config(self, rate: str = "20", min_requests: int = 10):
        return pilot_gate.GateConfig(
            service_url="https://pilot.example.test",
            window_start=datetime(2026, 9, 12, 10, tzinfo=timezone.utc),
            window_end=datetime(2026, 9, 12, 11, tzinfo=timezone.utc),
            max_5xx_rate_percent=Decimal(rate),
            min_requests=min_requests,
            pilot_users=("alice", "bob"),
            version="git-deadbeef",
        )

    def test_fixture_counts_only_half_open_observation_window(self):
        with FIXTURE.open(encoding="utf-8") as handle:
            result = pilot_gate.evaluate(handle, self.config())

        self.assertTrue(result.passed)
        self.assertEqual(result.request_count, 10)
        self.assertEqual(result.server_error_count, 2)
        self.assertEqual(result.allowed_server_errors, 2)
        self.assertEqual(result.outside_window_requests, 2)
        self.assertEqual(result.status_classes["5xx"], 2)
        self.assertEqual(result.server_errors_by_status, {"500": 1, "503": 1})

    def test_budget_uses_floor_and_fails_when_5xx_exceeds_it(self):
        with FIXTURE.open(encoding="utf-8") as handle:
            result = pilot_gate.evaluate(handle, self.config(rate="19.99"))

        self.assertFalse(result.passed)
        self.assertEqual(result.allowed_server_errors, 1)
        self.assertIn("5xx count 2 exceeds budget 1", result.failures[0])

    def test_malformed_nonempty_line_fails_loud(self):
        lines = [
            '10.0.0.1 - u [12/Sep/2026:10:00:00 +0000] "GET / HTTP/1.1" 200 1\n',
            "not an access log line\n",
        ]
        result = pilot_gate.evaluate(lines, self.config(min_requests=1))

        self.assertFalse(result.passed)
        self.assertEqual(result.malformed_line_numbers, (2,))
        self.assertTrue(result.failures[0].startswith("access log has 1 malformed"))

    def test_cli_writes_machine_report_and_pending_checklist(self):
        with tempfile.TemporaryDirectory() as directory:
            report_path = Path(directory) / "report.json"
            checklist_path = Path(directory) / "checklist.md"
            command = [
                sys.executable,
                str(SCRIPT),
                "--service-url",
                "https://pilot.example.test",
                "--window-start",
                "2026-09-12T10:00:00Z",
                "--window-end",
                "2026-09-12T11:00:00Z",
                "--max-5xx-rate-percent",
                "20",
                "--min-requests",
                "10",
                "--pilot-user",
                "alice",
                "--pilot-user",
                "bob",
                "--version",
                "git-deadbeef",
                "--access-log",
                str(FIXTURE),
                "--output",
                str(report_path),
                "--checklist-output",
                str(checklist_path),
            ]
            completed = subprocess.run(command, capture_output=True, text=True, check=False)

            self.assertEqual(completed.returncode, 0, completed.stderr)
            report = json.loads(report_path.read_text(encoding="utf-8"))
            self.assertEqual(report["decision"], "pass")
            self.assertEqual(report["pilot"]["users"], ["alice", "bob"])
            self.assertEqual(report["pilot"]["version"], "git-deadbeef")
            self.assertEqual(report["observed"]["request_count"], 10)
            self.assertEqual(report["observed"]["server_error_count"], 2)
            self.assertEqual(report["manual_checklist_status"], "pending")
            self.assertTrue(report["external_execution_required"])
            self.assertFalse(report["actual_pilot_claimed"])
            checklist = checklist_path.read_text(encoding="utf-8")
            self.assertIn("- [ ]", checklist)
            self.assertIn("生成本身不代表 pilot 已通过", checklist)


if __name__ == "__main__":
    unittest.main()
