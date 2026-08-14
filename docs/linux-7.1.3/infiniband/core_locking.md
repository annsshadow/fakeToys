## InfiniBand 中间层加锁


  本指南试图明确 InfiniBand 中间层所做出的加锁假设。它描述了对位于中间层之下的底层驱动以及使用中间层的上层协议的要求。

## 睡眠与中断上下文


  除以下例外，底层驱动对 struct ib_device 中所有方法的实现都可能睡眠。例外是来自下列列表的任何方法：

    - create_ah
    - modify_ah
    - query_ah
    - destroy_ah
    - post_send
    - post_recv
    - poll_cq
    - req_notify_cq

  这些方法不能睡眠，并且必须可以从任何上下文调用。

  导出给上层协议使用者的相应函数：

    - rdma_create_ah
    - rdma_modify_ah
    - rdma_query_ah
    - rdma_destroy_ah
    - ib_post_send
    - ib_post_recv
    - ib_req_notify_cq

  因此可以安全地从任何上下文调用。

  此外，函数

    - ib_dispatch_event

  由底层驱动用来通过中间层派发异步事件，也可以安全地从任何上下文调用。

### 可重入性


  底层驱动导出的 struct ib_device 中的所有方法都必须完全可重入。底层驱动需要执行所有必要的同步以保持一致性，即使使用同一对象的多个函数调用同时运行也是如此。

  IB 中间层不对函数调用执行任何串行化。

  由于底层驱动是可重入的，上层协议使用者不需要执行任何串行化。然而，为了获得合理的结果，可能需要一些串行化。例如，使用者可以安全地在多个 CPU 上同时调用 ib_poll_cq()。但是，不同 ib_poll_cq() 调用之间的工作完成信息的顺序并未定义。

### 回调


  底层驱动不得在与 ib_device 方法调用相同的调用链中直接执行回调。例如，底层驱动不允许从其 post_send 方法中直接调用使用者的完成事件处理程序。相反，底层驱动应通过例如调度一个 tasklet 来执行回调，从而推迟该回调。

  底层驱动负责确保同一 CQ 的多个完成事件处理程序不会被同时调用。驱动必须保证对于给定的 CQ，任一时刻只有一个 CQ 事件处理程序在运行。换句话说，
```

          CPU1                                    CPU2

    low-level driver ->
      consumer CQ event callback:
        /* ... */
        ib_req_notify_cq(cq, ...);
                                          low-level driver ->
        /* ... */                           consumer CQ event callback:
                                              /* ... */
        return from CQ event handler

  The context in which completion event and asynchronous event
  callbacks run is not defined.  Depending on the low-level driver, it
  may be process context, softirq context, or interrupt context.
  Upper level protocol consumers may not sleep in a callback.

```
### 热插拔


  底层驱动在调用 ib_register_device() 时向使用者宣告设备已可供使用，所有初始化必须在此调用之前完成。设备必须保持可用，直到驱动的 ib_unregister_device() 调用返回。

  底层驱动必须从进程上下文调用 ib_register_device() 和 ib_unregister_device()。它不能持有任何可能在使用者通过这些调用回调到驱动时导致死锁的信号量。

  一旦为其调用了 struct ib_client 的 add 方法，上层协议使用者就可以开始使用该 IB 设备。使用者必须在从 remove 方法返回之前完成所有清理并释放与该设备相关的所有资源。

  使用者可以在其 add 和 remove 方法中睡眠。
