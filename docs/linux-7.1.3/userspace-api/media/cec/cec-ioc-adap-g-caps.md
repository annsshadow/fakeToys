


######## ioctl CEC_ADAP_G_CAPS


## 名称


CEC_ADAP_G_CAPS - 查询设备能力

## 概要


`int ioctl(int fd, CEC_ADAP_G_CAPS, struct cec_caps *argp)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`argp`


## 描述


所有 cec 设备都必须支持 ioctl CEC_ADAP_G_CAPS <CEC_ADAP_G_CAPS>。为查询设备信息，应用程序以指向 struct `cec_caps` 的指针调用该 ioctl。驱动填充该结构并将信息返回给应用程序。该 ioctl 永远不会失败。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 16

    - - char
      - `driver[^32^]`
      - cec 适配器驱动的名称。
    - - char
      - `name[^32^]`
      - 此 CEC 适配器的名称。`driver` 与 `name` 的组合必须唯一。
    - - __u32
      - `available_log_addrs`
      - 可配置的逻辑地址最大数量。
    - - __u32
      - `capabilities`
      - CEC 适配器的能力，参见 cec-capabilities。
    - - __u32
      - `version`
      - CEC 框架 API 版本，使用 `KERNEL_VERSION()` 宏格式化。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 8

    - .. _`CEC-CAP-PHYS-ADDR`:

      - `CEC_CAP_PHYS_ADDR`
      - 0x00000001
      - 用户空间必须通过调用 ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 配置物理地址。如果未设置此能力，则物理地址的设置在 EDID 被设置（对 HDMI 接收器）或读取（对 HDMI 发送器）时由内核处理。
    - .. _`CEC-CAP-LOG-ADDRS`:

      - `CEC_CAP_LOG_ADDRS`
      - 0x00000002
      - 用户空间必须通过调用 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 配置逻辑地址。如果未设置此能力，则由内核完成配置。
    - .. _`CEC-CAP-TRANSMIT`:

      - `CEC_CAP_TRANSMIT`
      - 0x00000004
      - 用户空间可以通过调用 ioctl CEC_TRANSMIT <CEC_TRANSMIT> 发送 CEC 消息。这意味着用户空间也可以成为 follower，因为能够发送消息是成为 follower 的前提。如果未设置此能力，则由内核处理所有 CEC 发送并处理它收到的所有 CEC 消息。
    - .. _`CEC-CAP-PASSTHROUGH`:

      - `CEC_CAP_PASSTHROUGH`
      - 0x00000008
      - 用户空间可以通过调用 ioctl CEC_S_MODE <CEC_S_MODE> 使用直通（passthrough）模式。
    - .. _`CEC-CAP-RC`:

      - `CEC_CAP_RC`
      - 0x00000010
      - 此适配器支持遥控（remote control）协议。
    - .. _`CEC-CAP-MONITOR-ALL`:

      - `CEC_CAP_MONITOR_ALL`
      - 0x00000020
      - CEC 硬件可以监控所有消息，而不仅仅是定向和广播消息。
    - .. _`CEC-CAP-NEEDS-HPD`:

      - `CEC_CAP_NEEDS_HPD`
      - 0x00000040
      - CEC 硬件仅在 HDMI Hotplug Detect 引脚为高电平时才处于活动状态。这使得无法使用 CEC 唤醒在待机模式下将 HPD 引脚置低、但保持 CEC 总线存活的显示器。
    - .. _`CEC-CAP-MONITOR-PIN`:

      - `CEC_CAP_MONITOR_PIN`
      - 0x00000080
      - CEC 硬件可以监控 CEC 引脚从低电压到高电压的变化及其反向变化。在引脚监控模式下，应用程序将收到 `CEC_EVENT_PIN_CEC_LOW` 和 `CEC_EVENT_PIN_CEC_HIGH` 事件。
    - .. _`CEC-CAP-CONNECTOR-INFO`:

      - `CEC_CAP_CONNECTOR_INFO`
      - 0x00000100
      - 如果设置了此能力，则可以使用 CEC_ADAP_G_CONNECTOR_INFO。
    - .. _`CEC-CAP-REPLY-VENDOR-ID`:

      - `CEC_CAP_REPLY_VENDOR_ID`
      - 0x00000200
      - 如果设置了此能力，则可以使用 CEC_MSG_FL_REPLY_VENDOR_ID <cec-msg-flags>。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
