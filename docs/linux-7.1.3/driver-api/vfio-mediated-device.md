
## VFIO Mediated 设备


:Copyright: |copy| 2016, NVIDIA CORPORATION. 全部 rights reserved.
:Author: Neo Jia <cjia@nvidia.com>
:Author: Kirti Wankhede <kwankhede@nvidia.com>



## 虚拟 函数 I/O (VFIO) Mediated 设备[1]


The 数字 的 使用 cases 用于 virtualizing DMA 设备 该 执行 不 具有 built-in
SR_IOV capability 是 increasing. Previously, 到 virtualize 此类 设备,
developers 曾有 到 创建 它们的 own 管理 interfaces 和 APIs, 和 然后
integrate them 与 用户空间 软件. 到 simplify integration 与 用户空间
软件, 我们 具有 identified 通用 requirements 和 一个 unified 管理
接口 用于 此类 设备.

The VFIO 驱动 framework 提供 unified APIs 用于 direct 设备 access. 它是
一个 IOMMU/device-agnostic framework 用于 exposing direct 设备 access 到 用户
space 在 一个 secure, IOMMU-protected environment. 此 framework 是 使用 用于
多个 设备, 例如 GPUs, 网络 adapters, 和 compute accelerators. 与
direct 设备 access, 虚拟 machines 或 用户空间 applications 具有 direct
access 到 the 物理 设备. 此 framework 是 reused 用于 mediated 设备.

The mediated 核心 驱动 提供 一个 通用 接口 用于 mediated 设备
管理 该 可 为 使用 由 驱动 的 不同 设备. 此 模块
提供 一个 generic 接口 到 perform 这些 操作:

- 创建 和 destroy 一个 mediated 设备
- Add 一个 mediated 设备 到 和 remove 它 来自 一个 mediated 总线 驱动
- Add 一个 mediated 设备 到 和 remove 它 来自 一个 IOMMU group

The mediated 核心 驱动 也 提供 一个 接口 到 注册 一个 总线 驱动.
例如, the mediated VFIO mdev 驱动 是 designed 用于 mediated 设备 和
supports VFIO APIs. The mediated 总线 驱动 adds 一个 mediated 设备 到 和
removes 它 来自 一个 VFIO group.

The 以下 high-level 块 diagram 显示 the 主要 components 和 interfaces
在 the VFIO mediated 驱动 framework. The diagram 显示 NVIDIA, Intel, 和 IBM
```

     +---------------+
     |               |
     | +-----------+ |  mdev_register_driver() +--------------+
     | |           | +<------------------------+              |
     | |  mdev     | |                         |              |
     | |  bus      | +------------------------>+ vfio_mdev.ko |<-> VFIO user
     | |  driver   | |     probe()/remove()    |              |    APIs
     | |           | |                         +--------------+
     | +-----------+ |
     |               |
     |  MDEV CORE    |
     |   MODULE      |
     |   mdev.ko     |
     | +-----------+ |  mdev_register_parent() +--------------+
     | |           | +<------------------------+              |
     | |           | |                         | ccw_device.ko|<-> physical
     | |           | +------------------------>+              |    device
     | |           | |        callbacks        +--------------+
     | | Physical  | |
     | |  device   | |  mdev_register_parent() +--------------+
     | | interface | |<------------------------+              |
     | |           | |                         |  i915.ko     |<-> physical
     | |           | +------------------------>+              |    device
     | |           | |        callbacks        +--------------+
     | +-----------+ |
     +---------------+


```
## Registration Interfaces


The mediated 核心 驱动 提供 the 以下 types 的 registration
interfaces:

- Registration 接口 用于 一个 mediated 总线 驱动
- 物理 设备 驱动 接口

### Registration 接口 用于 一个 Mediated 总线 驱动


The registration 接口 用于 一个 mediated 设备 驱动 提供 the 以下
```

     /*
      * struct mdev_driver [2] - Mediated device's driver
      * @probe: called when new device created
      * @remove: called when device removed
      * @driver: device driver structure
      */
     struct mdev_driver {
	     int  (*probe)  (struct mdev_device *dev);
	     void (*remove) (struct mdev_device *dev);
	     unsigned int (*get_available)(struct mdev_type *mtype);
	     ssize_t (*show_description)(struct mdev_type *mtype, char *buf);
	     struct device_driver    driver;
     };

```
一个 mediated 总线 驱动 用于 mdev 应当 使用 此 结构体 在 the 函数 calls
到 注册 和 注销 itself 与 the 核心 驱动:

```

    int mdev_register_driver(struct mdev_driver *drv);

```
```

    void mdev_unregister_driver(struct mdev_driver *drv);

```
The mediated 总线 驱动's probe 函数 应当 创建 一个 vfio_设备 在…之上
the mdev_设备 和 connect 它 到 一个 appropriate implementation 的
vfio_设备_ops.

当 一个 驱动 wants 到 add the GUID creation sysfs 到 一个 existing 设备 它 具有
```

    int mdev_register_parent(struct mdev_parent *parent, struct device *dev,
			struct mdev_driver *mdev_driver);

```
此 将 提供 the 'mdev_受支持_types/XX/创建' 文件 其 可 然后 为
使用 到 trigger the creation 的 一个 mdev_设备. The 已创建 mdev_设备 将 为
attached 到 the specified 驱动.

```

    void mdev_unregister_parent(struct mdev_parent *parent);

```
其 将 unbind 和 destroy 全部 the 已创建 mdevs 和 remove the sysfs 文件.

## Mediated 设备 管理 接口 Through sysfs


The 管理 接口 through sysfs enables 用户空间 软件, 例如
libvirt, 到 query 和 configure mediated 设备 在 一个 hardware-agnostic fashion.
此 管理 接口 提供 flexibility 到 the underlying 物理
设备's 驱动 到 支持 特性 例如:

- Mediated 设备 hot plug
- 多个 mediated 设备 在 一个 单个 虚拟 machine
- 多个 mediated 设备 来自 不同 物理 设备

### Links 在 the mdev_总线 类 Directory

The /sys/类/mdev_总线/ directory 包含 links 到 设备 该 是 registered
与 the mdev 核心 驱动.

### Directories 和 文件 在…下 the sysfs 用于 每个 物理 设备


```

  |- [parent physical device]
  |--- Vendor-specific-attributes [optional]
  |--- [mdev_supported_types]
  |     |--- [<type-id>]
  |     |   |--- create
  |     |   |--- name
  |     |   |--- available_instances
  |     |   |--- device_api
  |     |   |--- description
  |     |   |--- [devices]
  |     |--- [<type-id>]
  |     |   |--- create
  |     |   |--- name
  |     |   |--- available_instances
  |     |   |--- device_api
  |     |   |--- description
  |     |   |--- [devices]
  |     |--- [<type-id>]
  |          |--- create
  |          |--- name
  |          |--- available_instances
  |          |--- device_api
  |          |--- description
  |          |--- [devices]

```
- [mdev_受支持_types]

  The 列出 的 currently 受支持 mediated 设备 types 和 它们的 details.

  [<type-id>], 设备_api, 和 可用_instances 是 mandatory attributes
  该 应当 为 provided 由 厂商 驱动.

- [<type-id>]

  The [<type-id>] name 是 已创建 由 adding the 设备 驱动 字符串 作为 一个 prefix
  到 the 字符串 provided 由 the 厂商 驱动. 此 格式 的 此 name 是 作为
```

	sprintf(buf, "%s-%s", dev_driver_string(parent->dev), group->name);

```
- 设备_api

  此 attribute 显示 其 设备 API 是 正在 已创建, 例如,
  "vfio-PCI" 用于 一个 PCI 设备.

- 可用_instances

  此 attribute 显示 the 数字 的 设备 的 类型 <type-id> 该 可 为
  已创建.

- [设备]

  此 directory 包含 links 到 the 设备 的 类型 <type-id> 该 具有 已经
  已创建.

- name

  此 attribute 显示 一个 human readable name.

- description

  此 attribute 可 显示 brief 特性/description 的 the 类型. 这是 一个
  可选 attribute.

### Directories 和 文件 在…下 the sysfs 用于 每个 mdev 设备


```

  |- [parent phy device]
  |--- [$MDEV_UUID]
         |--- remove
         |--- mdev_type {link to its type}
         |--- vendor-specific-attributes [optional]

```
- remove (写入 仅)

Writing '1' 到 the 'remove' 文件 destroys the mdev 设备. The 厂商 驱动 可
fail the remove() 回调函数 若 该 设备 是 active 和 the 厂商 驱动
doesn't 支持 hot unplug.

```

	# echo 1 > /sys/bus/mdev/devices/$mdev_UUID/remove

```
### Mediated 设备 Hot plug


Mediated 设备 可 为 已创建 和 assigned 在 runtime. The procedure 到 hot
plug 一个 mediated 设备 是 the 相同 作为 the procedure 到 hot plug 一个 PCI 设备.

## Translation APIs 用于 Mediated 设备


The 以下 APIs 是 provided 用于 translating 用户 pfn 到 host pfn 在 一个 VFIO
```

	int vfio_pin_pages(struct vfio_device *device, dma_addr_t iova,
				  int npage, int prot, struct page **pages);

	void vfio_unpin_pages(struct vfio_device *device, dma_addr_t iova,
				    int npage);

```
这些 函数 call back 进入 the back-end IOMMU 模块 由 使用 the pin_页
和 unpin_页 callbacks 的 the 结构体 vfio_iommu_驱动_ops[^4^]. Currently
这些 callbacks 是 受支持 在 the 类型1 IOMMU 模块. 到 启用 them 用于
其他 IOMMU backend 模块, 例如 PPC64 sPAPR 模块, 它们 需要 到 提供
这些 two 回调函数 函数.

## References


1. 参见 Documentation/driver-api/vfio.rst 用于 更多 information 在 VFIO.
2. 结构体 mdev_驱动 在 包含/linux/mdev.h
3. 结构体 mdev_parent_ops 在 包含/linux/mdev.h
4. 结构体 vfio_iommu_驱动_ops 在 包含/linux/vfio.h
