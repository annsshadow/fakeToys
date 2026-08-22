

## PSI —压力阻塞信息（Pressure Stall Information


:Date: April, 2018
:Author: Johannes Weiner <hannes@cmpxchg.org>

CPU、内存或 IO 设备发生竞争时，工作负载会经历延迟尖峰、吞吐量下降，并面临
OOM 杀死的风险

如果没有对此类竞争的精确度量，用户被迫要么求稳而低效率地利用硬件资源，要么孤注一掷
频繁遭受过度提交（overcommit）所导致的混乱

psi 特性识别并量化了此类资源紧张所造成的干扰，以及它对复杂工作负载甚至整个系统
所造成的时间影响

拥有对资源稀缺所导致生产率损失的精确度量，有助于用户按照硬件规模规划工作负载
或根据工作负载需求来配置硬件

由于 psi 实时聚合这些信息，系统可以使用诸如卸载负载（load shedding）、将作业迁移
其他系统或数据中心，或策略性地暂停、杀死低优先级或可重启的批处理作业等动态管理技术

这使得在最大化硬件利用率的同时，不会牺牲工作负载健康，也不会冒 OOM 杀死等严重混乱的风险

## 压力接口


每种资源的压力信息通过 /proc/pressure/ 下对应的文件导出——cpu、memory io


```

	some avg10=0.00 avg60=0.00 avg300=0.00 total=0
	full avg10=0.00 avg60=0.00 avg300=0.00 total=0

```
“some行表示至少有一部分任务在某个给定资源上被阻塞的时间占比

“full行表示所有非空闲任务同时被阻塞在某个给定资源上的时间占比。在这种状态下
实际CPU 周期被浪费，而长时间处于这种状态的工作负载被视为在发生抖动（thrashing）
这会对性能产生严重影响，因此有必要将此情形与“部分任务被阻塞CPU 仍在进行有效工作
的状态区分开来。因此，处于该阻塞状态子集中的时间被单独追踪，并在“full”平均值中导出

CPU full 在系统级别是未定义的，但5.13 起已被导出，因此为了向后兼容它被设为 0

这些比率（百分比）被作为最近十秒、六十秒与三百秒窗口上的近期趋势来追踪，从而既可以
洞察短期事件，也能观察中长期趋势。总的绝对阻塞时间（单位为 us）也被追踪并导出，以
检测到那些不一定会在时间平均值上留下痕迹的延迟尖峰，或用于在自定义时间帧上计算平均趋势

## 监控压力阈


用户可以注册触发器，并在资源压力超过某些阈值时使用 poll() 被唤醒

触发器描述了在特定时间窗口内的累计最大阻塞时间，例如在任500ms 窗口内累100ms 
阻塞时间以产生一次唤醒事件

要注册触发器，用户必须打开 /proc/pressure/ 下代表待监控资源psi 接口文件，并写入
期望的阈值与时间窗口。打开的文件描述符应使select()、poll() epoll() 来等待触发事件

```

	<some|full> <stall amount in us> <time window in us>

```
例如，向 /proc/pressure/memory 写入 “some 150000 1000000会添150ms 的部分内存阻
阈值（1 秒时间窗口内测量）。向 /proc/pressure/io 写入 “full 50000 1000000会添
50ms 的完io 阻塞阈值（1 秒时间窗口内测量）

可以为同一psi 指标设置多个触发器，也可以为同一psi 指标指定多个触发器。但每个触发
都需要一个独立的文件描述符，以便能够与其他触发器分别进行 poll，因此即使打开的是同一
psi 接口文件，也应针对每个触发器分别进行 open() 系统调用。对已存psi 触发器的文件描述
执行写操作将EBUSY 而失败

监视器仅在系统进入被监控 psi 指标的阻塞状态时才激活，并在退出阻塞状态时停用。当系统处于
阻塞状态时，psi 信号的增长以每个跟踪窗口 10 次的频率被监控

内核接受的窗口大小范围为 500ms 10s，因此最小监控更新间隔为 50ms，最大为 1s。设置最
限制是为了防止过于频繁的轮询。最大限制被选定为一个足够大的数值，超过之后通常不再需要监视器
而可以改psi 平均值

非特权用户也可以创建监视器，唯一的限制是窗口大小必须2s 的倍数，以防止过度消耗资源

激活后，psi 监视器会保持激活至少为一个跟踪窗口的时长，以避免在系统进出阻塞状态时反复激停用

向用户空间的通知被限制为每个跟踪窗口一次

当用于定义触发器的文件描述符被关闭时，该触发器将注销

## 用户空间监视器使用示



```

  #include <errno.h>
  #include <fcntl.h>
  #include <stdio.h>
  #include <poll.h>
  #include <string.h>
  #include <unistd.h>

  /*
   * Monitor memory partial stall with 1s tracking window size
   * and 150ms threshold.
   */
  int main() {
	const char trig[] = "some 150000 1000000";
	struct pollfd fds;
	int n;

	fds.fd = open("/proc/pressure/memory", O_RDWR | O_NONBLOCK);
	if (fds.fd < 0) {
		printf("/proc/pressure/memory open error: %s\n",
			strerror(errno));
		return 1;
	}
	fds.events = POLLPRI;

	if (write(fds.fd, trig, strlen(trig) + 1) < 0) {
		printf("/proc/pressure/memory write error: %s\n",
			strerror(errno));
		return 1;
	}

	printf("waiting for events...\n");
	while (1) {
		n = poll(&fds, 1, -1);
		if (n < 0) {
			printf("poll error: %s\n", strerror(errno));
			return 1;
		}
		if (fds.revents & POLLERR) {
			printf("got POLLERR, event source is gone\n");
			return 0;
		}
		if (fds.revents & POLLPRI) {
			printf("event triggered!\n");
		} else {
			printf("unknown event received: 0x%x\n", fds.revents);
			return 1;
		}
	}

	return 0;
  }

```
## Cgroup2 接口


在一个启用了 CONFIG_CGROUPS=y 内核并挂载了 cgroup2 文件系统的系统中，压力阻塞信息也
针对分组cgroup 中的任务进行追踪。cgroupfs 挂载点下的每个子目录都包cpu.pressure
memory.pressure io.pressure 文件；其格式/proc/pressure/ 文件相同

每个 cgroup psi 监视器可以像系统级的监视器一样被指定和使用
