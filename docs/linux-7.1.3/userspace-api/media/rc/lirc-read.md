
######## LIRC read()


## 名称


lirc-read - 从 LIRC 设备读取

## 概要


    #include <unistd.h>


## 参数


`fd`
    由 `open()` 返回的文件描述符。

`buf`
   待填充的缓冲区

`count`
   最多读取的字节数

## 描述


`read()` 尝试从文件描述符 `fd` 向起始于 `buf` 的缓冲区读取最多 `count` 个字节。如果 `count` 为零，`read()` 返回零且没有其他结果。如果 `count` 大于 `SSIZE_MAX`，结果是未指定的。

数据的确切格式取决于驱动使用的 lirc_modes。使用 lirc_get_features 获取支持的模式，并使用 lirc_set_rec_mode 设置当前活动模式。

LIRC_MODE_MODE2 <lirc-mode-mode2> 模式用于原始 IR，其中读取自字符设备的包包含一个描述 IR 信号的无符号 int 值。

另外，LIRC_MODE_SCANCODE <lirc-mode-scancode> 也可能可用，在该模式下，扫描码由软件解码器或硬件解码器解码。`rc_proto` 成员被设为用于传输的 IR 协议 <Remote_controllers_Protocols>，`scancode` 被设为解码后的扫描码，`keycode` 被设为键码或 `KEY_RESERVED`。

## 返回值


成功时返回读取的字节数。如果这个数字小于请求的字节数，或一帧所需的数据量，这不算错误。出错时返回 -1，并相应地设置 `errno` 变量。
