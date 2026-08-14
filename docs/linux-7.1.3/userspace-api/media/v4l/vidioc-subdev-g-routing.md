


######## ioctl VIDIOC_SUBDEV_G_ROUTING, VIDIOC_SUBDEV_S_ROUTING


## 名称


VIDIOC_SUBDEV_G_ROUTING - VIDIOC_SUBDEV_S_ROUTING - 获取或设置媒体实体中媒体 pad 之间的流路由。

## 概要


`int ioctl(int fd, VIDIOC_SUBDEV_G_ROUTING, struct v4l2_subdev_routing *argp)`

`int ioctl(int fd, VIDIOC_SUBDEV_S_ROUTING, struct v4l2_subdev_routing *argp)`

## 参数


`fd`
    由 open() <func-open> 返回的文件描述符。

`argp`
    指向 struct `v4l2_subdev_routing` 的指针。

## 描述


这些 ioctl 用于获取和设置媒体实体中的路由。
路由配置决定了实体内部的数据流。

驱动使用 `VIDIOC_SUBDEV_G_ROUTING` ioctl 报告其当前路由表，
而应用程序可以通过添加或移除路由、以及设置或清除
struct `v4l2_subdev_route` 的 `flags` 字段中的标志，
使用 `VIDIOC_SUBDEV_S_ROUTING` ioctl 来启用或禁用路由。
与 `VIDIOC_SUBDEV_G_ROUTING` 类似，`VIDIOC_SUBDEV_S_ROUTING`
也会将路由返回给用户。

当调用 `VIDIOC_SUBDEV_S_ROUTING` 时，所有流配置都会被重置。
这意味着用户空间必须在调用该 ioctl 之后，例如使用
`VIDIOC_SUBDEV_S_FMT` 重新配置所有的流格式与选择（selections）。

只有同时具有 sink 与 source pad 的子设备才能支持路由。

`len_routes` 字段表示用户空间分配的 `routes` 数组中
能够容纳的路由数量。它由应用程序为两个 ioctl 设置，
以指示内核可以返回多少条路由，并且决不会被内核修改。

`num_routes` 字段表示路由表中的路由数量。
对于 `VIDIOC_SUBDEV_S_ROUTING`，它由用户空间设置为
应用程序存储在 `routes` 数组中的路由数量。对于两个 ioctl，
它都由内核返回，并指示子设备路由表中存储了多少条路由。
这可能小于或大于应用程序为 `VIDIOC_SUBDEV_S_ROUTING`
设置的 `num_routes` 值，因为驱动可能会调整所请求的路由表。

内核可以从两个 ioctl 返回比 `len_routes` 更大的 `num_routes` 值。
这表示路由表中的路由数量多于 `routes` 数组所能容纳的。
在这种情况下，内核会用子设备路由表的前 `len_routes` 个
条目填充 `routes` 数组。这不被视为错误，ioctl 调用会成功。
如果应用程序希望取回缺失的路由，它可以发出一个新的
`VIDIOC_SUBDEV_G_ROUTING` 调用，并提供一个足够大的 `routes` 数组。

`VIDIOC_SUBDEV_S_ROUTING` 可能会返回比用户在 `num_routes`
字段中提供的更多路由，例如由于硬件特性。




    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `which`
      - 要访问的路由表，来自 enum
        v4l2_subdev_format_whence <v4l2-subdev-format-whence>。
    - - __u32
      - `len_routes`
      - 数组的长度（即数组所占用的内存）。
    - - struct `v4l2_subdev_route`
      - `routes[]`
      - struct `v4l2_subdev_route` 条目组成的数组。
    - - __u32
      - `num_routes`
      - routes 数组的条目数量。
    - - __u32
      - `reserved`\ [^11^]
      - 为未来扩展保留。应用程序与驱动必须将该数组
	设置为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `sink_pad`
      - Sink pad 编号。
    - - __u32
      - `sink_stream`
      - Sink pad 流编号。
    - - __u32
      - `source_pad`
      - Source pad 编号。
    - - __u32
      - `source_stream`
      - Source pad 流编号。
    - - __u32
      - `flags`
      - 路由启用/禁用标志
	v4l2_subdev_routing_flags <v4l2-subdev-routing-flags>。
    - - __u32
      - `reserved`\ [^5^]
      - 为未来扩展保留。应用程序与驱动必须将该数组
	设置为零。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - V4L2_SUBDEV_ROUTE_FL_ACTIVE
      - 0x0001
      - 该路由已启用。由应用程序设置。

## 返回值


成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。
通用错误码在通用错误码 <gen-errors> 一章中描述。

EINVAL
    sink 或 source pad 标识符引用了不存在的 pad，或者引用了
    不同类型的 pad（即 sink_pad 标识符引用了一个 source pad），
    `which` 字段的值不受支持，或者对于 `VIDIOC_SUBDEV_S_ROUTING`，
    应用程序设置的 num_routes 字段大于 len_routes 字段的值。

ENXIO
    应用程序请求的路由无法创建，或者指定路由的状态
    无法修改。仅针对 `VIDIOC_SUBDEV_S_ROUTING` 返回。

E2BIG
    应用程序为 `VIDIOC_SUBDEV_S_ROUTING` 提供的 `num_routes`
    大于驱动所能处理的路由数量。
