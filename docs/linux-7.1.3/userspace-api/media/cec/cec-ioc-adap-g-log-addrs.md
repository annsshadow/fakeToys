


######## ioctl CEC_ADAP_G_LOG_ADDRS 与 CEC_ADAP_S_LOG_ADDRS


## 名称


CEC_ADAP_G_LOG_ADDRS、CEC_ADAP_S_LOG_ADDRS - 获取或设置逻辑地址

## 概要


`int ioctl(int fd, CEC_ADAP_G_LOG_ADDRS, struct cec_log_addrs *argp)`


`int ioctl(int fd, CEC_ADAP_S_LOG_ADDRS, struct cec_log_addrs *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符。

`argp`
    指向 struct `cec_log_addrs` 的指针。

## 描述


要查询当前的 CEC 逻辑地址，应用程序以指向 struct `cec_log_addrs` 的指针调用
ioctl CEC_ADAP_G_LOG_ADDRS <CEC_ADAP_G_LOG_ADDRS>，驱动在其中存储逻辑地址。

要设置新的逻辑地址，应用程序填写 struct `cec_log_addrs` 并以指向此结构的指针调用 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>。ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>
仅在设置了 `CEC_CAP_LOG_ADDRS` 时可用（否则返回 `ENOTTY` 错误码）。ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS>
只能由处于发起者模式的文件描述符调用（见 CEC_S_MODE），否则将返回 `EBUSY` 错误码。

要清除现有的逻辑地址，将 `num_log_addrs` 设为 0。此时所有其他字段都将被忽略。适配器将进入未配置状态，且
`cec_version`、`vendor_id` 和 `osd_name` 字段都被重置为其默认值（CEC 版本 2.0、无厂商 ID 和空的 OSD 名称）。

如果物理地址有效（见 ioctl CEC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR>），则此 ioctl 会阻塞，直到所有请求的逻辑地址都被认领。如果文件描述符处于非阻塞模式，则它不会等待逻辑地址被认领，而是直接返回 0。

当逻辑地址被认领或清除时，会发送一个 CEC_EVENT_STATE_CHANGE <CEC-EVENT-STATE-CHANGE> 事件。

在逻辑地址类型已定义的情况下尝试调用 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 将返回错误 `EBUSY`。



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 16

    - - __u8
      - `log_addr[CEC_MAX_LOG_ADDRS]`
      - 被认领的实际逻辑地址。由驱动设置。如果无法认领任何逻辑地址，则将其设为
	`CEC_LOG_ADDR_INVALID`。如果此适配器是未注册的，则
	`log_addr[^0^]` 设为 0xf，所有其他地址设为
	`CEC_LOG_ADDR_INVALID`。
    - - __u16
      - `log_addr_mask`
      - 此适配器已认领的所有逻辑地址的位掩码。如果此适配器是未注册的，则 `log_addr_mask` 置位第 15 位
	并清除所有其他位。如果此适配器根本未配置，则 `log_addr_mask` 设为 0。由驱动设置。
    - - __u8
      - `cec_version`
      - 此适配器应当使用的 CEC 版本。见
	cec-versions。用于实现
	`CEC_MSG_CEC_VERSION` 和 `CEC_MSG_REPORT_FEATURES` 消息。
	注意 CEC_OP_CEC_VERSION_1_3A <CEC-OP-CEC-VERSION-1-3A> 不被 CEC 框架允许。
    - - __u8
      - `num_log_addrs`
      - 要设置的 logical 地址数量。必须 ≤
	CEC_ADAP_G_CAPS 返回的
	`available_log_addrs`。此结构中的所有数组只填充到索引
	`available_log_addrs`-1。其余数组元素将被忽略。注意 CEC 2.0 标准允许最多 2 个逻辑地址，尽管某些硬件支持更多。
	`CEC_MAX_LOG_ADDRS` 为 4。驱动将返回它实际能够认领的逻辑地址数量，可能少于所请求的。如果此字段设为 0，则 CEC
	适配器应清除所有已认领的逻辑地址，并且所有其他字段都将被忽略。
    - - __u32
      - `vendor_id`
      - 厂商 ID 是一个 24 位的数字，用于标识特定的厂商或实体。基于此 ID 可以定义厂商特定的命令。如果你不想要厂商 ID，则将其设为
	`CEC_VENDOR_ID_NONE`。
    - - __u32
      - `flags`
      - 标志。可用标志列表见 cec-log-addrs-flags。
    - - char
      - `osd_name[^15^]`
      - 由 `CEC_MSG_SET_OSD_NAME` 消息返回的屏上显示名称。
    - - __u8
      - `primary_device_type[CEC_MAX_LOG_ADDRS]`
      - 每个逻辑地址的主设备类型。可能类型见
	cec-prim-dev-types。
    - - __u8
      - `log_addr_type[CEC_MAX_LOG_ADDRS]`
      - 逻辑地址类型。可能类型见 cec-log-addr-types。
	驱动会用它实际认领的逻辑地址类型更新此字段（例如它可能需要回退到 CEC_LOG_ADDR_TYPE_UNREGISTERED <CEC-LOG-ADDR-TYPE-UNREGISTERED>）。
    - - __u8
      - `all_device_types[CEC_MAX_LOG_ADDRS]`
      - CEC 2.0 特有：所有设备类型的位掩码。见
	cec-all-dev-types-flags。它用于 CEC 2.0 的
	`CEC_MSG_REPORT_FEATURES` 消息。对于 CEC 1.4，你可以将此字段保留为 0，或者按照 CEC 2.0 的指南填写，以向 CEC 框架提供关于设备类型的更多信息，即使框架不会在 CEC 消息中直接使用它。
    - - __u8
      - `features[CEC_MAX_LOG_ADDRS][^12^]`
      - 每个逻辑地址的特性。它用于 CEC 2.0 的
	`CEC_MSG_REPORT_FEATURES` 消息。这 12 个字节同时包含
	RC Profile 和设备特性。对于 CEC 1.4，你可以将此字段保留为全 0，或者按照 CEC 2.0 的指南填写，以向 CEC 框架提供关于设备类型的更多信息，即使框架不会在 CEC 消息中直接使用它。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-LOG-ADDRS-FL-ALLOW-UNREG-FALLBACK`:

      - `CEC_LOG_ADDRS_FL_ALLOW_UNREG_FALLBACK`
      - 1
      - 默认情况下，如果无法认领所请求类型的逻辑地址，则它将回到未配置状态。如果设置了此标志，则它会回退到未注册的逻辑地址。注意，如果显式请求了未注册的逻辑地址，则此标志不起作用。
    - .. _`CEC-LOG-ADDRS-FL-ALLOW-RC-PASSTHRU`:

      - `CEC_LOG_ADDRS_FL_ALLOW_RC_PASSTHRU`
      - 2
      - 默认情况下，`CEC_MSG_USER_CONTROL_PRESSED` 和 `CEC_MSG_USER_CONTROL_RELEASED`
        消息只传递给 follower（如果有）。如果设置了此标志，则这些消息也会传递给远程控制输入子系统，并作为按键出现。此特性需要显式启用。如果 CEC 用于输入密码等，你可能不想启用此特性，以避免对按键的简单嗅探。
    - .. _`CEC-LOG-ADDRS-FL-CDC-ONLY`:

      - `CEC_LOG_ADDRS_FL_CDC_ONLY`
      - 4
      - 如果设置了此标志，则该设备是 CDC-Only（仅 CDC）。CDC-Only 的 CEC 设备是只能处理 CDC 消息的 CEC 设备。

	所有其他消息都被忽略。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-OP-CEC-VERSION-1-3A`:

      - `CEC_OP_CEC_VERSION_1_3A`
      - 4
      - 根据 HDMI 1.3a 标准的 CEC 版本。
    - .. _`CEC-OP-CEC-VERSION-1-4B`:

      - `CEC_OP_CEC_VERSION_1_4B`
      - 5
      - 根据 HDMI 1.4b 标准的 CEC 版本。
    - .. _`CEC-OP-CEC-VERSION-2-0`:

      - `CEC_OP_CEC_VERSION_2_0`
      - 6
      - 根据 HDMI 2.0 标准的 CEC 版本。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-OP-PRIM-DEVTYPE-TV`:

      - `CEC_OP_PRIM_DEVTYPE_TV`
      - 0
      - 用于电视。
    - .. _`CEC-OP-PRIM-DEVTYPE-RECORD`:

      - `CEC_OP_PRIM_DEVTYPE_RECORD`
      - 1
      - 用于录像设备。
    - .. _`CEC-OP-PRIM-DEVTYPE-TUNER`:

      - `CEC_OP_PRIM_DEVTYPE_TUNER`
      - 3
      - 用于带调谐器的设备。
    - .. _`CEC-OP-PRIM-DEVTYPE-PLAYBACK`:

      - `CEC_OP_PRIM_DEVTYPE_PLAYBACK`
      - 4
      - 用于播放设备。
    - .. _`CEC-OP-PRIM-DEVTYPE-AUDIOSYSTEM`:

      - `CEC_OP_PRIM_DEVTYPE_AUDIOSYSTEM`
      - 5
      - 用于音频系统（例如音频/视频接收器）。
    - .. _`CEC-OP-PRIM-DEVTYPE-SWITCH`:

      - `CEC_OP_PRIM_DEVTYPE_SWITCH`
      - 6
      - 用于 CEC 开关。
    - .. _`CEC-OP-PRIM-DEVTYPE-VIDEOPROC`:

      - `CEC_OP_PRIM_DEVTYPE_VIDEOPROC`
      - 7
      - 用于视频处理器设备。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-LOG-ADDR-TYPE-TV`:

      - `CEC_LOG_ADDR_TYPE_TV`
      - 0
      - 用于电视。
    - .. _`CEC-LOG-ADDR-TYPE-RECORD`:

      - `CEC_LOG_ADDR_TYPE_RECORD`
      - 1
      - 用于录像设备。
    - .. _`CEC-LOG-ADDR-TYPE-TUNER`:

      - `CEC_LOG_ADDR_TYPE_TUNER`
      - 2
      - 用于调谐器设备。
    - .. _`CEC-LOG-ADDR-TYPE-PLAYBACK`:

      - `CEC_LOG_ADDR_TYPE_PLAYBACK`
      - 3
      - 用于播放设备。
    - .. _`CEC-LOG-ADDR-TYPE-AUDIOSYSTEM`:

      - `CEC_LOG_ADDR_TYPE_AUDIOSYSTEM`
      - 4
      - 用于音频系统设备。
    - .. _`CEC-LOG-ADDR-TYPE-SPECIFIC`:

      - `CEC_LOG_ADDR_TYPE_SPECIFIC`
      - 5
      - 用于第二台电视或视频处理器设备。
    - .. _`CEC-LOG-ADDR-TYPE-UNREGISTERED`:

      - `CEC_LOG_ADDR_TYPE_UNREGISTERED`
      - 6
      - 如果你只想保持未注册，则使用此类型。用于纯 CEC 开关或仅 CDC 设备（CDC：能力发现与控制）。



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - .. _`CEC-OP-ALL-DEVTYPE-TV`:

      - `CEC_OP_ALL_DEVTYPE_TV`
      - 0x80
      - 支持 TV 类型。
    - .. _`CEC-OP-ALL-DEVTYPE-RECORD`:

      - `CEC_OP_ALL_DEVTYPE_RECORD`
      - 0x40
      - 支持录制类型。
    - .. _`CEC-OP-ALL-DEVTYPE-TUNER`:

      - `CEC_OP_ALL_DEVTYPE_TUNER`
      - 0x20
      - 支持调谐器类型。
    - .. _`CEC-OP-ALL-DEVTYPE-PLAYBACK`:

      - `CEC_OP_ALL_DEVTYPE_PLAYBACK`
      - 0x10
      - 支持播放类型。
    - .. _`CEC-OP-ALL-DEVTYPE-AUDIOSYSTEM`:

      - `CEC_OP_ALL_DEVTYPE_AUDIOSYSTEM`
      - 0x08
      - 支持音频系统类型。
    - .. _`CEC-OP-ALL-DEVTYPE-SWITCH`:

      - `CEC_OP_ALL_DEVTYPE_SWITCH`
      - 0x04
      - 支持 CEC 开关或视频处理类型。

## 返回值


成功时返回 0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 一章中描述。

ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 可以返回以下错误码：

ENOTTY
    未设置 `CEC_CAP_LOG_ADDRS` 能力，因此不支持此 ioctl。

EBUSY
    CEC 适配器当前正在自行配置，或者它已经配置且
    `num_log_addrs` 非零，或者另一个文件句柄处于独占 follower 或发起者模式，或者文件句柄处于 `CEC_MODE_NO_INITIATOR` 模式。

EINVAL
    struct `cec_log_addrs` 的内容无效。
