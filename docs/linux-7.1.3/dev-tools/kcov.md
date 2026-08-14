## KCOV：用于模糊测试的覆盖率（code coverage）


KCOV 收集并以适合覆盖率引导模糊测试的形式暴露内核代码覆盖率信息。运行中内核的覆盖率数据通过 `kcov` debugfs 文件导出。覆盖率收集是按任务启用的，因此 KCOV 可以捕获单次系统调用的精确覆盖率。

注意，KCOV 的目标并非收集尽可能多的覆盖率。它的目标是收集大致稳定的、作为系统调用输入函数的覆盖率。为了实现这一目标，它不会在软/硬中断中收集覆盖率（除非启用了远程覆盖率收集，见下文），也不会从内核中一些本质上非确定性的部分（例如调度器、锁）收集。

除了收集代码覆盖率，KCOV 还可以收集比较操作数。详见 "Comparison operands collection" 一节。

除了从系统调用处理程序收集覆盖率数据，KCOV 还可以为在内核后台任务或软中断中执行的内核已注解部分收集覆盖率。详见 "Remote coverage collection" 一节。

### 先决条件


KCOV 依赖编译器插桩，需要 GCC 6.1.0 或更高版本，或者内核支持的任意 Clang 版本。

收集比较操作数受 GCC 8+ 或 Clang 支持。

```

        CONFIG_KCOV=y

```

```

	CONFIG_KCOV_ENABLE_COMPARISONS=y

```

```

        mount -t debugfs none /sys/kernel/debug

```
### 覆盖率收集


以下程序演示了如何在测试程序中使用 KCOV 为单次系统调用收集覆盖率：

```

    #include <stdio.h>
    #include <stddef.h>
    #include <stdint.h>
    #include <stdlib.h>
    #include <sys/types.h>
    #include <sys/stat.h>
    #include <sys/ioctl.h>
    #include <sys/mman.h>
    #include <unistd.h>
    #include <fcntl.h>
    #include <linux/types.h>

    #define KCOV_INIT_TRACE			_IOR('c', 1, unsigned long)
    #define KCOV_ENABLE			_IO('c', 100)
    #define KCOV_DISABLE			_IO('c', 101)
    #define COVER_SIZE			(64<<10)

    #define KCOV_TRACE_PC  0
    #define KCOV_TRACE_CMP 1

    int main(int argc, char **argv)
    {
	int fd;
	unsigned long *cover, n, i;

	/* A single fd descriptor allows coverage collection on a single
  - thread.
	 */
	fd = open("/sys/kernel/debug/kcov", O_RDWR);
	if (fd == -1)
		perror("open"), exit(1);
	/** Setup trace mode and trace size. **/
	if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
		perror("ioctl"), exit(1);
	/** Mmap buffer shared between kernel- and user-space. **/
	cover = (unsigned long**)mmap(NULL, COVER_SIZE ** sizeof(unsigned long),
				     PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	if ((void*)cover == MAP_FAILED)
		perror("mmap"), exit(1);
	/** Enable coverage collection on the current thread. **/
	if (ioctl(fd, KCOV_ENABLE, KCOV_TRACE_PC))
		perror("ioctl"), exit(1);
	/** Reset coverage from the tail of the ioctl() call. **/
	__atomic_store_n(&cover[^0^], 0, __ATOMIC_RELAXED);
	/** Call the target syscall call. **/
	read(-1, NULL, 0);
	/** Read number of PCs collected. **/
	n = __atomic_load_n(&cover[^0^], __ATOMIC_RELAXED);
	for (i = 0; i < n; i++)
		printf("0x%lx\n", cover[i + 1]);
	/* Disable coverage collection for the current thread. After this call
  - coverage can be enabled for a different thread.
	 */
	if (ioctl(fd, KCOV_DISABLE, 0))
		perror("ioctl"), exit(1);
	/** Free resources. **/
	if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
		perror("munmap"), exit(1);
	if (close(fd))
		perror("close"), exit(1);
	return 0;
    }

```

```

    SyS_read
    fs/read_write.c:562
    __fdget_pos
    fs/file.c:774
    __fget_light
    fs/file.c:746
    __fget_light
    fs/file.c:750
    __fget_light
    fs/file.c:760
    __fdget_pos
    fs/file.c:784
    SyS_read
    fs/read_write.c:562

```
如果程序需要从多个线程（各自独立地）收集覆盖率，则需要在每个线程中分别打开 `/sys/kernel/debug/kcov`。

该接口是细粒度的，以便高效地 fork 测试进程。也就是说，父进程打开 `/sys/kernel/debug/kcov`、启用 trace 模式、mmap 覆盖率缓冲区，然后在循环中 fork 子进程。子进程只需要启用覆盖率（当线程退出时它会自动禁用）。

### 比较操作数收集


比较操作数的收集与覆盖率收集类似：

```

    /** Same includes and defines as above. **/

    /** Number of 64-bit words per record. **/
    #define KCOV_WORDS_PER_CMP 4

    /*
     - The format for the types of collected comparisons.
     *
     - Bit 0 shows whether one of the arguments is a compile-time constant.
     - Bits 1 & 2 contain log2 of the argument size, up to 8 bytes.
     */

    #define KCOV_CMP_CONST          (1 << 0)
    #define KCOV_CMP_SIZE(n)        ((n) << 1)
    #define KCOV_CMP_MASK           KCOV_CMP_SIZE(3)

    int main(int argc, char **argv)
    {
	int fd;
	uint64_t *cover, type, arg1, arg2, is_const, size;
	unsigned long n, i;

	fd = open("/sys/kernel/debug/kcov", O_RDWR);
	if (fd == -1)
		perror("open"), exit(1);
	if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
		perror("ioctl"), exit(1);
	/*
 - Note that the buffer pointer is of type uint64_t*, because all
 - the comparison operands are promoted to uint64_t.
	*/
	cover = (uint64_t **)mmap(NULL, COVER_SIZE ** sizeof(unsigned long),
				     PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	if ((void*)cover == MAP_FAILED)
		perror("mmap"), exit(1);
	/** Note KCOV_TRACE_CMP instead of KCOV_TRACE_PC. **/
	if (ioctl(fd, KCOV_ENABLE, KCOV_TRACE_CMP))
		perror("ioctl"), exit(1);
	__atomic_store_n(&cover[^0^], 0, __ATOMIC_RELAXED);
	read(-1, NULL, 0);
	/** Read number of comparisons collected. **/
	n = __atomic_load_n(&cover[^0^], __ATOMIC_RELAXED);
	for (i = 0; i < n; i++) {
		uint64_t ip;

		type = cover[i * KCOV_WORDS_PER_CMP + 1];
		/** arg1 and arg2 - operands of the comparison. **/
		arg1 = cover[i * KCOV_WORDS_PER_CMP + 2];
		arg2 = cover[i * KCOV_WORDS_PER_CMP + 3];
		/** ip - caller address. **/
		ip = cover[i * KCOV_WORDS_PER_CMP + 4];
		/** size of the operands. **/
		size = 1 << ((type & KCOV_CMP_MASK) >> 1);
		/** is_const - true if either operand is a compile-time constant.**/
		is_const = type & KCOV_CMP_CONST;
		printf("ip: 0x%lx type: 0x%lx, arg1: 0x%lx, arg2: 0x%lx, "
			"size: %lu, %s\n",
			ip, type, arg1, arg2, size,
		is_const ? "const" : "non-const");
	}
	if (ioctl(fd, KCOV_DISABLE, 0))
		perror("ioctl"), exit(1);
	/** Free resources. **/
	if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
		perror("munmap"), exit(1);
	if (close(fd))
		perror("close"), exit(1);
	return 0;
    }

```
注意，KCOV 的各模式（收集代码覆盖率或比较操作数）是互斥的。

### 远程覆盖率收集


除了从用户空间进程发起的系统调用处理程序收集覆盖率数据，KCOV 还可以为在其他上下文中执行的内核部分收集覆盖率——即所谓的"远程"覆盖率。

使用 KCOV 收集远程覆盖率需要：

1. 修改内核代码，用 `kcov_remote_start` 和 `kcov_remote_stop` 注解应当从中收集覆盖率的代码段。

2. 在收集覆盖率的用户空间进程中使用 `KCOV_REMOTE_ENABLE` 代替 `KCOV_ENABLE`。

`kcov_remote_start` 和 `kcov_remote_stop` 注解以及 `KCOV_REMOTE_ENABLE` ioctl 都接受用于标识特定覆盖率收集段的句柄。句柄的使用方式取决于匹配代码段执行的上下文。

KCOV 支持从以下上下文收集远程覆盖率：

1. 全局内核后台任务。这些是在内核启动期间生成、实例数量有限的任务（例如每个 USB HCD 生成一个 USB `hub_event` worker）。

2. 本地内核后台任务。这些是在用户空间进程与某些内核接口交互时生成、通常在该进程退出时被杀掉的任务（例如 vhost workers）。

3. 软中断。

对于 #1 和 #3，必须选择一个唯一的全局句柄并传递给相应的 `kcov_remote_start` 调用。然后用户空间进程必须将该句柄通过 `kcov_remote_arg` 结构体的 `handles` 数组字段传递给 `KCOV_REMOTE_ENABLE`。这会将所使用的 KCOV 设备附加到该句柄所引用的代码段。可以同时传递标识不同代码段的多个全局句柄。

对于 #2，用户空间进程必须通过 `kcov_remote_arg` 结构体的 `common_handle` 字段传递一个非零句柄。该公共句柄会被保存到当前 `task_struct` 的 `kcov_handle` 字段中，并且需要通过自定义的内核代码修改传递给新生成的本地任务。这些任务反过来应当在它们的 `kcov_remote_start` 和 `kcov_remote_stop` 注解中使用所传递的句柄。

KCOV 对全局句柄和公共句柄都遵循预定义格式。每个句柄是一个 `u64` 整数。目前只使用了最高字节和较低的 4 字节。字节 4-7 保留，必须为零。

对于全局句柄，句柄的最高字节表示它所属子系统的 id。例如，KCOV 使用 `1` 作为 USB 子系统 id。全局句柄较低的 4 字节表示该系统内任务实例的 id。例如，每个 `hub_event` worker 使用 USB 总线号作为任务实例 id。

对于公共句柄，保留值 `0` 被用作子系统 id，因为此类句柄不属于某个特定子系统。公共句柄较低的 4 字节标识由向 `KCOV_REMOTE_ENABLE` 传递公共句柄的用户空间进程所生成的所有本地任务的集合实例。

在实践中，如果覆盖率仅从系统上单个用户空间进程收集，则公共句柄实例 id 可以使用任意值。但是，如果公共句柄被多个进程使用，则必须为每个进程使用唯一的实例 id。一种选择是使用进程 id 作为公共句柄实例 id。

以下程序演示了使用 KCOV 从进程生成的本地任务以及处理 USB 总线 #1 的全局任务收集覆盖率：

```

    /** Same includes and defines as above. **/

    struct kcov_remote_arg {
	__u32		trace_mode;
	__u32		area_size;
	__u32		num_handles;
	__aligned_u64	common_handle;
	__aligned_u64	handles[^0^];
    };

    #define KCOV_INIT_TRACE			_IOR('c', 1, unsigned long)
    #define KCOV_DISABLE			_IO('c', 101)
    #define KCOV_REMOTE_ENABLE		_IOW('c', 102, struct kcov_remote_arg)

    #define COVER_SIZE	(64 << 10)

    #define KCOV_TRACE_PC	0

    #define KCOV_SUBSYSTEM_COMMON	(0x00ull << 56)
    #define KCOV_SUBSYSTEM_USB	(0x01ull << 56)

    #define KCOV_SUBSYSTEM_MASK	(0xffull << 56)
    #define KCOV_INSTANCE_MASK	(0xffffffffull)

    static inline __u64 kcov_remote_handle(__u64 subsys, __u64 inst)
    {
	if (subsys & ~KCOV_SUBSYSTEM_MASK || inst & ~KCOV_INSTANCE_MASK)
		return 0;
	return subsys | inst;
    }

    #define KCOV_COMMON_ID	0x42
    #define KCOV_USB_BUS_NUM	1

    int main(int argc, char **argv)
    {
	int fd;
	unsigned long *cover, n, i;
	struct kcov_remote_arg *arg;

	fd = open("/sys/kernel/debug/kcov", O_RDWR);
	if (fd == -1)
		perror("open"), exit(1);
	if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
		perror("ioctl"), exit(1);
	cover = (unsigned long**)mmap(NULL, COVER_SIZE ** sizeof(unsigned long),
				     PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	if ((void*)cover == MAP_FAILED)
		perror("mmap"), exit(1);

	/** Enable coverage collection via common handle and from USB bus #1. **/
	arg = calloc(1, sizeof(*arg) + sizeof(uint64_t));
	if (!arg)
		perror("calloc"), exit(1);
	arg->trace_mode = KCOV_TRACE_PC;
	arg->area_size = COVER_SIZE;
	arg->num_handles = 1;
	arg->common_handle = kcov_remote_handle(KCOV_SUBSYSTEM_COMMON,
							KCOV_COMMON_ID);
	arg->handles[^0^] = kcov_remote_handle(KCOV_SUBSYSTEM_USB,
						KCOV_USB_BUS_NUM);
	if (ioctl(fd, KCOV_REMOTE_ENABLE, arg))
		perror("ioctl"), free(arg), exit(1);
	free(arg);

	/*
  - Here the user needs to trigger execution of a kernel code section
  - that is either annotated with the common handle, or to trigger some
  - activity on USB bus #1.
	 */
	sleep(2);

        /*
         - The load to the coverage count should be an acquire to pair with
         - pair with the corresponding write memory barrier (smp_wmb()) on
         - the kernel-side in kcov_move_area().
         */
	n = __atomic_load_n(&cover[^0^], __ATOMIC_ACQUIRE);
	for (i = 0; i < n; i++)
		printf("0x%lx\n", cover[i + 1]);
	if (ioctl(fd, KCOV_DISABLE, 0))
		perror("ioctl"), exit(1);
	if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
		perror("munmap"), exit(1);
	if (close(fd))
		perror("close"), exit(1);
	return 0;
    }

```
