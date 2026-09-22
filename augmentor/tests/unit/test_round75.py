"""第75轮: quality_gate质量门控"""
import pytest
from augmentor.quality_gate import QualityGate, build_default_gate


class TestQualityGate:
    def test_build_default_gate(self):
        gate = build_default_gate()
        assert gate is not None

    def test_has_run_method(self):
        gate = build_default_gate()
        assert hasattr(gate, 'run') or hasattr(gate, 'check')

    def test_gate_report(self):
        gate = build_default_gate()
        items = [{"instruction": "test", "output": "out"}]
        result = gate.run(items) if hasattr(gate, 'run') else gate.check(items)
        assert result is not None
