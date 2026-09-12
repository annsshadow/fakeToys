#!/usr/bin/env python3
"""Evaluate an OA4Rust S4 pilot window from a dedicated Nginx access log."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from decimal import Decimal, InvalidOperation, ROUND_FLOOR
from pathlib import Path
from typing import Iterable
from urllib.parse import urlsplit

NGINX_ACCESS_RE = re.compile(
    r'^\S+\s+\S+\s+\S+\s+\[(?P<timestamp>[^]]+)]\s+'
    r'"[^"]*"\s+(?P<status>[1-5][0-9]{2})\s+(?:[0-9]+|-)(?:\s+.*)?$'
)


class GateInputError(ValueError):
    """Raised when gate inputs cannot produce a trustworthy result."""


@dataclass(frozen=True)
class GateConfig:
    service_url: str
    window_start: datetime
    window_end: datetime
    max_5xx_rate_percent: Decimal
    min_requests: int
    pilot_users: tuple[str, ...]
    version: str


@dataclass(frozen=True)
class Evaluation:
    nonempty_lines: int
    parsed_requests: int
    outside_window_requests: int
    request_count: int
    server_error_count: int
    status_classes: dict[str, int]
    server_errors_by_status: dict[str, int]
    malformed_line_numbers: tuple[int, ...]
    first_request_at: datetime | None
    last_request_at: datetime | None
    allowed_server_errors: int
    failures: tuple[str, ...]

    @property
    def passed(self) -> bool:
        return not self.failures

def parse_utc(value: str) -> datetime:
    raw = value.strip()
    if raw.endswith("Z"):
        raw = raw[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(raw)
    except ValueError as exc:
        raise GateInputError(f"invalid ISO-8601 timestamp: {value}") from exc
    if parsed.tzinfo is None:
        raise GateInputError(f"timestamp must include a UTC offset: {value}")
    return parsed.astimezone(timezone.utc)


def format_utc(value: datetime | None) -> str | None:
    if value is None:
        return None
    return value.astimezone(timezone.utc).isoformat().replace("+00:00", "Z")


def parse_rate(value: str) -> Decimal:
    try:
        rate = Decimal(value)
    except InvalidOperation as exc:
        raise GateInputError(f"invalid 5xx rate percent: {value}") from exc
    if not rate.is_finite() or rate < 0 or rate > 100:
        raise GateInputError("5xx rate percent must be between 0 and 100")
    return rate


def validate_service_url(value: str) -> str:
    parsed = urlsplit(value)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise GateInputError("service URL must be an absolute http(s) URL")
    return value.rstrip("/")


def parse_access_line(line: str) -> tuple[datetime, int] | None:
    match = NGINX_ACCESS_RE.match(line)
    if not match:
        return None
    try:
        timestamp = datetime.strptime(
            match.group("timestamp"), "%d/%b/%Y:%H:%M:%S %z"
        ).astimezone(timezone.utc)
    except ValueError:
        return None
    return timestamp, int(match.group("status"))


def evaluate(lines: Iterable[str], config: GateConfig) -> Evaluation:
    nonempty_lines = 0
    parsed_requests = 0
    outside_window_requests = 0
    request_count = 0
    server_error_count = 0
    status_classes = {f"{number}xx": 0 for number in range(1, 6)}
    server_errors_by_status: dict[str, int] = {}
    malformed_line_numbers: list[int] = []
    first_request_at: datetime | None = None
    last_request_at: datetime | None = None

    for line_number, raw_line in enumerate(lines, start=1):
        line = raw_line.rstrip("\r\n")
        if not line.strip():
            continue
        nonempty_lines += 1
        parsed = parse_access_line(line)
        if parsed is None:
            malformed_line_numbers.append(line_number)
            continue
        parsed_requests += 1
        timestamp, status = parsed
        if not config.window_start <= timestamp < config.window_end:
            outside_window_requests += 1
            continue
        request_count += 1
        status_classes[f"{status // 100}xx"] += 1
        first_request_at = min(first_request_at, timestamp) if first_request_at else timestamp
        last_request_at = max(last_request_at, timestamp) if last_request_at else timestamp
        if 500 <= status <= 599:
            server_error_count += 1
            key = str(status)
            server_errors_by_status[key] = server_errors_by_status.get(key, 0) + 1

    allowed_server_errors = int(
        (Decimal(request_count) * config.max_5xx_rate_percent / Decimal(100)).to_integral_value(
            rounding=ROUND_FLOOR
        )
    )
    failures: list[str] = []
    if malformed_line_numbers:
        preview = ",".join(str(number) for number in malformed_line_numbers[:10])
        suffix = "..." if len(malformed_line_numbers) > 10 else ""
        failures.append(
            f"access log has {len(malformed_line_numbers)} malformed non-empty line(s): {preview}{suffix}"
        )
    if request_count < config.min_requests:
        failures.append(
            f"request count {request_count} is below required minimum {config.min_requests}"
        )
    if server_error_count > allowed_server_errors:
        failures.append(
            f"5xx count {server_error_count} exceeds budget {allowed_server_errors} "
            f"at {config.max_5xx_rate_percent}%"
        )

    return Evaluation(
        nonempty_lines=nonempty_lines,
        parsed_requests=parsed_requests,
        outside_window_requests=outside_window_requests,
        request_count=request_count,
        server_error_count=server_error_count,
        status_classes=status_classes,
        server_errors_by_status=server_errors_by_status,
        malformed_line_numbers=tuple(malformed_line_numbers),
        first_request_at=first_request_at,
        last_request_at=last_request_at,
        allowed_server_errors=allowed_server_errors,
        failures=tuple(failures),
    )

def build_report(
    config: GateConfig,
    evaluation: Evaluation,
    access_log: Path,
    source_sha256: str,
    generated_at: datetime,
) -> dict[str, object]:
    actual_rate = (
        Decimal(evaluation.server_error_count) * Decimal(100) / Decimal(evaluation.request_count)
        if evaluation.request_count
        else None
    )
    checklist = [
        {
            "id": "pilot-users-confirmed",
            "prompt": "确认报告中的 pilot 用户均知情且仅使用新栈。",
            "completed": False,
        },
        {
            "id": "critical-journeys-sampled",
            "prompt": "抽检登录、创建/保存、执行/审批和查询等本次 pilot 关键旅程。",
            "completed": False,
        },
        {
            "id": "sampled-data-correct",
            "prompt": "核对抽样记录的页面结果、状态变化和持久化数据正确。",
            "completed": False,
        },
        {
            "id": "support-feedback-reviewed",
            "prompt": "复核 pilot 用户反馈与运维告警，记录未解决问题及负责人。",
            "completed": False,
        },
        {
            "id": "rollback-readiness-confirmed",
            "prompt": "确认停止放大、隔离 pilot 和版本回退步骤仍可执行。",
            "completed": False,
        },
    ]
    return {
        "schema_version": 1,
        "gate": "oa4rust-s4-pilot",
        "decision": "pass" if evaluation.passed else "fail",
        "generated_at": format_utc(generated_at),
        "pilot": {
            "service_url": config.service_url,
            "users": list(config.pilot_users),
            "version": config.version,
            "window_start": format_utc(config.window_start),
            "window_end": format_utc(config.window_end),
        },
        "source": {
            "type": "nginx_combined_access_log",
            "path": str(access_log),
            "sha256": source_sha256,
            "nonempty_lines": evaluation.nonempty_lines,
            "parsed_requests": evaluation.parsed_requests,
            "outside_window_requests": evaluation.outside_window_requests,
            "malformed_line_count": len(evaluation.malformed_line_numbers),
            "malformed_line_numbers": list(evaluation.malformed_line_numbers),
        },
        "observed": {
            "request_count": evaluation.request_count,
            "server_error_count": evaluation.server_error_count,
            "server_error_rate_percent": float(actual_rate) if actual_rate is not None else None,
            "status_classes": evaluation.status_classes,
            "server_errors_by_status": evaluation.server_errors_by_status,
            "first_request_at": format_utc(evaluation.first_request_at),
            "last_request_at": format_utc(evaluation.last_request_at),
        },
        "budget": {
            "max_5xx_rate_percent": float(config.max_5xx_rate_percent),
            "allowed_server_error_count": evaluation.allowed_server_errors,
            "remaining_server_error_count": evaluation.allowed_server_errors
            - evaluation.server_error_count,
            "min_requests": config.min_requests,
        },
        "failures": list(evaluation.failures),
        "manual_checklist": checklist,
        "manual_checklist_status": "pending",
        "external_execution_required": True,
        "actual_pilot_claimed": False,
    }


def file_sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_checklist(path: Path, report: dict[str, object]) -> None:
    pilot = report["pilot"]
    observed = report["observed"]
    budget = report["budget"]
    assert isinstance(pilot, dict) and isinstance(observed, dict) and isinstance(budget, dict)
    lines = [
        "# OA4Rust S4 pilot 人工抽检清单",
        "",
        "> 自动门禁只计算访问日志中的请求量与 5xx 预算；以下项目必须由现场人员完成。",
        "",
        f"- 服务 URL：`{pilot['service_url']}`",
        f"- pilot 用户：`{', '.join(pilot['users'])}`",
        f"- 版本：`{pilot['version']}`",
        f"- 观察窗口：`{pilot['window_start']}` 至 `{pilot['window_end']}`（结束时刻不含）",
        f"- 自动门禁：**{str(report['decision']).upper()}**",
        f"- 请求数 / 5xx：`{observed['request_count']}` / `{observed['server_error_count']}`",
        f"- 允许 5xx：`{budget['allowed_server_error_count']}`（阈值 `{budget['max_5xx_rate_percent']}%`）",
        "",
        "## 抽检项",
        "",
    ]
    for item in report["manual_checklist"]:
        lines.append(f"- [ ] {item['prompt']} 证据/工单：________________")
    lines.extend(
        [
            "",
            "## 人工结论",
            "",
            "- [ ] 自动门禁 PASS，且以上抽检全部完成后，批准进入下一放大阶段。",
            "- [ ] 任一项失败或证据缺失，停止放大并记录处置。",
            "- 审核人：________________",
            "- 审核时间（UTC）：________________",
            "- 结论/备注：________________",
            "",
            "> 此文件由脚本生成时所有复选框均为未完成；生成本身不代表 pilot 已通过。",
        ]
    )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")

def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Evaluate an OA4Rust S4 pilot window from an Nginx access log."
    )
    parser.add_argument("--service-url", required=True)
    parser.add_argument("--window-start", required=True, help="inclusive ISO-8601 time")
    parser.add_argument("--window-end", required=True, help="exclusive ISO-8601 time")
    parser.add_argument("--max-5xx-rate-percent", required=True)
    parser.add_argument("--min-requests", required=True, type=int)
    parser.add_argument("--pilot-user", required=True, action="append", dest="pilot_users")
    parser.add_argument("--version", required=True)
    parser.add_argument("--access-log", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--checklist-output", required=True, type=Path)
    return parser


def config_from_args(args: argparse.Namespace) -> GateConfig:
    if args.min_requests < 1:
        raise GateInputError("minimum request count must be at least 1")
    window_start = parse_utc(args.window_start)
    window_end = parse_utc(args.window_end)
    if window_start >= window_end:
        raise GateInputError("window end must be later than window start")
    pilot_users = tuple(user.strip() for user in args.pilot_users if user.strip())
    if not pilot_users:
        raise GateInputError("at least one non-empty pilot user is required")
    if not args.version.strip():
        raise GateInputError("version must not be empty")
    return GateConfig(
        service_url=validate_service_url(args.service_url),
        window_start=window_start,
        window_end=window_end,
        max_5xx_rate_percent=parse_rate(args.max_5xx_rate_percent),
        min_requests=args.min_requests,
        pilot_users=pilot_users,
        version=args.version.strip(),
    )


def run(argv: list[str] | None = None) -> int:
    args = build_parser().parse_args(argv)
    try:
        config = config_from_args(args)
        if not args.access_log.is_file():
            raise GateInputError(f"access log does not exist or is not a file: {args.access_log}")
        source_sha256 = file_sha256(args.access_log)
        with args.access_log.open("r", encoding="utf-8") as handle:
            evaluation = evaluate(handle, config)
        report = build_report(
            config=config,
            evaluation=evaluation,
            access_log=args.access_log,
            source_sha256=source_sha256,
            generated_at=datetime.now(timezone.utc),
        )
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(
            json.dumps(report, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        write_checklist(args.checklist_output, report)
    except (GateInputError, OSError, UnicodeError) as exc:
        print(f"pilot gate input error: {exc}", file=sys.stderr)
        return 2

    print(
        f"pilot gate {report['decision'].upper()}: "
        f"requests={evaluation.request_count}, 5xx={evaluation.server_error_count}, "
        f"budget={evaluation.allowed_server_errors}; report={args.output}"
    )
    if evaluation.failures:
        for failure in evaluation.failures:
            print(f"failure: {failure}", file=sys.stderr)
    return 0 if evaluation.passed else 1


if __name__ == "__main__":
    sys.exit(run())
