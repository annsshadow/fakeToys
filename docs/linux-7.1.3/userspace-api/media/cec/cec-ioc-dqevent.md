

######## ioctl CEC_DQEVENT


## Name


CEC_DQEVENT - 出队（Dequeue）一个 CEC 事件

## Synopsis



`int ioctl(int fd, CEC_DQEVENT, struct cec_event *argp)`

## Arguments


`fd`
    `open()` 返回的文件描述符。

`argp`

## Description


CEC 设备可以发送异步事件。可通过调用 `CEC_DQEVENT` 来检索这些事件。如果文件描述符处于非阻塞模式且没有挂起事件，则返回 -1 并将 errno 设置为 `EAGAIN` 错误码。

内部事件队列是按文件句柄（filehandle）和事件类型分别维护的。如果队列已满，则最后一个事件会被新事件覆盖。这意味着中间结果可能被丢弃，但最新事件始终可用。这也意味着有可能读到两个具有相同值的连续事件（例如两个 CEC_EVENT_STATE_CHANGE <CEC-EVENT-STATE-CHANGE> 事件，其状态相同）。在这种情况下，中间的状态变化会丢失，但可以保证两次事件之间的状态确实发生过变化。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 8

    - - __u16
      - `phys_addr`
      - The current physical address. This is `CEC_PHYS_ADDR_INVALID` if no
        valid physical address is set.
    - - __u16
      - `log_addr_mask`
      - The current set of claimed logical addresses. This is 0 if no logical
        addresses are claimed or if `phys_addr` is `CEC_PHYS_ADDR_INVALID`.
	If bit 15 is set (`1 << CEC_LOG_ADDR_UNREGISTERED`) then this device
	has the unregistered logical address. In that case all other bits are 0.
    - - __u16
      - `have_conn_info`
      - If non-zero, then HDMI connector information is available.
        This field is only valid if `CEC_CAP_CONNECTOR_INFO` is set. If that
        capability is set and `have_conn_info` is zero, then that indicates
        that the HDMI connector device is not instantiated, either because
        the HDMI driver is still configuring the device or because the HDMI
        device was unbound.



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 16

    - - __u32
      - `lost_msgs`
      - Set to the number of lost messages since the filehandle was opened
	or since the last time this event was dequeued for this
	filehandle. The messages lost are the oldest messages. So when a
	new message arrives and there is no more room, then the oldest
	message is discarded to make room for the new one. The internal
	size of the message queue guarantees that all messages received in
	the last two seconds will be stored. Since messages should be
	replied to within a second according to the CEC specification,
	this is more than enough.



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 8

    - - __u64
      - `ts`
      - 事件的时间戳，单位为 ns。

	该时间戳取自 `CLOCK_MONOTONIC` 时钟。

	若要在用户空间访问同一时钟，可使用 `clock_gettime`。
    - - __u32
      - `event`
      - CEC 事件类型，参见 cec-events。
    - - __u32
      - `flags`
      - 事件标志，参见 cec-event-flags。
    - - union {
      - (anonymous)
    - - struct cec_event_state_change
      - `state_change`
      - 由 CEC_EVENT_STATE_CHANGE <CEC-EVENT-STATE-CHANGE> 事件
	发送的新的适配器状态。
    - - struct cec_event_lost_msgs
      - `lost_msgs`
      - 由 CEC_EVENT_LOST_MSGS <CEC-EVENT-LOST-MSGS> 事件
	发送的丢失消息数量。
    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-EVENT-STATE-CHANGE`:

      - `CEC_EVENT_STATE_CHANGE`
      - 1
      - 当 CEC 适配器状态发生变化时生成。调用 open() 时会为该文件句柄
	生成一条初始事件，反映当时 CEC 适配器的状态。
    - .. _`CEC-EVENT-LOST-MSGS`:

      - `CEC_EVENT_LOST_MSGS`
      - 2
      - 如果由于应用程序未能及时出队 CEC 消息而导致一条或多条
	CEC 消息丢失，则生成该事件。
    - .. _`CEC-EVENT-PIN-CEC-LOW`:

      - `CEC_EVENT_PIN_CEC_LOW`
      - 3
      - 当 CEC 引脚从高电压变为低电压时生成。仅适用于设置了
	`CEC_CAP_MONITOR_PIN` 能力的适配器。
    - .. _`CEC-EVENT-PIN-CEC-HIGH`:

      - `CEC_EVENT_PIN_CEC_HIGH`
      - 4
      - 当 CEC 引脚从低电压变为高电压时生成。仅适用于设置了
	`CEC_CAP_MONITOR_PIN` 能力的适配器。
    - .. _`CEC-EVENT-PIN-HPD-LOW`:

      - `CEC_EVENT_PIN_HPD_LOW`
      - 5
      - 当 HPD 引脚从高电压变为低电压时生成。仅适用于设置了
	`CEC_CAP_MONITOR_PIN` 能力的适配器。调用 open() 时可读取 HPD
	引脚，若 HPD 为低电平，则将为该文件句柄生成一条初始事件。
    - .. _`CEC-EVENT-PIN-HPD-HIGH`:

      - `CEC_EVENT_PIN_HPD_HIGH`
      - 6
      - 当 HPD 引脚从低电压变为高电压时生成。仅适用于设置了
	`CEC_CAP_MONITOR_PIN` 能力的适配器。调用 open() 时可读取 HPD
	引脚，若 HPD 为高电平，则将为该文件句柄生成一条初始事件。
    - .. _`CEC-EVENT-PIN-5V-LOW`:

      - `CEC_EVENT_PIN_5V_LOW`
      - 6
      - 当 5V 引脚从高电压变为低电压时生成。仅适用于设置了
	`CEC_CAP_MONITOR_PIN` 能力的适配器。调用 open() 时可读取 5V
	引脚，若 5V 为低电平，则将为该文件句柄生成一条初始事件。
    - .. _`CEC-EVENT-PIN-5V-HIGH`:

      - `CEC_EVENT_PIN_5V_HIGH`
      - 7
      - 当 5V 引脚从低电压变为高电压时生成。仅适用于设置了
	`CEC_CAP_MONITOR_PIN` 能力的适配器。调用 open() 时可读取 5V
	引脚，若 5V 为高电平，则将为该文件句柄生成一条初始事件。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 8

    - .. _`CEC-EVENT-FL-INITIAL-STATE`:

      - `CEC_EVENT_FL_INITIAL_STATE`
      - 1
      - 针对设备打开时生成的初始事件设置。哪些事件会这样做，参见上表。
	这样应用程序可以在 open() 时了解到 CEC 适配器的初始状态。
    - .. _`CEC-EVENT-FL-DROPPED-EVENTS`:

      - `CEC_EVENT_FL_DROPPED_EVENTS`
      - 2
      - 如果给定事件类型的一个或多个事件已被丢弃，则设置该标志。
	这表明应用程序无法跟上处理速度。


## Return Value


成功时返回 0，出错时返回 -1 并适当地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述。

ioctl CEC_DQEVENT <CEC_DQEVENT> 可能返回以下错误码：

EAGAIN
    当文件句柄处于非阻塞模式且没有挂起事件时返回。

ERESTARTSYS
    在阻塞模式下等待事件到达时，收到了一个中断（例如 Ctrl-C）。
