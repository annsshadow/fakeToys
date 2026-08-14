## 块设备 IO 优先级


### 简介


io 优先级特性使用户能够对进程或进程组进行 io nice 设置，类似于长期以来对 cpu 调度所可能做到的事情。
对 io 优先级的支持取决于 io 调度器，目前由 bfq 和 mq-deadline 支持。

### 调度类


为 io 优先级实现了三个通用的调度类，它们决定了一个进程的 io 如何被服务。

IOPRIO_CLASS_RT：这是实时 io 类。此调度类被赋予比系统中任何其他类更高的优先级，来自此类的进程每次都
优先访问磁盘。因此使用它需要一些谨慎，一个 io RT 进程可能让整个系统饿死。在 RT 类内部，有 8 个级别的
类数据，用于精确决定该进程每次服务需要多少磁盘时间。将来这可能会改变为更可直接映射到性能，通过传入一个
期望的数据速率来代替。

IOPRIO_CLASS_BE：这是尽力而为（best-effort）调度类，是任何未设置特定 io 优先级的进程的默认类。类数据
决定该进程将获得多少 io 带宽，它可直接映射到 cpu nice 级别，只是实现得更粗略。0 是最高 BE 优先级级别，
7 是最低级别。cpu nice 级别与 io nice 级别之间的映射确定为：io_nice = (cpu_nice + 20) / 5。

IOPRIO_CLASS_IDLE：这是空闲调度类，运行在此级别的进程仅当没有其他任何人需要磁盘时才会获得 io 时间。
空闲类没有类数据，因为在这里它确实不适用。

### 工具


```

	# ionice -c<class> -n<level> -p<pid>

```
如果未给定 pid，则假定为当前进程。IO 优先级设置在 fork 时被继承，因此你可以使用 ionice 在给定
```

	# ionice -c2 -n0 /bin/ls

```
下启动进程，将以最高优先级的尽力而为调度类运行 ls。
```

	# ionice -c1 -n2 -p100

```
会将 pid 100 更改为以实时调度类、优先级 2 运行。

```

  #include <stdio.h>
  #include <stdlib.h>
  #include <errno.h>
  #include <getopt.h>
  #include <unistd.h>
  #include <sys/ptrace.h>
  #include <asm/unistd.h>

  extern int sys_ioprio_set(int, int, int);
  extern int sys_ioprio_get(int, int);

  #if defined(__i386__)
  #define __NR_ioprio_set		289
  #define __NR_ioprio_get		290
  #elif defined(__ppc__)
  #define __NR_ioprio_set		273
  #define __NR_ioprio_get		274
  #elif defined(__x86_64__)
  #define __NR_ioprio_set		251
  #define __NR_ioprio_get		252
  #else
  #error "Unsupported arch"
  #endif

  static inline int ioprio_set(int which, int who, int ioprio)
  {
	return syscall(__NR_ioprio_set, which, who, ioprio);
  }

  static inline int ioprio_get(int which, int who)
  {
	return syscall(__NR_ioprio_get, which, who);
  }

  enum {
	IOPRIO_CLASS_NONE,
	IOPRIO_CLASS_RT,
	IOPRIO_CLASS_BE,
	IOPRIO_CLASS_IDLE,
  };

  enum {
	IOPRIO_WHO_PROCESS = 1,
	IOPRIO_WHO_PGRP,
	IOPRIO_WHO_USER,
  };

  #define IOPRIO_CLASS_SHIFT	13

  const char *to_prio[] = { "none", "realtime", "best-effort", "idle", };

  int main(int argc, char *argv[])
  {
	int ioprio = 4, set = 0, ioprio_class = IOPRIO_CLASS_BE;
	int c, pid = 0;

	while ((c = getopt(argc, argv, "+n:c:p:")) != EOF) {
		switch (c) {
		case 'n':
			ioprio = strtol(optarg, NULL, 10);
			set = 1;
			break;
		case 'c':
			ioprio_class = strtol(optarg, NULL, 10);
			set = 1;
			break;
		case 'p':
			pid = strtol(optarg, NULL, 10);
			break;
		}
	}

	switch (ioprio_class) {
		case IOPRIO_CLASS_NONE:
			ioprio_class = IOPRIO_CLASS_BE;
			break;
		case IOPRIO_CLASS_RT:
		case IOPRIO_CLASS_BE:
			break;
		case IOPRIO_CLASS_IDLE:
			ioprio = 7;
			break;
		default:
			printf("bad prio class %d\n", ioprio_class);
			return 1;
	}

	if (!set) {
		if (!pid && argv[optind])
			pid = strtol(argv[optind], NULL, 10);

		ioprio = ioprio_get(IOPRIO_WHO_PROCESS, pid);

		printf("pid=%d, %d\n", pid, ioprio);

		if (ioprio == -1)
			perror("ioprio_get");
		else {
			ioprio_class = ioprio >> IOPRIO_CLASS_SHIFT;
			ioprio = ioprio & 0xff;
			printf("%s: prio %d\n", to_prio[ioprio_class], ioprio);
		}
	} else {
		if (ioprio_set(IOPRIO_WHO_PROCESS, pid, ioprio | ioprio_class << IOPRIO_CLASS_SHIFT) == -1) {
			perror("ioprio_set");
			return 1;
		}

		if (argv[optind])
			execvp(argv[optind], &argv[optind]);
	}

	return 0;
  }


```
March 11 2005, Jens Axboe <jens.axboe@oracle.com>
