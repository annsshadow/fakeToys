
## 网络流处理器（Network Flow Processor，NFP）内核驱
:Copyright: |copy| 2019, Netronome Systems, Inc.
:Copyright: |copy| 2022, Corigine, Inc.

## 目录

- `概述`_
- `获取固件`_
- `Devlink 信息`_
- `配置设备`_
- `统计信息`_

## 概述

本驱动支Netronome Corigine 的系列网络流处理器（Network Flow Processor）设备，
包括 NFP3800、NFP4000、NFP5000 NFP6000 型号，这些设备也被集成到该公司的 Agilio
SmartNIC 系列中。驱动支持这些设备的 SR-IOV 物理功能与虚拟功能
## 获取固件

NFP3800、NFP4000 NFP6000 设备需要特定的应用固件才能工作。应用固件可以位于主文件系统上，也可以位于设备闪存中（前提是管理固件支持）
主机文件系统上的固件文件包含卡类型（`AMDA-*` 字符串）、介质配置等信息。若要从主机
文件系统加载固件，应将其放在 `/lib/firmware/netronome` 目录中
用于基本 NIC 操作的固件可在上游的 `linux-firmware.git` 仓库中获取
更完整的固件列表可从 `Corigine 支持站点 <https://www.corigine.com/DPUDownload.html>`_
下载
### 闪存中的固件

近期版本的管理固件支持在主机驱动被探测（probe）时从闪存加载应用固件。可以使用固加载策略配置来恰当地配置此功能
可以使用 Devlink ethtool，通过向相应命令提供合适的 `nic_AMDA*.nffw` 文件来更设备闪存上的应用固件。用户需要注意向闪存写入与卡和介质配置相匹配的正确固件映像
闪存中可用的存储空间取决于所使用的卡
### 处理多个项目

NFP 硬件是完全可编程的，因此可能存在面向不同应用的不同固件映像
当使用来自主机上的应用固件时，我们建议将实际的固件文件放在以应用命名的子目录中，
例如
```
    $ tree /lib/firmware/netronome/
    /lib/firmware/netronome/
    鈹溾攢鈹€ bpf
    鈹偮犅?鈹溾攢鈹€ nic_AMDA0081-0001_1x40.nffw
    鈹偮犅?鈹斺攢鈹€ nic_AMDA0081-0001_4x10.nffw
    鈹溾攢鈹€ flower
    鈹偮犅?鈹溾攢鈹€ nic_AMDA0081-0001_1x40.nffw
    鈹偮犅?鈹斺攢鈹€ nic_AMDA0081-0001_4x10.nffw
    鈹溾攢鈹€ nic
    鈹偮犅?鈹溾攢鈹€ nic_AMDA0081-0001_1x40.nffw
    鈹偮犅?鈹斺攢鈹€ nic_AMDA0081-0001_4x10.nffw
    鈹溾攢鈹€ nic_AMDA0081-0001_1x40.nffw -> bpf/nic_AMDA0081-0001_1x40.nffw
    鈹斺攢鈹€ nic_AMDA0081-0001_4x10.nffw -> bpf/nic_AMDA0081-0001_4x10.nffw

    3 directories, 8 files

```
在使用旧`mkinitrd` 命令而非 `dracut`（例Ubuntu）的发行版上，可能需要使用硬链接
而非符号链接
更改固件文件后，可能需要重新生initramfs 映像。initramfs 包含系统启动可能需要的
驱动和固件文件。请参考你的发行版文档以了解如何更initramfs。initramfs 过时的一明显迹象是：系统启动时加载了错误的驱动或固件，但手动重新加载驱动后一切正常
### 按设备选择固件

最常见的情况是系统上的所有卡都使用相同类型的固件。如果你想为特定的卡加载特定固件映像，可以使PCI 总线地址或序列号。驱动会按以下优先顺序查找固件文件：

```
    nfp: Looking for firmware file in order of priority:
    nfp:  netronome/serial-00-12-34-aa-bb-cc-10-ff.nffw: not found
    nfp:  netronome/pci-0000:02:00.0.nffw: not found
    nfp:  netronome/nic_AMDA0081-0001_1x40.nffw: found, loading...

```
在这种情况下，如`/lib/firmware/netronome` 中存在名**serial-00-12-34-aa-bb-5d-10-ff.nffw** **pci-0000:02:00.0.nffw** 的文件（链接），则该固件文件会优先于 `nic_AMDA*` 文件
请注意，`serial-**` `pci-**` 文件**不会**自动包含initramfs 中，你需要参相应工具的文档来了解如何包含它们
### 运行中的固件版本

可以通过 `ethtool -i` 查看特定 <netdev> 接口（例enp4s0）或接口端口 <netdev port>
（例enp4s0np0）所加载的固件版本：

```
  $ ethtool -i <netdev>

```
### 固件加载策略

固件加载策略由三HWinfo 参数控制，这些参数以键值对的形式存储在设备闪存中：

app_fw_from_flash
    定义应使用哪种固件优先，'Disk'）Flash'）或 'Preferred'）固件    当选择 'Preferred' 时，管理固件会比较闪存固件与主机提供固件的版本，从而决    加载哪个固件。该变量可使'fw_load_policy' devlink 参数进行配置
abi_drv_reset
    定义驱动在被探测时是否应重置固件，可选择 'Disk'，即如果在磁盘上找到固件）    'Always'，总是重置）或 'Never'，从不重置）。注意：如果在驱动被探测    固件已加载，则在驱动卸载时设备总是会被重置。该变量可使'reset_dev_on_drv_probe'
    devlink 参数进行配置
abi_drv_load_ifc
    定义允许在设备上加载 FW PF 设备列表。该变量当前不可由用户配置
## Devlink 信息

devlink info 命令会显示设备上运行中和已存储的固件版本、序列号以及板卡信息
```
  $ devlink dev info pci/0000:03:00.0
    pci/0000:03:00.0:
      driver nfp
      serial_number CSAAMDA2001-1003000111
      versions:
          fixed:
            board.id AMDA2001-1003
            board.rev 01
            board.manufacture CSA
            board.model mozart
          running:
            fw.mgmt 22.10.0-rc3
            fw.cpld 0x1000003
            fw.app nic-22.09.0
            chip.init AMDA-2001-1003  1003000111
          stored:
            fw.bundle_id bspbundle_1003000111
            fw.mgmt 22.10.0-rc3
            fw.cpld 0x0
            chip.init AMDA-2001-1003  1003000111

```
## 配置设备

本节介绍如何使用运行基本 NIC 固件Agilio SmartNIC
### 配置接口链路速率

以下步骤说明如何Agilio CX 2x25GbE 网卡上在 10G 模式25G 模式之间切换。端速率的更改必须按顺序进行：端0（p0）必须先设为 10G，之后端1（p1）才能设10G
```
  $ ip link set dev <netdev port 0> down
  $ ip link set dev <netdev port 1> down

```
```
  $ ethtool -s <netdev port 0> speed 10000
  $ ethtool -s <netdev port 1> speed 10000

```
```
  $ ethtool -s <netdev port 0> speed 25000
  $ ethtool -s <netdev port 1> speed 25000

```
```
  $ rmmod nfp; modprobe nfp

```
### 配置接口最大传输单元（MTU
接口MTU 可以使用 iproute2、ip link ifconfig 工具临时设置。请注意此更改不持久化。建议使Network Manager 或其他合适的操作系统配置工具进行设置，因为通过
Network Manager MTU 的更改可以持久化
```
  $ ip link set dev <netdev port> mtu 9000

```
在处理巨型帧（jumbo frames）或使用隧道时，由用户或编排层负责设置合适的 MTU 值。例如，
如果VM 发出的数据包要在卡上封装并从物理端口发出，则 VF MTU 应设置为低于物理端口
MTU，以容纳附加头部所增加的字节数。如果预SmartNIC 与内核之间会有回退流量，那用户还应确保 PF MTU 设置得当，以避免该路径上出现意外丢包
### 配置前向纠错（FEC）模
Agilio SmartNIC 支持 FEC 模式配置，例Auto、Firecode Base-R、ReedSolomon 以及 Off
模式。每个物理端口的 FEC 模式都可以通过 ethtool 独立设置。可以通过 `ethtool <netdev>`
查看某接口所支持FEC 模式
```
  $ ethtool <netdev>

```
```
  $ ethtool --show-fec <netdev>

```
要强制特定端口的 FEC 模式，必须先禁用自动协商（见 `自动协商`_ 一节）。设FEC 模式示例如下
```
  $ ethtool --set-fec <netdev> encoding rs

```
### 自动协商

要更改自动协商设置，必须先让链路 down。在链路 down 后：

```
  ethtool -s <netdev> autoneg <on|off>

```
## 统计信息

以下设备统计信息可通过 `ethtool -S` 接口获取
   :header-rows: 1
   :widths: 3 1 11

   - - Name
     - ID
     - Meaning

   - - dev_rx_discards
     - 1
     - 数据包可能因以下任一原因RX 路径上被丢弃
        - NIC 未处于混杂模式，且目MAC 地址与接口的 MAC 地址不匹配        - 接收到的数据包大于主机上的最大缓冲区大小，即超过了第 3 MRU        - 主机上没有可用于该数据包的空闲列表描述符。很可能NIC 未能及时缓存一个        - 某个 BPF 程序丢弃了该数据包        - 执行了数据面丢包动作        - MAC NIC 上缺少入口缓冲区空间而丢弃了该数据包
   - - dev_rx_errors
     - 2
     - 数据包可能因以下原因被计为（并丢弃为）RX 错误
       - VEB 查找出现问题（仅在使SR-IOV 时）       - 导致以太网错误的物理层问题，例如 FCS 或对齐错误。原因通常是故障线缆或 SFP
   - - dev_rx_bytes
     - 3
     - 接收到的字节总数
   - - dev_rx_uc_bytes
     - 4
     - 接收到的单播字节数
   - - dev_rx_mc_bytes
     - 5
     - 接收到的多播字节数
   - - dev_rx_bc_bytes
     - 6
     - 接收到的广播字节数
   - - dev_rx_pkts
     - 7
     - 接收到的数据包总数
   - - dev_rx_mc_pkts
     - 8
     - 接收到的多播数据包数
   - - dev_rx_bc_pkts
     - 9
     - 接收到的广播数据包数
   - - dev_tx_discards
     - 10
     - MAC 被流控且 NIC TX 队列空间耗尽时，数据包可能在 TX 方向被丢弃
   - - dev_tx_errors
     - 11
     - 数据包可能因以下任一原因被计TX 错误（并丢弃）：

       - 数据包是一LSO 分片，但无法确定3 层或4 层的偏移，因LSO 无法进行       - 通过 PCIe 收到了无效的数据包描述符       - 数据包的3 层长度超过了设备 MTU       - MAC/物理层出错。通常由于故障线缆SFP 所致       - 无法分配 CTM 缓冲区       - 数据包偏移不正确NIC 无法修复
   - - dev_tx_bytes
     - 12
     - 发送的字节总数
   - - dev_tx_uc_bytes
     - 13
     - 发送的单播字节数
   - - dev_tx_mc_bytes
     - 14
     - 发送的多播字节数
   - - dev_tx_bc_bytes
     - 15
     - 发送的广播字节数
   - - dev_tx_pkts
     - 16
     - 发送的数据包总数
   - - dev_tx_mc_pkts
     - 17
     - 发送的多播数据包数
   - - dev_tx_bc_pkts
     - 18
     - 发送的广播数据包数
注意，驱动未知的统计信息会显示为 `dev_unknown_stat$ID`，其`$ID` 指上表中的第二列