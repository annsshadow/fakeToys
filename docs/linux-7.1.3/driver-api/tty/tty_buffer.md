## TTY 缓冲


此处记录用于管理 tty 缓冲区及其翻转（flip）的函数。驱动应通过以下某个函数填充缓冲区，然后翻转缓冲区，以便数据被传递给 :doc:`线路规程
<tty_ldisc>` 作进一步处理

## 翻转缓冲区管


   :identifiers: tty_prepare_flip_string
           tty_flip_buffer_push tty_ldisc_receive_buf

   :identifiers: tty_insert_flip_string_fixed_flag tty_insert_flip_string_flags
           tty_insert_flip_char

----

## 其他函数


   :identifiers: tty_buffer_space_avail tty_buffer_set_limit

----

## 缓冲区加


这些仅用于特殊情形。应避免使用

   :identifiers: tty_buffer_lock_exclusive tty_buffer_unlock_exclusive

----

## 内部函数


   :internal:
