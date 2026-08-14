## 故障注入能力基础设施


另请参见 scsi_debug 的 "every_nth" 模块选项。


### 可用的故障注入能力


- failslab

  注入 slab 分配失败。（kmalloc()、kmem_cache_alloc()、...）

- fail_page_alloc

  注入页分配失败。（alloc_pages()、get_free_pages()、...）

- fail_usercopy

  在用户内存访问函数中注入失败。（copy_from_user()、get_user()、...）

- fail_futex

  注入 futex 死锁和 uaddr 错误。

- fail_sunrpc

  注入内核 RPC 客户端和服务器端失败。

- fail_make_request

  在通过设置 /sys/block/<device>/make-it-fail 或 /sys/block/<device>/<partition>/make-it-fail 所允许的设备上注入磁盘 IO 错误。（submit_bio_noacct()）

- fail_mmc_request

  在通过设置 /sys/kernel/debug/mmc0/fail_mmc_request 下的 debugfs 条目所允许的设备上注入 MMC 数据错误。

- fail_function

  通过设置在 /sys/kernel/debug/fail_function 下的 debugfs 条目，对标记了 ALLOW_ERROR_INJECTION() 宏的特定函数注入错误返回。不支持引导选项。

- fail_skb_realloc

  将 skb（套接字缓冲区）重新分配事件注入网络路径。主要目标是识别并防止网络子系统中与指针管理不当相关的问题。通过在关键点强制 skb 重新分配，此特性制造了现有指向 skb 头部指针失效的场景。

  当故障被注入并触发重新分配时，缓存的指向 skb 头部和数据的指针不再引用有效的内存位置。这种故意的失效有助于暴露重新分配事件后未正确更新指针的代码路径。

  通过创建这些受控的故障场景，系统可以捕获使用陈旧指针的情况，这可能导致内存损坏或系统不稳定。

  要选择作用的接口，将网络名称写入 /sys/kernel/debug/fail_skb_realloc/devname。如果此字段留空（即默认值），skb 重新分配将被强制应用于所有网络接口。

  当启用 KASAN 时，此故障检测的有效性会增强，因为它有助于识别无效内存引用和释放后使用（UAF）问题。

- NVMe 故障注入

  在通过设置 /sys/kernel/debug/nvme*/fault_inject 下的 debugfs 条目所允许的设备上，注入 NVMe 状态码和重试标志。默认状态码为 NVME_SC_INVALID_OPCODE，不重试。状态码和重试标志可以通过 debugfs 设置。

- Null 测试块设备驱动故障注入

  通过设置 /sys/kernel/config/nullb/<disk>/timeout_inject 下的配置项注入 IO 超时，通过设置 /sys/kernel/config/nullb/<disk>/requeue_inject 下的配置项注入重新排队请求，以及通过设置 /sys/kernel/config/nullb/<disk>/init_hctx_fault_inject 下的配置项注入 init_hctx() 错误。

### 配置故障注入能力的行为


##### debugfs 条目


fault-inject-debugfs 内核模块提供了一些 debugfs 条目，用于运行时配置故障注入能力。

- /sys/kernel/debug/fail*/probability:

	注入失败的可能性，以百分比表示。

	格式：<percent>

	注意，每百次一次失败对某些测试用例来说是相当高的错误率。对于此类测试用例，考虑设置 probability=100 并配置 /sys/kernel/debug/fail*/interval。

- /sys/kernel/debug/fail*/interval:

	指定失败之间的间隔，针对通过了所有其他测试的 should_fail() 调用。

	注意，如果你通过 interval>1 启用了它，你很可能想要设置 probability=100。

- /sys/kernel/debug/fail*/times:

	指定失败最多可能发生的次数。值 -1 表示"无限制"。

- /sys/kernel/debug/fail*/space:

	指定一个初始资源"预算"，每次调用 should_fail(,size) 时按 "size" 递减。在 "space" 达到零之前，故障注入被抑制。

- /sys/kernel/debug/fail*/verbose

	格式：{ 0 | 1 | 2 }

	指定注入失败时消息的详细程度。'0' 表示无消息；'1' 每次失败只打印一行日志；'2' 还会打印调用栈跟踪——有助于调试故障注入暴露的问题。

- /sys/kernel/debug/fail*/task-filter:

	格式：{ 'Y' | 'N' }

	值 'N' 禁用按进程过滤（默认）。任何正值将失败限制为仅由 /proc/<pid>/make-it-fail==1 指示的进程。

- /sys/kernel/debug/fail*/require-start、
  /sys/kernel/debug/fail*/require-end、
  /sys/kernel/debug/fail*/reject-start、
  /sys/kernel/debug/fail*/reject-end:

	指定在栈跟踪遍历期间测试的虚拟地址范围。仅当被遍历栈跟踪中的某个调用者位于所需范围内，且没有调用者位于拒绝范围内时，才注入失败。默认所需范围为 [0,ULONG_MAX)（整个虚拟地址空间）。默认拒绝范围为 [0,0)。

- /sys/kernel/debug/fail*/stacktrace-depth:

	指定在搜索 [require-start,require-end) 或 [reject-start,reject-end) 范围内调用者时遍历的最大栈跟踪深度。

- /sys/kernel/debug/fail_page_alloc/ignore-gfp-highmem:

	格式：{ 'Y' | 'N' }

	默认是 'Y'，将其设置为 'N' 也会向 highmem/用户分配（__GFP_HIGHMEM 分配）注入失败。

- /sys/kernel/debug/failslab/cache-filter
	格式：{ 'Y' | 'N' }

        默认是 'N'，将其设置为 'Y' 将只在对象来自某些特定缓存时才注入失败。

        通过向 /sys/kernel/slab/<cache>/failslab 写入 '1' 来选择缓存：

- /sys/kernel/debug/failslab/ignore-gfp-wait:
- /sys/kernel/debug/fail_page_alloc/ignore-gfp-wait:

	格式：{ 'Y' | 'N' }

	默认是 'Y'，将其设置为 'N' 也会向可以睡眠的分配（__GFP_DIRECT_RECLAIM 分配）注入失败。

- /sys/kernel/debug/fail_page_alloc/min-order:

	指定要注入失败的最小页分配阶。

- /sys/kernel/debug/fail_futex/ignore-private:

	格式：{ 'Y' | 'N' }

	默认是 'N'，将其设置为 'Y' 将在处理私有（地址空间）futex 时禁用失败注入。

- /sys/kernel/debug/fail_sunrpc/ignore-client-disconnect:

	格式：{ 'Y' | 'N' }

	默认是 'N'，将其设置为 'Y' 将禁用 RPC 客户端上的断开连接注入。

- /sys/kernel/debug/fail_sunrpc/ignore-server-disconnect:

	格式：{ 'Y' | 'N' }

	默认是 'N'，将其设置为 'Y' 将禁用 RPC 服务器端上的断开连接注入。

- /sys/kernel/debug/fail_sunrpc/ignore-cache-wait:

	格式：{ 'Y' | 'N' }

	默认是 'N'，将其设置为 'Y' 将禁用 RPC 服务器端上的缓存等待注入。

- /sys/kernel/debug/fail_function/inject:

	格式：{ 'function-name' | '!function-name' | '' }

	通过名称指定错误注入的目标函数。如果函数名带有 '!' 前缀，则将从注入列表中移除给定函数。如果未指定任何内容（''），则清空注入列表。

- /sys/kernel/debug/fail_function/injectable:

	（只读）显示可注入错误的函数以及可以指定的错误值类型。错误类型将是以下之一；
 - NULL:	retval 必须为 0。
 - ERRNO: retval 必须为 -1 到 -MAX_ERRNO（-4096）。
 - ERR_NULL: retval 必须为 0 或 -1 到 -MAX_ERRNO（-4096）。

- /sys/kernel/debug/fail_function/<function-name>/retval:

	指定要注入到给定函数的"错误"返回值。当用户指定一个新的注入条目时会创建此文件。注意此文件只接受无符号值。因此，如果你想使用负的 errno，你最好使用 'printf' 而不是 'echo'，例如：
	$ printf %#x -12 > retval

- /sys/kernel/debug/fail_skb_realloc/devname:

        指定要强制进行 SKB 重新分配的网络接口。如果留空，SKB 重新分配将应用于所有网络接口。

```
          # 在 eth0 上强制进行 skb 重新分配
          echo "eth0" > /sys/kernel/debug/fail_skb_realloc/devname

          # 清除选择并在所有接口上强制进行 skb 重新分配
          echo "" > /sys/kernel/debug/fail_skb_realloc/devname
```

##### 引导选项


为了在 debugfs 不可用时（早期启动期间）注入故障，

```
	failslab=
	fail_page_alloc=
	fail_usercopy=
	fail_make_request=
	fail_futex=
	fail_skb_realloc=
	mmc_core.fail_request=<interval>,<probability>,<space>,<times>
```

##### proc 条目


- /proc/<pid>/fail-nth、
  /proc/self/task/<tid>/fail-nth:

	向此文件写入整数 N 会使该任务中的第 N 次调用失败。从此文件读取会返回一个整数值。值 '0' 表示用先前对此文件的写入所设置的故障已被注入。正整数 N 表示故障尚未被注入。注意此文件启用所有类型的故障（slab、futex 等）。此设置优先于所有其他通用的 debugfs 设置（如 probability、interval、times 等）。但每能力设置（例如 fail_futex/ignore-private）优先于它。

	此特性旨在用于单个系统调用的故障系统性测试。参见下面的例子。


### 可注入错误的函数


本部分面向考虑向 ALLOW_ERROR_INJECTION() 宏添加函数的内核开发者。


##### 可注入错误函数的要求


由于函数级错误注入会强行改变代码路径并返回错误，即使输入和条件都正确，如果允许对不可注入错误的函数进行错误注入，可能导致意外的内核崩溃。因此，你（和审阅者）必须确保：

- 函数在失败时会返回错误码，并且调用者必须正确检查它（需要能够从中恢复）。

- 函数在第一次错误返回之前不会执行任何可能改变任何状态的代码。该状态包括全局或局部，或输入变量。例如，清除输出地址存储（例如 `*ret = NULL`）、递增/递减计数器、设置标志、抢占/中断禁用或获取锁（如果这些在返回错误之前被恢复，则可以）。

第一个要求很重要，它会导致释放（释放对象）函数通常比分配函数更难注入错误。如果此类释放函数的错误没有被正确处理，很容易导致内存泄漏（调用者会误以为对象已被释放或已损坏）。

第二个要求是针对调用者的，它期望函数总是做一些事情。因此，如果函数的错误注入跳过了整个函数，这种期望就被违背了，并导致意外错误。


##### 可注入错误函数的类型


每个可注入错误的函数都会由 ALLOW_ERROR_INJECTION() 宏指定错误类型。如果你添加一个新的可注入错误函数，必须仔细选择它。如果选择了错误的错误类型，内核可能会崩溃，因为它可能无法处理该错误。在 include/asm-generic/error-injection.h 中定义了 4 种错误类型

EI_ETYPE_NULL
  此函数在失败时会返回 `NULL`。例如返回已分配对象的地址。

EI_ETYPE_ERRNO
  此函数在失败时会返回 `-errno` 错误码。例如当输入错误时返回 -EINVAL。这将包括那些通过 ERR_PTR() 宏返回编码了 `-errno` 的地址的函数。

EI_ETYPE_ERRNO_NULL
  此函数在失败时会返回 `-errno` 或 `NULL`。如果此函数的调用者使用 IS_ERR_OR_NULL() 宏检查返回值，则此类型是合适的。

EI_ETYPE_TRUE
  此函数在失败时会返回 `true`（非零的正值）。

如果你指定了错误的类型，例如为返回已分配对象的函数指定 EI_TYPE_ERRNO，可能会导致问题，因为返回值不是对象地址，调用者无法访问该地址。


### 如何添加新的故障注入能力


- #include <linux/fault-inject.h>

- 定义故障属性

  DECLARE_FAULT_ATTR(name);

  有关 struct fault_attr 的定义，请参阅 fault-inject.h 中的细节。

- 提供配置故障属性的方法

- 引导选项

  如果你需要从启动时就启用故障注入能力，可以提供引导选项来配置它。为此有一个辅助函数：

	setup_fault_attr(attr, str);

- debugfs 条目

  failslab、fail_page_alloc、fail_usercopy 和 fail_make_request 使用这种方式。辅助函数：

	fault_create_debugfs_attr(name, parent, attr);

- 模块参数

  如果故障注入能力的范围仅限于单个内核模块，最好提供模块参数来配置故障属性。

- 添加插入失败的钩子

  当 should_fail() 返回 true 时，客户端代码应注入一个失败：

	should_fail(attr, size);


### 应用示例


```
    #!/bin/bash

    FAILTYPE=failslab
    echo Y > /sys/kernel/debug/$FAILTYPE/task-filter
    echo 10 > /sys/kernel/debug/$FAILTYPE/probability
    echo 100 > /sys/kernel/debug/$FAILTYPE/interval
    echo -1 > /sys/kernel/debug/$FAILTYPE/times
    echo 0 > /sys/kernel/debug/$FAILTYPE/space
    echo 2 > /sys/kernel/debug/$FAILTYPE/verbose
    echo Y > /sys/kernel/debug/$FAILTYPE/ignore-gfp-wait

    faulty_system()
    {
	bash -c "echo 1 > /proc/self/make-it-fail && exec $*"
    }

    if [ $# -eq 0 ]
    then
	echo "Usage: $0 modulename [ modulename ... ]"
	exit 1
    fi

    for m in $*
    do
	echo inserting $m...
	faulty_system modprobe $m

	echo removing $m...
	faulty_system modprobe -r $m
    done
```

------------------------------------------------------------------------------

```
    #!/bin/bash

    FAILTYPE=fail_page_alloc
    module=$1

    if [ -z $module ]
    then
	echo "Usage: $0 <modulename>"
	exit 1
    fi

    modprobe $module

    if [ ! -d /sys/module/$module/sections ]
    then
	echo Module $module is not loaded
	exit 1
    fi

    cat /sys/module/$module/sections/.text > /sys/kernel/debug/$FAILTYPE/require-start
    cat /sys/module/$module/sections/.data > /sys/kernel/debug/$FAILTYPE/require-end

    echo N > /sys/kernel/debug/$FAILTYPE/task-filter
    echo 10 > /sys/kernel/debug/$FAILTYPE/probability
    echo 100 > /sys/kernel/debug/$FAILTYPE/interval
    echo -1 > /sys/kernel/debug/$FAILTYPE/times
    echo 0 > /sys/kernel/debug/$FAILTYPE/space
    echo 2 > /sys/kernel/debug/$FAILTYPE/verbose
    echo Y > /sys/kernel/debug/$FAILTYPE/ignore-gfp-wait
    echo Y > /sys/kernel/debug/$FAILTYPE/ignore-gfp-highmem
    echo 10 > /sys/kernel/debug/$FAILTYPE/stacktrace-depth

    trap "echo 0 > /sys/kernel/debug/$FAILTYPE/probability" SIGINT SIGTERM EXIT

    echo "Injecting errors into the module $module... (interrupt to stop)"
    sleep 1000000
```

------------------------------------------------------------------------------

```
    #!/bin/bash

    rm -f testfile.img
    dd if=/dev/zero of=testfile.img bs=1M seek=1000 count=1
    DEVICE=$(losetup --show -f testfile.img)
    mkfs.btrfs -f $DEVICE
    mkdir -p tmpmnt

    FAILTYPE=fail_function
    FAILFUNC=open_ctree
    echo $FAILFUNC > /sys/kernel/debug/$FAILTYPE/inject
    printf %#x -12 > /sys/kernel/debug/$FAILTYPE/$FAILFUNC/retval
    echo N > /sys/kernel/debug/$FAILTYPE/task-filter
    echo 100 > /sys/kernel/debug/$FAILTYPE/probability
    echo 0 > /sys/kernel/debug/$FAILTYPE/interval
    echo -1 > /sys/kernel/debug/$FAILTYPE/times
    echo 0 > /sys/kernel/debug/$FAILTYPE/space
    echo 1 > /sys/kernel/debug/$FAILTYPE/verbose

    mount -t btrfs $DEVICE tmpmnt
    if [ $? -ne 0 ]
    then
	echo "SUCCESS!"
    else
	echo "FAILED!"
	umount tmpmnt
    fi

    echo > /sys/kernel/debug/$FAILTYPE/inject

    rmdir tmpmnt
    losetup -d $DEVICE
    rm testfile.img
```

------------------------------------------------------------------------------

```
    # 将 skbuff_head_cache 标记为故障
    echo 1 > /sys/kernel/slab/skbuff_head_cache/failslab
    # 开启缓存过滤（默认关闭）
    echo 1 > /sys/kernel/debug/failslab/cache-filter
    # 开启故障注入
    echo 1 > /sys/kernel/debug/failslab/times
    echo 1 > /sys/kernel/debug/failslab/probability
```

### 用于运行带 failslab 或 fail_page_alloc 命令的工具

为了使上述任务更容易完成，我们可以使用 tools/testing/fault-injection/failcmd.sh。请运行命令 "./tools/testing/fault-injection/failcmd.sh --help" 获取更多信息并参见以下示例。

示例：

运行命令 "make -C tools/testing/selftests/ run_tests" 并注入 slab

```
	# ./tools/testing/fault-injection/failcmd.sh \
		-- make -C tools/testing/selftests/ run_tests
```

与上述相同，但指定最多 100 次失败而不是一次

```
	# ./tools/testing/fault-injection/failcmd.sh --times=100 \
		-- make -C tools/testing/selftests/ run_tests
```

与上述相同，但注入页分配失败而不是 slab

```
	# env FAILCMD_TYPE=fail_page_alloc \
		./tools/testing/fault-injection/failcmd.sh --times=100 \
		-- make -C tools/testing/selftests/ run_tests
```

### 使用 fail-nth 进行系统性故障

以下代码系统性地对第 0、1、2……次故障进行注入

```
  #include <sys/types.h>
  #include <sys/stat.h>
  #include <sys/socket.h>
  #include <sys/syscall.h>
  #include <fcntl.h>
  #include <unistd.h>
  #include <string.h>
  #include <stdlib.h>
  #include <stdio.h>
  #include <errno.h>

  int main()
  {
	int i, err, res, fail_nth, fds[2];
	char buf[128];

	system("echo N > /sys/kernel/debug/failslab/ignore-gfp-wait");
	sprintf(buf, "/proc/self/task/%ld/fail-nth", syscall(SYS_gettid));
	fail_nth = open(buf, O_RDWR);
	for (i = 1;; i++) {
		sprintf(buf, "%d", i);
		write(fail_nth, buf, strlen(buf));
		res = socketpair(AF_LOCAL, SOCK_STREAM, 0, fds);
		err = errno;
		pread(fail_nth, buf, sizeof(buf), 0);
		if (res == 0) {
			close(fds[0]);
			close(fds[1]);
		}
		printf("%d-th fault %c: res=%d/%d\n", i, atoi(buf) ? 'N' : 'Y',
			res, err);
		if (atoi(buf))
			break;
	}
	return 0;
  }
```

```
	1-th fault Y: res=-1/23
	2-th fault Y: res=-1/23
	3-th fault Y: res=-1/12
	4-th fault Y: res=-1/12
	5-th fault Y: res=-1/23
	6-th fault Y: res=-1/23
	7-th fault Y: res=-1/23
	8-th fault Y: res=-1/12
	9-th fault Y: res=-1/12
	10-th fault Y: res=-1/12
	11-th fault Y: res=-1/12
	12-th fault Y: res=-1/12
	13-th fault Y: res=-1/12
	14-th fault Y: res=-1/12
	15-th fault Y: res=-1/12
	16-th fault N: res=0/12
```
