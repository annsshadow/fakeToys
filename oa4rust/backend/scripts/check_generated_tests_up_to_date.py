#!/usr/bin/env python3
# Copyright (C) 2026 annsshadow
# SPDX-License-Identifier: AGPL-3.0-or-later
"""O14 门禁：tests_generated.rs 必须与生成器输出逐字一致。

做法：把生成器跑到临时目录（不改工作树），再与已提交产物逐字节比对。
不一致 → exit 1，并打印第一处差异的 crate / 行号。

同时守住几条“生成物不能被绕过”的硬约束：
  1. 生成器本身必须被 git 跟踪（openapi-paths 的教训：脚本没入库 = 门禁空转）。
  2. 每个产物 crate 的 lib.rs 必须由生成器写入了 `mod tests_generated;`。
  3. 集合规模下限：产物文件数 ≥ 90、`#[tokio::test]` 总数 ≥ 1200。
     防止生成器某次改动把整批用例“生成”成空壳后，逐字比对反而全绿。
"""

import os
import subprocess
import sys
import tempfile

BACKEND_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
GENERATOR = os.path.join(BACKEND_DIR, 'scripts', 'generate_handler_tests.py')
CRATES_DIR = os.path.join(BACKEND_DIR, 'crates')

MIN_FILES = 90
MIN_TESTS = 1200

failures = []


def fail(msg):
    failures.append(msg)


def rel(path):
    return os.path.relpath(path, BACKEND_DIR).replace('\\', '/')


def main():
    if not os.path.isfile(GENERATOR):
        fail(f'生成器不存在：{rel(GENERATOR)}')
        return

    # 1) 生成器必须被 git 跟踪（干净克隆 / CI 上不能是某个人的本地文件）。
    root_run = subprocess.run(['git', 'rev-parse', '--show-toplevel'],
                              cwd=BACKEND_DIR, capture_output=True, text=True)
    if root_run.returncode != 0:
        fail('无法定位 git 仓库根目录（生成器跟踪检查无法执行）')
    else:
        repo_root = root_run.stdout.strip()
        tracked = subprocess.run(
            ['git', 'ls-files', '--error-unmatch',
             'oa4rust/backend/scripts/generate_handler_tests.py'],
            cwd=repo_root, capture_output=True, text=True)
        if tracked.returncode != 0:
            fail('generate_handler_tests.py 未被 git 跟踪（干净克隆 / CI 上会空转）')

    with tempfile.TemporaryDirectory(prefix='o14-gen-') as out_dir:
        run = subprocess.run(
            [sys.executable, GENERATOR, '--out-dir', out_dir],
            cwd=BACKEND_DIR, capture_output=True, text=True)
        if run.returncode != 0:
            fail(f'生成器执行失败（exit={run.returncode}）：{(run.stderr or run.stdout).strip()[-400:]}')
            return

        generated = {}
        for root, _, files in os.walk(out_dir):
            for name in files:
                if name == 'tests_generated.rs':
                    crate = os.path.basename(root)
                    generated[crate] = os.path.join(root, name)

        committed = {}
        for crate in os.listdir(CRATES_DIR):
            path = os.path.join(CRATES_DIR, crate, 'src', 'tests_generated.rs')
            if os.path.isfile(path):
                committed[crate] = path

        missing = sorted(set(committed) - set(generated))
        extra = sorted(set(generated) - set(committed))
        for crate in missing:
            fail(f'{crate}: 已提交产物存在，但生成器不再产出（产物被手工创建？）')
        for crate in extra:
            fail(f'{crate}: 生成器产出了文件，但仓库里没有（未提交的产物）')

        total_tests = 0
        for crate in sorted(set(generated) & set(committed)):
            with open(generated[crate], 'rb') as f:
                gen_bytes = f.read()
            with open(committed[crate], 'rb') as f:
                repo_bytes = f.read()
            total_tests += gen_bytes.count(b'#[tokio::test]')
            if gen_bytes != repo_bytes:
                gen_lines = gen_bytes.decode('utf-8', 'replace').split('\n')
                repo_lines = repo_bytes.decode('utf-8', 'replace').split('\n')
                for i in range(max(len(gen_lines), len(repo_lines))):
                    g = gen_lines[i] if i < len(gen_lines) else '<EOF>'
                    r = repo_lines[i] if i < len(repo_lines) else '<EOF>'
                    if g != r:
                        fail(f'{crate}: 产物与生成器不一致（首个差异在第 {i + 1} 行）\n'
                             f'    committed: {r}\n'
                             f'    generated: {g}')
                        break

            lib_rs = os.path.join(CRATES_DIR, crate, 'src', 'lib.rs')
            if os.path.isfile(lib_rs):
                with open(lib_rs, encoding='utf-8') as f:
                    if 'mod tests_generated;' not in f.read():
                        fail(f'{crate}: lib.rs 缺少 `mod tests_generated;` 声明（应由生成器负责）')

        # 3) 集合规模下限：防止“生成器产出空壳但逐字比对绿”。
        if len(generated) < MIN_FILES:
            fail(f'产物数 {len(generated)} 低于下限 {MIN_FILES}（生成器可能只产出了空壳）')
        if total_tests < MIN_TESTS:
            fail(f'#[tokio::test] 总数 {total_tests} 低于下限 {MIN_TESTS}（测试集合异常收缩）')

    if failures:
        print('tests_generated.rs 与生成器不一致：')
        for msg in failures:
            print('  ✗ ' + msg)
        sys.exit(1)

    print(f'OK: {len(generated)} 个产物与生成器逐字一致（{total_tests} 条 tokio 测试，'
          f'下限 {MIN_TESTS}）')


if __name__ == '__main__':
    main()
