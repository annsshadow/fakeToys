
## 面向 Intel(R) 1G 以太网 Linux 基础虚拟功能驱动

Intel 千兆虚拟功能 Linux 驱动。
Copyright(c) 1999-2018 Intel Corporation。

## 目录

- 识别你的适配器
- 额外配置
- 支持

该驱动支持基于 Intel 82576 的虚拟功能设备——这些虚拟功能设备只能在支持 SR-IOV 的内核上激活。

SR-IOV 需要正确的平台与操作系统支持。

加载此驱动的客户机操作系统必须支持 MSI-X 中断。

有关硬件要求的问题，请参阅随你的 Intel 适配器提供的文档。列出的所有硬件要求均适用于 Linux 下的使用。

驱动信息可以使用 ethtool、lspci 和 ifconfig 获取。关于更新 ethtool 的说明可在本文档后面的“额外配置”章节中找到。

注意：总共 32 个共享 VLAN 到 1 个或多个 VF 存在限制。

## 识别你的适配器

有关如何识别你的适配器以及获取最新 Intel 网络驱动的信息，请参阅 Intel 支持网站：
https://www.intel.com/support

## 附加功能与配置

### ethtool

驱动利用 ethtool 接口进行驱动配置和诊断，以及显示统计信息。此功能需要最新版本的 ethtool。下载地址：

https://www.kernel.org/pub/software/network/ethtool/

## 支持

有关常规信息，请访问 Intel 支持网站：
https://www.intel.com/support/

如果在使用受支持适配器、在受支持内核上发布的源代码中发现问题，请将与该问题相关的具体信息通过电子邮件发送至 intel-wired-lan@lists.osuosl.org。
