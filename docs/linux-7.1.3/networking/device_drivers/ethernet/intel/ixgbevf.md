
## 面向 Intel(R) 10G 以太Linux 基础虚拟功能驱动


Intel 10 千兆虚拟功能 Linux 驱动Copyright(c) 1999-2018 Intel Corporation
## 目录


- 识别你的适配- 已知问题
- 支持

该驱动支持基82599、X540、X550 X552 的虚拟功能设备，这些设备只能在支SR-IOV 的内核上激活
有关硬件要求的问题，请参阅随你的 Intel 适配器提供的文档。列出的所有硬件要求均适用Linux 下的使用

## 识别你的适配
该驱动与基于以下器件的设备兼容：

  - Intel(R) Ethernet Controller 82598
  - Intel(R) Ethernet Controller 82599
  - Intel(R) Ethernet Controller X520
  - Intel(R) Ethernet Controller X540
  - Intel(R) Ethernet Controller x550
  - Intel(R) Ethernet Controller X552
  - Intel(R) Ethernet Controller X553

有关如何识别你的适配器以及获取最Intel 网络驱动的信息，请参Intel 支持网站https://www.intel.com/support

## 已知问题/故障排查


SR-IOV 需要正确的平台与操作系统支持
加载此驱动的客户机操作系统必须支MSI-X 中断
目前该驱动仅作为可加载模块受支持。Intel 不提供针对内核源码的补丁以允许对驱动进行静态链接
VLAN：总共 64 个共VLAN 1 个或多个 VF 存在限制

## 支持

有关常规信息，请访问 Intel 支持网站https://www.intel.com/support/

如果在使用受支持适配器、在受支持内核上发布的源代码中发现问题，请将与该问题相关的具体信息通过电子邮件发送至 intel-wired-lan@lists.osuosl.org