


######## ioctls LIRC_GET_REC_MODE and LIRC_SET_REC_MODE


## 名称


LIRC_GET_REC_MODE/LIRC_SET_REC_MODE - 获取/设置当前接收模式
## 概要



`int ioctl(int fd, LIRC_GET_REC_MODE, __u32 *mode)`


`int ioctl(int fd, LIRC_SET_REC_MODE, __u32 *mode)`

## 参数


`fd`
    open() 返回的文件描述符
`mode`
    用于接收mode
## 描述


获取并设置当前接收模式。仅支持 LIRC_MODE_MODE2 <lirc-mode-mode2> LIRC_MODE_SCANCODE <lirc-mode-scancode>。使lirc_get_features 查明驱动
程序支持哪些模式
## 杩斿洖鍊。


    :header-rows:  0
    :stub-columns: 0

    - .. row 1

       - `ENODEV`

       - 设备不可用
    - .. row 2

       - `ENOTTY`

       - 设备不支持接收
    - .. row 3

       - `EINVAL`

       - 无效模式，或对此设备无效的模式