
## Asynchronous Transfers/Transforms API



  1. 简介

  2 起源

  3 用法
  3.1 API 的一般格式
  3.2 支持的操作
  3.3 描述符管理
  3.4 操作何时执行？
  3.5 操作何时完成？
  3.6 约束
  3.7 示例

  4 DMAENGINE 驱动开发者注意事项
  4.1 一致性要点
  4.2 “我的应用需要对硬件通道的独占控制”

  5 源码

## 1. Introduction


async_tx API 提供了一组方法，用于描述一串异步批量内存传输/转换操作链，并支持事务间依赖。
它被实现为一个 dmaengine 客户端，屏蔽了不同硬件卸载引擎实现的细节。按照该 API 编写的代码
可以针对异步操作进行优化，而该 API 会将操作链适配到可用的卸载资源上。

## 2.Genealogy


该 API 最初设计用于使用 Intel(R) Xscale 系列 I/O 处理器中的卸载引擎，来卸载 md-raid5 驱动
的内存拷贝和 xor 奇偶校验计算。它也建立在“dmaengine”层之上，该层是为在网络栈中使用
Intel(R) I/OAT 引擎卸载内存拷贝而开发的。由此产生了以下设计特性：

1. 隐式同步路径：API 的用户无需知道他们所运行的平台是否具有卸载能力。当引擎可用时操作会被
   卸载，否则在软件中执行。
2. 跨通道依赖链：API 允许提交一串依赖操作，例如 raid5 情况下的 xor->copy->xor。API 会自动
   处理从一个操作过渡到另一个操作意味着硬件通道切换的情况。
3. 对 dmaengine 的扩展，以支持多个客户端以及“memcpy”之外的操作类型

## 3. Usage


### 3.1 General format of the API


```

  struct dma_async_tx_descriptor *
  async_<operation>(<op specific parameters>, struct async_submit_ctl *submit)

```
### 3.2 Supported operations


========  ====================================================================
memcpy    在源缓冲和目的缓冲之间进行内存拷贝
memset    用某个字节值填充目的缓冲
xor      对一系列源缓冲进行 xor 并将结果写入目的缓冲
xor_val   对一系列源缓冲进行 xor，如果结果为零则设置一个标志。实现会尽量避免写入内存
pq       从一系列源缓冲生成 p+q（raid6 校验码）
pq_val    验证 p 和/或 q 缓冲与给定的一系列源是否同步
datap    （raid6_datap_recov）从给定源中恢复一个 raid6 数据块和 p 块
2data    （raid6_2data_recov）从给定源中恢复 2 个 raid6 数据块
========  ====================================================================

### 3.3 Descriptor management


当操作已被排队以异步执行时，返回值为非 NULL，并指向一个“描述符”（descriptor）。描述符是
在卸载引擎驱动控制下被回收复用的资源，随着操作完成而被重用。当应用需要提交一串操作时，它
必须保证在依赖被提交之前描述符不会被自动回收。这要求所有描述符在被卸载引擎驱动允许回收（或
释放）之前，先被应用确认（acknowledge）。描述符可以通过以下任一方式被确认：

1. 如果没有要提交的子操作，则设置 ASYNC_TX_ACK 标志
2. 将一个未确认的描述符作为依赖提交给另一个 async_tx 调用，将隐式设置确认状态。
3. 在描述符上调用 async_tx_ack()。

### 3.4 When does the operation execute?


操作在从 async_<operation> 调用返回后不会立即发出。卸载引擎驱动会对操作进行批处理，以减少
管理通道所需的 mmio 周期数量，从而提高性能。一旦达到驱动特定的阈值，驱动会自动发出待处理的
操作。应用可以通过调用 async_tx_issue_pending_all() 强制触发该事件。它作用于所有通道，因为
应用不知道通道到操作的映射关系。

### 3.5 When does the operation complete?


应用可以通过两种方法了解操作的完成。

1. 调用 dma_wait_for_async_tx()。该调用使 CPU 在轮询操作完成的同时自旋。它处理依赖链并发出
   待处理操作。
2. 指定一个完成回调函数。如果卸载引擎驱动支持中断，该回调例程在 tasklet 上下文中运行；如果
   操作在软件中同步执行，则在应用上下文中调用。该回调可以在对 async_<operation> 的调用中设置，
   或者当应用需要提交长度未知的链时，可以使用 async_trigger_callback() 例程在链的末尾设置完成
   中断/回调。

### 3.6 Constraints


1. 不允许在 IRQ 上下文中调用 async_<operation>。只要不违反约束 #2，其他上下文是允许的。
2. 完成回调例程不能提交新操作。这在同步情况下会导致递归，在异步情况下会导致自旋锁被获取两次。

### 3.7 Example


执行一个 xor->copy->xor 操作，其中每个操作依赖于
```

    #include <linux/async_tx.h>

    static void callback(void *param)
    {
	    complete(param);
    }

    #define NDISKS  2

    static void run_xor_copy_xor(struct page **xor_srcs,
				 struct page *xor_dest,
				 size_t xor_len,
				 struct page *copy_src,
				 struct page *copy_dest,
				 size_t copy_len)
    {
	    struct dma_async_tx_descriptor *tx;
	    struct async_submit_ctl submit;
	    addr_conv_t addr_conv[NDISKS];
	    struct completion cmp;

	    init_async_submit(&submit, ASYNC_TX_XOR_DROP_DST, NULL, NULL, NULL,
			    addr_conv);
	    tx = async_xor(xor_dest, xor_srcs, 0, NDISKS, xor_len, &submit);

	    submit.depend_tx = tx;
	    tx = async_memcpy(copy_dest, copy_src, 0, 0, copy_len, &submit);

	    init_completion(&cmp);
	    init_async_submit(&submit, ASYNC_TX_XOR_DROP_DST | ASYNC_TX_ACK, tx,
			    callback, &cmp, addr_conv);
	    tx = async_xor(xor_dest, xor_srcs, 0, NDISKS, xor_len, &submit);

	    async_tx_issue_pending_all();

	    wait_for_completion(&cmp);
    }

```
有关这些标志的更多信息，请参阅 include/linux/async_tx.h。有关更多实现示例，请参阅
drivers/md/raid5.c 中的 ops_run_** 和 ops_complete_** 例程。

## 4. Driver Development Notes


### 4.1 Conformance points


dmaengine 驱动需要符合若干一致性要点，以适应使用 async_tx API 的应用所做的假设：

1. 完成回调预期在 tasklet 上下文中发生
2. dma_async_tx_descriptor 字段绝不能在 IRQ 上下文中被操作
3. 在描述符清理路径中使用 async_tx_run_dependencies() 来处理依赖操作的提交

### 4.2 "My application needs exclusive control of hardware channels"


这一要求主要出现在 DMA 引擎驱动被用于支持设备到内存操作的情况。由于许多平台特定的原因，
执行这些操作的通道不能被共享。针对这些情况提供了 dma_request_channel() 接口。

```

  struct dma_chan *dma_request_channel(dma_cap_mask_t mask,
				       dma_filter_fn filter_fn,
				       void *filter_param);

```
```

  typedef bool (*dma_filter_fn)(struct dma_chan *chan, void *filter_param);

```
当可选的 'filter_fn' 参数为 NULL 时，dma_request_channel 简单地返回满足能力掩码的第一个
通道。否则，当掩码参数不足以指定所需通道时，可以使用 filter_fn 例程来调度系统中的可用通道。
filter_fn 例程对系统中每个空闲通道调用一次。看到合适的通道时，filter_fn 返回 DMA_ACK，将该
通道标记为 dma_request_channel 的返回值。通过该接口分配的通道在调用 dma_release_channel()
之前对调用者是独占的。

DMA_PRIVATE 能力标志用于标记不应被通用分配器使用的 dma 设备。如果已知某个通道将始终是私有的，
可以在初始化时设置它。或者，当 dma_request_channel() 找到一个未使用的“公共”通道时设置它。

实现驱动和使用者时需要注意几点：

1. 一旦一个通道被私有分配，即使调用了 dma_release_channel()，通用分配器也不会再考虑它。
2. 由于能力是在设备级别指定的，具有多个通道的 dma_device 要么所有通道都是公共的，要么所有
   通道都是私有的。

### 5. Source


include/linux/dmaengine.h:
    DMA 驱动和 api 用户的核心头文件
drivers/dma/dmaengine.c:
    卸载引擎通道管理例程
drivers/dma/:
    卸载引擎驱动的存放位置
include/linux/async_tx.h:
    async_tx api 的核心头文件
crypto/async_tx/async_tx.c:
    async_tx 到 dmaengine 的接口及公共代码
crypto/async_tx/async_memcpy.c:
    拷贝卸载
crypto/async_tx/async_xor.c:
    xor 及 xor 零和卸载
