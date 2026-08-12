#!/usr/bin/env bash
# 在 docker 内运行覆盖率工具链（x86_64 + QEMU 路径）。
#
# 关键坑：Docker Desktop 的 bind mount 不穿透 /work 内的宿主 symlink
# （其目标为 Windows 路径 D:\...，Linux 容器内无法解析）。
# 因此这里用「嵌套 bind mount」把 fakeToys 副本直接挂到
# /work/tools/testing/coverage，docker 原生支持、无需解析 symlink。
#
# 用法：
#   ./run_docker.sh recapture.py          # 重采集（不重建内核）
#   ./run_docker.sh baseline_measurement.py --srcdir . --builddir build-qemu --output tools/testing/coverage/baseline --tool gcov --runs 1 --arch x86_64
set -euo pipefail

IMAGE="${IMAGE:-kernel-coverage-qemu:latest}"
KERNEL="${KERNEL:-/d/WORKSPACE/linux-7.1.3}"
TOOLCHAIN="${TOOLCHAIN:-/d/WORKSPACE/fakeToys/linux-gcov-coverage}"
SCRIPT="${1:-recapture.py}"
shift || true

docker run --rm --privileged \
  -v "$KERNEL:/work" \
  -v "$TOOLCHAIN:/work/tools/testing/coverage" \
  -w /work \
  "$IMAGE" \
  bash -c "cd /work && python3 -u tools/testing/coverage/$SCRIPT $*"
