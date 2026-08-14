


######## Introduction


LIRC 代表 Linux Infrared Remote Control（Linux 红外遥控）。LIRC 设备接口是一个双向接口，用于在用户空间与内核空间之间传输原始 IR 和解码后的扫描码数据。从根本上说，它只是一个字符设备（/dev/lircX，其中 X = 0, 1, 2, ...），在其上定义了一些标准的 struct file_operations。就来回传输原始 IR 和解码后的扫描码而言，关键的 fops 是 read、write 和 ioctl。

也可以向 LIRC 设备附加一个 BPF 程序，将原始 IR 解码为扫描码。

驱动注册带有 LIRC 时 dmesg 输出示例：


    $ dmesg |grep lirc_dev
    rc rc0: lirc_dev: driver mceusb registered at minor = 0, raw IR receiver, raw IR transmitter


你应该看到的字符设备：


    $ ls -l /dev/lirc*
    crw-rw---- 1 root root 248, 0 Jul 2 22:20 /dev/lirc0


注意 `v4l-utils <https://git.linuxtv.org/v4l-utils.git/>`_ 软件包包含用于处理 LIRC 设备的工具：

 - ir-ctl: 可以接收原始 IR 并发送 IR，以及查询 LIRC 设备特性。

 - ir-keytable: 可以加载键映射；允许你设置 IR 内核协议；加载 BPF IR 解码器并测试 IR 解码。也提供了一些 BPF IR 解码器。


######## LIRC modes


LIRC 支持几种接收和发送 IR 码的模式，如下表所示。


`LIRC_MODE_SCANCODE`

    该模式用于发送和接收 IR。

    对于发送（即 transmitting），创建一个 struct lirc_scancode，在 `scancode` 成员中设置期望的扫描码，`rc_proto` 设置为 IR 协议 <Remote_controllers_Protocols>，其他所有成员设为 0。把这个结构体写入 lirc 设备。

    对于接收，你从 LIRC 设备读取 struct lirc_scancode。`scancode` 字段被设为接收到的扫描码，IR 协议 <Remote_controllers_Protocols> 被设在 `rc_proto` 中。如果扫描码映射到一个有效的键码，则它会被设在 `keycode` 字段中，否则设为 `KEY_RESERVED`。

    `flags` 可以在支持 toggle 位的协议中设置 `LIRC_SCANCODE_FLAG_TOGGLE`（例如 rc-5 和 rc-6），或者在支持 repeat 的协议中收到 repeat 时设置 `LIRC_SCANCODE_FLAG_REPEAT`（例如 nec）。

    在 Sanyo 和 NEC 协议中，如果你按住遥控器上的按钮，遥控器不是重复整个扫描码，而是发送一条不含扫描码的更短消息，仅表示按钮被按住，即“repeat”。当收到这个时，`LIRC_SCANCODE_FLAG_REPEAT` 被设置，并且扫描码和键码被重复。

    对于 nec，无法区分“按住按钮”与“反复按同一个按钮”。rc-5 和 rc-6 协议有一个 toggle 位。当按钮被释放并再次按下时，toggle 位被取反。如果设置了 toggle 位，则 `LIRC_SCANCODE_FLAG_TOGGLE` 被设置。

    `timestamp` 字段被填入扫描码被解码时的时间，单位为纳秒（在 `CLOCK_MONOTONIC` 下）。


`LIRC_MODE_MODE2`

    驱动向用户空间返回一串脉冲（pulse）和间隔（space）码，形式为一系列 u32 值。

    该模式仅用于 IR 接收。

    高 8 位决定包类型，低 24 位为负载。使用 `LIRC_VALUE()` 宏获取负载，`LIRC_MODE2()` 宏给出类型，类型之一是：

    `LIRC_MODE2_PULSE`

        表示存在 IR，单位为微秒，也称为 **flash**。

    `LIRC_MODE2_SPACE`

        表示不存在 IR，单位为微秒，也称为 **gap**。

    `LIRC_MODE2_FREQUENCY`

        如果已使用 lirc_set_measure_carrier_mode 启用载波频率测量，则该包给出以赫兹为单位的载波频率。

    `LIRC_MODE2_TIMEOUT`

        当使用 lirc_set_rec_timeout 设置的超时由于未检测到 IR 而到期时，会发送该包，包中是没有 IR 的微秒数。

    `LIRC_MODE2_OVERFLOW`

        表示 IR 接收器遇到了溢出，部分 IR 缺失。此后的 IR 数据应当再次正确。实际值不重要，但为与 lircd 兼容，内核将其设为 0xffffff。


`LIRC_MODE_PULSE`

    在脉冲模式下，使用 lirc-write 把一串脉冲/间隔整数值写入 lirc 设备。

    这些值是交替的脉冲和间隔长度，单位为微秒。第一个和最后一个条目必须是脉冲，因此条目数必须为奇数。

    该模式仅用于 IR 发送。

######## Data types used by LIRC_MODE_SCANCODE


    :identifiers: lirc_scancode rc_proto

######## BPF based IR decoder


内核支持解码最常见的 IR 协议 <Remote_controllers_Protocols>，但还有许多协议不受支持。为了支持这些协议，可以加载一个执行解码的 BPF 程序。这只能在支持读取原始 IR 的 LIRC 设备上完成。

首先，使用带有 `BPF_LOAD_PROG` 参数的 `bpf(2)`_ 系统调用，必须加载类型为 `BPF_PROG_TYPE_LIRC_MODE2` 的程序。一旦附加到 LIRC 设备，该程序将在 LIRC 设备上的每个脉冲、间隔或超时事件时被调用。BPF 程序的上下文是一个指向 unsigned int 的指针，即一个 LIRC_MODE_MODE2 <lirc-mode-mode2> 值。当程序解码出扫描码后，可以使用 BPF 函数 `bpf_rc_keydown()` 或 `bpf_rc_repeat()` 提交。鼠标或指针移动可以使用 `bpf_rc_pointer_rel()` 报告。

一旦你有了 `BPF_PROG_TYPE_LIRC_MODE2` BPF 程序的文件描述符，就可以使用 `bpf(2)`_ 系统调用将其附加到 LIRC 设备。目标必须是 LIRC 设备的文件描述符，附加类型必须是 `BPF_LIRC_MODE2`。一个 LIRC 设备上一次最多可附加 64 个 BPF 程序。
