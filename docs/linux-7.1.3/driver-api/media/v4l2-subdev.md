
### V4L2 子设备（sub-devices


许多驱动需要与其子设备（sub-devices）通信。这些设备可以完成各
任务，但最常见的是处理音频或视频的复用（muxing）
编码或解码。对于网络摄像头，常见的子设备是传感器和摄像
控制器

通常这些I2C 设备，但不一定。为了给驱动提供一个一致的接口来访问这些子设备
创建
`v4l2_subdev` 结构体（v4l2-subdev.h）

每个子设备驱动都必须有一`v4l2_subdev` 结构体。对于简单的子设备，该结构体
可以独立存在，或者，如果需要存储更多状态信息，它可能被嵌入到一个更大的
结构体中。通常会有一
底层设备结构体（例如 `i2c_client`），其中包含由内核设置的设备数据。建
使用 `v4l2_set_subdevdata` 将该指针存储`v4l2_subdev` 的私有数据中。这
可以方便地从 `v4l2_subdev` 找到实际的底层总线相关
设备数据


你还需要一种从底层结构体到 `v4l2_subdev` 的方法
对于常见i2c_client 结构体，使用 i2c_set_clientdata() 调用来存
一`v4l2_subdev` 指针；对于其他总线，你可能必须使用其他方法


桥（bridge）驱动可能还需要存储每个子设备的私有数据，例如指向
桥相关的每子设备私有数据的指针。`v4l2_subdev` 结构体为此提供了
主机私有数据，可通过
`v4l2_get_subdev_hostdata` `v4l2_set_subdev_hostdata` 访问

从桥驱动的视角来看，你加载子设备模块并以某种方式
获取 `v4l2_subdev` 指针。对i2c 设备这很容易：你调用
`i2c_get_clientdata()`。对于其他总线则需要做类似的操作
对于 I2C 总线上的子设备，存在辅助函数为你完成大部分此
棘手的工作

每个 `v4l2_subdev` 都包含子设备驱动可以实现
函数指针（如果不适用可以保留`NULL`）。由于子设备可以做很多不同的事情
而你又不希望最终得到一个庞大的 ops 结构体、其中只有少ops 被普遍实现，因此
函数指针按类别分类，每个类别有自己独立的 ops 结构体


顶层ops 结构体包含指向各类别 ops 结构体的指针，如果子设备驱动
不支持该类别中的任何功能，则可以NULL

其结构如下所示：


	struct v4l2_subdev_core_ops {
		int (**log_status)(struct v4l2_subdev **sd);
		int (**init)(struct v4l2_subdev **sd, u32 val);
		...
	};

	struct v4l2_subdev_tuner_ops {
		...
	};

	struct v4l2_subdev_audio_ops {
		...
	};

	struct v4l2_subdev_video_ops {
		...
	};

	struct v4l2_subdev_pad_ops {
		...
	};

	struct v4l2_subdev_ops {
		const struct v4l2_subdev_core_ops  *core;
		const struct v4l2_subdev_tuner_ops *tuner;
		const struct v4l2_subdev_audio_ops *audio;
		const struct v4l2_subdev_video_ops *video;
		const struct v4l2_subdev_pad_ops *video;
	};

核心（core）ops 是所有子设备共有的，其他类别则根据子设备的不同分别实现。例如，
视频设备不太可能支持 audio ops，反之亦然


这种组织方式在限制函数指针数量的同时，仍然便
添加新的 ops 和类别

子设备驱动使用以下方式初始化 `v4l2_subdev` 结构体：

	`v4l2_subdev_init <v4l2_subdev_init>`
	(`sd <v4l2_subdev>`, &\ `ops <v4l2_subdev_ops>`).


之后，你需要用唯一的名称初始化 `sd <v4l2_subdev>`->name 并设置模块所有者
如果你使i2c 辅助函数，这些会为你自动完成


如果需要与 media 框架集成，你必须通过调用 `media_entity_pads_init` 来初始化
嵌入`v4l2_subdev` 结构体中`media_entity` 结构体（entity 字段），
前提是该实体具有
pads锛。


	struct media_pad *pads = &my_sd->pads;
	int err;

	err = media_entity_pads_init(&sd->entity, npads, pads);

pads 数组必须事先初始化。无需手动设置 struct media_entity function name 字段
但如有需要，必须初始revision 字段


当子设备节点（如果有）被打开/关闭时，对该实体的引用会被自
获取/释放

在子设备被销毁之前，不要忘记清理 media 实体


	media_entity_cleanup(&sd->entity);

如果子设备驱动实现了 sink pads，子设备驱动可以设置 `v4l2_subdev_pad_ops` 中的
link_validate 字段，以提供其自己的链路
验证函数。对于管线中的每条链路，都会调用该链sink 端的 link_validate pad
操作。在这两种情况下，驱动仍然负责验证子设备与视频节点之
格式配置的正确性


如果未设link_validate op，则改用默认函数
`v4l2_subdev_link_validate_default`。该函数确保链路的源端和
sink 端的宽度、高度和 media 总线像素码相等。子设备驱动也可以自由使用此函数
在它们自己的检查之外再执行上述检查


#### 子设备注册（Subdev registration


目前有两种方式向 V4L2 核心注册子设备。第一种（传统）方式是让桥驱动
注册子设备。当桥驱动拥有与其连接的子设备的完整信息，并且确切知
何时注册它们时，就可以这样做。对于内部子设备（如 SoC 内部的视频数据处理单
或复杂的 PCI(e) 板卡、USB 摄像头中的摄像头传感器或连接SoC 的传感器）来说，
通常就是这种情况，它们通常在其平台数据中将这些信息传递给桥驱动




然而，也存在子设备必须相对于桥设备异步注册的情况。这种配置的一个例子是
基于设备树（Device Tree）的系统，其中关于子设备的信息独立于桥设备提供给系统
例如当子设备DT 中定义为 I2C 设备节点时。第二种情况使用API 将在下文进一步描述




使用哪种注册方法只影响探测（probing）过程，运行时的子设
交互在两种情况下都是相同的

##### 注册同步子设


*同步（synchronous*情况下，设备（桥）驱动需要使v4l2_device 注册
`v4l2_subdev`锛。

	`v4l2_device_register_subdev <v4l2_device_register_subdev>`
	(`v4l2_dev <v4l2_device>`, `sd <v4l2_subdev>`).

如果在注册之前子设备模块已消失，则可能会失败
该函数成功调用后，subdev->dev 字段指向
`v4l2_device`銆。

如果 v4l2_device 父设备的 mdev 字段非空，则该子设备
实体将自动向 media 设备注册

你可以使用以下方式注销子设备：

	`v4l2_device_unregister_subdev <v4l2_device_unregister_subdev>`
	(`sd <v4l2_subdev>`).

之后，子设备模块可以被卸载，并且
`sd <v4l2_subdev>`->dev == `NULL`銆。


##### 注册异步子设


*异步（asynchronous*情况下，子设备的探测可以独立
桥驱动的可用性而被调用。子设备驱动随后必须验证成功探测所需
所有条件是否都满足。这可能包括对主时钟可用性的检查。如果任何条件不满足
驱动可能会决定返`-EPROBE_DEFER` 以请求进一步的重探测尝试。一旦所有条件都满足
子设备应使用 `v4l2_async_register_subdev` 函数注册。注销则使
`v4l2_async_unregister_subdev` 调用。以这种方式注册的子设备存储在全局子设备列表中
随时准备被桥驱动拾取




驱动必须在使`v4l2_async_register_subdev` 注册子设备之前完成其所有初始化
包括启用运行PM（runtime PM）。这是因为子设备一旦注册就立即可被访问



##### 异步子设备通知器（notifiers


桥驱动反过来必须注册一notifier 对象。这是通过
使用 `v4l2_async_nf_register` 调用完成。要注销 notifier，则使用
`v4l2_async_nf_unregister`。在释放已注销 notifier 的内存之前，必须调用
`v4l2_async_nf_cleanup` 对其进行清理


在注notifier 之前，桥驱动必须做两件事：首先，必须使用
`v4l2_async_nf_init` 初始notifier。其次，桥驱动可以开始形
桥设备运行所需的异步连接描述符列表


`v4l2_async_nf_add_fwnode_remote` 涓?`v4l2_async_nf_add_i2c`

异步连接描述符描述与尚未被探测的外部子设备的连接。基于一个异步连接，
当相关子设备可用时，可能会创建一media 数据链路或辅助链路。对于一个给定的子设备，
可能有一个或多个异步连接，但在将这些连接添加notifier 时还不知道。异步连接会随着
匹配到的异步子设备被逐个绑定



##### 用于子设备的异步子设备通知


注册异步子设备的驱动也可以注册一个异notifier。这称为异步子设notifier
其过程与桥驱动类似，不同之处在于 notifier 是使
 `v4l2_async_subdev_nf_init` 初始化的（而非桥驱动那样）。子设备
可用之后才能完成，即存在一条经由异步子设备notifier 到达某个非异步子设备 notifier 的路径



##### 用于摄像头传感器驱动的异步子设备注册辅助函数


`v4l2_async_register_subdev_sensor` 是一个用于传感器驱动的辅助函数，它注
自己的异步连接，同时还会注册一notifier，并进一步为在固件中找到的镜头和闪光灯设
注册异步连接。该子设备的 notifier 会使`v4l2_async_unregister_subdev`
随该异步子设备一起被注销和清理


##### 异步子设备通知器示


这些函数分配一个异步连接描述符，其类型struct
`v4l2_async_connection`，嵌入在一个驱动特定的结构体中struct
`v4l2_async_connection` 应为该结构体的第一个成员：


	struct my_async_connection {
		struct v4l2_async_connection asc;
		...
	};

	struct my_async_connection *my_asc;
	struct fwnode_handle *ep;

	...

	my_asc = v4l2_async_nf_add_fwnode_remote(&notifier, ep,
						 struct my_async_connection);
	fwnode_handle_put(ep);

	if (IS_ERR(my_asc))
		return PTR_ERR(my_asc);

##### Asynchronous sub-device notifier callbacks


然后 V4L2 核心将使用这些连接描述符，将异步注册的子设备与之匹配。如果检测到匹配
则调`.bound()` notifier 回调。在所有连接都绑定后，调用 .complete() 回调
当某个连接从系统中移除时，调`.unbind()` 方法。这三个回调都是可选的



驱动可以在其驱动特定
`v4l2_async_connection` 包装器中存储任何类型的自定义数据。如果该数据在结构体
被释放时需要特殊处理，驱动必须实现 `.destroy()` notifier 回调。框架将在释
`v4l2_async_connection` 之前立即调用它


#### 调用子设备操


使用 `v4l2_subdev` 的优点在于它是一个通用结构体，
不包含任何关于底层硬件的信息。因此，一个驱动可能包含多个使I2C 总线的子设备
也可能包含一个通过 GPIO 引脚控制的子设备。这种区别只在设置设备时相关
一旦子设备注册完成就完全透明了


一旦子设备注册完成，你可以直接调用一ops 函数



	err = sd->ops->core->g_std(sd, &norm);

但使用这个宏更好也更简单：


	err = v4l2_subdev_call(sd, core, g_std, &norm);

该宏会执行正确的 `NULL` 指针检查，并在 `sd <v4l2_subdev>` `NULL` 时返`-ENODEV`
`sd <v4l2_subdev>`->core `sd <v4l2_subdev>`->core->g_std `NULL` 时返`-ENOIOCTLCMD`
否则返回 `sd <v4l2_subdev>`->ops->core->g_std ops 的实际结果


也可以调用所有或一部分子设备：


	v4l2_device_call_all(v4l2_dev, 0, core, g_std, &norm);

任何不支持该 ops 的子设备都会被跳过，错误结果被忽略。如果要检查错误，请使用：



	err = v4l2_device_call_until_err(v4l2_dev, 0, core, g_std, &norm);

`-ENOIOCTLCMD` 之外的任何错误都会以该错误退出循环。如果没
发生错误（除 `-ENOIOCTLCMD` 外），则返回 0

这两个调用的第二个参数是一个组 ID（group ID）。如果为 0，则调用所有子设备
如果非零，则只调用组 ID 与该值匹配的子设备。在桥驱动注册子设备之前，它可以
`sd <v4l2_subdev>`->grp_id 设置为它想要的任何值（默认0）。该值由桥驱动拥有，
子设备驱动永远不会修改或使用它



ID 让桥驱动能更好地控制回调的调用方式。例如，板上可能有多个音频芯片，
每个都能改变音量。但通常当用户想要改变音量时，只有其中一个会真正被使用。你可以将该子设备的
ID 设置为例AUDIO_CONTROLLER，并在调`v4l2_device_call_all()` 时将其指定为ID 值
这确保它只会发送到需要它的那个子设备




如果子设备需要向v4l2_device 父设备通知某个事件，它可以调用
`v4l2_subdev_notify(sd, notification, arg)`。该宏检查是否定义了 `notify()` 回调
如果没有则返`-ENODEV`。否则返`notify()` 调用的结果


### V4L2 子设备用户空API


桥驱动传统上向用户空间暴露一个或多个视频节点，并通过对视频节点操作的
响应，通过 `v4l2_subdev_ops` 操作来控制子设备。这向应用程序隐藏了
底层硬件的复杂性。对于复杂设备，可能需要比视频节点提供的更细粒度的设备控制
在这种情况下，实现了 media controller API <media_controller> 的桥驱动可以选择
让子设备操作可直接从用户空间访问



Device nodes named `v4l-subdev`\ **X** can be created in `/dev` to access
子设备。如果子设备支持直接的用户空间配置，它必须在注册之前设置
`V4L2_SUBDEV_FL_HAS_DEVNODE` 标志

注册子设备后，`v4l2_device` 驱动可以通过调用
`v4l2_device_register_subdev_nodes` 为所有标记了
`V4L2_SUBDEV_FL_HAS_DEVNODE` 的已注册子设备创建设备节点。当子设备被注销时，
这些设备节点会被自动移除


该设备节点处V4L2 API 的一个子集

`VIDIOC_QUERYCTRL`,
`VIDIOC_QUERYMENU`,
`VIDIOC_G_CTRL`,
`VIDIOC_S_CTRL`,
`VIDIOC_G_EXT_CTRLS`,
`VIDIOC_S_EXT_CTRLS` and
`VIDIOC_TRY_EXT_CTRLS`:

	这些 controls ioctl V4L2 中定义的完全相同。它们的行为也相同，唯一的例外是
	它们只处理由子设备生成的事件。取决于驱动，这些事件也可以由一个（或多个）
	V4L2 设备节点访问
	
	

`VIDIOC_DQEVENT`,
`VIDIOC_SUBSCRIBE_EVENT` and
`VIDIOC_UNSUBSCRIBE_EVENT`

	这些 events ioctl V4L2 中定义的完全相同。它们的行为也相同，唯一的例外是
	它们只处理由子设备生成的事件。取决于驱动，这些事件也可以由一个（或多个）
	V4L2 设备节点报告
	

	希望使用事件的子设备驱动需要在注册子设备之前设
	`V4L2_SUBDEV_FL_HAS_EVENTS` `v4l2_subdev`.flags。注册后，事件可以像往常一
	`v4l2_subdev`.devnode 设备节点上排队
	

	为了正确支持事件，`poll()` 文件操作也被实现
	

私有 ioctl

	上述列表之外的所ioctl 都通过 core::ioctl 操作直接传递给子设
	驱动

### 只读子设备用户空API


通过 `v4l2_subdev_ops` 结构体实现的 kernel API 直接调用控制其连接子设备的桥驱动
通常不希望用户空间能够通过子设备设备节点更改相同的参数，因此通常不会注册任何此类节点



有时通过只读 API 向用户空间报告当前子设备的配置是很有用的，该 API 不允
应用程序更改设备参数，但允许与子设备设备节点交互以检查它们



例如，为了实现基于计算摄影的摄像头，用户空间需要了解每个受支持输出分辨率下
详细的摄像头传感器配置（包括跳过、合并（binning）、裁剪和缩放）。为了支持此类用例，
桥驱动可以通过只读 API 将子设备操作暴露给用户空间



要为所有使`V4L2_SUBDEV_FL_HAS_DEVNODE` 注册的子设备创建只读设备节点
`v4l2_device` 驱动应调`v4l2_device_register_ro_subdev_nodes`


对于使用 `v4l2_device_register_ro_subdev_nodes` 注册的子设备设备节点
用户空间应用程序对以ioctl 的访问受到限制


`VIDIOC_SUBDEV_S_FMT`,
`VIDIOC_SUBDEV_S_CROP`,
`VIDIOC_SUBDEV_S_SELECTION`:

	这些 ioctl 仅在只读子设备设备节点上被允许用
	V4L2_SUBDEV_FORMAT_TRY <v4l2-subdev-format-whence> 的格式和选择矩形
	

`VIDIOC_SUBDEV_S_FRAME_INTERVAL`,
`VIDIOC_SUBDEV_S_DV_TIMINGS`,
`VIDIOC_SUBDEV_S_STD`:

	这些 ioctl 在只读子设备节点上不被允许

如果 ioctl 不被允许，或者要修改的格式被设置
`V4L2_SUBDEV_FORMAT_ACTIVE`，核心返回一个负的错误码，并errno 变量被设置为 `-EPERM`


### I2C 子设备驱


由于这些驱动非常常见，提供了专门的辅助函数以简化其使用（`v4l2-common.h`）


I2C 驱动添加 `v4l2_subdev` 支持的推荐方法是`v4l2_subdev` 结构
嵌入到为每个 I2C 设备实例创建的状态结构体中。非常简单的设备没有状态结构体
在这种情况下你可以直接创建一`v4l2_subdev`


一个典型的状态结构体如下所示（其中 'chipname' 替换为芯片的名称）：



	struct chipname_state {
		struct v4l2_subdev sd;
		...  /** additional state fields **/
	};

按如下方式初始化 `v4l2_subdev` 结构体：


	v4l2_i2c_subdev_init(&state->sd, client, subdev_ops);

该函数将填充 `v4l2_subdev` 的所有字段，确保
`v4l2_subdev` i2c_client 彼此指向对方

你还应该添加一个辅助内联函数，用于`v4l2_subdev`
指针转换chipname_state 结构体：


	static inline struct chipname_state **to_state(struct v4l2_subdev **sd)
	{
		return container_of(sd, struct chipname_state, sd);
	}

用它来从 `v4l2_subdev` 结构体转换到 `i2c_client`
结构体：


	struct i2c_client *client = v4l2_get_subdevdata(sd);

以下代码用于从一`i2c_client` 转到 `v4l2_subdev` 结构体：


	struct v4l2_subdev *sd = i2c_get_clientdata(client);

确保在调`remove()` 回调时调

即使该子设备从未被注册，调用它也是安全的



你需要这样做，因为当桥驱动销i2c 适配器时，会调用该适配器上 i2c 设备
`remove()` 回调。此后相应的 v4l2_subdev 结构体失效，因此必须先将它们注销。在
`remove()` 回调中调`v4l2_device_unregister_subdev`\ (`sd <v4l2_subdev>`)
可确保这一点始终被正确完成




桥驱动也有一些可以使用的辅助函数


	struct v4l2_subdev *sd = v4l2_i2c_new_subdev(v4l2_dev, adapter,
					"module_foo", "chipid", 0x36, NULL);

这会加载给定的模块（如果不需要加载模块则`NULL`），并使用给定的 `i2c_adapter` 
芯片/地址参数调用 `i2c_new_client_device`。如果一切顺利，则将该子设备注册
v4l2_device銆。


你也可以使用 `v4l2_i2c_new_subdev` 的最后一个参数来传递一个它应该探测
可能I2C 地址数组。这些探测地址仅在前一个参数为 0 时使用。非零参数意味着你知
确切i2c 地址，因此在这种情况下不会进行探测


如果出现问题，这两个函数都返`NULL`

请注意，你传递给 `v4l2_i2c_new_subdev` chipid 通常与模块名称相同。它允许
指定一个芯片变体，例如 "saa7114" "saa7115"。不过一般来说，i2c 驱动会自动检测这一点
chipid 的使用是需要在日后更仔细研究的事情。它在不同的 i2c 驱动之间有所差异，因此可能令人困惑
要查看支持哪些芯片变体，可以查看 i2c 驱动代码中的 i2c_device_id 表。它列出了所有可能性




还有一个辅助函数：

`v4l2_i2c_new_subdev_board` 使用一`i2c_board_info` 结构体，
该结构体被传递给 i2c 驱动，并替代 irq、platform_data addr 参数


如果子设备支s_config core ops，则在子设备设置完成后，会以 irq platform_data 参数调用op


`v4l2_i2c_new_subdev` 函数会在内部调用
`v4l2_i2c_new_subdev_board`，使`client_type` 
`addr` 填充一`i2c_board_info` 结构体


### 集中管理的子设备活动状


传统上，V4L2 子设备驱动为活动设备配置维护内部状态。这通常实现为例如一
v4l2_mbus_framefmt 结构体数组，每个 pad 一个条目，裁剪（crop）和合成（compose）矩形也类似



除了活动配置外，每个子设备文件句柄都有一个由 V4L2 核心管理struct
v4l2_subdev_state，其中包try


为了简化子设备驱动，V4L2 子设API 现在可选地支持
`v4l2_subdev_state` 表示的集中管理的活动配置。一个包含活动设备配置的
状态实例，作为 `v4l2_subdev` 结构体的一部分存储在子设备自身中；而核心将
一try 状态关联到每个打开的文件句柄，以存储与该文件句柄相关的 try 配置




子设备驱动可以选择使用 state 来管理其活动配置，方法是在注册子设备之前调用
v4l2_subdev_init_finalize() 来初始化子设备状态。它们还必须在注销子设备之前调
v4l2_subdev_cleanup() 来释放所有已分配的资源。核心会自动为每个打开的文件句
分配并初始化一个状态以存储 try 配置，并在关闭文件句柄时释放它




同时使用 :ref:`ACTIVE TRY 格式 <v4l2-subdev-format-whence>` V4L2 子设备操作，
通过 'state' 参数接收要操作的正确状态。调用者必须通过调用
`v4l2_subdev_lock_state()` `v4l2_subdev_unlock_state()` 来锁定和解锁该状态
调用者可以通过 `v4l2_subdev_call_state_active()` 宏来调用子设备操作



不接state 参数的操作隐式地对子设备活动状态进行操作，驱动可以通过调用
`v4l2_subdev_lock_and_get_active_state()` 独占访问该状态。子设备的活动状
同样必须通过调用 `v4l2_subdev_unlock_state()` 来释放


驱动绝不能直接手动访问存储在 `v4l2_subdev` 或文件句柄中的状态，而不经过指定的辅助函数


虽然 V4L2 核心会将正确try 或活动状态传递给子设备操作，但许多现有的设备驱动在调
`v4l2_subdev_call()` 操作时会传递一NULL 状态。这种遗留写法会给让 V4L2 核心管理活动状态的
子设备驱动带来问题，因为它们期望接收适当的状态作为参敎数。为了帮助子设备驱动转换为受管理的活动状态，
而无需同时转换所有调用者，v4l2_subdev_call() 中添加了一个额外的封装层，它通过获取并锁
被调用者的活动状态（使用 `v4l2_subdev_lock_and_get_active_state()`）来处理 NULL 情况
并在调用后解锁该状态





整个子设备状态实际上分为三个部分：v4l2_subdev_state、子设备控制项（controls）和子设备驱动的
内部状态。将来这些部分应合并为单一状态。目前我们需要一种方法来处理这些部分的锁定。这可以通过
共享一个锁来实现。v4l2_ctrl_handler 已经通过'lock' 指针支持这一点，状态也使用相同的模型。驱动可以在
调用 v4l2_subdev_init_finalize() 之前执行以下操作





	sd->ctrl_handler->lock = &priv->mutex;
	sd->state_lock = &priv->mutex;

这在控制项和状态之间共享驱动的私有互斥锁

### 流、多路复media pads 与内部路


子设备驱动可以通过设置 V4L2_SUBDEV_FL_STREAMS 子设备标志，并实现集中管理的
子设备活动状态、路由以及基于流的配置，从而实现对多路复用流的支持



### V4L2 子设备函数与数据结构


