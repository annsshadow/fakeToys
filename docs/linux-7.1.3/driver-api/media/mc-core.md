
### 媒体控制器设备

#### 媒体控制器

媒体控制器用户空间 API 记录在 Media Controller uAPI book <media_controller> 中。本文档关注媒体框架的内核侧实现。

##### 抽象媒体设备模型

发现设备内部拓扑并运行时配置它，是媒体框架的目标之一。为实现这一点，硬件设备被建模为一个由称为实体（entities）的构建块通过 pad 连接而成的有向图。

实体（entity）是一个基本的媒体硬件构建块。它可以对应于各种各样的逻辑块，例如物理硬件设备（比如 CMOS 传感器）、逻辑硬件设备（SoC 图像处理流水线中的一个构建块）、DMA 通道或物理连接器。

Pad 是一个连接端点，实体通过它与其他实体交互。由实体产生的数据（不限于视频）从实体的输出流向一个或多个实体输入。Pad 不应与芯片边界上的物理引脚混淆。

Link（链接）是两个 pad 之间的点对点有向连接，可以在同一实体上，也可以在不同实体上。数据从源 pad 流向宿（sink）pad。

##### 媒体设备

媒体设备由一个 `struct media_device` 实例表示，定义于 `include/media/media-device.h`。该结构的分配由媒体设备驱动处理，通常是将 `media_device` 实例内嵌在一个更大的驱动特定结构中。

驱动通过调用 `media_device_init()` 初始化媒体设备实例。初始化媒体设备实例后，通过宏 `media_device_register()` 调用 `__media_device_register()` 来注册它，并通过调用 `media_device_unregister()` 来注销它。一个已初始化的媒体设备最终必须通过调用 `media_device_cleanup()` 进行清理。

注意，不允许注销一个先前未注册的媒体设备实例，或清理一个先前未初始化的媒体设备实例。

##### 实体

实体由 `struct media_entity` 实例表示，定义于 `include/media/media-entity.h`。该结构通常内嵌到更高层的结构中，例如 `v4l2_subdev` 或 `video_device` 实例，尽管驱动也可以直接分配实体。

驱动通过调用 `media_entity_pads_init()` 初始化实体的 pad。

驱动通过调用 `media_device_register_entity()` 将实体注册到媒体设备，并通过调用 `media_device_unregister_entity()` 注销。

##### 接口

接口由 `struct media_interface` 实例表示，定义于 `include/media/media-entity.h`。目前只定义了一种接口类型：设备节点（device node）。此类接口由 `struct media_intf_devnode` 表示。

驱动通过调用 `media_devnode_create()` 初始化并创建设备节点接口，并通过调用 `media_devnode_remove()` 移除它们。

##### Pads

Pad 由 `struct media_pad` 实例表示，定义于 `include/media/media-entity.h`。每个实体将其 pad 存储在由实体驱动管理的 pad 数组中。驱动通常将该数组内嵌在驱动特定的结构中。

Pad 通过其所属的实体以及它们在 pad 数组中的从 0 开始的索引来标识。

这两类信息都存储在 `struct media_pad` 中，使得 `struct media_pad` 指针成为存储和传递 link 引用的规范方式。

Pad 具有描述其能力与状态的标志。

`MEDIA_PAD_FL_SINK` 表示该 pad 支持接收（sinking）数据。
`MEDIA_PAD_FL_SOURCE` 表示该 pad 支持产生（sourcing）数据。

  每个 pad 必须且仅必须设置 `MEDIA_PAD_FL_SINK` 或 `MEDIA_PAD_FL_SOURCE` 之一。

##### 链接

链接由 `struct media_link` 实例表示，定义于 `include/media/media-entity.h`。有两种类型的链接：

**1. pad 到 pad 链接**：

通过 PAD 关联两个实体。每个实体都有一个列表，指向所有源自或指向其任一 pad 的链接。因此，给定的链接被存储两次，一次在源实体中，一次在目标实体中。

驱动通过调用 `media_create_pad_link()` 创建 pad 到 pad 链接，并通过 `media_entity_remove_links()` 移除。

**2. interface 到 entity 链接**：

将一个接口关联到一个链接。

驱动通过调用 `media_create_intf_link()` 创建 interface 到 entity 链接，并通过 `media_remove_intf_links()` 移除。

   链接只能在两端都已创建之后创建。

链接具有描述其能力与状态的标志。有效值在 `media_create_pad_link()` 和 `media_create_intf_link()` 中描述。

##### 图遍历

媒体框架提供了遍历媒体图、定位相连实体和链接的 API。

要遍历属于某个媒体设备的所有实体，驱动可以使用 `media_device_for_each_entity` 宏，定义于 `include/media/media-device.h`。


    struct media_entity *entity;

    media_device_for_each_entity(entity, mdev) {
    // entity 将依次指向每个实体
    ...
    }

辅助函数可用于查找两个给定 pad 之间的链接，或通过已启用链接连接到另一个 pad 的 pad
（`media_entity_find_link()`、`media_pad_remote_pad_first()`、
`media_entity_remote_source_pad_unique()` 和
`media_pad_remote_pad_unique()`）。

##### 使用计数与电源处理

由于驱动在电源管理需求方面差异很大，媒体控制器不实现电源管理。不过，`struct media_entity` 包含一个 `use_count` 字段，媒体驱动可以使用它来跟踪每个实体的用户数量以满足电源管理需求。

`media_entity`.\ `use_count` 字段归媒体驱动所有，实体驱动不得触碰。对该字段的访问必须由 `media_device`.\ `graph_mutex` 锁保护。

##### 链接设置

链接属性可以通过调用 `media_entity_setup_link()` 在运行时修改。

##### 流水线与媒体流

媒体流（media stream）是源自一个或多个源设备（例如传感器）并流经媒体实体 pad 到达最终宿的像素或元数据流。该流可以在路径上被设备修改（例如缩放或像素格式转换），也可以被拆分为多个分支，或者多个分支可以被合并。

媒体流水线（media pipeline）是一组相互依赖的媒体流。这种相互依赖可能是由硬件引起的（例如，如果第一条流已使能，则第二条流配置无法更改），或由驱动由于软件设计引起。最常见的情况是，媒体流水线由一条不分叉的单个流组成。

开始流式传输时，驱动必须通知流水线中的所有实体，以防止在流式传输期间链接状态被修改，方法是调用 `media_pipeline_start()`。

该函数会将流水线中所有作为流水线一部分的 pad 标记为正在流式传输。

`pipe` 参数指向的 `struct media_pipeline` 实例将存储在流水线中的每个 pad 中。驱动应将 `struct media_pipeline` 内嵌到更高层的流水线结构中，然后可以通过 `struct media_pad` 的 pipe 字段访问该流水线。

对 `media_pipeline_start()` 的调用可以嵌套。所有嵌套调用该函数时，流水线指针必须相同。

`media_pipeline_start()` 可能返回错误。在这种情况下，它会自行清理它所做的任何更改。

停止流时，驱动必须通过 `media_pipeline_stop()` 通知实体。

如果多次调用 `media_pipeline_start()`，则需要相同次数的 `media_pipeline_stop()` 调用才能停止流式传输。在最后一个嵌套 stop 调用时，`media_entity`.\ `pipe` 字段被重置为 `NULL`。

如果链接的任一端是正在流式传输的实体，默认情况下链接配置将失败并返回 `-EBUSY`。在流式传输期间可以修改的链接必须标记为 `MEDIA_LNK_FL_DYNAMIC` 标志。

如果其他操作需要被禁止在流式传输的实体上（例如更改实体配置参数），驱动可以显式检查 media_entity 的 stream_count 字段以查明某个实体是否正在流式传输。此操作必须在持有 media_device graph_mutex 的情况下进行。

##### 链接验证

`media_pipeline_start()` 会对流水线中任何具有宿 pad 的实体执行链接验证。为此使用 `media_entity`.\ `link_validate()` 回调。在 `link_validate()` 回调中，实体驱动应检查相连实体的源 pad 的属性与其自身的宿 pad 是否匹配。实际匹配的含义取决于实体的类型（最终取决于硬件的属性）。

子系统应当通过提供子系统特定的辅助函数来便于链接验证，以便轻松访问通常需要的信息，并最终提供一种使用驱动特定回调的方式。

##### 流水线遍历

一旦使用 `media_pipeline_start()` 构建好流水线，驱动就可以使用 `:c:macro:´media_pipeline_for_each_entity` 和 `:c:macro:´media_pipeline_for_each_pad` 宏遍历流水线中的实体或 pad。遍历 pad 是直接的：


   media_pipeline_pad_iter iter;
   struct media_pad *pad;

   media_pipeline_for_each_pad(pipe, &iter, pad) {
       /** 'pad' 将依次指向每个 pad **/
       ...
   }

要遍历实体，作为额外步骤，迭代器需要被初始化和清理：


   media_pipeline_entity_iter iter;
   struct media_entity *entity;
   int ret;

   ret = media_pipeline_entity_iter_init(pipe, &iter);
   if (ret)
       ...;

   media_pipeline_for_each_entity(pipe, &iter, entity) {
       /** 'entity' 将依次指向每个实体 **/
       ...
   }

   media_pipeline_entity_iter_cleanup(&iter);

##### 媒体控制器设备分配器 API

当媒体设备属于多个驱动时，共享的媒体设备使用共享的 struct device 作为查找的键来分配。

共享媒体设备应一直保持注册状态，直到最后一个驱动注销它。此外，当所有引用都被释放时，媒体设备才应被释放。每个驱动在探测（probe）期间分配媒体设备时获得对媒体设备的一个引用。如果媒体设备已被分配，分配 API 会增加引用计数并返回现有的媒体设备。驱动在其断开连接（disconnect）例程中调用 `media_device_delete()` 时将该引用放回。

媒体设备从 kref put 处理程序进行注销和清理，以确保媒体设备保持注册状态，直到最后一个驱动注销媒体设备。

**驱动用法**

驱动应使用适当的 media-core 例程来管理共享媒体设备的生命周期，处理两种状态：
1. allocate -> register -> delete
2. 获取对已注册设备的引用 -> delete

调用 `media_device_delete()` 例程以确保共享媒体设备的删除被正确处理。

**驱动探测（probe）：**
调用 `media_device_usb_allocate()` 来分配或获取引用
如果媒体 devnode 尚未注册，则调用 `media_device_register()`

**驱动断开连接（disconnect）：**
调用 `media_device_delete()` 释放 media_device。释放由 kref put 处理程序处理。

##### API 定义

















