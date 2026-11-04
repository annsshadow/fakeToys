#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""O14 生成物清单审计后的补充门禁：其余 Python 生成器的产物漂移检查。

覆盖对象（与 check_openapi_up_to_date.py / check_generated_tests_up_to_date.py
不重叠）：
  - scripts/gen_mcp_tools.py → crates/mcp_server/src/generated_routes.rs
  - scripts/generate_parity_tests.py → crates/parity/src/generated_tests.rs
    （输入 docs/audits/o2server-parity-report.json 已随本门禁一并入库）

判据：快照 → 就地重生成 → 逐字节比对（仅归一化 CRLF/LF）→ 写回快照。
退出码：0 一致；1 漂移；2 生成器执行失败。
"""

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
JOBS = [
    {
        'name': 'gen_mcp_tools',
        'gen': ROOT / 'scripts' / 'gen_mcp_tools.py',
        'targets': [
            ROOT / 'crates' / 'mcp_server' / 'src' / 'generated_routes.rs',
            ROOT.parent / 'tests' / 'contracts' / 'backend-route-methods.fixture.ts',
        ],
    },
    {
        'name': 'generate_parity_tests',
        'gen': ROOT / 'scripts' / 'generate_parity_tests.py',
        'targets': [ROOT / 'crates' / 'parity' / 'src' / 'generated_tests.rs'],
    },
]


def normalized(data: bytes) -> bytes:
    return data.replace(b'\r\n', b'\n')


def main() -> int:
    for job in JOBS:
        if not job['gen'].exists():
            print(f'{job["name"]}: 生成器不存在（{job["gen"]}）')
            return 1
        missing = [p for p in job['targets'] if not p.exists()]
        if missing:
            for p in missing:
                print(f'生成物缺失：{p.relative_to(ROOT).as_posix()}')
            return 1

        snapshots = {p: p.read_bytes() for p in job['targets']}
        try:
            proc = subprocess.run(
                [sys.executable, str(job['gen'])],
                capture_output=True, text=True, cwd=str(job['gen'].parent))
            if proc.returncode != 0:
                print(f'{job["name"]}.py 执行失败（退出码 {proc.returncode}）')
                print((proc.stdout or '') + (proc.stderr or ''))
                return 2

            stale = [(p, p.relative_to(ROOT.parent).as_posix())
                     for p, before in snapshots.items()
                     if normalized(p.read_bytes()) != normalized(before)]
            if stale:
                print(f'{job["name"]}: 生成物与生成器不一致：' + ', '.join(rel for _, rel in stale))
                for path, rel in stale:
                    committed = normalized(snapshots[path]).split(b'\n')
                    generated = normalized(path.read_bytes()).split(b'\n')
                    for lineno, (a, b) in enumerate(zip(committed, generated), 1):
                        if a != b:
                            print(f'首个差异：{rel}:{lineno}')
                            print(f'  已提交: {a.decode("utf-8", "replace")[:120]}')
                            print(f'  重新生成: {b.decode("utf-8", "replace")[:120]}')
                            break
                print(f'修复：cd oa4rust/backend && python scripts/{job["name"]}.py（并提交生成物）')
                return 1
        finally:
            for p, before in snapshots.items():
                if p.read_bytes() != before:
                    p.write_bytes(before)

    print('generated-artifacts: 其余 Python 生成器产物一致（%d 组）' % len(JOBS))
    return 0


if __name__ == '__main__':
    sys.exit(main())
