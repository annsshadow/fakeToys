# gcov 用于 Linux 内核


gcov 性能分析（profiling）内核支持使得可以将 GCC 的覆盖率测试工具 gcov_ 用于 Linux 内核。运行内核的覆盖率数据以 gcov 兼容的格式通过 “gcovdebugfs 目录导出。要获取特定文件的覆盖率数据，请切换到内核构
```

    # cd /tmp/linux-out
    # gcov -o /sys/kernel/debug/gcov/tmp/linux-out/kernel spinlock.c

```
这将在当前目录中创建带有执行次数标注的源代码文件。此外，也可以使lcov_ 等图形化 gcov 前端来自动化收集整个内核数据的过程，并提HTML 格式的覆盖率概览

## 可能的用途：

- 调试（这一行是否被执行过？
- 改进测试（如何修改测试以覆盖这些行？
- 精简内核配置（如果相关代码从未运行，我是否还需要该选项？）



## 准备工作


```

        CONFIG_DEBUG_FS=y
        CONFIG_GCOV_KERNEL=y

```
```

        CONFIG_GCOV_PROFILE_ALL=y

```
请注意，使用性能分析标志编译的内核会明显更大且运行更慢。此外，并非所有架构都支持 CONFIG_GCOV_PROFILE_ALL

只有debugfs 被挂载后，性能分析数据才会变得可访问
```

        mount -t debugfs none /sys/kernel/debug


```
定制


要针对特定文件或目录启用性能分析，请在相应的内核 Makefile 中添加一行类似于以下内容

```

	GCOV_PROFILE_main.o := y

```
```

	GCOV_PROFILE := y

```
即使启用CONFIG_GCOV_PROFILE_ALL，也要将某些文件排除在性能分析之外
```

	GCOV_PROFILE_main.o := n

```
```

	GCOV_PROFILE := n

```
该机制仅支持链接到主内核映像或编译为内核模块的那些文件


## 模块特定的配


## 下面描述了针对特定模块的 gcov 内核配置

CONFIG_GCOV_PROFILE_RDS锛。
        RDS 上启GCOV 性能分析，用于检查哪些函数或行被执行。该配置rds 自测试用于生成覆盖率报告。如果未设置，则省略该报告


## 文件


## gcov 内核支持debugfs 中创建以下文件：

`/sys/kernel/debug/gcov`
	所gcov 相关文件的父目录

`/sys/kernel/debug/gcov/reset`
	全局重置文件：向其写入时会将所有覆盖率数据重置为零

`/sys/kernel/debug/gcov/path/to/compile/dir/file.gcda`
	gcov 工具所能识别的实际 gcov 数据文件。向其写入时会将该文件的覆盖率数据重置为零

`/sys/kernel/debug/gcov/path/to/compile/dir/file.gcno`
	gcov 工具所需的静态数据文件的符号链接。该文件gcc 在配`-ftest-coverage` 选项编译时生成


## 模块


内核模块可能包含仅在模块卸载时运行的清理代码。gcov 机制通过保留与已卸载模块相关联的数据副本，提供了一种收集此类代码覆盖率数据的手段。这些数据通过 debugfs 保持可用。一旦模块再次加载，相关的覆盖率计数器会用其上一次实例化的数据进行初始化

通过在运行时指定 gcov_persist 参数可以停用此行为
```

        gcov_persist=0

```
在运行时，用户也可以通过写入其数据文件或全局重置文件，来选择丢弃某个已卸载模块的数据


## 构建机与测试机分离的情况


gcov 内核性能分析基础设施的设计初衷是让内核在同一台机器上构建和运行的开箱即用场景。如果内核运行在另一台独立的机器上，则必须根gcov 工具的使用位置做出特殊准备：


## a) gcov 在测试机上运

    The gcov tool version on the test machine must be compatible with the
    gcc version used for kernel build. Also the following files need to be
    copied from build to test machine:

    来自源码树：
      - 所C 源文件与头文

    来自构建树：
      - 所C 源文件与头文
      - 所.gcda .gcno 文件
      - 所有指向目录的符号链接

    需要注意的是，这些文件必须放置在测试机上与构建机完全相同的文件系统位置。如果任何路径组件是符号链接，则必须使用实际的目录（这是由于 make CURDIR 的处理方式）


## b) gcov 在构建机上运

## 每次测试用例执行后，需要以下文件从测试机复制到构建机：

    来自 sysfs 中的 gcov 目录
      - 所.gcda 文件
      - 所有指.gcno 文件的链

    这些文件可以复制到构建机上的任意位置。随后必须使-o 选项指向该目录来调用 gcov

```

      /tmp/linux:    kernel source tree
      /tmp/out:      kernel build directory as specified by make O=
      /tmp/coverage: location of the files copied from the test machine

      [user@build] cd /tmp/out
      [user@build] gcov -o /tmp/coverage/tmp/out/init main.c


```
关于编译器的说明


GCC LLVM gcov 工具不一定兼容。请使用 gcov_ 来处GCC 生成.gcno .gcda 文件，使llvm-cov_ 来处Clang


GCC Clang gcov 在构建上的差异由 Kconfig 处理。它会根据检测到的工具链自动选择合适的 gcov 格式


## 故障排查


问题
    编译在链接器步骤期间中止

原因
    为那些未链接到主内核、或通过自定义链接过程链接的源文件指定了性能分析标志

解决方案
    通过在相应的 Makefile 中指`GCOV_PROFILE := n` `GCOV_PROFILE_basename.o := n`，将受影响的源文件排除在性能分析之外

问题
    sysfs 复制的文件显示为空或不完整

原因
    由于 seq_file 的工作方式，某些工具（如 cp tar）可能无法正确复sysfs 中的文件

解决方案
    使用 `cat` 读取 `.gcda` 文件，使`cp -d` 复制链接。也可以使用附录 B 所示的机制


## 附录 A：gather_on_build.sh


用于在构建机上收集覆盖率元文件的示例脚本
（参见“构建机与测试机分离的情况a. <gcov-test>）：


    #!/bin/bash

    KSRC=$1
    KOBJ=$2
    DEST=$3

    if [ -z "$KSRC" ] || [ -z "$KOBJ" ] || [ -z "$DEST" ]; then
      echo "Usage: $0 <ksrc directory> <kobj directory> <output.tar.gz>" >&2
      exit 1
    fi

    KSRC=$(cd $KSRC; printf "all:\n\t@echo \${CURDIR}\n" | make -f -)
    KOBJ=$(cd $KOBJ; printf "all:\n\t@echo \${CURDIR}\n" | make -f -)

    find $KSRC $KOBJ \( -name '**.gcno' -o -name '**.[ch]' -o -type l \) -a \
                     -perm /u+r,g+r | tar cfz $DEST -P -T -

    if [ $? -eq 0 ] ; then
      echo "$DEST successfully created, copy to test system and unpack with:"
      echo "  tar xfz $DEST -P"
    else
      echo "Could not create file $DEST"
    fi


## 附录 B：gather_on_test.sh


用于在测试机上收集覆盖率数据文件的示例脚
（参见“构建机与测试机分离的情况b. <gcov-build>）：



    #!/bin/bash -e

    DEST=$1
    GCDA=/sys/kernel/debug/gcov

    if [ -z "$DEST" ] ; then
      echo "Usage: $0 <output.tar.gz>" >&2
      exit 1
    fi

    TEMPDIR=$(mktemp -d)
    echo Collecting data..
    find $GCDA -type d -exec mkdir -p $TEMPDIR/\{\} \;
    find $GCDA -name '*.gcda' -exec sh -c 'cat < $0 > '$TEMPDIR'/$0' {} \;
    find $GCDA -name '*.gcno' -exec sh -c 'cp -d $0 '$TEMPDIR'/$0' {} \;
    tar czf $DEST -C $TEMPDIR sys
    rm -rf $TEMPDIR

    echo "$DEST successfully created, copy to build system and unpack with:"
    echo "  tar xfz $DEST"
