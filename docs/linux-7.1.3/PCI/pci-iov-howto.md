
## PCI Express I/O 虚拟化使用指南（Howto）


:Copyright: |copy| 2009 Intel Corporation
:Authors: - Yu Zhao <yu.zhao@intel.com>
          - Donald Dutile <ddutile@redhat.com>

## 概述


### 什么是 SR-IOV

单根 I/O 虚拟化（Single Root I/O Virtualization，SR-IOV）是 PCI Express 的一项扩展能力，它使一个物理设备表现为多个虚拟设备。物理设备称为物理功能（Physical Function，PF），而虚拟设备称为虚拟功能（Virtual Function，VF）。VF 的分配可由 PF 通过封装在该能力中的寄存器动态控制。默认情况下，此特性未启用，PF 表现为传统 PCIe 设备。一旦开启，每个 VF 的 PCI 配置空间可由其自身的总线（Bus）、设备（Device）与功能（Function）号（即路由 ID，Routing ID）访问。每个 VF 还拥有 PCI 内存空间（PCI Memory Space），用于映射其寄存器集。VF 设备驱动作用于该寄存器集，从而使其能正常工作并表现为真实存在的 PCI 设备。

## 用户指南


### 如何启用 SR-IOV 能力

启用 SR-IOV 有多种方法。
第一种方法中，设备驱动（PF 驱动）通过 SR-IOV 核心提供的 API 来控制该能力的开启与关闭。如果硬件具备 SR-IOV 能力，加载其 PF 驱动将启用它以及与该 PF 关联的所有 VF。某些 PF 驱动需要一个模块参数来设定要启用的 VF 数量。
第二种方法中，向 sysfs 文件 sriov_numvfs 写入，将启用或关闭与某个 PCIe PF 关联的 VF。该方法实现的是逐 PF 的 VF 启用/关闭值，而第一种方法作用于同一设备的所有 PF。此外，PCI SRIOV 核心层会确保启用/关闭操作合法，以减少多个驱动对相同检查的重复实现，例如，启用 VF 时检查 numvfs == 0，确保 numvfs <= totalvfs。
第二种方法是面向新的/未来的 VF 设备所推荐的方法。

### 如何使用虚拟功能（VF）

VF 在内核中被当作热插拔的 PCI 设备处理，因此应能像真实 PCI 设备一样工作。VF 需要与其普通 PCI 设备相同的设备驱动。

## 开发者指南


### SR-IOV API


启用 SR-IOV 能力：

```

	int pci_enable_sriov(struct pci_dev *dev, int nr_virtfn);

```
`nr_virtfn` 为要启用的 VF 数量。

```

	echo 'nr_virtfn' > \
        /sys/bus/pci/devices/<DOMAIN:BUS:DEVICE.FUNCTION>/sriov_numvfs

```
禁用 SR-IOV 能力：

```

	void pci_disable_sriov(struct pci_dev *dev);

```
```

	echo  0 > \
        /sys/bus/pci/devices/<DOMAIN:BUS:DEVICE.FUNCTION>/sriov_numvfs

```
要启用主机上由兼容驱动自动探测 VF（默认行为），请在启用 SR-IOV 能力之前运行以下命令。
```

	echo 1 > \
        /sys/bus/pci/devices/<DOMAIN:BUS:DEVICE.FUNCTION>/sriov_drivers_autoprobe

```
要禁用主机上由兼容驱动自动探测 VF，请在启用 SR-IOV 能力之前运行以下命令。更新此条目不会影响已被探测到的 VF。
```

	echo  0 > \
        /sys/bus/pci/devices/<DOMAIN:BUS:DEVICE.FUNCTION>/sriov_drivers_autoprobe

```
### 使用示例


以下代码片段演示了 SR-IOV API 的用法。
```

	static int dev_probe(struct pci_dev *dev, const struct pci_device_id *id)
	{
		pci_enable_sriov(dev, NR_VIRTFN);

		...

		return 0;
	}

	static void dev_remove(struct pci_dev *dev)
	{
		pci_disable_sriov(dev);

		...
	}

	static int dev_suspend(struct device *dev)
	{
		...

		return 0;
	}

	static int dev_resume(struct device *dev)
	{
		...

		return 0;
	}

	static void dev_shutdown(struct pci_dev *dev)
	{
		...
	}

	static int dev_sriov_configure(struct pci_dev *dev, int numvfs)
	{
		if (numvfs > 0) {
			...
			pci_enable_sriov(dev, numvfs);
			...
			return numvfs;
		}
		if (numvfs == 0) {
			....
			pci_disable_sriov(dev);
			...
			return 0;
		}
	}

	static struct pci_driver dev_driver = {
		.name =		"SR-IOV Physical Function driver",
		.id_table =	dev_id_table,
		.probe =	dev_probe,
		.remove =	dev_remove,
		.driver.pm =	&dev_pm_ops,
		.shutdown =	dev_shutdown,
		.sriov_configure = dev_sriov_configure,
	};

```
