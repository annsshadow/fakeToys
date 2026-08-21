
## PCI Peer-to-Peer DMA 支持


The PCI 总线 具有 pretty decent 支持 用于 performing DMA transfers
之间 two 设备 the 总线. 类型 transaction henceforth
called Peer-to-Peer (P2P). 然 存在 一数字 issues 
make P2P transactions tricky 执行 一perfectly safe way.

用于 PCIe the routing Transaction Layer Packets (TLPs) well-defined up
直到 它们 reach 一host bridge root 端口. the path 包含 PCIe switches
然后 基于 the ACS 设置 the transaction route entirely 之内
the PCIe hierarchy 从不 reach the root 端口. The 内核 evaluate
the PCIe topology 始终 permit P2P 这些 well-defined cases.

然 the P2P transaction reaches the host bridge 然后 可能 具有 
hairpin back out the 相同 root 端口, routed inside the CPU SOC another
PCIe root 端口, routed internally the SOC.

The PCIe specification doesn't 定义 the forwarding transactions 之间
hierarchy domains 内核 defaults blocking 此类 routing. 存在 一
允许 列出 允许 detecting known-good HW, case P2P 之间 任何
two PCIe 设备 permitted.

Since P2P inherently doing transactions 之间 two 设备 需two
驱动 co-operating inside the 内核. The providing 驱动 具有 convey
MMIO the consuming 驱动. meet the 驱动 型号 lifecycle rules the
MMIO 必须 具有 全部 DMA 映射 removed, 全部 CPU accesses prevented, 全部 
mappings undone 之前 the providing 驱动 completes remove().

需the providing consuming 驱动 actively work together 
guarantee the consuming 驱动 具有 stopped 使用 the MMIO 期间 一removal
cycle. 这是 已完任一一synchronous invalidation shutdown waiting
用于 全部 usage refcounts reach zero.

the lowest level the P2P 子系offers 一naked 结构p2p_provider 
delegates lifecycle 管理 the providing 驱动. 它是 expected 
驱动 使用 选项 wrap 它们MMIO 内存 DMABUF 使用 DMABUF
提供 一invalidation shutdown. 这些 MMIO 地址 具有 结构 
使用 mmap() 必须 创建 特殊 PTEs. 作为 此类 存在 very 少量
内核 uAPIs accept 指针 them; 特别它们 cannot 使用
读取()/写入(), including O_DIRECT.

Building  the 子系offers 一layer wrap the MMIO 一ZONE_设备
pgmap 内存_设备_PCI_P2PDMA 创建 结构 The lifecycle 
pgmap ensures the pgmap destroyed 全部 其他 驱动 具有 stopped
使用 the MMIO. 选项 works O_DIRECT flows, 一cases, the
underlying 子系supports handling 内存_设备_PCI_P2PDMA through
FOLL_PCI_P2PDMA. The 使用 FOLL_LONGTERM prevented. 作为 relies pgmap
relies architecture 支持 along alignment 最大小
limitations.


## 驱动 Writer's Guide


一given P2P implementation 那里 three 更多 不同
types 内核 驱动 play:

- Provider - 一驱动 提供 publishes P2P resources 类似
  内存 doorbell 寄存其他 驱动.
- Client - 一驱动 makes 使用 一resource 设置 up 一
  DMA transaction 来自 
- Orchestrator - 一驱动 orchestrates the flow 数据 之间
  clients 鍜?providers.

许多 cases 那里 可以 overlap 之间 这些 three types (i.e.,
典型 用于 一驱动 两一provider 一client).

例如, the NVMe Target Copy Offload implementation:

- The NVMe PCI 驱动 两一client, provider orchestrator
  exposes 任何 CMB (控制内存 缓冲 作为 一P2P 内存
  resource (provider), accepts P2P 内存 作为 缓冲requests
  使用 directly (client) make 使用 the CMB 作为
  submission 队列 条目 (orchestrator).
- The RDMA 驱动 一client arrangement 因此 一RNIC
  DMA directly the 内存 exposed the NVMe 设备.
- The NVMe Target 驱动 (nvmet) orchestrate the 数据 来自 the RNIC
  the P2P 内存 (CMB) 然后 the NVMe 设备 (vice versa).

这是 currently the arrangement 受支the 内核 
one 可以 imagine slight tweaks 将会 允许 用于 the 相同
functionality. 例如, 一特定 RNIC added 一BAR 一
内存 behind  驱动 可以 add 支持 作为 一P2P provider 
然后 the NVMe Target 可以 使用 the RNIC's 内存 而非 the CMB
cases 何处 the NVMe 使用 执行 具有 CMB 支持.


### Provider 驱动


一provider simply needs 注册 一BAR (一portion 一BAR)
作为 一P2P DMA resource 使用 `pci_p2pdma_add_resource()`.
注册 结构用于 全部 the specified 内存.

之后 optionally publish 全部 resources 作为
P2P 内存 使用 `pci_p2pmem_publish()`. 允许
任何 orchestrator 驱动 find 使用 the 内存. marked 
way, the resource 必须 regular 内存 side effects.

用于 the time 正在 这是 fairly rudimentary 全部 resources
typically going P2P 内存. Future work likely expand
包含 其他 types resources 类似 doorbells.


### Client 驱动


一client 驱动 具有 使用 the 映射 API `dma_map_sg()`
`dma_unmap_sg()` 函数 作为 usual, the implementation
执行 the right thing 用于 the P2P capable 内存.


### Orchestrator 驱动


The 第一 task 一orchestrator 驱动 必须 执行 compile 一列出 
全部 client 设备 involved 一given transaction. 用于
示例, the NVMe Target 驱动 creates 一列出 including the namespace
设备 the RNIC 使用. the orchestrator 具有 access 
一特定 P2P provider 使用 check compatibility 使用
`pci_p2pdma_distance()` 否则 find 一内存 provider
s compatible 全部 clients 使用  `pci_p2pmem_find()`.
多于 one provider 受支 the one nearest 全部 the clients 
chosen 第一. 多于 one provider 一equal distance away, the
one returned chosen random (它是 一arbitrary 
truly random). 函数 returns the PCI 设备 使用 用于 the provider
一参taken 因此 s longer needed 应当 
returned 涓?PCI_dev_put().

一一provider selected, the orchestrator 然后 使用
`pci_alloc_p2pmem()` 鍜?`pci_free_p2pmem()` 鍒。
allocate P2P 内存 来自 the provider. `pci_p2pmem_alloc_sgl()`
`pci_p2pmem_free_sgl()` convenience 函数 用于
allocating scatter-gather 列表 P2P 内存.

### 结构Caveats


同时 the 内存_设备_PCI_P2PDMA installed VMAs,
pin_用户_) related return them 除非 FOLL_PCI_P2PDMA set.

The 内存_设备_PCI_P2PDMA 需care 支持 the 内核. The
KVA 仍然 MMIO 必须 仍然 accessed through the 正常
readX()/writeX()/绛?helpers. Direct CPU access (e.g. memcpy) 鏄?forbidden, just
类似 任何 其他 MMIO 映射. 同时 actually work 一
architectures, others experience corruption just crash the 内核.
Supporting FOLL_PCI_P2PDMA 一子系需scrubbing ensure CPU
access happens.


## Usage 涓?DMABUF


DMABUF 提供 一alternative the 上文 结构page-based
client/provider/orchestrator 系统 应当 使用 结构
doesn't exist. 模式 the exporting 驱动 wrap
一MMIO 一DMABUF give the DMABUF FD userspace.

Userspace 然后 pass the FD 一importing 驱动 ask the
exporting 驱动 map the importer.

case the initiator target PCI_设备 known the P2P 子系
使用 determine the 映射 类型. The phys_addr_t-based DMA API 使用 
establish the dma_addr_t.

Lifecycle controlled DMABUF move_notify(). the exporting 驱动 wants
remove() 必须 deliver 一invalidation shutdown 全部 DMABUF importing
椹卞姩 through move_notify() 鍜?synchronously DMA unmap 鍏ㄩ儴 the MMIO.

importing 驱动 continue 具有 一DMA map the MMIO 之后 the
exporting 椹卞姩 鍏锋湁 destroyed 鍏?p2p_provider.


## P2P DMA 支持 


   :export:
