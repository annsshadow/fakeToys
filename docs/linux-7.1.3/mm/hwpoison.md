## hwpoison


## 什么是 hwpoison？


即将推出的 Intel CPU 支持从某些内存错误中恢复（`MCA recovery`）。这要求操作系统将某个页声明为“已中毒（poisoned）”，杀死与之关联的进程，并在今后避免使用该页。

该补丁集在 VM（虚拟内存管理）中实现了所需的基础设施。

```
	高层机器检查处理程序。处理由硬件报告为已损坏的页，通常是由于 2 位 ECC 内存或
	缓存故障所致。

	这侧重于在后台检测为已损坏的页。当当前 CPU 试图消费损坏数据时，正在运行的
	进程可以直接被杀死。这意味着如果由于某种原因无法处理该错误，可以安全地忽略它，
	因为尚未消费任何损坏数据。反之，当这种情况发生时，会再次发生一次机器检查。

	处理处于各种状态的页缓存页。这里棘手的部分在于，我们可以异步于其他 VM 用户
	访问任意页，因为内存故障可能随时随地在任何地方发生，可能会违反它们的一些
	假设。这就是为什么此代码必须极为小心。通常它尝试使用正常的加锁规则，即获取
	标准锁，即使这意味着错误处理可能需要较长的时间。

	这里的某些操作效率较低且具有非线性的算法复杂度，因为数据结构尚未针对这种
	情况优化。从 vma 到进程的映射尤其如此。由于这种情况预计很少见，我们希望可以
	接受这一点。```
代码由 mm/memory-failure.c 中的高层处理程序、一个新的页中毒标志，以及在 VM 中用于处理中毒页的各种检查组成。

目前的主要目标是 KVM 客户机，但它也适用于各类应用程序。KVM 支持需要较新的 qemu-kvm 版本。

为了 KVM 的使用，需要一种新的信号类型，以便 KVM 能够将机器检查以正确的地址注入到客户机中。这在理论上也允许其他应用程序处理内存故障。预计大多数应用程序不会这么做，但某些非常专用的应用程序可能会。

## 故障恢复模式


内存故障恢复可以处于两种（实际上是三种）模式：

vm.memory_failure_recovery sysctl 设为零：
	所有内存故障都会导致 panic。不要尝试恢复。

early kill
	（可在全局和每个进程级别控制）
	一旦检测到错误就向应用程序发送 SIGBUS
	这允许能够以温和方式处理内存错误的应用程序
	（例如丢弃受影响对象）
	这是 KVM qemu 使用的模式。

late kill
	当应用程序遇到损坏的页时发送 SIGBUS。
	这对于没有内存错误感知的应用程序最合适，也是默认模式
	注意某些页始终按 late kill 方式处理。

## 用户控制


vm.memory_failure_recovery
	见 sysctl.txt

vm.memory_failure_early_kill
	在全局启用 early kill 模式

PR_MCE_KILL
	设置 early/late kill 模式 / 恢复为系统默认

	arg1: PR_MCE_KILL_CLEAR:
		恢复为系统默认
	arg1: PR_MCE_KILL_SET:
		arg2 定义线程特定的模式

		PR_MCE_KILL_EARLY:
			Early kill
		PR_MCE_KILL_LATE:
			Late kill
		PR_MCE_KILL_DEFAULT
			使用系统全局默认

	注意，如果你希望有一个专用线程代表进程处理
	SIGBUS(BUS_MCEERR_AO)，你应该在指定线程上调用
	prctl(PR_MCE_KILL_EARLY)。否则，SIGBUS 会被发送给主线程。

PR_MCE_KILL_GET
	返回当前模式

## 测试


- madvise(MADV_HWPOISON, ....)（以 root 身份）- 在进程中毒化一个页以用于测试

- 通过 debugfs `/sys/kernel/debug/hwpoison/` 的 hwpoison-inject 模块

  corrupt-pfn
	向回显到该文件的 PFN 处注入 hwpoison 故障。这会做一些
	早期过滤，以避免在测试套件中损坏非预期的页。

  unpoison-pfn
	对回显到该文件的 PFN 处的页进行软件解毒。这样
	该页可以再次被使用。这仅对 Linux 注入的故障有效，
	对真实的内存故障无效。一旦发生任何硬件内存故障，
	该特性将被禁用。

  注意这些注入接口并不稳定，可能会在不同内核版本之间发生变化

  corrupt-filter-dev-major, corrupt-filter-dev-minor
	仅处理与由块设备主/次设备号定义
	的文件系统相关联的页。-1U 为通配值。这应仅用于
	人工注入的测试。

  corrupt-filter-memcg
	将注入限制为属于 memgroup 的页。由 memcg 的 inode
	号指定。

```

		mkdir /sys/fs/cgroup/mem/hwpoison

	        usemem -m 100 -s 1000 &
		echo `jobs -p` > /sys/fs/cgroup/mem/hwpoison/tasks

		memcg_ino=$(ls -id /sys/fs/cgroup/mem/hwpoison | cut -f1 -d' ')
		echo $memcg_ino > /debug/hwpoison/corrupt-filter-memcg

		page-types -p `pidof init`   --hwpoison  # shall do nothing
		page-types -p `pidof usemem` --hwpoison  # poison its pages

  corrupt-filter-flags-mask, corrupt-filter-flags-value
	当指定时，仅当 ((page_flags & mask) == value)
	时才毒化页。这允许对多种类型的页进行压力测试。
	page_flags 与 /proc/kpageflags 中相同。标志位定义于
	include/linux/kernel-page-flags.h，并在
	Documentation/admin-guide/mm/pagemap.rst 中有文档说明。

```
- 架构特定的 MCE 注入器

  x86 有 mce-inject、mce-test

  mce-test 中一些可移植的 hwpoison 测试程序，见下文。

```
## 参考资料


http://halobates.de/mce-lc09-2.pdf
	LinuxCon 09 上的概述演讲

git://git.kernel.org/pub/scm/utils/cpu/mce/mce-test.git
	测试套件（可移植的 hwpoison 专用测试位于 tsrc）

git://git.kernel.org/pub/scm/utils/cpu/mce/mce-inject.git
	x86 特定的注入器


## 局限性

- 并非所有页类型都受支持，也永远不会全部支持。大多数内核内部
  对象无法恢复，目前仅支持 LRU 页。

---
Andi Kleen, Oct 2009
