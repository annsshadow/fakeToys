#### USB Request Block（URB）


:Revised: 2000-Dec-05
:Again:   2002-Jul-06
:Again:   2005-Sep-19
:Again:   2017-Mar-29



    USB 子系统现在在 usb-hostside-api 章节有一个相当完整的部分，由当前
    源代码生成。这份特定的文档并不完整，也可能没有更新到最新版本；
    除快速概览之外，请勿依赖它。

## 基本概念，或者说“什么是 URB？”


新驱动的基本思想是消息传递，消息本身被称为 USB 请求块（USB Request
Block），简称 URB。

- 一个 URB 包含执行任何 USB 事务并将数据与状态回传所需的全部相关信息。

- URB 的执行本质上是一个异步操作，即 `usb_submit_urb` 调用在成功地将
  所请求的操作加入队列后会立即返回。

- 一个 URB 的传输可以随时通过 `usb_unlink_urb` 取消。

- 每个 URB 都有一个完成处理函数，在动作成功完成或取消后被调用。URB
  还包含一个用于向完成处理函数传递信息的上下文指针。

- 设备的每个端点从逻辑上都支持一个请求队列。你可以填满该队列，这样当
  你的驱动处理另一个请求的完成时，USB 硬件仍能向某个端点传输数据。这能
  最大化 USB 带宽的利用，并在使用周期性传输模式时支持与设备之间无缝的
  数据流传输。


## URB 结构


```

  struct urb
  {
  // (IN) device and pipe specify the endpoint queue
	struct usb_device *dev;         // pointer to associated USB device
	unsigned int pipe;              // endpoint information

	unsigned int transfer_flags;    // URB_ISO_ASAP, URB_SHORT_NOT_OK, etc.

  // (IN) all urbs need completion routines
	void *context;                  // context for completion routine
	usb_complete_t complete;        // pointer to completion routine

  // (OUT) status after each completion
	int status;                     // returned status

  // (IN) buffer used for data transfers
	void *transfer_buffer;          // associated data buffer
	u32 transfer_buffer_length;     // data buffer length
	int number_of_packets;          // size of iso_frame_desc

  // (OUT) sometimes only part of CTRL/BULK/INTR transfer_buffer is used
	u32 actual_length;              // actual data buffer length

  // (IN) setup stage for CTRL (pass a struct usb_ctrlrequest)
	unsigned char *setup_packet;    // setup packet (control only)

  // Only for PERIODIC transfers (ISO, INTERRUPT)
    // (IN/OUT) start_frame is set unless URB_ISO_ASAP isn't set
	int start_frame;                // start frame
	int interval;                   // polling interval

    // ISO only: packets are only "best effort"; each can have errors
	int error_count;                // number of errors
	struct usb_iso_packet_descriptor iso_frame_desc[0];
  };

```
你的驱动必须使用它所声明接口中相应端点描述符的值来创建“pipe”值。


## 如何获取一个 URB？


```

	struct urb *usb_alloc_urb(int isoframes, int mem_flags)

```
返回值是所分配 URB 的指针，若分配失败则为 0。参数 isoframes 指定了你
想要调度的等时传输帧的数量。对于 CTRL/BULK/INT，使用 0。mem_flags 参数
保存标准的存储分配标志，使你能控制（除其他外）底层代码是否可能阻塞。

```

	void usb_free_urb(struct urb *urb)

```
你可以释放一个已经提交、但还未在 completion 回调中返回给你的 urb。它会在
不再使用时被自动释放。


## 需要填充哪些内容？


根据事务类型的不同，`linux/usb.h` 中定义了一些内联函数来简化初始化，
例如 `usb_fill_control_urb`、`usb_fill_bulk_urb` 和
`usb_fill_int_urb`。一般而言，它们需要 usb 设备指针、pipe（usb.h 中
的常用格式）、传输缓冲区、期望的传输长度、完成处理函数及其上下文。可以
查看一些已有的驱动来了解它们的用法。

标志：

- 对于 ISO，有两种启动行为：指定的 start_frame 或 ASAP。
- 对于 ASAP，在 transfer_flags 中设置 `URB_ISO_ASAP`。

如果不允许短包，则在 transfer_flags 中设置 `URB_SHORT_NOT_OK`。


## 如何提交一个 URB？


```

	int usb_submit_urb(struct urb *urb, int mem_flags)

```
`mem_flags` 参数（如 `GFP_ATOMIC`）控制存储分配，例如当内存紧张时
底层是否可能阻塞。

它会立即返回，返回状态为 0（请求已入队）或某个错误码，通常由以下原因
引起：

- 内存不足（`-ENOMEM`）
- 设备已拔出（`-ENODEV`）
- 端点停滞（`-EPIPE`）
- 排队的 ISO 传输过多（`-EAGAIN`）
- 请求的 ISO 帧过多（`-EFBIG`）
- 无效的 INT 间隔（`-EINVAL`）
- INT 的数据包超过一个（`-EINVAL`）

提交后，`urb->status` 为 `-EINPROGRESS`；但是，除了在你的完成回调中，
你绝不应查看该值。

对于等时端点，你的完成处理函数应使用多缓冲，以 `URB_ISO_ASAP` 标志向
同一端点（重新）提交 URB，以获得无缝的 ISO 流传输。


## 如何取消一个正在运行的 URB？


有两种方法可以取消你已经提交但还未返回给你的驱动的 URB。对于异步取消，
调用
```

	int usb_unlink_urb(struct urb *urb)

```
它会把 urb 从内部列表中移除，并释放所有已分配的硬件描述符。状态会被
修改为反映 unlink。注意 `usb_unlink_urb` 返回时 URB 通常尚未完成；你
必须继续等待完成处理函数被调用。

```

	void usb_kill_urb(struct urb *urb)

```
它完成 `usb_unlink_urb` 所做的所有事情，此外还会等待 URB 已返回且完成
处理函数已执行完毕。它还将 URB 标记为暂时不可用，这样如果完成处理函数
或其他任何代码尝试重新提交它，会得到一个 `-EPERM` 错误。因此你可以确信，
当 `usb_kill_urb` 返回时，该 URB 已完全空闲。

有一个生命周期问题需要考虑。一个 URB 可能在任何时候完成，而完成处理
函数可能会释放该 URB。如果这种情况发生在 `usb_unlink_urb` 或
`usb_kill_urb` 运行时，将导致内存访问违规。驱动有责任避免这种情况，
通常意味着需要某种锁来防止 URB 仍在使用时被释放。

另一方面，由于 usb_unlink_urb 可能最终会调用完成处理函数，该处理函数
不能获取在调用 usb_unlink_urb 时所持有的任何锁。解决此问题的通用方法是
在持有锁时增加 URB 的引用计数，然后释放锁并调用 usb_unlink_urb 或
usb_kill_urb，最后再减少 URB 的引用计数。你增加
```

	struct urb *usb_get_urb(struct urb *urb)

```
（忽略返回值；它与参数相同）并通过调用 `usb_free_urb` 减少引用计数。
当然，如果不存在完成处理函数释放 URB 的危险，上述这些都无需进行。


## 关于完成处理函数？


```

	typedef void (*usb_complete_t)(struct urb *)

```
也就是说，它获得引发完成调用的 URB。在完成处理函数中，你应该查看
`urb->status` 以检测任何 USB 错误。由于 context 参数包含在 URB 中，你
可以向完成处理函数传递信息。

注意，即使报告了错误（或 unlink），数据也可能已经被传输。这是因为 USB
传输是分包的；传输你的 1KByte 缓冲区可能需要十六个包，而在完成被调用
之前，其中十个可能已经成功传输。


   NEVER SLEEP IN A COMPLETION HANDLER.

   这些函数经常在原子上下文被调用。

在当前内核中，完成处理函数运行时本地中断是关闭的，但未来这一点会改变，
因此不要假设本地 IRQ 在完成处理函数内部总是被禁用。

## 如何进行等时（ISO）传输？


除了批量传输中存在的字段外，对于 ISO，你还需要设置 `urb->interval`
以指明进行传输的频率；通常每帧一次（对于高速设备则是每微帧一次）。实际
使用的间隔将是小于等于你所指定值的一个 2 的幂。你可以使用
`usb_fill_int_urb` 宏来填充大多数 ISO 传输字段。

对于 ISO 传输，你还需要为想要调度的每个包填充一个
`usb_iso_packet_descriptor` 结构，该结构由 `usb_alloc_urb` 分配在
URB 的末尾。

`usb_submit_urb` 调用会把 `urb->interval` 修改为小于等于所请求间隔值
的实际实现间隔值。如果使用了 `URB_ISO_ASAP` 调度，`urb->start_frame`
也会被更新。

对于每一项，你必须指定此帧的数据偏移（基址为 transfer_buffer），以及你
想要写入/期望读取的长度。完成后，actual_length 包含实际传输的长度，
status 包含此帧 ISO 传输的结果状态。允许为不同帧指定不同的长度（例如用于
音频同步/自适应传输速率）。你也可以使用长度 0 来省略一个或多个帧
（striping）。

对于调度，你可以选择自己的起始帧或 `URB_ISO_ASAP`。如前所述，如果你始终
至少保持一个 URB 在队列中，并且你的完成处理函数不断（重新）提交一个更晚
的 URB，你将获得平滑的 ISO 流传输（在 usb 带宽允许的情况下）。

如果你指定自己的起始帧，请确保它比当前帧提前若干帧。如果你要将 ISO 数据
与某个其他事件流同步，可能会需要这种模型。


## 如何启动中断（INT）传输？


中断传输与等时传输类似，是周期性的，发生在 2 的幂（1、2、4 等）个单位
的间隔上。单位对于全速和低速设备是帧，对于高速设备是微帧。你可以使用
`usb_fill_int_urb` 宏来填充 INT 传输字段。

`usb_submit_urb` 调用会把 `urb->interval` 修改为小于等于所请求间隔值
的实际实现间隔值。

在 Linux 2.6 中，与早期版本不同，中断 URB 在完成时不会自动重启。它们在
完成处理函数被调用时结束，就像其他 URB 一样。如果你希望中断 URB 重新启动，
你的完成处理函数必须重新提交它。