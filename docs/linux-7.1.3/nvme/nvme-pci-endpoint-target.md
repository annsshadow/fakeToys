
## NVMe PCI 端点功能目标


:Author: Damien Le Moal <dlemoal@kernel.org>

NVMe PCI 端点功能目标驱动使用配置PCI 传输类型NVMe fabrics 目标
控制器，实现了一NVMe PCIe 控制器
## 概述


NVMe PCI 端点功能目标驱动允许通过 PCIe 链路暴露一NVMe 目标控制器，
从而实现一个类似于普M.2 SSD NVMe PCIe 设备。目标控制器的创建方式与
使用 NVMe over fabrics 时相同：该控制器表示通过端口访问某个 NVMe 子系统的
接口。端口传输类型必须配置为 “pci”。子系统可配置为使用常规文件或块设备
作为命名空间后端，也可使NVMe 直通，PCI 主机暴露一个现有的物理 NVMe
设备，或一NVMe fabrics 主机控制器（例如一NVMe TCP 主机控制器）
NVMe PCI 端点功能目标驱动尽可能依NVMe 目标核心代码来解析和执行PCIe
主机提交NVMe 命令。然而，借助 PCI 端点框架 API DMA API，该驱动还负管理通过 PCIe 链路进行的所有数据传输。这意味着 NVMe PCI 端点功能目标驱动
实现了若NVMe 数据结构管理与部NVMe 命令解析
1) 该驱动使DMA（若支持）或 MMIO（否则）来从提交队列中获NVMe 命令   取回的每条命令随后使用一个工作项执行，以在不CPU 上并行执行多条命   来最大化性能。该驱动使用一个工作项不断轮询所有提交队列的 doorbell
   （门铃），以检测来PCIe 主机的命令提交
2) 该驱动使MMIO 将已完成命令的完成队列条目复制到主机的完成队列，从   将其传输PCIe 主机。在将完成条目投递到完成队列后，该驱动使PCI 端点
   框架 API 向主机触发中断，以通知命令完成
3) 对于任何带有数据缓冲区的命令，NVMe PCI 端点目标驱动解析命令PRP    SGL 列表，以创建一组表示命令数据缓冲区在主机上映射PCI 地址段列表   命令数据缓冲区通过该组 PCI 地址段使DMA（若支持）在 PCIe 链路上传输   若不支持 DMA，则使用 MMIO，这会导致性能低下。对于写命令，命令数据缓冲区
   在执行命令前（使用目标核心代码）从主机传输到本地内存缓冲区。对于读命令   会分配一个本地内存缓冲区来执行命令，命令完成后该缓冲区内容被传输给主机
### 鎺у埗鍣ㄨ兘鍔。

通过 BAR 0 寄存器向 PCIe 主机暴露NVMe 能力，几乎与目标核心代码实现NVMe 目标控制器能力相同。但有一些例外
1) NVMe PCI 端点目标驱动始终将控制器能力 CQR 位置位，以请求“要求连续队列”   这是为了便于将队列的 PCI 地址范围映射到本CPU 地址空间
2) doorbell stride（门铃步幅，DSTRB）始终设4B

3) 由于 PCI 端点框架没有提供处理 PCI 层级复位的方法，控制器能NSSR    （NVM Subsystem Reset Supported，NVM 子系统复位支持）始终被清零
4) 启动分区支持（BPS）、持久内存区域支持（PMRS）以及控制器内存缓冲区支   （CMBS）能力从不报告
### 支持的特

NVMe PCI 端点目标驱动实现了对 PRP SGL 的支持。该驱动还实现了 IRQ 向量
合并与提交队列仲裁突发
队列的最大数量以及最大数据传输大小（MDTS）可在启动控制器前通过 configfs
配置。为避免执行命令时本地内存占用过多，MDTS 默认512 KB，并被限制为
最2 MB（人为上限）
### 所需的最PCI 地址映射窗口数量


大多PCI 端点控制器提供的映射窗口数量有限，用于将 PCI 地址范围映射本地 CPU 内存地址。NVMe PCI 端点目标控制器将映射窗口用于以下目的
1) 一个用于触MSI MSI-X 中断的内存窗2) 一个用MMIO 传输的内存窗3) 每个完成队列一个内存窗
考虑NVMe PCI 端点目标驱动操作的高度异步性质，上述内存窗口一般不会被
同时使用，但这种情况可能发生。因此，可支持的安全完成队列最大数量等PCI
端点控制器的内存映射窗口总数减去二。例如，对于一个有 32 个可用出站内存窗的端PCI 控制器，最多可安全地运30 个完成队列，而不会有因内存窗口不导致 PCI 地址映射错误的风险
### 队列对的最大数

NVMe PCI 端点目标驱动绑定PCI 端点控制器时，会分配 BAR 0，其空间
足以容纳管理队列和多I/O 队列。可支持I/O 队列对的最大数量受若干因素
限制
1) NVMe 目标核心代码I/O 队列的最大数量限制为在线 CPU 的数量2) 包括管理队列在内的队列对总数，不能超过可用的 MSI-X MSI 向量数量3) 完成队列的总数不能超过 PCI 映射窗口总数2（见上文）
NVMe 端点功能驱动允许通过 configfs 配置队列对的最大数量
### 限制与对 NVMe 规范的不合规


NVMe 目标核心代码类似，NVMe PCI 端点目标驱动不支持多个提交队列共同一个完成队列。所有提交队列必须指定一个唯一的完成队列

## 用户指南


本节描述硬件需求，以及如何搭建一NVMe PCI 端点目标设备
### 内核需

内核必须编译时启用配置选项 CONFIG_PCI_ENDPOINT、CONFIG_PCI_ENDPOINT_CONFIGFS
CONFIG_NVME_TARGET_PCI_EPF。CONFIG_PCI、CONFIG_BLK_DEV_NVME CONFIG_NVME_TARGET 也必须启用（这显然）
除此以外，还应至少为所用的端点硬件提供至少一PCI 端点控制器驱动
为便于测试，还建议启null-blk 驱动（CONFIG_BLK_DEV_NULL_BLK）。这样即使用一个以 null_blk 块设备作为子系统命名空间的简单搭建
### 硬件需

要使NVMe PCI 端点目标驱动，至少需要一个端点控制器设备
```

       # ls /sys/class/pci_epc/
        a40000000.pcie-ep

```
```

       # ls /sys/kernel/config/pci_ep/controllers
        a40000000.pcie-ep

```
端点板卡当然也必须通过一RX-TX 信号交叉PCI 线缆连接到主机。如果所的主PCI 插槽不具备即插即用能力，则应在配NVMe PCI 端点设备时关闭主电源
### NVMe 端点设备


创建一NVMe 端点设备是一个两步过程。首先，必须定义一NVMe 目标子系统和
端口。其次，必须搭建 NVMe PCI 端点设备，并将其绑定到所创建的子系统和端口
### 创建 NVMe 子系统与端口


如何配置 NVMe 目标子系统和端口的详细信息不在本文档范围内。下文仅提供一简单示例，展示一个有单个null_blk 设备作为后端的命名空间的端口和子系统
```

       # mount -t configfs none /sys/kernel/config

```
接下来，创建一null_blk 设备（默认设置会给出一250 GB 的设备）```

        # modprobe null_blk
        # ls /dev/nullb0
        /dev/nullb0

```
```

        # modprobe nvmet_pci_epf
        # lsmod | grep nvmet
        nvmet_pci_epf          32768  0
        nvmet                 118784  1 nvmet_pci_epf
        nvme_core             131072  2 nvmet_pci_epf,nvmet

```
现在，创建一个子系统和端口，我们将在搭建 NVMe PCI 端点目标设备时使用它来创建一PCI 目标控制器。在此：
```

        # cd /sys/kernel/config/nvmet/subsystems
        # mkdir nvmepf.0.nqn
        # echo -n "Linux-pci-epf" > nvmepf.0.nqn/attr_model
        # echo "0x1b96" > nvmepf.0.nqn/attr_vendor_id
        # echo "0x1b96" > nvmepf.0.nqn/attr_subsys_vendor_id
        # echo 1 > nvmepf.0.nqn/attr_allow_any_host
        # echo 4 > nvmepf.0.nqn/attr_qid_max

```
接下来，使用 null_blk 块设备创建并启用子系统命名空间：
```

        # mkdir nvmepf.0.nqn/namespaces/1
        # echo -n "/dev/nullb0" > nvmepf.0.nqn/namespaces/1/device_path
        # echo 1 > "nvmepf.0.nqn/namespaces/1/enable"

```
```

        # cd /sys/kernel/config/nvmet/ports
        # mkdir 1
        # echo -n "pci" > 1/addr_trtype
        # ln -s /sys/kernel/config/nvmet/subsystems/nvmepf.0.nqn \
                /sys/kernel/config/nvmet/ports/1/subsystems/nvmepf.0.nqn

```
### 创建 NVMe PCI 端点设备


NVMe 目标子系统和端口准备就绪后，现在即可创建并启NVMe PCI 端点设备NVMe PCI 端点目标驱动```

        # ls /sys/kernel/config/pci_ep/functions
        nvmet_pci_epf

```
```

        # cd /sys/kernel/config/pci_ep/functions/nvmet_pci_epf
        # mkdir nvmepf.0
        # ls nvmepf.0/
        baseclass_code    msix_interrupts   secondary
        cache_line_size   nvme              subclass_code
        deviceid          primary           subsys_id
        interrupt_pin     progif_code       subsys_vendor_id
        msi_interrupts    revid             vendorid

```
使用任意设备 ID 配置该功能（设备的厂ID 将自动设为与 NVMe 目标子系厂商 ID 相同的值）```

        # cd /sys/kernel/config/pci_ep/functions/nvmet_pci_epf
        # echo 0xBEEF > nvmepf.0/deviceid
        # echo 32 > nvmepf.0/msix_interrupts

```
如果所用的 PCI 端点控制器不支持 MSI-X，则可使MSI```

        # echo 32 > nvmepf.0/msi_interrupts

```
接下来，将我们的端点设备与之前创建的目标子系统和端口绑定```

        # echo 1 > nvmepf.0/nvme/portid
        # echo "nvmepf.0.nqn" > nvmepf.0/nvme/subsysnqn

```
随后即可将该端点功能绑定到端点控制器并：
```

        # cd /sys/kernel/config/pci_ep
        # ln -s functions/nvmet_pci_epf/nvmepf.0 controllers/a40000000.pcie-ep/
        # echo 1 > controllers/a40000000.pcie-ep/start

```
在端点机器上，内核消息会显示 NVMe 目标设备和端点设备被创建并连接时的信息

        null_blk: disk nullb0 created
        null_blk: module loaded
        nvmet: adding nsid 1 to subsystem nvmepf.0.nqn
        nvmet_pci_epf nvmet_pci_epf.0: PCI endpoint controller supports MSI-X, 32 vectors
        nvmet: Created nvm controller 1 for subsystem nvmepf.0.nqn for NQN nqn.2014-08.org.nvmexpress:uuid:2ab90791-2246-4fbb-961d-4c3d5a5a0176.
        nvmet_pci_epf nvmet_pci_epf.0: New PCI ctrl "nvmepf.0.nqn", 4 I/O queues, mdts 524288 B

### PCI 根复合体主机


启动 PCI 主机会触PCIe 链路的初始化（PCI 端点驱动可能以内核消息提示）当主NVMe 驱动启用端点时，端点上的内核消息也会给出提示```

        nvmet_pci_epf nvmet_pci_epf.0: Enabling controller

```
在主机一侧，NVMe PCI 端点功能目标设备为：
```

        # lspci -n
        0000:01:00.0 0108: 1b96:beef

```
```

        # lsblk
        NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINTS
        nvme0n1     259:0    0   250G  0 disk

```
NVMe 端点块设备之后可像任何其他常NVMe 命名空间块设备一样使用命令行工**nvme** 可用于获取更多信息：
```

        # nvme id-ctrl /dev/nvme0
        NVME Identify Controller:
        vid       : 0x1b96
        ssvid     : 0x1b96
        sn        : 94993c85650ef7bcd625
        mn        : Linux-pci-epf
        fr        : 6.13.0-r
        rab       : 6
        ieee      : 000000
        cmic      : 0xb
        mdts      : 7
        cntlid    : 0x1
        ver       : 0x20100
        ...


```
## 端点绑定


NVMe PCI 端点目标驱动使用 PCI 端点 configfs 设备属性的方式如下
================   ===========================================================
vendorid           忽略（使NVMe 目标子系统的厂商 IDdeviceid           任意值均可（例如 PCI_ANY_IDrevid              不关progif_code        必须0x02（NVM Expressbaseclass_code     必须0x01（PCI_BASE_CLASS_STORAGEsubclass_code      必须0x08（Non-Volatile Memory controllercache_line_size    不关subsys_vendor_id   忽略（使NVMe 目标子系统的子系统厂IDsubsys_id          任意值均可（例如 PCI_ANY_IDmsi_interrupts     至少等于期望的队列对数量
msix_interrupts    至少等于期望的队列对数量
interrupt_pin      在不支持 MSI MSI-X 时使用的 IRQ PIN
================   ===========================================================

NVMe PCI 端点目标功能在功能目录的 **nvme** 子目录下还有一些特定的可配字段。这些字段如下
================   ===========================================================
mdts_kb            最大数据传输大小，单位KiB（默认：512portid             要使用的目标端口 ID
subsysnqn          要使用的目标子系NQN
================   ===========================================================
