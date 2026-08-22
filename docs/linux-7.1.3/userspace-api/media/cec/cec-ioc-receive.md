


######## ioctls CEC_RECEIVE 鍜?CEC_TRANSMIT


## 名称


CEC_RECEIVE、CEC_TRANSMIT - 接收或发送一CEC 消息

## 概要



`int ioctl(int fd, CEC_RECEIVE, struct cec_msg *argp)`


`int ioctl(int fd, CEC_TRANSMIT, struct cec_msg *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct cec_msg 的指针
## 描述


要接收一CEC 消息，应用程序必须填struct `cec_msg` `timeout` 字段，并将其传给 ioctl CEC_RECEIVE <CEC_RECEIVE>。如果文件描述符处于非阻塞模式，且没有待接收的消息，那么它会返回 -1 并将 errno 设为 `EAGAIN` 错误码。如果文件描述符处于阻塞模式，且 `timeout` 非零，而在 `timeout` 毫秒内没有消息到达，那么它会返回 -1 并将 errno 设为 `ETIMEDOUT` 错误码
一条接收到的消息可以是
1. 从另一CEC 设备接收到的消息（`sequence` 字段0，`tx_status` 0，`rx_status` 非零）2. 之前一次非阻塞发送的发送结果（`sequence` 字段非零，`tx_status` 非零，`rx_status` 0）3. 之前一次非阻塞发送的应答（reply）（`sequence` 字段非零，`tx_status` 0，`rx_status` 非零）
要发送一CEC 消息，应用程序必须填struct `cec_msg` 并将其传ioctl CEC_TRANSMIT <CEC_TRANSMIT>。ioctl CEC_TRANSMIT <CEC_TRANSMIT> 仅在设置`CEC_CAP_TRANSMIT` 时才可用。如果发送队列中没有更多空间，那么它会返-1 并将 errno 设为 `EBUSY` 错误码。发送队列有足够的空间容18 条消息（大约相当1 秒的 2 字节消息）。注意，CEC 内核框架也会计（reply）核心消息（参见 cec-core-processing），因此将发送队列完全填满并不是个好主意
如果文件描述符处于非阻塞模式，那么发送会返回 0，并且在发送完成后，发送的结果可通过 ioctl CEC_RECEIVE <CEC_RECEIVE> 获得。如果一次非阻塞发送还指定了等待应答（reply），那么应答会在一个后续消息中到达。`sequence` 字段可用于将发送结果和应答与原始发送相关联
通常，当物理地址无效时（例如由于断开连接）调ioctl CEC_TRANSMIT <CEC_TRANSMIT> 会返`ENONET`
然而，CEC 规范允许在物理地址无效时，'Unregistered' 'TV' 发送消息，因为某些电视在进入待机状态或切换到另一个输入时，会HDMI 连接器的热插拔检测（hotplug detect）引脚拉低
当热插拔检测引脚变低时，EDID 消失，从而物理地址也消失，但线缆仍然连接，CEC 仍然工作。为了检唤醒设备，允许从发起0xfUnregistered'）向目标 0TV'）发送轮询（poll）和 'Image/Text View On' 消息



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 16

    - - __u64
      - `tx_ts`
      - 消息最后一个字节被发送时的时间戳，单位为 ns	该时间戳取自 `CLOCK_MONOTONIC` 时钟。要从用户空间访问同一个时钟，
	请使`clock_gettime`    - - __u64
      - `rx_ts`
      - 消息最后一个字节被接收时的时间戳，单位ns	该时间戳取自 `CLOCK_MONOTONIC` 时钟。要从用户空间访问同一个时钟，
	请使`clock_gettime`    - - __u32
      - `len`
      - 消息的长度。对ioctl CEC_TRANSMIT <CEC_TRANSMIT>	这由应用程序填入。驱动会ioctl CEC_RECEIVE <CEC_RECEIVE> 填入此字段	对于 ioctl CEC_TRANSMIT <CEC_TRANSMIT>，如果设置了 `reply`	驱动会将其填入为应答消息的长度    - - __u32
      - `timeout`
      - 超时时间，单位为毫秒。这是设备在超时之前等待接收一条消息的时间	如果设为 0，那么当它由 ioctl CEC_RECEIVE <CEC_RECEIVE> 调用时，
	将无限期等待。如果它0 且由 ioctl CEC_TRANSMIT <CEC_TRANSMIT> 调用	那么`reply` 非零时它会被替换1000，或者当 `reply` 0 时被忽略    - - __u32
      - `sequence`
      - 一个非零的序列号，CEC 框架为所有已发送的消息自动分配	CEC 框架为非阻塞发送排队发送结果时，会用到它	这使得应用程序能够将接收到的消息与原始发送相关联
	此外，如果一次非阻塞发送会等待应答（即 `timeout` 不为 0），
	那么应答`sequence` 字段会被设为原始发送的序列值	这使得应用程序能够将接收到的消息与原始发送相关联    - - __u32
      - `flags`
      - 标志位。可用标志列表参cec-msg-flags    - - __u8
      - `msg[^16^]`
      - 消息有效载荷。对ioctl CEC_TRANSMIT <CEC_TRANSMIT>	这由应用程序填入。驱动会ioctl CEC_RECEIVE <CEC_RECEIVE> 填入此字段	对于 ioctl CEC_TRANSMIT <CEC_TRANSMIT>，如果设置了 `timeout`	驱动会将其填入为应答消息的有效载荷    - - __u8
      - `reply`
      - 等待此消息被应答。如`reply` 0 `timeout` 0	则不等待应答，而是在发送消息后返回。ioctl CEC_RECEIVE <CEC_RECEIVE> 会忽略它	`reply` 0（这Feature Abort 消息的操作码）且 `timeout` 非零的情	被特意允许，以便能够发送一条消息并等待最`timeout` 毫秒以收到一	Feature Abort 应答。在这种情况下，`rx_status` 会被设为
	CEC_RX_STATUS_TIMEOUT <CEC-RX-STATUS-TIMEOUT> 鎴?	CEC_RX_STATUS_FEATURE_ABORT <CEC-RX-STATUS-FEATURE-ABORT>銆。
	如果发送方消息`CEC_MSG_INITIATE_ARC`，那`reply` 	`CEC_MSG_REPORT_ARC_INITIATED` `CEC_MSG_REPORT_ARC_TERMINATED`
	会被区别处理：任一值都能匹配两种可能的应答	原因`CEC_MSG_INITIATE_ARC` 消息是唯一一条除 Feature Abort 	还有两种可能应答CEC 消息。`reply` 字段会被更新为实际的应答	以便与所接收消息的内容保持同步    - - __u8
      - `rx_status`
      - 所接收消息的状态位。可能的状态值参cec-rx-status    - - __u8
      - `tx_status`
      - 所发送消息的状态位。可能的状态值参cec-tx-status	当以非阻塞模式调ioctl CEC_TRANSMIT <CEC_TRANSMIT> 时，
	如果发送已开始，此字段为 0；如果发送结果立即可知，则为0	后一种情况发生在尝试向自己发Poll 消息时。这会导致一	CEC_TX_STATUS_NACK <CEC-TX-STATUS-NACK>，而实际上从未发送该 Poll 消息    - - __u8
      - `tx_arb_lost_cnt`
      - 导致仲裁丢失（Arbitration Lost）错误的发送尝试计数	仅在硬件支持此功能时设置，否则始终为 0	此计数器仅在设置CEC_TX_STATUS_ARB_LOST <CEC-TX-STATUS-ARB-LOST>
	状态位时有效    - - __u8
      - `tx_nack_cnt`
      - 导致未确认（Not Acknowledged）错误的发送尝试计数	仅在硬件支持此功能时设置，否则始终为 0	此计数器仅在设置CEC_TX_STATUS_NACK <CEC-TX-STATUS-NACK>
	状态位时有效    - - __u8
      - `tx_low_drive_cnt`
      - 导致仲裁丢失（Arbitration Lost）错误的发送尝试计数	仅在硬件支持此功能时设置，否则始终为 0	此计数器仅在设置CEC_TX_STATUS_LOW_DRIVE <CEC-TX-STATUS-LOW-DRIVE>
	状态位时有效    - - __u8
      - `tx_error_cnt`
      - 除仲裁丢失或未确认之外的发送错误计数	仅在硬件支持此功能时设置，否则始终为 0	此计数器仅在设置CEC_TX_STATUS_ERROR <CEC-TX-STATUS-ERROR>
	状态位时有效



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-MSG-FL-REPLY-TO-FOLLOWERS`:

      - `CEC_MSG_FL_REPLY_TO_FOLLOWERS`
      - 1
      - 如果一CEC 发送期望一个应答，那么默认情况下该应答只发送给
	调用ioctl CEC_TRANSMIT <CEC_TRANSMIT> 的文件句柄（filehandle）	如果设置了此标志，那么应答也会发送给所follower（如果有的话）	如果调用ioctl CEC_TRANSMIT <CEC_TRANSMIT> 的文件句柄本身也是一	follower，那么该文件句柄会收到两次应答：一次作ioctl CEC_TRANSMIT <CEC_TRANSMIT>
	的结果，一次通过 ioctl CEC_RECEIVE <CEC_RECEIVE>
    - .. _`CEC-MSG-FL-RAW`:

      - `CEC_MSG_FL_RAW`
      - 2
      - 通常 CEC 消息在发送前会经过校验。如果调ioctl CEC_TRANSMIT <CEC_TRANSMIT>
	时设置了此标志，则不进行任何校验，消息按原样发送。这在调CEC 问题时很有用	此标志仅在进程具`CAP_SYS_RAWIO` 能力（capability）时才允许使用	如果未设置，则返`EPERM` 错误码
    - .. _`CEC-MSG-FL-REPLY-VENDOR-ID`:

      - `CEC_MSG_FL_REPLY_VENDOR_ID`
      - 4
      - 此标志仅在设置了 `CEC_CAP_REPLY_VENDOR_ID` 能力时可用	如果设置了此标志，则期望应答`CEC_MSG_VENDOR_COMMAND_WITH_ID` 操作	后跟厂商 ID（消息的1-4 字节），再后struct cec_msg `reply` 字段组成
	注意，这假设厂商 ID 之后的字节是一个厂商特定的操作码
	此标志使得等待厂商命令的应答变得更加容易


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-TX-STATUS-OK`:

      - `CEC_TX_STATUS_OK`
      - 0x01
      - 消息发送成功。这CEC_TX_STATUS_MAX_RETRIES <CEC-TX-STATUS-MAX-RETRIES>
	互斥。如果早期尝试在最终发送成功之前遭遇失败，其他位仍可被设置    - .. _`CEC-TX-STATUS-ARB-LOST`:

      - `CEC_TX_STATUS_ARB_LOST`
      - 0x02
      - CEC 线路仲裁丢失，即另一个发送在同时以更高优先级开始	可选状态，并非所有硬件都能检测到此错误条件    - .. _`CEC-TX-STATUS-NACK`:

      - `CEC_TX_STATUS_NACK`
      - 0x04
      - 消息未被确认。注意某些硬件无法区未确状态与其他错误条件	即发送结果只OK FAIL。在这种情况下，发送失败时会返回此状态    - .. _`CEC-TX-STATUS-LOW-DRIVE`:

      - `CEC_TX_STATUS_LOW_DRIVE`
      - 0x08
      - CEC 总线上检测到低驱动（low drive）。这表示某个 follower
	检测到总线上的错误并请求重传。可选状态，并非所有硬件都能检测到此错误条件    - .. _`CEC-TX-STATUS-ERROR`:

      - `CEC_TX_STATUS_ERROR`
      - 0x10
      - 发生了某些错误。这用于任何不适合 `CEC_TX_STATUS_ARB_LOST`
	`CEC_TX_STATUS_LOW_DRIVE` 的错误，可能是因为硬件无法判断发生了哪个错误	或者硬件测试了除这两者之外的其他条件。可选状态    - .. _`CEC-TX-STATUS-MAX-RETRIES`:

      - `CEC_TX_STATUS_MAX_RETRIES`
      - 0x20
      - 在重试一次或多次后发送仍然失败。此状态位CEC_TX_STATUS_OK <CEC-TX-STATUS-OK>
	互斥。其他位仍可被设置，以说明看到了哪些失败    - .. _`CEC-TX-STATUS-ABORTED`:

      - `CEC_TX_STATUS_ABORTED`
      - 0x40
      - 发送因 HDMI 断开连接、或适配器被取消配置（unconfigured）	或一次发送被中断、或驱动在尝试开始一次发送时返回错误而被中止    - .. _`CEC-TX-STATUS-TIMEOUT`:

      - `CEC_TX_STATUS_TIMEOUT`
      - 0x80
      - 发送超时。这通常不应发生，表明存在驱动问题


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-RX-STATUS-OK`:

      - `CEC_RX_STATUS_OK`
      - 0x01
      - 消息接收成功    - .. _`CEC-RX-STATUS-TIMEOUT`:

      - `CEC_RX_STATUS_TIMEOUT`
      - 0x02
      - 对一条较早发送消息的应答超时    - .. _`CEC-RX-STATUS-FEATURE-ABORT`:

      - `CEC_RX_STATUS_FEATURE_ABORT`
      - 0x04
      - 消息接收成功，但应答`CEC_MSG_FEATURE_ABORT`	此状态仅在消息是对一条较早发送消息的应答时才被设置    - .. _`CEC-RX-STATUS-ABORTED`:

      - `CEC_RX_STATUS_ABORTED`
      - 0x08
      - 等待一条较早发送消息的应答被中止，原因HDMI 线缆被断开	适配器被取消配置，或等待应答CEC_TRANSMIT <CEC_RECEIVE> 被中断
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
通用错误<gen-errors> 一章中描述
ioctl CEC_RECEIVE <CEC_RECEIVE> 可以返回以下错误码：

EAGAIN
    接收队列中没有消息，且文件句柄处于非阻塞模式
ETIMEDOUT
    等待消息时达到了 `timeout`
ERESTARTSYS
    等待消息被中断（例如Ctrl-C 中断）
ioctl CEC_TRANSMIT <CEC_TRANSMIT> 可以返回以下错误码：

ENOTTY
    未设`CEC_CAP_TRANSMIT` 能力，因此不支持ioctl
EPERM
    CEC 适配器未配置，即从未调用ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>    或`CEC_MSG_FL_RAW` 被一个不具有 `CAP_SYS_RAWIO` 能力的进程使用
ENONET
    CEC 适配器未配置，即调用ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>    但物理地址无效，因此没有声明（claim）逻辑地址    此情况下有一个例外，允许从发起0xfUnregistered'）向目标 0TV'）发送    那种情况下发送会照常进行
EBUSY
    另一个文件句柄处于独follower initiator 模式，或者文件句柄处    `CEC_MODE_NO_INITIATOR` 模式。当发送队列已满时也会返回此错误
EINVAL
    struct `cec_msg` 的内容无效
ERESTARTSYS
    等待一次成功发送被中断（例如被 Ctrl-C 中断）