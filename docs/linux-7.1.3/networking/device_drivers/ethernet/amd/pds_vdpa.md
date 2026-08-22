
## 面向 AMD/Pensando(R) DSC 适配器系列的 PCI vDPA 驱动


AMD/Pensando vDPA VF 设备驱动

Copyright(c) 2023 Advanced Micro Devices, Inc

## 概述


`pds_vdpa` 驱动是一个辅助总线（auxiliary bus）驱动，提供一个供 virtio 网络协议栈使用的 vDPA 设备。它与提vDPA virtio 队列服务Pensando 虚拟功能（Virtual Function）设备一起使用。它依赖 `pds_core` 驱动与硬件来处理 PF VF PCI 事务，以及设备配置服务
## 使用设备


`pds_vdpa` 设备通过多个配置步骤启用，并依赖 `pds_core` 驱动来创建并启用 SR-IOV 虚拟功能设备。在 VF 启用后，我们`pds_core` 设备中启vDPA 服务，以创建pds_vdpa 使用的辅助设备
示例步骤

  #!/bin/bash

  modprobe pds_core
  modprobe vdpa
  modprobe pds_vdpa

  PF_BDF=`ls /sys/module/pds_core/drivers/pci\:pds_core/*/sriov_numvfs | awk -F / '{print $7}'`

  # PF 中启vDPA VF 辅助设备
  devlink dev param set pci/$PF_BDF name enable_vnet cmode runtime value true

  # vDPA 创建一VF
  echo 1 > /sys/bus/pci/drivers/pds_core/$PF_BDF/sriov_numvfs

  # 查找可用vDPA 服务/设备
  PDS_VDPA_MGMT=`vdpa mgmtdev show | grep vDPA | head -1 | cut -d: -f1`

  # 创建一个用virtio 网络配置vDPA 设备
  vdpa dev add name vdpa1 mgmtdev $PDS_VDPA_MGMT mac 00:11:22:33:44:55

  # 在该 vdpa 设备上建立以太网接口
  modprobe virtio_vdpa



## 启用驱动


该驱动通过标准内核配置系统启用```

  make oldconfig/menuconfig/etc.

```
该驱动位于菜单结构中的：

  -> Device Drivers
    -> Network device support (NETDEVICES [=y])
      -> Ethernet driver support
        -> Pensando devices
          -> Pensando Ethernet PDS_VDPA Support

## 支持


对于一Linux 网络支持，请使用 netdev 邮件列表
```

  netdev@vger.kernel.org

```
对于更具体的支持需求，请使Pensando 驱动支持
```

  drivers@pensando.io

```
