######## ioctls LIRC_GET_SEND_MODE and LIRC_SET_SEND_MODE


## 名称


LIRC_GET_SEND_MODE/LIRC_SET_SEND_MODE - 获取/设置当前发送模式
## 概要


`int ioctl(int fd, LIRC_GET_SEND_MODE, __u32 *mode)`


`int ioctl(int fd, LIRC_SET_SEND_MODE, __u32 *mode)`

## 参数


`fd`
    open() 返回的文件描述符
`mode`
    用于发送的 mode
## 描述


获取/设置当前发送模式
根据驱动的不同，IR 发送仅支持 LIRC_MODE_PULSE <lirc-mode-pulse> LIRC_MODE_SCANCODE <lirc-mode-scancode>。使lirc_get_features 可查明驱动支持哪些模式
## 杩斿洖鍊。

    :header-rows:  0
    :stub-columns: 0

    - .. row 1

       - `ENODEV`

       - 设备不可用
    - .. row 2

       - `ENOTTY`

       - 设备不支持发送
    - .. row 3

       - `EINVAL`

       - 无效的模式或该设备的无效模式