


######## LIRC write()


## 名称


lirc-write - 写入一LIRC 设备

## 概要


    #include <unistd.h>


## 参数


`fd`
    `open()` 返回的文件描述符
`buf`
    包含待写入数据的缓冲
`count`
    缓冲区中的字节数

## 描述


`write()` 将从 `buf` 开始的缓冲区中最`count` 个字节写入由文件描述`fd` 所引用的设备
数据的确切格式取决于驱动所处的模式，请使用 lirc_get_features 获取所支持的模式，并使lirc_set_send_mode 设置模式
当处LIRC_MODE_PULSE <lirc-mode-PULSE> 模式时，写入 chardev 的数据是一串整数值表示的脉冲/空白（pulse/space）序列。脉冲和空白仅通过它们的位置隐式标记。数据必须以一个脉冲开始并以一个脉冲结束，因此数据必须始终包含奇数个采样。write 函数会阻塞，直到硬件传输完数据为止。如果提供的数据多于硬件能够发送的量，驱动返回 `EINVAL`
当处LIRC_MODE_SCANCODE <lirc-mode-scancode> 模式时，每次必须chardev 写入一`struct lirc_scancode`，否则返`EINVAL`。在 `scancode` 成员中设置所需的扫描码，在 `rc_proto`：成员中设置 IR 协议 <Remote_controllers_Protocols>。所有其他成员必须设置为 0，否则返`EINVAL`。如果没有该协议的协议编码器，或者该扫描码对指定协议无效，则返回 `EINVAL`。write 函数会阻塞，直到扫描码被硬件传输完毕
## 杩斿洖鍊。

成功时返回写入的字节数。如果这个数小于请求的字节数，或小于一帧所需的数据量，这并不算错误。出错时返回 -1，并相应地设`errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述