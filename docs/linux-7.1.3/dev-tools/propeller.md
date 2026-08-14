
将 Propeller 用于 Linux 内核


启用后，在使用 Clang 编译器时为内核提供 Propeller 构建支持。Propeller 是一种基于性能剖析的优化（PGO）方法，用于优化二进制可执行文件。与 AutoFDO 类似，它利用硬件采样收集二进制中不同代码路径的执行频率信息。与 AutoFDO 不同，该信息随后会在链接阶段之前被用于优化（其中包括）函数内部及跨函数的基本块布局。

采用 Propeller 优化时的一些重要注意事项：

#. 尽管它可作为独立的优化步骤使用，但强烈建议在 AutoFDO、AutoFDO+ThinLTO 或 Instrument FDO 之上应用 Propeller。本文档的其余部分均以此范式为前提。

#. Propeller 在 AutoFDO/AutoFDO+ThinLTO/iFDO 之上再进行一轮性能剖析。整个构建过程包括“build-afdo - train-afdo - build-propeller - train-propeller - build-optimized”。

#. Propeller 需要 Clang/Clang++ 与链接器（ld.lld）为 LLVM 19 或更高版本。

#. 除 LLVM 工具链外，Propeller 还需要一个性能剖析转换工具：https://github.com/google/autofdo，版本需在 v0.30.1 之后：https://github.com/google/autofdo/releases/tag/v0.30.1。

Propeller 优化过程包含以下步骤：

#. 初始构建：像通常那样构建 AutoFDO 或 AutoFDO+ThinLTO 二进制文件，但需带上一组编译期/链接期标志，以便在内核二进制文件中创建一个特殊的元数据段。该特殊段仅用于性能剖析工具，它不是运行时映像的一部分，也不会改变内核运行时的文本段。

#. 性能剖析：随后使用具有代表性的工作负载运行上述内核，以收集执行频率数据。这些数据通过 perf 利用硬件采样收集。Propeller 在支持高级 PMU 特性（如 Intel 机器上的 LBR）的平台上最为有效。此步骤与为 AutoFDO 剖析内核的过程相同（具体的 perf 参数可能不同）。

#. Propeller 剖析文件生成：通过离线工具将 perf 输出文件转换为一对 Propeller 剖析文件。

#. 优化构建：像通常那样构建 AutoFDO 或 AutoFDO+ThinLTO 优化二进制文件，但需带上编译期/链接期标志以使用 Propeller 的编译期与链接期剖析文件。此构建步骤使用 3 个剖析文件——AutoFDO 剖析文件、Propeller 编译期剖析文件和 Propeller 链接期剖析文件。

#. 部署：优化后的内核二进制文件被部署并用于生产环境，从而提供更高的性能和更低的延迟。

准备工作


```

   CONFIG_AUTOFDO_CLANG=y
   CONFIG_PROPELLER_CLANG=y

```
自定义


默认的 CONFIG_PROPELLER_CLANG 设置覆盖 Propeller 构建的内核空间目标。不过，可以通过在相应的内核 Makefile 中添加类似下面的一行，来为单个文件或目录启用或禁用 Propeller 构建：

```

   PROPELLER_PROFILE_foo.o := y

```
```

   PROPELLER_PROFILE := y

```
```

   PROPELLER_PROFILE_foo.o := n

```
```

   PROPELLER__PROFILE := n


```
工作流程


以下是构建 AutoFDO+Propeller 内核的示例工作流程：

1) 假设已按照 AutoFDO 文档中的说明收集了 AutoFDO 剖析文件，在主机上构建内核
```

      CONFIG_AUTOFDO_CLANG=y
      CONFIG_PROPELLER_CLANG=y

   and ::

      $ make LLVM=1 CLANG_AUTOFDO_PROFILE=<autofdo-profile-name>

```
2) 在测试机器上安装该内核。

3) 运行负载测试。perf 中的 '-c' 选项指定采样事件周期。建议为此使用一个合适的素数，例如 500009。

```

      $ perf record -e BR_INST_RETIRED.NEAR_TAKEN:k -a -N -b -c <count> -o <perf_file> -- <loadtest>

   - For AMD platforms::

      $ perf record --pfm-event RETIRED_TAKEN_BRANCH_INSTRUCTIONS:k -a -N -b -c <count> -o <perf_file> -- <loadtest>

   Note you can repeat the above steps to collect multiple <perf_file>s.

```
4) （可选）将原始 perf 文件下载到主机。

5) 使用 create_llvm_prof 工具（https://github.com/google/autofdo）来
```

      $ create_llvm_prof --binary=<vmlinux> --profile=<perf_file>
                         --format=propeller --propeller_output_module_name
                         --out=<propeller_profile_prefix>_cc_profile.txt
                         --propeller_symorder=<propeller_profile_prefix>_ld_profile.txt

   "<propeller_profile_prefix>" can be something like "/home/user/dir/any_string".

   This command generates a pair of Propeller profiles:
   "<propeller_profile_prefix>_cc_profile.txt" and
   "<propeller_profile_prefix>_ld_profile.txt".

   If there are more than 1 perf_file collected in the previous step,
   you can create a temp list file "<perf_file_list>" with each line
   containing one perf file name and run::

      $ create_llvm_prof --binary=<vmlinux> --profile=@<perf_file_list>
                         --format=propeller --propeller_output_module_name
                         --out=<propeller_profile_prefix>_cc_profile.txt
                         --propeller_symorder=<propeller_profile_prefix>_ld_profile.txt

```
6) 使用 AutoFDO 与 Propeller 重新构建内核
```

      CONFIG_AUTOFDO_CLANG=y
      CONFIG_PROPELLER_CLANG=y

   and ::

      $ make LLVM=1 CLANG_AUTOFDO_PROFILE=<profile_file> CLANG_PROPELLER_PROFILE_PREFIX=<propeller_profile_prefix>

```