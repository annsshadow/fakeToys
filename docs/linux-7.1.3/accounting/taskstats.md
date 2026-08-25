## 每任务统计接

Taskstats 是一个基netlink 的接口，用于将每任务和每进程的统计信息从内核发送到用户空间
Taskstats 的设计目标具有以下好处：

- 在任务生命周期内及其退出时高效地提供统计信- 为多个记账子系统提供统一的接- 可扩展，供未来的记账补丁使用

### 术语


"pid"tid" "task" 可互换使用，均指struct task_struct 定义的标Linux 任务。per-pid 统计per-task 统计相同
"tgid"process" "thread group" 可互换使用，均指共享 mm_struct 的任务，即传统的 Unix 进程。尽管使用了 tgid，但线程组组长（thread group leader）任务并不享受特殊处理——只要进程还拥有任何属于它的任务，就视为该进程仍然存活
### 用法


要在任务生命周期内获取统计信息，用户空间需打开一个单netlink 套接字（NETLINK_GENERIC 系列）并发送指pid tgid 的命令。响应中包含某任务的统计信息（若指定pid），或该进程所有任务统计信息的合计（若指定tgid）
要获取正在退出的任务的统计信息，用户空间监听器会发送一个注册命令并指定一cpumask。只要某个任务在 cpumask 中的某个 CPU 上退出，它的 per-pid 统计就会被发送给已注册的监听器。使cpumask 可以限制单个监听器接收到的数据，并有助于netlink 接口上进行流控，下文会详细说明
如果退出的任务是其线程组中最后一个退出的线程，还会向用户空间发送一条额外的记录，其中包per-tgid 统计。后者包含该线程组所有线程（过去和现在）per-pid 统计合计
getdelays.c 是一个简单的工具，用于演示如何使taskstats 接口报告延迟记账（delay accounting）统计信息。用户可以注cpumask、发送命令并处理响应、监per-tid/tgid 退出数据、将接收到的数据写入文件，并通过增大接收缓冲区大小来进行基本的流控
### 接口


用户-内核接口封装include/linux/taskstats.h 中
为避免本文档随着接口演进而过时，这里仅给出当前版本的概要。taskstats.h 始终优先于这里的描述
struct taskstats per-pid per-tgid 数据共用的记账结构。它带有版本号，并且可以被内核中加入的每个记账子系统扩展。其字段及其语义taskstats.h 文件中定义
用户空间与内核空间之间交换的数据是一条属NETLINK_GENERIC 系列、并使用 netlink 属性接口的 netlink 消息```

    +----------+- - -+-------------+-------------------+
    | nlmsghdr | Pad |  genlmsghdr | taskstats payload |
    +----------+- - -+-------------+-------------------+


```
taskstats 载荷是以下三种之一
1. 命令：从用户发往内核。获pid/tgid 数据的命令由一个类型为 TASKSTATS_CMD_ATTR_PID/TGID 的属性组成，其属性载荷中包含一u32 类型pid tgid。该 pid/tgid 表示用户空间想要其统计信息的任务/进程
   用于注册/注销对一CPU 退出数据关注度的命令由一个类型为 TASKSTATS_CMD_ATTR_REGISTER/DEREGISTER_CPUMASK 的属性组成，其属性载荷中包含一cpumask。cpumask 以一个逗号分隔CPU 范围组成ascii 字符串形式指定，例如，若要监听来CPU 1 的退出数据，cpumask 应为 "1-3,5,7-8"。如果用户空间在关闭监听套接字前忘记注销CPU 的的关注，内核会在一段时间后清理其关注集合。不过，出于效率考虑，建议显式地注销
2. 命令的响应：从内核发出，以响应来自用户空间的命令。载荷是一系列三种类型的属性：

   a) TASKSTATS_TYPE_AGGR_PID/TGID：不含载荷的属性，但表示接下来会跟随某pid/tgid 及其统计信息
   b) TASKSTATS_TYPE_PID/TGID：其载荷为正在返回统计信息的 pid/tgid
   c) TASKSTATS_TYPE_STATS：以 struct taskstats 作为载荷的属性。同一结构既用per-pid 也用per-tgid 统计
3. 任务退出时内核发送的新消息。载荷由以下类型的属性序列组成：

   a) TASKSTATS_TYPE_AGGR_PID：表示接下来两个属性将pid+stats
   b) TASKSTATS_TYPE_PID：包含退出任务的 pid
   c) TASKSTATS_TYPE_STATS：包含退出任务的 per-pid 统计
   d) TASKSTATS_TYPE_AGGR_TGID：表示接下来两个属性将tgid+stats
   e) TASKSTATS_TYPE_TGID：包含该任务所属进程的 tgid
   f) TASKSTATS_TYPE_STATS：包含退出任务所属进程的 per-tgid 统计


### per-tgid 统计


per-task 统计外，Taskstats 还提per-process 统计，因为资源管理通常以进程为粒度进行，而仅用户空间聚合任务统计既低效又可能不准确（由于缺乏原子性）
不过，在内核中同时维per-process per-task 统计会带来空间和时间开销。为解决这个问题，taskstats 代码把每个退出任务的统计累积到一个进程级的数据结构中。当进程的最后一个任务退出时，累积的进程级数据也会（连同 per-task 数据一起）发送给用户空间
当用户查询获per-tgid 数据时，组中所有其他存活线程的合计会被累加，并加到同一线程组此前已退出线程的累积总数上
### 扩展 taskstats


有两种方式可以扩taskstats 接口，以导出更多per-task/per-process 统计，随着未来内核加入收集这些统计的补丁：

1. 在现struct taskstats 的末尾添加更多字段。结构内部的版本号保证了向后兼容性。用户空间只会使用与其所用版本相对应的结构字段
2. 定义独立的统计结构，并使netlink 属性接口返回它们。由于用户空间独立处理每netlink 属性，它总能忽略其不理解的类型属性（因为它使用的是较旧版本的接口）

1. 2. 之间选择，是在灵活性与开销之间权衡。如果只需添加少量字段，那1. 是更佳路径，因为内核和用户空间无需承担处理netlink 属性的开销。但如果新字段使现有结构膨胀过多，导致不同的用户空间记账工具不必要地接收包含无关字段的大结构，那么扩展属性结构就值得考虑
### taskstats 的流

当任务退出速率变得很大时，监听器可能跟不上内核发per-tid/tgid 退出数据的速率，从而导致数据丢失。当 taskstats 结构被扩展、CPU 数量增多时，这种可能性会进一步加剧
为避免丢失统计信息，用户空间应当采取以下一项或多项措施
- 增大监听器为接收退出数据而打开netlink 套接字的接收缓冲区大小
- 创建更多监听器，并减少每个监听器所监听CPU 数量。在极端情况下，可以为每CPU 各设一个监听器。用户也可以考虑将监听器CPU 亲和性设置为它所监听CPU 子集，尤其是在它只监听一CPU 时
尽管采取了这些措施，如果用户空间收到表明接收缓冲区溢出的 ENOBUFS 错误消息，它应当采取措施来处理数据丢失