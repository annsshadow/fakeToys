## VME Device Drivers


### Driver registration


与其他 Linux 内核子系统一样，VME 设备驱动向 VME 子系统注册，通常从设备的 init
例程中调用。这是通过调用 `vme_register_driver` 实现的。

必须向注册函数提供一个指向 `struct vme_driver <vme_driver>` 类型结构的指针，
以及你的驱动所能支持的最大设备数量。

至少，`struct vme_driver <vme_driver>` 的 '.name'、'.match' 和 '.probe' 元素应
被正确设置。'.name' 元素是一个指向保存设备驱动名称字符串的指针。

'.match' 函数用于控制哪些 VME 设备应当注册到该驱动。如果某设备应当被探测，match
函数应返回 1，否则返回 0。下面这个 match 函数示例（来自 vme_user.c）将探测的设备
数量限制为一个：


	#define USER_BUS_MAX	1
	...
	static int vme_user_match(struct vme_dev *vdev)
	{
		if (vdev->id.num >= USER_BUS_MAX)
			return 0;
		return 1;
	}

'.probe' 元素应包含指向探测例程的指针。探测例程以一个 `struct vme_dev <vme_dev>`
指针作为参数。

这里，'num' 字段指的是该特定驱动的顺序设备 ID。桥号（或总线号）可通过
dev->bridge->num 访问。

还提供了一个用于从 VME 核心注销驱动的函数 `vme_unregister_driver`，通常应在设备
驱动的退出例程中调用。


### Resource management


一旦驱动向 VME 核心注册，所提供的 match 例程将被调用注册时指定的次数。如果匹配
成功，应返回一个非零值；返回零表示失败。对于所有成功匹配的情况，会调用对应驱动的
probe 例程。probe 例程会传入一个指向设备 device 结构的指针。该指针应当被保存，在
请求 VME 资源时会用到它。

驱动可以请求一个或多个主窗口（`vme_master_request`）、从窗口
（`vme_slave_request`）和/或 DMA 通道（`vme_dma_request`）的所有权。API 不是让设备
驱动请求特定的窗口或 DMA 通道（可能被其他驱动占用），而是根据所讨论驱动所需的
属性来分配资源。对于从窗口，这些属性分为需要访问的 VME 地址空间（'aspace'）和
所需的 VME 总线周期类型（'cycle'）。主窗口另外增加一组 'width' 属性，指定所需的
数据传输宽度。这些属性定义为位掩码，因此可以为单个窗口请求任意组合的属性；核心会
分配一个满足要求的窗口，并返回一个 vme_resource 类型的指针，用于在后续使用已分配
资源时标识它。对于 DMA 控制器，请求函数需要提供任何传输可能的方向（route 属性）。
通常是 VME-to-MEM 和/或 MEM-to-VME，不过某些硬件还能支持 VME-to-VME 与 MEM-to-MEM
传输以及测试模式生成。如果找不到符合要求的未分配窗口，将返回 NULL 指针。

还提供了在不再需要时分释放窗口分配的函数。这些函数（`vme_master_free`、
`vme_slave_free` 和 `vme_dma_free`）应传入资源分配时提供的资源指针。


### Master windows


主窗口提供从本地处理器访问 VME 总线的能力。可用窗口数量以及可用的访问模式取决于
底层芯片组。窗口在使用前必须先配置。


#### Master window configuration


主窗口分配后，可用 `vme_master_set` 配置它，用 `vme_master_get` 获取当前设置。
地址空间、传输宽度和周期类型与资源管理中描述的相同，但其中一些选项是互斥的。例如，
只能指定一个地址空间。


#### Master window access


函数 `vme_master_read` 可用于从已配置的主窗口读取，`vme_master_write` 用于写入。

除了简单的读写，`vme_master_rmw` 提供读-修改-写事务。VME 窗口的部分区域也可以使用
`vme_master_mmap_prepare` 映射到用户空间内存。


### Slave windows


从窗口提供 VME 总线上的设备访问本地内存映射区域的途径。可用窗口数量以及可使用的
访问模式取决于底层芯片组。窗口在使用前必须先配置。


#### Slave window configuration


从窗口分配后，可用 `vme_slave_set` 配置它，用 `vme_slave_get` 获取当前设置。

地址空间、传输宽度和周期类型与资源管理中描述的相同，但其中一些选项是互斥的。例如，
只能指定一个地址空间。


#### Slave window buffer allocation


提供了一些函数，允许用户分配（`vme_alloc_consistent`）和释放（`vme_free_consistent`）
一段连续的、VME 桥可访问的缓冲区。不一定要使用这些函数，也可以用其他方法分配缓冲区，
但必须注意确保它们是连续的且 VME 桥可访问。


#### Slave window access


从窗口将本地内存映射到 VME 总线，应使用访问内存的标准方法。


### DMA channels


VME DMA 传输提供运行链表 DMA 传输的能力。该 API 引入了 DMA 列表的概念。每个 DMA
列表是一个可传给 DMA 控制器的链表。可以创建、扩展、执行、复用和销毁多个列表。


#### List Management


函数 `vme_new_dma_list` 用于创建 DMA 列表，`vme_dma_list_free` 用于销毁。执行列表
不会自动销毁它，因此列表可被复用于重复性任务。


#### List Population


可以使用 `vme_dma_list_add` 向列表添加一项（源和目标的属性需要在调用该函数前创建，
这部分在“Transfer Attributes”中介绍）。


	传输源和目标的详细属性直到向 DMA 列表添加条目时才会被检查；请求 DMA 通道
	只是检查控制器预期传输数据的方向。因此这次调用有可能返回错误，例如源或
	目标位于不支持的 VME 地址空间中。

#### Transfer Attributes


源和目标的属性与向列表添加条目是分开处理的。这是因为每种类型的源和目标所需的属性
差异很大。提供了为 PCI、VME 以及 pattern（在适用时）源和目标创建属性的函数：

 - PCI 源或目标：`vme_dma_pci_attribute`
 - VME 源或目标：`vme_dma_vme_attribute`
 - Pattern 源：`vme_dma_pattern_attribute`

函数 `vme_dma_free_attribute` 应用于释放一个属性。


#### List Execution


函数 `vme_dma_list_exec` 将一个列表排入执行队列，并在列表执行完毕后返回。


### Interrupts


VME API 提供了将回调函数挂接/分离到特定 VME 电平与状态 ID 组合，以及以特定 VME 电平
和状态 ID 生成 VME 中断的函数。


#### Attaching Interrupt Handlers


函数 `vme_irq_request` 可用于挂接，`vme_irq_free` 用于释放一个特定的 VME 电平与
状态 ID 组合。任何一个给定的组合只能分配一个回调函数。提供了一个 void 指针参数，其
值会传给回调函数，该指针的用途由用户自定义。回调函数参数如下。编写回调函数时必须
小心，回调函数运行在中断上下文中：


	void callback(int level, int statid, void *priv);


#### Interrupt Generation


函数 `vme_irq_generate` 可用于以给定的 VME 电平和 VME 状态 ID 生成 VME 中断。


### Location monitors


VME API 提供以下功能来配置 location monitor（位置监视器）。


#### Location Monitor Management


函数 `vme_lm_request` 用于请求使用一块位置监视器，`vme_lm_free` 在不再需要时释放它们。
每块可提供若干个位置监视器，监视相邻位置。函数 `vme_lm_count` 可用于确定提供了多少
个位置。


#### Location Monitor Configuration


一块位置监视器分配后，函数 `vme_lm_set` 用于配置位置监视器的位置和模式。函数
`vme_lm_get` 可用于获取已有设置。


#### Location Monitor Use


函数 `vme_lm_attach` 用于挂接回调，`vme_lm_detach` 用于从每个位置监视器位置分离。每个
位置监视器可以监视若干个相邻位置。回调函数声明如下。


	void callback(void *data);


### Slot Detection


函数 `vme_slot_num` 返回所提供桥的插槽 ID。


### Bus Detection


函数 `vme_bus_num` 返回所提供桥的总线 ID。


### VME API


   :internal:

   :export:
