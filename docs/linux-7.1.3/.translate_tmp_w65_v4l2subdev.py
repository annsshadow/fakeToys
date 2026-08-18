import os

F = r"D:/WORKSPACE/linux-7.1.3/docs/系统文档/driver-api/media/v4l2-subdev.md"

T = {
"### V4L2 sub-devices":
"### V4L2 子设备（sub-devices）",
"Many drivers need to communicate with sub-devices. These devices can do all":
"许多驱动需要与其子设备（sub-devices）通信。这些设备可以完成各种",
"sort of tasks, but most commonly they handle audio and/or video muxing,":
"任务，但最常见的是处理音频和/或视频的复用（muxing）、",
"encoding or decoding. For webcams common sub-devices are sensors and camera":
"编码或解码。对于网络摄像头，常见的子设备是传感器和摄像头",
"controllers.":
"控制器。",
"Usually these are I2C devices, but not necessarily. In order to provide the":
"通常这些是 I2C 设备，但不一定。为了给驱动提供一个一致的接口来访问这些子设备，",
"driver with a consistent interface to these sub-devices the":
"创建了",
"`v4l2_subdev` struct (v4l2-subdev.h) was created.":
"`v4l2_subdev` 结构体（v4l2-subdev.h）。",
"Each sub-device driver must have a `v4l2_subdev` struct. This struct":
"每个子设备驱动都必须有一个 `v4l2_subdev` 结构体。对于简单的子设备，该结构体",
"can be stand-alone for simple sub-devices or it might be embedded in a larger":
"可以独立存在，或者，如果需要存储更多状态信息，它可能被嵌入到一个更大的",
"struct if more state information needs to be stored. Usually there is a":
"结构体中。通常会有一个",
"low-level device struct (e.g. `i2c_client`) that contains the device data as":
"底层设备结构体（例如 `i2c_client`），其中包含由内核设置的设备数据。建议",
"setup by the kernel. It is recommended to store that pointer in the private":
"使用 `v4l2_set_subdevdata` 将该指针存储在 `v4l2_subdev` 的私有数据中。这样",
"data of `v4l2_subdev` using `v4l2_set_subdevdata`. That makes":
"可以方便地从 `v4l2_subdev` 找到实际的底层总线相关",
"it easy to go from a `v4l2_subdev` to the actual low-level bus-specific":
"设备数据。",
"device data.":
"",
"You also need a way to go from the low-level struct to `v4l2_subdev`.":
"你还需要一种从底层结构体到 `v4l2_subdev` 的方法。",
"For the common i2c_client struct the i2c_set_clientdata() call is used to store":
"对于常见的 i2c_client 结构体，使用 i2c_set_clientdata() 调用来存储",
"a `v4l2_subdev` pointer, for other buses you may have to use other":
"一个 `v4l2_subdev` 指针；对于其他总线，你可能必须使用其他方法。",
"methods.":
"",
"Bridges might also need to store per-subdev private data, such as a pointer to":
"桥（bridge）驱动可能还需要存储每个子设备的私有数据，例如指向",
"bridge-specific per-subdev private data. The `v4l2_subdev` structure":
"桥相关的每子设备私有数据的指针。`v4l2_subdev` 结构体为此提供了",
"provides host private data for that purpose that can be accessed with":
"主机私有数据，可通过",
"`v4l2_get_subdev_hostdata` and `v4l2_set_subdev_hostdata`.":
"`v4l2_get_subdev_hostdata` 和 `v4l2_set_subdev_hostdata` 访问。",
"From the bridge driver perspective, you load the sub-device module and somehow":
"从桥驱动的视角来看，你加载子设备模块并以某种方式",
"obtain the `v4l2_subdev` pointer. For i2c devices this is easy: you call":
"获取 `v4l2_subdev` 指针。对于 i2c 设备这很容易：你调用",
"`i2c_get_clientdata()`. For other buses something similar needs to be done.":
"`i2c_get_clientdata()`。对于其他总线则需要做类似的操作。",
"Helper functions exist for sub-devices on an I2C bus that do most of this":
"对于 I2C 总线上的子设备，存在辅助函数为你完成大部分此类",
"tricky work for you.":
"棘手的工作。",
"Each `v4l2_subdev` contains function pointers that sub-device drivers":
"每个 `v4l2_subdev` 都包含子设备驱动可以实现的",
"can implement (or leave `NULL` if it is not applicable). Since sub-devices can":
"函数指针（如果不适用可以保留为 `NULL`）。由于子设备可以做很多不同的事情，",
"do so many different things and you do not want to end up with a huge ops struct":
"而你又不希望最终得到一个庞大的 ops 结构体、其中只有少数 ops 被普遍实现，因此",
"of which only a handful of ops are commonly implemented, the function pointers":
"函数指针按类别分类，每个类别有自己独立的 ops 结构体。",
"are sorted according to category and each category has its own ops struct.":
"",
"The top-level ops struct contains pointers to the category ops structs, which":
"顶层的 ops 结构体包含指向各类别 ops 结构体的指针，如果子设备驱动",
"may be NULL if the subdev driver does not support anything from that category.":
"不支持该类别中的任何功能，则可以为 NULL。",
"It looks like this:":
"其结构如下所示：",
"The core ops are common to all subdevs, the other categories are implemented":
"核心（core）ops 是所有子设备共有的，其他类别则根据子设备的不同分别实现。例如，",
"depending on the sub-device. E.g. a video device is unlikely to support the":
"视频设备不太可能支持 audio ops，反之亦然。",
"audio ops and vice versa.":
"",
"This setup limits the number of function pointers while still making it easy":
"这种组织方式在限制函数指针数量的同时，仍然便于",
"to add new ops and categories.":
"添加新的 ops 和类别。",
"A sub-device driver initializes the `v4l2_subdev` struct using:":
"子设备驱动使用以下方式初始化 `v4l2_subdev` 结构体：",
"Afterwards you need to initialize `sd <v4l2_subdev>`->name with a":
"之后，你需要用唯一的名称初始化 `sd <v4l2_subdev>`->name 并设置模块所有者。",
"unique name and set the module owner. This is done for you if you use the":
"如果你使用 i2c 辅助函数，这些会为你自动完成。",
"i2c helper functions.":
"",
"If integration with the media framework is needed, you must initialize the":
"如果需要与 media 框架集成，你必须通过调用 `media_entity_pads_init` 来初始化",
"`media_entity` struct embedded in the `v4l2_subdev` struct":
"嵌入在 `v4l2_subdev` 结构体中的 `media_entity` 结构体（entity 字段），",
"(entity field) by calling `media_entity_pads_init`, if the entity has":
"前提是该实体具有",
"pads:":
"pads：",
"The pads array must have been previously initialized. There is no need to":
"pads 数组必须事先初始化。无需手动设置 struct media_entity 的 function 和 name 字段，",
"manually set the struct media_entity function and name fields, but the":
"但如有需要，必须初始化 revision 字段。",
"revision field must be initialized if needed.":
"",
"A reference to the entity will be automatically acquired/released when the":
"当子设备节点（如果有）被打开/关闭时，对该实体的引用会被自动",
"subdev device node (if any) is opened/closed.":
"获取/释放。",
"Don't forget to cleanup the media entity before the sub-device is destroyed:":
"在子设备被销毁之前，不要忘记清理 media 实体：",
"If a sub-device driver implements sink pads, the subdev driver may set the":
"如果子设备驱动实现了 sink pads，子设备驱动可以设置 `v4l2_subdev_pad_ops` 中的",
"link_validate field in `v4l2_subdev_pad_ops` to provide its own link":
"link_validate 字段，以提供其自己的链路",
"validation function. For every link in the pipeline, the link_validate pad":
"验证函数。对于管线中的每条链路，都会调用该链路 sink 端的 link_validate pad",
"operation of the sink end of the link is called. In both cases the driver is":
"操作。在这两种情况下，驱动仍然负责验证子设备与视频节点之间",
"still responsible for validating the correctness of the format configuration":
"格式配置的正确性。",
"between sub-devices and video nodes.":
"",
"If link_validate op is not set, the default function":
"如果未设置 link_validate op，则改用默认函数",
"`v4l2_subdev_link_validate_default` is used instead. This function":
"`v4l2_subdev_link_validate_default`。该函数确保链路的源端和",
"ensures that width, height and the media bus pixel code are equal on both source":
"sink 端的宽度、高度和 media 总线像素码相等。子设备驱动也可以自由使用此函数，",
"and sink of the link. Subdev drivers are also free to use this function to":
"在它们自己的检查之外再执行上述检查。",
"perform the checks mentioned above in addition to their own checks.":
"",
"#### Subdev registration":
"#### 子设备注册（Subdev registration）",
"There are currently two ways to register subdevices with the V4L2 core. The":
"目前有两种方式向 V4L2 核心注册子设备。第一种（传统）方式是让桥驱动",
"first (traditional) possibility is to have subdevices registered by bridge":
"注册子设备。当桥驱动拥有与其连接的子设备的完整信息，并且确切知道",
"drivers. This can be done when the bridge driver has the complete information":
"何时注册它们时，就可以这样做。对于内部子设备（如 SoC 内部的视频数据处理单元",
"about subdevices connected to it and knows exactly when to register them. This":
"或复杂的 PCI(e) 板卡、USB 摄像头中的摄像头传感器或连接到 SoC 的传感器）来说，",
"is typically the case for internal subdevices, like video data processing units":
"通常就是这种情况，它们通常在其平台数据中将这些信息传递给桥驱动。",
"within SoCs or complex PCI(e) boards, camera sensors in USB cameras or connected":
"",
"to SoCs, which pass information about them to bridge drivers, usually in their":
"",
"platform data.":
"",
"There are however also situations where subdevices have to be registered":
"然而，也存在子设备必须相对于桥设备异步注册的情况。这种配置的一个例子是",
"asynchronously to bridge devices. An example of such a configuration is a Device":
"基于设备树（Device Tree）的系统，其中关于子设备的信息独立于桥设备提供给系统，",
"Tree based system where information about subdevices is made available to the":
"例如当子设备在 DT 中定义为 I2C 设备节点时。第二种情况使用的 API 将在下文进一步描述。",
"system independently from the bridge devices, e.g. when subdevices are defined":
"",
"in DT as I2C device nodes. The API used in this second case is described further":
"",
"below.":
"",
"Using one or the other registration method only affects the probing process, the":
"使用哪种注册方法只影响探测（probing）过程，运行时的桥-子设备",
"run-time bridge-subdevice interaction is in both cases the same.":
"交互在两种情况下都是相同的。",
"##### Registering synchronous sub-devices":
"##### 注册同步子设备",
"In the **synchronous** case a device (bridge) driver needs to register the":
"在**同步（synchronous）**情况下，设备（桥）驱动需要使用 v4l2_device 注册",
"`v4l2_subdev` with the v4l2_device:":
"`v4l2_subdev`：",
"This can fail if the subdev module disappeared before it could be registered.":
"如果在注册之前子设备模块已消失，则可能会失败。",
"After this function was called successfully the subdev->dev field points to":
"该函数成功调用后，subdev->dev 字段指向",
"the `v4l2_device`.":
"`v4l2_device`。",
"If the v4l2_device parent device has a non-NULL mdev field, the sub-device":
"如果 v4l2_device 父设备的 mdev 字段非空，则该子设备",
"entity will be automatically registered with the media device.":
"实体将自动向 media 设备注册。",
"You can unregister a sub-device using:":
"你可以使用以下方式注销子设备：",
"Afterwards the subdev module can be unloaded and":
"之后，子设备模块可以被卸载，并且",
"`sd <v4l2_subdev>`->dev == `NULL`.":
"`sd <v4l2_subdev>`->dev == `NULL`。",
"##### Registering asynchronous sub-devices":
"##### 注册异步子设备",
"In the **asynchronous** case subdevice probing can be invoked independently of":
"在**异步（asynchronous）**情况下，子设备的探测可以独立于",
"the bridge driver availability. The subdevice driver then has to verify whether":
"桥驱动的可用性而被调用。子设备驱动随后必须验证成功探测所需的",
"all the requirements for a successful probing are satisfied. This can include a":
"所有条件是否都满足。这可能包括对主时钟可用性的检查。如果任何条件不满足，",
"check for a master clock availability. If any of the conditions aren't satisfied":
"驱动可能会决定返回 `-EPROBE_DEFER` 以请求进一步的重探测尝试。一旦所有条件都满足，",
"the driver might decide to return `-EPROBE_DEFER` to request further reprobing":
"子设备应使用 `v4l2_async_register_subdev` 函数注册。注销则使用",
"attempts. Once all conditions are met the subdevice shall be registered using":
"`v4l2_async_unregister_subdev` 调用。以这种方式注册的子设备存储在全局子设备列表中，",
"the `v4l2_async_register_subdev` function. Unregistration is":
"随时准备被桥驱动拾取。",
"performed using the `v4l2_async_unregister_subdev` call. Subdevices":
"",
"registered this way are stored in a global list of subdevices, ready to be":
"",
"picked up by bridge drivers.":
"",
"Drivers must complete all initialization of the sub-device before":
"驱动必须在使用 `v4l2_async_register_subdev` 注册子设备之前完成其所有初始化，",
"registering it using `v4l2_async_register_subdev`, including":
"包括启用运行时 PM（runtime PM）。这是因为子设备一旦注册就立即可被访问。",
"enabling runtime PM. This is because the sub-device becomes accessible":
"",
"as soon as it gets registered.":
"",
"##### Asynchronous sub-device notifiers":
"##### 异步子设备通知器（notifiers）",
"Bridge drivers in turn have to register a notifier object. This is performed":
"桥驱动反过来必须注册一个 notifier 对象。这是通过",
"`v4l2_async_nf_register` call. To unregister the notifier the":
"`v4l2_async_nf_register` 调用完成的。要注销 notifier，驱动必须调用",
"driver has to call `v4l2_async_nf_unregister`. Before releasing memory":
"`v4l2_async_nf_unregister`。在释放已注销 notifier 的内存之前，必须调用",
"of an unregister notifier, it must be cleaned up by calling":
"`v4l2_async_nf_cleanup` 对其进行清理。",
"`v4l2_async_nf_cleanup`.":
"",
"Before registering the notifier, bridge drivers must do two things: first, the":
"在注册 notifier 之前，桥驱动必须做两件事：首先，必须使用",
"notifier must be initialized using the `v4l2_async_nf_init`.  Second,":
"`v4l2_async_nf_init` 初始化 notifier。其次，桥驱动可以开始形成",
"bridge drivers can then begin to form a list of async connection descriptors":
"桥设备运行所需的异步连接描述符列表。",
"that the bridge device needs for its":
"",
"operation. `v4l2_async_nf_add_fwnode`,":
"",
"Async connection descriptors describe connections to external sub-devices the":
"异步连接描述符描述与尚未被探测的外部子设备的连接。基于一个异步连接，",
"drivers for which are not yet probed. Based on an async connection, a media data":
"当相关子设备可用时，可能会创建一个 media 数据链路或辅助链路。对于一个给定的子设备，",
"or ancillary link may be created when the related sub-device becomes":
"可能有一个或多个异步连接，但在将这些连接添加到 notifier 时还不知道。异步连接会随着",
"available. There may be one or more async connections to a given sub-device but":
"匹配到的异步子设备被逐个绑定。",
"this is not known at the time of adding the connections to the notifier. Async":
"",
"connections are bound as matching async sub-devices are found, one by one.":
"",
"##### Asynchronous sub-device notifier for sub-devices":
"##### 用于子设备的异步子设备通知器",
"A driver that registers an asynchronous sub-device may also register an":
"注册异步子设备的驱动也可以注册一个异步 notifier。这称为异步子设备 notifier，",
"asynchronous notifier. This is called an asynchronous sub-device notifier and the":
"其过程与桥驱动类似，不同之处在于 notifier 是使用",
"`v4l2_async_subdev_nf_init` instead. A sub-device":
"`v4l2_async_subdev_nf_init` 初始化的。子设备 notifier 只能在 V4L2 设备",
"notifier may complete only after the V4L2 device becomes available, i.e. there's":
"可用之后才能完成，即存在一条经由异步子设备和 notifier 到达某个非异步子设备 notifier 的路径。",
"a path via async sub-devices and notifiers to a notifier that is not an":
"",
"asynchronous sub-device notifier.":
"",
"##### Asynchronous sub-device registration helper for camera sensor drivers":
"##### 用于摄像头传感器驱动的异步子设备注册辅助函数",
"`v4l2_async_register_subdev_sensor` is a helper function for sensor":
"`v4l2_async_register_subdev_sensor` 是一个用于传感器驱动的辅助函数，它注册",
"drivers registering their own async connection, but it also registers a notifier":
"自己的异步连接，同时还会注册一个 notifier，并进一步为在固件中找到的镜头和闪光灯设备",
"and further registers async connections for lens and flash devices found in":
"注册异步连接。该子设备的 notifier 会使用 `v4l2_async_unregister_subdev`",
"firmware. The notifier for the sub-device is unregistered and cleaned up with":
"随该异步子设备一起被注销和清理。",
"the async sub-device, using `v4l2_async_unregister_subdev`.":
"",
"##### Asynchronous sub-device notifier example":
"##### 异步子设备通知器示例",
"These functions allocate an async connection descriptor which is of type struct":
"这些函数分配一个异步连接描述符，其类型为 struct",
"`v4l2_async_connection` embedded in a driver-specific struct. The &struct":
"`v4l2_async_connection`，嵌入在一个驱动特定的结构体中。&struct",
"`v4l2_async_connection` shall be the first member of this struct:":
"`v4l2_async_connection` 应为该结构体的第一个成员：",
"The V4L2 core will then use these connection descriptors to match asynchronously":
"然后 V4L2 核心将使用这些连接描述符，将异步注册的子设备与之匹配。如果检测到匹配，",
"registered subdevices to them. If a match is detected the `.bound()` notifier":
"则调用 `.bound()` notifier 回调。在所有连接都绑定后，调用 .complete() 回调。",
"callback is called. After all connections have been bound the .complete()":
"当某个连接从系统中移除时，调用 `.unbind()` 方法。这三个回调都是可选的。",
"callback is called. When a connection is removed from the system the":
"",
"`.unbind()` method is called. All three callbacks are optional.":
"",
"Drivers can store any type of custom data in their driver-specific":
"驱动可以在其驱动特定的",
"`v4l2_async_connection` wrapper. If any of that data requires special":
"`v4l2_async_connection` 包装器中存储任何类型的自定义数据。如果该数据在结构体",
"handling when the structure is freed, drivers must implement the `.destroy()`":
"被释放时需要特殊处理，驱动必须实现 `.destroy()` notifier 回调。框架将在释放",
"notifier callback. The framework will call it right before freeing the":
"`v4l2_async_connection` 之前立即调用它。",
"`v4l2_async_connection`.":
"",
"#### Calling subdev operations":
"#### 调用子设备操作",
"The advantage of using `v4l2_subdev` is that it is a generic struct and":
"使用 `v4l2_subdev` 的优点在于它是一个通用结构体，",
"does not contain any knowledge about the underlying hardware. So a driver might":
"不包含任何关于底层硬件的信息。因此，一个驱动可能包含多个使用 I2C 总线的子设备，",
"contain several subdevs that use an I2C bus, but also a subdev that is":
"也可能包含一个通过 GPIO 引脚控制的子设备。这种区别只在设置设备时相关，",
"controlled through GPIO pins. This distinction is only relevant when setting":
"一旦子设备注册完成就完全透明了。",
"up the device, but once the subdev is registered it is completely transparent.":
"",
"Once the subdev has been registered you can call an ops function either":
"一旦子设备注册完成，你可以直接调用一个 ops 函数：",
"directly:":
"",
"but it is better and easier to use this macro:":
"但使用这个宏更好也更简单：",
"The macro will do the right `NULL` pointer checks and returns `-ENODEV`":
"该宏会执行正确的 `NULL` 指针检查，并在 `sd <v4l2_subdev>` 为 `NULL` 时返回 `-ENODEV`，",
"if `sd <v4l2_subdev>` is `NULL`, `-ENOIOCTLCMD` if either":
"在 `sd <v4l2_subdev>`->core 或 `sd <v4l2_subdev>`->core->g_std 为 `NULL` 时返回 `-ENOIOCTLCMD`，",
"`sd <v4l2_subdev>`->core or `sd <v4l2_subdev>`->core->g_std is `NULL`, or the actual result of the":
"否则返回 `sd <v4l2_subdev>`->ops->core->g_std ops 的实际结果。",
"`sd <v4l2_subdev>`->ops->core->g_std ops.":
"",
"It is also possible to call all or a subset of the sub-devices:":
"也可以调用所有或一部分子设备：",
"Any subdev that does not support this ops is skipped and error results are":
"任何不支持该 ops 的子设备都会被跳过，错误结果被忽略。如果要检查错误，请使用：",
"ignored. If you want to check for errors use this:":
"",
"Any error except `-ENOIOCTLCMD` will exit the loop with that error. If no":
"除 `-ENOIOCTLCMD` 之外的任何错误都会以该错误退出循环。如果没有",
"errors (except `-ENOIOCTLCMD`) occurred, then 0 is returned.":
"发生错误（除 `-ENOIOCTLCMD` 外），则返回 0。",
"The second argument to both calls is a group ID. If 0, then all subdevs are":
"这两个调用的第二个参数是一个组 ID（group ID）。如果为 0，则调用所有子设备。",
"called. If non-zero, then only those whose group ID match that value will":
"如果非零，则只调用组 ID 与该值匹配的子设备。在桥驱动注册子设备之前，它可以将",
"be called. Before a bridge driver registers a subdev it can set":
"`sd <v4l2_subdev>`->grp_id 设置为它想要的任何值（默认为 0）。该值由桥驱动拥有，",
"`sd <v4l2_subdev>`->grp_id to whatever value it wants (it's 0 by":
"子设备驱动永远不会修改或使用它。",
"default). This value is owned by the bridge driver and the sub-device driver":
"",
"will never modify or use it.":
"",
"The group ID gives the bridge driver more control how callbacks are called.":
"组 ID 让桥驱动能更好地控制回调的调用方式。例如，板上可能有多个音频芯片，",
"For example, there may be multiple audio chips on a board, each capable of":
"每个都能改变音量。但通常当用户想要改变音量时，只有其中一个会真正被使用。你可以将该子设备的",
"changing the volume. But usually only one will actually be used when the":
"组 ID 设置为例如 AUDIO_CONTROLLER，并在调用 `v4l2_device_call_all()` 时将其指定为组 ID 值。",
"user want to change the volume. You can set the group ID for that subdev to":
"这确保它只会发送到需要它的那个子设备。",
"e.g. AUDIO_CONTROLLER and specify that as the group ID value when calling":
"",
"`v4l2_device_call_all()`. That ensures that it will only go to the subdev":
"",
"that needs it.":
"",
"If the sub-device needs to notify its v4l2_device parent of an event, then":
"如果子设备需要向其 v4l2_device 父设备通知某个事件，它可以调用",
"it can call `v4l2_subdev_notify(sd, notification, arg)`. This macro checks":
"`v4l2_subdev_notify(sd, notification, arg)`。该宏检查是否定义了 `notify()` 回调，",
"whether there is a `notify()` callback defined and returns `-ENODEV` if not.":
"如果没有则返回 `-ENODEV`。否则返回 `notify()` 调用的结果。",
"Otherwise the result of the `notify()` call is returned.":
"",
"### V4L2 sub-device userspace API":
"### V4L2 子设备用户空间 API",
"Bridge drivers traditionally expose one or multiple video nodes to userspace,":
"桥驱动传统上向用户空间暴露一个或多个视频节点，并通过对视频节点操作的",
"and control subdevices through the `v4l2_subdev_ops` operations in":
"响应，通过 `v4l2_subdev_ops` 操作来控制子设备。这向应用程序隐藏了",
"response to video node operations. This hides the complexity of the underlying":
"底层硬件的复杂性。对于复杂设备，可能需要比视频节点提供的更细粒度的设备控制。",
"hardware from applications. For complex devices, finer-grained control of the":
"在这种情况下，实现了 media controller API <media_controller> 的桥驱动可以选择",
"device than what the video nodes offer may be required. In those cases, bridge":
"让子设备操作可直接从用户空间访问。",
"drivers that implement the media controller API <media_controller> may":
"",
"opt for making the subdevice operations directly accessible from userspace.":
"",
"Device nodes named `v4l-subdev`\\**X** can be created in `/dev` to access":
"可以在 `/dev` 中创建名为 `v4l-subdev`\\**X** 的设备节点，以直接访问",
"sub-devices directly. If a sub-device supports direct userspace configuration":
"子设备。如果子设备支持直接的用户空间配置，它必须在注册之前设置",
"it must set the `V4L2_SUBDEV_FL_HAS_DEVNODE` flag before being registered.":
"`V4L2_SUBDEV_FL_HAS_DEVNODE` 标志。",
"After registering sub-devices, the `v4l2_device` driver can create":
"注册子设备后，`v4l2_device` 驱动可以通过调用",
"device nodes for all registered sub-devices marked with":
"`v4l2_device_register_subdev_nodes` 为所有标记了",
"`V4L2_SUBDEV_FL_HAS_DEVNODE` by calling":
"`V4L2_SUBDEV_FL_HAS_DEVNODE` 的已注册子设备创建设备节点。当子设备被注销时，",
"`v4l2_device_register_subdev_nodes`. Those device nodes will be":
"这些设备节点会被自动移除。",
"automatically removed when sub-devices are unregistered.":
"",
"The device node handles a subset of the V4L2 API.":
"该设备节点处理 V4L2 API 的一个子集。",
"The controls ioctls are identical to the ones defined in V4L2. They":
"这些 controls ioctl 与 V4L2 中定义的完全相同。它们的行为也相同，唯一的例外是",
"behave identically, with the only exception that they deal only with":
"它们只处理在子设备中实现的控制项。取决于驱动，这些控制项也可以通过一个（或多个）",
"controls implemented in the sub-device. Depending on the driver, those":
"V4L2 设备节点访问。",
"controls can be also be accessed through one (or several) V4L2 device":
"",
"nodes.":
"",
"The events ioctls are identical to the ones defined in V4L2. They":
"这些 events ioctl 与 V4L2 中定义的完全相同。它们的行为也相同，唯一的例外是",
"behave identically, with the only exception that they deal only with":
"它们只处理由子设备生成的事件。取决于驱动，这些事件也可以由一个（或多个）",
"events generated by the sub-device. Depending on the driver, those":
"V4L2 设备节点报告。",
"events can also be reported by one (or several) V4L2 device nodes.":
"",
"Sub-device drivers that want to use events need to set the":
"希望使用事件的子设备驱动需要在注册子设备之前设置",
"`V4L2_SUBDEV_FL_HAS_EVENTS` `v4l2_subdev`.flags before registering":
"`V4L2_SUBDEV_FL_HAS_EVENTS` `v4l2_subdev`.flags。注册后，事件可以像往常一样",
"the sub-device. After registration events can be queued as usual on the":
"在 `v4l2_subdev`.devnode 设备节点上排队。",
"`v4l2_subdev`.devnode device node.":
"",
"To properly support events, the `poll()` file operation is also":
"为了正确支持事件，`poll()` 文件操作也被实现。",
"implemented.":
"",
"Private ioctls":
"私有 ioctl",
"All ioctls not in the above list are passed directly to the sub-device":
"上述列表之外的所有 ioctl 都通过 core::ioctl 操作直接传递给子设备",
"driver through the core::ioctl operation.":
"驱动。",
"### Read-only sub-device userspace API":
"### 只读子设备用户空间 API",
"Bridge drivers that control their connected subdevices through direct calls to":
"通过 `v4l2_subdev_ops` 结构体实现的 kernel API 直接调用控制其连接子设备的桥驱动，",
"the kernel API realized by `v4l2_subdev_ops` structure do not usually":
"通常不希望用户空间能够通过子设备设备节点更改相同的参数，因此通常不会注册任何此类节点。",
"want userspace to be able to change the same parameters through the subdevice":
"",
"device node and thus do not usually register any.":
"",
"It is sometimes useful to report to userspace the current subdevice":
"有时通过只读 API 向用户空间报告当前子设备的配置是很有用的，该 API 不允许",
"configuration through a read-only API, that does not permit applications to":
"应用程序更改设备参数，但允许与子设备设备节点交互以检查它们。",
"change to the device parameters but allows interfacing to the subdevice device":
"",
"node to inspect them.":
"",
"For instance, to implement cameras based on computational photography, userspace":
"例如，为了实现基于计算摄影的摄像头，用户空间需要了解每个受支持输出分辨率下",
"needs to know the detailed camera sensor configuration (in terms of skipping,":
"详细的摄像头传感器配置（包括跳过、合并（binning）、裁剪和缩放）。为了支持此类用例，",
"binning, cropping and scaling) for each supported output resolution. To support":
"桥驱动可以通过只读 API 将子设备操作暴露给用户空间。",
"such use cases, bridge drivers may expose the subdevice operations to userspace":
"",
"through a read-only API.":
"",
"To create a read-only device node for all the subdevices registered with the":
"要为所有使用 `V4L2_SUBDEV_FL_HAS_DEVNODE` 注册的子设备创建只读设备节点，",
"`V4L2_SUBDEV_FL_HAS_DEVNODE` set, the `v4l2_device` driver should call":
"`v4l2_device` 驱动应调用 `v4l2_device_register_ro_subdev_nodes`。",
"`v4l2_device_register_ro_subdev_nodes`.":
"",
"Access to the following ioctls for userspace applications is restricted on":
"对于使用 `v4l2_device_register_ro_subdev_nodes` 注册的子设备设备节点，",
"sub-device device nodes registered with":
"用户空间应用程序对以下 ioctl 的访问受到限制。",
"`v4l2_device_register_ro_subdev_nodes`.":
"",
"These ioctls are only allowed on a read-only subdevice device node":
"这些 ioctl 仅在只读子设备设备节点上被允许用于",
"for the V4L2_SUBDEV_FORMAT_TRY <v4l2-subdev-format-whence>":
"V4L2_SUBDEV_FORMAT_TRY <v4l2-subdev-format-whence> 的格式和选择矩形。",
"formats and selection rectangles.":
"",
"These ioctls are not allowed on a read-only subdevice node.":
"这些 ioctl 在只读子设备节点上不被允许。",
"In case the ioctl is not allowed, or the format to modify is set to":
"如果 ioctl 不被允许，或者要修改的格式被设置为",
"`V4L2_SUBDEV_FORMAT_ACTIVE`, the core returns a negative error code and":
"`V4L2_SUBDEV_FORMAT_ACTIVE`，核心返回一个负的错误码，并且 errno 变量被设置为 `-EPERM`。",
"the errno variable is set to `-EPERM`.":
"",
"### I2C sub-device drivers":
"### I2C 子设备驱动",
"Since these drivers are so common, special helper functions are available to":
"由于这些驱动非常常见，提供了专门的辅助函数以简化其使用（`v4l2-common.h`）。",
"ease the use of these drivers (`v4l2-common.h`).":
"",
"The recommended method of adding `v4l2_subdev` support to an I2C driver":
"向 I2C 驱动添加 `v4l2_subdev` 支持的推荐方法是将 `v4l2_subdev` 结构体",
"is to embed the `v4l2_subdev` struct into the state struct that is":
"嵌入到为每个 I2C 设备实例创建的状态结构体中。非常简单的设备没有状态结构体，",
"created for each I2C device instance. Very simple devices have no state":
"在这种情况下你可以直接创建一个 `v4l2_subdev`。",
"struct and in that case you can just create a `v4l2_subdev` directly.":
"",
"A typical state struct would look like this (where 'chipname' is replaced by":
"一个典型的状态结构体如下所示（其中 'chipname' 替换为芯片的名称）：",
"the name of the chip):":
"",
"Initialize the `v4l2_subdev` struct as follows:":
"按如下方式初始化 `v4l2_subdev` 结构体：",
"This function will fill in all the fields of `v4l2_subdev` ensure that":
"该函数将填充 `v4l2_subdev` 的所有字段，确保",
"the `v4l2_subdev` and i2c_client both point to one another.":
"`v4l2_subdev` 和 i2c_client 彼此指向对方。",
"You should also add a helper inline function to go from a `v4l2_subdev`":
"你还应该添加一个辅助内联函数，用于从 `v4l2_subdev`",
"pointer to a chipname_state struct:":
"指针转换到 chipname_state 结构体：",
"Use this to go from the `v4l2_subdev` struct to the `i2c_client`":
"用它来从 `v4l2_subdev` 结构体转换到 `i2c_client`",
"struct:":
"结构体：",
"Make sure to call":
"确保在调用 `remove()` 回调时调用",
"`v4l2_device_unregister_subdev`\\ (`sd <v4l2_subdev>`)":
"`v4l2_device_unregister_subdev`\\ (`sd <v4l2_subdev>`)。这将从桥驱动注销该子设备。",
"when the `remove()` callback is called. This will unregister the sub-device":
"即使该子设备从未被注册，调用它也是安全的。",
"from the bridge driver. It is safe to call this even if the sub-device was":
"",
"never registered.":
"",
"You need to do this because when the bridge driver destroys the i2c adapter":
"你需要这样做，因为当桥驱动销毁 i2c 适配器时，会调用该适配器上 i2c 设备的",
"the `remove()` callbacks are called of the i2c devices on that adapter.":
"`remove()` 回调。此后相应的 v4l2_subdev 结构体失效，因此必须先将它们注销。在",
"After that the corresponding v4l2_subdev structures are invalid, so they":
"`remove()` 回调中调用 `v4l2_device_unregister_subdev`\\ (`sd <v4l2_subdev>`)",
"have to be unregistered first. Calling":
"可确保这一点始终被正确完成。",
"`v4l2_device_unregister_subdev`\\ (`sd <v4l2_subdev>`)":
"",
"from the `remove()` callback ensures that this is always done correctly.":
"",
"The bridge driver also has some helper functions it can use:":
"桥驱动也有一些可以使用的辅助函数：",
"This loads the given module (can be `NULL` if no module needs to be loaded)":
"这会加载给定的模块（如果不需要加载模块则为 `NULL`），并使用给定的 `i2c_adapter` 和",
"and calls `i2c_new_client_device` with the given `i2c_adapter` and":
"芯片/地址参数调用 `i2c_new_client_device`。如果一切顺利，则将该子设备注册到",
"chip/address arguments. If all goes well, then it registers the subdev with":
"v4l2_device。",
"the v4l2_device.":
"",
"You can also use the last argument of `v4l2_i2c_new_subdev` to pass":
"你也可以使用 `v4l2_i2c_new_subdev` 的最后一个参数来传递一个它应该探测的",
"an array of possible I2C addresses that it should probe. These probe addresses":
"可能的 I2C 地址数组。这些探测地址仅在前一个参数为 0 时使用。非零参数意味着你知道",
"are only used if the previous argument is 0. A non-zero argument means that you":
"确切的 i2c 地址，因此在这种情况下不会进行探测。",
"know the exact i2c address so in that case no probing will take place.":
"",
"Both functions return `NULL` if something went wrong.":
"如果出现问题，这两个函数都返回 `NULL`。",
"Note that the chipid you pass to `v4l2_i2c_new_subdev` is usually":
"请注意，你传递给 `v4l2_i2c_new_subdev` 的 chipid 通常与模块名称相同。它允许你",
"the same as the module name. It allows you to specify a chip variant, e.g.":
"指定一个芯片变体，例如 \"saa7114\" 或 \"saa7115\"。不过一般来说，i2c 驱动会自动检测这一点。",
"\"saa7114\" or \"saa7115\". In general though the i2c driver autodetects this.":
"chipid 的使用是需要在日后更仔细研究的事情。它在不同的 i2c 驱动之间有所差异，因此可能令人困惑。",
"The use of chipid is something that needs to be looked at more closely at a":
"要查看支持哪些芯片变体，可以查看 i2c 驱动代码中的 i2c_device_id 表。它列出了所有可能性。",
"later date. It differs between i2c drivers and as such can be confusing.":
"",
"To see which chip variants are supported you can look in the i2c driver code":
"",
"for the i2c_device_id table. This lists all the possibilities.":
"",
"There are one more helper function:":
"还有一个辅助函数：",
"`v4l2_i2c_new_subdev_board` uses an `i2c_board_info` struct":
"`v4l2_i2c_new_subdev_board` 使用一个 `i2c_board_info` 结构体，",
"which is passed to the i2c driver and replaces the irq, platform_data and addr":
"该结构体被传递给 i2c 驱动，并替代 irq、platform_data 和 addr 参数。",
"arguments.":
"",
"If the subdev supports the s_config core ops, then that op is called with":
"如果子设备支持 s_config core ops，则在子设备设置完成后，会以 irq 和 platform_data 参数调用该 op。",
"the irq and platform_data arguments after the subdev was setup.":
"",
"The `v4l2_i2c_new_subdev` function will call":
"`v4l2_i2c_new_subdev` 函数会在内部调用",
"`v4l2_i2c_new_subdev_board`, internally filling a":
"`v4l2_i2c_new_subdev_board`，使用 `client_type` 和",
"`i2c_board_info` structure using the `client_type` and the":
"`addr` 填充一个 `i2c_board_info` 结构体。",
"`addr` to fill it.":
"",
"### Centrally managed subdev active state":
"### 集中管理的子设备活动状态",
"Traditionally V4L2 subdev drivers maintained internal state for the active":
"传统上，V4L2 子设备驱动为活动设备配置维护内部状态。这通常实现为例如一个",
"device configuration. This is often implemented as e.g. an array of struct":
"v4l2_mbus_framefmt 结构体数组，每个 pad 一个条目，裁剪（crop）和合成（compose）矩形也类似。",
"v4l2_mbus_framefmt, one entry for each pad, and similarly for crop and compose":
"",
"rectangles.":
"",
"In addition to the active configuration, each subdev file handle has a struct":
"除了活动配置外，每个子设备文件句柄都有一个由 V4L2 核心管理的 struct",
"v4l2_subdev_state, managed by the V4L2 core, which contains the try":
"v4l2_subdev_state，其中包含 try",
"configuration.":
"配置。",
"To simplify the subdev drivers the V4L2 subdev API now optionally supports a":
"为了简化子设备驱动，V4L2 子设备 API 现在可选地支持由",
"centrally managed active configuration represented by":
"`v4l2_subdev_state` 表示的集中管理的活动配置。一个包含活动设备配置的",
"`v4l2_subdev_state`. One instance of state, which contains the active":
"状态实例，作为 `v4l2_subdev` 结构体的一部分存储在子设备自身中；而核心将",
"device configuration, is stored in the sub-device itself as part of":
"一个 try 状态关联到每个打开的文件句柄，以存储与该文件句柄相关的 try 配置。",
"the `v4l2_subdev` structure, while the core associates a try state to":
"",
"each open file handle, to store the try configuration related to that file":
"",
"handle.":
"",
"Sub-device drivers can opt-in and use state to manage their active configuration":
"子设备驱动可以选择使用 state 来管理其活动配置，方法是在注册子设备之前调用",
"by initializing the subdevice state with a call to v4l2_subdev_init_finalize()":
"v4l2_subdev_init_finalize() 来初始化子设备状态。它们还必须在注销子设备之前调用",
"before registering the sub-device. They must also call v4l2_subdev_cleanup()":
"v4l2_subdev_cleanup() 来释放所有已分配的资源。核心会自动为每个打开的文件句柄",
"to release all the allocated resources before unregistering the sub-device.":
"分配并初始化一个状态以存储 try 配置，并在关闭文件句柄时释放它。",
"The core automatically allocates and initializes a state for each open file":
"",
"handle to store the try configurations and frees it when closing the file":
"",
"handle.":
"",
"V4L2 sub-device operations that use both the :ref:`ACTIVE and TRY formats":
"同时使用 :ref:`ACTIVE 和 TRY 格式 <v4l2-subdev-format-whence>` 的 V4L2 子设备操作，",
"<v4l2-subdev-format-whence>` receive the correct state to operate on through":
"通过 'state' 参数接收要操作的正确状态。调用者必须通过调用",
"the 'state' parameter. The state must be locked and unlocked by the":
"`v4l2_subdev_lock_state()` 和 `v4l2_subdev_unlock_state()` 来锁定和解锁该状态。",
"caller by calling `v4l2_subdev_lock_state()` and":
"调用者可以通过 `v4l2_subdev_call_state_active()` 宏来调用子设备操作。",
"`v4l2_subdev_unlock_state()`. The caller can do so by calling the subdev":
"",
"operation through the `v4l2_subdev_call_state_active()` macro.":
"",
"Operations that do not receive a state parameter implicitly operate on the":
"不接收 state 参数的操作隐式地对子设备活动状态进行操作，驱动可以通过调用",
"subdevice active state, which drivers can exclusively access by":
"`v4l2_subdev_lock_and_get_active_state()` 独占访问该状态。子设备的活动状态",
"calling `v4l2_subdev_lock_and_get_active_state()`. The sub-device active":
"同样必须通过调用 `v4l2_subdev_unlock_state()` 来释放。",
"state must equally be released by calling `v4l2_subdev_unlock_state()`.":
"",
"Drivers must never manually access the state stored in the `v4l2_subdev`":
"驱动绝不能直接手动访问存储在 `v4l2_subdev` 或文件句柄中的状态，而不经过指定的辅助函数。",
"or in the file handle without going through the designated helpers.":
"",
"While the V4L2 core passes the correct try or active state to the subdevice":
"虽然 V4L2 核心会将正确的 try 或活动状态传递给子设备操作，但许多现有的设备驱动在调用",
"operations, many existing device drivers pass a NULL state when calling":
"`v4l2_subdev_call()` 操作时会传递一个 NULL 状态。这种遗留写法会给让 V4L2 核心管理活动状态的",
"operations with `v4l2_subdev_call()`. This legacy construct causes":
"子设备驱动带来问题，因为它们期望接收适当的状态作为参敎数。为了帮助子设备驱动转换为受管理的活动状态，",
"issues with subdevice drivers that let the V4L2 core manage the active state,":
"而无需同时转换所有调用者，在 v4l2_subdev_call() 中添加了一个额外的封装层，它通过获取并锁定",
"as they expect to receive the appropriate state as a parameter. To help the":
"被调用者的活动状态（使用 `v4l2_subdev_lock_and_get_active_state()`）来处理 NULL 情况，",
"conversion of subdevice drivers to a managed active state without having to":
"并在调用后解锁该状态。",
"convert all callers at the same time, an additional wrapper layer has been":
"",
"added to v4l2_subdev_call(), which handles the NULL case by getting and locking":
"",
"the callee's active state with `v4l2_subdev_lock_and_get_active_state()`,":
"",
"and unlocking the state after the call.":
"",
"The whole subdev state is in reality split into three parts: the":
"整个子设备状态实际上分为三个部分：v4l2_subdev_state、子设备控制项（controls）和子设备驱动的",
"v4l2_subdev_state, subdev controls and subdev driver's internal state. In the":
"内部状态。将来这些部分应合并为单一状态。目前我们需要一种方法来处理这些部分的锁定。这可以通过",
"future these parts should be combined into a single state. For the time being":
"共享一个锁来实现。v4l2_ctrl_handler 已经通过其 'lock' 指针支持这一点，状态也使用相同的模型。驱动可以在",
"we need a way to handle the locking for these parts. This can be accomplished":
"调用 v4l2_subdev_init_finalize() 之前执行以下操作：",
"by sharing a lock. The v4l2_ctrl_handler already supports this via its 'lock'":
"",
"pointer and the same model is used with states. The driver can do the following":
"",
"before calling v4l2_subdev_init_finalize():":
"",
"This shares the driver's private mutex between the controls and the states.":
"这在控制项和状态之间共享驱动的私有互斥锁。",
"### Streams, multiplexed media pads and internal routing":
"### 流、多路复用 media pads 与内部路由",
"A subdevice driver can implement support for multiplexed streams by setting":
"子设备驱动可以通过设置 V4L2_SUBDEV_FL_STREAMS 子设备标志，并实现集中管理的",
"the V4L2_SUBDEV_FL_STREAMS subdev flag and implementing support for":
"子设备活动状态、路由以及基于流的配置，从而实现对多路复用流的支持。",
"centrally managed subdev active state, routing and stream based":
"",
"configuration.":
"",
"### V4L2 sub-device functions and data structures":
"### V4L2 子设备函数与数据结构",
}

src = open(F, encoding='utf-8').read()
out_lines = []
for line in src.split('\n'):
    key = line.strip()
    if key in T:
        indent = line[:len(line)-len(line.lstrip())]
        out_lines.append(indent + T[key])
    else:
        out_lines.append(line)
new = '\n'.join(out_lines)

n = new.count('```')
assert n % 2 == 0, "fence odd: %d" % n

tmp = F + '.tmp'
open(tmp, 'w', encoding='utf-8').write(new)
os.replace(tmp, F)
print("v4l2-subdev done; fences:", n)
