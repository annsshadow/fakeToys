
## VFIO Mediated 设备


:Copyright: |copy| 2016, NVIDIA CORPORATION. 全部 rights reserved.
:Author: Neo Jia <cjia@nvidia.com>
:Author: Kirti Wankhede <kwankhede@nvidia.com>



## 虚拟 函数 I/O (VFIO) Mediated 设备[1]


The 数字 使用 cases 用于 virtualizing DMA 设备 执行 具有 built-in
SR_IOV capability increasing. Previously, virtualize 此类 设备,
developers 曾有 创建 它们own 管理 interfaces APIs, 然后
integrate them 用户空间 软件. simplify integration 用户空间
软件, 我们 具有 identified 通用 requirements 一unified 管理
接口 用于 此类 设备.

The VFIO 驱动 framework 提供 unified APIs 用于 direct 设备 access. 它是
一IOMMU/device-agnostic framework 用于 exposing direct 设备 access 用户
space 一secure, IOMMU-protected environment. framework 使用 用于
多个 设备, 例如 GPUs, 网络 adapters, compute accelerators. 
direct 设备 access, 虚拟 machines 用户空间 applications 具有 direct
access the 物理 设备. framework reused 用于 mediated 设备.

The mediated 核心 驱动 提供 一通用 接口 用于 mediated 设备
管理 使用 驱动 不同 设备. 模块
提供 一generic 接口 perform 这些 操作:

- 创建 destroy 一mediated 设备
- Add 一mediated 设备 remove 来自 一mediated 总线 驱动
- Add 一mediated 设备 remove 来自 一IOMMU group

The mediated 核心 驱动 提供 一接口 注册 一总线 驱动.
例如, the mediated VFIO mdev 驱动 designed 用于 mediated 设备 
supports VFIO APIs. The mediated 总线 驱动 adds 一mediated 设备 
removes 来自 一VFIO group.

The 以下 high-level diagram 显示 the 主要 components interfaces
the VFIO mediated 驱动 framework. The diagram 显示 NVIDIA, Intel, IBM
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


The mediated 核心 驱动 提供 the 以下 types registration
interfaces:

- Registration 接口 用于 一mediated 总线 驱动
- 物理 设备 驱动 接口

### Registration 接口 用于 一Mediated 总线 驱动


The registration 接口 用于 一mediated 设备 驱动 提供 the 以下
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
一mediated 总线 驱动 用于 mdev 应当 使用 结构the 函数 calls
注册 注销 itself the 核心 驱动:

```

    int mdev_register_driver(struct mdev_driver *drv);

```
```

    void mdev_unregister_driver(struct mdev_driver *drv);

```
The mediated 总线 驱动's probe 函数 应当 创建 一vfio_设备 在…之
the mdev_设备 connect 一appropriate implementation 
vfio_设备_ops.

一驱动 wants add the GUID creation sysfs 一existing 设备 具有
```

    int mdev_register_parent(struct mdev_parent *parent, struct device *dev,
			struct mdev_driver *mdev_driver);

```
提供 the 'mdev_受支持_types/XX/创建' 文件 然后 
使用 trigger the creation 一mdev_设备. The 已创mdev_设备 
attached the specified 驱动.

```

    void mdev_unregister_parent(struct mdev_parent *parent);

```
unbind destroy 全部 the 已创mdevs remove the sysfs 文件.

## Mediated 设备 管理 接口 Through sysfs


The 管理 接口 through sysfs enables 用户空间 软件, 例如
libvirt, query configure mediated 设备 一hardware-agnostic fashion.
管理 接口 提供 flexibility the underlying 物理
设备's 驱动 支持 特例如:

- Mediated 设备 hot plug
- 多个 mediated 设备 一单个 虚拟 machine
- 多个 mediated 设备 来自 不同 物理 设备

### Links the mdev_总线 Directory

The /sys/mdev_总线/ directory 包含 links 设备 registered
the mdev 核心 驱动.

### Directories 文件 在…下 the sysfs 用于 每个 物理 设备


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

  The 列出 currently 受支mediated 设备 types 它们details.

  [<type-id>], 设备_api, 可用_instances mandatory attributes
  应当 provided 厂商 驱动.

- [<type-id>]

  The [<type-id>] name 已创adding the 设备 驱动 字符作为 一prefix
  the 字符provided the 厂商 驱动. 格式 name 作为
```

	sprintf(buf, "%s-%s", dev_driver_string(parent->dev), group->name);

```
- 设备_api

  attribute 显示 设备 API 正在 已创 例如,
  "vfio-PCI" 用于 一PCI 设备.

- 可用_instances

  attribute 显示 the 数字 设备 类型 <type-id> 
  宸插垱寤。

- [设备]

  directory 包含 links the 设备 类型 <type-id> 具有 已经
  宸插垱寤。

- name

  attribute 显示 一human readable name.

- description

  attribute 显示 brief 特description the 类型. 这是 一
  可attribute.

### Directories 文件 在…下 the sysfs 用于 每个 mdev 设备


```

  |- [parent phy device]
  |--- [$MDEV_UUID]
         |--- remove
         |--- mdev_type {link to its type}
         |--- vendor-specific-attributes [optional]

```
- remove (写入 

Writing '1' the 'remove' 文件 destroys the mdev 设备. The 厂商 驱动 
fail the remove() 回调函数 设备 active the 厂商 驱动
doesn't 支持 hot unplug.

```

	# echo 1 > /sys/bus/mdev/devices/$mdev_UUID/remove

```
### Mediated 设备 Hot plug


Mediated 设备 已创assigned runtime. The procedure hot
plug 一mediated 设备 the 相同 作为 the procedure hot plug 一PCI 设备.

## Translation APIs 用于 Mediated 设备


The 以下 APIs provided 用于 translating 用户 pfn host pfn 一VFIO
```

	int vfio_pin_pages(struct vfio_device *device, dma_addr_t iova,
				  int npage, int prot, struct page **pages);

	void vfio_unpin_pages(struct vfio_device *device, dma_addr_t iova,
				    int npage);

```
这些 函数 call back 进入 the back-end IOMMU 模块 使用 the pin_
unpin_callbacks the 结构vfio_iommu_驱动_ops[^4^]. Currently
这些 callbacks 受支the 类型1 IOMMU 模块. 启用 them 用于
其他 IOMMU backend 模块, 例如 PPC64 sPAPR 模块, 它们 需提供
这些 two 回调函数 函数.

## References


1. 参见 Documentation/driver-api/vfio.rst 用于 更多 information VFIO.
2. 结构mdev_驱动 包含/linux/mdev.h
3. 结构mdev_parent_ops 包含/linux/mdev.h
4. 结构vfio_iommu_驱动_ops 包含/linux/vfio.h
