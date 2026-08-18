#!/usr/bin/env bash
set -euo pipefail

IMAGE="kernel-coverage-baseline:latest"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# coverage 目录在 <repo>/tools/testing/coverage,需向上三级才到仓库根
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Docker Desktop 守护进程运行在 Windows 上,只认 Windows 原生路径,
# 不接受 Git Bash 的 /d/... 路径。cygpath -w 把 MSYS 路径翻译成 C:\... 形式。
winpath() { cygpath -w "$1" 2>/dev/null || printf '%s' "$1"; }
SCRIPT_DIR_WIN="$(winpath "$SCRIPT_DIR")"
REPO_ROOT_WIN="$(winpath "$REPO_ROOT")"

docker build -t "$IMAGE" -f "$SCRIPT_DIR_WIN/Dockerfile.baseline" "$SCRIPT_DIR_WIN"

docker run --rm --privileged \
  -v "$REPO_ROOT_WIN:/work" \
  -w /work \
  "$IMAGE" \
  bash -c "mkdir -p arch/um/include/shared && ln -sf ../../x86/um/shared/sysdep arch/um/include/shared/sysdep && python3 tools/testing/coverage/baseline_measurement.py --srcdir . --builddir build-baseline --output tools/testing/coverage/baseline --tool gcov --runs 1 --arch um"
