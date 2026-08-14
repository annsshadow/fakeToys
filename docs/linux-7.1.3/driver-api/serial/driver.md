## 底层串口 API


本文档旨在简要概述新串口驱动的一些方面。它并不完整，你有任何问题应直接联系 <rmk@arm.linux.org.uk>

参考实现包含在 amba-pl011.c 中。


### 底层串口硬件驱动


底层串口硬件驱动负责向核心串口驱动提供端口信息（由 uart_port 定义）与一组控制方法（由 uart_ops 定义）。底层驱动还负责处理该端口的中断，并提供任何控制台支持。


### 控制台支持


串口核心提供了一些辅助函数。这包括解析命令行参数（uart_parse_options()）。

还有一个辅助函数（uart_console_write()）执行逐字符写入，将换行符转换为 CRLF 序列。建议驱动编写者使用此函数，而不是实现自己的版本。


### 锁定


底层硬件驱动有责任使用 port->lock 执行必要的锁定。有一些例外（在下面的 struct uart_ops 列表中有描述）。

有两把锁。一把是每个端口的自旋锁，另一把是整体的信号量。

从核心驱动的视角看，port->lock 锁定以下内容
```

	port->mctrl
	port->icount
	port->state->xmit.head (circ_buf->head)
	port->state->xmit.tail (circ_buf->tail)

```

底层驱动可以自由使用这把锁来提供任何额外的锁定。

port_sem 信号量用于防止端口在不恰当时机被添加/移除或重新配置。自 v2.6.27 起，这把信号量已成为 tty_port 结构体的 'mutex' 成员，通常称为端口互斥体（port mutex）。


### uart_ops


   :identifiers: uart_ops

### 其他函数


   :identifiers: uart_update_timeout uart_get_baud_rate uart_get_divisor
           uart_match_port uart_write_wakeup uart_register_driver
           uart_unregister_driver uart_suspend_port uart_resume_port
           uart_add_one_port uart_remove_one_port uart_console_write
           uart_parse_earlycon uart_parse_options uart_set_options
           uart_get_lsr_info uart_handle_dcd_change uart_handle_cts_change
           uart_try_toggle_sysrq

   :identifiers: uart_port_tx_limited uart_port_tx

### 其他说明


计划有朝一日从 uart_port 中移除 'unused' 条目，并允许底层驱动向核心注册它们各自的 uart_port。这将允许驱动把 uart_port 用作一个指向结构体的指针，该结构体既包含 uart_port 条目，也包含它们自己的扩展，
```

	struct my_port {
		struct uart_port	port;
		int			my_stuff;
	};

```

### 通过 GPIO 的调制解调器控制线


提供了一些辅助函数，用于通过 GPIO 设置/获取调制解调器控制线。

   :identifiers: mctrl_gpio_init mctrl_gpio_to_gpiod
           mctrl_gpio_set mctrl_gpio_get mctrl_gpio_enable_ms
           mctrl_gpio_disable_ms_sync mctrl_gpio_disable_ms_no_sync
