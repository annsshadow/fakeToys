
## 用户空间块设备驱动（ublk 驱动）


## 概述


ublk 是一个用于从用户空间实现块设备逻辑的通用框架。其背后的动机是：将虚拟块
驱动（如 loop、nbd 以及类似的驱动）移入用户空间可能会非常有帮助。它有助于实现
新的虚拟块设备，例如 ublk-qcow2（业界已有数次在内核中实现 qcow2 驱动的尝试）。

用户空间块设备之所以有吸引力，是因为：

- 它们可以用多种编程语言编写。
- 它们可以使用内核中不可用的库。
- 它们可以用应用开发者熟悉的工具进行调试。
- 崩溃不会导致机器内核恐慌（kernel panic）。
- 与内核代码中的缺陷相比，缺陷的安全影响可能更小。
- 它们可以独立于内核安装和更新。
- 它们可以方便地用用户指定参数/设置来模拟块设备，以用于测试/调试目的。

ublk 块设备（`/dev/ublkb*`）由 ublk 驱动添加。该设备上的任何 IO 请求都将被转发给
ublk 用户空间程序。为方便起见，在本文档中，`ublk server` 指通用的 ublk 用户空间
程序。`ublksrv` [#userspace]_ 就是此类实现之一。它提供 `libublksrv` [#userspace_lib]_
库用于方便地开发特定的用户块设备，同时也包含通用的类型块设备，例如 loop 与 null。
Richard W.M. Jones 基于 `libublksrv` [#userspace_lib]_ 编写了用户空间 nbd 设备
`nbdublk` [#userspace_nbdublk]_。

IO 由用户空间处理完成后，结果会被提交回驱动，从而完成请求周期。如此一来，任何
特定的 IO 处理逻辑都完全由用户空间完成，例如 loop 的 IO 处理、NBD 的 IO 通信，或
qcow2 的 IO 映射。

`/dev/ublkb*` 由基于 blk-mq 请求（request-based）的驱动驱动。每个请求被分配一个
队列范围内唯一的 tag。ublk server 也为每个 IO 分配唯一的 tag，它与 `/dev/ublkb*`
的 IO 是 1:1 映射的。

IO 请求的转发与 IO 处理结果的提交都通过 `io_uring` 直通（passthrough）命令完成；
这正是 ublk 也是一个基于 io_uring 的块驱动的原因。已经观察到，使用 io_uring 直通
命令可以获得比块 IO 更好的 IOPS；这就是 ublk 成为用户空间块设备的高性能实现之一
的原因：不仅 IO 请求通信通过 io_uring 完成，ublk server 中首选的 IO 处理方式也是
基于 io_uring 的方案。

ublk 提供控制接口来设置/获取 ublk 块设备的参数。该接口是可扩展的，并且 kabi 兼容：
基本上任何 ublk 请求队列的参数或 ublk 通用特性参数都可以通过该接口设置/获取。因此，
ublk 是通用的用户空间块设备框架。例如，可以方便地从用户空间用指定的块参数来建立
一个 ublk 设备。

## 使用 ublk


ublk 需要用户空间的 ublk server 来处理真实的块设备逻辑。

下面是使用 `ublksrv` 提供基于 ublk 的 loop 设备的示例。

```

     ublk add -t loop -f ublk-loop.img

```
```

     mkfs.xfs /dev/ublkb0
     mount /dev/ublkb0 /mnt
     # do anything. all IOs are handled by io_uring
     ...
     umount /mnt

```
```

     ublk list

```
```

     ublk del -a
     ublk del -n $ublk_dev_id

```
使用细节参见 `ublksrv` [#userspace_readme]_ 的 README。

## 设计


### 控制平面


ublk 驱动提供全局杂项设备节点（`/dev/ublk-control`）用于管理和控制 ublk 设备，
借助若干控制命令：

- `UBLK_CMD_ADD_DEV`

  添加一个 ublk 字符设备（`/dev/ublkc*`），ublk server 与之就 IO 命令通信。基本的
  设备信息随此命令一起发送。它设置 `ublksrv_ctrl_dev_info` 的 UAPI 结构，例如
  `nr_hw_queues`、`queue_depth` 以及最大 IO 请求缓冲区大小，这些信息与驱动协商后
  回送给 server。当此命令完成时，基本设备信息变为不可变。

- `UBLK_CMD_SET_PARAMS` / `UBLK_CMD_GET_PARAMS`

  设置或获取设备的参数，可以是通用特性相关的，也可以是请求队列限制相关的，但不能
  是 IO 逻辑特定的，因为驱动不处理任何 IO 逻辑。此命令必须在发送 `UBLK_CMD_START_DEV`
  之前发送。

- `UBLK_CMD_START_DEV`

  在 server 准备好用户空间资源（例如创建用于 handle ublk IO 的 I/O 处理线程 &
  io_uring）之后，发送此命令给驱动以分配并暴露 `/dev/ublkb*`。通过
  `UBLK_CMD_SET_PARAMS` 设置的参数会被应用于创建设备。

- `UBLK_CMD_STOP_DEV`

  停止 `/dev/ublkb*` 上的 IO 并移除设备。当此命令返回时，ublk server 将释放资源
  （例如销毁 I/O 处理线程 & io_uring）。

- `UBLK_CMD_DEL_DEV`

  移除 `/dev/ublkc*`。当此命令返回时，已分配的 ublk 设备号可被复用。

- `UBLK_CMD_GET_QUEUE_AFFINITY`

  当添加 `/dev/ublkc` 时，驱动创建块层 tagset，于是每个队列的亲和性（affinity）
  信息可用。server 发送 `UBLK_CMD_GET_QUEUE_AFFINITY` 来检索队列亲和性信息。它可以
  高效地建立每队列上下文，例如将亲和 CPU 与 IO pthread 绑定，并尝试在 IO 线程上下文中
  分配缓冲区。

- `UBLK_CMD_GET_DEV_INFO`

  用于通过 `ublksrv_ctrl_dev_info` 检索设备信息。在用户空间保存 IO 目标特定信息是
  server 的职责。

- `UBLK_CMD_GET_DEV_INFO2`
  与 `UBLK_CMD_GET_DEV_INFO` 目的相同，但 ublk server 必须提供 `/dev/ublkc*` 字符
  设备的路径，供内核执行权限检查，此命令是为支持非特权 ublk 设备而添加的，并与
  `UBLK_F_UNPRIVILEGED_DEV` 一起引入。只有拥有所请求设备的用户才能检索设备信息。

  如何处理用户空间/内核兼容性：

  1) 如果内核能够处理 `UBLK_F_UNPRIVILEGED_DEV`

    如果 ublk server 支持 `UBLK_F_UNPRIVILEGED_DEV`：

    ublk server 应该发送 `UBLK_CMD_GET_DEV_INFO2`，因为非特权应用随时可能需要查询
    当前用户所拥有的设备；当应用无从知晓 `UBLK_F_UNPRIVILEGED_DEV` 是否已设置（因为
    能力信息是无状态的）时，应用应始终通过 `UBLK_CMD_GET_DEV_INFO2` 来检索它。

    如果 ublk server 不支持 `UBLK_F_UNPRIVILEGED_DEV`：

    `UBLK_CMD_GET_DEV_INFO` 始终被发送给内核，而 `UBLK_F_UNPRIVILEGED_DEV` 特性对用户
    不可用。

  2) 如果内核不能处理 `UBLK_F_UNPRIVILEGED_DEV`

    如果 ublk server 支持 `UBLK_F_UNPRIVILEGED_DEV`：

    先尝试 `UBLK_CMD_GET_DEV_INFO2`，将会失败，然后需要重试 `UBLK_CMD_GET_DEV_INFO`，
    因为 `UBLK_F_UNPRIVILEGED_DEV` 无法被设置。

    如果 ublk server 不支持 `UBLK_F_UNPRIVILEGED_DEV`：

    `UBLK_CMD_GET_DEV_INFO` 始终被发送给内核，而 `UBLK_F_UNPRIVILEGED_DEV` 特性对用户
    不可用。

- `UBLK_CMD_START_USER_RECOVERY`

  此命令在 `UBLK_F_USER_RECOVERY` 特性启用时有效。此命令在旧进程已退出、ublk 设备已
  静止（quiesced）且 `/dev/ublkc*` 已释放后被接受。用户应在启动重新打开 `/dev/ublkc*`
  的新进程之前发送此命令。当此命令返回时，ublk 设备已为新进程准备就绪。

- `UBLK_CMD_END_USER_RECOVERY`

  此命令在 `UBLK_F_USER_RECOVERY` 特性启用时有效。此命令在 ublk 设备已静止、且新进程
  已打开 `/dev/ublkc*` 并使所有 ublk 队列准备就绪后被接受。当此命令返回时，ublk 设备
  取消静止，新的 I/O 请求被传递给新进程。

- 用户恢复（user recovery）特性描述

  为支持用户恢复新增了三个特性：`UBLK_F_USER_RECOVERY`、`UBLK_F_USER_RECOVERY_REISSUE`
  和 `UBLK_F_USER_RECOVERY_FAIL_IO`。为了在 ublk server 退出后能够恢复 ublk 设备，ublk
  server 应在创建设备时指定 `UBLK_F_USER_RECOVERY` 标志。ublk server 还可额外指定至多
  一个 `UBLK_F_USER_RECOVERY_REISSUE` 与 `UBLK_F_USER_RECOVERY_FAIL_IO`，以修改在 ublk
  server 正在死亡/已死亡时（这被称为驱动代码中的 `nosrv` 情形）如何处理 I/O。

  仅设置 `UBLK_F_USER_RECOVERY` 时，在 ublk server 退出后，ublk 在整个恢复阶段都不会
  删除 `/dev/ublkb*`，并且 ublk 设备 ID 会被保留。由 ublk server 自行负责根据自身知识
  恢复设备上下文。尚未下发到用户空间的请求会被重新入队。已下发到用户空间的请求会被
  中止（abort）。

  额外设置 `UBLK_F_USER_RECOVERY_REISSUE` 时，与 `UBLK_F_USER_RECOVERY` 相反，在 ublk
  server 退出后，已下发到用户空间的请求会被重新入队，并会在处理完
  `UBLK_CMD_END_USER_RECOVERY` 后被重新下发给新进程。`UBLK_F_USER_RECOVERY_REISSUE` 是
  为那些可容忍双重写入的后端设计的，因为驱动可能两次下发同一个 I/O 请求。它可能对
  只读文件系统或 VM 后端有用。

  额外设置 `UBLK_F_USER_RECOVERY_FAIL_IO` 时，在 ublk server 退出后，已下发到用户空间的
  请求会失败，任何后续下发的请求也同样失败。持续对设置了该标志的设备发起 I/O 的应用
  将看到一串 I/O 错误，直到新的 ublk server 恢复该设备。

非特权 ublk 设备通过传递 `UBLK_F_UNPRIVILEGED_DEV` 来支持。一旦设置了该标志，所有控制
命令都可以由非特权用户发送。除了 `UBLK_CMD_ADD_DEV` 命令外，ublk 驱动会对所有其他
控制命令执行针对指定字符设备（`/dev/ublkc*`）的权限检查；为此，这些命令的载荷中必须
由 ublk server 提供字符设备的路径。通过这种方式，ublk 设备变得容器感知（container-aware），
在一个容器中创建的设备只能在该容器内部被控制/访问。

### 数据平面


ublk server 应创建专用线程来处理 I/O。每个线程应有其自身的 io_uring，通过它来获知
新的 I/O，也通过它来完成 I/O。这些专用线程应专注于 IO 处理，不应处理任何控制与管理
任务。

ublk 的 IO 由一个唯一的 tag 分配，它与 `/dev/ublkb*` 的 IO 请求是 1:1 映射。

定义 `ublksrv_io_desc` 的 UAPI 结构用于描述来自驱动的每个 IO。在 `/dev/ublkc*` 上
提供了一个固定的 mmap 区域（数组）用于向 server 导出 IO 信息，例如 IO 偏移、长度、
OP/标志以及缓冲区地址。每个 `ublksrv_io_desc` 实例可通过队列 id 和 IO tag 直接索引。

以下 IO 命令通过 io_uring 直通命令通信，每个命令仅用于转发 IO 以及提交命令数据中
指定 IO tag 的结果：

#### 传统按 I/O 命令


- `UBLK_U_IO_FETCH_REQ`

  从 server 的 I/O pthread 发送，用于获取发往 `/dev/ublkb*` 的未来传入 I/O 请求。该
  命令仅由 server 的 IO pthread 发送一次，以便 ublk 驱动建立 IO 转发环境。

  一旦某线程针对给定的 (qid,tag) 对发出此命令，该线程就注册为该 I/O 的守护进程
  （daemon）。今后，只有该 I/O 的守护进程才被允许针对该 I/O 发出命令。如果任何其他
  线程试图针对一个其并非守护进程的 (qid,tag) 对发出命令，该命令将失败。守护进程只能
  通过恢复（recovery）来重置。

  每个 (qid,tag) 对都能拥有各自独立的守护进程任务的能力，由 `UBLK_F_PER_IO_DAEMON`
  特性指示。如果驱动不支持该特性，则守护进程必须是按队列的——即与单个 qid 关联的所有
  I/O 必须由同一任务处理。

- `UBLK_U_IO_COMMIT_AND_FETCH_REQ`

  当某个 IO 请求发往 `/dev/ublkb*` 时，驱动将该 IO 的 `ublksrv_io_desc` 存入指定的
  映射区域；随后，此 IO tag 先前收到的 IO 命令（无论是 `UBLK_IO_FETCH_REQ` 还是
  `UBLK_IO_COMMIT_AND_FETCH_REQ`）完成，于是 server 通过 io_uring 获得 IO 通知。

  server 处理完 IO 后，其处理结果是通过将 `UBLK_IO_COMMIT_AND_FETCH_REQ` 发回给驱动
  来提交回去的。一旦 ublkdrv 收到此命令，它解析结果并完成对 `/dev/ublkb*` 的请求。
  同时建立环境以用相同的 IO tag 获取未来的请求。也就是说，`UBLK_IO_COMMIT_AND_FETCH_REQ`
  被复用于获取请求与提交回 IO 结果两件事。

- `UBLK_U_IO_NEED_GET_DATA`

  在启用 `UBLK_F_NEED_GET_DATA` 时，WRITE 请求将首先在不拷贝数据的情况下下发给 ublk
  server。然后，ublk server 的 IO 后端收到该请求，它可以分配数据缓冲区并将其地址嵌入
  这个新的 io 命令中。内核驱动收到该命令后，将执行从请求页到此后端缓冲区的拷贝。最后，
  后端再次收到带待写入数据的请求，它就能真正处理该请求。

  `UBLK_IO_NEED_GET_DATA` 增加了一趟额外的往返和一个 io_uring_enter() 系统调用。任何
  认为这会降低性能的用户都不应启用 UBLK_F_NEED_GET_DATA。默认情况下，ublk server 为每个
  IO 预分配 IO 缓冲区。任何新项目都应尝试使用此缓冲区来与 ublk 驱动通信。不过，现有项目
  可能会被破坏，或无法使用新的缓冲区接口；这就是为什么添加此命令是为了向后兼容，使现有
  项目仍能使用现有缓冲区。

- ublk server IO 缓冲区与 ublk 块 IO 请求之间的数据拷贝

  在通知 server 即将到来的 IO 之前，驱动需要先将块 IO 请求页拷贝到 server 缓冲区（页）
  中（针对 WRITE），以便 server 能够处理 WRITE 请求。

  当 server 处理 READ 请求并发送 `UBLK_IO_COMMIT_AND_FETCH_REQ` 时，ublkdrv 需要将 server
  缓冲区（页）中读取的内容拷贝到 IO 请求页中。

#### 批量 I/O 命令（UBLK_F_BATCH_IO）


`UBLK_F_BATCH_IO` 特性提供了一种替代的高性能 I/O 处理模型，它用按队列的批量命令替换
传统的按 I/O 命令。这显著减少了通信开销，并能在多个 server 任务间实现更好的负载均衡。

与传统模式的主要区别：

- **按队列 vs 按 I/O**：命令作用于队列而非单个 I/O
- **批量处理**：多个 I/O 在单次操作中被处理
- **多重触发（multishot）命令**：使用 io_uring multishot 以减少提交开销
- **灵活的任务分配**：任何任务都可处理任何 I/O（无按 I/O 的守护进程）
- **更好的负载均衡**：任务可动态调整其工作负载

批量 I/O 命令：

- `UBLK_U_IO_PREP_IO_CMDS`

  批量准备多个 I/O 命令。server 提供一个包含多个将被一起处理的 I/O 描述符的缓冲区。
  这减少了所需的单个命令提交数量。

- `UBLK_U_IO_COMMIT_IO_CMDS`

  批量提交多个 I/O 操作的结果，并准备好 I/O 描述符以接受新的请求。server 提供一个包含
  多个已完成 I/O 结果的缓冲区，从而允许高效地批量完成请求。

- `UBLK_U_IO_FETCH_IO_CMDS`

  用于批量获取 I/O 命令的**多重触发命令**。这是实现高性能批处理的关键命令：

  - 使用 io_uring multishot 能力以减少提交开销
  - 单个命令可在一段时间内获取多个 I/O 请求
  - 缓冲区大小决定每次操作的最大批大小
  - 可提交多个获取命令以实现负载均衡
  - 每个队列任意时刻只有一个获取命令处于活动状态
  - 支持跨多个 server 任务的动态负载均衡

  它是一个典型的带提供缓冲区的 multishot io_uring 请求，在触发任何失败之前不会被完成。

  每个任务可以提交具有不同缓冲区大小的 `UBLK_U_IO_FETCH_IO_CMDS` 来控制其处理的工作量。
  这使得多线程 server 中复杂的负载均衡策略成为可能。

迁移：使用传统命令（`UBLK_U_IO_FETCH_REQ`、`UBLK_U_IO_COMMIT_AND_FETCH_REQ`）的应用
无法同时使用批量模式。

### 零拷贝


ublk 零拷贝依赖于 io_uring 的固定内核缓冲区，它提供两个 API：`io_buffer_register_bvec()`
和 `io_buffer_unregister_bvec`。

ublk 添加了 `UBLK_IO_REGISTER_IO_BUF` IO 命令来调用 `io_buffer_register_bvec()`，以便
ublk server 将客户端请求缓冲区注册进 io_uring 缓冲区表，然后 ublk server 可用已注册的
缓冲区索引提交 io_uring 的 IO。`UBLK_IO_UNREGISTER_IO_BUF` IO 命令调用
`io_buffer_unregister_bvec()` 来注销缓冲区，该缓冲区保证在调用 `io_buffer_register_bvec()`
与 `io_buffer_unregister_bvec()` 之间一直存活。任何支持此类内核缓冲区的 io_uring 操作都会
持有该缓冲区的一个引用，直到操作完成。

实现零拷贝或用户拷贝的 ublk server 必须具有 CAP_SYS_ADMIN 且受信任，因为确保处理 read 命令
时 IO 缓冲区已填充数据、并在处理 READ 命令时向 ublk 驱动返回正确结果是 ublk server 的责任，
而且该结果必须与填充到 IO 缓冲区的字节数相符。否则，未初始化的内核 IO 缓冲区将被暴露给
客户端应用。

ublk server 需要使 `struct ublk_param_dma_align` 的参数与后端对齐，零拷贝才能正常工作。

为了达到最佳 IO 性能，ublk server 应使 `struct ublk_param_segment` 的段参数与后端对齐，
以避免不必要的 IO 拆分，这通常会有损 io_uring 性能。

### 自动缓冲区注册


`UBLK_F_AUTO_BUF_REG` 特性自动处理 I/O 请求的缓冲区注册与注销，这简化了缓冲区管理流程，并
减少了 ublk server 实现中的开销。

这是用于使用零拷贝的另一个特性标志，并且它与 `UBLK_F_SUPPORT_ZERO_COPY` 兼容。

#### 特性概述


该特性在将 I/O 命令递交给 ublk server 之前，自动将请求缓冲区注册到 io_uring 上下文，并在
完成 I/O 命令时注销它们。这消除了对通过 `UBLK_IO_REGISTER_IO_BUF` 与 `UBLK_IO_UNREGISTER_IO_BUF`
命令进行手动缓冲区注册/注销的需要，于是 ublk server 中的 IO 处理可以摆脱对这两个 uring_cmd
操作的依赖。

如果这些 IO 之间存在任何依赖，就不能向 io_uring 并发下发 IO。因此这种方式不仅简化了 ublk
server 的实现，还通过移除对缓冲区注册与注销命令的依赖，使并发 IO 处理成为可能。

#### 使用要求


1. ublk server 必须在用于 `UBLK_IO_FETCH_REQ` 与 `UBLK_IO_COMMIT_AND_FETCH_REQ` 的同一个
   `io_ring_ctx` 上创建稀疏缓冲区表。如果 uring_cmd 在不同的 `io_ring_ctx` 上发出，则需要
   手动注销缓冲区。

2. 缓冲区注册数据必须通过 uring_cmd 的 `sqe->addr` 传递，并且使用
```

    struct ublk_auto_buf_reg {
        __u16 index;      /* Buffer index for registration */
        __u8 flags;       /* Registration flags */
        __u8 reserved0;   /* Reserved for future use */
        __u32 reserved1;  /* Reserved for future use */
    };

   ublk_auto_buf_reg_to_sqe_addr() 用于将上面的结构转换为 ``sqe->addr``。

```
3. `ublk_auto_buf_reg` 中的所有保留字段必须清零。

4. 可选的标志可通过 `ublk_auto_buf_reg.flags` 传递。

#### 回退行为


如果自动缓冲区注册失败：

1. 当启用了 `UBLK_AUTO_BUF_REG_FALLBACK` 时：

   - uring_cmd 被完成
   - `UBLK_IO_F_NEED_REG_BUF` 被设置在 `ublksrv_io_desc.op_flags` 中
   - ublk server 必须手动处理该失败，例如手动注册缓冲区，或使用用户拷贝特性来获取数据以
     处理 ublk IO

2. 如果未启用回退：

   - ublk I/O 请求静默失败
   - uring_cmd 不会被完成

#### 限制


- 所有操作需要相同的 `io_ring_ctx`
- 在回退情形下可能需要手动缓冲区管理
- io_ring_ctx 缓冲区表的最大大小为 16K，在由单个 io_ring_ctx 处理过多 ublk 设备且每个设备
  队列深度很大时，可能不够用

### 共享内存零拷贝（UBLK_F_SHMEM_ZC）


`UBLK_F_SHMEM_ZC` 特性提供了一种替代的零拷贝路径，其工作原理是在客户端应用与 ublk server
之间共享物理内存页。与上述 io_uring 固定缓冲区方案不同，共享内存零拷贝不需要每个 I/O 都
进行 io_uring 缓冲区注册——相反，它依赖于内核在 I/O 时匹配物理页。这使得 ublk server 能够
直接访问共享缓冲区，而这在使用 io_uring 固定缓冲区方案时是不太可能的。

#### 动机


共享内存零拷贝采取了不同的方式：如果客户端应用与 ublk server 都映射了相同的物理内存，那就
没有什么需要拷贝的。内核会自动检测共享页，并告诉 server 数据已存在于何处。

`UBLK_F_SHMEM_ZC` 可被视作针对优化过的客户端应用的一项补充——当客户端愿意从共享内存分配
I/O 缓冲区时，整个数据路径就变成了零拷贝。

#### 用例


当客户端应用可被配置为对其 I/O 缓冲区使用特定的共享内存区域时，此特性很有用：

- **自定义存储客户端**：从共享内存（memfd、hugetlbfs）分配 I/O 缓冲区，并对 ublk 设备发起
  直接 I/O
- **数据库引擎**：使用带有 O_DIRECT 的预分配缓冲池

#### 工作原理


1. ublk server 与客户端都用 `MAP_SHARED` `mmap()` 同一个文件（memfd 或 hugetlbfs）。这让两个
   进程都能访问相同的物理页。

```

     struct ublk_shmem_buf_reg buf = { .addr = mmap_va, .len = size };
     ublk_ctrl_cmd(UBLK_U_CMD_REG_BUF, .addr = &buf);

   The kernel pins the pages and builds a PFN lookup tree.

```
3. 当客户端对 `/dev/ublkb*` 发起直接 I/O（`O_DIRECT`）时，内核通过比较 PFN 来检查 I/O 缓冲
   区页是否匹配任何已注册的页。

4. 匹配时，内核在 I/O 中设置 `UBLK_IO_F_SHMEM_ZC`
```

     if (iod->op_flags & UBLK_IO_F_SHMEM_ZC) {
         /* Data is already in our shared mapping — zero copy */
         index  = ublk_shmem_zc_index(iod->addr);
         offset = ublk_shmem_zc_offset(iod->addr);
         buf = shmem_table[index].mmap_base + offset;
     }

```
5. 如果页不匹配（例如客户端使用了非共享缓冲区），I/O 会静默回退到正常的拷贝路径。

共享内存可通过两种方法建立：

- **基于套接字**：客户端通过 unix 套接字上的 `SCM_RIGHTS` 向 ublk server 发送一个 memfd。
  server 映射并注册它。
- **基于 hugetlbfs**：两个进程 `mmap(MAP_SHARED)` 同一个 hugetlbfs 文件。无需 IPC——同一文件
  给出相同的物理页。

#### 优点


- **简单**：没有按 I/O 的缓冲区注册或注销命令。一旦共享缓冲区被注册，所有匹配的 I/O 都自动
  变为零拷贝。
- **直接缓冲区访问**：ublk server 可以通过自身的 mmap 直接读写共享缓冲区，而无需经过 io_uring
  固定缓冲区操作。这对 server 实现更友好。
- **快速**：PFN 匹配对每个 bvec 来说是一次 maple tree 查找。缓冲区管理没有 io_uring 命令的
  往返开销。
- **兼容**：不匹配的 I/O 静默回退到拷贝路径。该设备对任何客户端都能正常工作，在共享内存可用时
  零拷贝作为一种优化。

#### 限制


- **需要客户端配合**：客户端必须从其 I/O 缓冲区分配自共享内存区域。这需要一个自定义或经过配置的
  客户端——使用自身缓冲区的标准应用将无法受益。
- **仅直接 I/O**：缓冲 I/O（不带 `O_DIRECT`）会经过页缓存，页缓存会分配自己的页。这些内核分配的
  页永远不会匹配已注册的共享缓冲区。只有 `O_DIRECT` 才会将客户端的缓冲区页直接放入块 I/O 中。
- **仅连续数据**：每个 I/O 请求的数据必须在单个已注册缓冲区内部是连续的。跨越多个不相邻的已注册
  缓冲区的分散/聚集 I/O 不能使用零拷贝路径。

#### 控制命令


- `UBLK_U_CMD_REG_BUF`

  注册一个共享内存缓冲区。`ctrl_cmd.addr` 指向一个 `struct ublk_shmem_buf_reg`，其中包含缓冲区
  虚拟地址和大小。成功时返回所分配缓冲区索引（>= 0）。内核固定页并建立 PFN 查找树。队列冻结在
  内部处理。

- `UBLK_U_CMD_UNREG_BUF`

  注销先前注册的缓冲区。`ctrl_cmd.data[^0^]` 是缓冲区索引。解除固定页并从查找树中移除 PFN 条目。

## 参考资料
