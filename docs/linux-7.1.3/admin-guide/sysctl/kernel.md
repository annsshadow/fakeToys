## Documentation for /proc/sys/kernel/


Copyright (c) 1998, 1999,  Rik van Riel <riel@nl.linux.org>

Copyright (c) 2009,        Shen Feng<shen@cn.fujitsu.com>

有关总体信息与法律声明，请参
Documentation/admin-guide/sysctl/index.rst銆。

------------------------------------------------------------------------------

本文件包`/proc/sys/kernel/` sysctl 文件的说明文档

该目录下的文件可用于调整和监Linux 内核运行过程中的各类杂项与通用
事项。由于其中某些文*确实**可能被用来搞乱你的系统，因此建议在实
进行调整之前，同时阅读文档与源码

目前，这些文件（取决于你的配置）可能会出现在 `/proc/sys/kernel` 中：



## acct


```
    highwater lowwater frequency
```
如果启用BSD 风格进程记账（process accounting），这些值将控制其行为
如果日志所在文件系统的剩余空间低于 `lowwater`\ %，记账将暂停；若剩余空间
回升`highwater`\ % 以上，则记账恢复。`frequency` 决定了我们检查剩余空
的频率（单位为秒）。默认值：

```
    4 2 30
```
也就是说，当剩余空间低于 2% 时暂停记账；当其增加到至4% 时恢复记账；
认为剩余空间信息30 秒内有效

## acpi_video_flags


参见 Documentation/power/video.rst。它允许以与 `acpi_sleep` 内核参数
类似的方式，通过组合下列取值来设置视频恢复（resume）模式：

= =======
1 s3_bios
2 s3_mode
4 s3_beep
= =======

## arch


机器硬件名称，与 `uname -m` 的输出相
（例`x86_64` `aarch64`）

## auto_msgmni


该变量已无作用，并可能在未来的内核版本中被移除。读取它始终返回 0
Linux 3.17 之前，它在内存添移除IPC 命名空间创建/删除时，
启用/禁用 `msgmni`_ 的自动重算
向该文件写入 "1" 会启msgmni 的自动重算
写入 "0" 则将其关闭。默认值为 1

## bootloader_type (x86 only)


给出bootloader 指示bootloader 类型号，左移 4 位，再与 bootloader
版本号的4 位做 OR 运算。采用这种编码方式的原因是它曾经与内核头中的
`type_of_loader` 字段相匹配；为了保持向后兼容而保留了该编码方式
也就是说，如果完整的 bootloader 类型号为 0x15，完整版本号0x234
则该文件将包含数340 = 0x154

有关更多信息，请参见 Documentation/arch/x86/boot.rst 中的 `type_of_loader`
`ext_loader_type` 字段

## bootloader_version (x86 only)


完整bootloader 版本号。在上面的例子中，该文件将包含数564 = 0x234

有关更多信息，请参见 Documentation/arch/x86/boot.rst 中的 `type_of_loader`
`ext_loader_ver` 字段

## bpf_stats_enabled


控制内核是否应收BPF 程序的统计信
（运行所花费的总时间、运行次数……）。启用统计信息会导致每次程序运行
时性能略有下降。可以使`bpftool` 查看这些统计信息

= ===================================
0 不收集统计信息（默认）
1 收集统计信息
= ===================================

## cad_pid


这是将在重启时（尤其是通过 Ctrl-Alt-Delete）被发送信号的 pid
向该文件写入一个与正在运行的进程不对应的数值将导致 `-ESRCH`

另请参见 `ctrl-alt-del`_

## cap_last_cap


运行中内核的最高有效能力（capability）。从内核中导
`CAP_LAST_CAP`銆。

## core_pattern


`core_pattern` 用于指定核心转储（core dump）文件名模式

- 最大长度为 127 个字符；默认值为 "core"
- `core_pattern` 用作输出文件名模式的模板；某些字符串模式
  （以 '%' 开头）会被替换为它们的实际取值
- `core_uses_pid` 的向后兼容：

	如果 `core_pattern` 不包"%p"（默认不包含
	且设置了 `core_uses_pid`，则会在文件名后附加 .PID

- corename 格式说明

	========	==========================================
	%<NUL>		'%' 被丢
	%%		输出一'%'
	%p		pid
	%P		全局 pid（init PID 命名空间
	%i		tid
	%I		全局 tid（init PID 命名空间
	%u		uid（在初始用户命名空间中）
	%g		gid（在初始用户命名空间中）
	%d		转储模式，匹`PR_SET_DUMPABLE` 
			`/proc/sys/fs/suid_dumpable`
	%s		信号编号
	%t		转储时的 UNIX 时间
	%h		主机
	%e		可执行文件名（可能被截断，也可能prctl 等改变）
	%f      	可执行文件名
	%E		可执行文件路
	%c		受资源限RLIMIT_CORE 约束的核心文件最大大
	%C		任务运行CPU
	%F		pidfd 编号
	%<OTHER>	两者均被丢
	========	==========================================

- 如果模式的第一个字符是 '|'，内核将把模式的其余部分当作要运行的命令
  核心转储将被写入该程序的标准输入，而不是写入文件

## core_pipe_limit


sysctl 仅适用于将 `core_pattern`_ 配置为通过管道把核心文
送给用户空间辅助程序时（`core_pattern` 的第一个字符为 '|'，见上文）
当通过管道收集核心转储到某个应用程序时，收集应用程序有时需要从
`/proc/pid` 目录获取有关崩溃进程的数据
为了安全地做到这一点，内核必须等待收集进程退出，以免过早移除
崩溃进程proc 文件
这反过来又带来了一种可能：一个行为不端的用户空间收集进程可以简单地
通过永不退出，来阻塞对已崩溃进程的回收（reaping）
sysctl 正是防御这种情况
它定义了可以并行通过管道送往用户空间应用程序的并发改崩溃进程数量
如果超过该值，则超出该值的那些崩溃进程会被记录到内核日志中
其核心转储被跳过
0 是一个特殊值，表示可以并行捕获无限多的进程，但不会发生等待
（即不保证收集进程能够访``/proc/<crashing pid>/``）
该值默认为 0

## core_sort_vma


默认的核心转储按地址顺序写入 VMA。将 `core_sort_vma` 设为 1 后，
VMA 将按从最小到最大的大小顺序写入。已知这至少会让 elfutils 出问题，
但在处理非常庞大（且被截断）的核心转储时可能很方便，因为其中更有用的
调试细节包含在较小的 VMA 中

## core_uses_pid


默认的核心转储文件名"core"。将 `core_uses_pid` 设为 1 后，
核心转储文件名变core.PID
如果 `core_pattern`_ 不包"%p"（默认不包含）且设置`core_uses_pid`
则会在文件名后附.PID

## ctrl-alt-del


当本文件中的值为 0 时，ctrl-alt-del 被捕获并发送给 `init(1)` 程序
以进行优雅的重启处理
然而，当> 0 时，Linux Vulcan Nerve Pinch（tm）的反应将是
立即重启，甚至不会同步其脏缓冲区（dirty buffers）

注意
  当某个程序（dosemu）将键盘置于 'raw' 模式时，ctrl-alt-del 
  在到达内tty 层之前被该程序拦截，由该程序决定如何处理它

## dmesg_restrict


该开关指示是否阻止非特权用户使用 `dmesg(8)` 查看内核日志缓冲区中
消息
`dmesg_restrict` 设为 0 时没有任何限制
`dmesg_restrict` 设为 1 时，用户必须具有 `CAP_SYSLOG` 才能使用
`dmesg(8)`銆。

内核配置选项 `CONFIG_SECURITY_DMESG_RESTRICT` 设置 `dmesg_restrict`
的默认值

## domainname & hostname


这些文件可用于设NIS/YP 域名（domainname）和你的机器的主机名
（hostname），其方式与下列命令完全相同
```
	# echo "darkstar" > /proc/sys/kernel/hostname
	# echo "mydomain" > /proc/sys/kernel/domainname
```
```
	# hostname "darkstar"
	# domainname "mydomain"
```
但请注意，经典的 darkstar.frop.org 主机名为 "darkstar"，其 DNS
（Internet 域名服务器）域名"frop.org"，不要与 NIS（网络信息服务）
YP（Yellow Pages）域名混淆。这两类域名通常是不同的。详细讨
请参`hostname(1)` 手册页

## firmware_config


参见 Documentation/driver-api/firmware/fallback-mechanisms.rst

该目录中的条目可用于控制固件加载器辅助程序的回退（fallback）：

- `force_sysfs_fallback`，设1 时强制使用回退
- `ignore_sysfs_fallback`，设1 时忽略任何回退

## ftrace_dump_on_oops


决定是否应在 oops（或内核 panic）时调用 `ftrace_dump()`。这会把
ftrace 缓冲区的内容输出到控制台。这对于捕获导致崩溃的跟踪并将其
输出到串行控制台非常有用

======================= ===========================================
0                       Disabled (default).
1                       Dump buffers of all CPUs.
2(orig_cpu)             Dump the buffer of the CPU that triggered the
                        oops.
<instance>              Dump the specific instance buffer on all CPUs.
<instance>=2(orig_cpu)  Dump the specific instance buffer on the CPU
                        that triggered the oops.
======================= ===========================================

同时也支持多实例转储，实例之间用逗号分隔。如果还需要转储全局缓冲区，
请先为全局缓冲区指定转储模式（1/2/orig_cpu）

因此，例如要转储所CPU 上的 "foo" "bar" 实例缓冲区：
```
  echo "foo,bar" > /proc/sys/kernel/ftrace_dump_on_oops
```
要转储所CPU 上的全局缓冲区与 "foo" 实例缓冲区，以及触发 oops 
CPU 上的 "bar" 实例缓冲区：
```
  echo "1,foo,bar=2" > /proc/sys/kernel/ftrace_dump_on_oops
```

## ftrace_enabled, stack_tracer_enabled


参见 Documentation/trace/ftrace.rst

## hardlockup_all_cpu_backtrace


该值控制在检测到硬锁死（hard lockup）条件时，硬锁死检测器是否收集进一
的调试信息。如果启用，将启动特定于体系结构的全 CPU 栈转储

= ============================================
0 不执行任何操作。这是默认行为
1 在检测到时捕获更多调试信息
= ============================================

## hardlockup_panic


该参数可用于控制在检测到硬锁死时内核是否触发 panic

= ===========================
0 检测到硬锁死时不触panic
1 检测到硬锁死时触发 panic
= ===========================

有关更多信息请参Documentation/admin-guide/lockup-watchdogs.rst
也可以使nmi_watchdog 内核参数来设置

## hotplug


热插拔（hotplug）策略代理（agent）的路径
默认值为 `CONFIG_UEVENT_HELPER_PATH`，其默认值为空字符串

只有在启`CONFIG_UEVENT_HELPER` 时该文件才存在。大多数现代系统完全
依赖基于 netlink uevent 源，不需要它

## hung_task_all_cpu_backtrace


如果设置了该选项，当检测到挂起任务（hung task）时，内核将向所CPU
发NMI 以转储它们的回溯信息。该文件在启CONFIG_DETECT_HUNG_TASK
CONFIG_SMP 时会出现

0: 检测到挂起任务时不显示所CPU 的回溯
这是默认行为

1: 检测到挂起任务时会以不可屏蔽中断（NMI）打断所CPU 并转储它们的
回溯信息

## hung_task_panic


当被设为非零值时，如果在单次扫描中发现的挂起任务数量达到该值，将触
内核 panic。该文件在启`CONFIG_DETECT_HUNG_TASK` 时出现

= =======================================================
0 继续运行。这是默认行为
N 当单次扫描中发现 N 个挂起任务时触发 panic
= =======================================================

## hung_task_check_count


被检查任务数量的上限
该文件在启用 `CONFIG_DETECT_HUNG_TASK` 时出现

## hung_task_detect_count


表示自系统启动以来或自计数器被重置以来，被检测为挂起的任务总数。当写入
数0 时计数器被清零

该文件在启用 `CONFIG_DETECT_HUNG_TASK` 时出现

## hung_task_sys_info

以逗号分隔的、在检测到挂起任务时要转储的额外系统信息列表，例如
"tasks,mem,timers,locks,..."。更多细节请参阅下文'panic_sys_info' 小节

## hung_task_timeout_secs


当处D 状态的任务超过该值未被调度时，将报告一个警告
该文件在启用 `CONFIG_DETECT_HUNG_TASK` 时出现

0 表示无限超时，不进行任何检查

可设置的取值范围在 {0:`LONG_MAX`/`HZ`} 之间

## hung_task_check_interval_secs


挂起任务检查间隔。如果启用了挂起任务检查（参见 `hung_task_timeout_secs`_），
则每`hung_task_check_interval_secs` 秒检查一次
该文件在启用 `CONFIG_DETECT_HUNG_TASK` 时出现

0（默认）表示使用 `hung_task_timeout_secs` 作为检查间隔

可设置的取值范围在 {0:`LONG_MAX`/`HZ`} 之间

## hung_task_warnings


要报告的最大警告数量。在检查间隔内如果检测到挂起任务，该值减 1
当该值降0 时，将不再报告任何警告
该文件在启用 `CONFIG_DETECT_HUNG_TASK` 时出现

-1: 报告无限数量的警告

## hyperv_record_panic_msg


控制是否panic kmsg 数据上报Hyper-V

= =========================================================
0 不上panic kmsg 数据
1 上报 panic kmsg 数据。这是默认行为
= =========================================================

## ignore-unaligned-usertrap


在那些未对齐访问会触发陷阱（trap）、且支持该特
（`CONFIG_SYSCTL_ARCH_UNALIGN_NO_WARN`；目前为 `arc`、`parisc` 
`loongarch`）的体系结构上，控制是否记录所有未对齐陷阱

= =============================================================
0 记录所有未对齐访问
1 仅第一次进程触发陷阱时发出警告。这是默认设置
= =============================================================

另请参见 `unaligned-trap`_

## io_uring_disabled


禁止所有进程创建新io_uring 实例。启用它可以缩小内核的攻击面

= ======================================================================
0 所有进程都可以像平常一样创io_uring 实例。这是默认设置
1 对于不在 io_uring_group 组中的非特权进程，禁止创io_uring
  （io_uring_setup() 将失败并返回 -EPERM）。已有的 io_uring 实例
  仍可使用。有关更多信息请参阅 io_uring_group 的文档
2 禁止所有进程创io_uring。io_uring_setup() 总是失败并返-EPERM
  已有io_uring 实例仍可使用
= ======================================================================

## io_uring_group


io_uring_disabled 设为 1 时，进程必须具有特权（CAP_SYS_ADMIN）或
属于 io_uring_group 组，才能创建 io_uring 实例。如io_uring_group
设为 -1（默认），则只有具有 CAP_SYS_ADMIN 能力的进程可以创io_uring
实例

## kernel_sys_info

以逗号分隔的、在检测到硬锁死时要转储的额外系统信息列表，例
"tasks,mem,timers,locks,..."。更多细节请参阅下文'panic_sys_info' 小节

它作为默认的内核控制旋钮，当某个内核模块parameter==0 调用 sys_info()
时生效

## kexec_load_disabled


一个指示系统调`kexec_load` `kexec_file_load` 是否已被禁用的开关
该值默认为 0（false：`kexec_*load` 已启用），但可以设为 1
（true：`kexec_*load` 已禁用）
一旦为 true，就再也无法使用 kexec，且该开关无法被设回 false
这允许在禁用syscall 之前加载一kexec 镜像，使系统能够建立一个镜
（并在之后使用），而不会被改动
通常`modules_disabled`_ sysctl 一起使用

## kexec_load_limit_panic


该参数指定系统调`kexec_load` `kexec_file_load` 可以使用崩溃镜像
被调用的次数上限。它只能被设为比当前值更严格的数值

== ======================================================
-1 kexec 的调用次数不限。这是默认设置
N  剩余的调用次数
== ======================================================

## kexec_load_limit_reboot


`kexec_load_limit_panic` 功能类似，但针对的是正常镜像

## kptr_restrict


该开关指示是否对通过 `/proc` 及其他接口暴露内核地址施加限制

`kptr_restrict` 设为 0（默认）时，地址在打印前会被哈希化
（这等价%p。）

`kptr_restrict` 设为 1 时，使用 %pK 格式说明符打印的内核指针将被
替换0，除非用户具`CAP_SYSLOG` 且有效的用户和组 id 等于真实
id銆。
这是因为 %pK 的检查是read() 时而非 open() 时进行的，因此如果在
open() read() 之间提升了权限（例如通过 setuid 二进制文件），那
%pK 就不会将内核指针泄漏给非特权用户

注意，这只是一个临时解决方案
正确的长期解决方案是open() 时进行权限检查
如果担心将内核指针值泄漏给非特权用户，请考虑从使%pK 的文件中移除
其他用户的读权限，并使用 `dmesg_restrict`_ 来防范在 `dmesg(8)` 
使用 %pK

`kptr_restrict` 设为 2 时，使用 %pK 打印的内核指针将被替换为 0
而与特权无关

若要在启动早期（一次性）禁用这些安全限制，请改用 `hash_pointers`
启动参数

## softlockup_sys_info & hardlockup_sys_info

以逗号分隔的、在检测到硬锁死时要转储的额外系统信息列表，例
"tasks,mem,timers,locks,..."。更多细节请参阅下文'panic_sys_info' 小节

## modprobe


用于自动加载内核模块的用户态辅助程序的完整路径，默认是
`CONFIG_MODPROBE_PATH`，其默认值为 "/sbin/modprobe"。当内核请求某个
模块时就会执行该二进制文件。例如，如果用户空间mount() 传递了未知的文
系统类型，内核就会通过执行该用户态辅助程序来自动请求相应的文件系统模块
该用户态辅助程序应将所需的模块插入内核

sysctl 只影响模块的自动加载。它对显式插入模块的能力没有影响

```
    echo '#! /bin/sh' > /tmp/modprobe
    echo 'echo "$@" >> /tmp/modprobe.log' >> /tmp/modprobe
    echo 'exec /sbin/modprobe "$@"' >> /tmp/modprobe
    chmod a+x /tmp/modprobe
    echo /tmp/modprobe > /proc/sys/kernel/modprobe
```

此外，如果本 sysctl 被设为空字符串，则模块自动加载被完全禁用。内核将
根本不会尝试执行用户态辅助程序，也不会调kernel_module_request LSM
钩子

如果在内核配置中设置CONFIG_STATIC_USERMODEHELPER=y，则配置好的静
用户态辅助程序会覆盖sysctl，但上述空字符串仍可被接受以完全禁用模块
自动加载

## modules_disabled


一个指示在本身是模块化的内核中是否允许加载模块的开关。该开关默认为
关闭），但可以设true）。一旦为 true，模块既不能被加载也不能
被卸载，且该开关无法被设回 false。通常`kexec_load_disabled`_ 开
一起使用

## msgmax, msgmnb, and msgmni


`msgmax` IPC 消息的最大大小，以字节为单位。默8192（`MSGMAX`）

`msgmnb` IPC 队列的最大大小，以字节为单位。默16384（`MSGMNB`）

`msgmni` IPC 队列的最大数量。默32000（`MSGMNI`）

所有这些参数都是按 ipc 命名空间设置的。POSIX 消息队列中的最大字节数
`RLIMIT_MSGQUEUE` 限制。该限制在每个用户命名空间中被层级式地遵守

## msg_next_id, sem_next_id, and shm_next_id (System V IPC)


这三个开关允许分别指定下次分配的 IPC 对象（消息、信号量或共享内存）
的期id

默认它们都等-1，表示采用通用分配逻辑
可设置的取值范围在 {0:`INT_MAX`} 之间

注意
  1) 内核不保证新对象会具有期望的 id。因此，如何处理具有“错误id 
     对象，取决于用户空间
  2) 在成功分IPC 对象后，内核会把具有非默认值的开关重置回 -1。如
     IPC 对象分配系统调用失败，该值是保持不被修改还是被重置为 -1 是未定义
     的

## ngroups_max


补充组（supplementary groups）的最大数量，_即_ `setgroups` 所能接受的最
大小。从内核中导`NGROUPS_MAX`

## nmi_watchdog


该参数可用于控制 x86 系统上的 NMI 看门狗（即硬锁死检测器）

= =================================
0 禁用硬锁死检测器
1 启用硬锁死检测器
= =================================

硬锁死检测器监视每个 CPU 是否能响应定时器中断。该机制利用 CPU 性能计数
寄存器，CPU 繁忙时被编程为周期性地产生非屏蔽中断（NMI）。因此有
"NMI 看门这个别名

如果内核作为客户机（guest）运行，NMI 看门狗默认是禁用的。可以向上文中的
客户机内核命令行添加
```
   nmi_watchdog=1
```
来启用（参见 Documentation/admin-guide/kernel-parameters.rst）

## nmi_wd_lpm_factor (PPC only)


应用NMI 看门狗超时的系数（仅`nmi_watchdog` 设为 1 时）。该系数表示
LPM 期间计算 NMI 看门狗超时时间时，加`watchdog_thresh` 上的百分比。软锁死
超时不受影响

值为 0 表示不改变。默认值为 200，意味着 NMI 看门狗被设为 30s（基
`watchdog_thresh` 等于 10）

## numa_balancing


启用/禁用并配置基于自动缺页（page fault）的 NUMA 内存平衡。内存会被自
迁移到经常访问它的节点。要设置的值可以是下列取值的 OR 组合

= =================================
0 NUMA_BALANCING_DISABLED
1 NUMA_BALANCING_NORMAL
2 NUMA_BALANCING_MEMORY_TIERING
= =================================

或NUMA_BALANCING_NORMAL，用于优化不NUMA 节点之间的页面放置以减少
远程访问。在 NUMA 机器上，如果内存CPU 远程访问会有性能惩罚。启用该特性后
内核会周期性地取消页面映射，随后捕获缺页，以此对正在访问内存的任务线程进行
采样。在缺页发生时，会判断正在访问的数据是否应迁移到本地内存节点

取消页面映射并捕获缺页会带来额外的开销，理想情况下由改善的内存局部性来抵消
但这并没有普遍的保证。如果目标工作负载已经绑定到 NUMA 节点，则应禁用该特性

或NUMA_BALANCING_MEMORY_TIERING，用于优化不同类型内存（表示为不同的
NUMA 节点）之间的页面放置，将热页（hot pages）放到快速内存中。这也基于取消映
与缺页来实现

## numa_balancing_promote_rate_limit_MBps


不同类型内存之间过高的提升（promotion降级（demotion）吞吐量可能会损
应用程序延迟。这可用于对提升吞吐量进行限速。每个节点的最大提升吞吐量（MB/s
将被限制为不超过所设置的值

经验法则是将其设为小PMEM 节点写带宽的 1/10

## oops_all_cpu_backtrace


如果设置了该选项，当发生 oops 事件时，内核将向所CPU 发NMI 以转储它们的
回溯信息。它应作为最后的手段使用，例如在无法触发 panic（例如为了保护运行中
VM）或无法收集 kdump 时。该文件在启CONFIG_SMP 时出现

0: 检测到 oops 时不显示所CPU 的回溯
这是默认行为

1: 检测到 oops 事件时会以不可屏蔽中断（NMI）打断所CPU 并转储它们的回溯信息

## oops_limit


`panic_on_oops` 未设置的情况下，内核发生多少oops 之后应当触发 panic
将其设为 0 将禁用计数检查。将其设1 与设`panic_on_oops=1` 效果相同
默认值为 10000

## osrelease, ostype & version


```
  # cat osrelease
  2.1.88
  # cat ostype
  Linux
  # cat version
  #5 Wed Feb 25 21:49:24 MET 1998
```
`osrelease` `ostype` 这两个文件含义应该足够清楚了。`version` 则需
多一点说明#5" 表示这是从该源码基线构建的第五个内核，其后的日期表示内核
构建的时间。调整这些数值的唯一方法是重新构建内:-)

## overflowgid & overflowuid


如果你的体系结构并非始终支持 32 UID（即 arm、i386、m68k、sh sparc32），
那么对于使用旧的 16 UID/GID 系统调用的应用程序，如果实际UID GID
超过 65535，将返回一个固定的 UID GID

这些 sysctl 允许你更改该固定 UID GID 的值。默认值为 65534

## panic


本文件中的值决定了内核panic 时的行为

- 如果为零，内核将永远循环
- 如果为负，内核将立即重启
- 如果为正，内核将在相应的秒数之后重启

当你使用软件看门狗时，推荐设置为 60

## panic_on_io_nmi


控制当一CPU 收到IO 错误引起NMI 时内核的行为

= ==================================================================
0 尝试继续运行（默认）
1 立即 panic。IO 错误触发NMI。这表示一种可能导IO 数据损坏的严
  系统状况。与其继续运行，panic 可能是更好的选择。某些服务器在按
  dump 按钮时会发出此类 NMI，你可以利用该选项来获取崩溃转储
= ==================================================================

## panic_on_oops


控制当遇oops BUG 时内核的行为

= ===================================================================
0 尝试继续运行
1 立即 panic。如`panic` sysctl 也非零，则机器将被重启
= ===================================================================

## panic_on_stackoverflow


控制内核在检测到内核栈、IRQ 栈和异常栈（用户栈除外）溢出时的行为。该文件
启用 `CONFIG_DEBUG_STACKOVERFLOW` 时出现

= ==========================
0 尝试继续运行
1 立即 panic
= ==========================

## panic_on_unrecovered_nmi


Linux 对由内存或未知原因引起的 NMI 的默认行为是继续运行。对于许多环境（例如
科学计算）而言，将机器停机并处理错误，比让未纠正的奇偶/ECC 错误传播开来更好

少数系统确实会因一些奇怪的随机原因（例如电源管理）而产NMI，因此默认是关闭
的。该 sysctl 的工作方式与该目录中已有panic 控制开关相同

## panic_on_warn


当设1 时，WARN() 路径中调panic()。这对于在尝试在 WARN() 位置进行
kdump 时避免重新构建内核很有用

= ================================================
0 只调WARN()，这是默认行为
1 在打印出 WARN() 位置后调panic()
= ================================================

## panic_print


panic 发生时用于打印系统信息的位掩码。用户可以选择下列位的组合

=====  ============================================
bit 0  print all tasks info
bit 1  print system memory info
bit 2  print timer info
bit 3  print locks info if `CONFIG_LOCKDEP` is on
bit 4  print ftrace buffer
bit 5  replay all kernel messages on consoles at the end of panic
bit 6  print all CPUs backtrace (if available in the arch)
bit 7  print only tasks in uninterruptible (blocked) state
=====  ============================================

```
  echo 3 > /proc/sys/kernel/panic_print
```

## panic_sys_info


以逗号分隔的、在 panic 时要转储的额外信息列表，例如 "tasks,mem,timers,..."
它是 'panic_print' 的人类可读替代方案。可选值包括：

=============   ===================================================
tasks           print all tasks info
mem             print system memory info
timers          print timers info
locks           print locks info if CONFIG_LOCKDEP is on
ftrace          print ftrace buffer
all_bt          print all CPUs backtrace (if available in the arch)
blocked_tasks   print only tasks in uninterruptible (blocked) state
=============   ===================================================

## panic_on_rcu_stall


当设1 时，在打RCU stall 检测消息后调用 panic()。这对于使用 vmcore 
确定 RCU stall 的根本原因很有用

= ============================================================
0 发生 RCU stall 时不调用 panic()，这是默认行为
1 在打RCU stall 消息后调panic()
= ============================================================

## max_rcu_stall_to_panic


`panic_on_rcu_stall` 设为 1 时，该值决定了在调panic() 之前 RCU 可以
stall 的次数

`panic_on_rcu_stall` 设为 0 时，该值不起作用

## perf_cpu_time_max_percent


向内核提示它处理 perf 采样事件时应该被允许使用CPU 时间比例。如perf
子系统被告知其采样超过了该限制，它将降低采样频率以尝试减少其 CPU 使用量

一perf 采样发生NMI 中。如果这些采样意外地执行时间过长，NMI 可能
彼此堆叠在一起，以至于其他任何东西都无法执行

===== ========================================================
0     禁用该机制。无论占用多CPU 时间，都不监视或纠正 perf 
      采样频率

1-100 尝试perf 的采样频率限制为CPU 百分比。注意：内核会计
      每个采样事件的“预期”长度。这里的 100 表示该预期长度的 100%
      即使设为 100，如果超过了该长度，仍可能看到采样被节流。如果你
      确实不在乎消耗多CPU，请设为 0
===== ========================================================

## perf_event_paranoid


控制非特权用户（没有 CAP_PERFMON）对性能事件系统的使用。默认值为 2

出于向后兼容的原因，对系统性能监视和可观测性的访问对具CAP_SYS_ADMIN
特权的进程仍然开放，但相对于 CAP_PERFMON 的使用场景，不鼓励将 CAP_SYS_ADMIN
用于安全的系统性能监视与可观测性操作

===  ==================================================================
 -1  允许所有用户使用（几乎）所有事件

     在未持有 `CAP_IPC_LOCK` 的情况下，忽perf_event_mlock_kb 之后
     mlock 限制

>=0  禁止`CAP_PERFMON` 的用户使ftrace 函数跟踪点

     禁止`CAP_PERFMON` 的用户访问原始跟踪点

>=1  禁止`CAP_PERFMON` 的用户访CPU 事件

===  ==================================================================

## perf_event_max_stack


控制在为配置了（``attr.sample_type & PERF_SAMPLE_CALLCHAIN``）的事件复制
栈帧时的最大数量，例如在使'`perf record -g`' 
'`perf trace --call-graph fp`' 时

这只能在没有任何启用了调用链（callchains）的事件在使用时完成，否则写
该文件将返回 `-EBUSY`

默认值为 127

## perf_event_mlock_kb


控制不计mlock 限制的每 CPU 环形缓冲区大小

默认值为 512 + 1 

## perf_event_max_contexts_per_stack


控制在为配置了（`attr.sample_type & PERF_SAMPLE_CALLCHAIN`）的事件复制
栈帧上下文条目时的最大数量，例如在使'`perf record -g`' 
'`perf trace --call-graph fp`' 时

这只能在没有任何启用了调用链的事件在使用时完成，否则写入该文件将返回
`-EBUSY`銆。

默认值为 8

## perf_user_access (arm64 and riscv only)


控制用户空间读取 perf 事件计数器的访问权限

- 对于 arm64
  默认值为 0（禁用访问）

  当设1 时，用户空间可以直接读取性能监视计数器寄存器

  更多信息请参Documentation/arch/arm64/perf.rst

- 对于 riscv
  当设0 时，禁用用户空间访问

  默认值为 1，用户空间可以通过 perf 读取性能监视计数器寄存器，任何没
  perf 介入的直接访问都会触发非法指令

  当设2 时，启用传统模式（用户空间仅能直接访cycle instret CSRs）
  注意该传统值已被弃用，将在所有用户空间应用程序修复后被移除

  注意 time CSR 始终对所有模式直接可访问

## pid_max


PID 分配的回绕值。当内核的下一PID 值达到该值时，会回绕到最PID 值
值为 `pid_max` 或更大的 PID 不会被分配

## ns_last_pid


当前（使用该 sysctl 的任务所在的）pid 命名空间中最后分配的 pid。当
fork 时为下一个任务选择 pid 时，内核会尝试从该值开始分配编号

## powersave-nap (PPC only)


如果设置，Linux-PPC 将使powersaving 'nap' 模式，否则使'doze'
模式

==============================================================

## printk


printk 中的四个值分别表示：`console_loglevel`
`default_message_loglevel`、`minimum_console_loglevel` 
`default_console_loglevel`銆。

这些数值影printk() 在打印或记录错误消息时的行为。有关不同日志级
的更多信息，请参'`man 2 syslog`'

======================== =====================================
console_loglevel         优先级高于此值的消息将被打印到控制台
default_message_loglevel 没有显式优先级的消息将以该优先级打印
minimum_console_loglevel console_loglevel 可被设置的最小（最高）
default_console_loglevel console_loglevel 的默认
======================== =====================================

## printk_delay


延迟每条 printk 消息 `printk_delay` 毫秒

允许的值范围为 0 - 10000

## printk_ratelimit


某些警告消息会受到速率限制。`printk_ratelimit` 指定了这些消息之间的最
时间间隔（以秒为单位）。默认值为 5 秒

值为 0 将禁用速率限制

## printk_ratelimit_burst


从长远来看，我们强制`printk_ratelimit`_ 秒最多一条消息，但也确实允许
突发的消息通过。`printk_ratelimit_burst` 指定了在速率限制生效之前可以
发送的消息数量。在 `printk_ratelimit`_ 秒过去之后，可以再发送一波突
消息

默认值为 10 条消息

## printk_devkmsg


控制来自用户空间`/dev/kmsg` 的日志记录：

========= =============================================
ratelimit default, ratelimited
on        unlimited logging to /dev/kmsg from userspace
off       logging to /dev/kmsg disabled
========= =============================================

内核命令行参`printk.devkmsg=` 会覆盖本设置，并且是一次性的设置，直
下次重启：一旦设置，就无法再通过sysctl 接口更改

==============================================================

## pty


参见 Documentation/filesystems/devpts.rst

## random


这是一个目录，包含以下条目

- `boot_id`：第一次被检索时生成UUID，此后保持不变；

- `uuid`：每次被检索时生成UUID（因此可用于按需生成 UUID）；

- `entropy_avail`：池中熵的计数，以比特为单位

- `poolsize`：熵池大小，以比特为单位

- `urandom_min_reseed_secs`：已废弃（曾用于确定 urandom 池重新播种之间的
  最小秒数）。该文件可写是为了兼容，但写入它对任RNG 行为都没有影响；

- `write_wakeup_threshold`：当熵计数（以比特数计）低于此值时，等待写
  `/dev/random` 的进程会被唤醒。该文件可写是为了兼容，但写入它对任
  RNG 行为都没有影响

## randomize_va_space


对于支持该特性的体系结构，该选项可用于选择系统中所使用的进程地址空间
随机化（address space randomization）的类型

==  ===========================================================================
0   关闭进程地址空间随机化。对于本来就支持该特性的体系结构，以及以
    "norandmaps" 参数启动的内核而言，这是默认值

1   mmap 基址、栈VDSO 页面的地址随机化。这（除其他外）意味着共享
    库将被加载到随机地址。对PIE 链接的二进制文件，代码起始位置也
    随机化。如果启用了 `CONFIG_COMPAT_BRK` 选项，这是默认值

2   另外启用堆随机化。如果禁用了 `CONFIG_COMPAT_BRK`，这是默认值

    有一些遗留应用程序（例如 1996 年某些古老的 libc.so.5 版本）假
    brk 区域正好从代bss 的末尾之后开始。当 brk 区域的起始位置被随机化时
    这些应用程序就会出问题。然而，目前没有已知的会因这种方式而出问题
    非遗留应用程序，因此对大多数系统而言，选择完全随机化是安全的

    带有古老和/或有问题二进制文件的系统应配置为启用 `CONFIG_COMPAT_BRK`
    从而将堆排除在进程地址空间随机化之外
==  ===========================================================================

## reboot-cmd (SPARC only)


 这似乎是Sparc ROM/Flash 引导加载器传递参数的一种方式。也许是
告诉它在重启后做什么

## sched_energy_aware


启用/禁用能量感知调度（Energy Aware Scheduling，EAS）。EAS 会在能够运行它的
平台上自动启动（即具有非对称 CPU 拓扑并且存在可用能量模型的平台）。如果你
平台恰好满足 EAS 的要求但你不想使用它，请将该值改0。在EAS 平台上，
写操作会失败，读操作也不会返回任何内容

## task_delayacct


启用/禁用任务延迟统计（task delay accounting，参
Documentation/accounting/delay-accounting.rst）。启用该特性会在调度器中产
少量开销，但对调试和性能调优很有用。iotop 等一些工具需要它

## sched_schedstats


启用/禁用调度器统计信息。启用该特性会在调度器中产生少量开销，但对调试和性能
调优很有用

## sched_util_clamp_min


允许的最*最*利用率（utilization）

默认值为 1024，即可能的最大值

这意味着任何请求uclamp.min 值都不能大于 sched_util_clamp_min，即它被
限制[0:sched_util_clamp_min] 范围内

## sched_util_clamp_max


允许的最*最*利用率

默认值为 1024，即可能的最大值

这意味着任何请求uclamp.max 值都不能大于 sched_util_clamp_max，即它被
限制[0:sched_util_clamp_max] 范围内

## sched_util_clamp_min_rt_default


默认情况下，Linux 是为性能而调优的。这意味着 RT 任务总是以最高的频率以及
能力最强（最高容量）CPU（在异构系统中）运行

Uclamp 通过默认将所RT 任务的请uclamp.min 设为 1024 来实现这一点，
这实际上会将任务提升到以最高频率运行，并使它们偏向于在最大的 CPU 上运行

该旋钮允许管理员在使uclamp 时更改默认行为。特别是在电池供电的设备上，
以最大能力和频率运行会增加能耗并缩短电池寿命

该旋钮只对那些用户尚未通过 sched_setattr() 系统调用修改其请uclamp.min
值的 RT 任务有效

该旋钮不会突破上面定义的 sched_util_clamp_min 所施加的范围约束

例如如果

	sched_util_clamp_min_rt_default = 800
	sched_util_clamp_min = 600

那么提升将被限制600，因800 超出[0:600] 的允许范围。例如，如果某种
省电模式通过修改 sched_util_clamp_min 临时限制所有提升，就可能发生这种情况
一旦解除该限制，请求的 sched_util_clamp_min_rt_default 就会生效

## seccomp


参见 Documentation/userspace-api/seccomp_filter.rst

## sg-big-buff


本文件显示通用 SCSI（sg）缓冲区的大小。你还不能对它进行调优，但可以在
编译时通过编辑 `include/scsi/sg.h` 并修`SG_BIG_BUFF` 的值来更改它

应该没有任何理由去更改这个值。如果你能想出一个理由，那你可能本来就知
自己在做什:)

## shmall


该参数设ipc 命名空间内可以使用的共享内存页总量。共享内存页的计数是针对
每个 ipc 命名空间分别进行的，且不被继承。因此，`shmall` 至少应为
`ceil(shmmax/PAGE_SIZE)`銆。

```
	# getconf PAGE_SIZE
```
要减少或禁用分配共享内存的能力，你必须创建一个新ipc 命名空间，将此参
设为所需的值，并禁止在当前用户命名空间中创建新ipc 命名空间，或者可以使
cgroups銆。

## shmmax


该值可用于查询和设置运行时可创建的最大共享内存段大小的_limit_
内核现在支持最1Gb 的共享内存段。该值默认为 `SHMMAX`

## shmmni


该值决定了共享内存段的最大数量。默4096（`SHMMNI`）

## shm_rmid_forced


Linux 允许你通过 `setrlimit(2)` 设置资源限制，包括一个进程可以消耗多少内存
不幸的是，共享内存段被允许在没有任何进程关联的情况下存在，因此可能不被计
任何资源限制。如果启用，当共享内存段在分离（detach）或进程终止后其附加计数
变为零时，会被自动销毁。它还会在进程退出时销毁那些已创建但从未被附加的段
`IPC_RMID` 剩下的唯一用途就是立即销毁一个未附加的段。当然，这打破了事物
定义方式，因此某些应用程序可能会停止工作。注意，除非你还配置了资源限
（特别是 `RLIMIT_AS` `RLIMIT_NPROC`），否则该特性对你毫无用处。大多数
系统不需要它

注意，如果你将该值从 0 改为 1，那些已创建但没有用户、且原始进程已死亡的
将被销毁

## sysctl_writes_strict


控制文件位置如何影响通过 `/proc/sys` 接口更新 sysctl 值的行为

  ==   ======================================================================
  -1   传统的按写入处理 sysctl 值的方式，没printk 警告。每write 系统调用
       必须完整包含要写入的 sysctl 值，并且对同一 sysctl 文件描述符的多次写入
       将重sysctl 值，与文件位置无关
   0   与上述行为相同，但对在文件位置不0 时写sysctl 文件描述符的进程
       发出警告
   1   （默认）写入 sysctl 字符串时遵循文件位置。多次写入将追加sysctl 
       缓冲区。超sysctl 值缓冲区最大长度的部分将被忽略。对数值型 sysctl
       条目的写入必须始终位于文件位0，且值必须完整包含在 write 系统调用
       发送的缓冲区中
  ==   ======================================================================

## softlockup_all_cpu_backtrace


该值控制软锁死检测器线程在检测到软锁死条件时是否收集进一步调试信息的
行为。如果启用，每个 CPU 将被发出 NMI 并被指示捕获栈跟踪

该特性仅适用于支NMI 的体系结构

= ============================================
0 不执行任何操作。这是默认行为
1 在检测到时捕获更多调试信息
= ============================================

## softlockup_panic


该参数可用于控制当检测到软锁死时内核是否触发 panic

= ============================================
0 检测到软锁死时不触panic
1 检测到软锁死时触发 panic
= ============================================

也可以使softlockup_panic 内核参数来设置

## soft_watchdog


该参数可用于控制软锁死检测器

= =================================
0 禁用软锁死检测器
1 启用软锁死检测器
= =================================

软锁死检测器监视那些在没有主动重新调度的情况下长时间霸占 CPU 的线程，从
阻止 'migration/N' 线程运行，导致看门狗工作无法执行。该机制依赖CPU 响应
定时器中断的能力（看门狗工作正是由看门狗定时器函数排队所需），否则如果启用
NMI 看门狗，它就能检测到硬锁死条件

## split_lock_mitigate (x86 only)


x86 上，每次“拆分锁”（split lock）都会带来系统范围性的性能惩罚。在较大
系统上，来自非特权用户的大量拆分锁可能导致对行为良好、且可能更重要的用户
拒绝服务

内核通过检测拆分锁并施加惩罚来缓解这些不良行为：强制它们等待，并且一次只允许
一个核心执行拆分锁

这些缓解措施可能让那些不良应用程序变得极其缓慢。将 split_lock_mitigate=0 可能
会恢复一些应用程序性能，但也会增加系统遭受来自拆分锁用户的拒绝服务攻击的风险

= ===================================================================
0 禁用缓解模式——只在内核日志上警告拆分锁，并使系统面临来自拆分锁者的
  拒绝服务
1 启用缓解模式（这是默认）——通过故意的性能下降来惩罚拆分锁者
= ===================================================================

## stack_erasing


该参数可用于控制在使`CONFIG_KSTACK_ERASE` 构建的内核中，在系统调用结束
擦除内核栈的行为

这种擦除可以减少内核栈泄漏漏洞所能暴露的信息，并阻止某些未初始化栈变量攻击
代价是性能影响：在CPU 系统上内核编译会看到 1% 的减速，其他系统和负载可
有所不同

= ====================================================================
0 禁用内核栈擦除，不更KSTACK_ERASE_METRICS
1 启用内核栈擦除（默认），它在系统调用结束返回用户空间之前执行
= ====================================================================

## stop-a (SPARC only)


控制 Stop-A

= ====================================
0 Stop-A 不起作用
1 Stop-A 中断进入 PROM（默认）
= ====================================

在发panic Stop-A 总是启用的，以便用户可以返回到引PROM

## sysrq


参见 Documentation/admin-guide/sysrq.rst

## tainted


如果内核已被污染（tainted），则为非零值。数值可OR 在一起。字母出现在 Oops
报告"Tainted" 行中

======  =====  ==============================================================
     1  `(P)`  加载了专有（proprietary）模
     2  `(F)`  模块被强制加载（force loaded
     4  `(S)`  内核运行在超出规范（out of specification）的系统
     8  `(R)`  模块被强制卸载（force unloaded
    16  `(M)`  处理器报告了机器检查异常（MCE
    32  `(B)`  引用了坏页或某些意外的页标志
    64  `(U)`  用户空间应用程序请求了污染（taint
   128  `(D)`  内核最近死亡，即曾发生 OOPS BUG
   256  `(A)`  ACPI 表被用户覆盖
   512  `(W)`  内核发出了警
  1024  `(C)`  加载了暂存区（staging）驱
  2048  `(I)`  应用了针对平台固件中缺陷的变通方
  4096  `(O)`  加载了外部构建（"out-of-tree"）的模块
  8192  `(E)`  加载了未签名的模
 16384  `(L)`  发生了软锁死
 32768  `(K)`  内核已被实时补丁（live patched
 65536  `(X)`  辅助污染（Auxiliary taint），由发行版定义并使
131072  `(T)`  内核使用了结构体随机化插件构
======  =====  ==============================================================

有关更多信息，请参阅 Documentation/admin-guide/tainted-kernels.rst

注意
  如果内核以命令行选项 `panic_on_taint=<bitmask>,nousertaint` 启动，且写入
  `tainted` 的任一 OR 值与panic_on_taint 上声明的位掩码匹配，则对sysctl
  接口的写入将`EINVAL` 失败
  有关该特定内核命令行选项及其可选的 `nousertaint` 开关的更多细节，请参阅
  Documentation/admin-guide/kernel-parameters.rst銆。

## threads-max


该值控制在可以使用 `fork()` 创建的线程的最大数量

在内核初始化期间，内核会设置该值，使得即使创建了最大数量的线程，线程结构也
占用可用 RAM 页的一部分/8）

可以写入 `threads-max` 的最小值为 1

可以写入 `threads-max` 的最大值由常量 `FUTEX_TID_MASK`x3fffffff）给出

如果写入 `threads-max` 的值超出该范围，将发生 `EINVAL` 错误

## timer_migration


当设为非零值时，尝试将定时器从空闲 CPU 迁移出去，以使它们能更久地保持在低功
状态

默认是设置（1）

## traceoff_on_warning


当设置时，在命中 `WARN()` 时禁用跟踪（参见 Documentation/trace/ftrace.rst）

## tracepoint_printk


当跟踪点被发送到 printk() 时（`tp_printk` 启用）：
```
    echo 0 > /proc/sys/kernel/tracepoint_printk
```
```
    echo 1 > /proc/sys/kernel/tracepoint_printk
```
将再次把它们发送到 printk()

这仅在以内核启用`tp_printk` 启动时才有效

参见 Documentation/admin-guide/kernel-parameters.rst 
Documentation/trace/boottime-trace.rst銆。

## unaligned-trap


在那些未对齐访问会触发陷阱、且支持该特性（`CONFIG_SYSCTL_ARCH_UNALIGN_ALLOW`
目前`arc`、`parisc` `loongarch`）的体系结构上，控制是否捕获并模拟未对齐
陷阱（而不是直接失败）

= ========================================================
0 不模拟未对齐访问
1 模拟未对齐访问。这是默认设置
= ========================================================

另请参见 `ignore-unaligned-usertrap`_

## unknown_nmi_panic


本文件中的值影响处NMI 的行为。当该值为非零时，未知 NMI 被捕获并随后发生
panic。此时，内核调试信息会显示在控制台上

大多IA32 服务器都具有NMI 开关会触发未知 NMI，例如。如果系统挂起，尝试
按下 NMI 开关

## unprivileged_bpf_disabled


向该条目写入 1 将禁用对 `bpf()` 的非特权调用；一旦禁用，在没`CAP_SYS_ADMIN`
`CAP_BPF` 的情况下调用 `bpf()` 将返`-EPERM`。一旦设1，就无法再从运行中的
内核清除它

向该条目写入 2 也会禁用`bpf()` 的非特权调用，但是，如果需要，管理员之后仍
通过向该条目写入 0 1 来更改该设置

如果在内核配置中启用`BPF_UNPRIV_DEFAULT_OFF`，那么该条目将默认为 2 而不0

= =============================================================
0 启用`bpf()` 的非特权调用
1 禁用`bpf()` 的非特权调用且不可恢
2 禁用`bpf()` 的非特权调用
= =============================================================

## warn_limit


`panic_on_warn` 未设置的情况下，内核发生多少次警告之后应当触panic
将其设为 0 将禁用警告计数检查。将其设1 与设`panic_on_warn=1` 效果相同
默认值为 0

## watchdog


该参数可用于同时禁用或启用软锁死检测器NMI 看门狗（即硬锁死检测器）

= ==============================
0 禁用两个锁死检测器
1 启用两个锁死检测器
= ==============================

软锁死检测器NMI 看门狗也可以使用 `soft_watchdog` `nmi_watchdog`
参数分别单独禁用或启用
```
   cat /proc/sys/kernel/watchdog
```
该命令的输出 1）表`soft_watchdog` `nmi_watchdog` 的逻辑 OR

## watchdog_cpumask


该值可用于控制在哪CPU 上可以运行看门狗。默认的 cpumask 是所有可能的
核心，但如果内核配置中启用了 `NO_HZ_FULL`，并且通过 `nohz_full=` 启动参数
指定了核心，那些核心默认被排除在外

离线的核心可以包含在该掩码中，如果该核心之后上线，看门狗将根据掩码值启动

通常只有`nohz_full` 情况下才会触碰该值，以在怀疑那些默认没有运行看门狗
的核心上发生内核锁死时，重新启用这些核心

该参数值是 cpumask 的标cpulist 格式，因此例如要在核0 4 
启用看门狗：
```
  echo 0,2-4 > /proc/sys/kernel/watchdog_cpumask
```
## watchdog_thresh


该值可用于控制 hrtimer NMI 事件的频率，以及软锁死和硬锁死的阈值。默
阈值为 10 秒

软锁死阈值为（`2 * watchdog_thresh`）。将该可调参数设为零将完全禁用锁死检测
