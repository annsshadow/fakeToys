
## 用于 AMD/Pensando(R) DSC 适配器系列的 PCI VFIO 驱动


AMD/Pensando Linux VFIO PCI 设备驱动
Copyright(c) 2023 Advanced Micro Devices, Inc.

## 概述


`pds-vfio-pci` 模块是一个 PCI 驱动，支持 DSC 硬件中具备实时迁移（Live Migration）能力的虚拟功能（VF）设备。

## 使用设备


pds-vfio-pci 设备通过多个配置步骤启用，并依赖 `pds_core` 驱动来创建和启用 SR-IOV 虚拟功能设备。

下面展示了将驱动绑定到一个 VF，以及绑定到由 `pds_core` 驱动创建的关联辅助设备的步骤。此示例假设 pds_core 和 pds-vfio-pci 模块已经加载。

  :name: example-setup-script

  #!/bin/bash

  PF_BUS="0000:60"
  PF_BDF="0000:60:00.0"
  VF_BDF="0000:60:00.1"

  # 阻止非 vfio 的 VF 驱动探测 VF 设备
  echo 0 > /sys/class/pci_bus/$PF_BUS/device/$PF_BDF/sriov_drivers_autoprobe

  # 通过 pds_core 创建单个用于实时迁移的 VF
  echo 1 > /sys/bus/pci/drivers/pds_core/$PF_BDF/sriov_numvfs

  # 允许将 VF 绑定到 pds-vfio-pci 驱动
  echo "pds-vfio-pci" > /sys/class/pci_bus/$PF_BUS/device/$VF_BDF/driver_override

  # 将 VF 绑定到 pds-vfio-pci 驱动
  echo "$VF_BDF" > /sys/bus/pci/drivers/pds-vfio-pci/bind

执行上述步骤后，应当在 /dev/vfio/<iommu_group> 中创建了一个文件。


## 启用驱动


该驱动通过标准的内核配置系统启用，
```

  make oldconfig/menuconfig/etc.

```
该驱动在菜单结构中的位置为：

  -> 设备驱动（Device Drivers）
    -> 非特权用户空间 VFIO 驱动框架（VFIO Non-Privileged userspace driver framework）
      -> 用于 PDS PCI 设备的 VFIO 支持（VFIO support for PDS PCI devices）

## 支持


对于一般性的 Linux 网络支持，请使用 netdev 邮件列表
```

  netdev@vger.kernel.org

```
对于更具体的支持需求，请使用 Pensando 驱动支持
```

  drivers@pensando.io

```
