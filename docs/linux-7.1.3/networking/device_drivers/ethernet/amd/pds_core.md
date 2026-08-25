
## AMD/Pensando(R) DSC 适配器系列的 Linux 驱动


Copyright(c) 2023 Advanced Micro Devices, Inc

## 识别适配


要确定系统上是否安装了一个或多个 AMD/Pensando PCI Core 设备，可执行

```
  # lspci -d 1dd8:100c
  b5:00.0 Processing accelerators: Pensando Systems Device 100c
  b6:00.0 Processing accelerators: Pensando Systems Device 100c

```

如果列出了上述设备，`pds_core.ko` 驱动应能找到并配置它们以供使用。内核日志中应有如下条目

```
  $ dmesg | grep pds_core
  pds_core 0000:b5:00.0: 252.048 Gb/s available PCIe bandwidth (16.0 GT/s PCIe x16 link)
  pds_core 0000:b5:00.0: FW: 1.60.0-73
  pds_core 0000:b6:00.0: 252.048 Gb/s available PCIe bandwidth (16.0 GT/s PCIe x16 link)
  pds_core 0000:b6:00.0: FW: 1.60.0-73

```

```
  $ devlink dev info pci/0000:b5:00.0
  pci/0000:b5:00.0:
    driver pds_core
    serial_number FLM18420073
    versions:
        fixed:
          asic.id 0x0
          asic.rev 0x0
        running:
          fw 1.51.0-73
        stored:
          fw.goldfw 1.15.9-C-22
          fw.mainfwa 1.60.0-73
          fw.mainfwb 1.60.0-57

```

## Info versions


`pds_core` 驱动报告以下版本

   :widths: 5 5 90

   - - 名称
     - 类型
     - 描述
   - - `fw`
     - running
     - 设备上运行的固件版本
   - - `fw.goldfw`
     - stored
     - 存储goldfw 槽位中的固件版本
   - - `fw.mainfwa`
     - stored
     - 存储mainfwa 槽位中的固件版本
   - - `fw.mainfwb`
     - stored
     - 存储mainfwb 槽位中的固件版本
   - - `asic.id`
     - fixed
     - 该设备的 ASIC 类型
   - - `asic.rev`
     - fixed
     - 该设ASIC 的修订版

## 参数


`pds_core` 驱动实现了以下通用参数，用于控制作auxiliary_bus 设备提供的功能

   :widths: 5 5 8 82

   - - 名称
     - 模式
     - 类型
     - 描述
   - - `enable_vnet`
     - runtime
     - Boolean
     - 通过 auxiliary_bus 设备启用 vDPA 功能

## 固件管理


`flash` 命令可以更新 DSC 固件。下载的固件将保存到固件 bank 1 bank 2 中的任意一个（即当前未使用的那个）

```
  # devlink dev flash pci/0000:b5:00.0 \
            file pensando/dsc_fw_1.63.0-22.tar

```

## 健康报告


```
  # devlink health show pci/0000:2b:00.0 reporter fw
  pci/0000:2b:00.0:
    reporter fw
      state healthy error 0 recover 0
  # devlink health diagnose pci/0000:2b:00.0 reporter fw
   Status: healthy State: 1 Generation: 0 Recoveries: 0

```

## 启用驱动


该驱动通过标准内核配置系统启用

```
  make oldconfig/menuconfig/etc.

```

该驱动在菜单结构中的位置为：

  -> Device Drivers
    -> Network device support (NETDEVICES [=y])
      -> Ethernet driver support
        -> AMD devices
          -> AMD/Pensando Ethernet PDS_CORE Support

## 支持


有关通用 Linux 网络支持，请使用 netdev 邮件列表

```
  netdev@vger.kernel.org

```
