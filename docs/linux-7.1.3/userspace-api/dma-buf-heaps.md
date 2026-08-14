
## 使用 heaps 分配 dma-buf


Dma-buf Heaps 是用户空间分配 dma-buf 对象的一种方式。它们通常用于从特定的分配池中分配缓冲区，或在框架之间共享缓冲区。

## Heaps


一个 heap 代表一个特定的分配器。Linux 内核目前支持以下 heaps：

 - `system` heap 分配虚拟连续、可缓存的缓冲区。

 - `system_cc_shared` heap 分配虚拟连续、可缓存的缓冲区，使用共享（已解密）内存。它仅出现在内存加密处于活动状态的机密计算（CoCo）VM 上（例如 AMD SEV、Intel TDX）。所分配的页清除了加密位，使其在不支持 TDISP 的情况下也能被设备 DMA 访问。在非 CoCo 的 VM 配置下，该 heap 不被注册。

 - `default_cma_region` heap 分配物理连续、可缓存的缓冲区。仅当存在 CMA 区域时才出现。这样的区域通常或者通过内核命令行中的 `cma` 参数创建，或者通过设置了 `linux,cma-default` 属性的内存区域 Device-Tree 节点创建，或者通过 `CMA_SIZE_MBYTES` 或 `CMA_SIZE_PERCENTAGE` Kconfig 选项创建。在 Linux 6.17 之前，其名称不稳定，根据平台可能称为 `reserved`、`linux,cma` 或 `default-pool`。

 - 将为设备树中每个带有 `shared-dma-pool` 兼容属性的可复用区域创建一个 heap，使用完整的设备树节点名作为其名称。缓冲区的语义与 `default-cma-region` 相同。

## 命名约定


`dma-buf` heap 的名称应满足若干约束：

- 名称必须稳定，不能从一个版本变到另一个版本。用户空间通过名称来识别 heaps，因此如果名称发生变化，我们很可能会引入回归。

- 名称必须描述该 heap 将从中分配的 memory 区域，并且必须在给定平台上唯一标识它。由于用户空间应用程序使用 heap 名称作为判别依据，因此当存在多个 heap 时，它必须能够可靠地分辨它想使用哪一个。

- 名称不得提及实现细节，例如分配器。heap 驱动会随着时间发生变化，而引入它时的实现细节在未来可能不再相关。

- 名称应当描述将要分配的缓冲区的属性。这样做会让用户空间更容易识别 heap。这类属性包括：

  - `contiguous`，用于物理连续的缓冲区；

  - `protected`，用于操作系统不可访问的已加密缓冲区；

- 名称可以描述预期用途。这样做会让用户空间应用程序与用户更容易识别 heap。

例如，假设一个平台有一个位于 RAM 地址 0x42000000 的保留内存区域，用于分配视频帧缓冲，物理连续，并由 CMA 内核分配器支持，那么好的名称是 `memory@42000000-contiguous` 或 `video@42000000`，而 `cma-video` 则不是。
