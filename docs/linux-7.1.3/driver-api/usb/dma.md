#### USB DMA


在 Linux 2.5 内核（及更高版本）中，USB 设备驱动对如何使用 DMA 来执行 I/O 操作有了
更多的控制。这些 API 在内核 USB 编程指南（kerneldoc，来自源代码）中有详细说明。

## API 概览


总体情况是，USB 驱动可以继续忽略大多数 DMA 问题，尽管它们仍然必须提供 DMA 就绪的
缓冲区（参见 Documentation/core-api/dma-api-howto.rst）。这就是它们在 2.4（及更早）
内核中的工作方式，或者它们现在也可以感知 DMA。

感知 DMA 的 USB 驱动：

- 新增的调用使感知 DMA 的驱动能够分配 dma 缓冲区，并为已有的 dma 就绪缓冲区管理
  dma 映射（见下文）。

- URB 有一个额外的 "transfer_dma" 字段，以及一个指示其是否有效的 transfer_flags
  位。（控制请求也有 "setup_dma"，但驱动不得使用它。）

- 如果感知 DMA 的驱动没有抢先完成映射并设置 `URB_NO_TRANSFER_DMA_MAP`，则
  "usbcore" 会映射此 DMA 地址。HCD 不为 URB 管理 dma 映射。

- 有一个新的“通用 DMA API”，其中部分可供 USB 设备驱动使用。绝不要在任何 USB 接口
  或设备上使用 dma_set_mask()；那可能会破坏共享该总线的所有设备。

## 消除拷贝


避免让 CPU 不必要地拷贝数据是好事。代价会累积，而像缓存颠簸（cache-trashing）这类
影响会施加微妙的惩罚。

- 如果你一直从同一个缓冲区进行大量小数据传输，在使用 IOMMU 管理 DMA 映射的系统上，
  这真的会消耗大量资源。与执行 I/O 相比，为每个请求建立和拆除 IOMMU 映射的代价可能
  要高得多！

  对于这些特定情况，USB 提供了分配开销更低的内存的原语。它们的工作方式类似于
  kmalloc 和 kfree 版本，为你提供可存入 urb->transfer_buffer 和 urb->transfer_dma
  的正确类型的地址。
```

	void *usb_alloc_coherent (struct usb_device *dev, size_t size,
		int mem_flags, dma_addr_t *dma);

	void usb_free_coherent (struct usb_device *dev, size_t size,
		void *addr, dma_addr_t dma);

  大多数驱动**不应**使用这些原语；它们不需要使用这类内存（“dma-coherent”），而从
  :c:func:`kmalloc` 返回的内存也能正常工作。

  返回的内存缓冲区是“dma-coherent”的；有时你可能需要通过使用内存屏障来强制一致的
  内存访问顺序。它没有使用流式（streaming）DMA 映射，因此适用于在否则会颠簸 IOMMU
  映射的系统上进行小传输。（有关“coherent”和“streaming”DMA 映射的定义，请参阅
  Documentation/core-api/dma-api-howto.rst。）

  申请 1/N 页（以及申请 N 页）在空间上是相当高效的。

  在大多数系统上，返回的内存将是未缓存的，因为 dma-coherent 内存的语义要求要么绕过
  CPU 缓存，要么使用带有总线侦听（bus-snooping）支持的缓存硬件。虽然 x86 硬件具有
  这种总线侦听能力，但许多其他系统使用软件来刷新缓存行以防止 DMA 冲突。

```
- 某些 EHCI 控制器上的设备可以处理对高端内存（high memory）的 DMA 输入输出。

  遗憾的是，当前的 Linux DMA 基础设施没有合理的方式来暴露这些能力……而且无论如何，
  HIGHMEM 在很大程度上是 x86_32 特有的一个设计缺陷。所以你最好的办法是确保绝不将
  高端内存缓冲区传入 USB 驱动。这很容易；它是默认行为。只是不要覆盖它，例如使用
  `NETIF_F_HIGHDMA`。

  这可能会迫使你的调用者做一些反弹缓冲（bounce buffering），从高端内存复制到“普通”
  DMA 内存。如果你能想出解决此问题（针对内存超过 1 GByte 的 x86_32 机器）的好办法，
  欢迎提交补丁。

## 使用已有缓冲区


已有缓冲区在首先被映射到设备的 DMA 地址空间之前，不能用于 DMA。然而，传递给你的
驱动的大多数缓冲区都可以安全地用于这样的 DMA 映射。（请参阅
Documentation/core-api/dma-api-howto.rst 的第一节，标题为“哪些内存可用于 DMA？”）

- 当你拥有已为 USB 控制器映射好的 scatterlist 时，可以使用新的 `usb_sg_*()` 调用，
  它会将 scatterlist 转换为
```

	int usb_sg_init(struct usb_sg_request *io, struct usb_device *dev,
		unsigned pipe, unsigned	period, struct scatterlist *sg,
		int nents, size_t length, gfp_t mem_flags);

	void usb_sg_wait(struct usb_sg_request *io);

	void usb_sg_cancel(struct usb_sg_request *io);

  当 USB 控制器不支持 DMA 时，只要 scatterlist 中的页不在 Highmem 中，``usb_sg_init()``
  就会尝试以 PIO 方式提交 URB，而在现代架构上这种情况非常罕见。

```
