

######## ioctl LIRC_GET_FEATURES


## 名称

LIRC_GET_FEATURES - 获取底层硬件设备的特性


## Synopsis

`int ioctl(int fd, LIRC_GET_FEATURES, __u32 *features)`


## Arguments

`fd`
由 open() 返回的文件描述符。

`features`
LIRC 特性的位掩码。


## 描述

获取底层硬件设备的特性。驱动会宣告它所支持的某些特性，调用方再据此发起相应的 ioctl。


## LIRC 特性


`LIRC_CAN_REC_RAW`

未使用。保留仅为避免破坏 uAPI。


`LIRC_CAN_REC_PULSE`

未使用。保留仅为避免破坏 uAPI。在发送时使用 `LIRC_MODE_PULSE` <lirc-模式-pulse>。


`LIRC_CAN_REC_MODE2`

原始 IR 驱动接收时使用。意味着使用 `LIRC_MODE_MODE2` <lirc-模式-MODE2>。同时也意味着支持 `LIRC_MODE_SCANCODE` <lirc-模式-SCANCODE>，只要内核版本足够新。可使用 `lirc_set_rec_mode` 切换模式。


`LIRC_CAN_REC_LIRCCODE`

未使用。保留仅为避免破坏 uAPI。


`LIRC_CAN_REC_SCANCODE`

scancode 驱动接收时使用。意味着使用 `LIRC_MODE_SCANCODE` <lirc-模式-SCANCODE>。


`LIRC_CAN_SET_SEND_CARRIER`

驱动支持使用 ioctl `LIRC_SET_SEND_CARRIER` <LIRC_SET_SEND_CARRIER> 改变调制频率。


`LIRC_CAN_SET_SEND_DUTY_CYCLE`

驱动支持使用 ioctl `LIRC_SET_SEND_DUTY_CYCLE` <LIRC_SET_SEND_DUTY_CYCLE> 改变占空比。


`LIRC_CAN_SET_TRANSMITTER_MASK`

驱动支持使用 ioctl `LIRC_SET_TRANSMITTER_MASK` <LIRC_SET_TRANSMITTER_MASK> 改变激活的发送器。


`LIRC_CAN_SET_REC_CARRIER`

驱动支持使用 ioctl `LIRC_SET_REC_CARRIER` <LIRC_SET_REC_CARRIER> 设置接收载波频率。


`LIRC_CAN_SET_REC_CARRIER_RANGE`

驱动支持 ioctl `LIRC_SET_REC_CARRIER_RANGE` <LIRC_SET_REC_CARRIER_RANGE>。


`LIRC_CAN_GET_REC_RESOLUTION`

驱动支持 ioctl `LIRC_GET_REC_RESOLUTION` <LIRC_GET_REC_RESOLUTION>。


`LIRC_CAN_SET_REC_TIMEOUT`

驱动支持 ioctl `LIRC_SET_REC_TIMEOUT` <LIRC_SET_REC_TIMEOUT>。


`LIRC_CAN_MEASURE_CARRIER`

驱动支持使用 ioctl `LIRC_SET_MEASURE_CARRIER_MODE` <LIRC_SET_MEASURE_CARRIER_MODE> 测量调制频率。


`LIRC_CAN_USE_WIDEBAND_RECEIVER`

驱动支持使用 ioctl `LIRC_SET_WIDEBAND_RECEIVER` <LIRC_SET_WIDEBAND_RECEIVER> 进入学习模式。


`LIRC_CAN_SEND_RAW`

未使用。保留仅为避免破坏 uAPI。


`LIRC_CAN_SEND_PULSE`

驱动支持使用 `LIRC_MODE_PULSE` <lirc-模式-pulse> 发送（亦称 IR blasting / IR 发射）。意味着支持使用 `LIRC_MODE_SCANCODE` <lirc-模式-SCANCODE> 发送，只要内核版本足够新。可使用 `lirc_set_send_mode` 切换模式。


`LIRC_CAN_SEND_MODE2`

未使用。保留仅为避免破坏 uAPI。在接收时使用 `LIRC_MODE_MODE2` <lirc-模式-mode2>。


`LIRC_CAN_SEND_LIRCCODE`

未使用。保留仅为避免破坏 uAPI。


## 返回值

成功时返回 0，出错时返回 -1，并相应地设置 `errno` 变量。通用的错误码在《Generic 错误 Codes》<gen-错误> 章节中描述。
