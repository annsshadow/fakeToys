
## Perf 事件与工具安

### 概述


Linux 的性能计数器（perf_events）[^1^]_ [^2^]_ , [^3^]_ 的使用可能带来相当大风险，导致被监控进程访问的敏感数据泄露。无论是在直接使perf_events 系统调用
API [^2^]_ 的场景中，还是通过 Perf 工具用户态实用程序（Perf）[^3^]_ , [^4^]_ 生成
的数据文件中，都可能发生数据泄露。该风险取决perf_events 性能监控单元（PMU[^2^]_ Perf 为性能分析所采集和暴露的数据的性质。所采集的系统与性能数据分为以下几类
1. 系统硬件与软件配置数据，例如：CPU 型号及其缓存配置、可用内存大小及   拓扑、所用的内核Perf 版本、性能监控设置（含实验时间、事件配置、Perf
   命令行参数等）
2. 用户态与内核模块路径及其加载地址与大小、进程与线程名及PID TID   所捕获硬件与软件事件的时间戳
3. 内核软件计数器的内容（例如上下文切换、缺页、CPU 迁移）、架构硬件性能
   计数器（PMC）[^8^]_ 以及机器特定寄存器（MSR）[^9^]_ —它们为系统中各类
   被监控部分（例如内存控制器（IMC）、互连（QPI/UPI）或外设（PCIe）uncore
   计数器）提供执行度量，而不直接归属于任何执行上下文状态
4. 架构执行上下文寄存器的内容（例如 x86_64 上的 RIP、RSP、RBP）、进程的用户
   态与内核态内存地址及数据，以及捕获此类别数据的各类架构 MSR 的内容
属于第四类的数据可能包含敏感进程数据。如果某些监控模式下PMU 捕获执行上下寄存器的值或进程内存中的数据，那么对此类监控模式的访问必须被正确排序和安保护。因此，perf_events 性能监控与可观测性操作是安全访问控制管理的对[^5^]_ 
### perf_events 访问控制


为了执行安全检查，Linux 的实现将进程分为两类 [^6^]_ ：a）特权进程（其有效用ID 0，即超级用户root），以及 b）非特权进程（其有效 UID 非零）。特权进绕过所有内核安全权限检查，因此 perf_events 性能监控对特权进程完全开放，不受
访问、范围与资源限制
非特权进程则要进行基于进程凭[^5^]_ （通常是：有效 UID、有GID 以及附加
组列表）的完整安全权限检查
Linux 将传统上与超级用户关联的特权划分为不同的单元，称capabilities [^6^]_ 它们可以在非特权用户的进程和文件上按线程独立地启用和禁用
启用CAP_PERFMON capability 的非特权进程，在 perf_events 性能监控与可观测操作方面被视为特权进程，从而绕过内核中**范围（scope* 权限检查。CAP_PERFMON
在内核中为性能监控与可观测性操作实现了最小特权原[^13^]_ （POSIX 1003.1e:
2.2.2.39），并提供了一种安全的系统性能监控与可观测性方法
出于向后兼容的考虑，对 perf_events 监控与可观测性操作的访问也对 CAP_SYS_ADMIN
特权进程开放，但相CAP_PERFMON capability，不建议CAP_SYS_ADMIN 用于安全
监控与可观测性场景。如果某进程使用 perf_events 系统调用 API 的系统审计记[^14^]_
同时包含获取 CAP_PERFMON CAP_SYS_ADMIN 两种 capability 的拒绝记录，则建单独为该进程提供 CAP_PERFMON capability，作为解决性能监控与可观测性使用相双重访问拒绝日志的首选安全方法
Linux v5.9 之前，使perf_events 系统调用的非特权进程还须接受
PTRACE_MODE_READ_REALCREDS ptrace 访问模式检[^7^]_ ，其结果决定是否允许监控因此，提供了 CAP_SYS_PTRACE capability 的非特权进程实际上能够通过该检查。从
Linux v5.9 起，不再需CAP_SYS_PTRACE capability，只要为进程提供 CAP_PERFMON
就足以进行性能监控与可观测性操作
授予非特权进程的其他 capability 可以有效启用对后续被监控进程或系统性能分析所需
额外数据的采集。例如，CAP_SYSLOG capability 允许/proc/kallsyms 文件读取
内核态内存地址
### 特权 Perf 用户

capabilities 机制、特capability-dumb 文件 [^6^]_ 、文件系ACL [^10^]_ 以及
sudo [^15^]_ 实用程序可用来创建专用的特权 Perf 用户组，这些用户被允许无限制执行性能监控与可观测性。可以采取以下步骤来创建这样的特Perf 用户组
1. 创建特权 Perf 用户perf_users，将 perf_users 组分配给 Perf 工具可执行文件，
   并限制系统中不在 perf_users 组内的其他用户访问该可执行文件：

```
   # groupadd perf_users
   # ls -alhF
   -rwxr-xr-x  2 root root  11M Oct 19 15:12 perf
   # chgrp perf_users perf
   # ls -alhF
   -rwxr-xr-x  2 root perf_users  11M Oct 19 15:12 perf
   # chmod o-rwx perf
   # ls -alhF
   -rwxr-x---  2 root perf_users  11M Oct 19 15:12 perf
```

2. Perf 工具可执行文件分配所需capabilities，并perf_users 组成员具   监控与可观测性特[^6^]_ 
```
   # setcap "cap_perfmon,cap_sys_ptrace,cap_syslog=ep" perf
   # setcap -v "cap_perfmon,cap_sys_ptrace,cap_syslog=ep" perf
   perf: OK
   # getcap perf
   perf = cap_sys_ptrace,cap_syslog,cap_perfmon+ep
```

如果安装libcap [^16^]_ 尚不支持 "cap_perfmon"，则改用 "38"，即
```
   # setcap "38,cap_ipc_lock,cap_sys_ptrace,cap_syslog=ep" perf
```

注意，对'perf top' 这类工具，你可能需要在组合中加'cap_ipc_lock'，或改用 'perf top -m N' 以减少其用于 perf 环形缓冲区的内存，详见下文的“内存分配一节
使用不支CAP_PERFMON libcap 会导cap_get_flag(caps, 38, CAP_EFFECTIVE,
&val) 失败，进而使默认事件变为 'cycles:u'，因此作为变通，请显式请'cycles'
事件，即
```
  # perf top -e cycles
```

以便仅带 CAP_PERFMON perf 二进制也能获得内核与用户样本
这样一来，perf_users 组成员便能够使用所配置 Perf 工具可执行文件的功能进行性能
监控与可观测性，该可执行文件在执行时会通过 perf_events 子系统的范围检查
如果无法Perf 工具可执行文件分配所需capabilities（例如文件系统以 nosuid
选项挂载，或文件系统不支持扩展属性），那么可以创capabilities 特权环境（自就是 shell）。该 shell 为内部进程提CAP_PERFMON 及其他所需 capabilities，从在该环境中无限制地进行性能监控与可观测性操作。仅 perf_users 组成员可通过 sudo
实用程序进入该环境。为创建这样的环境：

1. 创建使用 capsh 实用程序 [^16^]_ shell 脚本，将 CAP_PERFMON 及其他所需
   capabilities 放入 shell 进程的环capability 集（ambient capability set），
   在启SECBIT_NO_SETUID_FIXUP、SECBIT_NOROOT SECBIT_NO_CAP_AMBIENT_RAISE
   位后锁定进程安全位，然后将进程身份切换为该脚本的 sudo 调用者（其本质上应为
   perf_users 组成员）
```
   # ls -alh /usr/local/bin/perf.shell
   -rwxr-xr-x. 1 root root 83 Oct 13 23:57 /usr/local/bin/perf.shell
   # cat /usr/local/bin/perf.shell
   exec /usr/sbin/capsh --iab=^cap_perfmon --secbits=239 --user=$SUDO_USER -- -l
```

2. /etc/sudoers 文件中为 perf_users 组扩sudo 策略
```
   # grep perf_users /etc/sudoers
   %perf_users    ALL=/usr/local/bin/perf.shell
```

3. 检perf_users 组成员是否能够访问该特权 shell，并在内部进程的允许（permitted）   有效（effective）与环境（ambient）capability 集中启用CAP_PERFMON 及其   所需 capabilities
```
  $ id
  uid=1003(capsh_test) gid=1004(capsh_test) groups=1004(capsh_test),1000(perf_users) context=unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023
  $ sudo perf.shell
  [sudo] password for capsh_test:
  $ grep Cap /proc/self/status
  CapInh:        0000004000000000
  CapPrm:        0000004000000000
  CapEff:        0000004000000000
  CapBnd:        000000ffffffffff
  CapAmb:        0000004000000000
  $ capsh --decode=0000004000000000
  0x0000004000000000=cap_perfmon
```

这样一来，perf_users 组成员就能访问该特权环境，在其中使用CAP_PERFMON Linux
capability 管控的性能监控 API 的工具
这种特定的访问控制管理仅对以 CAP_SETPCAP、CAP_SETFCAP [^6^]_ capabilities 运行超级用户root 进程可用
### 非特权用

非特权进程的 perf_events **范围（scope* **访问（access* 控制perf_event_paranoid [^2^]_ 设置管控
-1:
     perf_events 性能监控不施加任**范围** **访问** 限制。在为存     性能数据分配内存缓冲区时，忽略每用户CPU perf_event_mlock_kb [^2^]_
     锁定限制。这是最不安全的模式，因为允许的监控 **范围** 被最大化，且对用     性能监控**资源** 不施加任perf_events 特定的限制
>=0:
     **范围** 包含每进程与系统范围的性能监控，但排除原始 tracepoint ftrace
     函数 tracepoint 监控。在用户态或内核态执行时发生CPU 与系统事件都可以
     被监控和捕获以供后续分析。会施加每用户每 CPU perf_event_mlock_kb 锁定
     限制，但拥有 CAP_IPC_LOCK [^6^]_ capability 的非特权进程会忽略该限制
>=1:
     **范围** 仅包含每进程性能监控，排除系统范围的性能监控。在用户态或内核     执行时发生的 CPU 与系统事件都可以被监控和捕获以供后续分析。会施加每用     CPU perf_event_mlock_kb 锁定限制，但拥有 CAP_IPC_LOCK capability      非特权进程会忽略该限制
>=2:
     **范围** 仅包含每进程性能监控。只有执行于用户态时发生CPU 与系统事     可以被监控和捕获以供后续分析。会施加每用户每 CPU perf_event_mlock_kb
     锁定限制，但拥有 CAP_IPC_LOCK capability 的非特权进程会忽略该限制
### 资源控制


打开的文件描述符
+++++++++++++++++++++

perf_events 系统调用 API [^2^]_ 为每个配置的 PMU 事件分配文件描述符。打开文件描述符是一项按进程核算的资源，RLIMIT_NOFILE [^11^]_ 限制（ulimit -n管控，该限制通常源自登录 shell 进程。当在大型服务器系统上为长事件列表配Perf
采集时，很容易触及此限制，从而阻止所需的监控配置。RLIMIT_NOFILE 限制可以按用修改 limits.conf 文件 [^12^]_ 的内容来提高。通常，一Perf 采样会话（perf
record）所需的打开 perf_event 文件描述符数量不少于被监控事件数乘以被监CPU 数
内存分配
+++++++++++++++++

用户进程可用于捕获性能监控数据的内存量perf_event_mlock_kb [^2^]_ 设置管控这一 perf_events 特定的资源设置定义了允许用户进程为了执行性能监控而进行映射的
整体CPU 内存上限。该设置本质上扩展了 RLIMIT_MEMLOCK [^11^]_ 限制，但仅针专为捕获被监控性能事件及相关数据而映射的内存区域
例如，如果一台机器有八个核心，且 perf_event_mlock_kb 限制设为 516 KiB，那么用进程可获得超RLIMIT_MEMLOCK 限制（ulimit -l）的 516 KiB * 8 = 4128 KiB 内存用于
perf_event mmap 缓冲区。特别地，这意味着如果用户想启动两个或更多性能监控进程就需要手动在监控进程之间分配可用4128 KiB，例如使Perf record 模式选项
--mmap-pages。否则，第一个启动的性能监控进程会分配掉全部可用4128 KiB，而其进程将因内存不足而无法继续
RLIMIT_MEMLOCK perf_event_mlock_kb 资源约束对拥CAP_IPC_LOCK capability 进程被忽略。因此，通过Perf 可执行文件提CAP_IPC_LOCK capability，可以为
perf_events/Perf 特权用户提供超出这些约束的内存，用于 perf_events/Perf 性能监控
目的
### 参考文

