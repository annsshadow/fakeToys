## CPU 负载


Linux 通过 `/proc/stat` 和 `/proc/uptime` 导出各种信息，用户态工具（如 top(1)）利用这些信息来计算

```
    $ iostat
    Linux 2.6.18.3-exp (linmac)     02/20/2007

    avg-cpu:  %user   %nice %system %iowait  %steal   %idle
              10.01    0.00    2.92    5.44    0.00   81.63

    ...

```

此处系统认为，在默认采样周期内，系统有 10.01% 的时间在用户空间执行工作，2.92% 在内核中，整体空闲时间为 81.63%。

在大多数情况下，`/proc/stat` 反映的情况与现实相当接近，但由于内核采集此数据的时机与方式所限，有时它完全不可信。

那么这些信息是如何采集的呢？每当定时器中断触发时，内核会查看当前正在运行的任务类型，并让与该任务类型/状态对应的计数器加一。问题在于，在两次定时器中断之间，系统可能在多种状态之间切换了多次，但计数器只针对最后一种状态进行了累加。


### 示例


设想一个系统，其中有一个任务周期性地消耗 CPU 周期

```

     time line between two timer interrupts
    |--------------------------------------|
     ^                                    ^
     |_ something begins working          |
                                          |_ something goes to sleep
                                         (only to be awaken quite soon)

```

在上述情形下，根据 `/proc/stat` 的判断，系统负载为 0%（因为定时器中断总是发生在系统执行 idle 处理程序时），但实际上负载更接近 99%。

人们可以设想更多此类内核行为导致偏差的情形

```


	/* gcc -o hog smallhog.c */
	#include <time.h>
	#include <limits.h>
	#include <signal.h>
	#include <sys/time.h>
	#define HIST 10

	static volatile sig_atomic_t stop;

	static void sighandler(int signr)
	{
		(void) signr;
		stop = 1;
	}

	static unsigned long hog (unsigned long niters)
	{
		stop = 0;
		while (!stop && --niters);
		return niters;
	}

	int main (void)
	{
		int i;
		struct itimerval it = {
			.it_interval = { .tv_sec = 0, .tv_usec = 1 },
			.it_value    = { .tv_sec = 0, .tv_usec = 1 } };
		sigset_t set;
		unsigned long v[HIST];
		double tmp = 0.0;
		unsigned long n;
		signal(SIGALRM, &sighandler);
		setitimer(ITIMER_REAL, &it, NULL);

		hog (ULONG_MAX);
		for (i = 0; i < HIST; ++i) v[i] = ULONG_MAX - hog(ULONG_MAX);
		for (i = 0; i < HIST; ++i) tmp += v[i];
		tmp /= HIST;
		n = tmp - (tmp / 3.0);

		sigemptyset(&set);
		sigaddset(&set, SIGALRM);

		for (;;) {
			hog(n);
			sigwait(&set, &i);
		}
		return 0;
	}


```

### 参考

- https://lore.kernel.org/r/loom.20070212T063225-663@post.gmane.org
- Documentation/filesystems/proc.rst (1.8)


### 致谢


Con Kolivas, Pavel Machek
