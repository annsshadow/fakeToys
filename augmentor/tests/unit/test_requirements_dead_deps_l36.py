# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later

"""第 36 轮：主 requirements 不再声明零消费依赖。

- `google-generativeai` 全仓零 import（gemini 后端走 httpx REST）—— 已删；
- `nlpaug` / `nlpcda` 只被 archive/enhanceTXT.py（不在包内）引用 ——
  已移至 requirements-legacy.txt。
本守卫防止「顺手加回去」的回归：新依赖可以加，但死依赖清单要保持干净。
"""

from pathlib import Path

REQS = Path(__file__).resolve().parents[2] / "requirements.txt"
LEGACY = Path(__file__).resolve().parents[2] / "requirements-legacy.txt"


def test_main_requirements_has_no_dead_deps():
    lines = REQS.read_text(encoding="utf-8").splitlines()
    # 只查依赖行（不以 # 开头）：注释里允许解释历史，但声明行不能复活。
    dep_lines = [ln for ln in lines if ln.strip() and not ln.lstrip().startswith("#")]
    joined = "\n".join(dep_lines)
    for dead in ("google-generativeai", "nlpaug", "nlpcda"):
        assert dead not in joined, (
            f"主 requirements.txt 不应再声明 {dead}（零消费依赖，见第 36 轮）"
        )


def test_legacy_requirements_holds_archive_deps():
    text = LEGACY.read_text(encoding="utf-8")
    assert "nlpaug" in text
    assert "nlpcda" in text