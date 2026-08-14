
## TTY


电传打字机（TTY）层负责处理所有那些串行设备，包括像伪终端（PTY）这样的虚拟设备。

## TTY 结构


有若干主要的 TTY 结构。系统中的每个 TTY 设备都有一个对应的 struct tty_port。这些
设备由一个 TTY 驱动（即 struct tty_driver）维护。该结构描述了驱动，同时还包含对
可在 TTY 上执行的操作的引用，即 struct tty_operations。然后，在打开时，会分配一个
struct tty_struct，并一直存活到最终关闭。在此期间，TTY 层会调用 struct
tty_operations 中的若干回调。

内核接收到的每个字符（来自设备和用户两方）都会通过一个预选的
[tty_ldisc](tty_ldisc)（简称 ldisc；在 C 中为 struct tty_ldisc_ops）传递。它的
任务是对字符进行转换，转换方式由特定的 ldisc 或用户定义。默认的是 n_tty，它实现了
回显、信号处理、作业控制、特殊字符处理等。转换后的字符会根据来源进一步传递给
用户/设备。

对上述命名 TTY 结构的详细描述在各独立文档中：

- [tty_driver](tty_driver)
- [tty_port](tty_port)
- [tty_struct](tty_struct)
- [tty_ldisc](tty_ldisc)
- [tty_buffer](tty_buffer)
- [tty_ioctl](tty_ioctl)
- [tty_internals](tty_internals)
- [console](console)

## 编写 TTY 驱动


在着手编写 TTY 驱动之前，必须先考虑 [Serial <../serial/driver>](Serial
<../serial/driver>) 与 [USB Serial <../../usb/usb-serial>](USB Serial
<../../usb/usb-serial>) 层。串行设备的驱动通常可以使用这些特定层之一来实现一个
串行驱动。只有特殊设备才应由 TTY 层直接处理。如果你打算编写这样的驱动，请继续阅读。

一个 TTY 驱动执行的**典型**序列如下：

#. 分配并注册一个 TTY 驱动（模块初始化）
#. 在探测到时创建并注册 TTY 设备（probe 函数）
#. 处理 TTY 操作与事件（如中断）（前者由 TTY 核心调用，后者由设备调用）
#. 在设备移除时移除它们（remove 函数）
#. 注销并释放 TTY 驱动（模块退出）

有关驱动的步骤（即 1.、3. 与 5.）在 [tty_driver](tty_driver) 中有详细描述。对于
另外两步（设备处理），请参阅 [tty_port](tty_port)。

## 其它文档


其它杂项文档可进一步在这些文档中找到：

- [moxa-smartio](moxa-smartio)
- [n_gsm](n_gsm)
- [n_tty](n_tty)
