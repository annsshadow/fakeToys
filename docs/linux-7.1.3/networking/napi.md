
## NAPI


NAPI Linux 网络栈使用的事件处理机制。NAPI 这个名字不再代表任何特定的含[#]_

在基本操作中，设备通过中断将新事件通知主机。然后主机调度一NAPI 实例来处理这
事件。也可以在不先收到中断的情况下通过 NAPI 轮询设备以获取事件（忙轮poll>）

NAPI 处理通常发生在软件中断上下文中，但也可以选择使用单独的內核线threaded>来进
NAPI 处理

总而言之，NAPI 向驱动屏蔽了事件（数据包接收与发送）处理的上下文和配置

## 驱动 API


NAPI 最重要的两个元素是 struct napi_struct 和相关的 poll 方法。struct napi_struct
保存 NAPI 实例的状态，而该方法则是驱动特定的事件处理程序。该方法通常会释放已发送的
Tx 数据包并处理新收到的数据包


### 控制 API


netif_napi_add() netif_napi_del() 用于从系统中添加/删除一NAPI 实例。这些实
被附加到作为参数传入netdevice 上（并且netdevice 注销时会自动删除）。实例以
禁用状态被添加

napi_enable() napi_disable() 管理禁用状态。一个被禁用NAPI 不能被调度，并且
保证不会调用它的 poll 方法。napi_disable() 会等NAPI 实例的所有权被释放

控制 API 不是幂等的。控API 的调用对于数据路API 的并发使用是安全的，但是不正
的控API 调用序列可能导致崩溃、死锁或竞争条件。例如，连续多次调用 napi_disable()
会死锁

### 数据路径 API


napi_schedule() 是调NAPI 轮询的基本方法。驱动应该在它们的中断处理程序中调用
函数（更多信息见 drv_sched）。一次对 napi_schedule() 的成功调用将取得 NAPI 实例
所有权

之后，在 NAPI 被调度后，驱动的 poll 方法会被调用来处理事数据包。该方法接受一
`budget` 参数——驱动可以为任意数量Tx 数据包处理完成，但只应处理最`budget` 
Rx 数据包。Rx 处理通常昂贵得多

换句话说，对Rx 处理，`budget` 参数限制了驱动在一次轮询中可以处理的数据包数量。当
`budget` 0 时，完全不能使用像页池或 XDP 这样Rx 特定 API。无`budget` 如何
应进skb Tx 处理，但如果参数0，驱动不能调用任XDP（或页池）API


   `budget` 参数可能0，如果核心只尝试处理 skb Tx 完成而没Rx XDP 数据包

poll 方法返回已完成的工作量。如果驱动仍有未完成的工作要做（例如 `budget` 已耗尽），
poll 方法应正好返`budget`。在这种情况下，NAPI 实例将被再次服务/轮询（无需再被
调度）

如果事件处理已完成（所有未完成的数据包都已处理），poll 方法应在返回前调
napi_complete_done()。napi_complete_done() 释放该实例的所有权


   处理完所有事件且恰好用了 `budget` 的情况必须小心处理。没有办法向栈报告这个（罕见
   的情况，所以驱动必须要么不调用 napi_complete_done() 并等待再次被调用，要么返
   `budget - 1`銆。

   如果 `budget` 0，则绝不应调napi_complete_done()

### 调用序列


驱动不应假定调用的确切序列。即使没有驱动调度该实例（除非该实例被禁用），poll 方法
也可能被调用。类似地，即napi_schedule() 成功了，也不能保证会调用 poll 方法
（例如，如果该实例被禁用）

drvctrl 一节所述——napi_disable() 及其后对 poll 方法的调用只等待实例的所有权
被释放，而不是等poll 方法退出。这意味着驱动在调napi_complete_done() 之后
避免访问任何数据结构


### 调度IRQ 屏蔽


驱动在调NAPI 实例后应使中断保持屏蔽——在 NAPI 轮询完成之前，任何进一步的中断
是不必要的

需要显式屏蔽中断的驱动（与中断被设备自动屏蔽相反）应使napi_schedule_prep() 
__napi_schedule() 调用


  if (napi_schedule_prep(&v->napi)) {
      mydrv_mask_rxtx_irq(v->idx);
      /** 在屏蔽之后调度以避免竞争 **/
      __napi_schedule(&v->napi);
  }

只有当对 napi_complete_done() 的调用成功后，才应解除中断屏蔽：


  if (budget && napi_complete_done(&v->napi, work_done)) {
    mydrv_unmask_rxtx_irq(v->idx);
    return min(work_done, budget - 1);
  }

napi_schedule_irqoff() napi_schedule() 的一个变体，它利用了IRQ 上下文中被调
所提供的保证（无需屏蔽中断）。如IRQ 被线程化（例如启用了 `PREEMPT_RT`），
napi_schedule_irqoff() 会回退napi_schedule()

### 实例到队列的映射


现代设备每个接口有多NAPI 实例（struct napi_struct）。对于实例如何映射到队列
中断，没有严格的要求。NAPI 主要是一个轮处理抽象，没有特定的面向用户的语义。话
如此，大多数网络设备最终都以相当类似的方式使用 NAPI

NAPI 实例最常见的是与中断和队列对（队列对是一组单Rx 和单Tx 队列）以 1:1:1 
方式对应

在不太常见的情况下，一NAPI 实例可能用于多个队列，或Rx Tx 队列可以由单
核心上的独立 NAPI 实例服务。不过，无论队列如何分配，NAPI 实例和中断之间通常仍然
1:1 的映射

值得注意的是，ethtool API 使用“channel”术语，其中每个 channel 可以`rx`、`tx`
`combined`。目前还不清楚什么构成一channel；推荐的诠释是将 channel 理解为服务于
给定类型队列IRQ/NAPI。例如，1 `rx` `tx` 1 `combined` channel 
配置预期会使3 个中断 Rx 2 Tx 队列

### 持久 NAPI 配置


驱动通常动态分配和释放 NAPI 实例。这导致每次重新分配 NAPI 实例时都会丢失与 NAPI 相关
用户配置。netif_napi_add_config() API 通过基于驱动定义的索引值（如队列号）将每个 NAPI
实例与持久的 NAPI 配置关联起来，防止这种配置丢失

使用API 可以实现持久NAPI ID（以及其他设置），这对使`SO_INCOMING_NAPI_ID` 
用户空间程序是有益的。其NAPI 配置设置见下文各节

驱动应尽可能尝试使用 netif_napi_add_config()

## 用户 API


用户NAPI 的交互依赖于 NAPI 实例 ID。这些实ID 只有通过 `SO_INCOMING_NAPI_ID` 套接
选项对用户可见

用户可以使用 netlink 查询设备或设备队列的 NAPI ID。这可以在用户应用程序中以编程方
完成，或者使用内核源码树中附带的脚本：`tools/net/ynl/pyynl/cli.py`

例如，使用脚本转储设备的所有队列（这将显示每个队列NAPI ID）：


   $ kernel-source/tools/net/ynl/pyynl/cli.py \
             --spec Documentation/netlink/specs/netdev.yaml \
             --dump queue-get \
             --json='{"ifindex": 2}'

关于可用操作和属性的更多细节，请参见 `Documentation/netlink/specs/netdev.yaml`

### 软件 IRQ 合并


默认情况下，NAPI 不执行任何显式的事件合并。在大多数场景中，批处理是由于设备完成的
IRQ 合并而发生的。有些情况下软件合并是有帮助的

NAPI 可以配置为在数据包全部处理完后，挂起一个重新轮询定时器，而不是解除硬件中断的
屏蔽。netdevice `gro_flush_timeout` sysfs 配置被复用用于控制该定时器的延迟，
`napi_defer_hard_irqs` 控制NAPI 放弃并回到使用硬IRQ 之前连续空轮询的次数

上述参数也可以使netlink 通过 netdev-genl 在每NAPI 的基础上设置。当netlink
一起使用并基于每个 NAPI 配置时，上述参数使用连字符而不是下划线：`gro-flush-timeout`
鍜?`napi-defer-hard-irqs`銆。

基于每个 NAPI 的配置可以在用户应用程序中以编程方式完成，或者使用内核源码树中附带的
脚本：`tools/net/ynl/pyynl/cli.py`

例如，使用脚本：


  $ kernel-source/tools/net/ynl/pyynl/cli.py \
            --spec Documentation/netlink/specs/netdev.yaml \
            --do napi-set \
            --json='{"id": 345,
                     "defer-hard-irqs": 111,
                     "gro-flush-timeout": 11111}'

类似地，参数 `irq-suspend-timeout` 可以使用 netlink 通过 netdev-genl 设置。没有用
此值的全局 sysfs 参数

`irq-suspend-timeout` 用于确定应用程序可以完全挂起 IRQ 多长时间。它SO_PREFER_BUSY_POLL
结合使用，后者可以基于每epoll 上下文通过 `EPIOCSPARAMS` ioctl 设置


### 忙轮


忙轮询允许用户在设备中断触发之前检查是否有传入的数据包。与任何形式的忙轮询一样，
CPU 周期为代价换取更低的延迟（NAPI 忙轮询的生产用途尚不为人所知）

忙轮询通过要么在选定的套接字上设`SO_BUSY_POLL`，要么使用全局`net.core.busy_poll`
`net.core.busy_read` sysctl 来启用。也存在一个用NAPI 忙轮询的 io_uring API。NAPI
的线程化轮询也有一种模式，使用 NAPI 处理 kthread 来忙轮询数据包（线程化忙轮询
<threaded_busy_poll>）

### 基于 epoll 的忙轮询


可以直接从对 `epoll_wait` 的调用触发数据包处理。为了使用此功能，用户应用程序必须确
添加epoll 上下文的所有文件描述符具有相同NAPI ID

如果应用程序使用专用的接收线程，应用程序可以使用 SO_INCOMING_NAPI_ID 获取传入连接
NAPI ID，然后将该文件描述符分发给工作线程。工作线程会将该文件描述符添加到它的 epoll
上下文。这将确保每个工作线程都有一个包含具有相NAPI ID FD epoll 上下文

或者，如果应用程序使用 SO_REUSEPORT，可以插入一bpf ebpf 程序来将传入连接分发
线程，使得每个线程只得到具有相同 NAPI ID 的传入连接。必须小心处理系统可能有多个 NIC
的情况

为了启用忙轮询，有两个选择

1. `/proc/sys/net/core/busy_poll` 可以设置为以微秒为单位的时间，用于忙循环等待事件
   这是一个系统范围的设置，将导致所有基epoll 的应用程序在调用 epoll_wait 时忙轮询
   这可能并不可取，因为许多应用程序可能不需要忙轮询

2. 使用较新内核的应用程序可以在 epoll 上下文文件描述符上发ioctl 来设置（`EPIOCSPARAMS`
   或获取（`EPIOCGPARAMS`）``struct epoll_params``:，用户程序可以如下定义：


  struct epoll_params {
      uint32_t busy_poll_usecs;
      uint16_t busy_poll_budget;
      uint8_t prefer_busy_poll;

      /** 将结构填充到 64 位的倍数 **/
      uint8_t __pad;
  };

### IRQ 缓解


虽然忙轮询应该被低延迟应用程序使用，但类似的机制可用IRQ 缓解

非常高每秒请求数的应用程序（尤其是路转发应用程序，尤其是使用 AF_XDP 套接字的应用
程序）可能希望在完成处理一个请求或一批数据包之前不被中断

此类应用程序可以向内核保证它们将定期执行忙轮询操作，并且驱动应该使设IRQ 永久屏蔽
此模式通过使用 `SO_PREFER_BUSY_POLL` 套接字选项启用。为了避免系统异常行为，如果
`gro_flush_timeout` 在没有忙轮询调用的情况下过去，该保证将被撤销。对于基epoll 
忙轮询应用程序，``struct epoll_params` `prefer_busy_poll` 字段可以设为 1，并可以发出
`EPIOCSPARAMS`` ioctl 来启用此模式。更多细节见上一节

NAPI 忙轮询的预算低于默认（考虑到正常忙轮询的低延迟意图，这是合理的）。然而，IRQ 缓解
并非如此，因此预算可以通过 `SO_BUSY_POLL_BUDGET` 套接字选项调整。对于基epoll 的忙
轮询应用程序，可以在 `struct epoll_params` 中调`busy_poll_budget` 字段为所需的值，
使用 `EPIOCSPARAMS` ioctl 设置在特定的 epoll 上下文上。更多细节见上一节

需要注意的是，`gro_flush_timeout` 选择一个较大的值将推迟 IRQ 以允许更好的批处理，
会在系统未完全加载时引入延迟。为 `gro_flush_timeout` 选择一个较小的值可能会因为设备 IRQ
和软中断处理而干扰正在尝试忙轮询的用户应用程序。应在考虑这些权衡的情况下仔细选择此值
基于 epoll 的忙轮询应用程序也许能够通过`maxevents` 选择合适的值来缓解有多少用
处理发生

用户可能想考虑一种替代方法，IRQ 挂起，来帮助处理这些权衡

### IRQ 挂起


IRQ 挂起是一种机制，其中epoll 触发 NAPI 数据包处理时屏蔽设备 IRQ

当应用程序对 epoll_wait 的调用成功检索到事件时，内核将推IRQ 挂起定时器。如果内核在
忙轮询时没有检索到任何事件（例如，因为网络流量水平下降），IRQ 挂起被禁用，并启用上
IRQ 缓解策略

这允许用户平CPU 消耗与网络处理效率

要使用此机制

  1. 应将基于每个 NAPI 的配置参`irq-suspend-timeout` 设置为应用程序可以挂起其 IRQ 
     最长时间（以纳秒为单位）。这是使netlink 完成的，如上所述。此超时作为一个安
     机制，在应用程序停滞时重新启IRQ 驱动的中断处理。应选择此值以覆盖用户应用程序
     从其epoll_wait 的调用处理数据所需的时间量，注意应用程序可以通过在调epoll_wait
     时设`max_events` 来控制它们检索多少数据

  2. sysfs 参数或基于每NAPI 的配置参`gro_flush_timeout` `napi_defer_hard_irqs`
     可以设置为较小的值。它们将用于在忙轮询没有找到数据后推IRQ

  3. 必须`prefer_busy_poll` 标志设为 true。这可以使用如上所述的 `EPIOCSPARAMS` ioctl
     完成

  4. 应用程序如上所述使epoll 来触NAPI 数据包处理

如上所述，只要后续epoll_wait 的调用向用户空间返回事件，`irq-suspend-timeout` 就被
推迟，IRQ 被禁用。这允许应用程序不受干扰地处理数据

一旦对 epoll_wait 的调用没有找到任何事件，IRQ 挂起被自动禁用，并且 `gro_flush_timeout`
`napi_defer_hard_irqs` 缓解机制接管

预期 `irq-suspend-timeout` 会被设置为比 `gro_flush_timeout` 大得多的值，因为
`irq-suspend-timeout` 应该在一个用户空间处理周期内挂起 IRQ

虽然使用 IRQ 挂起并不严格需要使`napi_defer_hard_irqs` `gro_flush_timeout`，但强烈
建议使用它们

IRQ 挂起使系统在轮询模式和中断驱动的数据包交付之间交替。在繁忙期间，`irq-suspend-timeout`
覆盖 `gro_flush_timeout` 并使系统保持忙轮询，但当 epoll 没有找到事件时，`gro_flush_timeout`
`napi_defer_hard_irqs` 的设置决定下一步

网络处理和数据包交付基本上有三个可能的循环：

1) hardirq -> softirq -> napi poll；基本的中断交付
2) timer -> softirq -> napi poll；推迟的 irq 处理
3) epoll -> busy-poll -> napi poll；忙循环

如果设置`gro_flush_timeout` `napi_defer_hard_irqs`，Loop 2 可以Loop 1 夺取控制

如果设置`gro_flush_timeout` `napi_defer_hard_irqs`，Loop 2 3 会相互“争夺
控制权

在繁忙期间，`irq-suspend-timeout` Loop 2 中用作定时器，这本质上使网络处理偏向
Loop 3銆。

如果未设`gro_flush_timeout` `napi_defer_hard_irqs`，Loop 3 不能Loop 1 夺取
控制

因此，建议设`gro_flush_timeout` `napi_defer_hard_irqs`，因为否则设
`irq-suspend-timeout` 可能没有任何可辨别的效果


### 线程NAPI 忙轮


线程NAPI 忙轮询扩展了线程NAPI，并添加了对 NAPI 进行连续忙轮询的支持。这对转发或
AF_XDP 应用程序很有用

线程NAPI 忙轮询可以使Netlink 在每NIC 队列的基础上启用

例如，使用以下脚本：


  $ ynl --family netdev --do napi-set \
            --json='{"id": 66, "threaded": "busy-poll"}'

内核将创建一个在NAPI 上忙轮询kthread

用户可以选择将此 kthread CPU 亲和性设置为一个未使用CPU 核心，以提高 NAPI 被轮
的频率，代价是浪CPU 周期。注意，这将使该 CPU 核心保持 100% 的使用率

一旦为某个 NAPI 启用了线程化忙轮询，就可以使Netlink 获取kthread PID，以便设
kthread 的亲和性

例如，可以使用以下脚本获PID


  $ ynl --family netdev --do napi-get --json='{"id": 66}'

这将输出类似以下内容，pid `258` 是正在轮询此 NAPI kthread PID


  $ {'defer-hard-irqs': 0,
     'gro-flush-timeout': 0,
     'id': 66,
     'ifindex': 2,
     'irq-suspend-timeout': 0,
     'pid': 258,
     'threaded': 'busy-poll'}


### 线程NAPI


线程NAPI 是一种操作模式，它使用专用的内核线程而不是软IRQ 上下文来进行 NAPI 处理
每个线程NAPI 实例将生成一个单独的线程（称`napi/${ifc-name}-${napi-id}`）

建议将每个内核线程固定到单个 CPU，即服务该中断的同一CPU。注意，IRQ NAPI 实例之间
的映射可能并不简单（并且依赖于驱动）。NAPI 实例 ID 将以与内核线程的进程 ID 相反的顺
分配

线程NAPI 通过0/1 写入 netdev sysfs 目录中的 `threaded` 文件来控制。它也可以使
netlink 接口为特定的 NAPI 启用

例如，使用脚本：


  $ ynl --family netdev --do napi-set --json='{"id": 66, "threaded": 1}'
