
## TTY Port

本页面介TTY 子系统中 struct tty_port 辅助机制及其对外 API，面TTY 驱动开发者，涵盖端口的初始化、打开/关闭/挂断处理、引用计数与调制解调器信号控制等辅助函数


建议 TTY 驱动尽可能使struct tty_port 辅助函数。如果驱动实现了
:c`tty_port.ops.activate()` :c`tty_port.ops.shutdown()`，它们可以在相应
:c`tty_struct.ops` 钩子中使tty_port_open()、tty_port_close() 
tty_port_hangup()銆。

引用与细节包含在底部`TTY Port Reference`_ `TTY Port Operations Reference`_
小节中

## TTY Port 函数


### 初始化与销


   :identifiers: tty_port_init tty_port_destroy
        tty_port_get tty_port_put

### Open/Close/Hangup 辅助函数


   :identifiers: tty_port_install tty_port_open tty_port_block_til_ready
        tty_port_close tty_port_close_start tty_port_close_end tty_port_hangup
        tty_port_shutdown

### TTY 引用计数


   :identifiers: tty_port_tty_get tty_port_tty_set

### TTY 辅助函数


   :identifiers: tty_port_tty_hangup tty_port_tty_vhangup
   :identifiers: tty_port_tty_wakeup

### 调制解调器信


   :identifiers: tty_port_carrier_raised tty_port_raise_dtr_rts
        tty_port_lower_dtr_rts

----

## TTY Port 引用


   :identifiers: tty_port

----

## TTY Port 操作引用


   :identifiers: tty_port_operations
