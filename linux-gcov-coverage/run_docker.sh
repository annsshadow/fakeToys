#!/usr/bin/env bash
# 在 docker 内运行覆盖率工具链（x86_64 + QEMU 路径）。
#
# 关键坑：Docker Desktop 的 bind mount 不穿透 /work 内的宿主 symlink
# （其目标为 Windows 路径 D:\...，Linux 容器内无法解析）。
# 因此这里用「嵌套 bind mount」把 fakeToys 副本直接挂到
# /work/tools/testing/coverage，docker 原生支持、无需解析 symlink。
#
# 另一关键坑（9p 并发读竞争）：Docker Desktop 的 9p 挂载在 make -j 高并发
# 读取被每个编译单元包含的头文件(page_64.h/cpufeature.h)时发生读竞争，返回
# 截断/错乱数据，汇编器报 'bad or irreducible absolute expression'，导致 gcov
# 构建非确定性失败（不同文件在不同次运行失败）。解决：先把内核源码顺序复制到
# 容器本地磁盘 /src（单次顺序读、无竞争），编译全程走本地 IO；报告仍写回
# /work（经 symlink 落 fakeToys，仅单个小 JSON，无并发风险）。
#
# 复制用 cp -a（而非 tar）：内核源码树含大量构建产物软链（include/asm、生成
# 的 uapi 头、tools 软链），历史上以 srctree=/work 运行时被写成绝对 /work/* 软链，
# 在宿主上破损。tar 提取这类软链时会因目标不存在而失败并塌缩为 0 文件；cp -a
# 原样保留软链（不解析目标），在容器内 /work 仍存在，软链可正确解析，构建正常。
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
    echo \"[run_docker] 复制内核源码到容器本地磁盘 /src（规避 9p 并发读竞争）...\"
    rm -rf /src && mkdir -p /src
    # 顺序复制顶层条目（排除构建产物 build*、仓库元数据 .git、杂项 -p、以及
    # fakeToys 挂载 tools/testing/coverage，避免把副本冗余拷入 /src）。
    # cp -a 保留软链原样（不解析），对破损的绝对 /work/* 软链同样安全。
    cd /work
    for item in \$(ls -A); do
      case \"\$item\" in
        build*|\".git\"|\"-p\") ;;
        *) cp -a \"/work/\$item\" /src/ ;;
      esac
    done
    # 丢弃随源码拷入的 fakeToys 挂载副本（保持 /src 精简；报告仍经 /work 写回）。
    rm -rf /src/tools/testing/coverage
    count=\$(find /src -type f 2>/dev/null | wc -l | tr -d ' ')
    echo \"[run_docker] 源码复制完成（\${count} 文件）。开始测量...\"
    if [ \"\$count\" -lt 10000 ]; then
      echo \"[run_docker] ERROR: 复制文件数异常（\${count}），源码复制可能失败\" >&2
      exit 1
    fi
    # 脚本经 /work 的 symlink 解析（fakeToys），但源码/构建走本地 /src，规避 9p 竞争。
    # 注意：$SCRIPT 与 $* 由【外层】shell 展开（不转义），其余 $item/$count/$(ls) 由
    # 【容器内】shell 展开（转义 \$）。若误转义 \$SCRIPT，容器内该变量未定义，命令会
    # 塌缩成 python3 -u tools/testing/coverage/（目录）而报 can't find '__main__'。
    cd /work && python3 -u tools/testing/coverage/$SCRIPT \
        --srcdir /src --builddir /src/build-qemu \
        --output /work/tools/testing/coverage/baseline $*
    echo \"[run_docker] 测量完成。报告已写回 /work/tools/testing/coverage/baseline。\"
  "
