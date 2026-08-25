## 用户空间 verbs 访问


  ib_uverbs 模块由启CONFIG_INFINIBAND_USER_VERBS 编译得到，它通过“verbs”实现对 IB 硬件的直接用户空间访问，如《InfiniBand 体系结构规范》第 11 章所述
  要使verbs，需要使用可https://github.com/linux-rdma/rdma-core 获取libibverbs 库。libibverbs 包含一个用于使ib_uverbs 接口的设备无API。libibverbs 还需要针对你InfiniBand 硬件的相应设备相关内核驱动与用户空间驱动。例如，要使Mellanox HCA，你需要安ib_mthca 内核模块libmthca 用户空间驱动
## 用户空间与内核的通信


  用户空间通过 /dev/infiniband/uverbsN 字符设备与内核通信，用于慢速路径与资源管理操作。快速路径操作通常是通过直接写入 mmap() 到用户空间的硬件寄存器来完成的，不涉及系统调用，也不发生到内核的上下文切换
  命令通过这些设备文件上的 write() 发送给内核。ABI 定义drivers/infiniband/include/ib_user_verbs.h 中。需要内核响应的命令结构体包含一64 位字段，用于传递指向输出缓冲区的指针。状态作write() 系统调用的返回值返回给用户空间
## 资源管理


  由于所IB 资源的创建与销毁都是通过文件描述符传递的命令完成的，内核可以跟踪哪些资源附加到给定的用户空间上下文。ib_uverbs 模块维护 idr 表，用于在内核指针与不透明的用户空间句柄之间进行转换，从而内核指针永远不会暴露给用户空间，用户空间也无法欺骗内核去跟随一个伪造的指针
  这也允许内核在进程退出时进行清理，并防止一个进程触碰另一个进程的资源
## 内存固定（Memory pinning

  直接的用户空I/O 要求可能成为 I/O 目标的存储区域驻留在相同的物理地址上。ib_uverbs 模块通过 get_user_pages() put_page() 调用来管理存储区域的固定（pinning）与解除固定（unpinning）。它还会统计进程pinned_vm 里被固定的内存数量，并检查非特权进程未超过其 RLIMIT_MEMLOCK 限制
  被多次固定的页在每次固定时都会被计数，因pinned_vm 的值可能会高估一个进程所固定的页数
## /dev 文件


  要自动创建相应的字符设备文件，可以使
```
    KERNEL=="uverbs*", NAME="infiniband/%k"
```

  这将创建设备节点，其名称:

    /dev/infiniband/uverbs0

  以此类推。由InfiniBand 用户空间 verbs 应可供非特权进程安全使用，在 udev 规则中添加适当MODE GROUP 可能是有用的