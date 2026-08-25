## Powernv 上的 PCI Express I/O 虚拟化资

Wei Yang <weiyang@linux.vnet.ibm.com>

Benjamin Herrenschmidt <benh@au1.ibm.com>

Bjorn Helgaas <bhelgaas@google.com>

2014 骞?8 鏈?26 鏃。
本文档描述了硬件PowerKVM PCI MMIO 资源大小与分配的要求，以及通用 PCI 代码如何处理这一要求。前两节描述了可分区端点（Partitionable Endpoint）的概念以及 P8（IODA2）上的实现。接下来的两节讨论在 IODA2 上启SR-IOV 的考虑事项
## 1. 可分区端点简

可分区端点（PE，Partitionable Endpoint）是一种将设备或一组设备所关联的各种资源分组起来的方式，用于在不同分区之间提供隔离（即DMA、MSI 等进行过滤），并提供一种机制来冻结导致错误的设备，以限制坏数据传播的可能性
因此，在硬件中存在一PE 状态表，其中包含每PE 的一对“冻结（frozen）”状态位（一个用MMIO，一个用DMA，它们会被一起置位，但可以分别清除）
当一PE 被冻结时，任何方向的所有存储操作都会被丢弃，所有加载操作都返回1 值。MSI 也会被阻断。还有更多的状态用于记录导致冻结的错误细节等信息，但那些并不关键
有趣的部分在于各PCIe 事务（MMIO、DMA……）是如何匹配到它们所对应PE 的
以下一节粗略描述了我们P8（IODA2）上的实现。请记住，这一切都是按 PHB（PCI 主机桥）划分的。每PHB 都是一个完全独立的硬件实体，它复制了整个逻辑，因此拥有自己的一PE 等
## 2. P8（IODA2）上可分区端点的实现


P8 每个 PHB 最多支256 个可分区端点
  - 入站（Inbound
   对于 DMA、MSI 和入PCIe 错误消息，我们有一张表（位于内存中，但由芯片在硬件中访问），它提供 PCIe RID（bus/dev/fn）与 PE 编号之间的直接对应关系。我们称之为 RTT
    - 对于 DMA，我们随后为每个 PE 提供整个地址空间，根PCI 地址59 位的值不同，它可以包含两个“窗口（window）”。每个窗口都可以配置为通过“TCE 表”（IOMMU 转换表）进行重映射，该表具有各种此处未描述的可配置特性
    - 对于 MSI，我们在地址空间中有两个窗口（一个位32 位空间的顶部，另一个高得多），通过地址MSI 值的组合，将会触发每个桥2048 个中断之一。中断控制器描述符表中也有一PE#，它会与RTT 获得PE# 进行比较，以“授权”该设备发出那个特定的中断
    - 错误消息仅使RTT
  - 出站（Outbound）。这才是棘手的部分
    与其PCI 主机桥一样，Power8 IODA2 PHB 支持CPU 地址空间PCI 地址空间的“窗口”。有一M32 窗口和十六个 M64 窗口。它们具有不同的特性。首先是它们的共同点：它们将 CPU 地址空间中可配置的一部分转发PCIe 总线，并且其大小必须是自然对齐的 2 的幂。其余部分则各不相同
    - M32 窗口
      - 大小限制4GB
      - 丢弃地址的高位（高于大小的部分），并用一个可配置的值替换。这通常用来产生 32 PCIe 访问。我们在启动时由 FW 配置该窗口，并且不会Linux 中改动它；它通常被设置为CPU PCIe 的一2GB 地址空间转发0x8000_0000..0xffff_ffff。（注意：最高的 64KB 实际上是MSI 保留的，但这一点目前在此时不是问题；我们只需确保 Linux 不会在那里分配任何东西，不过 M32 逻辑会忽略这一点，如果我们尝试的话它仍会在该空间中转发）
      - 它被划分256 个等大小的段。芯片中的一张表将每个段映射到一PE#。这使得 MMIO 空间的一部分可以以段的粒度分配给 PE。对于一2GB 的窗口，段粒度为 2GB/256 = 8MB
    现在，这是今Linux 中使用的“主”窗口（不包SR-IOV）。我们基本上是利用将桥的 MMIO 窗口强制对齐/粒度为段这一技巧，使得桥后面的空间可以被分配给一PE
    理想情况下，我们希望能够让各个功能（function）位于不同的 PE 中，但那意味着要使用一种完全不同的地址分配方案，其中单个功BAR 可以被“分组”以适配在一个或多个段中
    - M64 窗口
      - 大小必须至少256MB
      - 不进行地址转换（PCIe 上的地址PowerBus 上的地址相同）。有一种方式可以设PowerBus 不传送的最14 位，但我们不使用它
      - 可以配置为分段（segmented）。当不分 segment 时，我们可以为整个窗口指PE#。当分段时，一个窗口有 256 个段；但是，没有用于将段映射PE# 的表。段编号**就是** PE#
      - 支持重叠。如果一个地址被多个窗口覆盖，则有定义的优先级顺序来决定应用哪个窗口
    我们有代码（相对M32 部分而言相当新）利用这一点来处理 64 位空间中的大 BAR
    我们配置一M64 窗口来覆盖由 FW PHB 分配的全部地址空间区域（约 64GB，忽M32 的空间，它来自不同的“保留（reserve）”）。我们将其配置为分段
    然后我们进行M32 相同的操作，使用桥对齐技巧，以匹配那些巨大的段
    由于我们不能重映射，我们还有两个额外的约束：

    - 我们64 位空间被分配**之后**才进PE# 分配，因为我们直接使用的地址决定PE#。然后我们为同时使用 32 位和 64 位空间的设备更新M32 PE#，或者将剩余PE# 分配给仅使用 32 位的设备
    - 我们无法在硬件中“分组”段，因此如果一个设备最终使用了超过一个段，我们就会得到超过一PE#。有一种硬件机制可以让冻结状态级联到“伴随（companion）”PE，但这仅PCIe 错误消息有效（通常用于当你冻结一个交换机时，它会冻结其所有子设备）。所以我们在软件中做这件事。在这种情况下我们会损失一EEH 的有效性，但那是我们能找到的最佳方案。因此当任何一PE 冻结时，我们都会冻结该“域（domain）”中的其PE。于是我们引入了“主 PE（master PE）”的概念，即用于 DMA、MSI 等的那个，以及用于其M64 段的“从 PE（secondary PE）”
    我们希望研究使用额外M64 窗口以“单 PE”模式叠加在特定BAR 上，以绕过其中一些问题，例如针对具有非常BAR 的设备（GPU）。这很有意义，但我们尚未这样做
## 3. PowerKVM SR-IOV 的考虑事项


  - SR-IOV 背景

    PCIe SR-IOV 特性允许单个物理功能（PF，Physical Function）支持多个虚拟功能（VF，Virtual Function）。PF SR-IOV 能力（Capability）中的寄存器控制 VF 的数量以及它们是否被启用
    VF 被启用时，它们会像普PCI 设备一样出现在配置空间（Configuration Space）中，但 VF 配置空间头中BAR 是不寻常的。对于非 VF 设备，软件使用配置空间头中的 BAR 来发BAR 的大小并为其分配地址。对VF 设备，软件使*PF** SR-IOV 能力中的 VF BAR 寄存器来发现大小并分配地址。VF 配置空间头中BAR 是只读的零
    PF SR-IOV 能力中的某个 VF BAR 被编程时，它会设置所有对VF(n) BAR 的基地址。例如，如果 PF SR-IOV 能力被编程为启用 8 VF，并且它有一1MB VF BAR0，那么该 VF BAR 中的地址就设置了一8MB 区域的基地址。该区域被划分为八个连续1MB 区域，每个区域是其中一VF BAR0。请注意，尽VF BAR 描述了一8MB 区域，但对齐要求是针对单VF 的，即本例中1MB
  有几种将 VF 隔离PE 中的策略
  - M32 窗口：只有一M32 窗口，它被划分为 256 个等大小的段。可能的最细粒度为带有 1MB 段的 256MB 窗口。大小为 1MB 或更大的 VF BAR 可以被映射到该窗口中的不PE。每个段都可以通过查找表单独映射到一PE，因此这非常灵活，但当所VF BAR 大小相同时效果最好。如果它们大小不同，整个窗口必须足够小，以使段大小与最小的 VF BAR 相匹配，这意味着较大VF BAR 会跨越多个段
  - 非分M64 窗口：一个非分段M64 窗口被整体映射到一个单一PE，因此它只能隔离一VF
  - 单个分段 M64 窗口：一个分段的 M64 窗口可以M32 窗口一样使用，但段不能被单独映射到 PE（段编号就是 PE#），因此灵活性没那么高。一个拥有多BAR VF 将不得不位于多个 PE 的“域”中，其隔离性不如单PE
  - 多个分段 M64 窗口：像往常一样，每个窗口被划分为 256 个等大小的段，段编号就是 PE#。但如果我们使用多个 M64 窗口，它们可以被设置为不同的基地址和不同的段大小。如果我们有VF 各自拥有一1MB BAR 和一32MB BAR，我们就可以用一M64 窗口分配 1MB 段，用另一M64 窗口分配 32MB 段
  最后，是使M64 窗口实现 SR-IOV 的计划，这将在接下来的两节中更详细地描述。对于给定的 VF BAR，我们需要有效地保留整个 256 个段56 * VF BAR 大小），并将 VF BAR 定位到该 M64 窗口中一段空闲段/PE 范围的起始位置
  目标当然是为每个 VF 提供一个独立的 PE
  IODA2 平台16 M64 窗口，用于将 MMIO 范围映射PE#。每M64 窗口定义一MMIO 范围，该范围被划分为 256 个段，每个段对应一PE
  我们决定利用这个 M64 窗口VF 映射到独立的 PE，因SR-IOV VF BAR 的大小都相同
  但这样做会带来另一个问题：total_VFs 通常小于 M64 窗口段的数量，因此如果我们将一VF BAR 直接映射到一M64 窗口，M64 窗口的某部分将映射到另一个设备的 MMIO 范围
  IODA 支持 256 PE，因此分段窗口包256 个段，所以如total_VFs 小于 256，我们就会遇到图 1.0 中的情况，其M64 窗口的段 [total_VFs, 255] 可能映射到某MMIO 范围```
     0      1                     total_VFs - 1
     +------+------+-     -+------+------+
     |      |      |  ...  |      |      |
     +------+------+-     -+------+------+

                           VF(n) BAR space

     0      1                     total_VFs - 1                255
     +------+------+-     -+------+------+-      -+------+------+
     |      |      |  ...  |      |      |   ...  |      |      |
     +------+------+-     -+------+------+-      -+------+------+

                           M64 window

		Figure 1.0 Direct map VF(n) BAR space

  Our current solution is to allocate 256 segments even if the VF(n) BAR
  space doesn't need that much, as shown in Figure 1.1::

     0      1                     total_VFs - 1                255
     +------+------+-     -+------+------+-      -+------+------+
     |      |      |  ...  |      |      |   ...  |      |      |
     +------+------+-     -+------+------+-      -+------+------+

                           VF(n) BAR space + extra

     0      1                     total_VFs - 1                255
     +------+------+-     -+------+------+-      -+------+------+
     |      |      |  ...  |      |      |   ...  |      |      |
     +------+------+-     -+------+------+-      -+------+------+

			   M64 window

		Figure 1.1 Map VF(n) BAR space + extra

  Allocating the extra space ensures that the entire M64 window will be
  assigned to this one SR-IOV device and none of the space will be
  available for other devices.  Note that this only expands the space
  reserved in software; there are still only total_VFs VFs, and they only
  respond to segments [0, total_VFs - 1].  There's nothing in hardware that
  responds to segments [total_VFs, 255].

```
## 4. 对通用 PCI 代码的影

PCIe SR-IOV 规范要求 VF(n) BAR 空间的基地址与单VF BAR 的大小对齐
IODA2 中，MMIO 地址决定PE#。如果地址位于 M32 窗口中，我们可以通过更新将段转换PE# 的表来设PE#。类似地，如果地址位于一个不分段M64 窗口中，我们可以设置该窗口的 PE#。但如果它位于一个分段的 M64 窗口中，段编号就PE#
因此，控VF PE# 的唯一方法是更VF BAR VF(n) BAR 空间的基地址。如PCI 核心分配VF(n) BAR 空间所需的确切大小，VF BAR 的值是固定的，无法更改
另一方面，如PCI 核心分配了额外的空间，只要整VF(n) BAR 空间仍位于核心分配的空间内，VF BAR 的值就可以更改
理想情况下，段大小将与单VF BAR 大小相同。这样每VF 都会位于自己PE 中。VF BAR（以及因PE#）是连续的。如VF0 位于 PE(x)，那VF(n) 就位PE(x+n)。如果我们分256 个段，那VF0 PE# (256 - numVFs) 种选择
如果段大小小VF BAR 大小，则需要多个段来覆盖一VF BAR，一VF 将位于多PE 中。这是可能的，但隔离性没那么好，并且它会减少 PE# 的选择数量，因VF(n) BAR 空间将消(numVFs * n) 个段，而不是只消numVFs 个段。这意味着可用于调VF(n) BAR 空间基地址的可用段没有那么多