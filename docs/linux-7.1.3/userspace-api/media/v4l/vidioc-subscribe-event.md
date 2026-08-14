


######## ioctl VIDIOC_SUBSCRIBE_EVENT, VIDIOC_UNSUBSCRIBE_EVENT


## 名称


VIDIOC_SUBSCRIBE_EVENT - VIDIOC_UNSUBSCRIBE_EVENT - 订阅或取消订阅事件

## 概要



`int ioctl(int fd, VIDIOC_SUBSCRIBE_EVENT, struct v4l2_event_subscription *argp)`


`int ioctl(int fd, VIDIOC_UNSUBSCRIBE_EVENT, struct v4l2_event_subscription *argp)`

## 参数



`fd`
    由 `open()` 返回的文件描述符。

`argp`
    指向结构体 `v4l2_event_subscription` 的指针。

## 描述


订阅或取消订阅 V4L2 事件。已订阅的事件通过 VIDIOC_DQEVENT ioctl 出队。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `type`
      - 事件的类型，参见 event-type。
```
	   ``V4L2_EVENT_ALL`` 可与
	   :ref:`VIDIOC_UNSUBSCRIBE_EVENT <VIDIOC_SUBSCRIBE_EVENT>` 一起使用，
	   用于一次性取消订阅所有事件。
    * - __u32
      - ``id``
      - 事件源的 ID。如果事件源没有关联的 ID，则将其设为 0。事件是否需要 ID
	取决于事件类型。
    * - __u32
      - ``flags``
      - 事件标志，参见 :ref:`event-flags`。
    * - __u32
      - ``reserved``\ [5]
      - 保留以备将来扩展。驱动和应用程序都必须将该数组置零。


```

    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_EVENT_SUB_FL_SEND_INITIAL`
      - 0x0001
      - 当订阅此事件时，会发送一个包含当前状态的初始事件。这仅对由状态变化触发的事件
	（如 `V4L2_EVENT_CTRL`）有意义。其他事件会忽略此标志。
    - - `V4L2_EVENT_SUB_FL_ALLOW_FEEDBACK`
      - 0x0002
      - 若设置，则直接由 ioctl 引起的事件也会发送给调用该 ioctl 的文件句柄。例如，
	使用 VIDIOC_S_CTRL <VIDIOC_G_CTRL> 改变一个控件会导致一个 V4L2_EVENT_CTRL 被
	发送回同一个文件句柄。
	通常此类事件会被抑制，以防止反馈环路：一个应用程序将某个控件改为一个值，
	然后又改为另一个值，接着收到一个事件告诉它该控件已变回第一个值。

	由于它无法判断该事件是由另一个应用程序引起的，还是由 VIDIOC_S_CTRL <VIDIOC_G_CTRL>
	调用引起的，因此很难决定是将控件设为事件中的值，还是忽略它。

	设置此标志时请仔细考虑，以免陷入此类情形。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在通用错误码 <gen-errors> 一章中描述。
