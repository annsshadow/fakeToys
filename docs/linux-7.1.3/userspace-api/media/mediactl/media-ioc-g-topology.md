######## ioctl MEDIA_IOC_G_TOPOLOGY


## 名称（Name）


MEDIA_IOC_G_TOPOLOGY - 枚举图拓扑和图元素属性

## 概要（Synopsis）


`int ioctl(int fd, MEDIA_IOC_G_TOPOLOGY, struct media_v2_topology *argp)`

## 参数（Arguments）


`fd`
    `open()` 返回的文件描述符。

`argp`
    指向 struct `media_v2_topology` 的指针。

## 描述（Description）


该 ioctl 的典型用法是调用两次。第一次调用时，struct
`media_v2_topology` 定义的结构体应被清零。返回时，如果没有错误发生，该 ioctl 将返回
`topology_version` 以及实体、接口、pad 和链接的总数。

在第二次调用之前，用户空间应分配数组来存储所需的图元素，将指向它们的指针放到
ptr_entities、ptr_interfaces、ptr_links 和/或 ptr_pads，其余值保持不变。

如果 `topology_version` 保持不变，该 ioctl 应使用媒体图元素填充所需的数组。



    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u64
       - `topology_version`
       - 媒体图拓扑的版本。当图被创建时，该字段从零开始。每当有图元素被
	  添加或移除，该字段都会递增。

    - -  __u32
       - `num_entities`
       - 图中实体的数量

    - -  __u32
       - `reserved1`
       - 应用程序和驱动应将其设为 0。

    - -  __u64
       - `ptr_entities`
       - 指向将存储 entities 数组的内存区域的指针，转换为 64 位整数。
	  它可以为零。如果为零，该 ioctl 将不会存储 entities，而只会更新
	  `num_entities`

    - -  __u32
       - `num_interfaces`
       - 图中接口的数量

    - -  __u32
       - `reserved2`
       - 应用程序和驱动应将其设为 0。

    - -  __u64
       - `ptr_interfaces`
       - 指向将存储 interfaces 数组的内存区域的指针，转换为 64 位整数。
	  它可以为零。如果为零，该 ioctl 将不会存储 interfaces，而只会更新
	  `num_interfaces`

    - -  __u32
       - `num_pads`
       - 图中 pad 的总数

    - -  __u32
       - `reserved3`
       - 应用程序和驱动应将其设为 0。

    - -  __u64
       - `ptr_pads`
       - 指向将存储 pads 数组的内存区域的指针，转换为 64 位整数。
	  它可以为零。如果为零，该 ioctl 将不会存储 pads，而只会更新
	  `num_pads`

    - -  __u32
       - `num_links`
       - 图中数据与接口链接的总数

    - -  __u32
       - `reserved4`
       - 应用程序和驱动应将其设为 0。

    - -  __u64
       - `ptr_links`
       - 指向将存储 links 数组的内存区域的指针，转换为 64 位整数。
	  它可以为零。如果为零，该 ioctl 将不会存储 links，而只会更新
	  `num_links`



    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `id`
       - 实体的唯一 ID。不要期望该 ID 对设备的每个实例都
	  始终相同。换句话说，不要在应用程序中硬编码实体 ID。

    - -  char
       - `name`\ [^64^]
       - 实体名称，作为以 UTF-8 NULL 结尾的字符串。该名称在媒体拓扑内
	  必须唯一。

    - -  __u32
       - `function`
       - 实体的主功能，详见 media-entity-functions。

    - -  __u32
       - `flags`
       - 实体标志，详见 media-entity-flag。
	  仅当 `MEDIA_V2_ENTITY_HAS_FLAGS(media_version)` 返回
	  true 时有效。`media_version` 定义于 struct
	  `media_device_info` 中，可通过
	  MEDIA_IOC_DEVICE_INFO 获取。

    - -  __u32
       - `reserved`\ [^5^]
       - 为未来扩展保留。驱动和应用程序必须将该数组设为零。



    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `id`
       - 接口的唯一 ID。不要期望该 ID 对设备的每个实例都
	  始终相同。换句话说，不要在应用程序中硬编码接口 ID。

    - -  __u32
       - `intf_type`
       - 接口类型，详见 media-intf-type。

    - -  __u32
       - `flags`
       - 接口标志。当前未使用。

    - -  __u32
       - `reserved`\ [^9^]
       - 为未来扩展保留。驱动和应用程序必须将该数组设为零。

    - -  struct media_v2_intf_devnode
       - `devnode`
       - 仅用于设备节点接口。详见
	  `media_v2_intf_devnode`。



    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `major`
       - 设备节点主设备号。

    - -  __u32
       - `minor`
       - 设备节点次设备号。



    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `id`
       - pad 的唯一 ID。不要期望该 ID 对设备的每个实例都
	  始终相同。换句话说，不要在应用程序中硬编码 pad ID。

    - -  __u32
       - `entity_id`
       - 此 pad 所属实体的唯一 ID。

    - -  __u32
       - `flags`
       - pad 标志，详见 media-pad-flag。

    - -  __u32
       - `index`
       - pad 索引，从 0 开始。仅当 `MEDIA_V2_PAD_HAS_INDEX(media_version)`
	  返回 true 时有效。`media_version` 定义于 struct
	  `media_device_info` 中，可通过 MEDIA_IOC_DEVICE_INFO 获取。

    - -  __u32
       - `reserved`\ [^4^]
       - 为未来扩展保留。驱动和应用程序必须将该数组设为零。



    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `id`
       - 链接的唯一 ID。不要期望该 ID 对设备的每个实例都
	  始终相同。换句话说，不要在应用程序中硬编码链接 ID。

    - -  __u32
       - `source_id`
       - 对于 pad 到 pad 的链接：源 pad 的唯一 ID。

	  对于接口到实体的链接：接口的唯一 ID。

    - -  __u32
       - `sink_id`
       - 对于 pad 到 pad 的链接：sink pad 的唯一 ID。

	  对于接口到实体的链接：实体的唯一 ID。

    - -  __u32
       - `flags`
       - 链接标志，详见 media-link-flag。

    - -  __u32
       - `reserved`\ [^6^]
       - 为未来扩展保留。驱动和应用程序必须将该数组设为零。

## 返回值（Return Value）


成功时返回 0，出错时返回 -1 并设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述。

ENOSPC
    当 num_entities、num_interfaces、num_links 或 num_pads 中有一个或多个非零，且
    小于图中实际元素数量时返回。如果 `topology_version` 与上次调用此 ioctl 时相比
    发生了变化，就可能发生这种情况。用户空间通常应释放指针所指向的区域，将结构体元素
    清零，然后再次调用此 ioctl。
