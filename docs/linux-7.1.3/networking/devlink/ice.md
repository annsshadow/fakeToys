
## ice devlink 支持


本文档描述了`ice` 设备驱动实现devlink 特性
## 参数


   :widths: 5 5 90

   - - Name
     - Mode
     - Notes
   - - `enable_roce`
     - runtime
     - `enable_iwarp` 互斥
   - - `enable_iwarp`
     - runtime
     - `enable_roce` 互斥
   - - `tx_scheduling_layers`
     - permanent
     - ice 硬件Tx 使用分层调度，调度树中有固定数量的层级。它们每一个都是决策点。根节点代表一个端口，而所有叶子代表队列。这种配Tx 调度器的方式允许诸如 DCB devlink-rate（下文有文档说明）之类的特性来配置给予任何给定队列或队列组的带宽量，从而实现细粒度控制，因为调度参数可以在树的任意层级上配置
       默认9 层树拓扑被认为最适合大多数工作负载，因为它提供了性能与可配置性的最佳比例。然而，在某些特定情况下，这9 层拓扑可能并非所愿。一个例子是向不8 的倍数的队列发送流量。因为在 9 层拓扑中最大基数限制为 8，第 9 个队列与其余队列有不同的父节点，并被给予更多带宽信用。当系统9 个队列发送流量时，这会导致一个问题：

       | tx_queue_0_packets: 24163396
       | tx_queue_1_packets: 24164623
       | tx_queue_2_packets: 24163188
       | tx_queue_3_packets: 24163701
       | tx_queue_4_packets: 24163683
       | tx_queue_5_packets: 24164668
       | tx_queue_6_packets: 23327200
       | tx_queue_7_packets: 24163853
       | tx_queue_8_packets: 91101417 < Too much traffic is sent from 9th

       为了满足这一需求，你可以切换到 5 层拓扑，它将最大拓扑基数改512。有了这一增强，性能特征是均等的，因为所有队列都可以被分配到树中同一个父节点。此解决方案明显的缺点是树的配置深度较低
       使用 devlink 命令`tx_scheduling_layer` 参数来改变发送调度器拓扑。要使用 5 层拓扑，使用5。例如：
       $ devlink dev param set pci/0000:16:00.0 name tx_scheduling_layers
       value 5 cmode permanent
       使用9 将其设回默认值
       你必须对 PCI 插槽进行断电再上电，所选拓扑才能生效
       要验证值已设置       $ devlink dev param show pci/0000:16:00.0 name tx_scheduling_layers
   - - `msix_vec_per_pf_max`
     - driverinit
     - 设置 PF 可以使用的最MSI-X，其余可用于 SRIOV。范围从 msix_vec_per_pf_min 中设置的最小值到 2k/端口数   - - `msix_vec_per_pf_min`
     - driverinit
     - 设置 PF 将使用的最MSI-X。此值指明将静态分配多MSI-X。范围从 2 到在 msix_vec_per_pf_max 中设置的值
    :widths: 5 5 90

    - - Name
      - Mode
      - Description
    - - `local_forwarding`
      - runtime
      - 通过调优调度器带宽来控制环回行为。它影响所有类型的函数：物理、虚拟和子函数        支持的值有
        `enabled` - 端口上允许环回流
        `disabled` - 此端口上不允许环回流
        `prioritized` - 此端口上环回流量被优先处
        `local_forwarding` 参数的默认值为 `enabled`。`prioritized` 提供调整环回流量速率的能力，以牺牲另一个端口为代价来增加一个端口的容量。用户需要在一个端口上禁用本地转发，以便在 `prioritized` 端口上获得增加的容量
## 信息版本


`ice` 驱动报告以下版本

    :widths: 5 5 5 90

    - - Name
      - Type
      - Example
      - Description
    - - `board.id`
      - fixed
      - K65390-000
      - 板卡的产品板装配（PBA）标识符    - - `cgu.id`
      - fixed
      - 36
      - 时钟生成单元（CGU）硬件修订标识符    - - `fw.mgmt`
      - running
      - 2.1.7
      - 运行在设备嵌入式管理处理器上的管理固件的 3 位版本号。它控制 PHY、链路、对设备资源的访问等。Intel 文档将其称为 EMP 固件    - - `fw.mgmt.api`
      - running
      - 1.5.1
      - 管理固件通过 AdminQ 导出API 3 位版本号（major.minor.patch）。驱动用它来识别支持哪些命令。内核的早期版本只显2 位版本号（major.minor）    - - `fw.mgmt.build`
      - running
      - 0x305d955f
      - 管理固件来源的唯一标识符    - - `fw.undi`
      - running
      - 1.2581.0
      - 包含 UEFI 驱动Option ROM 的版本。版本以 `major.minor.patch` 格式报告。每当发生重大的破坏性变更，或次版本将要溢出时，主版本递增。次版本在非破坏性变更时递增，并在主版本递增时重置为 1。补丁版本通常0，但当修复作为针对较旧基础 Option ROM 的补丁提供时递增    - - `fw.psid.api`
      - running
      - 0.80
      - 定义闪存内容格式的版本    - - `fw.bundle_id`
      - running
      - 0x80002ec0
      - 加载到设备上的固件映像文件的唯一标识符。也称为 NVM EETRACK 标识符    - - `fw.app.name`
      - running
      - ICE OS Default Package
      - 设备中处于活动状态的 DDP 包的名称。DDP 包由驱动在初始化期间加载。DDP 包的每种变体都有唯一的名称    - - `fw.app`
      - running
      - 1.3.1.0
      - 设备中处于活动状态的 DDP 包的版本。注意，要唯一标识该包，名称和版本（由 `fw.app.name` 报告）都是必需的    - - `fw.app.bundle_id`
      - running
      - 0xc0000001
      - 设备中加载的 DDP 包的唯一标识符。也称为 DDP Track ID。可用于唯一标识特定DDP 包    - - `fw.netlist`
      - running
      - 1.1.2000-6.7.0
      - netlist 模块的版本。该模块定义设备的以太网能力和默认设置，并被管理固件用作管理链路和设备连接性的一部分    - - `fw.netlist.build`
      - running
      - 0xee16ced7
      - netlist 模块内容的哈希的4 个字节    - - `fw.cgu`
      - running
      - 8032.16973825.6021
      - 时钟生成单元（CGU）的版本。格式：<CGU 类型>.<配置版本>.<固件版本>
## 闪存更新


`ice` 驱动使用 `devlink-flash` 接口实现闪存更新支持。它支持使用包含 `fw.mgmt`、`fw.undi` `fw.netlist` 组件的合并闪存映像来更新设备闪存
   :widths: 5 95

   - - Bits
     - Behavior
   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS`
     - 不保留正在更新的闪存组件中存储的设置。这包括覆盖确定设备将初始化多少个物理函数的端口配置   - - `DEVLINK_FLASH_OVERWRITE_SETTINGS` and `DEVLINK_FLASH_OVERWRITE_IDENTIFIERS`
     - 既不保留设置也不保留标识符。用所提供映像的内容覆盖闪存中的一切，不进行任何保留。这包括覆盖设备标识字段，如 MAC 地址、VPD 区域和设备序列号。预期此组合与为特定设备定制的映像一起使用
ice 硬件不支持在保留设置的同时仅覆盖标识符，因此单独`DEVLINK_FLASH_OVERWRITE_IDENTIFIERS` 会被拒绝。如果未提供覆盖掩码，固件将被指示在更新时保留所有设置和标识字段
## 重新加载


`ice` 驱动支持在闪存更新后使用带有 `DEVLINK_RELOAD_ACTION_FW_ACTIVATE` 动作`DEVLINK_CMD_RELOAD` 来激活新固件

    $ devlink dev reload pci/0000:01:00.0 reload action fw_activate

新固件通过发出设备特定的嵌入式管理处理器复位来激活，该复位请求设备重置并重新加载 EMP 固件映像
驱动当前不支持通过 `DEVLINK_RELOAD_ACTION_DRIVER_REINIT` 重新加载驱动
## 端口拆分


`ice` 驱动仅支持端0 的端口拆分，因为 FW 为整个设备预定义了一组可用的端口拆分选项
应用端口拆分需要系统重启
以下命令将选择具有 4 个端口的端口拆分选项

    $ devlink port split pci/0000:16:00.0/0 count 4

每次 `split` `unsplit` 命令后，所有可用端口选项的列表将被打印到动态调试中。第一个选项是默认值

    ice 0000:16:00.0: Available port split options and max port speeds (Gbps):
    ice 0000:16:00.0: Status  Split      Quad 0          Quad 1
    ice 0000:16:00.0:         count  L0  L1  L2  L3  L4  L5  L6  L7
    ice 0000:16:00.0: Active  2     100   -   -   - 100   -   -   -
    ice 0000:16:00.0:         2      50   -  50   -   -   -   -   -
    ice 0000:16:00.0: Pending 4      25  25  25  25   -   -   -   -
    ice 0000:16:00.0:         4      25  25   -   -  25  25   -   -
    ice 0000:16:00.0:         8      10  10  10  10  10  10  10  10
    ice 0000:16:00.0:         1     100   -   -   -   -   -   -   -

可能存在多个具有相同端口拆分计数FW 端口选项。当再次发出相同的端口拆分计数请求时，将选择具有相同端口拆分计数的下一FW 端口选项
`devlink port unsplit` 将选择拆分计数1 的选项。如果没有拆分计数为 1 FW 选项可用，你将收到一个错误
## 区域


`ice` 驱动实现了以下用于访问内部设备数据的区域
    :widths: 15 85

    - - Name
      - Description
    - - `nvm-flash`
      - 整个闪存芯片的内容，有时称为设备的非易失性存储器    - - `shadow-ram`
      - Shadow RAM 的内容，它从闪存开头加载。尽管内容主要来自闪存，该区域还包含在设备启动期间生成、但未存储在闪存中的数据    - - `device-caps`
      - 设备固件能力缓冲区的内容。有助于确定设备的当前状态和配置
`nvm-flash` `shadow-ram` 区域都可以在没有快照的情况下访问。`device-caps` 区域需要快照，因为其内容由固件发送，无法拆分为单独的读取
用户可以通过 `DEVLINK_CMD_REGION_NEW` 命令请求立即捕获所有三个区域的快照

    $ devlink region show
    pci/0000:01:00.0/nvm-flash: size 10485760 snapshot [] max 1
    pci/0000:01:00.0/device-caps: size 4096 snapshot [] max 10

    $ devlink region new pci/0000:01:00.0/nvm-flash snapshot 1
    $ devlink region dump pci/0000:01:00.0/nvm-flash snapshot 1

    $ devlink region dump pci/0000:01:00.0/nvm-flash snapshot 1
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30
    0000000000000010 0000 0000 ffff ff04 0029 8c00 0028 8cc8
    0000000000000020 0016 0bb8 0016 1720 0000 0000 c00f 3ffc
    0000000000000030 bada cce5 bada cce5 bada cce5 bada cce5

    $ devlink region read pci/0000:01:00.0/nvm-flash snapshot 1 address 0 length 16
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30

    $ devlink region delete pci/0000:01:00.0/nvm-flash snapshot 1

    $ devlink region new pci/0000:01:00.0/device-caps snapshot 1
    $ devlink region dump pci/0000:01:00.0/device-caps snapshot 1
    0000000000000000 01 00 01 00 00 00 00 00 01 00 00 00 00 00 00 00
    0000000000000010 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000020 02 00 02 01 32 03 00 00 0a 00 00 00 25 00 00 00
    0000000000000030 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000040 04 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000050 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000060 05 00 01 00 03 00 00 00 00 00 00 00 00 00 00 00
    0000000000000070 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000080 06 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000090 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000000a0 08 00 01 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000000b0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000000c0 12 00 01 00 01 00 00 00 01 00 01 00 00 00 00 00
    00000000000000d0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000000e0 13 00 01 00 00 01 00 00 00 00 00 00 00 00 00 00
    00000000000000f0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000100 14 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000110 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000120 15 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000130 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000140 16 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    0000000000000150 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000160 17 00 01 00 06 00 00 00 00 00 00 00 00 00 00 00
    0000000000000170 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000180 18 00 01 00 01 00 00 00 01 00 00 00 08 00 00 00
    0000000000000190 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000001a0 22 00 01 00 01 00 00 00 00 00 00 00 00 00 00 00
    00000000000001b0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000001c0 40 00 01 00 00 08 00 00 08 00 00 00 00 00 00 00
    00000000000001d0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    00000000000001e0 41 00 01 00 00 08 00 00 00 00 00 00 00 00 00 00
    00000000000001f0 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
    0000000000000200 42 00 01 00 00 08 00 00 00 00 00 00 00 00 00 00
    0000000000000210 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00

    $ devlink region delete pci/0000:01:00.0/device-caps snapshot 1

## Devlink 速率


`ice` 驱动实现devlink-rate API。它允许将分QoS 卸载到硬件。它使用户能够将虚拟函数分组为树形结构，并向树中的每个节点分配受支持的参数：tx_share、tx_max、tx_priority tx_weight。因此，用户实际上获得了控制为每VF 组分配多少带宽的能力。这随后由硬件强制执行
假定此特性与 FW 中执行的 DCB ADQ，或任何会触QoS 变更（例如创建新的流量类）的驱动特性互斥。如果用户开始使devlink-rate API 对节点进行任何更改，驱动将阻DCB ADQ 配置。要配置这些特性，需要重新加载驱动。相应地，如ADQ DCB 被配置，驱动将根本不导出层级结构；或者，如果在层级结构导出之后、但在进行任何更改之前启用了这些特性，驱动将移除未触及的层级结构
此特性还依赖于系统中启用switchdev。这是必需的，因为 devlink-rate 需devlink-port 对象存在，而这些对象仅switchdev 模式下创建
如果驱动设置switchdev 模式，它将在 VF 创建的那一刻导出内部层级结构。树的根始终node_0 表示。此节点不能被用户删除。叶子节点和有子节点的节点也不能被删除
    :widths: 15 85

    - - Name
      - Description
    - - `tx_max`
      - 树节点要消耗的最大带宽。速率限制是一个绝对数字，指定节点在一秒内可以消耗的最大字节数。速率限制保证链路不会使远端接收方过饱和，并在订阅者与网络提供者之间强制执SLA    - - `tx_share`
      - 当树节点未被阻塞时分配给它的最小带宽。它指定一个绝对带宽。虽tx_max 定义了节点可以消耗的最大带宽，tx_share 标记为该节点承诺的带宽    - - `tx_priority`
      - 允许在兄弟节点之间使用严格优先级仲裁器。只要节点在其带宽限制内，此仲裁方案就尝试根据节点的优先级进行调度。范0-7。优先级7 的节点具有最高优先级并首先被选中，而优先级0 的节点优先级最低。具有相同优先级的节点被平等对待    - - `tx_weight`
      - 允许在兄弟节点之间使用加权公平队列仲裁方案。此仲裁方案可以与严格优先级同时使用。范1-200。对于仲裁，只有相对值才有意义
`tx_priority` `tx_weight` 可以同时使用。在这种情况下，具有相同优先级的节点在兄弟节点组中形成一WFQ 子组，它们之间的仲裁基于分配的权重

    # enable switchdev
    $ devlink dev eswitch set pci/0000:4b:00.0 mode switchdev

    # at this point driver should export internal hierarchy
    $ echo 2 > /sys/class/net/ens785np0/device/sriov_numvfs

    $ devlink port function rate show
    pci/0000:4b:00.0/node_25: type node parent node_24
    pci/0000:4b:00.0/node_24: type node parent node_0
    pci/0000:4b:00.0/node_32: type node parent node_31
    pci/0000:4b:00.0/node_31: type node parent node_30
    pci/0000:4b:00.0/node_30: type node parent node_16
    pci/0000:4b:00.0/node_19: type node parent node_18
    pci/0000:4b:00.0/node_18: type node parent node_17
    pci/0000:4b:00.0/node_17: type node parent node_16
    pci/0000:4b:00.0/node_14: type node parent node_5
    pci/0000:4b:00.0/node_5: type node parent node_3
    pci/0000:4b:00.0/node_13: type node parent node_4
    pci/0000:4b:00.0/node_12: type node parent node_4
    pci/0000:4b:00.0/node_11: type node parent node_4
    pci/0000:4b:00.0/node_10: type node parent node_4
    pci/0000:4b:00.0/node_9: type node parent node_4
    pci/0000:4b:00.0/node_8: type node parent node_4
    pci/0000:4b:00.0/node_7: type node parent node_4
    pci/0000:4b:00.0/node_6: type node parent node_4
    pci/0000:4b:00.0/node_4: type node parent node_3
    pci/0000:4b:00.0/node_3: type node parent node_16
    pci/0000:4b:00.0/node_16: type node parent node_15
    pci/0000:4b:00.0/node_15: type node parent node_0
    pci/0000:4b:00.0/node_2: type node parent node_1
    pci/0000:4b:00.0/node_1: type node parent node_0
    pci/0000:4b:00.0/node_0: type node
    pci/0000:4b:00.0/1: type leaf parent node_25
    pci/0000:4b:00.0/2: type leaf parent node_25

    # let's create some custom node
    $ devlink port function rate add pci/0000:4b:00.0/node_custom parent node_0

    # second custom node
    $ devlink port function rate add pci/0000:4b:00.0/node_custom_1 parent node_custom

    # reassign second VF to newly created branch
    $ devlink port function rate set pci/0000:4b:00.0/2 parent node_custom_1

    # assign tx_weight to the VF
    $ devlink port function rate set pci/0000:4b:00.0/2 tx_weight 5

    # assign tx_share to the VF
    $ devlink port function rate set pci/0000:4b:00.0/2 tx_share 500Mbps
