RapidIO 子系统通道化消息字符设备驱动（rio_cm.c）



1. 概述


该设备驱动是 RapidIO.org 软件任务组（STG）内 Texas Instruments、Prodrive Technologies、Nokia Networks、BAE 与 IDT 之间协作的成果。其他 RapidIO.org 成员也提供了额外的输入。

其目标是创建一个字符模式驱动接口，将 RapidIO 端点设备（mports）的消息传递能力直接暴露给应用程序，以允许众多不同的 RapidIO 实现相互互操作。

该驱动（RIO_CM）为用户空间应用程序提供对 RapidIO 邮箱消息资源的共享访问。

RapidIO 规范（第 2 部分）定义，端点设备在多包消息（最大 4KB）情况下可拥有最多四个消息邮箱，而在使用单包消息（最大 256 B）时最多可有 64 个邮箱。除了协议定义的限制外，特定的硬件实现可能减少消息邮箱的数量。因此，支持 RapidIO 的应用程序必须共享 RapidIO 端点的消息资源。

该设备驱动的主要目的，是通过使用单个消息邮箱引入类套接字的操作，向大量用户空间进程提供 RapidIO 邮箱消息传递能力。这使得应用程序能够高效地使用有限的 RapidIO 消息硬件资源。

该设备驱动的大部分操作通过 'ioctl' 系统调用支持。

加载后，该设备驱动会在 /dev 目录下创建一个名为 rio_cm 的文件系统节点，该节点对所有已注册的 RapidIO mport 设备通用。

以下 ioctl 命令可供用户空间应用程序使用：

- RIO_CM_MPORT_GET_LIST:
    向调用者返回支持消息操作的本地 mport 设备列表（条目数最多为 RIO_MAX_MPORTS）。
    每个列表条目由 mport 在系统中的索引以及分配给该端口的 RapidIO
    目标 ID 组合而成。
- RIO_CM_EP_GET_LIST_SIZE:
    返回与指定 mport 设备关联的 RapidIO 网络中
    支持消息传递的远程端点数量。
- RIO_CM_EP_GET_LIST:
    返回与指定 mport 设备关联的 RapidIO 网络中可用的
    支持消息传递的远程端点（对等端）的 RapidIO 目标 ID 列表。
- RIO_CM_CHAN_CREATE:
    创建 RapidIO 消息交换通道数据结构，
    通道 ID 由系统自动分配或由调用者请求指定。
- RIO_CM_CHAN_BIND:
    将指定的通道数据结构绑定到指定的
    mport 设备。
- RIO_CM_CHAN_LISTEN:
    在指定通道上启用对连接请求的监听。
- RIO_CM_CHAN_ACCEPT:
    接受来自指定通道上对等端的连接请求。如果调用者指定了该请求的等待超时，
    则这是一个阻塞调用。如果超时设为 0，则为非阻塞调用——ioctl
    处理程序检查是否有待处理的连接请求，若没有则立即以 -EGAIN 错误状态退出。
- RIO_CM_CHAN_CONNECT:
    向远程对等端/通道发送连接请求。
- RIO_CM_CHAN_SEND:
    通过指定通道发送数据消息。
    该请求的处理程序假定调用者指定的消息缓冲区包含本驱动所需
    数据包头的预留空间。
- RIO_CM_CHAN_RECEIVE:
    通过已连接的通道接收数据消息。
    如果通道没有已就绪可返回的消息，此 ioctl 处理程序将等待新消息，
    直到调用者指定的超时到期。如果超时值设为 0，ioctl 处理程序使用
    MAX_SCHEDULE_TIMEOUT 定义的默认值。
- RIO_CM_CHAN_CLOSE:
    关闭指定通道并释放相关缓冲区。
    如果指定通道处于 CONNECTED 状态，则向远程对等端发送关闭通知。

供用户空间应用程序使用的 ioctl 命令码及相应数据结构定义于 'include/uapi/linux/rio_cm_cdev.h'。

2. 硬件兼容性


该设备驱动使用内核 RapidIO 子系统定义的标准接口，因此可与任何由 RapidIO 子系统注册、且受可用 mport 硬件邮箱实现限制的 mport 设备驱动配合使用。

3. 模块参数


- 'dbg_level'
      - 该参数用于控制本设备驱动生成的调试信息量。
        该参数由一组对应于特定功能块的位掩码组成。
        有关掩码定义，请参阅 'drivers/rapidio/devices/rio_cm.c'。
        该参数可动态更改。
        使用 CONFIG_RAPIDIO_DEBUG=y 可在顶层启用调试输出。


- 'cmbox'
      - 要使用的 RapidIO 邮箱编号（默认值为 1）。
        该参数用于设置将在整个 RapidIO 网络中使用的消息邮箱编号。
        当默认邮箱被其他设备驱动使用，或 RapidIO 网络中某些节点不支持时，可使用该参数。

- 'chstart'
      - 动态分配的起始通道号。默认值为 256。
        用于将该参数以下的通道号排除在动态分配之外，
        以避免与使用预留预定义通道号的软件组件产生冲突。

4. 已知问题


  无。

5. 用户空间应用程序与 API 库


使用本设备驱动的消息传递 API 库及应用程序可从 RapidIO.org 获取。

6. 待办列表


- 添加对系统通知消息的支持（预留通道 0）。
