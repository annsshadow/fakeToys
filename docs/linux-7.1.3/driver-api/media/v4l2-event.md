
### V4L2 事件


V4L2 事件提供了一种向用户空间传递事件的通用方式。驱动必须使`v4l2_fh` 才能
支持 V4L2 事件
事件是按文件句柄（filehandle）订阅的。一个事件规范由一`type` 组成，并可选地
关联一个通过 `id` 字段标识的对象。如果未使用，则 `id` 0。因此一个事件由
`(type, id)` 元组唯一标识
`v4l2_fh` 结构在其 `subscribed` 字段上有一个已订阅事件的列表
当用户订阅一个事件时，会`v4l2_fh`\ `.subscribed` 添加一`v4l2_subscribed_event`
结构，每个已订阅的事件对应一个
每个 `v4l2_subscribed_event` 结构以一`v4l2_kevent` 环形缓冲区结尾，其大小由
`v4l2_event_subscribe` 的调用者给定。该环形缓冲区用于存储驱动产生的任何事件
因此每个 `(type, ID)` 事件元组都有自己`v4l2_kevent` 环形缓冲区。这保证了如驱动在短时间内生成大量某一类型的事件，不会覆盖另一类型的事件
但是，如果某一类型的事件数量超过了 `v4l2_kevent` 环形缓冲区的大小，那么最旧的
事件将被丢弃，新事件被加入
`v4l2_kevent` 结构链入 `v4l2_fh` 结构`available` 列表，这VIDIOC_DQEVENT 知道应该先出队哪个事件
最后，如果事件订阅关联了某个特定对象（例如一V4L2 控制），那么该对象也需要知这一点，以便该对象能够产生事件。因`node` 字段可用于将 `v4l2_subscribed_event`
结构链入此类对象的一个列表中
总结一下：

- struct v4l2_fh 有两个列表：一个是 `subscribed` 事件列表，一个是 `available` 事件列表
- struct v4l2_subscribed_event 有一个该特定类型已产生（pending）事件的环形缓冲区
- 如果 struct v4l2_subscribed_event 关联了某个特定对象，那么该对象会有一个内部的
  struct v4l2_subscribed_event 列表，以便知道谁向该对象订阅了事件
此外，内部的 struct v4l2_subscribed_event 有驱动可以设置的 `merge()` `replace()`
回调。当产生一个新事件且没有更多空间时，会调用这些回调
`replace()` 回调允许你将旧事件的载荷替换为新事件的载荷，将旧载荷中的任何相关数据
合并到替换它的新载荷中。当此事件类型的环形缓冲区大小为 1（即环形缓冲区中只能存储
一个事件）时调用它
`merge()` 回调允许你将最旧事件的载荷合并到第二旧事件的载荷中。当环形缓冲区大大于 1 时调用它
这样就不会丢失任何状态信息，只会丢失到达该状态的中间步骤
这些 `replace`/`merge` 回调的一个好例子v4l2-event.c 中：用于控件事件`ctrls_replace()` `ctrls_merge()` 回调
	这些回调可能在中断上下文中被调用，因此它们必须很快
为了将事件排队到视频设备，驱动应调用
	`v4l2_event_queue <v4l2_event_queue>`
	(`vdev <video_device>`, `ev <v4l2_event>`)

驱动唯一的责任是填写 type data 字段。其它字段将V4L2 填写
#### 事件订阅


订阅一个事件通过以下方式
	`v4l2_event_subscribe <v4l2_event_subscribe>`
	(`fh <v4l2_fh>`, `sub <v4l2_event_subscription>` ,
	elems, `ops <v4l2_subscribed_event_ops>`)


该函数用于实`video_device`->
`ioctl_ops <v4l2_ioctl_ops>`-> `vidioc_subscribe_event`，但驱动必须首先检查驱动是能够产生具有指定事件 id 的事件，然后才调`v4l2_event_subscribe` 来订阅该事件
elems 参数是该事件的事件队列大小。如果为 0，框架将填入一个默认值（取决于事件类型）
ops 参数允许驱动指定若干回调

======== ==============================================================
Callback Description
======== ==============================================================
add      在添加一个新的监听者时调用（对同一事件订阅两次只会导致此回调被调用一次）
del      在监听者停止监听时调用
replace  用事‘new替换事件 ‘old’merge    将事‘old合并到事‘new中======== ==============================================================

全部 4 个回调都是可选的，如果你不想指定任何回调，ops 参数本身可以`NULL`
#### 取消订阅一个事

取消订阅一个事件通过以下方式
	`v4l2_event_unsubscribe <v4l2_event_unsubscribe>`
	(`fh <v4l2_fh>`, `sub <v4l2_event_subscription>`)

该函数用于实`video_device`->
`ioctl_ops <v4l2_ioctl_ops>`-> `vidioc_unsubscribe_event`驱动可以直接调用 `v4l2_event_unsubscribe`，除非它希望介入取消订阅过程
特殊类型 `V4L2_EVENT_ALL` 可用于取消订阅所有事件。驱动可能希望以特殊方式处理它
#### 检查是否有挂起的事

检查是否有挂起的事件通过以下方式
	`v4l2_event_pending <v4l2_event_pending>`
	(`fh <v4l2_fh>`)


该函数返回挂起事件的数量。在实现 poll 时很有用
#### 事件如何工作


事件通过 poll 系统调用传递给用户空间。驱动可以使`v4l2_fh`->wait（一wait_queue_head_t）作`poll_wait()` 的参数
有标准事件和私有事件。新的标准事件必须使smallest available 事件类型。驱动必须从
自己的类（class）中、以类基址（class base）为起点分配它们的事件。类基址`V4L2_EVENT_PRIVATE_START` + n * 1000，其n 是最小可用数字。类中的第一个事件类保留供将来使用，因此第一个可用事件类型是 ‘class base + 1’
关于如何使用 V4L2 事件的一个示例可以在 OMAP 3 ISP 驱动
（`drivers/media/platform/ti/omap3isp`）中找到
一个子设备（subdev）可以直接通过 `V4L2_DEVICE_NOTIFY_EVENT` `v4l2_device` notify 函数发送事件。这让桥接（bridge）驱动能够将发送事件的子设备映射到需要被通知
此类事件的、与该子设备关联的视频节点
##### V4L2 事件函数与数据结

