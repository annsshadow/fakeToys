## 延迟统计（Delay accounting）


当任务等待某个内核资源变为可用时（例如，一个可运行任务可能要等待一个空闲 CPU 来运行），其执行就会遇到延迟。

每任务延迟统计功能测量一个任务在以下情形中经历的延迟：

a) 等待 CPU（处于可运行状态时）
b) 该任务发起的同步块 I/O 完成
c) 换入页（swap in）
d) 内存回收（memory reclaim）
e) 颠簸（thrashing）
f) 直接内存规整（direct compact）
g) 写保护拷贝（write-protect copy）
h) IRQ/SOFTIRQ

并通过 taskstats 接口把这些统计信息提供给用户空间。

这类延迟为合理设置任务的 CPU 优先级、I/O 优先级和 RSS 限制值提供了反馈。重要任务的长时间延迟，可能成为提升其相应优先级的触发条件。

该功能借助 taskstats 接口，还提供属于某个线程组（对应于传统 Unix 进程）的所有任务（或线程）的聚合延迟统计。这是一种通常需要的聚合，由内核来完成效率更高。

用户空间工具——尤其是资源管理类应用——也可以把延迟统计聚合成任意分组。为此，任务的延迟统计在其生命周期内以及退出时都可用，从而确保能够进行连续且完整的监控。


### 接口


延迟统计使用 taskstats 接口，该接口在本目录的单独文档中有详细描述。Taskstats 向用户空间返回一个对应于每 PID 和每 TGID 统计的通用数据结构。延迟统计功能填充该结构的特定字段。参见

     include/uapi/linux/taskstats.h

了解与延迟统计相关字段的说明。这些字段通常采用计数器的形式，返回针对 CPU、同步块 I/O、swapin、内存回收、颠簸页缓存、直接规整、写保护拷贝、IRQ/SOFTIRQ 等所观察到的累计延迟。

对某个给定计数器（例如 cpu_delay_total）的两次连续读数取差值，即可得到该任务在该时间间隔内等待相应资源所经历的延迟。

当任务退出时，包含每任务统计的记录会在无需命令的情况下发送给用户空间。如果它是某个线程组最后一个退出的任务，每 TGID 统计也会一并发送。更多细节见 taskstats 接口描述。

tools/accounting 目录下的 getdelays.c 用户空间工具可以运行简单命令并显示相应的延迟统计，它同时也作为使用 taskstats 接口的一个示例。

### 用法


```

	CONFIG_TASK_DELAY_ACCT=y
	CONFIG_TASKSTATS=y

```
延迟统计在启动默认是关闭的。
```

   delayacct

```
加入内核启动选项。下面其余的说明都假定已执行此操作。或者，也可以使用 sysctl kernel.task_delayacct 在运行时切换状态。但需注意，只有在其启用之后启动的任务才会拥有 delayacct 信息。

系统启动后，使用一个类似 getdelays.c 的工具来访问给定任务或任务组（tgid）所经历的延迟。该工具也允许执行给定命令并查看相应的延迟。

```

	getdelays [-dilv] [-t tgid] [-p pid]

```
```

	# ./getdelays -d -p 10
	(output similar to next case)

```
```

	bash-4.4# ./getdelays -d -t 242
	print delayacct stats ON
	TGID    242




	CPU         count     real total  virtual total    delay total  delay average      delay max      delay min      delay max timestamp
	               46      188000000      192348334        4098012          0.089ms     0.429260ms     0.051205ms    2026-01-15T15:06:58
	IO          count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	SWAP        count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	RECLAIM     count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	THRASHING   count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	COMPACT     count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A
	WPCOPY      count    delay total  delay average      delay max      delay min      delay max timestamp
	              182       19413338          0.107ms     0.547353ms     0.022462ms    2026-01-15T15:05:24
	IRQ         count    delay total  delay average      delay max      delay min      delay max timestamp
	                0              0          0.000ms     0.000000ms     0.000000ms                    N/A

```
```

	# ./getdelays -i -p 1
	printing IO accounting
	linuxrc: read=65536, write=0, cancelled_write=0

```
上述命令可与 -v 一起使用以获取更多调试信息。

系统启动后，使用 `delaytop` 获取系统级延迟信息，其中包含系统级 PSI 信息和延迟最高的 Top-N 任务。
注意：PSI 支持需要 `CONFIG_PSI=y` 以及 `psi=1` 才能完整体工作。

`delaytop` 是一个用于监控系统压力与任务延迟的交互式工具。它支持多种排序选项、显示模式以及实时键盘控制。

```

	bash# ./delaytop
	System Pressure Information: (avg10/avg60vg300/total)
	CPU some:       0.0%/   0.0%/   0.0%/  106137(ms)
	CPU full:       0.0%/   0.0%/   0.0%/       0(ms)
	Memory full:    0.0%/   0.0%/   0.0%/       0(ms)
	Memory some:    0.0%/   0.0%/   0.0%/       0(ms)
	IO full:        0.0%/   0.0%/   0.0%/    2240(ms)
	IO some:        0.0%/   0.0%/   0.0%/    2783(ms)
	IRQ full:       0.0%/   0.0%/   0.0%/       0(ms)
	[o]sort [M]memverbose [q]quit
	Top 20 processes (sorted by cpu delay):
		PID      TGID  COMMAND           CPU(ms)   IO(ms)  IRQ(ms)  MEM(ms)
	------------------------------------------------------------------------
		110       110  kworker/15:0H-s   27.91     0.00     0.00     0.00
		57        57  cpuhp/7            3.18     0.00     0.00     0.00
		99        99  cpuhp/14           2.97     0.00     0.00     0.00
		51        51  cpuhp/6            0.90     0.00     0.00     0.00
		44        44  kworker/4:0H-sy    0.80     0.00     0.00     0.00
		60        60  ksoftirqd/7        0.74     0.00     0.00     0.00
		76        76  idle_inject/10     0.31     0.00     0.00     0.00
		100       100  idle_inject/14     0.30     0.00     0.00     0.00
		1309      1309  systemsettings     0.29     0.00     0.00     0.00
		45        45  cpuhp/5            0.22     0.00     0.00     0.00
		63        63  cpuhp/8            0.20     0.00     0.00     0.00
		87        87  cpuhp/12           0.18     0.00     0.00     0.00
		93        93  cpuhp/13           0.17     0.00     0.00     0.00
		1265      1265  acpid              0.17     0.00     0.00     0.00
		1552      1552  sshd               0.17     0.00     0.00     0.00
		2584      2584  sddm-helper        0.16     0.00     0.00     0.00
		1284      1284  rtkit-daemon       0.15     0.00     0.00     0.00
		1326      1326  nde-netfilter      0.14     0.00     0.00     0.00
		27        27  cpuhp/2            0.13     0.00     0.00     0.00
		631       631  kworker/11:2-rc    0.11     0.00     0.00     0.00

```
```

	o - Select sort field (CPU, IO, IRQ, Memory, etc.)
	M - Toggle display mode (Default/Memory Verbose)
	q - Quit

```
```

	cpu(c)       - CPU delay
	blkio(i)     - I/O delay
	irq(q)       - IRQ delay
	mem(m)       - Total memory delay
	swapin(s)    - Swapin delay (memory verbose mode only)
	freepages(r) - Freepages reclaim delay (memory verbose mode only)
	thrashing(t) - Thrashing delay (memory verbose mode only)
	compact(p)   - Compaction delay (memory verbose mode only)
	wpcopy(w)    - Write page copy delay (memory verbose mode only)

```
```

	# ./delaytop -s blkio
	Sorted by IO delay

	# ./delaytop -s mem -M
	Sorted by memory delay in memory verbose mode

	# ./delaytop -p pid
	Print delayacct stats

	# ./delaytop -P num
	Display the top N tasks

	# ./delaytop -n num
	Set delaytop refresh frequency (num times)

	# ./delaytop -d secs
	Specify refresh interval as secs

```
