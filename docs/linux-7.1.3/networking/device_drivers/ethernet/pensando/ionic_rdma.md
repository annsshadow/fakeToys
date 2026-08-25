
## 适用AMD Pensando(R) 以太网适配器系列的 RDMA 驱动


AMD Pensando RDMA 驱动
Copyright (C) 2018-2025, Advanced Micro Devices, Inc.

## 概述


ionic_rdma 驱动AMD Pensando DSC（Distributed Services Card，分布式服务卡）设备提供远程直接内存访问（RDMA）功能。该驱动作为辅助驱动实现，与 ionic 以太网驱动协同工作

ionic 以太网驱动在设备初始化期间检RDMA 能力，并创建 ionic_rdma 驱动所绑定的辅助设备，从而建RDMA 数据通路和控制接口

## 识别适配


有关识别适配器的更多信息，请参阅 Documentation/networking/device_drivers/ethernet/pensando/ionic.rst

## 启用驱动


ionic_rdma 驱动依赖ionic 以太网驱动。有关启用和配置 ionic 驱动的详细信息，请参Documentation/networking/device_drivers/ethernet/pensando/ionic.rst

ionic_rdma 驱动通过标准内核配置系统启用
```
  make oldconfig/menuconfig/etc.
```
该驱动在菜单结构中的位置为：

  -> Device Drivers
    -> InfiniBand support
      -> AMD Pensando DSC RDMA/RoCE Support

## 支持


有关通用 Linux RDMA 支持，请使用 RDMA 邮件列表
```
  linux-rdma@vger.kernel.org
```
