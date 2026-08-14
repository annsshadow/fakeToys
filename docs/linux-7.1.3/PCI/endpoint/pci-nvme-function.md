## PCI NVMe 功能（Function）


:Author: Damien Le Moal <dlemoal@kernel.org>

该 PCI NVMe 端点功能（endpoint function）使用 NVMe 子系统目标核心代码实现一个 PCI NVMe
控制器。该功能的驱动位于 NVMe 子系统中，即 drivers/nvme/target/pci-epf.c。

更多细节参见 Documentation/nvme/nvme-pci-endpoint-target.rst。
