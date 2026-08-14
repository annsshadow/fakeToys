## Pensando(R) 以太网适配器系列 Linux 驱动


Pensando Linux 以太网驱动。
Copyright(c) 2019 Pensando Systems, Inc

## 目录


- 识别适配器
- 启用驱动
- 配置驱动
- 通过辅助设备的 RDMA 支持
- 统计信息
- 支持

## 识别适配器


要确定系统上是否安装了一个或多个 Pensando PCI 以太网设备，可使用
```

  $ lspci -d 1dd8:
  b5:00.0 Ethernet controller: Device 1dd8:1002
  b6:00.0 Ethernet controller: Device 1dd8:1002

```
如果列出了如上所示的设备，那么 `ionic.ko` 驱动应该能找到并配置它们以供使用。内核日志中应当有相关条目
```

  $ dmesg | grep ionic
  ionic 0000:b5:00.0: 126.016 Gb/s available PCIe bandwidth (8.0 GT/s PCIe x16 link)
  ionic 0000:b5:00.0 enp181s0: renamed from eth0
  ionic 0000:b5:00.0 enp181s0: Link up - 100 Gbps
  ionic 0000:b6:00.0: 126.016 Gb/s available PCIe bandwidth (8.0 GT/s PCIe x16 link)
  ionic 0000:b6:00.0 enp182s0: renamed from eth0
  ionic 0000:b6:00.0 enp182s0: Link up - 100 Gbps

```
驱动和固件版本信息可以通过以下任一命令获取
```

  $ ethtool -i enp181s0
  driver: ionic
  version: 5.7.0
  firmware-version: 1.8.0-28
  ...

  $ devlink dev info pci/0000:b5:00.0
  pci/0000:b5:00.0:
    driver ionic
    serial_number FLM18420073
    versions:
        fixed:
          asic.id 0x0
          asic.rev 0x0
        running:
          fw 1.8.0-28

```
有关 devlink dev info 数据的更多信息，请参阅 `Documentation/networking/devlink/ionic.rst`。

## 启用驱动


驱动通过标准的内核配置系统启用，
```

  make oldconfig/menuconfig/etc.

```
该驱动在菜单结构中的位置为：

  -> Device Drivers
    -> Network device support (NETDEVICES [=y])
      -> Ethernet driver support
        -> Pensando devices
          -> Pensando Ethernet IONIC Support

## 配置驱动


### MTU


支持巨型帧（jumbo frame），最大大小为 9194 字节。

### 中断聚合（Interrupt coalescing）


中断聚合可以通过使用 "ethtool -C" 命令更改 rx-usecs 值来配置。rx-usecs 的取值范围是 0-190。tx-usecs 值反映了 rx-usecs 值，因为它们绑定在同一个中断上。

### SR-IOV


目前提供最基础的 SR-IOV 支持，可通过设置 sysfs 的 'sriov_numvfs' 值来启用（如果你的特定固件配置支持）。

### XDP


对 XDP 的支持包含基本功能，外加巨型帧、Redirect 和 `ndo_xmit`。目前不支持零拷贝套接字或硬件卸载。

## 通过辅助设备的 RDMA 支持


当固件声明支持时，ionic 驱动通过 Linux 辅助设备框架支持 RDMA（Remote Direct Memory Access，远程直接内存访问）功能。RDMA 能力在设备初始化期间被检测到，如果受支持，以太网驱动将创建一个辅助设备，允许 RDMA 驱动绑定并提供 InfiniBand/RoCE 功能。

## 统计信息


### 基础硬件统计


命令 `netstat -i`、`ip -s link show` 和 `ifconfig` 显示
```

  $ ip -s link show enp181s0
  7: enp181s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq state UP mode DEFAULT group default qlen 1000
      link/ether 00:ae:cd:00:07:68 brd ff:ff:ff:ff:ff:ff
      RX: bytes  packets  errors  dropped overrun mcast
      414        5        0       0       0       0
      TX: bytes  packets  errors  dropped carrier collsns
      1384       18       0       0       0       0

```
### ethtool -S


`ethtool -S` 命令显示的统计信息包含驱动计数器和固件计数器的组合，包括端口和队列相关的具体数值。驱动数值是由驱动计算得到的计数器，固件数值由固件从端口硬件采集并通过驱动透传，不做进一步解释。

```

     tx_packets: 12
     tx_bytes: 964
     rx_packets: 5
     rx_bytes: 414
     tx_tso: 0
     tx_tso_bytes: 0
     tx_csum_none: 12
     tx_csum: 0
     rx_csum_none: 0
     rx_csum_complete: 3
     rx_csum_error: 0
     xdp_drop: 0
     xdp_aborted: 0
     xdp_pass: 0
     xdp_tx: 0
     xdp_redirect: 0
     xdp_frames: 0

```
```

     tx_0_pkts: 3
     tx_0_bytes: 294
     tx_0_clean: 3
     tx_0_dma_map_err: 0
     tx_0_linearize: 0
     tx_0_frags: 0
     tx_0_tso: 0
     tx_0_tso_bytes: 0
     tx_0_hwstamp_valid: 0
     tx_0_hwstamp_invalid: 0
     tx_0_csum_none: 3
     tx_0_csum: 0
     tx_0_vlan_inserted: 0
     tx_0_xdp_frames: 0
     rx_0_pkts: 2
     rx_0_bytes: 120
     rx_0_dma_map_err: 0
     rx_0_alloc_err: 0
     rx_0_csum_none: 0
     rx_0_csum_complete: 0
     rx_0_csum_error: 0
     rx_0_hwstamp_valid: 0
     rx_0_hwstamp_invalid: 0
     rx_0_dropped: 0
     rx_0_vlan_stripped: 0
     rx_0_xdp_drop: 0
     rx_0_xdp_aborted: 0
     rx_0_xdp_pass: 0
     rx_0_xdp_tx: 0
     rx_0_xdp_redirect: 0

```
```

     hw_tx_dropped: 0
     hw_rx_dropped: 0
     hw_rx_over_errors: 0
     hw_rx_missed_errors: 0
     hw_tx_aborted_errors: 0
     frames_rx_ok: 15
     frames_rx_all: 15
     frames_rx_bad_fcs: 0
     frames_rx_bad_all: 0
     octets_rx_ok: 1290
     octets_rx_all: 1290
     frames_rx_unicast: 10
     frames_rx_multicast: 5
     frames_rx_broadcast: 0
     frames_rx_pause: 0
     frames_rx_bad_length: 0
     frames_rx_undersized: 0
     frames_rx_oversized: 0
     frames_rx_fragments: 0
     frames_rx_jabber: 0
     frames_rx_pripause: 0
     frames_rx_stomped_crc: 0
     frames_rx_too_long: 0
     frames_rx_vlan_good: 3
     frames_rx_dropped: 0
     frames_rx_less_than_64b: 0
     frames_rx_64b: 4
     frames_rx_65b_127b: 11
     frames_rx_128b_255b: 0
     frames_rx_256b_511b: 0
     frames_rx_512b_1023b: 0
     frames_rx_1024b_1518b: 0
     frames_rx_1519b_2047b: 0
     frames_rx_2048b_4095b: 0
     frames_rx_4096b_8191b: 0
     frames_rx_8192b_9215b: 0
     frames_rx_other: 0
     frames_tx_ok: 31
     frames_tx_all: 31
     frames_tx_bad: 0
     octets_tx_ok: 2614
     octets_tx_total: 2614
     frames_tx_unicast: 8
     frames_tx_multicast: 21
     frames_tx_broadcast: 2
     frames_tx_pause: 0
     frames_tx_pripause: 0
     frames_tx_vlan: 0
     frames_tx_less_than_64b: 0
     frames_tx_64b: 4
     frames_tx_65b_127b: 27
     frames_tx_128b_255b: 0
     frames_tx_256b_511b: 0
     frames_tx_512b_1023b: 0
     frames_tx_1024b_1518b: 0
     frames_tx_1519b_2047b: 0
     frames_tx_2048b_4095b: 0
     frames_tx_4096b_8191b: 0
     frames_tx_8192b_9215b: 0
     frames_tx_other: 0
     frames_tx_pri_0: 0
     frames_tx_pri_1: 0
     frames_tx_pri_2: 0
     frames_tx_pri_3: 0
     frames_tx_pri_4: 0
     frames_tx_pri_5: 0
     frames_tx_pri_6: 0
     frames_tx_pri_7: 0
     frames_rx_pri_0: 0
     frames_rx_pri_1: 0
     frames_rx_pri_2: 0
     frames_rx_pri_3: 0
     frames_rx_pri_4: 0
     frames_rx_pri_5: 0
     frames_rx_pri_6: 0
     frames_rx_pri_7: 0
     tx_pripause_0_1us_count: 0
     tx_pripause_1_1us_count: 0
     tx_pripause_2_1us_count: 0
     tx_pripause_3_1us_count: 0
     tx_pripause_4_1us_count: 0
     tx_pripause_5_1us_count: 0
     tx_pripause_6_1us_count: 0
     tx_pripause_7_1us_count: 0
     rx_pripause_0_1us_count: 0
     rx_pripause_1_1us_count: 0
     rx_pripause_2_1us_count: 0
     rx_pripause_3_1us_count: 0
     rx_pripause_4_1us_count: 0
     rx_pripause_5_1us_count: 0
     rx_pripause_6_1us_count: 0
     rx_pripause_7_1us_count: 0
     rx_pause_1us_count: 0
     frames_tx_truncated: 0

```
## 支持


有关一般性的 Linux 网络支持，请使用 netdev 邮件列表
```

  netdev@vger.kernel.org

```
如需更具体的支持，请使用 Pensando 驱动支持邮箱
```

  drivers@pensando.io

```
