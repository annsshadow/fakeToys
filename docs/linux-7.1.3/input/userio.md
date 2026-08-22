
## userio 协议


:Copyright: |copy| 2015 Stephen Chandler Paul <thatslyude@gmail.com>

Red Hat 赞助


## 简

本模块旨在让输入驱动开发者的工作更轻松：它允许开发者在没有真实物理设备
的情况下，测试各serio 设备（主要是笔记本上常见的各类触摸板）。userio
的实现方式是：允许任何特权用户态程序直接与内核serio 驱动交互，并由此
控制一个虚拟的 serio 端口
## 使用概览


要与 userio 内核模块交互，只需在应用程序中打开 /dev/userio 字符设备即可向该设备写入数据即可向内核模块发送命令，而从 serio 驱动接收到的任何数据
都可以按原样/dev/userio 设备读取。与设备交互所需的所有结构体和宏都定<linux/userio.h> <linux/serio.h> 中
## 命令结构


```

	struct userio_cmd {
		__u8 type;
		__u8 data;
	};

```
`type` 描述所发送命令的类型，可以是 <linux/userio.h> 中定义的任意一USERIO_CMD 宏。`data` 是随命令一起传入的参数。如果命令不带参数，则该字段
可以保持不变，内核会忽略它。每条命令都应通过将结构体直接写入字符设备发送。如果你发送的命令无效，字符设备会返回一个错误，并且内核日志中会打印
更详细的错误信息。同一时间只能发送一条命令，在初始命令之后写入字符设备的
任何额外数据都将被忽略
要关闭虚serio 端口，只需关闭 /dev/userio 即可
## 命令


#### USERIO_CMD_REGISTER


serio 驱动注册该端口，并开始来回传输数据。只有在USERIO_CMD_SET_PORT_TYPE 设置了端口类型后才能执行注册。该命令没有参数
#### USERIO_CMD_SET_PORT_TYPE


设置我们所模拟的端口类型，其中 `data` 为要设置的端口类型。可以是
<linux/serio.h> 中的任意宏。例如：SERIO_8042 会把端口类型设置为普通的
PS/2 端口
#### USERIO_CMD_SEND_INTERRUPT


通过虚拟 serio 端口serio 驱动发送一个中断，其中 `data` 为要发送的中断
数据
## 用户态工

userio 的用户态工具能够利i8042 的部分调试信息来录制 PS/2 设备，并/dev/userio 上回放这些设备。这些工具的最新版本可以在以下地址获取
	https://github.com/Lyude/ps2emu
