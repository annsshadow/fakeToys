
## I/O 请求处理


hypervisor 构造的 User VM I/O 请求，会ACRN Hypervisor Service Module
分发给与I/O 请求地址范围相对应的 I/O client。I/O 请求处理的细节在以下各节描述
### 1. I/O 请求


对于每个 User VM，存在一个共享的 4-KByte 内存区域，用hypervisor Service VM
之间I/O 请求通信。一I/O 请求是一256 字节的结构体缓冲区，即由 hypervisor
I/O handler User VM 中发生被捕获I/O 访问时填充的 'struct acrn_io_request'Service VM 中的 ACRN 用户态程序首先分配一4-KByte 页，并将该缓冲区GPA（Guest
Physical Address，客户机物理地址）传递给 hypervisor。该缓冲区被用作包含 16 I/O 请求槽的数组，每I/O 请求槽为 256 字节。该数组vCPU ID 作为索引
### 2. I/O clients


一I/O client 负责处理被访GPA 落在特定范围内的 User VM I/O 请求。每User VM
可关联多I/O client。每User VM 还关联一个特殊的 client，称default client它处理所有不属于任何其他 client 范围内的 I/O 请求。ACRN 用户态程序充当每User VM
鐨?default client銆。
下图展示I/O 请求共享缓冲区、I/O 请求以及 I/O client 之间的关系
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
### 3. I/O 请求状态转

一ACRN I/O 请求的状态转换如下
```

   FREE -> PENDING -> PROCESSING -> COMPLETE -> FREE -> ...

```
- FREE：该 I/O 请求槽为- PENDING：该槽中有一个有效的 I/O 请求正在等待处理
- PROCESSING：该 I/O 请求正在被处- COMPLETE：该 I/O 请求已被处理

处于 COMPLETE FREE 状态的 I/O 请求hypervisor 拥有。HSM ACRN 用户态程负责处理其余状态的请求
### 4. I/O 请求的处理流

a. User VM 中发生被捕获I/O 访问时，hypervisor I/O handler 会将一I/O
   请求填充PENDING 状态b. hypervisor Service VM 发起一upcall（即通知中断）c. upcall handler 调度一worker 来分I/O 请求d. worker 查找处于 PENDING 状态的 I/O 请求，根I/O 访问的地址将它们分配给不同   已注client，将其状态更新为 PROCESSING，并通知相应client 进行处理e. 被通知client 处理所分配I/O 请求f. HSM I/O 请求状态更新为 COMPLETE，并通过 hypercall 通知 hypervisor 处理完成