


## Linux 上的 Virtio


## 简

Virtio 是一个开放标准，定义了不同类型驱动与设备之间的通信协议，请参见
virtio 规范的第 5 章（“设备类型”）（`[^1^]`_）。它最初是作为由管理程实现的半虚拟化（paravirtualized）设备的标准开发的，但也可用于将任何兼容的
设备（真实的或仿真的）与驱动接口连接
出于说明目的，本文档将聚焦于一个常见情况：运行在虚拟机中的 Linux 内核使用管理程序提供的半虚拟化设备，管理程序通过 PCI 等标准机制将它们暴露virtio 设备
## 设备 - 驱动通信：virtqueue


尽管 virtio 设备实际上是管理程序中的一个抽象层，但它们被暴露给客户机，
就好像它们是使用特定传输方法——PCI、MMIO CCW——的物理设备，这独立设备本身。virtio 规范详细定义了这些传输方法，包括设备发现、能力与中断处理
客户机操作系统中的驱动与管理程序中的设备之间的通信是通过共享内存（这正是
virtio 设备如此高效的原因）完成的，使用称为 virtqueue 的专用数据结构，它们
实际上是缓冲区描述符的环形缓冲区（ring buffer）[#f1]_，类似于网络设备使用的那些：

    :identifiers: struct vring_desc

描述符指向的所有缓冲区都由客户机分配，并由主机用于读取或写入，但不能同用于两者
有关 virtqueue 的参考定义，请参virtio 规范的第 2.5 节（“Virtqueues”）
（`[^1^]`_），以及博客文章“Virtqueues and virtio ring: How the data travels（`[^2^]`_），了解主机设备与客户机驱动如何通信的图解概览
`vring_virtqueue` 结构体建模了一virtqueue，包括环形缓冲区与管理数据嵌入该结构体中的`virtqueue` 结构体，它是最终被 virtio 驱动使用的数结构
    :identifiers: struct virtqueue

该结构体指向的回调函数在设备消费了驱动提供的缓冲区时被触发。更具体地说触发将是管理程序发出的中断（参见 vring_interrupt()）。中断请求处理程序在
virtqueue 设置过程（传输相关）期间virtqueue 注册
    :identifiers: vring_interrupt


## 设备发现与探

在内核中，virtio 核心包含 virtio 总线驱动以及传输相关的驱动，`virtio-pci`
`virtio-mmio`。然后还有针对特定设备类型的各个 virtio 驱动，它们注册到
virtio 总线驱动
内核如何找到并配virtio 设备取决于管理程序如何定义它。以 `QEMU virtio-console
<https://gitlab.com/qemu-project/qemu/-/blob/master/hw/char/virtio-console.c>`__
设备为例。当使用 PCI 作为传输方法时，设备将以厂商 0x1af4（Red Hat, Inc.和设id 0x1003（virtio console）出现在 PCI 总线上，如规范中所定义，因内核会像对待任何其他 PCI 设备一样检测它
PCI 枚举过程中，如果发现某个设备匹配 virtio-pci 驱动（根virtio-pci
设备表，任何 PCI
```

	/* Qumranet donated their vendor ID for devices 0x1000 thru 0x10FF. */
	static const struct pci_device_id virtio_pci_id_table[] = {
		{ PCI_DEVICE(PCI_VENDOR_ID_REDHAT_QUMRANET, PCI_ANY_ID) },
		{ 0 }
	};

```
那么 virtio-pci 驱动会被探测，并且如果探测顺利，
```

	static int virtio_pci_probe(struct pci_dev *pci_dev,
				    const struct pci_device_id *id)
	{
		...

		if (force_legacy) {
			rc = virtio_pci_legacy_probe(vp_dev);
			/* Also try modern mode if we can't map BAR0 (no IO space). */
			if (rc == -ENODEV || rc == -ENOMEM)
				rc = virtio_pci_modern_probe(vp_dev);
			if (rc)
				goto err_probe;
		} else {
			rc = virtio_pci_modern_probe(vp_dev);
			if (rc == -ENODEV)
				rc = virtio_pci_legacy_probe(vp_dev);
			if (rc)
				goto err_probe;
		}

		...

		rc = register_virtio_device(&vp_dev->vdev);

```
当设备注册到 virtio 总线时，内核将在总线上寻找能够处理该设备的驱动，并调该驱动的 `probe` 方法
此时，virtqueue 将通过调用相应`virtio_find` 辅助函数来分配和配置，例virtio_find_single_vq() virtio_find_vqs()，它们最终会调用一个传输相关的
`find_vqs` 方法

## 参

_`[^1^]` Virtio Spec v1.2:
https://docs.oasis-open.org/virtio/virtio/v1.2/virtio-v1.2.html


_`[^2^]` Virtqueues and virtio ring: How the data travels
https://www.redhat.com/en/blog/virtqueues-and-virtio-ring-how-data-travels
