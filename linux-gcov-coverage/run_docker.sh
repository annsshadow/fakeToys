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
  bash -c "
    set -e
    # 关键修复：Docker Desktop 的 9p 挂载在 make -j 高并发读取被每个编译单元
    # 包含的头文件(page_64.h/cpufeature.h)时发生读竞争，返回截断/错乱数据，
    # 汇编器报 'bad or irreducible absolute expression'，导致 gcov 构建非确定性失败
    # （不同文件在不同次运行失败）。解决：先把内核源码顺序复制到容器本地磁盘 /src
    # （单次顺序读、无竞争），编译全程走本地 IO；报告仍写回 /work(经 symlink 落 fakeToys)。
    echo \"[run_docker] 复制内核源码到容器本地磁盘 /src（规避 9p 并发读竞争）...\"
    rm -rf /src && mkdir -p /src
    tar cf - -C /work --exclude=build* --exclude=.git \
        --exclude='*.o' --exclude='*.a' --exclude='*.gcda' \
        --exclude='*.gcno' --exclude='*.gcov' --exclude=__pycache__ \
        --exclude='*.pyc' --exclude='-p' . | tar xf - -C /src
    echo \"[run_docker] 源码复制完成（$(find /src -type f | wc -l) 文件）。开始测量...\"
    # 脚本经 /work 的 symlink 解析（fakeToys），但源码/构建走本地 /src，规避 9p 竞争。
    cd /work && python3 -u tools/testing/coverage/$SCRIPT \
        --srcdir /src --builddir /src/build-qemu \
        --output /work/tools/testing/coverage/baseline $*
    echo \"[run_docker] 测量完成。报告已写回 /work/tools/testing/coverage/baseline。\"
  "
