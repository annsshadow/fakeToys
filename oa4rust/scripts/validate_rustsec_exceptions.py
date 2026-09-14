#!/usr/bin/env python3
from __future__ import annotations

import argparse
import datetime as dt
import re
import sys
import tomllib
from pathlib import Path

REQUIRED = {
    "advisory",
    "package",
    "version",
    "dependency_path",
    "owner",
    "tracking_url",
    "created",
    "expires",
    "reason",
    "compensating_control",
    "approved_by",
}


def validate(path: Path) -> list[str]:
    data = tomllib.loads(path.read_text(encoding="utf-8"))
    exceptions = data.get("exception", [])
    errors: list[str] = []
    today = dt.date.today()
    for index, item in enumerate(exceptions, 1):
        missing = sorted(REQUIRED - item.keys())
        if missing:
            errors.append(f"exception {index}: missing {', '.join(missing)}")
            continue
        advisory = item["advisory"]
        if not re.fullmatch(r"RUSTSEC-\d{4}-\d{4}", advisory):
            errors.append(f"exception {index}: invalid advisory {advisory!r}")
        try:
            created = dt.date.fromisoformat(item["created"])
            expires = dt.date.fromisoformat(item["expires"])
        except (TypeError, ValueError) as error:
            errors.append(f"exception {index}: invalid date: {error}")
            continue
        if expires <= today:
            errors.append(f"exception {index}: expired on {expires}")
        if expires > created + dt.timedelta(days=30):
            errors.append(f"exception {index}: lifetime exceeds 30 days")
        for field in REQUIRED - {"created", "expires"}:
            if not isinstance(item[field], str) or not item[field].strip():
                errors.append(f"exception {index}: {field} must be non-empty")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("path", type=Path)
    parser.add_argument("--print-args", action="store_true")
    args = parser.parse_args()
    errors = validate(args.path)
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    data = tomllib.loads(args.path.read_text(encoding="utf-8"))
    if args.print_args:
        print(" ".join(f"--ignore {item['advisory']}" for item in data.get("exception", [])))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
