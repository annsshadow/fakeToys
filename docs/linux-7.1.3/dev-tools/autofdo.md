
AutoFDO 用于 Linux 内核


启用后，在使Clang 编译器时为内核提AutoFDO 构建支持。AutoFDO（Automatic Feedback-Directed Optimization，自动反馈导向优化）是一种基于性能剖析的优化（PGO），用于提升二进制可执行文件的性能。它利用硬件采样收集二进制中各种代码路径的执行频率信息。随后这些数据被用于指导编译器的优化决策，从而生成更高效的二进制文件。AutoFDO 是一种强大的优化技术，数据表明它可以显著提升内核性能。对于受前端停顿影响的工作负载尤为有益

与非 FDO 构建不同，AutoFDO 构建要求用户提供一个性能剖析文件（profile）。获AutoFDO 剖析文件有多种方式。AutoFDO 剖析文件是通过 "perf" 工具转换硬件采样而创建的。用于生成这perf 文件的工作负载必须具有代表性，它们必须表现出与拟优化工作负载相似的运行时特征。否则将导致编译器针对错误的目标进行优化

AutoFDO 剖析文件通常封装了程序的行为。如果性能关键代码是与体系结构无关的，则该剖析文件可跨平台应用以获得性能提升。例如，使用Intel 体系结构上生成的剖析文件来构建面AMD 体系结构的内核，同样可以带来性能改进

获取具有代表性的剖析文件有两种方法：
(1) 使用生产环境对真实工作负载进行采样
(2) 使用具有代表性的负载测试生成剖析文件
如果在启AutoFDO 构建配置时未提供 AutoFDO 剖析文件，编译器只会修改内核中的 dwarf 信息，而不会影响运行时性能。建议使用以相同 AutoFDO 配置构建的内核二进制文件来收perf 剖析文件。虽然也可以使用以不同选项构建的内核，但可能会导致性能下降

可以使用上一版内核的 AutoFDO 构建来收集剖析文件。AutoFDO 采用相对行号来匹配剖析文件，对源码变更有一定容忍度。这种模式常用于生产环境中收集剖析文件

在基于负载测试的剖析文件收集中，AutoFDO 收集过程包含以下步骤

#. 初始构建：使AutoFDO 选项构建内核，但不带剖析文件

#. 性能剖析：随后使用具有代表性的工作负载运行上述内核，以收集执行频率数据。这些数据通过 perf 利用硬件采样收集。AutoFDO 在支持高PMU 特性（Intel 机器上的 LBR）的平台上最为有效

#. AutoFDO 剖析文件生成：通过离线工具perf 输出文件转换AutoFDO 剖析文件

该支持需Clang 编译LLVM 17 或更高版本

准备工作


```

   CONFIG_AUTOFDO_CLANG=y

```
自定


默认CONFIG_AUTOFDO_CLANG 设置覆盖 AutoFDO 构建的内核空间目标。不过，可以通过在相应的内核 Makefile 中添加类似下面的一行，来为单个文件或目录启用或禁用 AutoFDO 构建

```

   AUTOFDO_PROFILE_foo.o := y

```
```

   AUTOFDO_PROFILE := y

```
```

   AUTOFDO_PROFILE_foo.o := n

```
```

   AUTOFDO_PROFILE := n

```
工作流程


以下AutoFDO 内核的示例工作流程：

1) 在启用了 LLVM 的主机上构建内核
```

      $ make menuconfig LLVM=1

    Turn on AutoFDO build config::

      CONFIG_AUTOFDO_CLANG=y

    With a configuration that with LLVM enabled, use the following command::

      $ scripts/config -e AUTOFDO_CLANG

    After getting the config, build with ::

      $ make LLVM=1

```
2) 在测试机器上安装该内核

3) 运行负载测试。perf 中的 '-c' 选项指定采样事件周期。建议为此使用一个合适的素数，例500009

```

      $ perf record -e BR_INST_RETIRED.NEAR_TAKEN:k -a -N -b -c <count> -o <perf_file> -- <loadtest>

   - For AMD platforms:

     The supported systems are: Zen3 with BRS, or Zen4 with amd_lbr_v2. To check,

     For Zen3::

      $ cat /proc/cpuinfo | grep " brs"

     For Zen4::

      $ cat /proc/cpuinfo | grep amd_lbr_v2

     The following command generated the perf data file::

      $ perf record --pfm-events RETIRED_TAKEN_BRANCH_INSTRUCTIONS:k -a -N -b -c <count> -o <perf_file> -- <loadtest>

```
4) （可选）将原perf 文件下载到主机

5) 要生AutoFDO 剖析文件，有两个离线工具可用：create_llvm_prof llvm_profgen。create_llvm_prof 工具AutoFDO 项目的一部分，可GitHub（https://github.com/google/autofdo）上找到，版本为 v0.30.1 或更高。llvm_profgen 工具包含LLVM 编译器本身中。需要注意的是，llvm_profgen 的版本无需Clang 的版本匹配。需要的Clang LLVM 19 版本发布
```

      $ llvm-profgen --kernel --binary=<vmlinux> --perfdata=<perf_file> -o <profile_file>

   or ::

      $ create_llvm_prof --binary=<vmlinux> --profile=<perf_file> --format=extbinary --out=<profile_file>

   Note that multiple AutoFDO profile files can be merged into one via::

      $ llvm-profdata merge -o <profile_file> <profile_1> <profile_2> ... <profile_n>

```
6) 使用与步1 相同的配置和 AutoFDO 剖析文件重新构建内核
```

      $ make LLVM=1 CLANG_AUTOFDO_PROFILE=<profile_file>

```