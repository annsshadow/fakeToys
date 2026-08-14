
## PCI Peer-to-Peer DMA 支持


The PCI 总线 具有 pretty decent 支持 用于 performing DMA transfers
之间 two 设备 在 the 总线. 此 类型 的 transaction 是 henceforth
called Peer-to-Peer (或 P2P). 然而, 存在 一个 数字 的 issues 该
make P2P transactions tricky 到 执行 在 一个 perfectly safe way.

用于 PCIe the routing 的 Transaction Layer Packets (TLPs) 是 well-defined up
直到 它们 reach 一个 host bridge 或 root 端口. 若 the path 包含 PCIe switches
然后 基于 the ACS 设置 the transaction 可 route entirely 之内
the PCIe hierarchy 和 从不 reach the root 端口. The 内核 将 evaluate
the PCIe topology 和 始终 permit P2P 在 这些 well-defined cases.

然而, 若 the P2P transaction reaches the host bridge 然后 它 可能 具有 到
hairpin back out the 相同 root 端口, 为 routed inside the CPU SOC 到 another
PCIe root 端口, 或 routed internally 到 the SOC.

The PCIe specification doesn't 定义 the forwarding 的 transactions 之间
hierarchy domains 和 内核 defaults 到 blocking 此类 routing. 存在 一个
允许 列出 到 允许 detecting known-good HW, 在 其 case P2P 之间 任何
two PCIe 设备 将 为 permitted.

Since P2P inherently 是 doing transactions 之间 two 设备 它 需要 two
驱动 到 为 co-operating inside the 内核. The providing 驱动 具有 到 convey
其 MMIO 到 the consuming 驱动. 到 meet the 驱动 型号 lifecycle rules the
MMIO 必须 具有 全部 DMA 映射 removed, 全部 CPU accesses prevented, 全部 页
表 mappings undone 之前 the providing 驱动 completes remove().

此 需要 the providing 和 consuming 驱动 到 actively work together 到
guarantee 该 the consuming 驱动 具有 stopped 使用 the MMIO 期间 一个 removal
cycle. 这是 已完成 由 任一个 一个 synchronous invalidation shutdown 或 waiting
用于 全部 usage refcounts 到 reach zero.

在 the lowest level the P2P 子系统 offers 一个 naked 结构体 p2p_provider 该
delegates lifecycle 管理 到 the providing 驱动. 它是 expected 该
驱动 使用 此 选项 将 wrap 它们的 MMIO 内存 在 DMABUF 和 使用 DMABUF
到 提供 一个 invalidation shutdown. 这些 MMIO 地址 具有 无 结构体 页, 和
若 使用 与 mmap() 必须 创建 特殊 PTEs. 作为 此类 存在 very 少量
内核 uAPIs 该 可 accept 指针 到 them; 特别是 它们 cannot 为 使用
与 读取()/写入(), including O_DIRECT.

Building 在 此, the 子系统 offers 一个 layer 到 wrap the MMIO 在 一个 ZONE_设备
pgmap 的 内存_设备_PCI_P2PDMA 到 创建 结构体 页. The lifecycle 的
pgmap ensures 该 当 the pgmap 是 destroyed 全部 其他 驱动 具有 stopped
使用 the MMIO. 此 选项 works 与 O_DIRECT flows, 在 一些 cases, 若 the
underlying 子系统 supports handling 内存_设备_PCI_P2PDMA through
FOLL_PCI_P2PDMA. The 使用 的 FOLL_LONGTERM 是 prevented. 作为 此 relies 在 pgmap
它 也 relies 在 architecture 支持 along 与 alignment 和 最小 大小
limitations.


## 驱动 Writer's Guide


在 一个 given P2P implementation 那里 可 为 three 或 更多 不同
types 的 内核 驱动 在 play:

- Provider - 一个 驱动 其 提供 或 publishes P2P resources 类似
  内存 或 doorbell 寄存器 到 其他 驱动.
- Client - 一个 驱动 其 makes 使用 的 一个 resource 由 设置 up 一个
  DMA transaction 到 或 来自 它.
- Orchestrator - 一个 驱动 其 orchestrates the flow 的 数据 之间
  clients 和 providers.

在 许多 cases 那里 可以 为 overlap 之间 这些 three types (i.e.,
它 可 为 典型 用于 一个 驱动 到 为 两者 一个 provider 和 一个 client).

例如, 在 the NVMe Target Copy Offload implementation:

- The NVMe PCI 驱动 是 两者 一个 client, provider 和 orchestrator
  在 该 它 exposes 任何 CMB (控制器 内存 缓冲区) 作为 一个 P2P 内存
  resource (provider), 它 accepts P2P 内存 页 作为 缓冲区 在 requests
  到 为 使用 directly (client) 和 它 可 也 make 使用 的 the CMB 作为
  submission 队列 条目 (orchestrator).
- The RDMA 驱动 是 一个 client 在 此 arrangement 因此 该 一个 RNIC
  可 DMA directly 到 the 内存 exposed 由 the NVMe 设备.
- The NVMe Target 驱动 (nvmet) 可 orchestrate the 数据 来自 the RNIC
  到 the P2P 内存 (CMB) 和 然后 到 the NVMe 设备 (和 vice versa).

这是 currently the 仅 arrangement 受支持 由 the 内核 但
one 可以 imagine slight tweaks 到 此 该 将会 允许 用于 the 相同
functionality. 例如, 若 一个 特定 RNIC added 一个 BAR 与 一些
内存 behind 它, 其 驱动 可以 add 支持 作为 一个 P2P provider 和
然后 the NVMe Target 可以 使用 the RNIC's 内存 而非 the CMB
在 cases 何处 the NVMe 卡 在 使用 执行 不 具有 CMB 支持.


### Provider 驱动


一个 provider simply needs 到 注册 一个 BAR (或 一个 portion 的 一个 BAR)
作为 一个 P2P DMA resource 使用 `pci_p2pdma_add_resource()`.
此 将 注册 结构体 页 用于 全部 the specified 内存.

之后 该 它 可 optionally publish 全部 的 其 resources 作为
P2P 内存 使用 `pci_p2pmem_publish()`. 此 将 允许
任何 orchestrator 驱动 到 find 和 使用 the 内存. 当 marked 在
此 way, the resource 必须 为 regular 内存 与 无 side effects.

用于 the time 正在 这是 fairly rudimentary 在 该 全部 resources
是 typically going 到 为 P2P 内存. Future work 将 likely expand
此 到 包含 其他 types 的 resources 类似 doorbells.


### Client 驱动


一个 client 驱动 仅 具有 到 使用 the 映射 API `dma_map_sg()`
和 `dma_unmap_sg()` 函数 作为 usual, 和 the implementation
将 执行 the right thing 用于 the P2P capable 内存.


### Orchestrator 驱动


The 第一 task 一个 orchestrator 驱动 必须 执行 是 compile 一个 列出 的
全部 client 设备 该 将 为 involved 在 一个 given transaction. 用于
示例, the NVMe Target 驱动 creates 一个 列出 including the namespace
块 设备 和 the RNIC 在 使用. 若 the orchestrator 具有 access 到
一个 特定 P2P provider 到 使用 它 可 check compatibility 使用
`pci_p2pdma_distance()` 否则 它 可 find 一个 内存 provider
该's compatible 与 全部 clients 使用  `pci_p2pmem_find()`.
若 多于 one provider 是 受支持, the one nearest 到 全部 the clients 将
为 chosen 第一. 若 多于 one provider 是 一个 equal distance away, the
one returned 将 为 chosen 在 random (它是 不 一个 arbitrary 但
truly random). 此 函数 returns the PCI 设备 到 使用 用于 the provider
与 一个 参考 taken 和 因此 当 它's 无 longer needed 它 应当 为
returned 与 PCI_dev_put().

一旦 一个 provider 是 selected, the orchestrator 可 然后 使用
`pci_alloc_p2pmem()` 和 `pci_free_p2pmem()` 到
allocate P2P 内存 来自 the provider. `pci_p2pmem_alloc_sgl()`
和 `pci_p2pmem_free_sgl()` 是 convenience 函数 用于
allocating scatter-gather 列表 与 P2P 内存.

### 结构体 页 Caveats


同时 the 内存_设备_PCI_P2PDMA 页 可 为 installed 在 VMAs,
pin_用户_页() 和 related 将 不 return them 除非 FOLL_PCI_P2PDMA 是 set.

The 内存_设备_PCI_P2PDMA 页 需要 care 到 支持 在 the 内核. The
KVA 是 仍然 MMIO 和 必须 仍然 为 accessed through the 正常
readX()/writeX()/等 helpers. Direct CPU access (e.g. memcpy) 是 forbidden, just
类似 任何 其他 MMIO 映射. 同时 此 将 actually work 在 一些
architectures, others 将 experience corruption 或 just crash 在 the 内核.
Supporting FOLL_PCI_P2PDMA 在 一个 子系统 需要 scrubbing 它 到 ensure 无 CPU
access happens.


## Usage 与 DMABUF


DMABUF 提供 一个 alternative 到 the 上文 结构体 page-based
client/provider/orchestrator 系统 和 应当 为 使用 当 结构体 页
doesn't exist. 在 此 模式 the exporting 驱动 将 wrap
一些 的 其 MMIO 在 一个 DMABUF 和 give the DMABUF FD 到 userspace.

Userspace 可 然后 pass the FD 到 一个 importing 驱动 其 将 ask the
exporting 驱动 到 map 它 到 the importer.

在 此 case the initiator 和 target PCI_设备 是 known 和 the P2P 子系统
是 使用 到 determine the 映射 类型. The phys_addr_t-based DMA API 是 使用 到
establish the dma_addr_t.

Lifecycle 是 controlled 由 DMABUF move_notify(). 当 the exporting 驱动 wants
到 remove() 它 必须 deliver 一个 invalidation shutdown 到 全部 DMABUF importing
驱动 through move_notify() 和 synchronously DMA unmap 全部 the MMIO.

无 importing 驱动 可 continue 到 具有 一个 DMA map 到 the MMIO 之后 the
exporting 驱动 具有 destroyed 其 p2p_provider.


## P2P DMA 支持 库


   :export:
