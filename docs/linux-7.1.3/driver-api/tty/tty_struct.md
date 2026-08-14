
## TTY 结构体


struct tty_struct 由 TTY 层在首次打开 TTY 设备时分配，并在最后一次关闭后
释放。TTY 层将此结构传递给 struct tty_operation 的大多数钩子。tty_struct
的成员在底部的 `TTY Struct Reference`_ 中记录。

## 初始化


   :identifiers: tty_init_termios

## 名称


   :identifiers: tty_name

## 引用计数


   :identifiers: tty_kref_get

   :identifiers: tty_kref_put

## 安装


   :identifiers: tty_standard_install

## 读与写


   :identifiers: tty_put_char

## 启动与停止


   :identifiers: start_tty stop_tty

## 唤醒


   :identifiers: tty_wakeup

## 挂断


   :identifiers: tty_hangup tty_vhangup tty_hung_up_p

## 杂项


   :identifiers: tty_do_resize

## TTY 结构体标志


   :identifiers: tty_struct_flags

## TTY 结构体参考


   :identifiers: tty_struct
