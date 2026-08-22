
## io_uring 零拷贝接收（Rx

## 简

io_uring 零拷贝接收（ZC Rx）是一项在网络接收路径上消除内核到用户拷贝的特性，允许数据包数据被直接接收到用户空间内存中。该特性与 TCP_ZEROCOPY_RECEIVE 的不同之处在于，没有严格的对齐要求，也不需mmap()/munmap()。与 DPDK 等内核旁路方案相比，数据包头由内TCP 栈正常处理
## NIC 硬件需

io_uring ZC Rx 工作需要若NIC 硬件特性。目前内API 不会配置 NIC，必须由用户来完成
### 数据分离


需要在 L4 边界将数据包拆分为头部与负载。头部像往常一样被接收到内核内存中，并TCP 栈正常处理。负载被直接接收到用户空间内存中
### 流导

为此特性配置了特定的硬Rx 队列，但现代 NIC 通常将流分布到所有硬Rx 队列上。需要流导向（flow steering）来确保只有期望的流被导向到io_uring ZC Rx 配置的硬件队列
### RSS


除了上面的流导向之外，还需RSS 来将所有其他非零拷贝流从为 io_uring ZC Rx 配置的队列上引开
## 用法


### 配置 NIC


目前必须在带外完成
```

  ethtool -L eth0 combined 2

```
```

  ethtool -G eth0 tcp-data-split on

```
```

  ethtool -X eth0 equal 1

```
```

  ethtool -N eth0 flow-type tcp6 ... action 1

```
### 配置 io_uring


本节描述底层io_uring 内核 API。关于如何使用高API，请参liburing 文档
```

  IORING_SETUP_SINGLE_ISSUER
  IORING_SETUP_DEFER_TASKRUN
  IORING_SETUP_CQE32 or IORING_SETUP_CQE_MIXED

```
### 创建内存区域


```

  void *area_ptr = mmap(NULL, area_size,
                        PROT_READ | PROT_WRITE,
                        MAP_ANONYMOUS | MAP_PRIVATE,
                        0, 0);

```
### 创建补充

```

  void *ring_ptr = mmap(NULL, ring_size,
                        PROT_READ | PROT_WRITE,
                        MAP_ANONYMOUS | MAP_PRIVATE,
                        0, 0);

```
该补充环由头部的一些空间，加上一个数组组```

  size_t rq_entries = 4096;
  size_t ring_size = rq_entries * sizeof(struct io_uring_zcrx_rqe) + PAGE_SIZE;
  /* align to page size */
  ring_size = (ring_size + (PAGE_SIZE - 1)) & ~(PAGE_SIZE - 1);

```
### 注册 ZC Rx


```

  struct io_uring_zcrx_area_reg area_reg = {
    .addr = (__u64)(unsigned long)area_ptr,
    .len = area_size,
    .flags = 0,
  };

  struct io_uring_region_desc region_reg = {
    .user_addr = (__u64)(unsigned long)ring_ptr,
    .size = ring_size,
    .flags = IORING_MEM_REGION_TYPE_USER,
  };

  struct io_uring_zcrx_ifq_reg reg = {
    .if_idx = if_nametoindex("eth0"),
    /* this is the HW queue with desired flow steered into it */
    .if_rxq = 1,
    .rq_entries = rq_entries,
    .area_ptr = (__u64)(unsigned long)&area_reg,
    .region_ptr = (__u64)(unsigned long)&region_reg,
  };

```
```

  io_uring_register_ifq(ring, &reg);

```
### 映射补充

内核在注册时为补充环填充字段，注``struct
```

  struct io_uring_zcrx_rq refill_ring;

  refill_ring.khead = (unsigned *)((char *)ring_ptr + reg.offsets.head);
  refill_ring.khead = (unsigned *)((char *)ring_ptr + reg.offsets.tail);
  refill_ring.rqes =
    (struct io_uring_zcrx_rqe *)((char *)ring_ptr + reg.offsets.rqes);
  refill_ring.rq_tail = 0;
  refill_ring.ring_ptr = ring_ptr;

```
### 接收数据


```

  struct io_uring_sqe *sqe;

  sqe = io_uring_get_sqe(ring);
  io_uring_prep_rw(IORING_OP_RECV_ZC, sqe, fd, NULL, 0, 0);
  sqe->ioprio |= IORING_RECV_MULTISHOT;

```
```

  io_uring_submit_and_wait(ring, 1);

```
```

  struct io_uring_cqe *cqe;
  unsigned int count = 0;
  unsigned int head;

  io_uring_for_each_cqe(ring, head, cqe) {
    struct io_uring_zcrx_cqe *rcqe = (struct io_uring_zcrx_cqe *)(cqe + 1);

    unsigned long mask = (1ULL << IORING_ZCRX_AREA_SHIFT) - 1;
    unsigned char *data = area_ptr + (rcqe->off & mask);
    /* do something with the data */

    count++;
  }
  io_uring_cq_advance(ring, count);

```
### 回收缓冲

```

  struct io_uring_zcrx_rqe *rqe;
  unsigned mask = refill_ring.ring_entries - 1;
  rqe = &refill_ring.rqes[refill_ring.rq_tail & mask];

  unsigned long area_offset = rcqe->off & ~IORING_ZCRX_AREA_MASK;
  rqe->off = area_offset | area_reg.rq_area_token;
  rqe->len = cqe->res;
  IO_URING_WRITE_ONCE(*refill_ring.ktail, ++refill_ring.rq_tail);

```
### 区域分块


zcrx 将内存区域拆分为固定长度、物理上连续的块。这限制了单io_uring CQE 中返回的最大缓冲区大小。用户可以通过在注册期间将 `struct io_uring_zcrx_ifq_reg` `rx_buf_len` 字段设置为期望的长度，向内核提供使用更大块的提示。如果该字段被设置为零，内核默认使用系统页大小
要使用更大的尺寸，内存区域必须由物理上连续的、大小是 `rx_buf_len` 整数倍的范围作为后备。它还需要内核与硬件支持。如果注册失败，用户一般应通过将其 `rx_buf_len` 设置为零来回退到默认值
更大的块不会CQE 中返回的缓冲区大小提供任何额外保证，并且它们可能因流量模式、硬件卸载等许多因素而变化。除zcrx 注册之外，它不需要应用程序做任何更改
## 测试


参见 `tools/testing/selftests/drivers/net/hw/iou-zcrx.c`
