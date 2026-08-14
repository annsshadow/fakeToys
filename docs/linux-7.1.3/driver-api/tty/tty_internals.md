
## TTY 内部实现


本文件记录 TTY 子系统的内部实现与 API。

## Kopen


这些函数用于从内核空间打开一个 TTY：

      :identifiers: tty_kopen_exclusive tty_kopen_shared tty_kclose

----

## 导出的内部函数


   :identifiers: tty_release_struct tty_dev_name_to_number tty_get_icount

----

## 内部函数


   :internal:
