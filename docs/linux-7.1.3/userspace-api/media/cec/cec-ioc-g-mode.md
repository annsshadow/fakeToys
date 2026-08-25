

######## ioctls CEC_G_MODE 涓?CEC_S_MODE

CEC_G_MODE, CEC_S_MODE - 获取或设置对 CEC 适配器的独占使用

## 概要

`int ioctl(int fd, CEC_G_MODE, __u32 *argp)`

`int ioctl(int fd, CEC_S_MODE, __u32 *argp)`

## 参数

`fd`
    `open()` 返回的文件描述符
`argp`
    指向 CEC 模式的指针
## 描述

默认情况下，任何文件句柄（filehandle）都可以使用 CEC_TRANSMIT，但为了阻止各个应用程序互相干扰，必须能够获取对 CEC 适配器的独占访问。该 ioctl 将文件句柄设置为 initiator（发起者）follower（跟随者）模式，具体取决于所选择的模式，并且可以是独占的。initiator 是用于发起消息的文件句柄，即它命令其CEC 设备。follower 是接收发往 CEC 适配器的消息并处理它们的文件句柄。同一个文件句柄可以既initiator 又是 follower，也可以由两个不同的文件句柄分别担任这两个角色
当接收到一CEC 消息时，CEC 框架会决定如何处理它。如果这条消息是对早先发出的消息的应答，那么该应答会被送回正在等待它的文件句柄。此外，CEC 框架也会处理它
如果这条消息不是应答，那CEC 框架会先处理它。如果没follower，那么该消息会被直接丢弃，并且如果框架无法处理它，则会向 initiator 发回一feature abort（功能拒绝）。如果有 follower，则该消息会被传递给 follower，follower 将使ioctl CEC_RECEIVE <CEC_RECEIVE> 将这条新消息出队。框架期follower 做出正确的决策
除非 follower 另有要求，否CEC 框架会处理核心消息。follower 可以启用 passthrough（透传）模式。在这种情况下，CEC 框架会将大多数核心消息直接传递过去而不处理它们，follower 必须自行实现这些消息。有些消息是核心始终都会处理的，无论透传模式如何。详cec-core-processing
如果没有 initiator，那么任CEC 文件句柄都可以使ioctl CEC_TRANSMIT <CEC_TRANSMIT>。如果存在一个独占的 initiator，那么只有该 initiator 可以调用 CEC_TRANSMIT。当然，follower 始终可以调用 ioctl CEC_TRANSMIT <CEC_TRANSMIT>
可用initiator 模式有：



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-MODE-NO-INITIATOR`:

      - `CEC_MODE_NO_INITIATOR`
      - 0x0
      - 这不是一initiator，即它不能发CEC 消息，也不能CEC 适配器做任何其他更改    - .. _`CEC-MODE-INITIATOR`:

      - `CEC_MODE_INITIATOR`
      - 0x1
      - 这是一initiator（设备打开时的默认值），它可以发CEC 消息并对 CEC 适配器进行更改，除非存在一个独占的 initiator    - .. _`CEC-MODE-EXCL-INITIATOR`:

      - `CEC_MODE_EXCL_INITIATOR`
      - 0x2
      - 这是一个独占的 initiator，该文件描述符是唯一能够发CEC 消息并对 CEC 适配器进行更改的句柄。如果已经有其他人成为独占的 initiator，那么尝试成为独initiator 将返`EBUSY` 错误码
可用follower 模式有：




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-MODE-NO-FOLLOWER`:

      - `CEC_MODE_NO_FOLLOWER`
      - 0x00
      - 这不是一follower（设备打开时的默认值）    - .. _`CEC-MODE-FOLLOWER`:

      - `CEC_MODE_FOLLOWER`
      - 0x10
      - 这是一follower，它会接CEC 消息，除非存在一个独占的 follower。如果未设置 CEC_CAP_TRANSMIT <CEC-CAP-TRANSMIT>，或者指定了 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR>，则不能成为 follower，这种情况下会返`EINVAL` 错误码    - .. _`CEC-MODE-EXCL-FOLLOWER`:

      - `CEC_MODE_EXCL_FOLLOWER`
      - 0x20
      - 这是一个独占的 follower，只有该文件描述符会接收 CEC 消息进行处理。如果已经有其他人成为独占的 follower，那么尝试成为独follower 将返`EBUSY` 错误码。如果未设置 CEC_CAP_TRANSMIT <CEC-CAP-TRANSMIT>，或者指定了 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR>，则不能成为 follower，这种情况下会返`EINVAL` 错误码    - .. _`CEC-MODE-EXCL-FOLLOWER-PASSTHRU`:

      - `CEC_MODE_EXCL_FOLLOWER_PASSTHRU`
      - 0x30
      - 这是一个独占的 follower，只有该文件描述符会接收 CEC 消息进行处理。此外，它会CEC 设备置于 passthrough 模式，从而允许独follower 来处理大多数核心消息，而不必依CEC 框架。如果已经有其他人成为独follower，那么尝试成为独follower 将返`EBUSY` 错误码。如果未设置 CEC_CAP_TRANSMIT <CEC-CAP-TRANSMIT>，或者指定了 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR>，则不能成为 follower，这种情况下会返`EINVAL` 错误码    - .. _`CEC-MODE-MONITOR-PIN`:

      - `CEC_MODE_MONITOR_PIN`
      - 0xd0
      - 将文件描述符置于引脚监视模式。只能与 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR> 组合使用，否则会返回 `EINVAL` 错误码。该模式要求设置CEC_CAP_MONITOR_PIN <CEC-CAP-MONITOR-PIN> 能力，否则会返回 `EINVAL` 错误码。在引脚监视模式下，该文件描述符可以接收 `CEC_EVENT_PIN_CEC_LOW` `CEC_EVENT_PIN_CEC_HIGH` 事件，以观察底层CEC 引脚状态变化。这对于调试非常有用。该模式仅在进程拥有 `CAP_NET_ADMIN` 能力时才被允许。如果未设置该能力，则返`EPERM` 错误码    - .. _`CEC-MODE-MONITOR`:

      - `CEC_MODE_MONITOR`
      - 0xe0
      - 将文件描述符置于监视模式。只能与 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR> 组合使用，否则会返回 `EINVAL` 错误码。在监视模式下，CEC 设备发送的所有消息以及它接收的所有消息（包括广播消息以及发往其某个逻辑地址的定向消息）都会被报告。这对于调试非常有用。该模式仅在进程拥有 `CAP_NET_ADMIN` 能力时才被允许。如果未设置该能力，则返`EPERM` 错误码    - .. _`CEC-MODE-MONITOR-ALL`:

      - `CEC_MODE_MONITOR_ALL`
      - 0xf0
      - 将文件描述符置于“监视全部”模式。只能与 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR> 组合使用，否则会返回 `EINVAL` 错误码。在“监视全部”模式下，该 CEC 设备发送的所有消息以及它接收的所有消息，包括发给其他 CEC 设备的定向消息，都会被报告。这对于调试非常有用，但并非所有设备都支持此模式。该模式要求设置CEC_CAP_MONITOR_ALL <CEC-CAP-MONITOR-ALL> 能力，否则会返回 `EINVAL` 错误码。该模式仅在进程拥有 `CAP_NET_ADMIN` 能力时才被允许。如果未设置该能力，则返`EPERM` 错误码
核心消息处理细节


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 8

    - .. _`CEC-MSG-GET-CEC-VERSION`:

      - `CEC_MSG_GET_CEC_VERSION`
      - 核心会返回通过 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 设置CEC 版本，透传模式除外。在透传模式下，核心不做任何处理，该消息必须follower 来处理    - .. _`CEC-MSG-GIVE-DEVICE-VENDOR-ID`:

      - `CEC_MSG_GIVE_DEVICE_VENDOR_ID`
      - 核心会返回通过 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 设置的厂ID，透传模式除外。在透传模式下，核心不做任何处理，该消息必须follower 来处理    - .. _`CEC-MSG-ABORT`:

      - `CEC_MSG_ABORT`
      - 按照规范，核心会返回一reason 为“Feature Refused”（功能被拒绝）Feature Abort 消息，透传模式除外。在透传模式下，核心不做任何处理，该消息必须follower 来处理    - .. _`CEC-MSG-GIVE-PHYSICAL-ADDR`:

      - `CEC_MSG_GIVE_PHYSICAL_ADDR`
      - 核心会报告当前的物理地址，透传模式除外。在透传模式下，核心不做任何处理，该消息必须follower 来处理    - .. _`CEC-MSG-GIVE-OSD-NAME`:

      - `CEC_MSG_GIVE_OSD_NAME`
      - 核心会报告通过 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 设置的当OSD 名称，透传模式除外。在透传模式下，核心不做任何处理，该消息必须follower 来处理    - .. _`CEC-MSG-GIVE-FEATURES`:

      - `CEC_MSG_GIVE_FEATURES`
      - 如果 CEC 版本低于 2.0，核心不做任何处理；否则它会报告通过 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 设置的当前特性，透传模式除外。在透传模式下，核心不做任何处理（对于任CEC 版本），该消息必须由 follower 来处理    - .. _`CEC-MSG-USER-CONTROL-PRESSED`:

      - `CEC_MSG_USER_CONTROL_PRESSED`
      - 如果设置CEC_CAP_RC <CEC-CAP-RC>，并且设置了 CEC_LOG_ADDRS_FL_ALLOW_RC_PASSTHRU <CEC-LOG-ADDRS-FL-ALLOW-RC-PASSTHRU>，则生成一个遥控按键按下事件。该消息始终会被传递给 follower(s)    - .. _`CEC-MSG-USER-CONTROL-RELEASED`:

      - `CEC_MSG_USER_CONTROL_RELEASED`
      - 如果设置CEC_CAP_RC <CEC-CAP-RC>，并且设置了 CEC_LOG_ADDRS_FL_ALLOW_RC_PASSTHRU <CEC-LOG-ADDRS-FL-ALLOW-RC-PASSTHRU>，则生成一个遥控按键释放事件。该消息始终会被传递给 follower(s)    - .. _`CEC-MSG-REPORT-PHYSICAL-ADDR`:

      - `CEC_MSG_REPORT_PHYSICAL_ADDR`
      - CEC 框架会记录所报告的物理地址，然后直接将消息传递给 follower(s)
## 杩斿洖鍊。
成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 一章中描述
ioctl CEC_S_MODE <CEC_S_MODE> 可以返回以下错误码：

EINVAL
    所请求的模式无效
EPERM
    请求了监视模式，但进程未拥有 `CAP_NET_ADMIN` 能力
EBUSY
    已经有其他进程成为独占的 follower initiator