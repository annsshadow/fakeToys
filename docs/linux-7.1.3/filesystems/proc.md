
## /proc 文件系统


=====================  =======================================  ================
/proc/sys              Terrehon Bowden <terrehon@pacbell.net>,  1999 年 10 月 7 日
                       Bodo Bauer <bb@ricochet.net>
2.4.x update	       Jorge Nerin <comandante@zaralinux.com>   2000 年 11 月 14 日
move /proc/sys	       Shen Feng <shen@cn.fujitsu.com>	       2009 年 4 月 1 日
fixes/update part 1.1  Stefani Seibold <stefani@seibold.net>    2009 年 6 月 9 日
=====================  =======================================  ================



  0     前言
  0.1	简介/致谢
  0.2	法律声明

  1	收集系统信息
  1.1	进程特定的子目录
  1.2	内核数据
  1.3	/proc/ide 中的 IDE 设备
  1.4	/proc/net 中的网络信息
  1.5	SCSI 信息
  1.6	/proc/parport 中的并行端口信息
  1.7	/proc/tty 中的 TTY 信息
  1.8	/proc/stat 中的杂项内核统计
  1.9	Ext4 文件系统参数

  2	修改系统参数

  3	每进程参数
  3.1	/proc/<pid>/oom_adj & /proc/<pid>/oom_score_adj - 调整 oom-killer
							分数
  3.2	/proc/<pid>/oom_score - 显示当前 oom-killer 分数
  3.3	/proc/<pid>/io - 显示 IO 统计字段
  3.4	/proc/<pid>/coredump_filter - 核心转储过滤设置
  3.5	/proc/<pid>/mountinfo - 关于挂载的信息
  3.6	/proc/<pid>/comm  & /proc/<pid>/task/<tid>/comm
  3.7   /proc/<pid>/task/<tid>/children - 关于任务子进程的信息
  3.8   /proc/<pid>/fdinfo/<fd> - 关于已打开文件的信息
  3.9   /proc/<pid>/map_files - 关于内存映射文件的信息
  3.10  /proc/<pid>/timerslack_ns - 任务 timerslack 值
  3.11	/proc/<pid>/patch_state - Livepatch 补丁操作状态
  3.12	/proc/<pid>/arch_status - 任务架构特定信息
  3.13  /proc/<pid>/fd - 指向打开文件的符号链接列表
  3.14  /proc/<pid>/ksm_stat - 关于进程 ksm 状态的信息

  4	配置 procfs
  4.1	挂载选项

  5	文件系统行为

## 前言


### 0.1 简介/致谢


我们要感谢 Alan Cox、Rik van Riel、Alexey Kuznetsov 以及许多其他人，感谢他们帮助编写本文档。我们还要特别感谢 Andi Kleen 提供的文档，我们在创建本文档时大量依赖它，以及他提供的额外信息。感谢所有为 Linux 内核贡献源代码或文档，并帮助创造这款出色软件的其他人…… :)

本文档的最新版本可在线获取：
https://www.kernel.org/doc/html/latest/filesystems/proc.html

### 0.2 法律声明


我们不保证本文档的正确性，如果你因为文档不正确而搞乱了系统来找我们抱怨，我们不会感到负责……

## 第 1 章：收集系统信息


### 本章内容


- 研究伪文件系统 /proc 的属性及其提供正在运行的 Linux 系统信息的能力
- 检查 /proc 的结构
- 揭示有关内核和系统中运行进程的各种信息

------------------------------------------------------------------------------

proc 文件系统充当内核内部数据结构的一个接口。它可以用来获取系统信息，并在运行时（sysctl）更改某些内核参数。

首先，我们来看看 /proc 的只读部分。在第 2 章中，我们将展示如何使用 /proc/sys 来更改设置。

### 1.1 进程特定的子目录


目录 /proc（除其他内容外）包含系统中每个运行进程的子目录，该子目录以进程 ID（PID）命名。

链接 'self' 指向正在读取该文件系统的进程。每个进程子目录都具有表 1-1 中列出的条目。

进程可以在没有任何额外权限的情况下从 /proc/PID/* 读取自身信息。当读取其他进程的 /proc/PID/* 信息时，读取进程需要具有 PTRACE_MODE_READ 访问权限的 CAP_SYS_PTRACE 能力，或者具有 CAP_PERFMON 能力。这适用于所有只读信息，如 `maps`、`environ`、`pagemap` 等。唯一的例外是 `mem` 文件，由于其读写性质，它需要具有更高权限 PTRACE_MODE_ATTACH 的 CAP_SYS_PTRACE 能力；CAP_PERFMON 能力不授予对其他进程的 /proc/PID/mem 的访问权限。

注意，对 /proc/<pid> 或其包含的任何文件或子目录的已打开文件描述符，并不能防止 <pid> 在该进程退出时被其他进程复用。对已打开 /proc/<pid> 文件描述符、且对应于已死亡进程的操作，绝不会作用于内核可能碰巧也分配了进程 ID <pid> 的任何新进程。相反，对这些 FD 的操作通常会以 ESRCH 失败。


 =============  ===============================================================
 文件	内容
 =============  ===============================================================
 clear_refs	清除 smaps 输出中显示的页引用位
 cmdline	命令行参数
 cpu		当前和上次执行所在的 cpu	(2.4)(smp)
 cwd		指向当前工作目录的链接
 environ	环境变量的值
 exe		指向该进程可执行文件的链接
 fd		包含所有文件描述符的目录
 maps		到可执行文件和库文件的内存映射	(2.4)
 mem		该进程持有的内存
 root		指向该进程根目录的链接
 stat		进程状态
 statm		进程内存状态信息
 status		人类可读形式的进程状态
 wchan		启用 CONFIG_KALLSYMS=y 时存在：显示任务被阻塞于其中的内核函数
		符号，未阻塞则为 "0"。
 pagemap	页表
 stack		报告完整的栈回溯，通过 CONFIG_STACKTRACE 启用
 smaps		基于 maps 的扩展，显示每个映射的内存消耗及其关联的标志
 smaps_rollup	该进程所有映射的 smaps 累计统计。这可以从 smaps 推导，但更快更方便
 numa_maps	基于 maps 的扩展，显示每个映射的内存局部性、绑定策略以及
		内存使用量（以页为单位）。
 =============  ===============================================================

例如，要获取进程的状态信息，只需：

```
  >cat /proc/self/status
  Name:   cat
  State:  R (running)
  Tgid:   5452
  Pid:    5452
  PPid:   743
  TracerPid:      0						(2.4)
  Uid:    501     501     501     501
  Gid:    100     100     100     100
  FDSize: 256
  Groups: 100 14 16
  Kthread:    0
  VmPeak:     5004 kB
  VmSize:     5004 kB
  VmLck:         0 kB
  VmHWM:       476 kB
  VmRSS:       476 kB
  RssAnon:             352 kB
  RssFile:             120 kB
  RssShmem:              4 kB
  VmData:      156 kB
  VmStk:        88 kB
  VmExe:        68 kB
  VmLib:      1412 kB
  VmPTE:        20 kb
  VmSwap:        0 kB
  HugetlbPages:          0 kB
  CoreDumping:    0
  THP_enabled:	  1
  Threads:        1
  SigQ:   0/28578
  SigPnd: 0000000000000000
  ShdPnd: 0000000000000000
  SigBlk: 0000000000000000
  SigIgn: 0000000000000000
  SigCgt: 0000000000000000
  CapInh: 00000000fffffeff
  CapPrm: 0000000000000000
  CapEff: 0000000000000000
  CapBnd: ffffffffffffffff
  CapAmb: 0000000000000000
  NoNewPrivs:     0
  Seccomp:        0
  Speculation_Store_Bypass:       thread vulnerable
  SpeculationIndirectBranch:      conditional enabled
  voluntary_ctxt_switches:        0
  nonvoluntary_ctxt_switches:     1
```

这显示的信息与你用 ps 命令看到的信息几乎相同。实际上，ps 使用 proc 文件系统来获取其信息。但读取文件 /proc/PID/status 可以让你更详细地查看该进程。它的字段在表 1-2 中描述。

statm 文件包含关于进程内存使用更详细的信息。它的七个字段在表 1-3 中解释。stat 文件包含关于进程本身的详细信息。它的字段在表 1-4 中解释。

（针对 SMP CONFIG 用户）

为了使统计可扩展，RSS 相关的信息以异步方式处理，其值可能不是很精确。要查看某个时刻的精确快照，你可以查看 /proc/<pid>/smaps 文件并扫描页表。这很慢但非常精确。


 ==========================  ===================================================
 字段                       内容
 ==========================  ===================================================
 Name                        可执行文件的文件名
 Umask                       文件模式创建掩码
 State                       状态（R 为运行中，S 为睡眠，D 为处于不可中断等待中的睡眠，
			     Z 为僵尸进程，T 为被跟踪或停止）
 Tgid                        线程组 ID
 Ngid                        NUMA 组 ID（无则为 0）
 Pid                         进程 id
 PPid                        父进程的进程 id
 TracerPid                   跟踪此进程的进程 PID（如果没有，或跟踪者在当前 pid 命名空间之外则为 0）
 Uid                         真实、有效、保存集和文件系统 UIDs
 Gid                         真实、有效、保存集和文件系统 GIDs
 FDSize                      当前已分配的文件描述符槽数量
 Groups                      补充组列表
 NStgid                      后代命名空间线程组 ID 层级
 NSpid                       后代命名空间进程 ID 层级
 NSpgid                      后代命名空间进程组 ID 层级
 NSsid                       后代命名空间会话 ID 层级
 Kthread                     内核线程标志，1 为是，0 为否
 VmPeak                      峰值虚拟内存大小
 VmSize                      程序总大小
 VmLck                       锁定内存大小
 VmPin                       固定（pinned）内存大小
 VmHWM                       峰值常驻集大小（"高水位标记"）
 VmRSS                       内存部分的大小。它包含以下三个部分
                             （VmRSS = RssAnon + RssFile + RssShmem）
 RssAnon                     常驻匿名内存大小
 RssFile                     常驻文件映射大小
 RssShmem                    常驻 shmem 内存大小（包括 SysV shm、
                             tmpfs 映射和共享匿名映射）
 VmData                      私有数据段大小
 VmStk                       栈段大小
 VmExe                       文本段大小
 VmLib                       共享库代码大小
 VmPTE                       页表项大小
 VmSwap                      匿名私有数据使用的交换量
                             （不包括 shmem 交换使用量）
 HugetlbPages                hugetlb 内存部分大小
 CoreDumping                 进程内存当前正在被转储
                             （杀死进程可能导致核心文件损坏）
 THP_enabled                 进程被允许使用 THP（当进程上设置了
                             PR_SET_THP_DISABLE 以完全禁用 THP 时返回 0，
                             而不仅仅是部分禁用）
 Threads                     线程数量
 SigQ                        已排队信号数/队列最大数
 SigPnd                      线程挂起信号位图
 ShdPnd                      进程共享挂起信号位图
 SigBlk                      阻塞信号位图
 SigIgn                      忽略信号位图
 SigCgt                      捕获信号位图
 CapInh                      可继承能力位图
 CapPrm                      允许的能力位图
 CapEff                      有效能力位图
 CapBnd                      能力边界集位图
 CapAmb                      环境能力位图
 NoNewPrivs                  no_new_privs，类似 prctl(PR_GET_NO_NEW_PRIV, ...)
 Seccomp                     seccomp 模式，类似 prctl(PR_GET_SECCOMP, ...)
 Speculation_Store_Bypass    推测性存储绕过缓解状态
 SpeculationIndirectBranch   间接分支推测模式
 Cpus_allowed                该进程可在其上运行的 CPU 掩码
 Cpus_allowed_list           与前者相同，但为"列表格式"
 Mems_allowed               该进程允许的内存节点掩码
 Mems_allowed_list           与前者相同，但为"列表格式"
 voluntary_ctxt_switches     自愿上下文切换次数
 nonvoluntary_ctxt_switches  非自愿上下文切换次数
 ==========================  ===================================================



 ======== ===============================	==============================
 字段    内容
 ======== ===============================	==============================
 size     程序总大小（页）			（同 status 中的 VmSize）
 resident 内存部分大小（页）			（同 status 中的 VmRSS）
 shared   共享页数量				（即由文件支持，同 status 中的 RssFile+RssShmem）
 trs      为'代码'的页数量			（不包括库；已损坏，包含数据段）
 lrs      库页数量				（2.6 上始终为 0）
 drs      数据/栈的页数量			（包括库；已损坏，包含库文本）
 dt       脏页数量				（2.6 上始终为 0）
 ======== ===============================	==============================



  ============= ===============================================================
  字段         内容
  ============= ===============================================================
  pid           进程 id
  tcomm         可执行文件的文件名
  state         状态（R 为运行中，S 为睡眠，D 为处于不可中断等待中的睡眠，
                 Z 为僵尸进程，T 为被跟踪或停止）
  ppid          父进程的进程 id
  pgrp          进程的 pgrp
  sid           会话 id
  tty_nr        进程使用的 tty
  tty_pgrp      tty 的 pgrp
  flags         任务标志
  min_flt       次要缺页次数
  cmin_flt      包含子进程的次要缺页次数
  maj_flt       主要缺页次数
  cmaj_flt      包含子进程的主要缺页次数
  utime         用户模式 jiffies
  stime         内核模式 jiffies
  cutime        包含子进程的用户模式 jiffies
  cstime        包含子进程的内核模式 jiffies
  priority      优先级级别
  nice          nice 级别
  num_threads   线程数量
  it_real_value	(已废弃，始终为 0)
  start_time    进程在系统启动后启动的时间
  vsize         虚拟内存大小
  rss           常驻集内存大小
  rsslim        当前 rss 的字节数限制
  start_code    程序文本可运行的地址上限
  end_code      程序文本可运行的地址下限
  start_stack   主进程栈起始地址
  esp           ESP 当前值
  eip           EIP 当前值
  pending       挂起信号位图
  blocked       阻塞信号位图
  sigign        忽略信号位图
  sigcatch      捕获信号位图
  0		(占位符，曾为 wchan 地址，
		改用 /proc/PID/wchan)
  0             (占位符)
  0             (占位符)
  exit_signal   退出时发送给父线程的信号
  task_cpu      任务被调度到的 CPU
  rt_priority   实时优先级
  policy        调度策略（man sched_setscheduler）
  blkio_ticks   等待块设备 IO 花费的时间
  gtime         任务在 jiffies 中的客户（guest）时间
  cgtime        任务子进程在 jiffies 中的客户时间
  start_data    程序数据+bss 放置的地址上限
  end_data      程序数据+bss 放置的地址下限
  start_brk     可通过 brk() 扩展程序堆的地址上限
  arg_start     程序命令行放置的地址上限
  arg_end       程序命令行放置的地址下限
  env_start     程序环境放置的地址上限
  env_end       程序环境放置的地址下限
  exit_code     线程的 exit_code，形式为 waitpid 系统调用所报告的值
  ============= ===============================================================

/proc/PID/maps 文件包含当前已映射的内存区域及其访问权限。

```
    address           perms offset  dev   inode      pathname

    08048000-08049000 r-xp 00000000 03:00 8312       /opt/test
    08049000-0804a000 rw-p 00001000 03:00 8312       /opt/test
    0804a000-0806b000 rw-p 00000000 00:00 0          [heap]
    a7cb1000-a7cb2000 ---p 00000000 00:00 0
    a7cb2000-a7eb2000 rw-p 00000000 00:00 0
    a7eb2000-a7eb3000 ---p 00000000 00:00 0
    a7eb3000-a7ed5000 rw-p 00000000 00:00 0
    a7ed5000-a8008000 r-xp 00000000 03:00 4222       /lib/libc.so.6
    a8008000-a800a000 r--p 00133000 03:00 4222       /lib/libc.so.6
    a800a000-a800b000 rw-p 00135000 03:00 4222       /lib/libc.so.6
    a800b000-a800e000 rw-p 00000000 00:00 0
    a800e000-a8022000 r-xp 00000000 03:00 14462      /lib/libpthread.so.0
    a8022000-a8023000 r--p 00013000 03:00 14462      /lib/libpthread.so.0
    a8023000-a8024000 rw-p 00014000 03:00 14462      /lib/libpthread.so.0
    a8024000-a8027000 rw-p 00000000 00:00 0
    a8027000-a8043000 r-xp 00000000 03:00 8317       /lib/ld-linux.so.2
    a8043000-a8044000 r--p 0001b000 03:00 8317       /lib/ld-linux.so.2
    a8044000-a8045000 rw-p 0001c000 03:00 8317       /lib/ld-linux.so.2
    aff35000-aff4a000 rw-p 00000000 00:00 0          [stack]
    ffffe000-fffff000 r-xp 00000000 00:00 0          [vdso]
```

其中 "address" 是它所占用的进程地址空间，"perms"：

```
 r = read
 w = write
 x = execute
 s = shared
 p = private (copy on write)
```

"offset" 是映射内的偏移，"dev" 是设备（major:minor），"inode" 是该设备上的 inode。0 表示没有与该内存区域关联的 inode，BSS（未初始化数据）就是这种情况。"pathname" 显示该映射关联的文件名。如果该映射未与文件关联：

 ===================        ===========================================
 [heap]                     程序的堆
 [stack]                    主进程的栈
 [vdso]                     "虚拟动态共享对象"，
                            内核系统调用处理程序
 [anon:<name>]              由用户空间命名的一个私有匿名映射
 [anon_shmem:<name>]        由用户空间命名的一个匿名共享内存映射
 ===================        ===========================================

或者如果为空，则该映射是匿名的。

从 6.11 内核开始，/proc/PID/maps 提供了一个替代的基于 ioctl() 的 API，能够灵活且高效地查询和过滤单个 VMA。这个接口是二进制的，旨在用于更高效、更方便的程序化使用。`struct procmap_query`（定义在 linux/fs.h UAPI 头文件中）作为 `PROCMAP_QUERY` ioctl() 命令的输入/输出参数。有关查询语义、支持的标志、返回的数据以及一般 API 使用信息的详细信息，请参阅 linus/fs.h UAPI 头文件中的注释。

/proc/PID/smaps 是基于 maps 的扩展，显示进程每个映射的内存消耗。对于每个映射（即虚拟：

```
    08048000-080bc000 r-xp 00000000 03:02 13130      /bin/bash

    Size:               1084 kB
    KernelPageSize:        4 kB
    MMUPageSize:           4 kB
    Rss:                 892 kB
    Pss:                 374 kB
    Pss_Dirty:             0 kB
    Shared_Clean:        892 kB
    Shared_Dirty:          0 kB
    Private_Clean:         0 kB
    Private_Dirty:         0 kB
    Referenced:          892 kB
    Anonymous:             0 kB
    KSM:                   0 kB
    LazyFree:              0 kB
    AnonHugePages:         0 kB
    FilePmdMapped:         0 kB
    ShmemPmdMapped:        0 kB
    Shared_Hugetlb:        0 kB
    Private_Hugetlb:       0 kB
    Swap:                  0 kB
    SwapPss:               0 kB
    Locked:                0 kB
    THPeligible:           0
    VmFlags: rd ex mr mw me dw
```

这些行中的第一行显示的信息与 /proc/PID/maps 中显示的映射信息相同。后面的行显示：映射的大小（size）；在支持一个 VMA 时分配的最小可能页大小（KernelPageSize），它是可修改 VMA 的粒度；MMU 在支持一个 VMA 时可使用的最小可能页大小（MMUPageSize）；当前驻留在 RAM 中的该映射的数量（RSS）；该进程在此映射中的比例份额（PSS）；以及该映射中干净和脏的共享与私有页的数量。

"KernelPageSize" 始终对应于 "MMUPageSize"，除非在 MMU 使用较小页大小的系统上模拟了更大的内核页大小，某些带有 hugetlb 的 PPC64 配置就是这样的情况。此外，"KernelPageSize" 和 "MMUPageSize" 始终对应于在 VMA 整个生命周期中可能遇到的最小可能粒度（回退）。这些值不受生效中的透明大页（Transparent Huge Pages），或任何对更大 MMU 页大小的使用（无论是通过架构性大页映射，还是 MMU 执行的虚拟范围的其他显式/隐式合并）的影响。"AnonHugePages"、"ShmemPmdMapped" 和 "FilePmdMapped" 提供了对 PMD 级别架构性大页映射使用情况的洞察。

一个进程的"比例集大小"（PSS）是它在内存中拥有的页计数，其中每个页都除以共享它的进程数量。因此，如果一个进程有 1000 个页完全归自己所有，并与另一个进程共享 1000 个页，它的 PSS 将为 1500。"Pss_Dirty" 是 PSS 中由脏页组成的部分。（不包含 "Pss_Clean"，但可以通过从 "Pss" 中减去 "Pss_Dirty" 来计算。）

传统上，一个页如果恰好被映射一次，则记为"私有"，而当被映射多次时（即使在同一个进程中被映射多次）记为"共享"。注意这种记账独立于 MAP_SHARED。

在某些内核配置中，属于更大分配（例如 THP）一部分的页的语义可能不同：如果一个较大分配的所有页**确定**映射在同一个进程中，即使该页在该进程中被映射多次，也记为"私有"。如果一个较大分配的任意页**可能**映射在不同的进程中，则记为"共享"。在某些情况下，一个较大的分配可能被视为"可能被多个进程映射"，即使实际已不再如此。

某些内核配置不跟踪较大分配中一部分的页被映射的精确次数。在这种情况下，计算 PSS 时，可能会使用该较大分配中每页的平均映射数，作为该页映射数量的近似值。这种情况下 PSS 计算将不精确。

"Referenced" 表示当前被标记为引用或已访问的内存量。

"Anonymous" 显示不属于任何文件的内存量。即使是与文件关联的映射也可能包含匿名页：当使用 MAP_PRIVATE 且某页被修改时，该文件页会被一个私有的匿名副本替换。

"KSM" 报告有多少页是 KSM 页。注意 KSM 放置的零页不包含在内，只包含实际的 KSM 页。

"LazyFree" 显示由 madvise(MADV_FREE) 标记的内存量。内存不会随 madvise() 立即释放。在内存压力下，如果内存是干净的，它会被释放。请注意，由于当前实现中使用的优化，打印的值可能低于真实值。如果不希望这样，请提交 bug 报告。

"AnonHugePages"、"ShmemPmdMapped" 和 "FilePmdMapped" 显示了当前由 PMD 级别的架构性大页映射所支持的透明大页的内存量。"AnonHugePages" 对应于不属于文件的内存，"ShmemPmdMapped" 对应于共享内存（shmem/tmpfs），"FilePmdMapped" 对应于文件支持的内存（不包括 shmem/tmpfs）。

对于未被 PMD 级别的架构性大页映射映射的透明大页（或类似概念），没有专门的条目。

"Shared_Hugetlb" 和 "Private_Hugetlb" 显示了由 hugetlbfs 页支持的内存量，由于历史原因，这部分**不**计入 "RSS" 或 "PSS" 字段。并且它们也不包含在 {Shared,Private}_{Clean,Dirty} 字段中。

"Swap" 显示了被使用但位于交换空间中的、原本应为匿名的内存量。

对于 shmem 映射，"Swap" 还包括底层 shmem 对象中已映射（且未被写时复制替换）并位于交换空间中的那部分大小。"SwapPss" 显示该映射的比例交换份额。与 "Swap" 不同，它不计入底层 shmem 对象换出的页。"Locked" 指示该映射是否被锁定在内存中。

"THPeligible" 指示该映射是否有资格分配任何当前已启用大小的自然对齐 THP 页。为真则为 1，否则为 0。

如果内核和 CPU 都支持保护键（pkeys），"ProtectionKey" 指示与该虚拟内存区域关联的内存保护键。

"VmFlags" 字段值得单独描述。该成员以双字母编码的方式表示与特定虚拟内存区域关联的内核标志。代码如下：

    ==    =============================================================
    rd    可读（readable）
    wr    可写（writeable）
    ex    可执行（executable）
    sh    共享（shared）
    mr    可读取（may read）
    mw    可写入（may write）
    me    可执行（may execute）
    ms    可共享（may share）
    gd    栈段向下增长（stack segment growns down）
    pf    纯 PFN 范围（pure PFN range）
    lo    页被锁定在内存中（pages are locked in memory）
    io    内存映射 I/O 区域（memory mapped I/O area）
    sr    提供了顺序读建议（sequential read advise provided）
    rr    提供了随机读建议（random read advise provided）
    dc    派生（fork）时不复制该区域（do not copy area on fork）
    de    重映射时不扩展该区域（do not expand area on remapping）
    ac    该区域可记账（area is accountable）
    nr    未为该区域保留交换空间（swap space is not reserved for the area）
    ht    该区域使用大 tlb 页（area uses huge tlb pages）
    sf    同步页错误（synchronous page fault）
    ar    架构特定标志（architecture specific flag）
    wf    派生时擦除（wipe on fork）
    dd    不包含在核心转储中（do not include area into core dump）
    sd    软脏标志（soft dirty flag）
    mm    混合映射区域（mixed map area）
    hg    大页建议标志（huge page advise flag）
    nh    无大页建议标志（no huge page advise flag）
    mg    可合并建议标志（mergeable advise flag）
    bt    arm64 BTI 保护页（arm64 BTI guarded page）
    mt    启用了 arm64 MTE 分配标签（arm64 MTE allocation tags are enabled）
    um    userfaultfd 缺失跟踪（userfaultfd missing tracking）
    uw    userfaultfd 写保护跟踪（userfaultfd wr-protect tracking）
    ui    userfaultfd 次要错误（userfaultfd minor fault）
    ss    影子/保护控制栈页（shadow/guarded control stack page）
    sl    已封存（sealed）
    lf    出错时锁定页（lock on fault pages）
    dp    始终可惰性释放的映射（always lazily freeable mapping）
    gu    可能包含保护区域（若未设置，则肯定不包含）
    ==    =============================================================

注意，不能保证每个标志和关联助记符在所有后续内核版本中都存在。事情会发生变化，标志可能会消失，或者相反——新增。它们含义的解释在未来也可能改变。因此这些标志的每个使用者都必须针对每个特定的内核版本来跟踪其确切语义。

只有当启用了 CONFIG_MMU 内核配置选项时，此文件才存在。

注意：读取 /proc/PID/maps 或 /proc/PID/smaps 本质上是存在竞态的（只有在单次读取调用中才能获得一致的输出）。

这通常在进行这些文件的部分读取、同时内存映射正在被修改时表现出来。尽管存在竞态，我们仍提供以下保证：

1) 映射的地址永远不会后退，这意味着任意两个区域永远不会重叠。
2) 如果在 smaps/maps 遍历的整个生命周期内某个给定 vaddr 上始终有内容，则会有对应的输出。

/proc/PID/smaps_rollup 文件包含与 /proc/PID/smaps 相同的字段，但它们的值是该进程所有映射对应值的总和。此外，它还包含以下字段：

- Pss_Anon
- Pss_File
- Pss_Shmem

它们表示如上为 smaps 所描述的匿名、文件和 shmem 页的比例份额。这些字段在 smaps 中被省略，因为每个映射都标识了它所包含的所有页的类型（anon、file 或 shmem）。因此 smaps_rollup 中的所有信息都可以从 smaps 推导出来，但代价要高得多。

/proc/PID/clear_refs 用于重置与进程关联的物理和虚拟页上的 PG_Referenced 和 ACCESSED/YOUNG 位，以及 pte 上的软脏位（详见 Documentation/admin-guide/mm/soft-dirty.rst）。

```
    > echo 1 > /proc/PID/clear_refs
```

```
    > echo 2 > /proc/PID/clear_refs
```

```
    > echo 3 > /proc/PID/clear_refs
```

```
    > echo 4 > /proc/PID/clear_refs
```

要重置峰值常驻集大小（"高水位标记"）为进程的：

```
    > echo 5 > /proc/PID/clear_refs
```

写入 /proc/PID/clear_refs 的任何其他值都不会产生效果。

/proc/pid/pagemap 给出 PFN，可用于通过 /proc/kpageflags 查找 pageflags，以及通过 /proc/kpagecount 查找一个页被映射的次数。详细解释见 Documentation/admin-guide/mm/pagemap.rst。

/proc/pid/numa_maps 是基于 maps 的扩展，显示内存局部性和绑定策略，以及每个映射的内存使用量（以页为单位）。输出遵循通用格式，其中映射细节由

```
    address   policy    mapping details

    00400000 default file=/usr/local/bin/app mapped=1 active=0 N3=1 kernelpagesize_kB=4
    00600000 default file=/usr/local/bin/app anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206000000 default file=/lib64/ld-2.12.so mapped=26 mapmax=6 N0=24 N3=2 kernelpagesize_kB=4
    320621f000 default file=/lib64/ld-2.12.so anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206220000 default file=/lib64/ld-2.12.so anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206221000 default anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206800000 default file=/lib64/libc-2.12.so mapped=59 mapmax=21 active=55 N0=41 N3=18 kernelpagesize_kB=4
    320698b000 default file=/lib64/libc-2.12.so
    3206b8a000 default file=/lib64/libc-2.12.so anon=2 dirty=2 N3=2 kernelpagesize_kB=4
    3206b8e000 default file=/lib64/libc-2.12.so anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206b8f000 default anon=3 dirty=3 active=1 N3=3 kernelpagesize_kB=4
    7f4dc10a2000 default anon=3 dirty=3 N3=3 kernelpagesize_kB=4
    7f4dc10b4000 default anon=2 dirty=2 active=1 N3=2 kernelpagesize_kB=4
    7f4dc1200000 default file=/anon_hugepage\040(deleted) huge anon=1 dirty=1 N3=1 kernelpagesize_kB=2048
    7fff335f0000 default stack anon=3 dirty=3 N3=3 kernelpagesize_kB=4
    7fff3369d000 default mapped=1 mapmax=35 active=0 N3=1 kernelpagesize_kB=4
```

其中：

"address" 是该映射的起始地址；

"policy" 报告为该映射设置的 NUMA 内存策略（见 Documentation/admin-guide/mm/numa_memory_policy.rst）；

"mapping details" 汇总了映射数据，如映射类型、页使用计数器、节点局部性页计数器（N0 == node0，N1 == node1，……）以及支持该映射的内核页大小（以 KB 为单位）。

注意，某些内核配置不跟踪较大分配（例如 THP）中一部分的页被映射的精确次数。在这些配置中，"mapmax" 可能对应于此类较大分配中每页的平均映射数。
### 1.2 内核数据


与进程条目类似，内核数据文件提供关于运行中的内核的信息。用于获取这些信息的文件包含在 /proc 中，并列于表 1-5。并非所有这些文件都会出现在你的系统中。这取决于内核配置和已加载的模块，哪些文件存在，哪些缺失。


 ============ ===============================================================
 文件        内容
 ============ ===============================================================
 allocinfo    内存分配性能分析信息
 apm          高级电源管理（Advanced power management）信息
 bootconfig   从 boot config 获取的内核命令行，
 	      以及，如果有来自引导加载程序的
 	      内核参数，则有一行 "# Parameters from bootloader:"
 	      后跟包含这些参数的行，前面加 "# "。	(5.5)
 buddyinfo    内核内存分配器信息（见正文）			(2.5)
 bus          包含总线特定信息的目录
 cmdline      内核命令行，包括来自引导加载程序和嵌入在内核映像中的
 cpuinfo      关于 CPU 的信息
 devices      可用设备（块设备和字符设备）
 dma          已使用的 DMA 通道
 filesystems  支持的文件系统
 driver       在此分组的不同驱动，目前为 rtc			(2.4)
 execdomains  执行域（Execdomains），与安全相关			(2.4)
 fb 	      帧缓冲（Frame Buffer）设备			(2.4)
 fs 	      文件系统参数，目前为 nfs/exports		(2.4)
 ide          包含关于 IDE 子系统信息的目录
 interrupts   中断使用情况
 iomem 	      内存映射（Memory map）				(2.4)
 ioports      I/O 端口使用情况
 irq 	      irq 到 cpu 亲和性的掩码				(2.4)(smp?)
 isapnp       ISA PnP (Plug&Play) 信息				(2.4)
 kcore        内核核心映像（可以是 ELF 或 A.OUT（在 2.4 中已废弃））
 kmsg         内核消息
 ksyms        内核符号表
 loadavg      过去 1、5 和 15 分钟的平均负载；
                当前可运行进程数（运行或在就绪队列中）；
                系统中进程总数；
                最后创建的 pid。
                除"当前可运行进程数"和"系统中进程总数"外，
                所有字段都用空格分隔，这两者之间用斜杠（'/'）分隔。示例：
                0.61 0.61 0.55 3/828 22084
 locks        内核锁
 meminfo      内存信息
 misc         杂项
 modules      已加载模块列表
 mounts       已挂载的文件系统
 net          网络信息（见正文）
 pagetypeinfo 额外的页分配器信息（见正文）			(2.5)
 partitions   系统已知的 partitions 表
 pci 	      PCI 总线的已废弃信息（新方式 -> /proc/bus/pci/，
                由 lspci 解耦				(2.4)
 rtc          实时时钟（Real time clock）
 scsi         SCSI 信息（见正文）
 slabinfo     Slab 池信息
 softirqs     softirq 使用情况
 stat         总体统计
 swaps        交换空间利用率
 sys          见第 2 章
 sysvipc      SysVIPC 资源（msg、sem、shm）的信息			(2.4)
 tty 	      tty 驱动的信息
 uptime       自启动以来的墙上时钟时间，以及所有 cpu 的合并空闲时间
 version      内核版本
 video 	      video 资源的 bttv 信息				(2.4)
 vmallocinfo  显示 vmalloced 区域
 ============ ===============================================================

例如，你可以检查当前正在使用的中断以及哪些

```
  > cat /proc/interrupts
             CPU0
    0:    8728810          XT-PIC  timer
    1:        895          XT-PIC  keyboard
    2:          0          XT-PIC  cascade
    3:     531695          XT-PIC  aha152x
    4:    2014133          XT-PIC  serial
    5:      44401          XT-PIC  pcnet_cs
    8:          2          XT-PIC  rtc
   11:          8          XT-PIC  i82365
   12:     182918          XT-PIC  PS/2 Mouse
   13:          1          XT-PIC  fpu
   14:    1232265          XT-PIC  ide0
   15:          7          XT-PIC  ide1
  NMI:          0
```

在 2.4.* 中，向该文件添加了 couple 行 LOC & ERR（这次是

```
  > cat /proc/interrupts

             CPU0       CPU1
    0:    1243498    1214548    IO-APIC-edge  timer
    1:       8949       8958    IO-APIC-edge  keyboard
    2:          0          0          XT-PIC  cascade
    5:      11286      10161    IO-APIC-edge  soundblaster
    8:          1          0    IO-APIC-edge  rtc
    9:      27422      27407    IO-APIC-edge  3c503
   12:     113645     113873    IO-APIC-edge  PS/2 Mouse
   13:          0          0          XT-PIC  fpu
   14:      22491      24012    IO-APIC-edge  ide0
   15:       2183       2415    IO-APIC-edge  ide1
   17:      30564      30414   IO-APIC-level  eth0
   18:        177        164   IO-APIC-level  bttv
  NMI:    2457961    2457959
  LOC:    2457882    2457881
  ERR:       2155
```

在这种情况下 NMI 增加，因为每次定时器中断都会生成一个 NMI（不可屏蔽中断），NMI 看门狗用它来检测死锁。

LOC 是每个 CPU 内部 APIC 的本地中断计数器。

ERR 在 IO-APIC 总线（在 SMP 系统中连接 CPU 的总线）出现错误时增加。这意味着检测到了一个错误，IO-APIC 会自动重试传输，因此这应该不是大问题，但你应该阅读 SMP-FAQ。

在 2.6.2* 中，/proc/interrupts 再次被扩展。这次的目标是让 /proc/interrupts 显示系统中使用的每个 IRQ 向量，而不仅仅是那些被认为是"最重要"的。新的向量有：

THR
  当机器检查阈值计数器（通常计数内存或缓存的 ECC 纠正错误）超过可配置阈值时引发的中断。仅在某些系统上可用。

TRM
  当 CPU 的温度阈值被超过时发生热事件中断。当温度降回正常时也可能生成此中断。

SPU
  伪中断（spurious interrupt）是某个 IO 设备在能被 APIC 完全处理之前被引发又拉低的中断。因此 APIC 看到了中断，但不知道它来自哪个设备。对于这种情况，APIC 将生成 IRQ 向量为 0xff 的中断。这也可能由芯片组 bug 引起。

RES、CAL、TLB
  重新调度、调用和 TLB 刷新中断是根据操作系统的需要从一个 CPU 发送到另一个 CPU 的。通常，它们的统计信息被内核开发者和感兴趣的用户用来确定给定类型中断的发生情况。

上述 IRQ 向量仅在相关时显示。例如，阈值向量在 x86_64 平台上不存在。当系统为单处理器时，其他向量会被抑制。截至本文撰写时，只有 i386 和 x86_64 平台支持新的 IRQ 向量显示。

值得关注的一点是 2.4 中引入了 /proc/irq 目录。它可以用来设置 IRQ 到 CPU 的亲和性。这意味着你可以将 IRQ"挂钩"到仅一个 CPU，或排除某个 CPU 处理 IRQ。irq 子目录的内容是每个 IRQ 的一个子目录，以及 default_smp_affinity。

```
  > ls /proc/irq/
  0  10  12  14  16  18  2  4  6  8  default_smp_affinity
  1  11  13  15  17  19  3  5  7  9
  > ls /proc/irq/0/
  smp_affinity
```

smp_affinity 是一个位掩码，可以在其中指定哪些 CPU 可以处理

```
  > echo 1 > /proc/irq/10/smp_affinity
```

这意味着只有第一个 CPU 会处理该 IRQ，但你也可以 echo 5，这意味着只有第一个和第三个 CPU 可以处理该 IRQ。

```
  > cat /proc/irq/0/smp_affinity
  ffffffff
```

还有一个替代接口 smp_affinity_list，允许指定

```
  > cat /proc/irq/0/smp_affinity_list
  1024-1031
```

default_smp_affinity 掩码适用于所有非活动 IRQ，即尚未被分配/激活、因此缺少 /proc/irq/[0-9]* 目录的 IRQ。

SMP 系统上的 node 文件显示使用 IRQ 的设备所报告的、其自身所附加到的节点。该硬件局部性信息不包括任何可能的驱动局部性偏好的信息。

IRQ 的路由方式由 IO-APIC 处理，并且在所有被允许处理它的 CPU 之间采用轮询（Round Robin）。像往常一样，内核拥有比你更多的信息，并且做得比你好，因此默认值对几乎所有人来说都是最佳选择。[注意这仅适用于那些支持"Round Robin"中断分布的 IO-APIC。]

/proc 中还有三个更重要的子目录：net、scsi 和 sys。一般的规则是，这些目录的内容，甚至它们的存在，都取决于你的内核配置。如果未启用 SCSI，则 scsi 目录可能不存在。net 也是一样，它只有在运行中的内核存在网络支持时才存在。

slabinfo 文件提供 slab 级别的内存使用信息。Linux 在 2.2 版本中使用 slab 池进行页级别以上的内存管理。常用对象拥有自己的 slab 池（如网络缓冲区、目录缓存等）。

```
    > cat /proc/buddyinfo

    Node 0, zone      DMA      0      4      5      4      4      3 ...
    Node 0, zone   Normal      1      0      0      1    101      8 ...
    Node 0, zone  HighMem      2      0      0      1      1      0 ...
```

外部碎片在某些工作负载下是个问题，buddyinfo 是帮助诊断这些问题的一个有用工具。Buddyinfo 会给你一个线索，告诉你能够安全分配多大的区域，或者为什么之前的分配会失败。

每一列表示可用的某个阶（order）的页数量。在这种情况下，ZONE_DMA 中有 0 个 2^0*PAGE_SIZE 的块，ZONE_DMA 中有 4 个 2^1*PAGE_SIZE 的块，ZONE_NORMAL 中有 101 个 2^4*PAGE_SIZE 的块，等等……

关于外部碎片的更多信息可以在以下找到：

```
    > cat /proc/pagetypeinfo
    Page block order: 9
    Pages per block:  512

    Free pages count per migrate type at order       0      1      2      3      4      5      6      7      8      9     10
    Node    0, zone      DMA, type    Unmovable      0      0      0      1      1      1      1      1      1      1      0
    Node    0, zone      DMA, type  Reclaimable      0      0      0      0      0      0      0      0      0      0      0
    Node    0, zone      DMA, type      Movable      1      1      2      1      2      1      1      0      1      0      2
    Node    0, zone      DMA, type      Reserve      0      0      0      0      0      0      0      0      0      1      0
    Node    0, zone      DMA, type      Isolate      0      0      0      0      0      0      0      0      0      0      0
    Node    0, zone    DMA32, type    Unmovable    103     54     77      1      1      1     11      8      7      1      9
    Node    0, zone    DMA32, type  Reclaimable      0      0      2      1      0      0      0      0      1      0      0
    Node    0, zone    DMA32, type      Movable    169    152    113     91     77     54     39     13      6      1    452
    Node    0, zone    DMA32, type      Reserve      1      2      2      2      2      0      1      1      1      1      0
    Node    0, zone    DMA32, type      Isolate      0      0      0      0      0      0      0      0      0      0      0

    Number of blocks type     Unmovable  Reclaimable      Movable      Reserve      Isolate
    Node 0, zone      DMA            2            0            5            1            0
    Node 0, zone    DMA32           41            6          967            2            0
```

内核中的碎片避免通过将不同迁移类型的页分组到称为页块（page block）的相同连续内存区域来工作。页块通常是默认大页大小，例如 X86-64 上为 2MB。通过根据页的可移动性对其进行分组，内核可以回收页块内的页以满足高阶分配。

pagetypinfo 以关于页块大小的信息开头。然后它给出与 buddyinfo 相同类型的信息，只是按迁移类型细分，并以每种类型有多少个页块的详细信息结束。

如果 min_free_kbytes 已被正确调整（由来自 libhugetlbfs 的 hugeadm 提出建议 https://github.com/libhugetlbfs/libhugetlbfs/），则可以估计在给定时刻可以分配的大页的可能数量。除非内存已被 mlock() 锁定，否则所有"Movable"块都应该是可分配的。一些 Reclaimable 块也应该是可分配的，尽管为此可能必须回收大量文件系统元数据。

#### allocinfo


提供关于代码库中所有位置的内存分配的信息。代码中的每个分配由其源文件、行号、模块（如果来自可加载模块）以及调用该分配的函数标识。会报告每个位置分配的字节数和调用次数。第一行指示文件的版本，第二行是列出文件中各字段的表头。
如果文件版本为 2.0 或更高，则每行可能包含额外的 <key>:<value> 对，表示关于调用点的额外信息。例如，如果计数器不准确，该行会被附加 "accurate:no" 对。

v2 中支持的标记：
accurate:no

              由于未能分配内存来跟踪在此位置进行的部分分配，本行中计数器的绝对值不准确。这些计数器的增量是准确的，因此计数器可用于跟踪分配大小和计数变化。

示例输出。

```

    > tail -n +3 /proc/allocinfo | sort -rn
   127664128    31168 mm/page_ext.c:270 func:alloc_page_ext
    56373248     4737 mm/slub.c:2259 func:alloc_slab_page
    14880768     3633 mm/readahead.c:247 func:page_cache_ra_unbounded
    14417920     3520 mm/mm_init.c:2530 func:alloc_large_system_hash
    13377536      234 block/blk-mq.c:3421 func:blk_mq_alloc_rqs
    11718656     2861 mm/filemap.c:1919 func:__filemap_get_folio
     9192960     2800 kernel/fork.c:307 func:alloc_thread_stack_node
     4206592        4 net/netfilter/nf_conntrack_core.c:2567 func:nf_ct_alloc_hashtable
     4136960     1010 drivers/staging/ctagmod/ctagmod.c:20 [ctagmod] func:ctagmod_start
     3940352      962 mm/memory.c:4214 func:alloc_anon_folio
     2894464    22613 fs/kernfs/dir.c:615 func:__kernfs_new_node
     ...

```

#### meminfo


提供关于内存分布和利用率的信息。这因架构和编译选项而异。这里报告的一些计数器有重叠。由非重叠计数器报告的内存可能不等于整体内存使用量，对于某些工作负载，差异可能很大。在许多情况下，有其他方法可以通过特定子系统的接口找到额外的内存，例如用于 TCP 内存分配的 /proc/net/sockstat。

示例输出。你可能没有所有这些字段。

```
    > cat /proc/meminfo

    MemTotal:       32858820 kB
    MemFree:        21001236 kB
    MemAvailable:   27214312 kB
    Buffers:          581092 kB
    Cached:          5587612 kB
    SwapCached:            0 kB
    Active:          3237152 kB
    Inactive:        7586256 kB
    Active(anon):      94064 kB
    Inactive(anon):  4570616 kB
    Active(file):    3143088 kB
    Inactive(file):  3015640 kB
    Unevictable:           0 kB
    Mlocked:               0 kB
    SwapTotal:             0 kB
    SwapFree:              0 kB
    Zswap:              1904 kB
    Zswapped:           7792 kB
    Dirty:                12 kB
    Writeback:             0 kB
    AnonPages:       4654780 kB
    Mapped:           266244 kB
    Shmem:              9976 kB
    KReclaimable:     517708 kB
    Slab:             660044 kB
    SReclaimable:     517708 kB
    SUnreclaim:       142336 kB
    KernelStack:       11168 kB
    PageTables:        20540 kB
    SecPageTables:         0 kB
    NFS_Unstable:          0 kB
    Bounce:                0 kB
    WritebackTmp:          0 kB
    CommitLimit:    16429408 kB
    Committed_AS:    7715148 kB
    VmallocTotal:   34359738367 kB
    VmallocUsed:       40444 kB
    VmallocChunk:          0 kB
    Percpu:            29312 kB
    EarlyMemtestBad:       0 kB
    HardwareCorrupted:     0 kB
    AnonHugePages:   4149248 kB
    ShmemHugePages:        0 kB
    ShmemPmdMapped:        0 kB
    FileHugePages:         0 kB
    FilePmdMapped:         0 kB
    CmaTotal:              0 kB
    CmaFree:               0 kB
    Unaccepted:            0 kB
    Balloon:               0 kB
    GPUActive:             0 kB
    GPUReclaim:            0 kB
    HugePages_Total:       0
    HugePages_Free:        0
    HugePages_Rsvd:        0
    HugePages_Surp:        0
    Hugepagesize:       2048 kB
    Hugetlb:               0 kB
    DirectMap4k:      401152 kB
    DirectMap2M:    10008576 kB
    DirectMap1G:    24117248 kB
```

MemTotal
              总可用 RAM（即物理 RAM 减去少量保留位和内核二进制代码）
MemFree
              总空闲 RAM。在 highmem 系统上，为 LowFree+HighFree 之和
MemAvailable
              在不交换的情况下，可用于启动新应用程序的内存量估计。根据 MemFree、
              SReclaimable、文件 LRU 列表的大小，以及每个 zone 的低
              水位线计算。
              该估计考虑了系统需要一些页缓存才能良好运行，并且由于有项目
              正在使用，并非所有可回收的 slab 都可回收。这些因素的
              影响会因系统而异。
Buffers
              原始磁盘块的相对临时存储，不应变得非常大（约 20MB 左右）
Cached
              从磁盘读取的文件（页缓存）以及 tmpfs 和 shmem 的内存缓存。
              不包括 SwapCached。
SwapCached
              曾经被换出、又被换入但仍在交换文件中的内存（如果内存不足，
              它不需要再次被换出，因为它已经在交换文件中。这节省了 I/O）
Active
              最近使用过的内存，通常除非绝对必要否则不被回收。
Inactive
              最近较少使用的内存。它更适合被回收用于其他目的
Unevictable
              为无法回收的用户空间分配的内存，例如 mlocked 页、ramfs 后端页、
              secret memfd 页等。
Mlocked
              用 mlock() 锁定的内存。
HighTotal, HighFree
              Highmem 是物理内存中 ~860MB 以上的所有内存。
              Highmem 区域供用户空间程序或页缓存使用。内核必须使用技巧来
              访问此内存，使其访问比 lowmem 慢。
LowTotal, LowFree
              Lowmem 是可以用于 highmem 可使用的所有用途的内存，但它也可供
              内核用于自身的数据结构。除许多其他用途外，Slab 中的所有内容
              都在这里分配。当你用尽 lowmem 时，会发生糟糕的事情。
SwapTotal
              可用交换空间的总量
SwapFree
              已从 RAM 中逐出、暂时位于磁盘上的内存
Zswap
              zswap 后端消耗的内存（压缩后的大小）
Zswapped
              存储在 zswap 中的匿名内存量（原始大小）
Dirty
              正在等待写回磁盘的内存
Writeback
              正在被主动写回磁盘的内存
AnonPages
              映射到用户空间页表的、无文件支撑的页。注意某些内核配置可能将
              较大分配（例如 THP）的所有页视为"已映射"，一旦单个页被映射。
Mapped
              已被 mmapped 的文件，例如库。注意某些内核配置可能将较大分配
              （例如 THP）的所有页视为"已映射"，一旦单个页被映射。
Shmem
              共享内存（shmem）和 tmpfs 使用的总内存
KReclaimable
              内核在内存压力下会尝试回收的内核分配。包括 SReclaimable（见下），
              以及其他带有 shrinker 的直接分配。
Slab
              内核内数据结构缓存
SReclaimable
              Slab 中可能被回收的部分，例如缓存
SUnreclaim
              Slab 中在内存压力下无法回收的部分
KernelStack
              所有任务的核栈消耗的内存
PageTables
              用户空间页表消耗的内存
SecPageTables
              次级页表消耗的内存，目前包括 x86 和 arm64 上的 KVM mmu 和 IOMMU 分配。
NFS_Unstable
              始终为零。以前用于计数已写入服务器但尚未提交到稳定存储的页。
Bounce
              始终为零。以前用于块设备"bounce buffers"的内存。
WritebackTmp
              始终为零。以前用于 FUSE 临时写回缓冲区的内存。
CommitLimit
              基于 overcommit 比率（'vm.overcommit_ratio'），这是系统上当前
              可用于分配的内存总量。只有在启用了严格 overcommit 记账时
              （'vm.overcommit_memory' 中的模式 2），才会遵守此限制。

```
                CommitLimit = ([总 RAM 页数] - [总 huge TLB 页数]) *
                               overcommit_ratio / 100 + [总交换页数]

              例如，在一个具有 1G 物理 RAM 和 7G 交换空间、且 `vm.overcommit_ratio`
              为 30 的系统上，将得到 7.3G 的 CommitLimit。

              更多详细信息，请参阅 mm/overcommit-accounting 中的 overcommit 文档。
```
Committed_AS
              系统上当前已分配的内存量。已提交的内存是所有进程已分配的
              内存之和，即使它们尚未"使用"。一个 malloc() 了 1G 内存但
              只触及其中 300M 的进程，会显示为使用了 1G。这 1G 是已被 VM
              "提交"的内存，可以由分配应用程序随时使用。在系统上启用了严格
              overcommit（'vm.overcommit_memory' 中的模式 2）时，超过
              CommitLimit（详见上文）的分配将不被允许。如果需要保证进程在
              成功分配内存后不会因缺少内存而失败，这很有用。
VmallocTotal
              vmalloc 虚拟地址空间的总大小
VmallocUsed
              已使用的 vmalloc 区域大小
VmallocChunk
              空闲的 vmalloc 区域中最大的连续块
Percpu
              分配给 percpu 分配器用于支撑 percpu 分配的内存。此统计不包括
              元数据的开销。
EarlyMemtestBad
              以 kB 为单位的、被早期 memtest 识别为损坏的 RAM/内存量。如果未运行
              memtest，则根本不会显示此字段。大小永远不会向下舍入到 0 kB。
              这意味着如果报告为 0 kB，你可以放心地假设至少进行了一次 memtest
              扫描，且没有任何一次扫描发现单个损坏的 RAM 字节。
HardwareCorrupted
              内核识别为已损坏的 RAM/内存量（KB）。
AnonHugePages
              映射到用户空间页表的无文件支撑的大页
ShmemHugePages
              由共享内存（shmem）和 tmpfs 用大页分配的内存
ShmemPmdMapped
              用大页映射到用户空间的共享内存
FileHugePages
              文件系统数据（页缓存）用大页分配的内存
FilePmdMapped
              用大页映射到用户空间的页缓存
CmaTotal
              为连续内存分配器（CMA）保留的内存
CmaFree
              CMA 保留区中剩余的空闲内存
Unaccepted
              尚未被 guest 接受的内存
Balloon
              由 VM Balloon 驱动返回给 Host 的内存
GPUActive
              分配给活动 GPU 对象的系统内存
GPUReclaim
              存储在 GPU 池中供复用的系统内存。此内存不计入 GPUActive。它是
              因具有非标准页表属性（如 WC 或 UC）而保留在复用池中的 shrinker
              可回收内存。
HugePages_Total, HugePages_Free, HugePages_Rsvd, HugePages_Surp, Hugepagesize, Hugetlb
              见 Documentation/admin-guide/mm/hugetlbpage.rst。
DirectMap4k, DirectMap2M, DirectMap1G
              内核 RAM 恒等映射中使用的页表大小细分

#### vmallocinfo


提供关于 vmalloced/vmaped 区域的信息。每个区域一行，包含该区域的虚拟地址范围、字节大小、创建者的调用者信息，以及取决于区域类型的可选信息：

 ==========  ===================================================
 pages=nr    页数量
 phys=addr   如果指定了物理地址
 ioremap     I/O 映射（ioremap() 及其相关函数）
 vmalloc     vmalloc() 区域
 vmap        vmap() 映射的页
 user        VM_USERMAP 区域
 vpages     页指针的缓冲区被 vmalloced（巨大区域）
 N<node>=nr  （仅 NUMA 内核上）
             在内存节点 <node> 上分配的页数量
 ==========  ===================================================

```

    > cat /proc/vmallocinfo
    0xffffc20000000000-0xffffc20000201000 2101248 alloc_large_system_hash+0x204 ...
    /0x2c0 pages=512 vmalloc N0=128 N1=128 N2=128 N3=128
    0xffffc20000201000-0xffffc20000302000 1052672 alloc_large_system_hash+0x204 ...
    /0x2c0 pages=256 vmalloc N0=64 N1=64 N2=64 N3=64
    0xffffc20000302000-0xffffc20000304000    8192 acpi_tb_verify_table+0x21/0x4f...
    phys=7fee8000 ioremap
    0xffffc20000304000-0xffffc20000307000   12288 acpi_tb_verify_table+0x21/0x4f...
    phys=7fee7000 ioremap
    0xffffc2000031d000-0xffffc2000031f000    8192 init_vdso_vars+0x112/0x210
    0xffffc2000031f000-0xffffc2000032b000   49152 cramfs_uncompress_init+0x2e ...
    /0x80 pages=11 vmalloc N0=3 N1=3 N2=2 N3=3
    0xffffc2000033a000-0xffffc2000033d000   12288 sys_swapon+0x640/0xac0      ...
    pages=2 vmalloc N1=2
    0xffffc20000347000-0xffffc2000034c000   20480 xt_alloc_table_info+0xfe ...
    /0x130 [x_tables] pages=4 vmalloc N0=4
    0xffffffffa0000000-0xffffffffa000f000   61440 sys_init_module+0xc27/0x1d00 ...
    pages=14 vmalloc N2=14
    0xffffffffa000f000-0xffffffffa0014000   20480 sys_init_module+0xc27/0x1d00 ...
    pages=4 vmalloc N1=4
    0xffffffffa0014000-0xffffffffa0017000   12288 sys_init_module+0xc27/0x1d00 ...
    pages=2 vmalloc N1=2
    0xffffffffa0017000-0xffffffffa0022000   45056 sys_init_module+0xc27/0x1d00 ...
    pages=10 vmalloc N0=10

```

#### softirqs


提供自启动以来每个 CPU 服务的 softirq 处理程序计数。

```

    > cat /proc/softirqs
		  CPU0       CPU1       CPU2       CPU3
	HI:          0          0          0          0
    TIMER:       27166      27120      27097      27034
    NET_TX:          0          0          0         17
    NET_RX:         42          0          0         39
    BLOCK:           0          0        107       1121
    TASKLET:         0          0          0        290
    SCHED:       27035      26983      26971      26746
    HRTIMER:         0          0          0          0
	RCU:      1678       1769       2178       2250
```

### 1.3 /proc/net 中的网络信息


子目录 /proc/net 遵循通常的模式。表 1-8 显示了如果你配置内核支持 IP 版本 6 所获得的额外值。表 1-9 列出了这些文件及其含义。



 ========== =====================================================
 文件      内容
 ========== =====================================================
 udp6       UDP 套接字（IPv6）
 tcp6       TCP 套接字（IPv6）
 raw6       原始设备统计（IPv6）
 igmp6      本机已加入的 IP 组播地址（IPv6）
 if_inet6   IPv6 接口地址列表
 ipv6_route 内核 IPv6 路由表
 rt6_stats  全局 IPv6 路由表统计
 sockstat6  套接字统计（IPv6）
 snmp6      Snmp 数据（IPv6）
 ========== =====================================================


 ============= ================================================================
 文件         内容
 ============= ================================================================
 arp           内核 ARP 表
 dev           带统计的网络设备
 dev_mcast     设备正在监听的二层组播组
                （接口索引、标签、引用计数、绑定地址数）。
 dev_stat      网络设备状态
 ip_fwchains   防火墙链链接
 ip_fwnames    防火墙链名称
 ip_masq       包含伪装表的目录
 ip_masquerade 主要伪装表
 netstat       网络统计
 raw           原始设备统计
 route         内核路由表
 rpc           包含 rpc 信息的目录
 rt_cache      路由缓存
 snmp          SNMP 数据
 sockstat      套接字统计
 softnet_stat  在线 CPU 的每 CPU 入站数据包队列统计
 tcp           TCP 套接字
 udp           UDP 套接字
 unix          UNIX 域套接字
 wireless      无线接口数据（Wavelan 等）
 igmp          本机已加入的 IP 组播地址
 psched        全局数据包调度器参数。
 netlink       PF_NETLINK 套接字列表
 ip_mr_vifs    组播虚拟接口列表
 ip_mr_cache   组播路由缓存列表
 ============= ================================================================

你可以利用此信息查看系统中可用的网络设备

```
  > cat /proc/net/dev
  Inter-|Receive                                                   |[...
   face |bytes    packets errs drop fifo frame compressed multicast|[...
      lo:  908188   5596     0    0    0     0          0         0 [...
    ppp0:15475140  20721   410    0    0   410          0         0 [...
    eth0:  614530   7085     0    0    0     0          0         1 [...

  ...] Transmit
  ...] bytes    packets errs drop fifo colls carrier compressed
  ...]  908188     5596    0    0    0     0       0          0
  ...] 1375103    17405    0    0    0     0       0          0
  ...] 1703981     5535    0    0    0     3       0          0
```

此外，每个 Channel Bond 接口都有自己的目录。例如，bond0 设备将有一个名为 /proc/net/bond0/ 的目录。它将包含特定于该 bond 的信息，例如 bond 的当前从设备、从设备的链路状态，以及从设备的链路失败次数。

### 1.4 SCSI 信息


如果你的系统中有 SCSI 或 ATA 主机适配器，你会在 /proc/scsi 中找到以该适配器驱动命名的子目录。

```
  >cat /proc/scsi/scsi
  Attached devices:
  Host: scsi0 Channel: 00 Id: 00 Lun: 00
    Vendor: IBM      Model: DGHS09U          Rev: 03E0
    Type:   Direct-Access                    ANSI SCSI revision: 03
  Host: scsi0 Channel: 00 Id: 06 Lun: 00
    Vendor: PIONEER  Model: CD-ROM DR-U06S   Rev: 1.04
    Type:   CD-ROM                           ANSI SCSI revision: 02


```

以驱动命名的目录针对系统中找到的每个适配器有一个文件。这些文件包含关于控制器的信息，包括所使用的 IRQ 和 IO 地址范围。显示的信息量取决于你使用的适配器。示例显示了 Adaptec 的输出

```
  > cat /proc/scsi/aic7xxx/0

  Adaptec AIC7xxx driver version: 5.1.19/3.2.4
  Compile Options:
    TCQ Enabled By Default : Disabled
    AIC7XXX_PROC_STATS     : Disabled
    AIC7XXX_RESET_DELAY    : 5
  Adapter Configuration:
             SCSI Adapter: Adaptec AHA-294X Ultra SCSI host adapter
                             Ultra Wide Controller
      PCI MMAPed I/O Base: 0xeb001000
   Adapter SEEPROM Config: SEEPROM found and used.
        Adaptec SCSI BIOS: Enabled
                      IRQ: 10
                     SCBs: Active 0, Max Active 2,
                           Allocated 15, HW 16, Page 255
               Interrupts: 160328
        BIOS Control Word: 0x18b6
     Adapter Control Word: 0x005b
     Extended Translation: Enabled
  Disconnect Enable Flags: 0xffff
       Ultra Enable Flags: 0x0001
   Tag Queue Enable Flags: 0x0000
  Ordered Queue Tag Flags: 0x0000
  Default Tag Queue Depth: 8
      Tagged Queue By Device array for aic7xxx host instance 0:
        {255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255}
      Actual queue depth per device for aic7xxx host instance 0:
        {1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1}
  Statistics:
  (scsi0:0:0:0)
    Device using Wide/Sync transfers at 40.0 MByte/sec, offset 8
    Transinfo settings: current(12/8/1/0), goal(12/8/1/0), user(12/15/1/0)
    Total transfers 160151 (74577 reads and 85574 writes)
  (scsi0:0:6:0)
    Device using Narrow/Sync transfers at 5.0 MByte/sec, offset 15
    Transinfo settings: current(50/15/0/0), goal(50/15/0/0), user(50/15/0/0)
    Total transfers 0 (0 reads and 0 writes)


```

### 1.5 /proc/parport 中的并行端口信息


目录 /proc/parport 包含关于你系统并行端口的信息。它为每个端口有一个以端口号（0,1,2,...）命名的子目录。

这些目录包含表 1-10 中所示的四个文件。



 ========= ====================================================================
 文件      内容
 ========= ====================================================================
 autoprobe 已获取的任意 IEEE-1284 设备 ID 信息。
 devices   使用该端口的设备驱动列表。当前正在使用该端口的设备名旁边会出现一个 +（它可能
           不出现在任何设备名旁）。
 hardware  并行端口的基地址、IRQ 线和 DMA 通道。
 irq       parport 用于该端口的 IRQ。它在一个单独的文件中，允许你通过写入新值
           （IRQ 号或 none）来更改它。
 ========= ====================================================================

### 1.6 /proc/tty 中的 TTY 信息


关于可用和实际使用的 tty 的信息可以在目录 /proc/tty 中找到。你会在这个目录中找到驱动和线路规程（line discipline）的条目，如表 1-11 所示。



 ============= ==============================================
 文件         内容
 ============= ==============================================
 drivers      驱动及其使用情况的列表
 ldiscs       已注册的线路规程
 driver/serial 单个 tty 线路的使用统计和状态
 ============= ==============================================

要查看当前正在使用哪些 tty，你可以直接查看文件

```
  > cat /proc/tty/drivers
  pty_slave            /dev/pts      136   0-255 pty:slave
  pty_master           /dev/ptm      128   0-255 pty:master
  pty_slave            /dev/ttyp       3   0-255 pty:slave
  pty_master           /dev/pty        2   0-255 pty:master
  serial               /dev/cua        5   64-67 serial:callout
  serial               /dev/ttyS       4   64-67 serial
  /dev/tty0            /dev/tty0       4       0 system:vtmaster
  /dev/ptmx            /dev/ptmx       5       2 system
  /dev/console         /dev/console    5       1 system:console
  /dev/tty             /dev/tty        5       0 system:/dev/tty
  unknown              /dev/tty        4    1-63 console


```
### 1.7 /proc/stat 中的杂项内核统计


关于内核活动的各种信息可以在 /proc/stat 文件中获取。该文件中报告的所有数字都是聚合值

```
  > cat /proc/stat
  cpu  237902850 368826709 106375398 1873517540 1135548 0 14507935 0 0 0
  cpu0 60045249 91891769 26331539 468411416 495718 0 5739640 0 0 0
  cpu1 59746288 91759249 26609887 468860630 312281 0 4384817 0 0 0
  cpu2 59489247 92985423 26904446 467808813 171668 0 2268998 0 0 0
  cpu3 58622065 92190267 26529524 468436680 155879 0 2114478 0 0 0
  intr 8688370575 8 3373 0 0 0 0 0 0 1 40791 0 0 353317 0 0 0 0 224789828 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 190974333 41958554 123983334 43 0 224593 0 0 0 <more 0's deleted>
  ctxt 22848221062
  btime 1605316999
  processes 746787147
  procs_running 2
  procs_blocked 0
  softirq 12121874454 100099120 3938138295 127375644 2795979 187870761 0 173808342 3072582055 52608 224184354
```

最开始的 "cpu" 行聚合了所有其他 "cpuN" 行中的数字。这些数字标识了 CPU 执行不同类型工作所花费的时间量。时间单位是 USER_HZ（通常是百分之一秒）。各列的含义从左到右如下：

- user：在用户模式下执行普通进程
- nice：在用户模式下执行被 nice 调整的进程
- system：在内核模式下执行的进程
- idle：空闲（twiddling thumbs）
- iowait：一言以蔽之，iowait 表示等待 I/O 完成。但有几个问题：

  1. CPU 不会等待 I/O 完成，iowait 是一个任务等待 I/O 完成的时间。当 CPU 因未完成的任务 I/O 而进入空闲状态时，另一个任务将被调度到该 CPU 上。
  2. 在多核 CPU 中，等待 I/O 完成的任务不在任何 CPU 上运行，因此每个 CPU 的 iowait 难以计算。
  3. 在某些情况下，/proc/stat 中 iowait 字段的值会减少。

  因此，从 /proc/stat 读取 iowait 并不可靠。
- irq：服务中断
- softirq：服务软中断
- steal：非自愿等待
- guest：运行普通 guest
- guest_nice：运行被 nice 调整的 guest

"intr" 行给出了自启动以来服务的、针对每个可能系统中断的中断计数。第一列是所有被服务中断的总数，包括未编号的架构特定中断；其后的每一列是该特定编号中断的总数。未编号的中断不会显示，只被汇总到总数中。

"ctxt" 行给出了跨所有 CPU 的上下文切换总数。

"btime" 行给出了系统启动的时间，以自 Unix 纪元以来的秒数表示。

"processes" 行给出了已创建的进程和线程数，包括（但不限于）通过 fork() 和 clone() 系统调用创建的那些。

"procs_running" 行给出了正在运行或准备运行的线程总数（即可运行线程总数）。

"procs_blocked" 行给出了当前被阻塞、等待 I/O 完成的进程数。

"softirq" 行给出了自启动以来服务的、针对每个可能系统 softirq 的 softirq 计数。第一列是所有被服务 softirq 的总数；其后的每一列是该特定 softirq 的总数。


### 1.8 Ext4 文件系统参数


关于已挂载 ext4 文件系统的信息可以在 /proc/fs/ext4 中找到。每个已挂载的文件系统会在 /proc/fs/ext4 下有一个基于其设备名的目录（即 /proc/fs/ext4/hdc 或 /proc/fs/ext4/sda9 或 /proc/fs/ext4/dm-0）。每个每设备目录下的文件如表 1-12 所示。



 ==============  ==========================================================
 文件           内容
 mb_groups       多块分配器空闲块 buddy 缓存的详细信息
 ==============  ==========================================================

### 1.9 /proc/consoles


显示已注册的系统控制台线路。

要查看当前用于系统控制台的字符设备线路：

```
  > cat /proc/consoles
  tty0                 -WU (ECp)       4:7
  ttyS0                -W- (Ep)        4:64
```

各列如下：

+--------------------+-------------------------------------------------------+
| device             | 设备名称                                            |
+====================+=======================================================+
| operations         | * R = 可以进行读操作                                 |
|                    | * W = 可以进行写操作                                 |
|                    | * U = 可以进行解除空白（unblank）                    |
+--------------------+-------------------------------------------------------+
| flags              | * E = 已启用                                          |
|                    | * C = 它是首选控制台                                  |
|                    | * B = 它是主引导控制台                                |
|                    | * p = 它用于 printk 缓冲区                            |
|                    | * b = 它不是 TTY 而是 Braille 设备                    |
|                    | * a = 在 cpu 离线时安全使用                           |
+--------------------+-------------------------------------------------------+
| major:minor        | 设备的主设备号和次设备号，以冒号分隔                  |
+--------------------+-------------------------------------------------------+

### 小结


/proc 文件系统提供关于运行系统的信息。它不仅允许访问进程数据，还允许你通过读取层级结构中的文件来请求内核状态。

/proc 的目录结构反映了信息的类型，并使查找特定数据的位置变得容易（即便不是显而易见）。

## 第 2 章：修改系统参数


### 本章内容


- 通过写入 /proc/sys 中的文件来修改内核参数
- 探索修改特定参数的文件
- 回顾 /proc/sys 文件树

------------------------------------------------------------------------------

/proc 中非常有趣的一部分是目录 /proc/sys。它不仅是信息的来源，还允许你更改内核中的参数。尝试此操作时要非常小心。你可以优化你的系统，但也可能使它崩溃。绝不要在生产系统上更改内核参数。搭建一台开发机器并进行测试，以确保一切按你想要的方式工作。一旦出错，你可能别无选择，只能重启机器。

要更改一个值，只需将新值 echo 到文件中。你需要是 root 才能这样做。你可以创建自己的引导脚本，在系统每次启动时执行此操作。

/proc/sys 中的文件可用于微调和监视 Linux 内核运行中的各种和一般事务。由于某些文件可能会不经意地扰乱你的系统，在实际进行调整之前，建议同时阅读文档和源代码。无论如何，写入这些文件中的任何文件时都要非常小心。/proc 中的条目在 2.1.* 和 2.2 内核之间可能略有变化，因此如有任何疑问，请查阅 linux/Documentation 目录中的内核文档。本章大量基于 2.2 之前内核中包含的文档，并在 Linux 内核 2.2.1 版本中成为其一部分。

请参阅：Documentation/admin-guide/sysctl/ 目录以获取这些条目的描述。

### 小结


内核行为的某些方面可以在运行时修改，无需重新编译内核，甚至无需重启系统。/proc/sys 树中的文件不仅可以读取，还可以修改。你可以使用 echo 命令将值写入这些文件，从而更改内核的默认设置。


## 第 3 章：每进程参数


### 3.1 /proc/<pid>/oom_adj & /proc/<pid>/oom_score_adj - 调整 oom-killer 分数


这些文件可用于调整用于选择内存不足（oom）条件下哪个进程被杀死的坏度（badness）启发式。

坏度启发式为每个候选任务分配一个从 0（从不杀死）到 1000（总是杀死）的值，以确定哪个进程是目标。这些单位大致是基于对其当前内存和交换使用量的估计，该进程可能从中分配的允许内存范围上的一个比例。例如，如果一个任务正在使用所有允许的内存，它的坏度分数将是 1000。如果它正在使用其允许内存的一半，它的分数将是 500。

"允许"的内存量取决于调用 oom killer 的上下文。如果是因为分配给分配任务的 cpuset 的内存耗尽，允许内存表示该 cpuset 被分配的一组 mems。如果是因为 mempolicy 的节点耗尽，允许内存表示该 mempolicy 节点集。如果是因为达到了内存限制（或交换限制），允许内存就是配置的限制。最后，如果是因为整个系统内存不足，允许内存表示所有可分配资源。

/proc/<pid>/oom_score_adj 的值在用于确定要杀死哪个任务之前被加到坏度分数上。可接受的值范围从 -1000（OOM_SCORE_ADJ_MIN）到 +1000（OOM_SCORE_ADJ_MAX）。这允许用户空间通过总是偏好某个任务或完全禁用它来极化 oom 杀死的偏好。最低可能的值 -1000 相当于完全禁用该任务的 oom 杀死，因为它总是报告坏度分数为 0。

因此，用户空间定义每个任务要考虑的内存量非常简单。例如，设置 /proc/<pid>/oom_score_adj 值为 +500，大致相当于允许共享同一系统、cpuset、mempolicy 或内存控制器资源的其余任务多使用至少 50% 的内存。另一方面，值 -500 大致相当于从该任务的计分中扣除其允许内存的 50%。

为了与以前的内核向后兼容，/proc/<pid>/oom_adj 也可用于调整坏度分数。其可接受的值范围从 -16（OOM_ADJUST_MIN）到 +15（OOM_ADJUST_MAX），以及特殊值 -17（OOM_DISABLE）以完全禁用该任务的 oom 杀死。其值随 /proc/<pid>/oom_score_adj 线性缩放。

/proc/<pid>/oom_score_adj 的值不能降低到最后一个由 CAP_SYS_RESOURCE 进程设置的值以下。要降低到更低的值需要 CAP_SYS_RESOURCE。


### 3.2 /proc/<pid>/oom_score - 显示当前 oom-killer 分数


该文件可用于检查 oom-killer 对任何给定 <pid> 使用的当前分数。将它和 /proc/<pid>/oom_score_adj 一起使用，以调整在内存不足情况下应杀死哪个进程。

请注意，导出的值包含 oom_score_adj，因此它实际上在 [0,2000] 范围内。


### 3.3  /proc/<pid>/io - 显示 IO 统计字段


该文件包含每个运行进程的 IO 统计。

#### 示例


```
    test:/tmp # dd if=/dev/zero of=/tmp/test.dat &
    [1] 3828

    test:/tmp # cat /proc/3828/io
    rchar: 323934931
    wchar: 323929600
    syscr: 632687
    syscw: 632675
    read_bytes: 0
    write_bytes: 323932160
    cancelled_write_bytes: 0
```

#### 描述


##### rchar


I/O 计数器：读取的字符数
该任务导致从存储读取的字节数。这简单是该进程传递给 read() 和 pread() 的字节数之和。它包含像 tty IO 之类的内容，并且不受是否需要实际物理磁盘 IO 的影响（读取可能由页缓存满足）。


##### wchar


I/O 计数器：写入的字符数
该任务导致或将要导致写入磁盘的字节数。这里适用的注意事项与 rchar 类似。


##### syscr


I/O 计数器：读系统调用数
尝试统计读 I/O 操作的数量，即像 read() 和 pread() 这样的系统调用。


##### syscw


I/O 计数器：写系统调用数
尝试统计写 I/O 操作的数量，即像 write() 和 pwrite() 这样的系统调用。


##### read_bytes


I/O 计数器：读取的字节数
尝试统计该进程真正导致从存储层获取到的字节数。在 submit_bio() 级别完成，因此对于块设备支持的文件系统是准确的。<请在以后补充关于 NFS 和 CIFS 的状态>


##### write_bytes


I/O 计数器：写入的字节数
尝试统计该进程导致发送到存储层的字节数。这在页变脏时进行。


##### cancelled_write_bytes


这里最大的不准确之处是截断（truncate）。如果一个进程向一个文件写入 1MB，然后删除该文件，它实际上不会执行任何写回。但它会被记为导致了 1MB 的写入。
换句话说：该进程通过截断页缓存而导致未发生的字节数。一个任务也可能导致"负"的 IO。如果该任务截断了某些脏页缓存，另一个任务已被计入（在其 write_bytes 中）的某些 IO 将不会发生。我们_可以_从截断任务的 write_bytes 中减去该值，但这样做会导致信息丢失。



   在其当前的实现状态下，这在 32 位机器上有些存在竞态：如果进程 A 在进程 B 更新其中一个 64 位计数器时读取进程 B 的 /proc/pid/io，进程 A 可能会看到一个中间结果。


关于此的更多信息可以在 Documentation/accounting 中的 taskstats 文档中找到。

### 3.4 /proc/<pid>/coredump_filter - 核心转储过滤设置

当一个进程被转储时，只要核心文件的大小不受限制，所有匿名内存都会被写入核心文件。但有时我们不想转储某些内存段，例如巨大的共享内存或 DAX。相反，有时我们想将文件支持的内存段保存到核心文件中，而不仅仅是各个文件。

/proc/<pid>/coredump_filter 允许你自定义当 <pid> 进程被转储时将转储哪些内存段。coredump_filter 是一个内存类型的位掩码。如果位掩码的某一位被设置，则相应内存类型的内存段会被转储，否则不会转储。

支持以下 9 种内存类型：

  - （位 0）匿名私有内存
  - （位 1）匿名共享内存
  - （位 2）文件支持的私有内存
  - （位 3）文件支持的共享内存
  - （位 4）文件支持的私有内存区域中的 ELF 头页（仅当位 2 被清除时有效）
  - （位 5）hugetlb 私有内存
  - （位 6）hugetlb 共享内存
  - （位 7）DAX 私有内存
  - （位 8）DAX 共享内存

  注意，MMIO 页（如帧缓冲）永远不会被转储，而 vDSO 页无论位掩码状态如何总是被转储。

  注意位 0-4 不影响 hugetlb 或 DAX 内存。hugetlb 内存仅受位 5-6 影响，DAX 仅受位 7-8 影响。

coredump_filter 的默认值是 0x33；这意味着所有匿名内存段、ELF 头页和 hugetlb 私有内存都会被转储。

如果你不想转储附加到 pid 1234 的所有共享内存段：

```
  $ echo 0x31 > /proc/1234/coredump_filter
```

当创建一个新进程时，该进程从其父进程继承位掩码状态。在程序运行之前设置 coredump_filter 很有用。

```
  $ echo 0x7 > /proc/self/coredump_filter
  $ ./some_program
```

### 3.5	/proc/<pid>/mountinfo - 关于挂载的信息


```
    36 35 98:0 /mnt1 /mnt2 rw,noatime master:1 - ext3 /dev/root rw,errors=continue
    (1)(2)(3)   (4)   (5)      (6)     (n…m) (m+1)(m+2) (m+3)         (m+4)

    (1)   mount ID:        挂载的唯一标识符（umount 后可能被复用）
    (2)   parent ID:       父挂载的 ID（或挂载树顶端的自身 ID）
    (3)   major:minor:     文件系统上文件的 st_dev 值
    (4)   root:            文件系统中该挂载的根
    (5)   mount point:     相对于进程根目录的挂载点
    (6)   mount options:   每个挂载的选项
    (n…m) optional fields: 零个或多个 "tag[:value]" 形式的字段
    (m+1) separator:       可选字段结束的标记
    (m+2) filesystem type: "type[.subtype]" 形式的文件系统名
    (m+3) mount source:    文件系统特定信息或 "none"
    (m+4) super options:   每个超级块的选项
```

解析器应忽略所有无法识别的可选字段。目前可能的可选字段有：

================  ==============================================================
shared:X          mount 在 peer group X 中共享
master:X          mount 是 peer group X 的从属（slave）
propagate_from:X  该 mount 是 slave 并从 peer group X 接收传播 [#]_
unbindable        mount 不可绑定（unbindable）
================  ==============================================================

       X 是该 mount 的直接 master，或者如果在同一根下没有占主导的 peer
       group，则只出现 "master:X" 字段，而不出现 "propagate_from:X" 字段。

关于挂载传播的更多信息，请参阅：

  Documentation/filesystems/sharedsubtree.rst


### 3.6	/proc/<pid>/comm  & /proc/<pid>/task/<tid>/comm

这些文件提供了一种访问任务 comm 值的方法。它还允许某个任务设置它自身或其某个线程兄弟的 comm 值。与 cmdline 值相比，comm 值的大小受到限制，因此写入超过内核 TASK_COMM_LEN（当前为 16 个字符，包含 NUL 终止符）的内容会导致 comm 值被截断。


### 3.7	/proc/<pid>/task/<tid>/children - 关于任务子进程的信息

该文件提供了一种快速获取由 <pid>/<tid> 对所指任务的第一层子进程 pid 的方法。其格式为以空格分隔的 pid 流。

注意这里的"第一层"——如果一个子进程还有它自己的子进程，则不会列在这里；需要读取 /proc/<children-pid>/task/<tid>/children 来获取其后代。

由于该接口旨在快速且廉价，它不保证提供精确的结果，某些子进程可能会被跳过，特别是如果它们在打印出 pid 之后立即退出，因此在需要精确结果时，需要停止或冻结被检查进程。


### 3.8	/proc/<pid>/fdinfo/<fd> - 关于已打开文件的信息

该文件提供与已打开文件相关的信息。常规文件至少有四个字段——'pos'、'flags'、'mnt_id' 和 'ino'。'pos' 以十进制形式表示该已打开文件的当前偏移量 [详见 lseek(2)]，'flags' 表示文件创建时使用的八进制 O_xxx 掩码 [详见 open(2)]，'mnt_id' 表示包含该已打开文件的文件系统的挂载 ID [详见 3.5 /proc/<pid>/mountinfo]。'ino' 表示该文件的 inode 号。

```
	pos:	0
	flags:	0100002
	mnt_id:	19
	ino:	63107
```

```
    lock:       1: FLOCK  ADVISORY  WRITE 359 00:13:11691 0 EOF
```

像 eventfd、fsnotify、signalfd、epoll 这样的文件，在常规的 pos/flags 之外还提供与其所代表对象相关的附加信息。

#### Eventfd 文件


```
	pos:	0
	flags:	04002
	mnt_id:	9
	ino:	63107
	eventfd-count:	5a
```

其中 'eventfd-count' 是一个计数器的十六进制值。

#### Signalfd 文件


```
	pos:	0
	flags:	04002
	mnt_id:	9
	ino:	63107
	sigmask:	0000000000000200
```

其中 'sigmask' 是与该文件关联的 signal mask 的十六进制值。

#### Epoll 文件


```
	pos:	0
	flags:	02
	mnt_id:	9
	ino:	63107
	tfd:        5 events:       1d data: ffffffffffffffff pos:0 ino:61af sdev:7
```

其中 'tfd' 是十进制形式的目标文件描述符编号，'events' 是正在被监视的事件掩码，'data' 是与目标关联的数据 [详见 epoll(7)]。

'pos' 是目标文件当前偏移量的十进制形式 [见 lseek(2)]，'ino' 和 'sdev' 是目标文件所在位置的 inode 和设备号，均以十六进制格式表示。

#### Fsnotify 文件


```
	pos:	0
	flags:	02000000
	mnt_id:	9
	ino:	63107
	inotify wd:3 ino:9e7e sdev:800013 mask:800afce ignored_mask:0 fhandle-bytes:8 fhandle-type:1 f_handle:7e9e0000640d1b6d
```

其中 'wd' 是十进制形式的监视描述符，即目标文件描述符编号，'ino' 和 'sdev' 是目标文件所在的 inode 和设备号，'mask' 是事件掩码，均以十六进制形式 [详见 inotify(7)]。

如果内核在构建时启用了 exportfs 支持，则到目标文件的路径被编码为一个文件句柄。该文件句柄由三个字段 'fhandle-bytes'、'fhandle-type' 和 'f_handle' 提供，均为十六进制格式。

如果内核在没有 exportfs 支持的情况下构建，则不会打印出文件句柄。

如果尚未附加任何 inotify 标记，则 'inotify' 行会被省略。

```
	pos:	0
	flags:	02
	mnt_id:	9
	ino:	63107
	fanotify flags:10 event-flags:0
	fanotify mnt_id:12 mflags:40 mask:38 ignored_mask:40000003
	fanotify ino:4f969 sdev:800013 mflags:0 mask:3b ignored_mask:40000000 fhandle-bytes:8 fhandle-type:1 f_handle:69f90400c275b5b4
```

其中 fanotify 的 'flags' 和 'event-flags' 是 fanotify_init 调用中使用的值，'mnt_id' 是挂载点标识符，'mflags' 是与标记关联的、与事件掩码分开跟踪的 flags 值。'ino' 和 'sdev' 是目标 inode 和设备号，'mask' 是事件掩码，'ignored_mask' 是要被忽略的事件掩码。所有均为十六进制格式。引入 'mflags'、'mask' 和 'ignored_mask' 提供了关于 fanotify_mark 调用中使用的 flags 和掩码的信息 [详见 fsnotify 手册页]。

虽然前三行是强制的且始终会打印，但其余部分是可选的，如果没有创建任何标记则可能会被省略。

#### Timerfd 文件


```
	pos:	0
	flags:	02
	mnt_id:	9
	ino:	63107
	clockid: 0
	ticks: 0
	settime flags: 01
	it_value: (0, 49406829)
	it_interval: (1, 0)
```

其中 'clockid' 是时钟类型，'ticks' 是已发生的定时器到期次数 [详见 timerfd_create(2)]。'settime flags' 是用于设置定时器的八进制形式 flags [详见 timerfd_settime(2)]。'it_value' 是距离定时器到期的剩余时间。'it_interval' 是定时器的间隔。注意，定时器可能使用 TIMER_ABSTIME 选项设置，这会显示在 'settime flags' 中，但 'it_value' 仍然显示定时器的剩余时间。

#### DMA Buffer 文件


```
	pos:	0
	flags:	04002
	mnt_id:	9
	ino:	63107
	size:   32768
	count:  2
	exp_name:  system-heap
```

其中 'size' 是 DMA buffer 的大小（以字节为单位）。'count' 是 DMA buffer 文件的文件计数。'exp_name' 是 DMA buffer 导出者的名称。

#### VFIO Device 文件


```
	pos:    0
	flags:  02000002
	mnt_id: 17
	ino:    5122
	vfio-device-syspath: /sys/devices/pci0000:e0/0000:e0:01.1/0000:e1:00.0/0000:e2:05.0/0000:e8:00.0
```

其中 'vfio-device-syspath' 是与 VFIO 设备文件对应的 sysfs 路径。

### 3.9	/proc/<pid>/map_files - 关于内存映射文件的信息

该目录包含表示内存映射文件的符号链接。

```
     | lr-------- 1 root root 64 Jan 27 11:24 333c600000-333c620000 -> /usr/lib64/ld-2.18.so
     | lr-------- 1 root root 64 Jan 27 11:24 333c81f000-333c820000 -> /usr/lib64/ld-2.18.so
     | lr-------- 1 root root 64 Jan 27 11:24 333c820000-333c821000 -> /usr/lib64/ld-2.18.so
     | ...
     | lr-------- 1 root root 64 Jan 27 11:24 35d0421000-35d0422000 -> /usr/lib64/libselinux.so.1
     | lr-------- 1 root root 64 Jan 27 11:24 400000-41a000 -> /usr/bin/ls
```

链接的名称表示一个映射的虚拟内存边界，即 **vm_area_struct**：vm_start-vm_area_struct::vm_end。

map_files 的主要用途是以快速的方式获取一组内存映射文件，而无需解析 /proc/<pid>/maps 或 /proc/<pid>/smaps（这两者都包含更多的记录）。同时，可以从两个进程的文件列表中 open(2) 映射，并比较它们的 inode 号，以确定哪些匿名内存区域实际上是共享的。

### 3.10	/proc/<pid>/timerslack_ns - 任务 timerslack 值

该文件提供任务的 timerslack 值（以纳秒为单位）。该值指定了普通定时器可以被推迟的一段时间，以便将定时器合并，避免不必要的唤醒。

这允许调整任务的交互性与功耗之间的权衡。

向该文件写入 0 会将任务的 timerslack 设为默认值。

有效值范围为 0 - ULLONG_MAX。

要更改某任务的 timerslack_ns 值，设置该值的应用程序必须对该指定任务具有 PTRACE_MODE_ATTACH_FSCREDS 级别的权限。

### 3.11	/proc/<pid>/patch_state - Livepatch 补丁操作状态

当启用 CONFIG_LIVEPATCH 时，该文件显示该任务的补丁状态值。

值 '-1' 表示没有补丁处于转换（transition）状态。

值 '0' 表示有一个补丁处于转换状态且该任务未被打补丁。如果补丁正在被启用，则该任务尚未被打补丁。如果补丁正在被禁用，则该任务已经被取消补丁。

值 '1' 表示有一个补丁处于转换状态且该任务已被打补丁。如果补丁正在被启用，则该任务已经被打补丁。如果补丁正在被禁用，则该任务尚未被取消补丁。

### 3.12 /proc/<pid>/arch_status - 任务架构特定状态

当启用 CONFIG_PROC_PID_ARCH_STATUS 时，该文件显示该任务的架构特定状态。

#### 示例


```
 $ cat /proc/6753/arch_status
 AVX512_elapsed_ms:      8
```

#### 描述


#### x86 特定条目


##### AVX512_elapsed_ms


  如果机器支持 AVX512，该条目显示自上次记录 AVX512 使用以来经过的毫秒数。记录是在任务被调度出 CPU 时尽力进行的。这意味着该值取决于两个因素：

    1) 任务在 CPU 上未被调度出所花费的时间。在 CPU 隔离且只有一个可运行任务的情况下，这可能花费数秒。

    2) 自任务上次被调度出以来经过的时间。根据被调度出的原因（时间片耗尽、syscall ...），这可能是任意长的时间。

  因此，该值不能被视作精确且权威的信息。使用此信息的应用程序必须了解系统上的整体场景，以确定某个任务是否真的是 AVX512 用户。精确信息可以通过性能计数器获得。

  特殊值 '-1' 表示没有记录到 AVX512 使用，因此该任务不太可能是 AVX512 用户，但这也取决于工作负载和调度场景，也可能出现上述假阴性。

### 3.13 /proc/<pid>/fd - 指向打开文件的符号链接列表

该目录包含表示打开文件的符号链接。

```
  lr-x------ 1 root root 64 Sep 20 17:53 0 -> /dev/null
  l-wx------ 1 root root 64 Sep 20 17:53 1 -> /dev/null
  lrwx------ 1 root root 64 Sep 20 17:53 10 -> 'socket:[12539]'
  lrwx------ 1 root root 64 Sep 20 17:53 11 -> 'socket:[12540]'
  lrwx------ 1 root root 64 Sep 20 17:53 12 -> 'socket:[12542]'
```

进程打开文件的数量存储在 /proc/<pid>/fd 的 stat() 输出的 'size' 成员中，以便快速访问。


### 3.14 /proc/<pid>/ksm_stat - 关于进程 ksm 状态的信息

当启用 CONFIG_KSM 时，每个进程都有此文件，显示 ksm 合并状态的信息。

#### 示例


```
    / # cat /proc/self/ksm_stat
    ksm_rmap_items 0
    ksm_zero_pages 0
    ksm_merging_pages 0
    ksm_process_profit 0
    ksm_merge_any: no
    ksm_mergeable: no
```

#### 描述


##### ksm_rmap_items


ksm_rmap_item 结构的使用数量。ksm_rmap_item 结构存储虚拟地址的反向映射信息。KSM 会为该进程每个被 ksm 扫描的页生成一个 ksm_rmap_item。

##### ksm_zero_pages


当 /sys/kernel/mm/ksm/use_zero_pages 被启用时，它表示有多少个空页被 KSM 与内核零页合并。

##### ksm_merging_pages


它表示有多少个该进程的页参与了 KSM 合并（不包括 ksm_zero_pages）。它与 /proc/<pid>/ksm_merging_pages 所显示的内容相同。

##### ksm_process_profit


KSM 带来的收益（节省的字节数）。KSM 可以通过合并相同的页来节省内存，但也可能消耗额外的内存，因为它需要为每个被扫描的页生成一个 rmap_item 来保存其简要的 rmap 信息。其中一些页可能被合并，但有些在多次检查后仍可能无法合并，这些就是被消耗的无收益内存。

##### ksm_merge_any


它指定该进程的 'mm 是否已被 prctl() 加入 KSM 的候选列表，以及 KSM 扫描是否在进程级别被完全启用。

##### ksm_mergeable


它指定该进程的 mms 中是否有任何 VMA 当前适用于 KSM。

关于 KSM 的更多信息可以在 Documentation/admin-guide/mm/ksm.rst 中找到。


## 第 4 章：配置 procfs


### 4.1	挂载选项


支持以下挂载选项：

	=========	========================================================
	hidepid=	设置 /proc/<pid>/ 的访问模式。
	gid=		设置被授权了解进程信息的组。
	subset=		只显示 procfs 的指定子集。
	pidns=		指定该 procfs 使用的命名空间。
	=========	========================================================

hidepid=off 或 hidepid=0 表示经典模式——每个人都可以访问所有 /proc/<pid>/ 目录（默认）。

hidepid=noaccess 或 hidepid=1 表示用户只能访问其自身的 /proc/<pid>/ 目录，不能访问其他任何目录。像 cmdline、sched*、status 这样的敏感文件现在受到保护，防止其他用户访问。这使得他人无法得知是否有用户运行了特定程序（前提是程序没有通过其自身行为暴露自己）。作为额外的好处，由于 /proc/<pid>/cmdline 对其他用户不可访问，那些通过程序参数传递敏感信息的编写不良的程序现在也受到保护，防止本地窃听者。

hidepid=invisible 或 hidepid=2 表示在 hidepid=1 的基础上，所有 /proc/<pid>/ 对其他用户完全不可见。这并不意味着隐藏了是否存在具有特定 pid 值的进程这一事实（它可以通过其他方式得知，例如 "kill -0 $PID"），但它隐藏了进程的 uid 和 gid，否则可以通过 stat() /proc/<pid>/ 来得知。它极大地增加了入侵者收集正在运行进程信息的难度，例如某个守护进程是否以提权方式运行、其他用户是否运行了某些敏感程序、其他用户是否运行了任何程序等等。

hidepid=ptraceable 或 hidepid=4 表示 procfs 应只包含调用者可以 ptrace 的 /proc/<pid>/ 目录。

gid= 定义一个被授权了解进程信息的组，否则该信息会被 hidepid= 禁止。如果你使用像 identd 这样需要了解进程信息的守护进程，只需将 identd 加入该组。

subset=pid 隐藏 procfs 中所有与任务无关的最顶层文件和目录。

pidns= 指定一个 pid 命名空间（可以是类似 `/proc/$pid/ns/pid` 的字符串路径，也可以是使用 `FSCONFIG_SET_FD` 时的文件描述符），procfs 实例在转换 pid 时将使用该命名空间。默认情况下，procfs 将使用调用进程的活动 pid 命名空间。注意，现有 procfs 实例的 pid 命名空间无法被修改（尝试这样做会得到 `-EBUSY` 错误）。

## 第 5 章：文件系统行为


最初，在 pid 命名空间出现之前，procfs 是一个全局文件系统。这意味着系统中只有一个 procfs 实例。

当加入 pid 命名空间后，在每个 pid 命名空间中会挂载一个独立的 procfs 实例。因此，procfs 的挂载选项在所有挂载实例之间是全局的：

```
	# grep ^proc /proc/mounts
	proc /proc proc rw,relatime,hidepid=2 0 0

	# strace -e mount mount -o hidepid=1 -t proc proc /tmp/proc
	mount("proc", "/tmp/proc", "proc", 0, "hidepid=1") = 0
	+++ exited with 0 +++

	# grep ^proc /proc/mounts
	proc /proc proc rw,relatime,hidepid=2 0 0
	proc /tmp/proc proc rw,relatime,hidepid=2 0 0
```

只有在重新挂载 procfs 之后，挂载选项才会改变：

```
	# mount -o remount,hidepid=1 -t proc proc /tmp/proc

	# grep ^proc /proc/mounts
	proc /proc proc rw,relatime,hidepid=1 0 0
	proc /tmp/proc proc rw,relatime,hidepid=1 0 0
```

这种行为与其他文件系统的行为不同。

新的 procfs 行为更类似于其他文件系统。每次挂载 procfs 都会创建一个新的 procfs 实例。挂载选项只影响自身的 procfs 实例。这意味着可以拥有多个 procfs 实例：

```
	# mount -o hidepid=invisible -t proc proc /proc
	# mount -o hidepid=noaccess -t proc proc /tmp/proc
	# grep ^proc /proc/mounts
	proc /proc proc rw,relatime,hidepid=invisible 0 0
	proc /tmp/proc proc rw,relatime,hidepid=noaccess 0 0
```
