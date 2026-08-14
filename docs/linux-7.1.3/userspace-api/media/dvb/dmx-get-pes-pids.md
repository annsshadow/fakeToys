## DMX_GET_PES_PIDS


### Name


DMX_GET_PES_PIDS

### Synopsis


`int ioctl(fd, DMX_GET_PES_PIDS, __u16 pids[^5^])`

### Arguments


`fd`
    `open()` 返回的文件描述符。

`pids`
    用于存储 5 个节目 ID（Program ID）的数组。

### Description


该 ioctl 用于查询 DVB 设备，以返回给定服务中音频、视频、图文电视（teletext）、字幕和 PCR 节目所使用的第一个 PID。它们按如下方式存储：

=======================	========	=======================================
PID  element		position	content
=======================	========	=======================================
pids[DMX_PES_AUDIO]	0		first audio PID
pids[DMX_PES_VIDEO]	1		first video PID
pids[DMX_PES_TELETEXT]	2		first teletext PID
pids[DMX_PES_SUBTITLE]	3		first subtitle PID
pids[DMX_PES_PCR]	4		first Program Clock Reference PID
=======================	========	=======================================


	等于 0xffff 的值表示该 PID 未被内核（Kernel）填充。

### Return Value


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码在 Generic Error Codes <gen-errors> 章节中描述。
