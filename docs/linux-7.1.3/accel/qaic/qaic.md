
##  QAIC 驱动


QAIC 驱动是 AIC100 系列 AI 加速产品的内核态驱动（KMD）。

## 中断


### IRQ 风暴缓解


虽然 AIC100 DMA Bridge 硬件实现了 IRQ 风暴缓解机制，但仍有可能发生 IRQ 风暴。当工作负载执行得特别快、且主机响应及时时，风暴就可能出现。如果主机能像设备向响应 FIFO 中插入元素一样快地排空它，那么设备会频繁地将响应 FIFO 从空切换到非空，并以等同于工作负载处理输入速度的频率产生 MSI。已知 lprnet（车牌识别网络）工作负载会触发这一状况，每秒可产生超过 100k 个 MSI。据观察，大多数系统无法长时间承受这种情况，并且会因为中断控制器频繁打断主机 CPU 所带来的开销，而在某种看门狗机制下崩溃。

为缓解此问题，QAIC 驱动实现了特定的 IRQ 处理方式。当 QAIC 收到一个 IRQ 时，它会禁用该中断线。这就阻止了中断控制器打断 CPU。随后 AIC 排空 FIFO。FIFO 排空后，QAIC 实现一种"最后机会"轮询算法：QAIC 会休眠一段时间，以观察工作负载是否会产生更多活动。在此期间 IRQ 线保持禁用。如果未检测到活动，QAIC 退出轮询模式并重新启用 IRQ 线。

QAIC 中的这一缓解措施非常有效。同样的 lprnet 用例（按 /proc/interrupts 计）每秒产生 10 万个 IRQ，在被降至约 5 分钟内 64 个 IRQ 的同时，仍保持主机系统稳定，并且工作负载吞吐性能一致（在多次运行的噪声波动范围内）。

### 单 MSI 模式


并非所有系统都良好支持 MultiMSI；虚拟化系统支持得更差（截至 2023 年）。在介于虚拟机监控程序屏蔽 PCIe MSI 能力结构、以及支持 MultiMSI 所需的 vIOMMU 占用大量内存之间，能够在需要时回退到单个 MSI 是很有用的。

为支持这种回退，我们允许仅能分配一个 MSI、并在 MHI 与各 DBC 之间共享这一个 MSI 的情况。设备会检测到仅配置了一个 MSI，并将原本发给 DBC 的中断导向通常用于 MHI 的中断。遗憾的是，这意味着每个 DBC 和 MHI 的中断处理函数都会在每次中断到来时被唤醒；不过，DBC 的线程化 IRQ 处理函数仅在检测到有待处理工作（MHI 总会启动其线程化处理函数）时才会被启动。

如果 DBC 被配置为强制产生 MSI 中断，这可能会绕开上文提到的软件 IRQ 风暴缓解机制。由于 MSI 是共享的，它永远不会被禁用，从而允许 FIFO 中的每个新条目都触发一个新的中断。


## 神经网络控制（NNC）协议


NNC 的实现在 KMD（QAIC）与 UMD 之间拆分。一般而言，QAIC 懂得如何编解码 NNC 线路协议，以及协议中需要内核空间知识才能处理的部分（例如，将主机内存映射到设备 IOVA）。QAIC 理解消息的结构，以及所有的交互事务。QAIC 不理解命令（passthrough 事务的载荷）。

QAIC 在力所能及的范围内，处理并强制要求所需的小端序与 64 位对齐。由于 QAIC 不知道 passthrough 事务的内容，它依赖 UMD 来满足这些要求。

terminate 事务对 QAIC 特别有用。QAIC 并不知道加载到设备上的资源，因为这类活动大多发生在 NNC 命令内部。因此，QAIC 没有手段来回滚用户空间的活动。为确保用户空间客户端的资源在进程崩溃或出现错误时被完全释放，QAIC 使用 terminate 命令来告知 QSM 某个用户已经离开，资源可以被释放。

QSM 可以上报它所支持的 NNC 协议版本号。该版本号由主版本号和次版本号组成。

主版本号更新表示影响了消息格式或交互事务（会影响 QAIC）的 NNC 协议变更。

次版本号更新表示影响了命令（不会影响 QAIC）的 NNC 协议变更。

## uAPI


QAIC 为每个物理 PCIe 设备创建一个 accel 设备。只要 Linux 知晓该 PCIe 设备，这个 accel 设备就一直存在。

PCIe 设备并非在所有时刻都处于可接收用户空间请求的状态。QAIC 会触发 KOBJ_ONLINE/OFFLINE uevent，以通告设备何时可以接收请求（ONLINE），以及由于复位或其他状态切换而导致设备不再接收请求（OFFLINE）的时机。

QAIC 定义了一些驱动特有的 IOCTL，作为用户空间 API 的一部分。

DRM_IOCTL_QAIC_MANAGE
  该 IOCTL 允许用户空间向 QSM 发送一个 NNC 请求。调用将阻塞，直到收到响应或请求超时。

DRM_IOCTL_QAIC_CREATE_BO
  该 IOCTL 允许用户空间分配一个缓冲对象（BO），用于向工作负载发送或从中接收数据。调用将返回一个代表所分配缓冲区的 GEM 句柄。在 BO 被切片之前（参见 DRM_IOCTL_QAIC_ATTACH_SLICE_BO），它是不可用的。

DRM_IOCTL_QAIC_MMAP_BO
  该 IOCTL 允许用户空间准备一个已分配的 BO，以便 mmap 到用户空间进程中。

DRM_IOCTL_QAIC_ATTACH_SLICE_BO
  该 IOCTL 允许用户空间对一个 BO 进行切片，为将其发送到设备做准备。切片是描述 BO 的哪些部分被发往工作负载何处的一种操作。这需要在 DMA Bridge 上执行一组 DMA 传输，因此会将 BO 锁定到特定的 DBC 上。

DRM_IOCTL_QAIC_EXECUTE_BO
  该 IOCTL 允许用户空间向设备提交一组已切片的 BO。调用是非阻塞的。成功仅表示 BO 已被排队到设备，但并不保证它们已经执行。

DRM_IOCTL_QAIC_PARTIAL_EXECUTE_BO
  该 IOCTL 的工作方式与 DRM_IOCTL_QAIC_EXECUTE_BO 类似，但它允许用户空间缩小本次特定调用中发往设备的 BO 大小。如果一个 BO 通常有 N 个输入，但只有其中一部分可用，该 IOCTL 允许用户空间指示只应将 BO 的前 M 字节发往设备，以尽量减少数据传输开销。该 IOCTL 会动态重新计算切片，因此在 BO 被排队到设备之前会有一些处理开销。

DRM_IOCTL_QAIC_WAIT_BO
  该 IOCTL 允许用户空间确定一个特定 BO 何时已被设备处理。调用将阻塞，直到 BO 已被处理并可重新排队到设备，或发生超时。

DRM_IOCTL_QAIC_PERF_STATS_BO
  该 IOCTL 允许用户空间收集对 BO 最近一次执行的性能统计。这使用户空间能够构建 BO 处理的端到端时间线，用于性能分析。

DRM_IOCTL_QAIC_DETACH_SLICE_BO
  该 IOCTL 允许用户空间移除由最初 DRM_IOCTL_QAIC_ATTACH_SLICE_BO 调用提供的 BO 切片信息。它是 DRM_IOCTL_QAIC_ATTACH_SLICE_BO 的逆操作。该 BO 必须处于空闲状态才能调用 DRM_IOCTL_QAIC_DETACH_SLICE_BO。在成功执行 detach slice 操作后，可通过再次调用 DRM_IOCTL_QAIC_ATTACH_SLICE_BO 为 BO 附加新的切片信息。detach slice 之后，在重新执行 attach slice 操作之前，BO 不能被执行。结合 attach slice 与 detach slice 调用，用户空间可以在多个工作负载中使用同一个 BO。

## 用户空间客户端隔离


AIC100 支持多个客户端。单个客户端可以占用多个 DBC，多个客户端也可以各自占用一个或多个 DBC。工作负载可能包含敏感信息，因此只允许拥有该工作负载的客户端与 DBC 交互。

客户端通过其 open() 相关联的实例来标识。一个客户端只能使用它们自己分配的内存，以及分配给其工作负载的 DBC。尝试访问分配给其他客户端的资源将被拒绝。

## 模块参数


QAIC 支持以下模块参数：

**datapath_polling (bool)**

配置 QAIC 使用轮询线程来处理数据通路事件，而非依赖设备中断。适用于多 MSI 损坏的平台。必须在 QAIC 驱动初始化时设置。默认为 0（关闭）。

**mhi_timeout_ms (unsigned int)**

以毫秒（ms）为单位设置 MHI 操作的超时值。必须在驱动检测到设备时设置。默认为 2000（2 秒）。

**control_resp_timeout_s (unsigned int)**

以秒（s）为单位设置 QSM 对 NNC 消息响应的超时值。必须在驱动向 QSM 发送请求时设置。默认为 60（一分钟）。

**wait_exec_default_timeout_ms (unsigned int)**

以毫秒（ms）为单位设置 wait_exec ioctl 的默认超时。必须在 wait_exec ioctl 调用之前设置。ioctl 调用中指定的值会覆盖该默认值。默认为 5000（5 秒）。

**datapath_poll_interval_us (unsigned int)**

当数据通路轮询处于活动状态时，以微秒（us）为单位设置轮询间隔。在下一个轮询间隔生效。默认为 100（100 us）。

**timesync_delay_ms (unsigned int)**

以毫秒（ms）为单位设置两次连续时间同步操作之间的时间间隔。默认为 1000（1000 ms）。
