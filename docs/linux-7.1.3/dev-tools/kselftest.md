## Linux 内核自测集（Kselftest）


内核在 tools/testing/selftests/ 目录下包含一组"自测试（self tests）"。这些测试旨在作为小型测试，用于单独演练内核中的各个代码路径。测试应在构建、安装并启动内核之后运行。

主线中的 kselftest 可以在较旧的稳定内核上运行。运行主线测试能提供最好的覆盖率。有多个测试环会在稳定版本上运行主线 kselftest 测试套件。原因是：当新增一个用于回归测试某已有代码中缺陷的测试时，我们应当能够在一个较旧的内核上运行该测试。因此，保留仍能测试较旧内核的代码，并确保其在较新版本上能够优雅地跳过该测试，这一点十分重要。

关于 Kselftest 框架以及如何使用该框架编写新测试的更多信息，可参阅 Kselftest wiki：

https://kselftest.wiki.kernel.org/

在某些系统上，热插拔测试可能会永远挂起，等待 cpu 和内存就绪以便离线。为此创建了一个特殊的热插拔目标来运行完整范围的热插拔测试。在默认模式下，热插拔测试以受限范围的安全模式运行。在受限模式下，cpu 热插拔测试只在单个 cpu 上运行，而非所有支持热插拔的 cpu；内存热插拔测试只在 2% 的支持热插拔的内存上运行，而非 10%。

kselftest 作为用户空间进程运行。可以在用户空间编写/运行的测试可能希望使用 `Test Harness`_。需要在内核空间运行的测试可能希望使用 `Test Module`_。

## 测试相关文档


有关 kselftests 本身的文档，请参阅：

- [testing-devices](testing-devices)

## 运行自测（热插拔测试以受限模式运行）


```
  $ make headers
  $ make -C tools/testing/selftests
```
```
  $ make -C tools/testing/selftests run_tests
```
```
  $ make kselftest
```
注意，部分测试需要 root 权限。

kselftest 支持将输出文件保存到单独的目录中，然后再运行测试。为了在单独的目录中定位输出文件，支持两种语法。两种情况下工作目录都必须是内核源码树的根目录。这一点同样适用于下文的"运行自测的子集"一节。

```
  $ make O=/tmp/kselftest kselftest
```
```
  $ export KBUILD_OUTPUT=/tmp/kselftest; make kselftest
```
O= 赋值优先于 KBUILD_OUTPUT 环境变量。

上述命令默认运行测试并打印完整的通过/失败报告。kselftest 支持"summary"选项以便更容易理解测试结果。当指定 summary 选项时，可在 /tmp/testname 文件中找到每个测试的详细单项结果。这一点同样适用于下文的"运行自测的子集"一节。

```
  $ make summary=1 kselftest
```

## 运行自测的子集


你可以在 make 命令行上使用 "TARGETS" 变量来指定要运行的单个测试，或一组要运行的测试。

```
  $ make -C tools/testing/selftests TARGETS=ptrace run_tests
```
```
  $  make TARGETS="size timers" kselftest
```
```
  $ make O=/tmp/kselftest TARGETS="size timers" kselftest
```
```
  $ export KBUILD_OUTPUT=/tmp/kselftest; make TARGETS="size timers" kselftest
```
此外，你还可以在 make 命令行上使用 "SKIP_TARGETS" 变量来指定要从 TARGETS 列表中排除的一个或多个目标。

```
  $ make -C tools/testing/selftests SKIP_TARGETS=ptrace run_tests
```
```
  $  make SKIP_TARGETS="size timers" kselftest
```
你也可以同时指定一个受限的测试列表来运行，例如：

```
  $  make TARGETS="breakpoints size timers" SKIP_TARGETS=size kselftest
```
所有可用目标的列表见顶层的 tools/testing/selftests/Makefile。

## 运行完整范围的热插拔自测


```
  $ make -C tools/testing/selftests hotplug
```
```
  $ make -C tools/testing/selftests run_hotplug
```
注意，部分测试需要 root 权限。


## 安装自测


你可以使用 "make" 的 "install" 目标（它会调用 `kselftest_install.sh` 工具）将自测安装到默认位置（`tools/testing/selftests/kselftest_install`），或通过 `INSTALL_PATH` 这个 "make" 变量安装到用户指定的位置。

```
   $ make -C tools/testing/selftests install
```
```
   $ make -C tools/testing/selftests install INSTALL_PATH=/some/other/path
```

## 运行已安装的自测


在安装目录以及 Kselftest tar 包中，都有一个名为 `run_kselftest.sh` 的脚本来运行测试。

你可以简单地执行以下命令来运行已安装的 Kselftests。例如：

```
   $ cd kselftest_install
   $ ./run_kselftest.sh
```
```
   $ ./run_kselftest.sh -l
```
`-c` 选项可用于从一个测试集合中运行所有测试，例如：

```
   $ ./run_kselftest.sh -c size -c seccomp -t timers:posix_timers -t timer:nanosleep
```
其他功能请参见脚本的使用输出（使用 `-h` 选项查看）。

## 自测超时


自测被设计为运行迅速，因此每个测试默认使用 45 秒的超时。测试可以通过在其目录中添加一个 settings 文件并在其中设置一个 timeout 变量，来覆盖默认超时，将其配置为该测试期望的上限超时。只有少数测试会将超时覆盖为高于 45 秒的值，kselftest 力求保持这一状况。自测中的超时不被认为是致命的，因为运行测试的系统可能会发生变化，这也会改变运行测试的预期耗时。如果你能控制将运行这些测试的系统，可以通过命令行上的 `-o` 或 `--override-timeout` 参数，在这些系统上配置测试运行器使用一个更大或更小的超时。例如，要使用 165 秒：

```
   $ ./run_kselftest.sh --override-timeout 165
```
你可以查看 TAP 输出来判断你是否遇到了超时。明确知道某个测试必须在特定时间内运行的测试运行器，随后可以选择性地将此类超时视为致命。

## 打包自测


在某些情况下需要打包，例如当测试需要在某个环境下运行时：

```
   $ make -C tools/testing/selftests gen_tar
```
这会在 `INSTALL_PATH/kselftest-packages` 目录中生成一个 tar 包。默认使用 `.gz` 格式。tar 的压缩格式可以通过指定 `FORMAT` make 变量来覆盖。任何被 `tar 的 auto-compress`_ 识别的值均可使用，例如：

```
    $ make -C tools/testing/selftests gen_tar FORMAT=.xz
```
`make gen_tar` 会调用 `make install`，因此你可以结合"运行自测的子集"一节中指定的变量来打包自测的一个子集，例如：

```
    $ make -C tools/testing/selftests gen_tar TARGETS="size" FORMAT=.xz
```

## 贡献新测试


一般而言，自测试的规则是：

 - 如果你不是 root，就尽量多做事；

 - 不要耗时太久；

 - 不要在任何架构上破坏构建；并且

 - 当你的功能未配置时，不要让顶层的 "make run_tests" 失败。

 - 测试的输出必须符合 TAP 标准，以确保较高的测试质量，并以具体细节捕获失败/错误。kselftest.h 与 kselftest_harness.h 头文件提供了输出测试结果的封装。这些封装应当用于通过、失败、退出和跳过消息。CI 系统可以轻松解析 TAP 输出消息以检测测试结果。

## 贡献新测试（细节）


 - 在你的 Makefile 中，通过包含 lib.mk 来使用其中的设施，而不是重复造轮子。在相应的行上指定标志和二进制生成标志，例如：

```
    CFLAGS = $(KHDR_INCLUDES)
    TEST_GEN_PROGS := close_range_test
    include ../lib.mk
```

 * 如果此类二进制或文件是在编译期间生成的，使用 TEST_GEN_XXX。

   TEST_PROGS、TEST_GEN_PROGS 表示它是默认被测试的的可执行文件。

   TEST_GEN_MODS_DIR 应由那些在测试开始之前需要构建模块的测试使用。该变量将包含存放模块的目录名。

   TEST_CUSTOM_PROGS 应由需要自定义构建规则并阻止使用通用构建规则的测试使用。

   TEST_PROGS 用于测试 shell 脚本。请确保 shell 脚本设置了可执行位。否则 lib.mk 的 run_tests 会产生警告。

   TEST_CUSTOM_PROGS 和 TEST_PROGS 会被通用的 run_tests 运行。

   TEST_PROGS_EXTENDED、TEST_GEN_PROGS_EXTENDED 表示它是默认不被测试的可执行文件。

   TEST_FILES、TEST_GEN_FILES 表示它是测试所使用的文件。

   TEST_INCLUDES 与 TEST_FILES 类似，它列出了在导出或安装测试时应包含的文件，但有以下区别：

    * 到其他目录中文件的符号链接会被保留
    * 在将文件复制到输出目录时，tools/testing/selftests/ 之下的路径部分会被保留

   TEST_INCLUDES 用于列出位于自测试层次结构中其他目录的依赖项。

 * 首先使用内核源码和/或 git 仓库中的头文件，然后再使用系统头文件。相对于发行版安装到系统上的头文件，应当优先关注该内核版本的头文件，以便能够发现回归。在 Makefile 中使用 KHDR_INCLUDES 来包含来自内核源码的头文件。

 * 如果某个测试需要启用特定的内核配置选项，请在测试目录中添加一个 config 文件来启用它们。

   例如：tools/testing/selftests/android/config

 * 在测试目录内创建一个 .gitignore 文件，并将所有生成的 object 加入其中。

 * 在 selftests/Makefile 的 TARGETS 中添加新的测试名：

    TARGETS += android

 * 所有改动都应通过以下检查：

```
    kselftest-{all,install,clean,gen_tar}
    kselftest-{all,install,clean,gen_tar} O=abo_path
    kselftest-{all,install,clean,gen_tar} O=rel_path
    make -C tools/testing/selftests {all,install,clean,gen_tar}
    make -C tools/testing/selftests {all,install,clean,gen_tar} O=abs_path
    make -C tools/testing/selftests {all,install,clean,gen_tar} O=rel_path
```

## 测试模块


kselftest 从用户空间测试内核。有时需要从内核内部进行测试，一种方法是创建一个测试模块。我们可以通过一个 shell 脚本测试运行器将该模块接入 kselftest 框架。`kselftest/module.sh` 就是为简化这一过程而设计的。同时还提供了一个头文件来辅助编写与 kselftest 配合使用的内核模块：

- `tools/testing/selftests/kselftest_module.h`
- `tools/testing/selftests/kselftest/module.sh`

注意，测试模块应当以 TAINT_TEST 污染内核。对于位于 `tools/testing/` 目录中的模块，或使用了上述 `kselftest_module.h` 头文件的模块，这会自动发生。否则，你需要在模块源码中添加 `MODULE_INFO(test, "Y")`。不加载模块的自测试通常不应污染内核，但在加载了非测试模块的情况下，可以通过向 `/proc/sys/kernel/tainted` 写入，从用户空间施加 TEST_TAINT。

### 如何使用


这里我们展示创建测试模块并将其接入 kselftest 的典型步骤。我们以 lib/ 的 kselftests 为例。

1. 创建测试模块

2. 创建将要运行（加载/卸载）该模块的测试脚本
   例如 `tools/testing/selftests/lib/bitmap.sh`

3. 向 config 文件添加一行，例如 `tools/testing/selftests/lib/config`

4. 向 makefile 添加测试脚本，例如 `tools/testing/selftests/lib/Makefile`

5. 验证其工作正常：


   # 假设你已经启动了一个该内核树的全新构建
   cd /path/to/linux/tree
   make kselftest-merge
   make modules
   sudo make modules_install
   make TARGETS=lib kselftest

### 示例模块


一个最简的测试模块可能如下所示：


   // SPDX-License-Identifier: GPL-2.0+

   #define pr_fmt(fmt) KBUILD_MODNAME ": " fmt

   #include "../tools/testing/selftests/kselftest_module.h"

   KSTM_MODULE_GLOBALS();

   /*
    - 用于测试 foobinator 的内核模块
    */

   static int __init test_function()
   {
           ...
   }

   static void __init selftest(void)
   {
           KSTM_CHECK_ZERO(do_test_case("", 0));
   }

   KSTM_MODULE_LOADERS(test_foo);
   MODULE_AUTHOR("John Developer <jd@fooman.org>");
   MODULE_LICENSE("GPL");
   MODULE_INFO(test, "Y");

### 示例测试脚本



    #!/bin/bash
    # SPDX-License-Identifier: GPL-2.0+
    $(dirname $0)/../kselftest/module.sh "foo" test_foo


## 测试框架（Test Harness）


kselftest_harness.h 文件包含了用于构建测试的有用辅助宏。该测试框架用于用户空间测试，关于内核空间测试请参见上文的 `Test Module`_。

tools/testing/selftests/seccomp/seccomp_bpf.c 中的测试可作为示例。

### 示例


    :doc: example


### 辅助宏


    :functions: TH_LOG TEST TEST_SIGNAL FIXTURE FIXTURE_DATA FIXTURE_SETUP
                FIXTURE_TEARDOWN TEST_F TEST_HARNESS_MAIN FIXTURE_VARIANT
                FIXTURE_VARIANT_ADD

### 运算符


    :doc: operators

    :functions: ASSERT_EQ ASSERT_NE ASSERT_LT ASSERT_LE ASSERT_GT ASSERT_GE
                ASSERT_NULL ASSERT_TRUE ASSERT_NULL ASSERT_TRUE ASSERT_FALSE
                ASSERT_STREQ ASSERT_STRNE EXPECT_EQ EXPECT_NE EXPECT_LT
                EXPECT_LE EXPECT_GT EXPECT_GE EXPECT_NULL EXPECT_TRUE
                EXPECT_FALSE EXPECT_STREQ EXPECT_STRNE
