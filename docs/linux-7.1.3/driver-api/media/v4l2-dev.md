
## 视频设备的内部表示


`/dev` 目录中的实际设备节点是使用 `video_device` 结构体（`v4l2-dev.h`）
创建的。该结构体既可以动态分配，也可以内嵌到更大的结构体中。

要动态分配它，使用 `video_device_alloc`：


	struct video_device *vdev = video_device_alloc();

	if (vdev == NULL)
		return -ENOMEM;

	vdev->release = video_device_release;

如果你把它内嵌到更大的结构体中，那么必须将 `release()` 回调设置为你
自己的函数：


	struct video_device *vdev = &my_vdev->vdev;

	vdev->release = my_vdev_release;

必须设置 `release()` 回调，它会在视频设备的最后一个使用者退出时被调用。

默认的 `video_device_release` 回调目前只是调用 `kfree` 来释放所分配的
内存。

还有一个 `video_device_release_empty` 函数，它什么也不做（为空），当结构体
被内嵌、且释放时无事可做时应当使用它。

你还应当设置 `video_device` 的以下字段：

- `video_device`->v4l2_dev：必须设置为父设备 `v4l2_device`。

- `video_device`->name：设置为有描述性且唯一的值。

- `video_device`->vfl_dir：对于采集设备（capture）设置为
  `VFL_DIR_RX`（`VFL_DIR_RX` 的值为 0，所以这通常已经是默认值），
  对于输出设备设置为 `VFL_DIR_TX`，对于 mem2mem（编解码）设备设置为
  `VFL_DIR_M2M`。

- `video_device`->fops：设置为 `v4l2_file_operations` 结构体。

- `video_device`->ioctl_ops：如果你使用 `v4l2_ioctl_ops`
  来简化 ioctl 的维护（强烈建议使用，并且将来可能变为强制要求！），
  则将其设置为你的 `v4l2_ioctl_ops` 结构体。`video_device`->vfl_type 和
  `video_device`->vfl_dir 字段用于禁用与类型/方向组合不匹配的操作。
  例如，非 VBI 节点会禁用 VBI 操作，采集设备会禁用输出操作。这样就有可能
  仅为 vbi 和 video 节点提供同一个 `v4l2_ioctl_ops` 结构体。

- `video_device`->lock：如果你想在驱动中完成所有的加锁，则保留为
  `NULL`。否则你要给它一个指向 `mutex_lock` 结构体的指针，在
  `video_device`->unlocked_ioctl 文件操作被调用之前，核心层会获取该锁，
  并在调用之后释放它。更多细节请参阅下一节。

- `video_device`->queue：一个指向与本设备节点关联的 struct vb2_queue
  的指针。如果 queue 不为 `NULL`，且 queue->lock 不为 `NULL`，那么对于
  排队类 ioctl（`VIDIOC_REQBUFS`、`CREATE_BUFS`、`QBUF`、`DQBUF`、
  `QUERYBUF`、`PREPARE_BUF`、`STREAMON` 和 `STREAMOFF`），会使用
  queue->lock 而非上面的锁。这样 vb2 <vb2_framework> 排队框架就无需
  等待其他 ioctl。该 queue 指针也被 vb2 <vb2_framework> 辅助函数用来
  检查排队的归属（即调用它的文件句柄是否被允许执行该操作）。

- `video_device`->prio：跟踪优先级。用于实现 `VIDIOC_G_PRIORITY`
  和 `VIDIOC_S_PRIORITY`。如果保留为 `NULL`，则会使用 `v4l2_device`
  中的 struct v4l2_prio_state。如果你想让每个（组）设备节点拥有独立的
  优先级状态，那么可以将其指向你自己的 struct `v4l2_prio_state`。

- `video_device`->dev_parent：仅当 v4l2_device 以 `NULL` 作为父
  `device` 结构体注册时才设置它。这种情况只出现在一个硬件设备拥有多个
  共享同一个 `v4l2_device` 核心的 PCI 设备时。

  cx88 驱动就是一个例子：一个核心 `v4l2_device` 结构体，但被一个原始视频
  PCI 设备（cx8800）和一个 MPEG PCI 设备（cx8802）共同使用。由于
  `v4l2_device` 不能同时关联两个 PCI 设备，它在建立时未设置父设备。
  但在初始化 struct video_device 时你**确实**知道该使用哪个父 PCI 设备，
  因此你将 `dev_device` 设置为正确的 PCI 设备。

如果你使用 `v4l2_ioctl_ops`，那么应当在你的 `v4l2_file_operations`
结构体中把 `video_device`->unlocked_ioctl 设置为 `video_ioctl2`。

在某些情况下，你想告知核心：你在 `v4l2_ioctl_ops` 中指定的某个函数应当被
忽略。你可以在调用 `video_register_device` 之前通过调用以下函数来标记此类
ioctl：

	`v4l2_disable_ioctl <v4l2_disable_ioctl>`
	(`vdev <video_device>`, cmd).

如果你希望基于外部因素（例如所使用的卡）关闭 `v4l2_ioctl_ops` 中的某些
特性，而又不想新建一个结构体，通常就需要这样做。

`v4l2_file_operations` 结构体是 file_operations 的一个子集。主要区别在于
省略了 inode 参数，因为它从未被使用。

如果需要与 media framework 集成，你必须通过调用 `media_entity_pads_init`
来初始化内嵌在 `video_device` 结构体中的 `media_entity` 结构体
（entity 字段）：


	struct media_pad *pad = &my_vdev->pad;
	int err;

	err = media_entity_pads_init(&vdev->entity, 1, pad);

pads 数组必须事先初始化完毕。无需手动设置 struct media_entity 的 type 和
name 字段。

当视频设备被打开/关闭时，对该 entity 的引用会被自动获取/释放。

### ioctls 与加锁


V4L 核心提供可选的加锁服务。主要的服务是 struct video_device 中的 lock
字段，它是一个指向互斥体的指针。如果你设置了该指针，那么 unlocked_ioctl
将使用它来串行化所有 ioctl。

如果你使用的是 videobuf2 框架 <vb2_framework>，那么还可以设置第二个锁：
`video_device`->queue->lock。如果设置了它，那么对于所有排队类 ioctl
（完整列表见上一节），将使用该锁而非 `video_device`->lock 来串行化。

对排队类 ioctl 使用不同锁的好处在于，对于某些驱动（尤其是 USB 驱动），
某些命令（例如设置控制项）可能耗时较长，因此你希望对缓冲区排队类 ioctl
使用独立的锁。这样你的 `VIDIOC_DQBUF` 就不会因为驱动正忙于更改（例如）
摄像头曝光参数而停滞。

当然，你也可以始终将那两个锁指针都保留为 `NULL`，自行完成所有的加锁。

在使用 videobuf2 <vb2_framework> 的情况下，你必须将 `queue->lock`
指针设置为你用于串行化排队类 ioctl 的锁。这能确保在 `VIDIOC_DQBUF`
等待缓冲区到达时该锁被释放，并在之后重新获取。

热插拔断开的实现也应当在调用 v4l2_device_disconnect 之前获取
`video_device` 上的锁。如果你还使用了 `video_device`->queue->lock，那么
必须先锁定 `video_device`->queue->lock，再锁定 `video_device`->lock。
这样你可以确保调用 `v4l2_device_disconnect` 时没有 ioctl 正在运行。

### 视频设备注册


接下来，你使用 `video_register_device` 注册视频设备。这会为你创建字符
设备。


	err = video_register_device(vdev, VFL_TYPE_VIDEO, -1);
	if (err) {
		video_device_release(vdev); /** or kfree(my_vdev); **/
		return err;
	}

如果 `v4l2_device` 父设备拥有非 `NULL` 的 mdev 字段，那么该视频设备的
entity 会自动注册到 media 设备。

注册哪个设备取决于 type 参数。现有的类型如下：

========================== ====================	 ==============================
`vfl_devnode_type` 设备名		     用途
========================== ====================	 ==============================
`VFL_TYPE_VIDEO`         `/dev/videoX`       用于视频输入/输出设备
`VFL_TYPE_VBI`           `/dev/vbiX`         用于垂直消隐数据（即字幕、
					     图文电视）
`VFL_TYPE_RADIO`         `/dev/radioX`       用于收音机调谐器
`VFL_TYPE_SUBDEV`        `/dev/v4l-subdevX`  用于 V4L2 子设备
`VFL_TYPE_SDR`           `/dev/swradioX`     用于软件定义无线电（SDR）
					     调谐器
`VFL_TYPE_TOUCH`         `/dev/v4l-touchX`   用于触摸传感器
========================== ====================	 ==============================

最后一个参数让你可以对所使用的设备节点编号（即 `videoX` 中的 X）施加一定
程度的控制。通常你会传入 -1，让 v4l2 框架挑选第一个空闲编号。但有时用户
希望选择特定的节点编号。驱动通常允许用户在驱动模块选项中指定特定的设备节点
编号。该编号随后被传给此函数，video_register_device 会尝试选择该设备节点
编号。如果该编号已被占用，则会选择下一个空闲的设备节点编号，并向内核日志
发送一条警告。

另一个使用场景是：如果驱动创建了很多设备。此时把不同的视频设备放在不同的
区间中可能会很有用。例如，视频采集设备从 0 开始，视频输出设备从 16 开始。
因此你可以使用最后一个参数来指定最小的设备节点编号，v4l2 框架会尝试挑选
等于或大于你所传入值的第一个空闲编号。如果失败，则只会挑选第一个空闲编号。

既然在这种情况下你并不关心无法选择指定设备节点编号的警告，你可以改为调用
`video_register_device_no_warn` 函数。

每当创建设备节点时，也会为你创建一些属性。如果你查看
`/sys/class/video4linux`，就能看到这些设备。进入例如 `video0`，你会看到
'name'、'dev_debug' 和 'index' 属性。'name' 属性就是 video_device 结构体的
'name' 字段。'dev_debug' 属性可用于启用核心调试。更详细的信息请参阅下一节。

'index' 属性是设备节点的索引：每调用一次 `video_register_device()`，索引就
加 1。你注册的第一个视频设备节点总是从索引 0 开始。

用户可以设置利用 index 属性的 udev 规则，以生成花哨的设备名（例如用于 MPEG
视频采集设备节点的 '`mpegX`'）。

设备成功注册后，你可以使用以下字段：

- `video_device`->vfl_type：传给 `video_register_device` 的设备类型。
- `video_device`->minor：所分配的设备次设备号。
- `video_device`->num：设备节点编号（即 `videoX` 中的 X）。
- `video_device`->index：设备索引号。

如果注册失败，那么你需要调用 `video_device_release` 来释放所分配的
`video_device` 结构体，或者如果该 `video_device` 是内嵌的，则释放你自己的
结构体。如果注册失败，`vdev->release()` 回调永远不会被调用，你也不应尝试
在注册失败的情况下注销该设备。

### 视频设备调试


为每个视频、vbi、radio 或 swradio 设备在 `/sys/class/video4linux/<devX>/`
下创建的 'dev_debug' 属性，可用于启用文件操作的日志。

它是一个位掩码，可以设置以下位：

===== ================================================================
掩码  描述
===== ================================================================
0x01  记录 ioctl 名称与错误码。VIDIOC_(D)QBUF ioctl 仅在 0x08 位也被
      设置时才会被记录。
0x02  记录 ioctl 名称参数与错误码。VIDIOC_(D)QBUF ioctl 仅在 0x08 位
      也被设置时才会被记录。
0x04  记录文件操作 open、release、read、write、mmap 和
      get_unmapped_area。read 和 write 操作仅在 0x08 位被设置时
      才会被记录。
0x08  记录 read 和 write 文件操作，以及 VIDIOC_QBUF 和
      VIDIOC_DQBUF ioctl。
0x10  记录 poll 文件操作。
0x20  记录控制操作中的错误与消息。
===== ================================================================

### 视频设备清理


当必须移除视频设备节点时（无论是驱动卸载期间，还是因为 USB 设备被断开），
你应当使用以下方式注销它们：

	`video_unregister_device`
	(`vdev <video_device>`);

这会将设备节点从 sysfs 中移除（导致 udev 将它们从 `/dev` 中移除）。

`video_unregister_device` 返回后，不能再打开新的设备。但是，对于 USB 设备，
某些应用程序可能仍然打开了其中一个设备节点。因此在注销之后，所有文件操作
（当然，release 除外）也都会返回错误。

当视频设备节点的最后一个使用者退出时，会调用 `vdev->release()`
回调，你可以在那里进行最终的清理。

如果已初始化，别忘了清理与视频设备关联的 media entity：

	`media_entity_cleanup <media_entity_cleanup>`
	(&vdev->entity);

这可以从 release 回调中完成。


### 辅助函数


有一些有用的辅助函数：

- 文件与 `video_device` 私有数据

你可以使用以下方式在 video_device 结构体中设置/获取驱动私有数据：

	`video_get_drvdata <video_get_drvdata>`
	(`vdev <video_device>`);

	`video_set_drvdata <video_set_drvdata>`
	(`vdev <video_device>`);

注意，你可以在调用 `video_register_device` 之前安全地调用
`video_set_drvdata`。

还有这个函数：

	`video_devdata <video_devdata>`
	(struct file \*file);

返回属于该 file 结构体的 video_device。

`video_devdata` 函数将 `video_get_drvdata` 与 `video_devdata` 结合起来：

	`video_drvdata <video_drvdata>`
	(struct file \*file);

你可以使用以下方式从 `video_device` 结构体转到 v4l2_device 结构体：


	struct v4l2_device *v4l2_dev = vdev->v4l2_dev;

- 设备节点名

`video_device` 节点的内核名可以使用以下方式获取：

	`video_device_node_name <video_device_node_name>`
	(`vdev <video_device>`);

该名称被 udev 等用户空间工具用作提示。应当尽可能使用该函数，而不要直接
访问 video_device 的 **num** 与 **video_device** 的 **minor** 字段。

### video_device 函数与数据结构
