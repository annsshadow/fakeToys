
## I/O 请求处理


由 hypervisor 构造的 User VM 的 I/O 请求，会由 ACRN Hypervisor Service Module
分发给与该 I/O 请求地址范围相对应的 I/O client。I/O 请求处理的细节在以下各节中
描述。

### 1. I/O 请求


对于每个 User VM，存在一个共享的 4-KByte 内存区域，用于 hypervisor 与 Service VM
之间的 I/O 请求通信。一个 I/O 请求是一个 256 字节的结构体缓冲区，即由 hypervisor
的 I/O handler 在 User VM 中发生被捕获的 I/O 访问时填充的 'struct acrn_io_request'。
Service VM 中的 ACRN 用户态程序首先分配一个 4-KByte 页，并将该缓冲区的 GPA（Guest
Physical Address，客户机物理地址）传递给 hypervisor。该缓冲区被用作包含 16 个
I/O 请求槽的数组，每个 I/O 请求槽为 256 字节。该数组以 vCPU ID 作为索引。

### 2. I/O clients


一个 I/O client 负责处理被访问 GPA 落在特定范围内的 User VM I/O 请求。每个 User VM
可关联多个 I/O client。每个 User VM 还关联一个特殊的 client，称为 default client，
它处理所有不属于任何其他 client 范围内的 I/O 请求。ACRN 用户态程序充当每个 User VM
的 default client。

下图展示了 I/O 请求共享缓冲区、I/O 请求以及 I/O client 之间的关系。

```

     +------------------------------------------------------+
     |                                       Service VM     |
     |+--------------------------------------------------+  |
     ||      +----------------------------------------+  |  |
     ||      | shared page            ACRN userspace  |  |  |
     ||      |    +-----------------+  +------------+ |  |  |
     ||   +----+->| acrn_io_request |<-+  default   | |  |  |
     ||   |  | |  +-----------------+  | I/O client | |  |  |
     ||   |  | |  |       ...       |  +------------+ |  |  |
     ||   |  | |  +-----------------+                 |  |  |
     ||   |  +-|--------------------------------------+  |  |
     ||---|----|-----------------------------------------|  |
     ||   |    |                             kernel      |  |
     ||   |    |            +----------------------+     |  |
     ||   |    |            | +-------------+  HSM |     |  |
     ||   |    +--------------+             |      |     |  |
     ||   |                 | | I/O clients |      |     |  |
     ||   |                 | |             |      |     |  |
     ||   |                 | +-------------+      |     |  |
     ||   |                 +----------------------+     |  |
     |+---|----------------------------------------------+  |
     +----|-------------------------------------------------+
          |
     +----|-------------------------------------------------+
     |  +-+-----------+                                     |
     |  | I/O handler |              ACRN Hypervisor        |
     |  +-------------+                                     |
     +------------------------------------------------------+

```
### 3. I/O 请求状态转换


一个 ACRN I/O 请求的状态转换如下。

```

   FREE -> PENDING -> PROCESSING -> COMPLETE -> FREE -> ...

```
- FREE：该 I/O 请求槽为空
- PENDING：该槽中有一个有效的 I/O 请求正在等待处理
- PROCESSING：该 I/O 请求正在被处理
- COMPLETE：该 I/O 请求已被处理

处于 COMPLETE 或 FREE 状态的 I/O 请求由 hypervisor 拥有。HSM 与 ACRN 用户态程序
负责处理其余状态的请求。

### 4. I/O 请求的处理流程


a. 当 User VM 中发生被捕获的 I/O 访问时，hypervisor 的 I/O handler 会将一个 I/O
   请求填充为 PENDING 状态。
b. hypervisor 向 Service VM 发起一个 upcall（即通知中断）。
c. upcall handler 调度一个 worker 来分发 I/O 请求。
d. worker 查找处于 PENDING 状态的 I/O 请求，根据 I/O 访问的地址将它们分配给不同的
   已注册 client，将其状态更新为 PROCESSING，并通知相应的 client 进行处理。
e. 被通知的 client 处理所分配的 I/O 请求。
f. HSM 将 I/O 请求状态更新为 COMPLETE，并通过 hypercall 通知 hypervisor 处理完成。
